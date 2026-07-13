#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use ch32v203g6u6_embassy_hal::gpio::{DRV_GPIOA_RESOURCES, GPIOA, Level};
use riscv::{asm::delay, interrupt::machine};

const GPIOA_BSHR: usize = 0x4001_0810;
const GPIOA_BCR: usize = 0x4001_0814;
const RCC_CTLR: *mut u32 = 0x4002_1000 as *mut u32;
const RCC_CFGR0: *mut u32 = 0x4002_1004 as *mut u32;
const PA4_MASK: u32 = 1 << 4;
const PA7_MASK: u32 = 1 << 7;
const MIRROR_MASK: u32 = PA4_MASK | PA7_MASK;
const LONG_RESET_DELAY_CYCLES: u32 = 100_000;
const STEP_HOLD_CYCLES: u32 = 120_000;
const FADE_STEPS: u16 = 64;
const RCC_PLLON: u32 = 1 << 24;
const RCC_PLLRDY: u32 = 1 << 25;
const RCC_SW_MASK: u32 = 0b11;
const RCC_SW_PLL: u32 = 0b10;
const RCC_SWS_MASK: u32 = 0b11 << 2;
const RCC_SWS_PLL: u32 = 0b10 << 2;
const RCC_HPRE_MASK: u32 = 0b1111 << 4;
const RCC_PPRE1_MASK: u32 = 0b111 << 8;
const RCC_PPRE2_MASK: u32 = 0b111 << 11;
const RCC_PLLSRC: u32 = 1 << 16;
const RCC_PLLXTPRE: u32 = 1 << 17;
const RCC_PLLMULL_MASK: u32 = 0b1111 << 18;
const RCC_PLLMULL6: u32 = 4 << 18;

const FADE_COLORS: [[u8; 3]; 3] = [[0x20, 0x00, 0x00], [0x00, 0x20, 0x00], [0x00, 0x00, 0x20]];

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[inline(always)]
fn write_neopixel_byte(byte: u8, mask: u32) {
    unsafe {
        asm!(
            "li {count}, 8",
            "1:",
            "andi {bit}, {byte}, 0x80",
            "sw {mask}, 0({set_addr})",
            "beqz {bit}, 2f",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "sw {mask}, 0({clr_addr})",
            "nop",
            "j 3f",
            "2:",
            "sw {mask}, 0({clr_addr})",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "3:",
            "slli {byte}, {byte}, 1",
            "addi {count}, {count}, -1",
            "bnez {count}, 1b",
            byte = inout(reg) byte => _,
            bit = out(reg) _,
            count = out(reg) _,
            set_addr = in(reg) GPIOA_BSHR,
            clr_addr = in(reg) GPIOA_BCR,
            mask = in(reg) mask,
            options(nostack)
        );
    }
}

#[inline(always)]
fn latch_low(mask: u32, reset_delay_cycles: u32) {
    unsafe {
        asm!(
            "sw {mask}, 0({clr_addr})",
            clr_addr = in(reg) GPIOA_BCR,
            mask = in(reg) mask,
            options(nostack)
        );
    }
    delay(reset_delay_cycles);
}

fn write_neopixel_frame_24(rgb: [u8; 3], mask: u32, reset_delay_cycles: u32) {
    machine::free(|| {
        write_neopixel_byte(rgb[1], mask);
        write_neopixel_byte(rgb[0], mask);
        write_neopixel_byte(rgb[2], mask);
        latch_low(mask, reset_delay_cycles);
    });
}

fn clear_neopixel_24(mask: u32, reset_delay_cycles: u32, repeats: usize) {
    machine::free(|| {
        for _ in 0..repeats {
            write_neopixel_byte(0x00, mask);
            write_neopixel_byte(0x00, mask);
            write_neopixel_byte(0x00, mask);
            latch_low(mask, reset_delay_cycles);
        }
    });
}

fn interpolate_channel(start: u8, end: u8, step: u16, total_steps: u16) -> u8 {
    let start = u32::from(start);
    let end = u32::from(end);
    let step = u32::from(step);
    let total_steps = u32::from(total_steps);
    (((start * (total_steps - step)) + (end * step)) / total_steps) as u8
}

fn interpolate_color(start: [u8; 3], end: [u8; 3], step: u16, total_steps: u16) -> [u8; 3] {
    [
        interpolate_channel(start[0], end[0], step, total_steps),
        interpolate_channel(start[1], end[1], step, total_steps),
        interpolate_channel(start[2], end[2], step, total_steps),
    ]
}

fn configure_sysclk_24mhz_from_hsi() {
    unsafe {
        let mut cfgr0 = RCC_CFGR0.read_volatile();
        cfgr0 &= !(RCC_SW_MASK
            | RCC_HPRE_MASK
            | RCC_PPRE1_MASK
            | RCC_PPRE2_MASK
            | RCC_PLLSRC
            | RCC_PLLXTPRE
            | RCC_PLLMULL_MASK);
        cfgr0 |= RCC_PLLMULL6;
        RCC_CFGR0.write_volatile(cfgr0);

        RCC_CTLR.write_volatile(RCC_CTLR.read_volatile() | RCC_PLLON);
        while (RCC_CTLR.read_volatile() & RCC_PLLRDY) == 0 {}

        let mut switched = RCC_CFGR0.read_volatile();
        switched &= !RCC_SW_MASK;
        switched |= RCC_SW_PLL;
        RCC_CFGR0.write_volatile(switched);
        while (RCC_CFGR0.read_volatile() & RCC_SWS_MASK) != RCC_SWS_PLL {}
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    configure_sysclk_24mhz_from_hsi();

    let gpioa = GPIOA::new(DRV_GPIOA_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();
    let _neopixel = gpioa.pa4().into_output(Level::Low).unwrap();
    let _probe = gpioa.pa7().into_output(Level::Low).unwrap();
    clear_neopixel_24(MIRROR_MASK, LONG_RESET_DELAY_CYCLES, 8);
    loop {
        for color_index in 0..FADE_COLORS.len() {
            let start = FADE_COLORS[color_index];
            let end = FADE_COLORS[(color_index + 1) % FADE_COLORS.len()];
            for step in 0..=FADE_STEPS {
                let color = interpolate_color(start, end, step, FADE_STEPS);
                write_neopixel_frame_24(color, MIRROR_MASK, LONG_RESET_DELAY_CYCLES);
                delay(STEP_HOLD_CYCLES);
            }
        }
    }
}
