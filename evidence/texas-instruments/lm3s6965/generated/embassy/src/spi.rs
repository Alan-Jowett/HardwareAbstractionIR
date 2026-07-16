//! Generated Embassy-style spi module for LM3S6965.

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
    module_name: "spi",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: SSI0 (spi) from canonical block block.ssi0 -> spi
pub const DRV_SSI0_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.ssi0",
    name: "SSI0",
    consumer_ref: "periph.ssi0",
    clock_ref: "clock.sysclk",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.sysctl.rcgc1"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_SSI0_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.ssi0",
    name: "SSI0",
    target_ref: "periph.ssi0",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rd.software"),
    binding_kind: "software",
    control_refs: &["reg.sysctl.srcr1"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_SSI0_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource {
    id: "isrc.ssi0",
    name: "SSI0 interrupt source",
    source_ref: "periph.ssi0",
    producer_ref: Some("periph.ssi0"),
    kind: "peripheral",
    flag_refs: &[],
    clear_operation_refs: &[],
}];
pub const DRV_SSI0_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.ssi0",
    name: "SSI0 interrupt source route",
    source_ref: "isrc.ssi0",
    interrupt_ref: "int.ssi0",
    controller_ref: "block.nvic",
    cpu_target_ref: Some("block.cpu0"),
    line_index: None,
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_SSI0_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_SSI0_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_SSI0_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.ssi0.sck.pa2",
    name: "SSI0 CLK on PA2",
    pin_ref: "pin.pa2",
    peripheral_ref: "periph.ssi0",
    signal: "SCK",
    route_type: "hardwired",
    control_refs: &["reg.gpioa.afsel", "reg.gpioa.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_SSI0_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.ssi0.miso.pa4",
    name: "SSI0 RX on PA4",
    pin_ref: "pin.pa4",
    peripheral_ref: "periph.ssi0",
    signal: "MISO",
    route_type: "hardwired",
    control_refs: &["reg.gpioa.afsel", "reg.gpioa.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_SSI0_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.ssi0.mosi.pa5",
    name: "SSI0 TX on PA5",
    pin_ref: "pin.pa5",
    peripheral_ref: "periph.ssi0",
    signal: "MOSI",
    route_type: "hardwired",
    control_refs: &["reg.gpioa.afsel", "reg.gpioa.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_SSI0_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.ssi0.fss.pa3",
    name: "SSI0 FSS on PA3",
    pin_ref: "pin.pa3",
    peripheral_ref: "periph.ssi0",
    signal: "FSS",
    route_type: "hardwired",
    control_refs: &["reg.gpioa.afsel", "reg.gpioa.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_SSI0_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "sck",
        signal: "SCK",
        routes: DRV_SSI0_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "miso",
        signal: "MISO",
        routes: DRV_SSI0_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "mosi",
        signal: "MOSI",
        routes: DRV_SSI0_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "fss",
        signal: "FSS",
        routes: DRV_SSI0_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_SSI0_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_SSI0_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_SSI0_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct SSI0RuntimeResources {}

pub const DRV_SSI0_RUNTIME_RESOURCES: SSI0RuntimeResources = SSI0RuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct SSI0MetadataResources {
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

pub const DRV_SSI0_METADATA_RESOURCES: SSI0MetadataResources = SSI0MetadataResources {
    clocks: DRV_SSI0_CLOCK_BINDINGS,
    resets: DRV_SSI0_RESET_BINDINGS,
    interrupt_sources: DRV_SSI0_INTERRUPT_SOURCES,
    interrupts: DRV_SSI0_INTERRUPT_ROUTES,
    dma_channels: DRV_SSI0_DMA_CHANNELS,
    dma: DRV_SSI0_DMA_ROUTES,
    pins: DRV_SSI0_PIN_ROLES,
    init_operations: DRV_SSI0_INIT_OPERATIONS,
    state_machines: DRV_SSI0_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_SSI0_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct SSI0;

impl SSI0 {
    pub fn new(resources: SSI0RuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> SSI0MetadataResources {
        DRV_SSI0_METADATA_RESOURCES
    }
    /// Enable the SSI0 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Disable the SSI0 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SSI0.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Release reset for SSI0.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }
}
