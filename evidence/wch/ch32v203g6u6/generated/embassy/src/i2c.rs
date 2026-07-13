//! Generated Embassy-style i2c module for CH32V203G6U6.

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
    module_name: "i2c",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: I2C1 (i2c) from canonical block block.i2c1 -> i2c
pub const DRV_I2C1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.i2c1", name: "I2C1 clock binding", consumer_ref: "periph.i2c1", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_I2C1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.i2c1", name: "I2C1 reset binding", target_ref: "periph.i2c1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_I2C1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.i2c1.er", name: "I2C1 ER interrupt source", source_ref: "periph.i2c1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.i2c1.ev", name: "I2C1 EV interrupt source", source_ref: "periph.i2c1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_I2C1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.i2c1.er", name: "I2C1 ER interrupt route", source_ref: "isrc.i2c1.er", interrupt_ref: "int.i2c1er", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.i2c1.ev", name: "I2C1 EV interrupt route", source_ref: "isrc.i2c1.ev", interrupt_ref: "int.i2c1ev", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_I2C1_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel { id: "dmach.dma1.ch6", name: "DMA1 Channel 6", controller_ref: "block.dma1", target_ref: None, channel_index: 6, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dmach.dma1.ch7", name: "DMA1 Channel 7", controller_ref: "block.dma1", target_ref: None, channel_index: 7, capabilities: &[], priority_levels: &[] }];
pub const DRV_I2C1_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute { id: "dmaroute.i2c1.tx", name: "I2C1 TX DMA route", peripheral_ref: "periph.i2c1", signal: Some("TX"), channel_ref: "dmach.dma1.ch6", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.i2c1.rx", name: "I2C1 RX DMA route", peripheral_ref: "periph.i2c1", signal: Some("RX"), channel_ref: "dmach.dma1.ch7", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: None }];
pub const DRV_I2C1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.i2c1.scl.pb6.r0", name: "I2C1 SCL on PB6 (remap 0)", pin_ref: "pin.pb6", peripheral_ref: "periph.i2c1", signal: "SCL", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_I2C1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.i2c1.sda.pb7.r0", name: "I2C1 SDA on PB7 (remap 0)", pin_ref: "pin.pb7", peripheral_ref: "periph.i2c1", signal: "SDA", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_I2C1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.i2c1.smba.pb5", name: "I2C1 SMBA on PB5", pin_ref: "pin.pb5", peripheral_ref: "periph.i2c1", signal: "SMBA", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_I2C1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "scl", signal: "SCL", routes: DRV_I2C1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "sda", signal: "SDA", routes: DRV_I2C1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "smba", signal: "SMBA", routes: DRV_I2C1_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_I2C1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_I2C1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_I2C1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct I2C1Resources {
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

pub const DRV_I2C1_RESOURCES: I2C1Resources = I2C1Resources {
    clocks: DRV_I2C1_CLOCK_BINDINGS,
    resets: DRV_I2C1_RESET_BINDINGS,
    interrupt_sources: DRV_I2C1_INTERRUPT_SOURCES,
    interrupts: DRV_I2C1_INTERRUPT_ROUTES,
    dma_channels: DRV_I2C1_DMA_CHANNELS,
    dma: DRV_I2C1_DMA_ROUTES,
    pins: DRV_I2C1_PIN_ROLES,
    init_operations: DRV_I2C1_INIT_OPERATIONS,
    state_machines: DRV_I2C1_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_I2C1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct I2C1 {
    resources: I2C1Resources,
}

impl I2C1 {
    pub fn new(resources: I2C1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> I2C1Resources {
        self.resources
    }
    /// Enable the I2C1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00200000u32, 0x00200000u32)?;
        Ok(())
    }

    /// Disable the I2C1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00200000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for I2C1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00200000u32, 0x00200000u32)?;
        Ok(())
    }

    /// Release reset for I2C1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00200000u32, 0x00000000u32)?;
        Ok(())
    }
}
