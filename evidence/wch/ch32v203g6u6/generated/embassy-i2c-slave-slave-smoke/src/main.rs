#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use core::panic::PanicInfo;
use core::{
    ptr::addr_of_mut,
    sync::atomic::{AtomicBool, AtomicU8, Ordering},
};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOB_RUNTIME_RESOURCES, GPIOB},
    i2c::{DRV_I2C1_SLAVE_RUNTIME_RESOURCES, I2C1Slave},
    rcc::{DRV_RCC_RUNTIME_RESOURCES, RCC},
    usb::{DRV_USBD_RUNTIME_RESOURCES, USBD, USBDUsbDriver},
    wch,
};
use embassy_executor::Spawner;
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};
use embassy_time::{Duration, Timer};

const SLAVE_ADDRESS: u8 = 0x42;

const GPIOB_CFGLR: u32 = 0x40010C00;
const GPIOB_BSHR: u32 = 0x40010C10;
const PB6_MODE_SHIFT: u32 = 24;
const PB7_MODE_SHIFT: u32 = 28;
const GPIO_ALT_OPEN_DRAIN_50MHZ: u32 = 0xF;
const PACKET_POLL_INTERVAL: Duration = Duration::from_millis(10);

static mut CONFIG_DESCRIPTOR: [u8; 256] = [0; 256];
static mut BOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut MSOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut CONTROL_BUFFER: [u8; 128] = [0; 128];
static mut CDC_STATE: CdcState<'static> = CdcState::new();
static ISR_PACKET_SEEN: AtomicBool = AtomicBool::new(false);
static ISR_REGISTER: AtomicU8 = AtomicU8::new(0);
static ISR_VALUE: AtomicU8 = AtomicU8::new(0);
static mut ISR_BUFFER: [u8; 32] = [0; 32];

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    let rcc = RCC::new(DRV_RCC_RUNTIME_RESOURCES).unwrap();
    rcc.configure_usb_fsdev_clock_48mhz().unwrap();
    wch::init_embassy_time_runtime().unwrap();

    let gpiob = GPIOB::new(DRV_GPIOB_RUNTIME_RESOURCES).unwrap();
    gpiob.enable_clock().unwrap();
    gpiob.release_reset().unwrap();
    enable_i2c1_default_pin_route();
    configure_board_i2c_pins();

    let i2c1 = I2C1Slave::new(DRV_I2C1_SLAVE_RUNTIME_RESOURCES).unwrap();
    i2c1.enable_clock().unwrap();
    i2c1.release_reset().unwrap();
    i2c1.init_slave().unwrap();
    i2c1.set_own_address_7bit(SLAVE_ADDRESS).unwrap();
    let usbd = USBD::new(DRV_USBD_RUNTIME_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();
    let mut config = Config::new(0xCAFE, 0x4011);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 I2C Slave Smoke");
    config.serial_number = Some("0002");
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
        i2c1
            .enable_rx_packet_isr_dispatch(
                unsafe { &mut *addr_of_mut!(ISR_BUFFER) },
                on_isr_packet,
            )
            .unwrap();
        while cdc.dtr() {
            Timer::after(PACKET_POLL_INTERVAL).await;
            if ISR_PACKET_SEEN.load(Ordering::Acquire) {
                ISR_PACKET_SEEN.store(false, Ordering::Release);
                log_received_register_value(&mut cdc).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn usb_task(mut device: UsbDevice<'static, USBDUsbDriver>) -> ! {
    device.run().await
}

async fn log_line(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>, line: &str) {
    let _ = cdc.write_packet(line.as_bytes()).await;
    let _ = cdc.write_packet(b"\r\n").await;
}

async fn log_received_register_value(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>) {
    let register = ISR_REGISTER.load(Ordering::Acquire);
    let value = ISR_VALUE.load(Ordering::Acquire);
    let mut line = heapless::String::<48>::new();
    if core::fmt::write(
        &mut line,
        format_args!("register=0x{register:02X} value=0x{value:02X}"),
    )
    .is_err()
    {
        return;
    }
    log_line(cdc, line.as_str()).await;
}

async fn wait_for_active_host(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>) {
    while !cdc.dtr() {
        Timer::after(Duration::from_millis(100)).await;
    }
}

fn on_isr_packet(bytes: &[u8], truncated: bool) {
    if truncated || bytes.len() < 2 {
        return;
    }
    ISR_REGISTER.store(bytes[0], Ordering::Relaxed);
    ISR_VALUE.store(bytes[1], Ordering::Relaxed);
    ISR_PACKET_SEEN.store(true, Ordering::Release);
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

fn enable_i2c1_default_pin_route() {
    modify_u32(0x40021018, 0x00000001, 0x00000001);
    modify_u32(0x40010004, 0x00000002, 0x00000000);
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
