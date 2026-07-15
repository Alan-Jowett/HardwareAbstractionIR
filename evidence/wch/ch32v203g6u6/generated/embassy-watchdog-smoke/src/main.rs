#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use core::panic::PanicInfo;
use core::{fmt::Write as _, ptr::addr_of_mut};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOA_RESOURCES, GPIOA, GPIOAOutput, Level},
    rcc::{DRV_RCC_RESOURCES, RCC},
    usb::{DRV_USBD_RESOURCES, USBD, USBDUsbDriver},
    watchdog::{DRV_IWDG_RESOURCES, IWDG, IWDGConfig},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};
use embedded_hal_02::watchdog::{Watchdog, WatchdogEnable};
use heapless::String;

const WATCHDOG_PRESCALER: u8 = 6;
const WATCHDOG_RELOAD: u16 = 0x0FFF;
const FEED_INTERVAL_MS: u64 = 1_000;
const FEED_WINDOW_MS: u64 = 10_000;
const USB_DTR_POLL_MS: u64 = 100;
const POST_MESSAGE_SETTLE_MS: u64 = 250;
const HEARTBEAT_HALF_PERIOD_MS: u64 = 125;

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

fn report_failure(output: &GPIOAOutput) -> ! {
    output.set_high().unwrap();
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::task]
async fn usb_task(mut device: UsbDevice<'static, USBDUsbDriver>) -> ! {
    device.run().await
}

async fn write_line(
    cdc: &mut CdcAcmClass<'static, USBDUsbDriver>,
    line: &str,
) -> Result<(), embassy_usb::driver::EndpointError> {
    cdc.write_packet(line.as_bytes()).await
}

async fn wait_for_active_host(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>) {
    cdc.wait_connection().await;
    while !cdc.dtr() {
        Timer::after_millis(USB_DTR_POLL_MS).await;
    }
}

async fn run_watchdog_sequence(
    cdc: &mut CdcAcmClass<'static, USBDUsbDriver>,
    heartbeat: &GPIOAOutput,
) -> ! {
    let watchdog = &mut IWDG::new(DRV_IWDG_RESOURCES).unwrap();
    let config = IWDGConfig::new(WATCHDOG_PRESCALER, WATCHDOG_RELOAD);

    let mut line: String<128> = String::new();
    let _ = write!(
        line,
        "armed prescaler={} reload={} feed_window_ms={}\r\n",
        WATCHDOG_PRESCALER, WATCHDOG_RELOAD, FEED_WINDOW_MS
    );
    if write_line(cdc, line.as_str()).await.is_err() {
        report_failure(heartbeat);
    }
    Timer::after_millis(POST_MESSAGE_SETTLE_MS).await;

    watchdog.start(config);

    let iterations = FEED_WINDOW_MS / FEED_INTERVAL_MS;
    let mut heartbeat_high = false;
    for second in 0..iterations {
        let prescaler = watchdog.read_prescaler().unwrap();
        let reload = watchdog.read_reload().unwrap();
        let pending = watchdog.is_configuration_update_pending().unwrap();
        let pvu = watchdog.is_prescaler_update_pending().unwrap();
        let rvu = watchdog.is_reload_update_pending().unwrap();

        let mut line: String<128> = String::new();
        let _ = write!(
            line,
            "tick={} prescaler={} reload={} pending={} pvu={} rvu={} feeding=1\r\n",
            second, prescaler, reload, pending as u8, pvu as u8, rvu as u8
        );
        if write_line(cdc, line.as_str()).await.is_err() {
            report_failure(heartbeat);
        }

        Watchdog::feed(watchdog);
        if heartbeat_high {
            heartbeat.set_low().unwrap();
        } else {
            heartbeat.set_high().unwrap();
        }
        heartbeat_high = !heartbeat_high;
        Timer::after_millis(FEED_INTERVAL_MS).await;
    }

    heartbeat.set_high().unwrap();
    if write_line(cdc, "stopping feed; expecting watchdog reset\r\n")
        .await
        .is_err()
    {
        report_failure(heartbeat);
    }
    Timer::after_millis(POST_MESSAGE_SETTLE_MS).await;

    loop {
        Timer::after_millis(HEARTBEAT_HALF_PERIOD_MS).await;
    }
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    let rcc = RCC::new(DRV_RCC_RESOURCES).unwrap();
    rcc.configure_usb_fsdev_clock_48mhz().unwrap();

    let gpioa = GPIOA::new(DRV_GPIOA_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();
    let heartbeat = gpioa.pa7().into_output(Level::Low).unwrap();
    heartbeat.set_low().unwrap();

    wch::init_embassy_time_runtime().unwrap();

    let usbd = USBD::new(DRV_USBD_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();

    let mut config = Config::new(0xCAFE, 0x4006);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 Watchdog Smoke");
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

    wait_for_active_host(&mut cdc).await;
    if write_line(&mut cdc, "host connected; waiting to arm watchdog run\r\n")
        .await
        .is_err()
    {
        report_failure(&heartbeat);
    }
    Timer::after_millis(POST_MESSAGE_SETTLE_MS).await;

    run_watchdog_sequence(&mut cdc, &heartbeat).await
}
