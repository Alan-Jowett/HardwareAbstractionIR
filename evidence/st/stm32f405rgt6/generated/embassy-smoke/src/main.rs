#![no_std]
#![no_main]

use core::ptr::read_volatile;

use cortex_m::asm::nop;
use cortex_m_semihosting::debug;
use cortex_m_semihosting::hprintln;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker, Timer};
use panic_halt as _;
use stm32_h405_generated::gpio::{DRV_GPIOA_RUNTIME_RESOURCES, GpioA, Level, Pull};
use stm32_h405_generated::time::{DRV_TIME_RUNTIME_RESOURCES, Time};
use stm32_h405_generated::usart::{DRV_USART1_RUNTIME_RESOURCES, Usart1};

const BRR_MANTISSA_115200_AT_16MHZ: u16 = 8;
const BRR_FRACTION_115200_AT_16MHZ: u8 = 11;

const RCC_AHB1RSTR: *const u32 = 0x4002_3810 as *const u32;
const RCC_AHB1ENR: *const u32 = 0x4002_3830 as *const u32;

const GPIOA_ENABLE_BIT: u32 = 1 << 0;
const GPIOA_RESET_BIT: u32 = 1 << 0;

fn read_reg(address: *const u32) -> u32 {
    unsafe { read_volatile(address) }
}

fn note(message: &str) {
    hprintln!("{}", message);
}

fn expect(label: &str, condition: bool) {
    if !condition {
        fail(label);
    }
}

fn fail(label: &str) -> ! {
    note(label);
    debug::exit(debug::EXIT_FAILURE);
    loop {
        nop();
    }
}

fn pass() -> ! {
    note("PASS");
    debug::exit(debug::EXIT_SUCCESS);
    loop {
        nop();
    }
}

fn smoke_gpioa() {
    note("smoke_gpioa:start");
    let gpioa = GpioA::new(DRV_GPIOA_RUNTIME_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    expect(
        "FAIL: gpioa clock enable bit not set",
        (read_reg(RCC_AHB1ENR) & GPIOA_ENABLE_BIT) != 0,
    );
    gpioa.assert_reset().unwrap();
    expect(
        "FAIL: gpioa reset bit not asserted",
        (read_reg(RCC_AHB1RSTR) & GPIOA_RESET_BIT) != 0,
    );
    gpioa.release_reset().unwrap();
    expect(
        "FAIL: gpioa reset bit not released",
        (read_reg(RCC_AHB1RSTR) & GPIOA_RESET_BIT) == 0,
    );

    let pa0 = gpioa.pa0();
    let pa0_input = pa0.into_input(Pull::Up).unwrap();
    // QEMU's netduinoplus2 STM32 model does not provide reliable GPIO register
    // or pin-state readback for this target, so this smoke test exercises the
    // input/output APIs here without asserting those GPIO state transitions.

    let pa0_output = pa0_input.into_flex().into_output(Level::Low).unwrap();
    pa0_output.set_high().unwrap();
    pa0_output.set_low().unwrap();

    let _pa0_input = pa0_output.into_flex().into_input(Pull::Down).unwrap();
    note("smoke_gpioa:ok");
}

fn smoke_usart1() {
    note("smoke_usart1:start");
    let usart1 = Usart1::new(DRV_USART1_RUNTIME_RESOURCES).unwrap();
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
    note("smoke_usart1:ok");
}

async fn smoke_embassy_time() {
    note("smoke_embassy_time:start");
    let time = Time::new(DRV_TIME_RUNTIME_RESOURCES).unwrap();
    expect(
        "FAIL: embassy time binding count mismatch",
        Time::interrupt_routes().len() == 1,
    );
    time.init_time_driver().unwrap();

    Timer::after(Duration::from_ticks(8)).await;

    let mut ticker = Ticker::every(Duration::from_ticks(4));
    ticker.next().await;
    ticker.next().await;
    note("smoke_embassy_time:ok");
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    smoke_gpioa();
    smoke_usart1();
    smoke_embassy_time().await;
    pass();
}
