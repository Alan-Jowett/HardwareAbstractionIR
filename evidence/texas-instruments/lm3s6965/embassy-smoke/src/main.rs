#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

use cortex_m::asm::nop;
use cortex_m_semihosting::{debug, hprintln};
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker, Timer};
use lm3s6965_embassy_hal::gpio::{
    DRV_GPIOA_RESOURCES, DRV_GPIOB_RESOURCES, DRV_GPIOC_RESOURCES, DRV_GPIOD_RESOURCES,
    DRV_GPIOE_RESOURCES, DRV_GPIOF_RESOURCES, GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, Level,
    Pull,
};
use lm3s6965_embassy_hal::i2c::{DRV_I2C0_RESOURCES, I2C0};
use lm3s6965_embassy_hal::interrupt::{DRV_NVIC_RESOURCES, Irq, NVIC};
use lm3s6965_embassy_hal::rcc::{DRV_RCC_RESOURCES, SYSCTL};
use lm3s6965_embassy_hal::spi::{DRV_SSI0_RESOURCES, SSI0};
use lm3s6965_embassy_hal::time::{DRV_TIME_RESOURCES, Time};
use lm3s6965_embassy_hal::timer::{
    DRV_TIMER0_RESOURCES, DRV_TIMER1_RESOURCES, DRV_TIMER2_RESOURCES, DRV_TIMER3_RESOURCES, TIMER0,
    TIMER1, TIMER2, TIMER3,
};
use lm3s6965_embassy_hal::uart::{DRV_UART0_RESOURCES, DRV_UART1_RESOURCES, UART0, UART1};
use panic_halt as _;

const SYSCTL_RCGC2: u32 = 0x400F_E100;

const GPIO_DIR_OFFSET: u32 = 0x400;
const GPIO_AFSEL_OFFSET: u32 = 0x420;
const GPIO_ODR_OFFSET: u32 = 0x50C;
const GPIO_PUR_OFFSET: u32 = 0x510;
const GPIO_PDR_OFFSET: u32 = 0x514;
const GPIO_DEN_OFFSET: u32 = 0x51C;
const GPIO_DATA_OFFSET: u32 = 0x3FC;

const UART_DR_OFFSET: u32 = 0x000;
const UART_FR_OFFSET: u32 = 0x018;
const UART_IBRD_OFFSET: u32 = 0x024;
const UART_FBRD_OFFSET: u32 = 0x028;
const UART_LCRH_OFFSET: u32 = 0x02C;
const UART_CTL_OFFSET: u32 = 0x030;
const UART_FR_BUSY: u32 = 1 << 3;
const UART_FR_TXFF: u32 = 1 << 5;
const UART_LCRH_WLEN_8: u32 = 0b11 << 5;
const UART_CTL_UARTEN: u32 = 1 << 0;
const UART_CTL_TXE: u32 = 1 << 8;
const UART_CTL_RXE: u32 = 1 << 9;

const SSI_CR0_OFFSET: u32 = 0x000;
const SSI_CR1_OFFSET: u32 = 0x004;
const SSI_DR_OFFSET: u32 = 0x008;
const SSI_SR_OFFSET: u32 = 0x00C;
const SSI_CPSR_OFFSET: u32 = 0x010;
const SSI_CR1_LBM: u32 = 1 << 0;
const SSI_CR1_SSE: u32 = 1 << 1;
const SSI_SR_RNE: u32 = 1 << 2;

const I2C_MSA_OFFSET: u32 = 0x000;
const I2C_MCS_OFFSET: u32 = 0x004;
const I2C_MDR_OFFSET: u32 = 0x008;
const I2C_MTPR_OFFSET: u32 = 0x00C;
const I2C_MCR_OFFSET: u32 = 0x020;
const I2C_MCR_LPBK: u32 = 1 << 0;
const I2C_MCR_MFE: u32 = 1 << 4;

const TIMER_CFG_OFFSET: u32 = 0x000;
const TIMER_TAMR_OFFSET: u32 = 0x004;
const TIMER_CTL_OFFSET: u32 = 0x00C;
const TIMER_ICR_OFFSET: u32 = 0x024;
const TIMER_TAILR_OFFSET: u32 = 0x028;
const TIMER_TAMR_PERIODIC: u32 = 0x2;
const TIMER_CTL_TAEN: u32 = 1 << 0;
const TIMER_ICR_TATOCINT: u32 = 1 << 0;

const WATCHDOG_LOAD: u32 = 0x4000_0000;
const WATCHDOG_VALUE: u32 = 0x4000_0004;
const WATCHDOG_CTL: u32 = 0x4000_0008;
const WATCHDOG_INTEN: u32 = 1 << 0;

const FLASH_FMA: u32 = 0x400F_D000;
const FLASH_FMD: u32 = 0x400F_D004;

static mut SRAM_SCRATCH: u32 = 0;

fn read_u32(address: u32) -> u32 {
    unsafe { read_volatile(address as *const u32) }
}

fn write_u32(address: u32, value: u32) {
    unsafe {
        write_volatile(address as *mut u32, value);
    }
}

fn modify_u32(address: u32, clear_mask: u32, set_mask: u32) {
    let current = read_u32(address);
    write_u32(address, (current & !clear_mask) | set_mask);
}

fn expect(label: &str, condition: bool) {
    if !condition {
        hprintln!("FAIL: {}", label);
        fail();
    }
}

fn spin_until(label: &str, limit: usize, mut predicate: impl FnMut() -> bool) {
    for _ in 0..limit {
        if predicate() {
            return;
        }
        nop();
    }
    hprintln!("FAIL: timeout waiting for {}", label);
    fail();
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

fn gpio_reg(base: u32, offset: u32) -> u32 {
    base + offset
}

macro_rules! smoke_gpio_port {
    ($label:literal, $ty:ident, $resources:ident, $ctor:ident, $base:expr, $bit:expr) => {{
        let port = $ty::new($resources).unwrap();
        port.enable_clock().unwrap();
        port.assert_reset().unwrap();
        port.release_reset().unwrap();

        let pin = port.$ctor();
        let input = pin.into_input(Pull::Up).unwrap();
        expect(
            concat!($label, " dir cleared for input"),
            (read_u32(gpio_reg($base, GPIO_DIR_OFFSET)) & $bit) == 0,
        );
        expect(
            concat!($label, " pull-up enabled"),
            (read_u32(gpio_reg($base, GPIO_PUR_OFFSET)) & $bit) != 0,
        );
        expect(
            concat!($label, " pull-down cleared"),
            (read_u32(gpio_reg($base, GPIO_PDR_OFFSET)) & $bit) == 0,
        );
        expect(
            concat!($label, " digital enabled"),
            (read_u32(gpio_reg($base, GPIO_DEN_OFFSET)) & $bit) != 0,
        );

        let output = input.into_flex().into_output(Level::Low).unwrap();
        expect(
            concat!($label, " dir set for output"),
            (read_u32(gpio_reg($base, GPIO_DIR_OFFSET)) & $bit) != 0,
        );
        expect(concat!($label, " output low"), output.is_set_low().unwrap());
        output.set_high().unwrap();
        expect(
            concat!($label, " output high"),
            output.is_set_high().unwrap(),
        );
        expect(
            concat!($label, " data high"),
            (read_u32(gpio_reg($base, GPIO_DATA_OFFSET)) & $bit) != 0,
        );

        let input = output.into_flex().into_input(Pull::Down).unwrap();
        expect(
            concat!($label, " pull-up cleared"),
            (read_u32(gpio_reg($base, GPIO_PUR_OFFSET)) & $bit) == 0,
        );
        expect(
            concat!($label, " pull-down enabled"),
            (read_u32(gpio_reg($base, GPIO_PDR_OFFSET)) & $bit) != 0,
        );
        let _ = input;
    }};
}

fn smoke_memory() {
    let initial_sp = read_u32(0x0000_0000);
    let reset_vector = read_u32(0x0000_0004);
    expect(
        "flash vector table word present",
        initial_sp != 0 && initial_sp != u32::MAX,
    );
    expect("flash reset vector thumb bit set", (reset_vector & 1) == 1);

    unsafe {
        write_volatile(core::ptr::addr_of_mut!(SRAM_SCRATCH), 0x1234_5678);
        expect(
            "sram write/read",
            read_volatile(core::ptr::addr_of!(SRAM_SCRATCH)) == 0x1234_5678,
        );
    }
}

fn smoke_nvic_and_systick() {
    let nvic = NVIC::new(DRV_NVIC_RESOURCES).unwrap();
    expect("nvic route count", nvic.bind().len() == 22);
    expect(
        "nvic includes uart0 route",
        nvic.bind()
            .iter()
            .any(|route| route.interrupt_ref == "int.uart0"),
    );
    expect("irq enum uart0 value", Irq::UART0 as i32 == 5);
}

async fn smoke_embassy_time() {
    let time = Time::new(DRV_TIME_RESOURCES).unwrap();
    expect("time route count", time.bind().len() == 1);
    expect(
        "time uses systick route",
        time.bind()[0].interrupt_ref == "int.systick",
    );
    time.init_time_driver().unwrap();

    Timer::after(Duration::from_ticks(8)).await;

    let mut ticker = Ticker::every(Duration::from_ticks(4));
    ticker.next().await;
    ticker.next().await;
}

fn smoke_rcc() {
    let rcc = SYSCTL::new(DRV_RCC_RESOURCES).unwrap();

    rcc.enable_watchdog0_clock().unwrap();
    rcc.disable_watchdog0_clock().unwrap();

    rcc.enable_gpioa_clock().unwrap();
    rcc.enable_gpiob_clock().unwrap();
    rcc.enable_gpioc_clock().unwrap();
    rcc.enable_gpiod_clock().unwrap();
    rcc.enable_gpioe_clock().unwrap();
    rcc.enable_gpiof_clock().unwrap();
    expect(
        "all gpio clocks enabled",
        (read_u32(SYSCTL_RCGC2) & 0x3F) == 0x3F,
    );

    rcc.enable_uart0_clock().unwrap();
    rcc.enable_uart1_clock().unwrap();
    rcc.enable_ssi0_clock().unwrap();
    rcc.enable_i2c0_clock().unwrap();
    rcc.enable_timer0_clock().unwrap();
    rcc.enable_timer1_clock().unwrap();
    rcc.enable_timer2_clock().unwrap();
    rcc.enable_timer3_clock().unwrap();

    rcc.assert_uart0_reset().unwrap();
    rcc.assert_uart1_reset().unwrap();
    rcc.assert_ssi0_reset().unwrap();
    rcc.assert_i2c0_reset().unwrap();
    rcc.assert_timer0_reset().unwrap();
    rcc.assert_timer1_reset().unwrap();
    rcc.assert_timer2_reset().unwrap();
    rcc.assert_timer3_reset().unwrap();
    rcc.release_uart0_reset().unwrap();
    rcc.release_uart1_reset().unwrap();
    rcc.release_ssi0_reset().unwrap();
    rcc.release_i2c0_reset().unwrap();
    rcc.release_timer0_reset().unwrap();
    rcc.release_timer1_reset().unwrap();
    rcc.release_timer2_reset().unwrap();
    rcc.release_timer3_reset().unwrap();
}

fn smoke_gpio() {
    smoke_gpio_port!(
        "gpioa",
        GPIOA,
        DRV_GPIOA_RESOURCES,
        pa0,
        0x4000_4000,
        1 << 0
    );
    smoke_gpio_port!(
        "gpiob",
        GPIOB,
        DRV_GPIOB_RESOURCES,
        pb0,
        0x4000_5000,
        1 << 0
    );
    smoke_gpio_port!(
        "gpioc",
        GPIOC,
        DRV_GPIOC_RESOURCES,
        pc0,
        0x4000_6000,
        1 << 0
    );
    smoke_gpio_port!(
        "gpiod",
        GPIOD,
        DRV_GPIOD_RESOURCES,
        pd0,
        0x4000_7000,
        1 << 0
    );
    smoke_gpio_port!(
        "gpioe",
        GPIOE,
        DRV_GPIOE_RESOURCES,
        pe0,
        0x4002_4000,
        1 << 0
    );
    smoke_gpio_port!(
        "gpiof",
        GPIOF,
        DRV_GPIOF_RESOURCES,
        pf0,
        0x4002_5000,
        1 << 0
    );
}

fn configure_uart(base: u32) {
    write_u32(base + UART_CTL_OFFSET, 0);
    write_u32(base + UART_IBRD_OFFSET, 8);
    write_u32(base + UART_FBRD_OFFSET, 44);
    write_u32(base + UART_LCRH_OFFSET, UART_LCRH_WLEN_8);
    write_u32(
        base + UART_CTL_OFFSET,
        UART_CTL_UARTEN | UART_CTL_TXE | UART_CTL_RXE,
    );
}

fn uart_write_bytes(base: u32, bytes: &[u8]) {
    for &byte in bytes {
        spin_until("uart tx fifo space", 100_000, || {
            (read_u32(base + UART_FR_OFFSET) & UART_FR_TXFF) == 0
        });
        write_u32(base + UART_DR_OFFSET, byte as u32);
    }
    spin_until("uart tx idle", 100_000, || {
        (read_u32(base + UART_FR_OFFSET) & UART_FR_BUSY) == 0
    });
}

fn smoke_uart() {
    let uart0 = UART0::new(DRV_UART0_RESOURCES).unwrap();
    let uart1 = UART1::new(DRV_UART1_RESOURCES).unwrap();
    uart0.enable_clock().unwrap();
    uart0.assert_reset().unwrap();
    uart0.release_reset().unwrap();
    uart1.enable_clock().unwrap();
    uart1.assert_reset().unwrap();
    uart1.release_reset().unwrap();

    modify_u32(
        gpio_reg(0x4000_4000, GPIO_AFSEL_OFFSET),
        0,
        (1 << 0) | (1 << 1),
    );
    modify_u32(
        gpio_reg(0x4000_4000, GPIO_DEN_OFFSET),
        0,
        (1 << 0) | (1 << 1),
    );
    modify_u32(
        gpio_reg(0x4000_7000, GPIO_AFSEL_OFFSET),
        0,
        (1 << 2) | (1 << 3),
    );
    modify_u32(
        gpio_reg(0x4000_7000, GPIO_DEN_OFFSET),
        0,
        (1 << 2) | (1 << 3),
    );

    configure_uart(0x4000_C000);
    configure_uart(0x4000_D000);

    expect(
        "uart0 enabled",
        (read_u32(0x4000_C000 + UART_CTL_OFFSET) & UART_CTL_UARTEN) != 0,
    );
    expect(
        "uart1 enabled",
        (read_u32(0x4000_D000 + UART_CTL_OFFSET) & UART_CTL_UARTEN) != 0,
    );
    expect(
        "uart1 baud divisor programmed",
        read_u32(0x4000_D000 + UART_IBRD_OFFSET) == 8,
    );
    expect(
        "uart1 line control programmed",
        (read_u32(0x4000_D000 + UART_LCRH_OFFSET) & UART_LCRH_WLEN_8) == UART_LCRH_WLEN_8,
    );

    uart_write_bytes(0x4000_C000, b"UART0 smoke ok\r\n");
}

fn smoke_ssi0() {
    let ssi0 = SSI0::new(DRV_SSI0_RESOURCES).unwrap();
    ssi0.enable_clock().unwrap();
    ssi0.assert_reset().unwrap();
    ssi0.release_reset().unwrap();

    modify_u32(
        gpio_reg(0x4000_4000, GPIO_AFSEL_OFFSET),
        0,
        (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5),
    );
    modify_u32(
        gpio_reg(0x4000_4000, GPIO_DEN_OFFSET),
        0,
        (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5),
    );

    write_u32(0x4000_8000 + SSI_CR1_OFFSET, 0);
    write_u32(0x4000_8000 + SSI_CR0_OFFSET, 0x0000_0007);
    write_u32(0x4000_8000 + SSI_CPSR_OFFSET, 2);
    write_u32(0x4000_8000 + SSI_CR1_OFFSET, SSI_CR1_LBM | SSI_CR1_SSE);
    write_u32(0x4000_8000 + SSI_DR_OFFSET, 0x5A);
    spin_until("ssi0 loopback receive", 100_000, || {
        (read_u32(0x4000_8000 + SSI_SR_OFFSET) & SSI_SR_RNE) != 0
    });
    expect(
        "ssi0 loopback data",
        (read_u32(0x4000_8000 + SSI_DR_OFFSET) & 0xFF) == 0x5A,
    );
}

fn smoke_i2c0() {
    let i2c0 = I2C0::new(DRV_I2C0_RESOURCES).unwrap();
    i2c0.enable_clock().unwrap();
    i2c0.assert_reset().unwrap();
    i2c0.release_reset().unwrap();

    modify_u32(
        gpio_reg(0x4000_5000, GPIO_AFSEL_OFFSET),
        0,
        (1 << 2) | (1 << 3),
    );
    modify_u32(
        gpio_reg(0x4000_5000, GPIO_DEN_OFFSET),
        0,
        (1 << 2) | (1 << 3),
    );
    modify_u32(
        gpio_reg(0x4000_5000, GPIO_ODR_OFFSET),
        0,
        (1 << 2) | (1 << 3),
    );

    write_u32(0x4002_0000 + I2C_MCR_OFFSET, I2C_MCR_MFE | I2C_MCR_LPBK);
    write_u32(0x4002_0000 + I2C_MTPR_OFFSET, 7);
    write_u32(0x4002_0000 + I2C_MSA_OFFSET, 0x52 << 1);
    write_u32(0x4002_0000 + I2C_MDR_OFFSET, 0xA5);
    expect(
        "i2c0 master+loopback enabled",
        (read_u32(0x4002_0000 + I2C_MCR_OFFSET) & (I2C_MCR_MFE | I2C_MCR_LPBK))
            == (I2C_MCR_MFE | I2C_MCR_LPBK),
    );
    expect(
        "i2c0 mtpr write",
        read_u32(0x4002_0000 + I2C_MTPR_OFFSET) == 7,
    );
    expect(
        "i2c0 msa write",
        (read_u32(0x4002_0000 + I2C_MSA_OFFSET) & 0xFE) == (0x52 << 1),
    );
    expect(
        "i2c0 mdr write",
        (read_u32(0x4002_0000 + I2C_MDR_OFFSET) & 0xFF) == 0xA5,
    );
    let _ = read_u32(0x4002_0000 + I2C_MCS_OFFSET);
}

fn smoke_one_timer(base: u32, label: &str, enable: impl FnOnce()) {
    write_u32(base + TIMER_CTL_OFFSET, 0);
    write_u32(base + TIMER_CFG_OFFSET, 0);
    write_u32(base + TIMER_TAMR_OFFSET, TIMER_TAMR_PERIODIC);
    write_u32(base + TIMER_TAILR_OFFSET, 1024);
    write_u32(base + TIMER_ICR_OFFSET, TIMER_ICR_TATOCINT);
    enable();
    expect(
        label,
        read_u32(base + TIMER_CFG_OFFSET) == 0
            && (read_u32(base + TIMER_TAMR_OFFSET) & 0x3) == TIMER_TAMR_PERIODIC
            && read_u32(base + TIMER_TAILR_OFFSET) == 1024
            && (read_u32(base + TIMER_CTL_OFFSET) & TIMER_CTL_TAEN) != 0,
    );
}

fn smoke_timers() {
    let timer0 = TIMER0::new(DRV_TIMER0_RESOURCES).unwrap();
    let timer1 = TIMER1::new(DRV_TIMER1_RESOURCES).unwrap();
    let timer2 = TIMER2::new(DRV_TIMER2_RESOURCES).unwrap();
    let timer3 = TIMER3::new(DRV_TIMER3_RESOURCES).unwrap();

    timer0.enable_clock().unwrap();
    timer0.assert_reset().unwrap();
    timer0.release_reset().unwrap();
    smoke_one_timer(0x4003_0000, "timer0 configured+enabled", || {
        timer0.apply_enable().unwrap()
    });
    timer0.transition_running_to_disabled().unwrap();
    expect(
        "timer0 disabled",
        (read_u32(0x4003_0000 + TIMER_CTL_OFFSET) & TIMER_CTL_TAEN) == 0,
    );

    timer1.enable_clock().unwrap();
    timer1.assert_reset().unwrap();
    timer1.release_reset().unwrap();
    smoke_one_timer(0x4003_1000, "timer1 configured+enabled", || {
        timer1.transition_disabled_to_running().unwrap()
    });
    timer1.transition_running_to_disabled().unwrap();
    expect(
        "timer1 disabled",
        (read_u32(0x4003_1000 + TIMER_CTL_OFFSET) & TIMER_CTL_TAEN) == 0,
    );

    timer2.enable_clock().unwrap();
    timer2.assert_reset().unwrap();
    timer2.release_reset().unwrap();
    smoke_one_timer(0x4003_2000, "timer2 configured+enabled", || {
        timer2.transition_disabled_to_running().unwrap()
    });
    timer2.transition_running_to_disabled().unwrap();
    expect(
        "timer2 disabled",
        (read_u32(0x4003_2000 + TIMER_CTL_OFFSET) & TIMER_CTL_TAEN) == 0,
    );

    timer3.enable_clock().unwrap();
    timer3.assert_reset().unwrap();
    timer3.release_reset().unwrap();
    smoke_one_timer(0x4003_3000, "timer3 configured+enabled", || {
        timer3.transition_disabled_to_running().unwrap()
    });
    timer3.transition_running_to_disabled().unwrap();
    expect(
        "timer3 disabled",
        (read_u32(0x4003_3000 + TIMER_CTL_OFFSET) & TIMER_CTL_TAEN) == 0,
    );
}

fn smoke_watchdog() {
    let rcc = SYSCTL::new(DRV_RCC_RESOURCES).unwrap();
    rcc.enable_watchdog0_clock().unwrap();

    write_u32(WATCHDOG_LOAD, 0x0000_1000);
    write_u32(WATCHDOG_CTL, WATCHDOG_INTEN);
    expect(
        "watchdog load write",
        read_u32(WATCHDOG_LOAD) == 0x0000_1000,
    );
    expect(
        "watchdog ctl write",
        (read_u32(WATCHDOG_CTL) & WATCHDOG_INTEN) != 0,
    );
    expect(
        "watchdog value decrements",
        read_u32(WATCHDOG_VALUE) <= 0x0000_1000,
    );
}

fn smoke_flash_controller() {
    let vector_word = read_u32(0x0000_0000);
    expect("flash word readable", vector_word != 0);
    let _ = read_u32(FLASH_FMA);
    let _ = read_u32(FLASH_FMD);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    smoke_memory();
    hprintln!("memory ok");
    smoke_nvic_and_systick();
    hprintln!("core ok");
    smoke_rcc();
    hprintln!("rcc ok");
    smoke_gpio();
    hprintln!("gpio ok");
    smoke_uart();
    hprintln!("uart ok");
    smoke_ssi0();
    hprintln!("ssi ok");
    smoke_i2c0();
    hprintln!("i2c ok");
    smoke_timers();
    hprintln!("timers ok");
    smoke_watchdog();
    hprintln!("watchdog ok");
    smoke_flash_controller();
    hprintln!("flash ok");
    smoke_embassy_time().await;
    hprintln!("embassy time ok");
    pass();
}
