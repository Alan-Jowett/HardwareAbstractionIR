#![no_std]
#![no_main]

use core::panic::PanicInfo;

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOB_RUNTIME_RESOURCES, GPIOB},
    i2c::{DRV_I2C1_RUNTIME_RESOURCES, I2C1},
    rcc::{DRV_RCC_RUNTIME_RESOURCES, RCC},
    wch,
};
use embassy_time::{Duration, Timer};
use embedded_hal::i2c::I2c;

const SLAVE_ADDRESS: u8 = 0x42;
const REQUEST: [u8; 3] = [0x10, 0x20, 0x30];
const RETRY_INTERVAL: Duration = Duration::from_millis(500);

const GPIOB_CFGLR: u32 = 0x40010C00;
const GPIOB_BSHR: u32 = 0x40010C10;
const PB6_MODE_SHIFT: u32 = 24;
const PB7_MODE_SHIFT: u32 = 28;
const GPIO_ALT_OPEN_DRAIN_50MHZ: u32 = 0xF;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    let _rcc = RCC::new(DRV_RCC_RUNTIME_RESOURCES).unwrap();
    wch::init_embassy_time_runtime().unwrap();

    let gpiob = GPIOB::new(DRV_GPIOB_RUNTIME_RESOURCES).unwrap();
    gpiob.enable_clock().unwrap();
    gpiob.release_reset().unwrap();
    configure_board_i2c_pins();

    let mut i2c1 = I2C1::new(DRV_I2C1_RUNTIME_RESOURCES).unwrap();
    i2c1.enable_clock().unwrap();
    i2c1.release_reset().unwrap();
    i2c1.init_master().unwrap();

    loop {
        let _ = I2c::write(&mut i2c1, SLAVE_ADDRESS, &REQUEST);
        Timer::after(RETRY_INTERVAL).await;
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
