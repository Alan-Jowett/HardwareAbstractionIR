#![no_std]
#![no_main]

use core::ptr::read_volatile;

use cortex_m_semihosting::{debug, hprintln};
use embassy_executor::Spawner;
use panic_halt as _;
use stm32_h405_generated::usart::{DRV_USART3_RESOURCES, Usart3};

const RCC_APB1RSTR: *const u32 = 0x4002_3820 as *const u32;
const RCC_APB1ENR: *const u32 = 0x4002_3840 as *const u32;
const USART3_BIT: u32 = 1 << 18;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let usart3 = Usart3::new(DRV_USART3_RESOURCES).unwrap();

    let before_en = unsafe { read_volatile(RCC_APB1ENR) };
    let before_rst = unsafe { read_volatile(RCC_APB1RSTR) };
    hprintln!(
        "before apb1enr=0x{:08x} apb1rstr=0x{:08x}",
        before_en,
        before_rst
    );

    usart3.enable_clock().unwrap();
    usart3.assert_reset().unwrap();
    let mid_en = unsafe { read_volatile(RCC_APB1ENR) };
    let mid_rst = unsafe { read_volatile(RCC_APB1RSTR) };
    hprintln!("mid    apb1enr=0x{:08x} apb1rstr=0x{:08x}", mid_en, mid_rst);

    usart3.release_reset().unwrap();
    let after_en = unsafe { read_volatile(RCC_APB1ENR) };
    let after_rst = unsafe { read_volatile(RCC_APB1RSTR) };
    let pass = (after_en & USART3_BIT) != 0 && (after_rst & USART3_BIT) == 0;
    hprintln!(
        "after  apb1enr=0x{:08x} apb1rstr=0x{:08x} pass={}",
        after_en,
        after_rst,
        pass
    );

    if pass {
        debug::exit(debug::EXIT_SUCCESS);
    } else {
        debug::exit(debug::EXIT_FAILURE);
    }

    loop {
        cortex_m::asm::bkpt();
    }
}
