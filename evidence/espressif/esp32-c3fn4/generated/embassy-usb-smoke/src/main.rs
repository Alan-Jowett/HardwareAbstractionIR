#![no_std]
#![no_main]

use embassy_executor::{Executor, task};
use embassy_time::{Duration, Timer};
use esp_hal as _;
use esp32c3fn4_generated::{
    gpio::{DRV_GPIO_RESOURCES, GPIOPort, GPIOPortOutput, Level},
    time::{DRV_TIME_RESOURCES, SystemTimer, generated_drv_time_time_driver_interrupt},
    usb::{DRV_USB_DEVICE_RESOURCES, UsbSerialJtag},
};
use panic_halt as _;
use static_cell::StaticCell;

const USB_READY_POLL_INTERVAL: Duration = Duration::from_millis(10);
const HEARTBEAT_ON_INTERVAL: Duration = Duration::from_millis(100);
const HEARTBEAT_OFF_INTERVAL: Duration = Duration::from_millis(400);
const HELLO_INTERVAL: Duration = Duration::from_secs(1);

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn SYSTIMER_TARGET0() {
    generated_drv_time_time_driver_interrupt();
}

fn note(usb: &UsbSerialJtag, message: &str) {
    usb.write_serial_packet(message.as_bytes()).unwrap();
}

fn init_usb_serial_jtag() -> UsbSerialJtag {
    let usb = UsbSerialJtag::new(DRV_USB_DEVICE_RESOURCES).unwrap();
    usb.apply_preserve_serial_jtag_link().unwrap();
    usb
}

fn init_status_led() -> GPIOPortOutput {
    let gpio = GPIOPort::new(DRV_GPIO_RESOURCES).unwrap();
    gpio.gpio10().into_output(Level::High).unwrap()
}

fn init_time() {
    let timer = SystemTimer::new(DRV_TIME_RESOURCES).unwrap();
    timer.init_time_driver().unwrap();
    esp_hal::interrupt::enable(
        esp_hal::peripherals::Interrupt::SYSTIMER_TARGET0,
        esp_hal::interrupt::Priority::Priority1,
    );
}

async fn wait_for_usb_ready(usb: &UsbSerialJtag) {
    loop {
        if usb.serial_in_ready().unwrap_or(false) {
            return;
        }
        Timer::after(USB_READY_POLL_INTERVAL).await;
    }
}

#[task]
async fn heartbeat_task(led: GPIOPortOutput) {
    loop {
        led.set_low().unwrap();
        Timer::after(HEARTBEAT_ON_INTERVAL).await;
        led.set_high().unwrap();
        Timer::after(HEARTBEAT_OFF_INTERVAL).await;
    }
}

#[task]
async fn usb_logger_task(usb: UsbSerialJtag) {
    wait_for_usb_ready(&usb).await;
    note(&usb, "ESP32-C3 USB Serial/JTAG smoke start\r\n");
    loop {
        note(&usb, "Hello World over USB Serial/JTAG\r\n");
        Timer::after(HELLO_INTERVAL).await;
    }
}

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    init_time();
    let usb = init_usb_serial_jtag();
    let led = init_status_led();

    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(heartbeat_task(led)).unwrap();
        spawner.spawn(usb_logger_task(usb)).unwrap();
    })
}
