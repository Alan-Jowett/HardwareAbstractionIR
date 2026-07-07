//! Generated Embassy-style spi module for STM32F405RGT6.

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

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "spi",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: Spi1 (spi) from canonical block block.spi1 -> spi
pub const DRV_SPI1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.spi1", name: "SPI1 clock", consumer_ref: "periph.spi1", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_SPI1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.spi1", name: "SPI1 reset", target_ref: "periph.spi1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.apb2"), binding_kind: "software", control_refs: &["reg.rcc.apb2rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_SPI1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.spi1.global", name: "SPI1 GLOBAL source", source_ref: "periph.spi1", producer_ref: Some("block.spi1"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_SPI1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.spi1.global", name: "SPI1 GLOBAL route", source_ref: "isrc.spi1.global", interrupt_ref: "irq.spi1", controller_ref: "block.nvic", cpu_target_ref: None, line_index: Some(35), route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_SPI1_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_SPI1_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_SPI1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.sck.pa5", name: "SPI1 SCK on PA5", pin_ref: "pin.pa5", peripheral_ref: "periph.spi1", signal: "SCK", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi1.sck.pb3", name: "SPI1 SCK on PB3", pin_ref: "pin.pb3", peripheral_ref: "periph.spi1", signal: "SCK", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.mosi.pa7", name: "SPI1 MOSI on PA7", pin_ref: "pin.pa7", peripheral_ref: "periph.spi1", signal: "MOSI", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi1.mosi.pb5", name: "SPI1 MOSI on PB5", pin_ref: "pin.pb5", peripheral_ref: "periph.spi1", signal: "MOSI", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.miso.pa6", name: "SPI1 MISO on PA6", pin_ref: "pin.pa6", peripheral_ref: "periph.spi1", signal: "MISO", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi1.miso.pb4", name: "SPI1 MISO on PB4", pin_ref: "pin.pb4", peripheral_ref: "periph.spi1", signal: "MISO", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi1.nss.pa15", name: "SPI1 NSS on PA15", pin_ref: "pin.pa15", peripheral_ref: "periph.spi1", signal: "NSS", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi1.nss.pa4", name: "SPI1 NSS on PA4", pin_ref: "pin.pa4", peripheral_ref: "periph.spi1", signal: "NSS", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "sck", signal: "SCK", routes: DRV_SPI1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "mosi", signal: "MOSI", routes: DRV_SPI1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "miso", signal: "MISO", routes: DRV_SPI1_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "nss", signal: "NSS", routes: DRV_SPI1_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_SPI1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_SPI1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_SPI1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Spi1Resources {
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

pub const DRV_SPI1_RESOURCES: Spi1Resources = Spi1Resources {
    clocks: DRV_SPI1_CLOCK_BINDINGS,
    resets: DRV_SPI1_RESET_BINDINGS,
    interrupt_sources: DRV_SPI1_INTERRUPT_SOURCES,
    interrupts: DRV_SPI1_INTERRUPT_ROUTES,
    dma_channels: DRV_SPI1_DMA_CHANNELS,
    dma: DRV_SPI1_DMA_ROUTES,
    pins: DRV_SPI1_PIN_ROLES,
    init_operations: DRV_SPI1_INIT_OPERATIONS,
    state_machines: DRV_SPI1_STATE_MACHINES,
    capability_tags: DRV_SPI1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Spi1 {
    resources: Spi1Resources,
}

impl Spi1 {
    pub fn new(resources: Spi1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Spi1Resources {
        self.resources
    }
    /// Enable the SPI1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023844u64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Disable the SPI1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023844u64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SPI1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023824u64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Release reset for SPI1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023824u64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: Spi2 (spi) from canonical block block.spi2 -> spi
pub const DRV_SPI2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.spi2", name: "SPI2 clock", consumer_ref: "periph.spi2", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_SPI2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.spi2", name: "SPI2 reset", target_ref: "periph.spi2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.apb1"), binding_kind: "software", control_refs: &["reg.rcc.apb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_SPI2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.spi2.global", name: "SPI2 GLOBAL source", source_ref: "periph.spi2", producer_ref: Some("block.spi2"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_SPI2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.spi2.global", name: "SPI2 GLOBAL route", source_ref: "isrc.spi2.global", interrupt_ref: "irq.spi2", controller_ref: "block.nvic", cpu_target_ref: None, line_index: Some(36), route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: Some("spi2") }];
pub const DRV_SPI2_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_SPI2_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_SPI2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.sck.pb10", name: "SPI2 SCK on PB10", pin_ref: "pin.pb10", peripheral_ref: "periph.spi2", signal: "SCK", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi2.sck.pb13", name: "SPI2 SCK on PB13", pin_ref: "pin.pb13", peripheral_ref: "periph.spi2", signal: "SCK", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.mosi.pb15", name: "SPI2 MOSI on PB15", pin_ref: "pin.pb15", peripheral_ref: "periph.spi2", signal: "MOSI", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi2.mosi.pc3", name: "SPI2 MOSI on PC3", pin_ref: "pin.pc3", peripheral_ref: "periph.spi2", signal: "MOSI", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.miso.pb14", name: "SPI2 MISO on PB14", pin_ref: "pin.pb14", peripheral_ref: "periph.spi2", signal: "MISO", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi2.miso.pc2", name: "SPI2 MISO on PC2", pin_ref: "pin.pc2", peripheral_ref: "periph.spi2", signal: "MISO", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.nss.pb12", name: "SPI2 NSS on PB12", pin_ref: "pin.pb12", peripheral_ref: "periph.spi2", signal: "NSS", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi2.nss.pb9", name: "SPI2 NSS on PB9", pin_ref: "pin.pb9", peripheral_ref: "periph.spi2", signal: "NSS", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "sck", signal: "SCK", routes: DRV_SPI2_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "mosi", signal: "MOSI", routes: DRV_SPI2_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "miso", signal: "MISO", routes: DRV_SPI2_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "nss", signal: "NSS", routes: DRV_SPI2_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_SPI2_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_SPI2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_SPI2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Spi2Resources {
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

pub const DRV_SPI2_RESOURCES: Spi2Resources = Spi2Resources {
    clocks: DRV_SPI2_CLOCK_BINDINGS,
    resets: DRV_SPI2_RESET_BINDINGS,
    interrupt_sources: DRV_SPI2_INTERRUPT_SOURCES,
    interrupts: DRV_SPI2_INTERRUPT_ROUTES,
    dma_channels: DRV_SPI2_DMA_CHANNELS,
    dma: DRV_SPI2_DMA_ROUTES,
    pins: DRV_SPI2_PIN_ROLES,
    init_operations: DRV_SPI2_INIT_OPERATIONS,
    state_machines: DRV_SPI2_STATE_MACHINES,
    capability_tags: DRV_SPI2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Spi2 {
    resources: Spi2Resources,
}

impl Spi2 {
    pub fn new(resources: Spi2Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Spi2Resources {
        self.resources
    }
    /// Enable the SPI2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00004000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Disable the SPI2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00004000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SPI2.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00004000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Release reset for SPI2.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00004000u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: Spi3 (spi) from canonical block block.spi3 -> spi
pub const DRV_SPI3_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.spi3", name: "SPI3 clock", consumer_ref: "periph.spi3", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_SPI3_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.spi3", name: "SPI3 reset", target_ref: "periph.spi3", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.apb1"), binding_kind: "software", control_refs: &["reg.rcc.apb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_SPI3_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.spi3.global", name: "SPI3 GLOBAL source", source_ref: "periph.spi3", producer_ref: Some("block.spi3"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_SPI3_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.spi3.global", name: "SPI3 GLOBAL route", source_ref: "isrc.spi3.global", interrupt_ref: "irq.spi3", controller_ref: "block.nvic", cpu_target_ref: None, line_index: Some(51), route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: Some("spi3") }];
pub const DRV_SPI3_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_SPI3_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_SPI3_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi3.sck.pb3", name: "SPI3 SCK on PB3", pin_ref: "pin.pb3", peripheral_ref: "periph.spi3", signal: "SCK", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi3.sck.pc10", name: "SPI3 SCK on PC10", pin_ref: "pin.pc10", peripheral_ref: "periph.spi3", signal: "SCK", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI3_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi3.mosi.pb5", name: "SPI3 MOSI on PB5", pin_ref: "pin.pb5", peripheral_ref: "periph.spi3", signal: "MOSI", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi3.mosi.pc12", name: "SPI3 MOSI on PC12", pin_ref: "pin.pc12", peripheral_ref: "periph.spi3", signal: "MOSI", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI3_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi3.miso.pb4", name: "SPI3 MISO on PB4", pin_ref: "pin.pb4", peripheral_ref: "periph.spi3", signal: "MISO", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi3.miso.pc11", name: "SPI3 MISO on PC11", pin_ref: "pin.pc11", peripheral_ref: "periph.spi3", signal: "MISO", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI3_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi3.nss.pa15", name: "SPI3 NSS on PA15", pin_ref: "pin.pa15", peripheral_ref: "periph.spi3", signal: "NSS", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.spi3.nss.pa4", name: "SPI3 NSS on PA4", pin_ref: "pin.pa4", peripheral_ref: "periph.spi3", signal: "NSS", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI3_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "sck", signal: "SCK", routes: DRV_SPI3_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "mosi", signal: "MOSI", routes: DRV_SPI3_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "miso", signal: "MISO", routes: DRV_SPI3_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "nss", signal: "NSS", routes: DRV_SPI3_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_SPI3_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_SPI3_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_SPI3_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Spi3Resources {
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

pub const DRV_SPI3_RESOURCES: Spi3Resources = Spi3Resources {
    clocks: DRV_SPI3_CLOCK_BINDINGS,
    resets: DRV_SPI3_RESET_BINDINGS,
    interrupt_sources: DRV_SPI3_INTERRUPT_SOURCES,
    interrupts: DRV_SPI3_INTERRUPT_ROUTES,
    dma_channels: DRV_SPI3_DMA_CHANNELS,
    dma: DRV_SPI3_DMA_ROUTES,
    pins: DRV_SPI3_PIN_ROLES,
    init_operations: DRV_SPI3_INIT_OPERATIONS,
    state_machines: DRV_SPI3_STATE_MACHINES,
    capability_tags: DRV_SPI3_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Spi3 {
    resources: Spi3Resources,
}

impl Spi3 {
    pub fn new(resources: Spi3Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Spi3Resources {
        self.resources
    }
    /// Enable the SPI3 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00008000u32, 0x00008000u32)?;
        Ok(())
    }

    /// Disable the SPI3 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00008000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SPI3.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00008000u32, 0x00008000u32)?;
        Ok(())
    }

    /// Release reset for SPI3.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00008000u32, 0x00000000u32)?;
        Ok(())
    }


}

