#![no_std]
#![no_main]

use core::ptr::addr_of_mut;

#[cfg(not(test))]
use core::panic::PanicInfo;

use ch32v203g6u6_embassy_hal::{
    flash::{DRV_FLASH_RUNTIME_RESOURCES, FLASH},
    gpio::{DRV_GPIOA_RUNTIME_RESOURCES, GPIOA, GPIOAOutput, Level},
};
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};

const PAGE_SIZE: usize = 4096;
const VERIFY_CHUNK_SIZE: usize = 64;
const SHORT_DELAY_CYCLES: u32 = 4_000_000;
const LONG_DELAY_CYCLES: u32 = 12_000_000;
const INTERVAL_DELAY_CYCLES: u32 = 6_000_000;
const PATTERN_REPEAT_DELAY_CYCLES: u32 = 18_000_000;

static mut ORIGINAL_PAGE: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

#[derive(Debug, Clone, Copy)]
enum FlashSmokeError {
    CapacityTooSmall,
    BackupRead,
    Erase,
    VerifyErased,
    PatternWrite,
    VerifyPattern,
    RestoreErase,
    RestoreWrite,
    VerifyRestored,
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn busy_delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::spin_loop();
    }
}

fn pulse(output: &GPIOAOutput, count: u8, on_cycles: u32, off_cycles: u32, pause_cycles: u32) {
    for _ in 0..count {
        output.set_high().unwrap();
        busy_delay(on_cycles);
        output.set_low().unwrap();
        busy_delay(off_cycles);
    }
    busy_delay(pause_cycles);
}

fn pattern_byte(index: usize) -> u8 {
    ((index as u8).wrapping_mul(37) ^ 0xA5).rotate_left(1)
}

fn fill_pattern(start_index: usize, bytes: &mut [u8]) {
    for (offset, byte) in bytes.iter_mut().enumerate() {
        *byte = pattern_byte(start_index + offset);
    }
}

fn verify_erased_page(flash: &mut FLASH, target_offset: u32) -> Result<(), FlashSmokeError> {
    let mut chunk = [0u8; VERIFY_CHUNK_SIZE];
    for chunk_offset in (0..PAGE_SIZE).step_by(VERIFY_CHUNK_SIZE) {
        flash
            .read(target_offset + chunk_offset as u32, &mut chunk)
            .map_err(|_| FlashSmokeError::Erase)?;
        for (byte_offset, &byte) in chunk.iter().enumerate() {
            if byte != 0xFF {
                let _ = byte_offset;
                let _ = byte;
                return Err(FlashSmokeError::VerifyErased);
            }
        }
    }
    Ok(())
}

fn write_pattern_page(flash: &mut FLASH, target_offset: u32) -> Result<(), FlashSmokeError> {
    let mut chunk = [0u8; VERIFY_CHUNK_SIZE];
    for chunk_offset in (0..PAGE_SIZE).step_by(VERIFY_CHUNK_SIZE) {
        fill_pattern(chunk_offset, &mut chunk);
        flash
            .write(target_offset + chunk_offset as u32, &chunk)
            .map_err(|_| FlashSmokeError::PatternWrite)?;
    }
    Ok(())
}

fn verify_pattern_page(flash: &mut FLASH, target_offset: u32) -> Result<(), FlashSmokeError> {
    let mut chunk = [0u8; VERIFY_CHUNK_SIZE];
    for chunk_offset in (0..PAGE_SIZE).step_by(VERIFY_CHUNK_SIZE) {
        flash
            .read(target_offset + chunk_offset as u32, &mut chunk)
            .map_err(|_| FlashSmokeError::PatternWrite)?;
        for (byte_offset, &actual) in chunk.iter().enumerate() {
            let expected = pattern_byte(chunk_offset + byte_offset);
            if actual != expected {
                let _ = byte_offset;
                let _ = expected;
                let _ = actual;
                return Err(FlashSmokeError::VerifyPattern);
            }
        }
    }
    Ok(())
}

fn restore_original_page(
    flash: &mut FLASH,
    target_offset: u32,
    original_page: &[u8; PAGE_SIZE],
) -> Result<(), FlashSmokeError> {
    flash
        .erase(target_offset, target_offset + PAGE_SIZE as u32)
        .map_err(|_| FlashSmokeError::RestoreErase)?;

    for chunk_offset in (0..PAGE_SIZE).step_by(VERIFY_CHUNK_SIZE) {
        flash
            .write(
                target_offset + chunk_offset as u32,
                &original_page[chunk_offset..chunk_offset + VERIFY_CHUNK_SIZE],
            )
            .map_err(|_| FlashSmokeError::RestoreWrite)?;
    }

    let mut verify_chunk = [0u8; VERIFY_CHUNK_SIZE];
    for chunk_offset in (0..PAGE_SIZE).step_by(VERIFY_CHUNK_SIZE) {
        flash
            .read(target_offset + chunk_offset as u32, &mut verify_chunk)
            .map_err(|_| FlashSmokeError::RestoreWrite)?;
        for (byte_offset, &actual) in verify_chunk.iter().enumerate() {
            let expected = original_page[chunk_offset + byte_offset];
            if actual != expected {
                let _ = byte_offset;
                let _ = expected;
                let _ = actual;
                return Err(FlashSmokeError::VerifyRestored);
            }
        }
    }
    Ok(())
}

fn run_flash_sequence(flash: &mut FLASH) -> Result<(), FlashSmokeError> {
    if flash.capacity() < PAGE_SIZE {
        return Err(FlashSmokeError::CapacityTooSmall);
    }
    let target_offset = u32::try_from(flash.capacity() - PAGE_SIZE)
        .map_err(|_| FlashSmokeError::CapacityTooSmall)?;
    let original_page = unsafe { &mut *addr_of_mut!(ORIGINAL_PAGE) };

    flash
        .read(target_offset, original_page)
        .map_err(|_| FlashSmokeError::BackupRead)?;

    flash
        .erase(target_offset, target_offset + PAGE_SIZE as u32)
        .map_err(|_| FlashSmokeError::Erase)?;
    verify_erased_page(flash, target_offset)?;
    write_pattern_page(flash, target_offset)?;
    verify_pattern_page(flash, target_offset)?;
    restore_original_page(flash, target_offset, original_page)
}

fn error_code(error: FlashSmokeError) -> u8 {
    match error {
        FlashSmokeError::CapacityTooSmall => 1,
        FlashSmokeError::BackupRead => 2,
        FlashSmokeError::Erase | FlashSmokeError::VerifyErased => 3,
        FlashSmokeError::PatternWrite | FlashSmokeError::VerifyPattern => 4,
        FlashSmokeError::RestoreErase
        | FlashSmokeError::RestoreWrite
        | FlashSmokeError::VerifyRestored => 5,
    }
}

fn loop_pass_pattern(output: &GPIOAOutput) -> ! {
    loop {
        pulse(
            output,
            3,
            SHORT_DELAY_CYCLES,
            SHORT_DELAY_CYCLES,
            PATTERN_REPEAT_DELAY_CYCLES,
        );
    }
}

fn loop_failure_pattern(output: &GPIOAOutput, code: u8) -> ! {
    loop {
        pulse(
            output,
            code,
            SHORT_DELAY_CYCLES,
            SHORT_DELAY_CYCLES,
            PATTERN_REPEAT_DELAY_CYCLES,
        );
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    let gpioa = GPIOA::new(DRV_GPIOA_RUNTIME_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();
    let indicator = gpioa.pa7().into_output(Level::Low).unwrap();
    indicator.set_low().unwrap();

    let mut flash = FLASH::new(DRV_FLASH_RUNTIME_RESOURCES).unwrap();

    pulse(
        &indicator,
        1,
        LONG_DELAY_CYCLES,
        INTERVAL_DELAY_CYCLES,
        INTERVAL_DELAY_CYCLES,
    );

    match run_flash_sequence(&mut flash) {
        Ok(()) => loop_pass_pattern(&indicator),
        Err(error) => loop_failure_pattern(&indicator, error_code(error)),
    }
}
