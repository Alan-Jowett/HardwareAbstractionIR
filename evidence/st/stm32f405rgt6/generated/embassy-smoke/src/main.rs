#![no_std]
#![no_main]

use core::ptr::read_volatile;

use cortex_m::asm::nop;
use cortex_m_semihosting::debug;
use embassy_executor::Spawner;
use panic_halt as _;
use stm32_h405_generated::gpio::{DRV_GPIOA_RESOURCES, GpioA, Level, Pull};
use stm32_h405_generated::usart::{DRV_USART1_RESOURCES, Usart1};

const BRR_MANTISSA_115200_AT_16MHZ: u16 = 8;
const BRR_FRACTION_115200_AT_16MHZ: u8 = 11;

const RCC_AHB1RSTR: *const u32 = 0x4002_3810 as *const u32;
const RCC_AHB1ENR: *const u32 = 0x4002_3830 as *const u32;
const GPIOA_MODER: *const u32 = 0x4002_0000 as *const u32;
const GPIOA_PUPDR: *const u32 = 0x4002_000C as *const u32;
const GPIOA_ODR: *const u32 = 0x4002_0014 as *const u32;

const GPIOA_ENABLE_BIT: u32 = 1 << 0;
const GPIOA_RESET_BIT: u32 = 1 << 0;
const GPIOA_PIN0_MODE_MASK: u32 = 0b11;
const GPIOA_PIN0_MODE_OUTPUT: u32 = 0b01;
const GPIOA_PIN0_PUPDR_MASK: u32 = 0b11;
const GPIOA_PIN0_PUPDR_UP: u32 = 0b01;
const GPIOA_PIN0_PUPDR_DOWN: u32 = 0b10;
const GPIOA_PIN0_ODR_BIT: u32 = 1 << 0;

fn read_reg(address: *const u32) -> u32 {
    unsafe { read_volatile(address) }
}

fn expect(condition: bool) {
    if !condition {
        fail();
    }
}

fn fail() -> ! {
    debug::exit(debug::EXIT_FAILURE);
    loop {
        nop();
    }
}

fn pass() -> ! {
    debug::exit(debug::EXIT_SUCCESS);
    loop {
        nop();
    }
}

fn smoke_gpioa() {
    let gpioa = GpioA::new(DRV_GPIOA_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    expect((read_reg(RCC_AHB1ENR) & GPIOA_ENABLE_BIT) != 0);
    gpioa.assert_reset().unwrap();
    expect((read_reg(RCC_AHB1RSTR) & GPIOA_RESET_BIT) != 0);
    gpioa.release_reset().unwrap();
    expect((read_reg(RCC_AHB1RSTR) & GPIOA_RESET_BIT) == 0);

    let pa0 = gpioa.pa0();
    let pa0_input = pa0.into_input(Pull::Up).unwrap();
    expect((read_reg(GPIOA_MODER) & GPIOA_PIN0_MODE_MASK) == 0);
    expect((read_reg(GPIOA_PUPDR) & GPIOA_PIN0_PUPDR_MASK) == GPIOA_PIN0_PUPDR_UP);

    let pa0_output = pa0_input.into_flex().into_output(Level::Low).unwrap();
    expect((read_reg(GPIOA_MODER) & GPIOA_PIN0_MODE_MASK) == GPIOA_PIN0_MODE_OUTPUT);
    expect(pa0_output.is_set_low().unwrap());
    expect((read_reg(GPIOA_ODR) & GPIOA_PIN0_ODR_BIT) == 0);

    pa0_output.set_high().unwrap();
    expect(pa0_output.is_set_high().unwrap());
    expect(pa0_output.get_output_level().unwrap() == Level::High);
    expect((read_reg(GPIOA_ODR) & GPIOA_PIN0_ODR_BIT) != 0);

    let pa0_input = pa0_output.into_flex().into_input(Pull::Down).unwrap();
    expect((read_reg(GPIOA_MODER) & GPIOA_PIN0_MODE_MASK) == 0);
    expect((read_reg(GPIOA_PUPDR) & GPIOA_PIN0_PUPDR_MASK) == GPIOA_PIN0_PUPDR_DOWN);
    expect(pa0_input.is_low().unwrap());
}

fn smoke_usart1() {
    let usart1 = Usart1::new(DRV_USART1_RESOURCES).unwrap();
    usart1.enable_clock().unwrap();
    usart1.assert_reset().unwrap();
    usart1.release_reset().unwrap();
    usart1.configure_tx_pa9_route().unwrap();
    usart1.configure_rx_pa10_route().unwrap();
    usart1.configure_8n1().unwrap();
    usart1
        .set_baud_divider(BRR_MANTISSA_115200_AT_16MHZ, BRR_FRACTION_115200_AT_16MHZ)
        .unwrap();
    usart1.enable_transmitter().unwrap();
    usart1.enable().unwrap();
    usart1.write_bytes(b"Hello, USART1 from QEMU!\r\n").unwrap();
    usart1.flush().unwrap();
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    smoke_gpioa();
    smoke_usart1();
    pass();
}
