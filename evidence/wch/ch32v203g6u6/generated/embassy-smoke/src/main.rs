#![no_std]
#![no_main]

use core::panic::PanicInfo;

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOA_RESOURCES, GPIOA, Level},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    let gpioa = GPIOA::new(DRV_GPIOA_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();
    let led = gpioa.pa7().into_output(Level::Low).unwrap();
    wch::init_embassy_time_runtime().unwrap();
    loop {
        led.set_high().unwrap();
        Timer::after(Duration::from_secs(1)).await;
        led.set_low().unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
