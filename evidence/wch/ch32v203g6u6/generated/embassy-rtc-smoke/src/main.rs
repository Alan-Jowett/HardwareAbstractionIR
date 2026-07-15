#![no_std]
#![no_main]

use core::{fmt::Write as _, panic::PanicInfo, ptr::addr_of_mut};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOA_RESOURCES, GPIOA, GPIOAOutput, Level},
    rcc::{DRV_RCC_RESOURCES, RCC},
    rtc::{DRV_RTC_RESOURCES, RTC},
    usb::{DRV_USBD_RESOURCES, USBD, USBDUsbDriver},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};
use heapless::String;

const HEARTBEAT_HALF_PERIOD_MS: u64 = 250;
const USB_REPORT_PERIOD_MS: u64 = 1000;
const RTC_EXPECTED_PRESCALER: u32 = 39;

static mut CONFIG_DESCRIPTOR: [u8; 256] = [0; 256];
static mut BOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut MSOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut CONTROL_BUFFER: [u8; 128] = [0; 128];
static mut CDC_STATE: CdcState<'static> = CdcState::new();

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

#[embassy_executor::task]
async fn heartbeat_task(output: GPIOAOutput) -> ! {
    let mut high = true;
    loop {
        Timer::after_millis(HEARTBEAT_HALF_PERIOD_MS).await;
        if high {
            output.set_low().unwrap();
        } else {
            output.set_high().unwrap();
        }
        high = !high;
    }
}

#[embassy_executor::task]
async fn usb_logger_task(mut cdc: CdcAcmClass<'static, USBDUsbDriver>) -> ! {
    let rtc = RTC::new(DRV_RTC_RESOURCES).unwrap();
    let mut await_count: u32 = 0;
    loop {
        cdc.wait_connection().await;
        loop {
            Timer::after_millis(USB_REPORT_PERIOD_MS).await;
            await_count = await_count.wrapping_add(1);
            let now = rtc.read_counter().unwrap();
            let mut line: String<48> = String::new();
            let _ = write!(line, "await_count={} count={}\r\n", await_count, now);
            if cdc.write_packet(line.as_bytes()).await.is_err() {
                break;
            }
        }
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

    // Hold high during bring-up so a permanently high line indicates startup
    // failed before the await-driven heartbeat began toggling.
    heartbeat.set_high().unwrap();

    wch::init_embassy_time_runtime().unwrap();

    let rtc = RTC::new(DRV_RTC_RESOURCES).unwrap();
    if rtc.read_prescaler().unwrap() != RTC_EXPECTED_PRESCALER {
        report_failure(&heartbeat);
    }

    let usbd = USBD::new(DRV_USBD_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();

    let mut config = Config::new(0xCAFE, 0x4005);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 RTC Await Smoke");
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
    let cdc = CdcAcmClass::new(&mut builder, cdc_state, 64);
    let usb = builder.build();

    spawner.spawn(usb_task(usb)).unwrap();
    spawner.spawn(heartbeat_task(heartbeat)).unwrap();
    spawner.spawn(usb_logger_task(cdc)).unwrap();

    loop {
        Timer::after_millis(60_000).await;
    }
}
