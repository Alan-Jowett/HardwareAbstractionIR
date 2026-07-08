#![no_std]
#![no_main]

use core::hint::spin_loop;

use esp_hal as _;
use esp32c3fn4_generated::{
    gpio::{DRV_GPIO_RESOURCES, GPIOPort, GPIOPortOutput, Level},
    usb::{DRV_USB_DEVICE_RESOURCES, UsbSerialJtag},
};
use panic_halt as _;

const SHORT_DELAY_CYCLES: usize = 2_000_000;
const LONG_DELAY_CYCLES: usize = 20_000_000;
const USB_READY_POLL_CYCLES: usize = 50_000;
const USB_READY_MAX_POLLS: usize = 200;

esp_bootloader_esp_idf::esp_app_desc!();

fn delay(cycles: usize) {
    for _ in 0..cycles {
        spin_loop();
    }
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

fn blink_code(led: &GPIOPortOutput, pulses: usize) {
    for _ in 0..pulses {
        led.set_low().unwrap();
        delay(SHORT_DELAY_CYCLES);
        led.set_high().unwrap();
        delay(SHORT_DELAY_CYCLES);
    }
    delay(LONG_DELAY_CYCLES / 4);
}

fn blink_forever(led: &GPIOPortOutput, pulses: usize) -> ! {
    loop {
        blink_code(led, pulses);
        delay(LONG_DELAY_CYCLES / 2);
    }
}

fn wait_for_usb_ready(usb: &UsbSerialJtag) -> bool {
    for _ in 0..USB_READY_MAX_POLLS {
        if usb.serial_in_ready().unwrap_or(false) {
            return true;
        }
        delay(USB_READY_POLL_CYCLES);
    }
    false
}

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let led = init_status_led();
    blink_code(&led, 1);

    let usb = init_usb_serial_jtag();
    blink_code(&led, 2);

    if !wait_for_usb_ready(&usb) {
        blink_forever(&led, 5);
    }
    blink_code(&led, 3);

    note(&usb, "ESP32-C3 USB Serial/JTAG smoke start\r\n");
    blink_code(&led, 4);

    loop {
        led.set_low().unwrap();
        note(&usb, "Hello World over USB Serial/JTAG\r\n");
        led.set_high().unwrap();
        delay(LONG_DELAY_CYCLES);
    }
}
