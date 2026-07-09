#![no_std]
#![no_main]

use esp_hal as _;
use esp32c3fn4_generated::{
    gpio::{DRV_GPIO_RESOURCES, GPIOPort, GPIOPortOutput, Level},
    time::{DRV_TIME_RESOURCES, SystemTimer},
    usb::{DRV_USB_DEVICE_RESOURCES, UsbSerialJtag},
};
use panic_halt as _;

const TICKS_PER_SECOND: u64 = 16_000_000;
const SHORT_DELAY_TICKS: u64 = TICKS_PER_SECOND / 10;
const LONG_DELAY_TICKS: u64 = TICKS_PER_SECOND;
const USB_READY_POLL_TICKS: u64 = TICKS_PER_SECOND / 100;
const USB_READY_MAX_POLLS: usize = 200;

esp_bootloader_esp_idf::esp_app_desc!();

fn delay(timer: &SystemTimer, ticks: u64) {
    timer.delay_ticks(ticks).unwrap();
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

fn init_time() -> SystemTimer {
    let timer = SystemTimer::new(DRV_TIME_RESOURCES).unwrap();
    timer.init_time_driver().unwrap();
    timer
}

fn blink_code(timer: &SystemTimer, led: &GPIOPortOutput, pulses: usize) {
    for _ in 0..pulses {
        led.set_low().unwrap();
        delay(timer, SHORT_DELAY_TICKS);
        led.set_high().unwrap();
        delay(timer, SHORT_DELAY_TICKS);
    }
    delay(timer, LONG_DELAY_TICKS / 4);
}

fn blink_forever(timer: &SystemTimer, led: &GPIOPortOutput, pulses: usize) -> ! {
    loop {
        blink_code(timer, led, pulses);
        delay(timer, LONG_DELAY_TICKS / 2);
    }
}

fn wait_for_usb_ready(timer: &SystemTimer, usb: &UsbSerialJtag) -> bool {
    for _ in 0..USB_READY_MAX_POLLS {
        if usb.serial_in_ready().unwrap_or(false) {
            return true;
        }
        delay(timer, USB_READY_POLL_TICKS);
    }
    false
}

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let timer = init_time();
    let led = init_status_led();
    blink_code(&timer, &led, 1);

    let usb = init_usb_serial_jtag();
    blink_code(&timer, &led, 2);

    if !wait_for_usb_ready(&timer, &usb) {
        blink_forever(&timer, &led, 5);
    }
    blink_code(&timer, &led, 3);

    note(&usb, "ESP32-C3 USB Serial/JTAG smoke start\r\n");
    blink_code(&timer, &led, 4);

    loop {
        led.set_low().unwrap();
        note(&usb, "Hello World over USB Serial/JTAG\r\n");
        led.set_high().unwrap();
        delay(&timer, LONG_DELAY_TICKS);
    }
}
