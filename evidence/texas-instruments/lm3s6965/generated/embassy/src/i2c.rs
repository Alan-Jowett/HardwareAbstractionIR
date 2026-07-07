//! Generated Embassy-style i2c module for LM3S6965.

use crate::metadata;
use core::ptr::{read_volatile, write_volatile};

#[allow(dead_code)]
fn checked_address(address: u64, align: usize) -> Result<usize, metadata::Error> {
    let address = usize::try_from(address)
        .map_err(|_| metadata::Error::Unsupported("MMIO address does not fit usize on this target"))?;
    if address % align != 0 {
        return Err(metadata::Error::Unsupported("MMIO address is not naturally aligned for the target register width"));
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
fn read_u32(address: u64) -> Result<u32, metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u32>())?;
    unsafe { Ok(read_volatile(address as *const u32)) }
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
    module_name: "i2c",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: I2C0 (i2c) from canonical block block.i2c0 -> i2c
pub const DRV_I2C0_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.i2c0", name: "I2C0", consumer_ref: "periph.i2c0", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_I2C0_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.i2c0", name: "I2C0", target_ref: "periph.i2c0", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_I2C0_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.i2c0", name: "I2C0 interrupt source", source_ref: "periph.i2c0", producer_ref: Some("periph.i2c0"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_I2C0_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.i2c0", name: "I2C0 interrupt source route", source_ref: "isrc.i2c0", interrupt_ref: "int.i2c0", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_I2C0_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_I2C0_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_I2C0_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.i2c0.scl.pb2", name: "I2C0 SCL on PB2", pin_ref: "pin.pb2", peripheral_ref: "periph.i2c0", signal: "SCL", route_type: "hardwired", control_refs: &["reg.gpiob.afsel", "reg.gpiob.den", "reg.gpiob.odr"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(false) }];
pub const DRV_I2C0_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.i2c0.sda.pb3", name: "I2C0 SDA on PB3", pin_ref: "pin.pb3", peripheral_ref: "periph.i2c0", signal: "SDA", route_type: "hardwired", control_refs: &["reg.gpiob.afsel", "reg.gpiob.den", "reg.gpiob.odr"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(false) }];
pub const DRV_I2C0_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "scl", signal: "SCL", routes: DRV_I2C0_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "sda", signal: "SDA", routes: DRV_I2C0_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_I2C0_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_I2C0_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_I2C0_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct I2C0Resources {
    pub clocks: &'static [metadata::ClockBinding],
    pub resets: &'static [metadata::ResetBinding],
    pub interrupt_sources: &'static [metadata::InterruptSource],
    pub interrupts: &'static [metadata::InterruptRoute],
    pub dma_channels: &'static [metadata::DmaChannel],
    pub dma: &'static [metadata::DmaRoute],
    pub pins: &'static [metadata::PinRole],
    pub init_operations: &'static [metadata::SemanticOperation],
    pub state_machines: &'static [metadata::SemanticStateMachine],
    pub capability_tags: &'static [&'static str],
}

pub const DRV_I2C0_RESOURCES: I2C0Resources = I2C0Resources {
    clocks: DRV_I2C0_CLOCK_BINDINGS,
    resets: DRV_I2C0_RESET_BINDINGS,
    interrupt_sources: DRV_I2C0_INTERRUPT_SOURCES,
    interrupts: DRV_I2C0_INTERRUPT_ROUTES,
    dma_channels: DRV_I2C0_DMA_CHANNELS,
    dma: DRV_I2C0_DMA_ROUTES,
    pins: DRV_I2C0_PIN_ROLES,
    init_operations: DRV_I2C0_INIT_OPERATIONS,
    state_machines: DRV_I2C0_STATE_MACHINES,
    capability_tags: DRV_I2C0_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct I2C0 {
    resources: I2C0Resources,
}

impl I2C0 {
    pub fn new(resources: I2C0Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> I2C0Resources {
        self.resources
    }
    /// Enable the I2C0 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Disable the I2C0 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for I2C0.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Release reset for I2C0.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }


}

