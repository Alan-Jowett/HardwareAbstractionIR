#![no_std]
#![no_main]

use core::{fmt::Write as _, panic::PanicInfo, ptr::addr_of_mut};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOB_RUNTIME_RESOURCES, GPIOB},
    i2c::{DRV_I2C1_RUNTIME_RESOURCES, I2C1},
    rcc::{DRV_RCC_RUNTIME_RESOURCES, RCC},
    usb::{DRV_USBD_RUNTIME_RESOURCES, USBD, USBDUsbDriver},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, with_timeout};
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};
use heapless::String;

const ACTIVE_HOST_POLL: Duration = Duration::from_millis(100);
const DISCONNECT_POLL: Duration = Duration::from_millis(250);
const I2C_ASYNC_TIMEOUT: Duration = Duration::from_millis(250);
const SHT40_MEASUREMENT_DELAY: Duration = Duration::from_millis(20);
const SHT40_ADDRESS: u8 = 0x44;
const SHT40_MEASURE_HIGH_PRECISION: u8 = 0xFD;

const GPIOB_CFGLR: u32 = 0x40010C00;
const GPIOB_BSHR: u32 = 0x40010C10;
const PB6_MODE_SHIFT: u32 = 24;
const PB7_MODE_SHIFT: u32 = 28;
const GPIO_ALT_OPEN_DRAIN_50MHZ: u32 = 0xF;

static mut CONFIG_DESCRIPTOR: [u8; 256] = [0; 256];
static mut BOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut MSOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut CONTROL_BUFFER: [u8; 128] = [0; 128];
static mut CDC_STATE: CdcState<'static> = CdcState::new();

#[derive(Clone, Copy)]
struct Sht40Sample {
    temperature_c_x100: i32,
    humidity_pct_x100: i32,
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::task]
async fn usb_task(mut device: UsbDevice<'static, USBDUsbDriver>) -> ! {
    device.run().await
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    let rcc = RCC::new(DRV_RCC_RUNTIME_RESOURCES).unwrap();
    rcc.configure_usb_fsdev_clock_48mhz().unwrap();
    wch::init_embassy_time_runtime().unwrap();

    let gpiob = GPIOB::new(DRV_GPIOB_RUNTIME_RESOURCES).unwrap();
    gpiob.enable_clock().unwrap();
    gpiob.release_reset().unwrap();
    configure_stemma_qt_i2c_pins();

    let mut i2c1 = I2C1::new(DRV_I2C1_RUNTIME_RESOURCES).unwrap();
    i2c1.enable_clock().unwrap();
    i2c1.release_reset().unwrap();
    i2c1.init_master().unwrap();

    let usbd = USBD::new(DRV_USBD_RUNTIME_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();

    let mut config = Config::new(0xCAFE, 0x4008);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 SHT40 I2C Smoke");
    config.serial_number = Some("0001");
    config.max_power = 100;

    let config_descriptor = unsafe { &mut *addr_of_mut!(CONFIG_DESCRIPTOR) };
    let bos_descriptor = unsafe { &mut *addr_of_mut!(BOS_DESCRIPTOR) };
    let msos_descriptor = unsafe { &mut *addr_of_mut!(MSOS_DESCRIPTOR) };
    let control_buffer = unsafe { &mut *addr_of_mut!(CONTROL_BUFFER) };
    let cdc_state = unsafe { &mut *addr_of_mut!(CDC_STATE) };

    let mut builder = Builder::new(
        driver,
        config,
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control_buffer,
    );
    let mut cdc = CdcAcmClass::new(&mut builder, cdc_state, 64);
    let usb = builder.build();

    spawner.spawn(usb_task(usb)).unwrap();

    loop {
        cdc.wait_connection().await;
        wait_for_active_host(&mut cdc).await;

        log_line(
            &mut cdc,
            "SHT40 smoke armed: plug the sensor into STEMMA QT (PB6=SCL, PB7=SDA).",
        )
        .await;

        match run_smoke_session(&mut cdc, &mut i2c1).await {
            Ok(()) => log_line(&mut cdc, "PASS: blocking and async SHT40 reads completed.").await,
            Err(message) => log_line(&mut cdc, message).await,
        }

        log_line(
            &mut cdc,
            "Session complete. Drop DTR or reconnect to rerun the smoke.",
        )
        .await;
        wait_for_disconnect(&mut cdc).await;
    }
}

async fn run_smoke_session(
    cdc: &mut CdcAcmClass<'static, USBDUsbDriver>,
    i2c1: &mut I2C1,
) -> Result<(), &'static str> {
    log_line(cdc, "blocking measurement: sending 0xFD command").await;
    let blocking = perform_blocking_measurement(i2c1).await?;
    log_sample(cdc, "blocking measurement", blocking).await;

    log_line(cdc, "async measurement: sending 0xFD command").await;
    let asynchronous = perform_async_measurement(i2c1).await?;
    log_sample(cdc, "async measurement", asynchronous).await;

    Ok(())
}

async fn perform_blocking_measurement(i2c1: &mut I2C1) -> Result<Sht40Sample, &'static str> {
    embedded_hal::i2c::I2c::write(i2c1, SHT40_ADDRESS, &[SHT40_MEASURE_HIGH_PRECISION])
        .map_err(|_| "FAIL: blocking I2C write to SHT40 returned an error.")?;
    Timer::after(SHT40_MEASUREMENT_DELAY).await;
    let mut raw = [0u8; 6];
    embedded_hal::i2c::I2c::read(i2c1, SHT40_ADDRESS, &mut raw)
        .map_err(|_| "FAIL: blocking I2C read from SHT40 returned an error.")?;
    decode_sht40_sample(raw)
}

async fn perform_async_measurement(i2c1: &mut I2C1) -> Result<Sht40Sample, &'static str> {
    with_timeout(
        I2C_ASYNC_TIMEOUT,
        embedded_hal_async::i2c::I2c::write(i2c1, SHT40_ADDRESS, &[SHT40_MEASURE_HIGH_PRECISION]),
    )
    .await
    .map_err(|_| "FAIL: async I2C write to SHT40 timed out.")?
    .map_err(|_| "FAIL: async I2C write to SHT40 returned an error.")?;
    Timer::after(SHT40_MEASUREMENT_DELAY).await;
    let mut raw = [0u8; 6];
    with_timeout(
        I2C_ASYNC_TIMEOUT,
        embedded_hal_async::i2c::I2c::read(i2c1, SHT40_ADDRESS, &mut raw),
    )
    .await
    .map_err(|_| "FAIL: async I2C read from SHT40 timed out.")?
    .map_err(|_| "FAIL: async I2C read from SHT40 returned an error.")?;
    decode_sht40_sample(raw)
}

fn decode_sht40_sample(raw: [u8; 6]) -> Result<Sht40Sample, &'static str> {
    let temp_crc = crc8(&raw[0..2]);
    if temp_crc != raw[2] {
        return Err("FAIL: SHT40 temperature CRC mismatch.");
    }
    let humidity_crc = crc8(&raw[3..5]);
    if humidity_crc != raw[5] {
        return Err("FAIL: SHT40 humidity CRC mismatch.");
    }

    let raw_temp = u16::from_be_bytes([raw[0], raw[1]]);
    let raw_humidity = u16::from_be_bytes([raw[3], raw[4]]);

    let temperature_c_x100 =
        -4500 + i32::try_from((17500u64 * u64::from(raw_temp)) / 65535u64).unwrap_or(0);
    let mut humidity_pct_x100 =
        -600 + i32::try_from((12500u64 * u64::from(raw_humidity)) / 65535u64).unwrap_or(0);
    humidity_pct_x100 = humidity_pct_x100.clamp(0, 10_000);

    Ok(Sht40Sample {
        temperature_c_x100,
        humidity_pct_x100,
    })
}

fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0xFFu8;
    for &byte in bytes {
        crc ^= byte;
        for _ in 0..8 {
            crc = if (crc & 0x80) != 0 {
                (crc << 1) ^ 0x31
            } else {
                crc << 1
            };
        }
    }
    crc
}

async fn log_sample(
    cdc: &mut CdcAcmClass<'static, USBDUsbDriver>,
    label: &str,
    sample: Sht40Sample,
) {
    let mut message = String::<128>::new();
    let _ = write!(
        &mut message,
        "{}: temp={}.{:02}C humidity={}.{:02}%RH",
        label,
        fixed_point_whole(sample.temperature_c_x100),
        fixed_point_frac(sample.temperature_c_x100),
        fixed_point_whole(sample.humidity_pct_x100),
        fixed_point_frac(sample.humidity_pct_x100),
    );
    log_line(cdc, message.as_str()).await;
}

fn fixed_point_whole(value_x100: i32) -> i32 {
    value_x100 / 100
}

fn fixed_point_frac(value_x100: i32) -> i32 {
    value_x100.abs() % 100
}

fn configure_stemma_qt_i2c_pins() {
    write_u32(GPIOB_BSHR, (1u32 << 6) | (1u32 << 7));
    modify_u32(
        GPIOB_CFGLR,
        (0xFu32 << PB6_MODE_SHIFT) | (0xFu32 << PB7_MODE_SHIFT),
        (GPIO_ALT_OPEN_DRAIN_50MHZ << PB6_MODE_SHIFT)
            | (GPIO_ALT_OPEN_DRAIN_50MHZ << PB7_MODE_SHIFT),
    );
}

fn write_u32(address: u32, value: u32) {
    unsafe {
        (address as *mut u32).write_volatile(value);
    }
}

fn modify_u32(address: u32, clear_mask: u32, set_mask: u32) {
    unsafe {
        let ptr = address as *mut u32;
        let current = ptr.read_volatile();
        ptr.write_volatile((current & !clear_mask) | set_mask);
    }
}

async fn log_line(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>, message: &str) {
    let _ = cdc.write_packet(message.as_bytes()).await;
    let _ = cdc.write_packet(b"\r\n").await;
}

async fn wait_for_active_host(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>) {
    while !cdc.dtr() {
        Timer::after(ACTIVE_HOST_POLL).await;
    }
}

async fn wait_for_disconnect(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>) {
    while cdc.dtr() {
        Timer::after(DISCONNECT_POLL).await;
    }
}
