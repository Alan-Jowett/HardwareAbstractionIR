#![no_std]
#![no_main]

use core::panic::PanicInfo;

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOA_RUNTIME_RESOURCES, GPIOA},
    pwm::{DRV_PWM_TIM3_RUNTIME_RESOURCES, TIM3PWM},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_hal::pwm::SetDutyCycle;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    let gpioa = GPIOA::new(DRV_GPIOA_RUNTIME_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();

    let tim3 = TIM3PWM::new(DRV_PWM_TIM3_RUNTIME_RESOURCES).unwrap();
    tim3.enable_clock().unwrap();
    tim3.release_reset().unwrap();
    tim3.enable_auto_reload_preload().unwrap();
    tim3.set_prescaler(79).unwrap();
    tim3.set_auto_reload(99).unwrap();
    tim3.set_counter(0).unwrap();
    tim3.configure_ch2_as_pwm_mode_1().unwrap();
    tim3.configure_ch2_pa7_as_pwm_output().unwrap();

    let mut ch2 = tim3.channel_ch2();
    ch2.set_duty_cycle_fully_off().unwrap();
    ch2.enable_output().unwrap();
    tim3.generate_update().unwrap();
    tim3.apply_enable().unwrap();

    wch::init_embassy_time_runtime().unwrap();

    loop {
        for percent in (0u8..=100).step_by(5) {
            ch2.set_duty_cycle_percent(percent).unwrap();
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}
