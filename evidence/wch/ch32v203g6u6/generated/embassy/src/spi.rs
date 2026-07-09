//! Generated Embassy-style spi module for CH32V203G6U6.

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
    module_name: "spi",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: SPI1 (spi) from canonical block block.spi1 -> spi
pub const DRV_SPI1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.spi1", name: "SPI1 clock binding", consumer_ref: "periph.spi1", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_SPI1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.spi1", name: "SPI1 reset binding", target_ref: "periph.spi1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_SPI1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.spi1.global", name: "SPI1 GLOBAL interrupt source", source_ref: "periph.spi1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_SPI1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.spi1.global", name: "SPI1 GLOBAL interrupt route", source_ref: "isrc.spi1.global", interrupt_ref: "int.spi1", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_SPI1_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel { id: "dmach.dma1.ch2", name: "DMA1 Channel 2", controller_ref: "block.dma1", target_ref: None, channel_index: 2, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dmach.dma1.ch3", name: "DMA1 Channel 3", controller_ref: "block.dma1", target_ref: None, channel_index: 3, capabilities: &[], priority_levels: &[] }];
pub const DRV_SPI1_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute { id: "dmaroute.spi1.rx", name: "SPI1 RX DMA route", peripheral_ref: "periph.spi1", signal: Some("RX"), channel_ref: "dmach.dma1.ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.spi1.tx", name: "SPI1 TX DMA route", peripheral_ref: "periph.spi1", signal: Some("TX"), channel_ref: "dmach.dma1.ch3", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: None }];
pub const DRV_SPI1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.sck.pa5.r0", name: "SPI1 SCK on PA5 (remap 0)", pin_ref: "pin.pa5", peripheral_ref: "periph.spi1", signal: "SCK", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.spi1.sck.pb3.r1", name: "SPI1 SCK on PB3 (remap 1)", pin_ref: "pin.pb3", peripheral_ref: "periph.spi1", signal: "SCK", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.miso.pa6.r0", name: "SPI1 MISO on PA6 (remap 0)", pin_ref: "pin.pa6", peripheral_ref: "periph.spi1", signal: "MISO", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.spi1.miso.pb4.r1", name: "SPI1 MISO on PB4 (remap 1)", pin_ref: "pin.pb4", peripheral_ref: "periph.spi1", signal: "MISO", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.mosi.pa7.r0", name: "SPI1 MOSI on PA7 (remap 0)", pin_ref: "pin.pa7", peripheral_ref: "periph.spi1", signal: "MOSI", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.spi1.mosi.pb5.r1", name: "SPI1 MOSI on PB5 (remap 1)", pin_ref: "pin.pb5", peripheral_ref: "periph.spi1", signal: "MOSI", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.nss.pa4.r0", name: "SPI1 NSS on PA4 (remap 0)", pin_ref: "pin.pa4", peripheral_ref: "periph.spi1", signal: "NSS", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.spi1.nss.pa15.r1", name: "SPI1 NSS on PA15 (remap 1)", pin_ref: "pin.pa15", peripheral_ref: "periph.spi1", signal: "NSS", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "sck", signal: "SCK", routes: DRV_SPI1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "miso", signal: "MISO", routes: DRV_SPI1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "mosi", signal: "MOSI", routes: DRV_SPI1_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "nss", signal: "NSS", routes: DRV_SPI1_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_SPI1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_SPI1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_SPI1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct SPI1Resources {
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
    pub capability_tags: &'static [&'static str],
}

pub const DRV_SPI1_RESOURCES: SPI1Resources = SPI1Resources {
    clocks: DRV_SPI1_CLOCK_BINDINGS,
    resets: DRV_SPI1_RESET_BINDINGS,
    interrupt_sources: DRV_SPI1_INTERRUPT_SOURCES,
    interrupts: DRV_SPI1_INTERRUPT_ROUTES,
    dma_channels: DRV_SPI1_DMA_CHANNELS,
    dma: DRV_SPI1_DMA_ROUTES,
    pins: DRV_SPI1_PIN_ROLES,
    init_operations: DRV_SPI1_INIT_OPERATIONS,
    state_machines: DRV_SPI1_STATE_MACHINES,
    lowering_pattern: None,
    capability_tags: DRV_SPI1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct SPI1 {
    resources: SPI1Resources,
}

impl SPI1 {
    pub fn new(resources: SPI1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> SPI1Resources {
        self.resources
    }
    /// Enable the SPI1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Disable the SPI1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SPI1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Release reset for SPI1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }


}

