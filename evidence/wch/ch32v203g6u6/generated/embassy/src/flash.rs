//! Generated Embassy-style flash module for CH32V203G6U6.

use crate::metadata;
use core::ptr::{read_volatile, write_volatile};

#[allow(dead_code)]
fn checked_address(address: u64, align: usize) -> Result<usize, metadata::Error> {
    let address = usize::try_from(address).map_err(|_| {
        metadata::Error::Unsupported("MMIO address does not fit usize on this target")
    })?;
    if address % align != 0 {
        return Err(metadata::Error::Unsupported(
            "MMIO address is not naturally aligned for the target register width",
        ));
    }
    Ok(address)
}

#[allow(dead_code)]
fn modify_u8(address: u64, clear_mask: u8, set_mask: u8) -> Result<(), metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u8>())?;
    unsafe {
        let current = read_volatile(address as *const u8);
        write_volatile(address as *mut u8, (current & !clear_mask) | set_mask);
    }
    Ok(())
}

#[allow(dead_code)]
fn modify_u16(address: u64, clear_mask: u16, set_mask: u16) -> Result<(), metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u16>())?;
    unsafe {
        let current = read_volatile(address as *const u16);
        write_volatile(address as *mut u16, (current & !clear_mask) | set_mask);
    }
    Ok(())
}

#[allow(dead_code)]
fn modify_u32(address: u64, clear_mask: u32, set_mask: u32) -> Result<(), metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u32>())?;
    unsafe {
        let current = read_volatile(address as *const u32);
        write_volatile(address as *mut u32, (current & !clear_mask) | set_mask);
    }
    Ok(())
}

#[allow(dead_code)]
fn read_u8(address: u64) -> Result<u8, metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u8>())?;
    unsafe { Ok(read_volatile(address as *const u8)) }
}

#[allow(dead_code)]
fn read_u16(address: u64) -> Result<u16, metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u16>())?;
    unsafe { Ok(read_volatile(address as *const u16)) }
}

#[allow(dead_code)]
fn read_u32(address: u64) -> Result<u32, metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u32>())?;
    unsafe { Ok(read_volatile(address as *const u32)) }
}

#[allow(dead_code)]
fn write_u8(address: u64, value: u8) -> Result<(), metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u8>())?;
    unsafe {
        write_volatile(address as *mut u8, value);
    }
    Ok(())
}

#[allow(dead_code)]
fn write_u16(address: u64, value: u16) -> Result<(), metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u16>())?;
    unsafe {
        write_volatile(address as *mut u16, value);
    }
    Ok(())
}

#[allow(dead_code)]
fn write_u32(address: u64, value: u32) -> Result<(), metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u32>())?;
    unsafe {
        write_volatile(address as *mut u32, value);
    }
    Ok(())
}

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "flash",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: FLASH (flash) from canonical block block.flash -> flash-controller
pub const DRV_FLASH_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[];
pub const DRV_FLASH_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_FLASH_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_FLASH_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_FLASH_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_FLASH_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_FLASH_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_FLASH_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[
    metadata::SemanticOperation {
        id: "op.flash.unlock",
        name: "Unlock FLASH control",
        description: None,
        kind: Some("configuration"),
        target_refs: &["periph.flash"],
        steps: &[
            metadata::SemanticOperationStep {
                index: 0,
                action: "write",
                target_ref: Some("reg.flash.keyr"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Write FKEYR = 0x45670123",
                }),
                value: None,
                description: Some("Write FLASH_KEY1 to the key register."),
            },
            metadata::SemanticOperationStep {
                index: 1,
                action: "write",
                target_ref: Some("reg.flash.keyr"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Write FKEYR = 0xCDEF89AB",
                }),
                value: None,
                description: Some("Write FLASH_KEY2 to complete the standard unlock sequence."),
            },
        ],
        preconditions: &[],
        postconditions: &[],
    },
    metadata::SemanticOperation {
        id: "op.flash.lock",
        name: "Lock FLASH control",
        description: None,
        kind: Some("configuration"),
        target_refs: &["periph.flash"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.flash.ctlr"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Set LOCK = 1",
            }),
            value: None,
            description: Some("Restore the standard FLASH control lock."),
        }],
        preconditions: &[],
        postconditions: &[],
    },
    metadata::SemanticOperation {
        id: "op.flash.clear_eop",
        name: "Clear FLASH end-of-operation flag",
        description: None,
        kind: Some("configuration"),
        target_refs: &["periph.flash"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.flash.statr"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Set EOP = 1",
            }),
            value: None,
            description: Some("Acknowledge the standard FLASH end-of-operation flag."),
        }],
        preconditions: &[],
        postconditions: &[],
    },
    metadata::SemanticOperation {
        id: "op.flash.clear_wrprterr",
        name: "Clear FLASH write-protect error flag",
        description: None,
        kind: Some("configuration"),
        target_refs: &["periph.flash"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.flash.statr"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Set WRPRTERR = 1",
            }),
            value: None,
            description: Some("Acknowledge the standard FLASH write-protect error flag."),
        }],
        preconditions: &[],
        postconditions: &[],
    },
];
pub const DRV_FLASH_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_FLASH_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct FLASHResources {
    pub clocks: &'static [metadata::ClockBinding],
    pub resets: &'static [metadata::ResetBinding],
    pub interrupt_sources: &'static [metadata::InterruptSource],
    pub interrupts: &'static [metadata::InterruptRoute],
    pub dma_channels: &'static [metadata::DmaChannel],
    pub dma: &'static [metadata::DmaRoute],
    pub pins: &'static [metadata::PinRole],
    pub init_operations: &'static [metadata::SemanticOperation],
    pub state_machines: &'static [metadata::SemanticStateMachine],
    pub lowering_pattern: Option<&'static str>,
    pub time_driver_source: Option<&'static str>,
    pub capability_tags: &'static [&'static str],
}

pub const DRV_FLASH_RESOURCES: FLASHResources = FLASHResources {
    clocks: DRV_FLASH_CLOCK_BINDINGS,
    resets: DRV_FLASH_RESET_BINDINGS,
    interrupt_sources: DRV_FLASH_INTERRUPT_SOURCES,
    interrupts: DRV_FLASH_INTERRUPT_ROUTES,
    dma_channels: DRV_FLASH_DMA_CHANNELS,
    dma: DRV_FLASH_DMA_ROUTES,
    pins: DRV_FLASH_PIN_ROLES,
    init_operations: DRV_FLASH_INIT_OPERATIONS,
    state_machines: DRV_FLASH_STATE_MACHINES,
    lowering_pattern: Some("stm32f1-page-flash"),
    time_driver_source: None,
    capability_tags: DRV_FLASH_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct FLASH {
    resources: FLASHResources,
}

impl FLASH {
    pub fn new(resources: FLASHResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> FLASHResources {
        self.resources
    }
    pub fn unlock(&self) -> Result<(), metadata::Error> {
        self.apply_unlock()?;
        Ok(())
    }

    pub fn clear_end_of_operation_flag(&self) -> Result<(), metadata::Error> {
        self.apply_clear_eop()
    }

    /// Report whether the flash controller is busy with an erase or program operation.
    pub fn is_busy(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x4002200Cu64)? & 0x00000001u32) != 0)
    }

    /// Report whether the flash controller completion flag is currently asserted.
    pub fn has_end_of_operation_flag(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x4002200Cu64)? & 0x00000020u32) != 0)
    }

    pub fn lock(&self) -> Result<(), metadata::Error> {
        self.apply_lock()?;
        Ok(())
    }

    /// Report whether the flash controller write-protect error flag is currently asserted.
    pub fn has_write_protect_error(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x4002200Cu64)? & 0x00000010u32) != 0)
    }

    pub fn clear_write_protect_error_flag(&self) -> Result<(), metadata::Error> {
        self.apply_clear_wrprterr()
    }

    pub fn apply_unlock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40022004u64, 0xFFFFFFFFu32, 0x45670123u32)?;
        modify_u32(0x40022004u64, 0xFFFFFFFFu32, 0xCDEF89ABu32)?;
        Ok(())
    }

    pub fn apply_lock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40022010u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    pub fn apply_clear_eop(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002200Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    pub fn apply_clear_wrprterr(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002200Cu64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FLASHFlashError {
    NotAligned,
    OutOfBounds,
    ProgramError,
    WriteProtectError,
    MissingCompletionFlag,
    Metadata(metadata::Error),
}

impl From<metadata::Error> for FLASHFlashError {
    fn from(value: metadata::Error) -> Self {
        Self::Metadata(value)
    }
}

impl embedded_storage::nor_flash::NorFlashError for FLASHFlashError {
    fn kind(&self) -> embedded_storage::nor_flash::NorFlashErrorKind {
        match self {
            Self::NotAligned => embedded_storage::nor_flash::NorFlashErrorKind::NotAligned,
            Self::OutOfBounds => embedded_storage::nor_flash::NorFlashErrorKind::OutOfBounds,
            Self::ProgramError
            | Self::WriteProtectError
            | Self::MissingCompletionFlag
            | Self::Metadata(_) => embedded_storage::nor_flash::NorFlashErrorKind::Other,
        }
    }
}

const DRV_FLASH_FLASH_STORAGE_BASE: u32 = 0x08000000u32;
const DRV_FLASH_FLASH_STORAGE_SIZE: usize = 32768;
const DRV_FLASH_FLASH_BUSY_STATUS_ADDRESS: u64 = 0x4002200Cu64;
const DRV_FLASH_FLASH_BUSY_MASK: u32 = 0x00000001u32;
const DRV_FLASH_FLASH_EOP_MASK: u32 = 0x00000020u32;

fn drv_flash_flash_check_read(offset: u32, len: usize) -> Result<(), FLASHFlashError> {
    let offset = usize::try_from(offset).map_err(|_| FLASHFlashError::OutOfBounds)?;
    let end = offset
        .checked_add(len)
        .ok_or(FLASHFlashError::OutOfBounds)?;
    if end > DRV_FLASH_FLASH_STORAGE_SIZE {
        return Err(FLASHFlashError::OutOfBounds);
    }
    Ok(())
}

fn drv_flash_flash_check_write(offset: u32, len: usize) -> Result<(), FLASHFlashError> {
    let align = 2usize;
    if !(offset as usize).is_multiple_of(align) || !len.is_multiple_of(align) {
        return Err(FLASHFlashError::NotAligned);
    }
    drv_flash_flash_check_read(offset, len)
}

fn drv_flash_flash_check_erase(from: u32, to: u32) -> Result<(), FLASHFlashError> {
    let align = 4096usize;
    let from = usize::try_from(from).map_err(|_| FLASHFlashError::OutOfBounds)?;
    let to = usize::try_from(to).map_err(|_| FLASHFlashError::OutOfBounds)?;
    if from >= to || from % align != 0 || to % align != 0 || to > DRV_FLASH_FLASH_STORAGE_SIZE {
        return Err(FLASHFlashError::OutOfBounds);
    }
    Ok(())
}

fn drv_flash_flash_status() -> Result<u32, FLASHFlashError> {
    read_u32(DRV_FLASH_FLASH_BUSY_STATUS_ADDRESS).map_err(Into::into)
}

fn drv_flash_flash_wait_ready() -> Result<(), FLASHFlashError> {
    while (drv_flash_flash_status()? & DRV_FLASH_FLASH_BUSY_MASK) != 0 {
        core::hint::spin_loop();
    }
    Ok(())
}

fn drv_flash_flash_clear_stale_flags(driver: &FLASH) -> Result<(), FLASHFlashError> {
    let status = drv_flash_flash_status()?;
    if (status & DRV_FLASH_FLASH_EOP_MASK) != 0 {
        driver
            .clear_end_of_operation_flag()
            .map_err(FLASHFlashError::from)?;
    }
    if (status & 0x00000010u32) != 0 {
        driver
            .apply_clear_wrprterr()
            .map_err(FLASHFlashError::from)?;
    }
    Ok(())
}

fn drv_flash_flash_begin(driver: &FLASH) -> Result<(), FLASHFlashError> {
    drv_flash_flash_wait_ready()?;
    drv_flash_flash_clear_stale_flags(driver)?;
    driver.unlock().map_err(Into::into)
}

fn drv_flash_flash_end(driver: &FLASH) -> Result<(), FLASHFlashError> {
    driver.lock().map_err(Into::into)
}

fn drv_flash_flash_finish_operation(driver: &FLASH) -> Result<(), FLASHFlashError> {
    drv_flash_flash_wait_ready()?;
    let status = drv_flash_flash_status()?;
    if (status & 0x00000010u32) != 0 {
        driver
            .apply_clear_wrprterr()
            .map_err(FLASHFlashError::from)?;
        return Err(FLASHFlashError::WriteProtectError);
    }
    if (status & DRV_FLASH_FLASH_EOP_MASK) == 0 {
        return Err(FLASHFlashError::MissingCompletionFlag);
    }
    driver
        .clear_end_of_operation_flag()
        .map_err(FLASHFlashError::from)?;
    Ok(())
}

fn drv_flash_flash_erase_page(driver: &FLASH, page_address: u32) -> Result<(), FLASHFlashError> {
    drv_flash_flash_clear_stale_flags(driver)?;
    modify_u32(0x40022010u64, 0x00000002u32, 0x00000002u32)?;
    modify_u32(0x40022014u64, 0xFFFFFFFFu32, page_address)?;
    modify_u32(0x40022010u64, 0x00000040u32, 0x00000040u32)?;
    let result = drv_flash_flash_finish_operation(driver);
    modify_u32(0x40022010u64, 0x00000002u32, 0x00000000u32)?;
    result
}

fn drv_flash_flash_program_halfword(
    driver: &FLASH,
    address: u32,
    value: u16,
) -> Result<(), FLASHFlashError> {
    drv_flash_flash_clear_stale_flags(driver)?;
    modify_u32(0x40022010u64, 0x00000001u32, 0x00000001u32)?;
    write_u16(u64::from(address), value).map_err(FLASHFlashError::from)?;
    let result = drv_flash_flash_finish_operation(driver);
    modify_u32(0x40022010u64, 0x00000001u32, 0x00000000u32)?;
    result
}

impl embedded_storage::nor_flash::ErrorType for FLASH {
    type Error = FLASHFlashError;
}

impl embedded_storage::nor_flash::ReadNorFlash for FLASH {
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        drv_flash_flash_check_read(offset, bytes.len())?;
        let base = u64::from(DRV_FLASH_FLASH_STORAGE_BASE) + u64::from(offset);
        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = read_u8(base + index as u64).map_err(FLASHFlashError::from)?;
        }
        Ok(())
    }

    fn capacity(&self) -> usize {
        DRV_FLASH_FLASH_STORAGE_SIZE
    }
}

impl embedded_storage::nor_flash::NorFlash for FLASH {
    const WRITE_SIZE: usize = 2;
    const ERASE_SIZE: usize = 4096;

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        drv_flash_flash_check_erase(from, to)?;
        drv_flash_flash_begin(self)?;
        let result = (|| {
            let mut page = from;
            while page < to {
                drv_flash_flash_erase_page(self, DRV_FLASH_FLASH_STORAGE_BASE + page)?;
                page = page.wrapping_add(Self::ERASE_SIZE as u32);
            }
            Ok(())
        })();
        let lock_result = drv_flash_flash_end(self);
        match (result, lock_result) {
            (Err(error), _) => Err(error),
            (Ok(()), Err(error)) => Err(error),
            (Ok(()), Ok(())) => Ok(()),
        }
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        drv_flash_flash_check_write(offset, bytes.len())?;
        drv_flash_flash_begin(self)?;
        let result = (|| {
            for (index, chunk) in bytes.chunks_exact(Self::WRITE_SIZE).enumerate() {
                let address = DRV_FLASH_FLASH_STORAGE_BASE
                    .wrapping_add(offset)
                    .wrapping_add((index * Self::WRITE_SIZE) as u32);
                let value = u16::from_le_bytes([chunk[0], chunk[1]]);
                drv_flash_flash_program_halfword(self, address, value)?;
            }
            Ok(())
        })();
        let lock_result = drv_flash_flash_end(self);
        match (result, lock_result) {
            (Err(error), _) => Err(error),
            (Ok(()), Err(error)) => Err(error),
            (Ok(()), Ok(())) => Ok(()),
        }
    }
}
