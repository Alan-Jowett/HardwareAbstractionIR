#![no_std]
#![no_main]

use core::hint::spin_loop;
use core::ptr::read_volatile;

use esp32c3fn4_generated::gpio::{DRV_GPIO_RESOURCES, GPIOPort, Level, Pull};
use esp32c3fn4_generated::interrupt::{DRV_IRQ_RESOURCES, InterruptMatrix};
use esp32c3fn4_generated::uart::{DRV_UART0_RESOURCES, Uart0};
use panic_halt as _;
use riscv_rt::entry;

const GPIO_OUT_REG: *const u32 = 0x6000_4004 as *const u32;
const GPIO8_MASK: u32 = 1 << 8;
const UART_INT_ENA_REG: *const u32 = 0x6000_000C as *const u32;
const UART_RXFIFO_FULL_INT_ENA: u32 = 1 << 0;
const UART_TXFIFO_EMPTY_INT_ENA: u32 = 1 << 1;

const UART_CLKDIV_80MHZ_115200_DIV: u16 = 694;
const UART_CLKDIV_80MHZ_115200_FRAG: u8 = 7;

fn read_reg(address: *const u32) -> u32 {
    unsafe { read_volatile(address) }
}

fn note(uart: &Uart0, message: &str) {
    uart.write_bytes(message.as_bytes()).unwrap();
    uart.flush().unwrap();
}

fn fail(uart: &Uart0, label: &str) -> ! {
    note(uart, "FAIL: ");
    note(uart, label);
    note(uart, "\r\n");
    loop {
        spin_loop();
    }
}

fn expect(uart: &Uart0, label: &str, condition: bool) {
    if !condition {
        fail(uart, label);
    }
}

fn init_uart0() -> Uart0 {
    let uart = Uart0::new(DRV_UART0_RESOURCES).unwrap();
    uart.enable_clock().unwrap();
    uart.assert_reset().unwrap();
    uart.release_reset().unwrap();
    uart.configure_8n1().unwrap();
    uart.set_baud_divider(UART_CLKDIV_80MHZ_115200_DIV, UART_CLKDIV_80MHZ_115200_FRAG)
        .unwrap();
    uart.enable().unwrap();
    uart
}

fn smoke_gpio(uart: &Uart0) {
    note(uart, "smoke_gpio:start\r\n");
    let gpio = GPIOPort::new(DRV_GPIO_RESOURCES).unwrap();
    let pin = gpio.gpio8().into_output(Level::Low).unwrap();
    expect(
        uart,
        "gpio8 should start low",
        (read_reg(GPIO_OUT_REG) & GPIO8_MASK) == 0 && pin.is_set_low().unwrap(),
    );
    pin.set_high().unwrap();
    expect(
        uart,
        "gpio8 should go high",
        (read_reg(GPIO_OUT_REG) & GPIO8_MASK) != 0 && pin.is_set_high().unwrap(),
    );
    pin.set_low().unwrap();
    expect(
        uart,
        "gpio8 should return low",
        (read_reg(GPIO_OUT_REG) & GPIO8_MASK) == 0 && pin.is_set_low().unwrap(),
    );
    let input = pin.into_flex().into_input(Pull::Up).unwrap();
    input.set_pull(Pull::Down).unwrap();
    let _pin = input.into_flex();
    note(uart, "smoke_gpio:ok\r\n");
}

fn smoke_interrupts(uart: &Uart0) {
    note(uart, "smoke_interrupts:start\r\n");
    let interrupt = InterruptMatrix::new(DRV_IRQ_RESOURCES).unwrap();
    let routes = interrupt.bind();
    expect(uart, "interrupt route count", routes.len() == 12);
    expect(
        uart,
        "uart0 route missing",
        routes.iter().any(|route| route.id == "iroute.uart0"),
    );

    uart.enable_txe_interrupt().unwrap();
    expect(
        uart,
        "uart txe interrupt enable bit",
        (read_reg(UART_INT_ENA_REG) & UART_TXFIFO_EMPTY_INT_ENA) != 0,
    );
    uart.disable_txe_interrupt().unwrap();
    expect(
        uart,
        "uart txe interrupt disable bit",
        (read_reg(UART_INT_ENA_REG) & UART_TXFIFO_EMPTY_INT_ENA) == 0,
    );

    uart.enable_rxne_interrupt().unwrap();
    expect(
        uart,
        "uart rx interrupt enable bit",
        (read_reg(UART_INT_ENA_REG) & UART_RXFIFO_FULL_INT_ENA) != 0,
    );
    uart.disable_rxne_interrupt().unwrap();
    expect(
        uart,
        "uart rx interrupt disable bit",
        (read_reg(UART_INT_ENA_REG) & UART_RXFIFO_FULL_INT_ENA) == 0,
    );
    note(uart, "smoke_interrupts:ok\r\n");
}

#[entry]
fn main() -> ! {
    let uart = init_uart0();
    note(&uart, "ESP32-C3 HAL smoke start\r\n");
    smoke_gpio(&uart);
    smoke_interrupts(&uart);
    note(&uart, "PASS\r\n");
    loop {
        spin_loop();
    }
}
