#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use core::panic::PanicInfo;
use core::{fmt::Write as _, ptr::addr_of_mut};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOB_RUNTIME_RESOURCES, GPIOB},
    i2c::{DRV_I2C1_RUNTIME_RESOURCES, I2C1},
    rcc::{DRV_RCC_RUNTIME_RESOURCES, RCC},
    usb::{DRV_USBD_RUNTIME_RESOURCES, USBD, USBDUsbDriver},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};
use embedded_hal::i2c::I2c;
use heapless::String;

const ACTIVE_HOST_POLL: Duration = Duration::from_millis(100);
const DISCONNECT_POLL: Duration = Duration::from_millis(250);
const INTER_PACKET_DELAY: Duration = Duration::from_millis(50);
const SLAVE_ADDRESS: u8 = 0x42;
const NORMAL_REQUEST: [u8; 3] = [0x10, 0x20, 0x30];
const NORMAL_RESPONSE: [u8; 4] = [0x30, 0x20, 0x10, 0x03];
const ISR_REQUEST: [u8; 3] = [0xA5, 0x5A, 0x01];
const STATUS_QUERY: [u8; 1] = [0x55];

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

#[cfg(not(test))]
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
    configure_board_i2c_pins();

    let mut i2c1 = I2C1::new(DRV_I2C1_RUNTIME_RESOURCES).unwrap();
    i2c1.enable_clock().unwrap();
    i2c1.release_reset().unwrap();
    i2c1.init_master().unwrap();

    let usbd = USBD::new(DRV_USBD_RUNTIME_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();

    let mut config = Config::new(0xCAFE, 0x4010);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 I2C Slave Master Smoke");
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
            "Slave smoke armed: connect PB6=SCL/PB7=SDA between the two boards and flash the matching slave image.",
        )
        .await;

        match run_smoke_session(&mut cdc, &mut i2c1).await {
            Ok(()) => {
                log_line(
                    &mut cdc,
                    "PASS: normal and ISR-driven slave packet paths completed.",
                )
                .await
            }
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
    let mut normal = [0u8; NORMAL_RESPONSE.len()];
    log_line(cdc, "normal path: writing request packet").await;
    I2c::write(i2c1, SLAVE_ADDRESS, &NORMAL_REQUEST)
        .map_err(|_| "FAIL: normal-path master write returned an error.")?;
    Timer::after(INTER_PACKET_DELAY).await;
    log_line(cdc, "normal path: reading response packet").await;
    I2c::read(i2c1, SLAVE_ADDRESS, &mut normal)
        .map_err(|_| "FAIL: normal-path master read returned an error.")?;
    if normal != NORMAL_RESPONSE {
        return Err("FAIL: normal-path slave response payload mismatch.");
    }
    let mut line = String::<96>::new();
    let _ = write!(&mut line, "normal path: response={:#04X?}", normal);
    log_line(cdc, line.as_str()).await;

    log_line(cdc, "isr path: writing completed-packet callback trigger").await;
    I2c::write(i2c1, SLAVE_ADDRESS, &ISR_REQUEST)
        .map_err(|_| "FAIL: ISR-path trigger write returned an error.")?;
    Timer::after(INTER_PACKET_DELAY).await;
    log_line(cdc, "isr path: querying callback status").await;
    I2c::write(i2c1, SLAVE_ADDRESS, &STATUS_QUERY)
        .map_err(|_| "FAIL: ISR-path status query write returned an error.")?;
    Timer::after(INTER_PACKET_DELAY).await;
    let mut status = [0u8; 1];
    I2c::read(i2c1, SLAVE_ADDRESS, &mut status)
        .map_err(|_| "FAIL: ISR-path status read returned an error.")?;
    if status[0] != 1 {
        return Err("FAIL: slave ISR callback did not report the completed RX packet.");
    }
    log_line(
        cdc,
        "isr path: slave callback reported the completed RX packet.",
    )
    .await;

    Ok(())
}

async fn log_line(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>, line: &str) {
    let _ = cdc.write_packet(line.as_bytes()).await;
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

fn configure_board_i2c_pins() {
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
