#![no_std]
#![no_main]

use core::hint::spin_loop;
use core::ptr::{read_volatile, write_volatile};

use ch32v203g6u6_embassy_hal::adc::{ADC1, DRV_ADC1_RESOURCES};
use ch32v203g6u6_embassy_hal::dma::{DMA1, DRV_DMA1_RESOURCES};
use ch32v203g6u6_embassy_hal::i2c::{DRV_I2C1_RESOURCES, I2C1};
use ch32v203g6u6_embassy_hal::interrupt::{DRV_PFIC_RESOURCES, PFIC};
use ch32v203g6u6_embassy_hal::spi::{DRV_SPI1_RESOURCES, SPI1};
use ch32v203g6u6_embassy_hal::timer::{DRV_TIM1_RESOURCES, TIM1};
use ch32v203g6u6_embassy_hal::uart::{DRV_USART1_RESOURCES, USART1};
use panic_halt as _;
use riscv_rt::entry;

const RCC_AHBPCENR: *const u32 = 0x4002_1014 as *const u32;
const RCC_APB2PRSTR: *const u32 = 0x4002_100C as *const u32;
const RCC_APB2PCENR: *const u32 = 0x4002_1018 as *const u32;
const RCC_APB1PRSTR: *const u32 = 0x4002_1010 as *const u32;
const RCC_APB1PCENR: *const u32 = 0x4002_101C as *const u32;
const TIM1_CTLR1: *const u16 = 0x4001_2C00 as *const u16;

const DMA1_ENABLE_BIT: u32 = 1 << 0;
const ADC1_ENABLE_BIT: u32 = 1 << 9;
const TIM1_ENABLE_BIT: u32 = 1 << 11;
const SPI1_ENABLE_BIT: u32 = 1 << 12;
const USART1_ENABLE_BIT: u32 = 1 << 14;
const I2C1_ENABLE_BIT: u32 = 1 << 21;

const ADC1_RESET_BIT: u32 = 1 << 9;
const TIM1_RESET_BIT: u32 = 1 << 11;
const SPI1_RESET_BIT: u32 = 1 << 12;
const USART1_RESET_BIT: u32 = 1 << 14;
const I2C1_RESET_BIT: u32 = 1 << 21;

fn read_reg32(address: *const u32) -> u32 {
    unsafe { read_volatile(address) }
}

fn read_reg16(address: *const u16) -> u16 {
    unsafe { read_volatile(address) }
}

fn fail() -> ! {
    unsafe {
        write_volatile(0x2000_0000 as *mut u32, 0xFA11_FA11);
    }
    loop {
        spin_loop();
    }
}

fn expect(condition: bool) {
    if !condition {
        fail();
    }
}

#[entry]
fn main() -> ! {
    let pfic = PFIC::new(DRV_PFIC_RESOURCES).unwrap();
    expect(!pfic.bind().is_empty());

    let dma1 = DMA1::new(DRV_DMA1_RESOURCES).unwrap();
    dma1.enable_clock().unwrap();
    expect((read_reg32(RCC_AHBPCENR) & DMA1_ENABLE_BIT) != 0);
    expect(dma1.resources().dma_channels.len() == 7);
    expect(!dma1.resources().dma.is_empty());

    let usart1 = USART1::new(DRV_USART1_RESOURCES).unwrap();
    usart1.enable_clock().unwrap();
    expect((read_reg32(RCC_APB2PCENR) & USART1_ENABLE_BIT) != 0);
    usart1.assert_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & USART1_RESET_BIT) != 0);
    usart1.release_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & USART1_RESET_BIT) == 0);

    let spi1 = SPI1::new(DRV_SPI1_RESOURCES).unwrap();
    spi1.enable_clock().unwrap();
    expect((read_reg32(RCC_APB2PCENR) & SPI1_ENABLE_BIT) != 0);
    spi1.assert_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & SPI1_RESET_BIT) != 0);
    spi1.release_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & SPI1_RESET_BIT) == 0);

    let i2c1 = I2C1::new(DRV_I2C1_RESOURCES).unwrap();
    i2c1.enable_clock().unwrap();
    expect((read_reg32(RCC_APB1PCENR) & I2C1_ENABLE_BIT) != 0);
    i2c1.assert_reset().unwrap();
    expect((read_reg32(RCC_APB1PRSTR) & I2C1_RESET_BIT) != 0);
    i2c1.release_reset().unwrap();
    expect((read_reg32(RCC_APB1PRSTR) & I2C1_RESET_BIT) == 0);

    let tim1 = TIM1::new(DRV_TIM1_RESOURCES).unwrap();
    tim1.enable_clock().unwrap();
    expect((read_reg32(RCC_APB2PCENR) & TIM1_ENABLE_BIT) != 0);
    tim1.assert_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & TIM1_RESET_BIT) != 0);
    tim1.release_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & TIM1_RESET_BIT) == 0);
    tim1.transition_disabled_to_enabled().unwrap();
    expect((read_reg16(TIM1_CTLR1) & 1) != 0);
    tim1.transition_enabled_to_disabled().unwrap();
    expect((read_reg16(TIM1_CTLR1) & 1) == 0);

    let adc1 = ADC1::new(DRV_ADC1_RESOURCES).unwrap();
    adc1.enable_clock().unwrap();
    expect((read_reg32(RCC_APB2PCENR) & ADC1_ENABLE_BIT) != 0);
    adc1.assert_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & ADC1_RESET_BIT) != 0);
    adc1.release_reset().unwrap();
    expect((read_reg32(RCC_APB2PRSTR) & ADC1_RESET_BIT) == 0);
    adc1.apply_calibrate().unwrap();

    unsafe {
        write_volatile(0x2000_0000 as *mut u32, 0x600D_600D);
    }

    loop {
        spin_loop();
    }
}
