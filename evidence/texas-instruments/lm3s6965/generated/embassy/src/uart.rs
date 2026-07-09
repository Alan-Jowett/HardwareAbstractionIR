//! Generated Embassy-style uart module for LM3S6965.

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
    module_name: "uart",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: UART0 (uart) from canonical block block.uart0 -> uart
pub const DRV_UART0_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.uart0", name: "UART0", consumer_ref: "periph.uart0", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_UART0_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.uart0", name: "UART0", target_ref: "periph.uart0", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_UART0_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.uart0", name: "UART0 interrupt source", source_ref: "periph.uart0", producer_ref: Some("periph.uart0"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_UART0_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.uart0", name: "UART0 interrupt source route", source_ref: "isrc.uart0", interrupt_ref: "int.uart0", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_UART0_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_UART0_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_UART0_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart0.rx.pa0", name: "UART0 RX on PA0", pin_ref: "pin.pa0", peripheral_ref: "periph.uart0", signal: "RX", route_type: "hardwired", control_refs: &["reg.gpioa.afsel", "reg.gpioa.den"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_UART0_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart0.tx.pa1", name: "UART0 TX on PA1", pin_ref: "pin.pa1", peripheral_ref: "periph.uart0", signal: "TX", route_type: "hardwired", control_refs: &["reg.gpioa.afsel", "reg.gpioa.den"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_UART0_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "rx", signal: "RX", routes: DRV_UART0_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "tx", signal: "TX", routes: DRV_UART0_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_UART0_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_UART0_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_UART0_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct UART0Resources {
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

pub const DRV_UART0_RESOURCES: UART0Resources = UART0Resources {
    clocks: DRV_UART0_CLOCK_BINDINGS,
    resets: DRV_UART0_RESET_BINDINGS,
    interrupt_sources: DRV_UART0_INTERRUPT_SOURCES,
    interrupts: DRV_UART0_INTERRUPT_ROUTES,
    dma_channels: DRV_UART0_DMA_CHANNELS,
    dma: DRV_UART0_DMA_ROUTES,
    pins: DRV_UART0_PIN_ROLES,
    init_operations: DRV_UART0_INIT_OPERATIONS,
    state_machines: DRV_UART0_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_UART0_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct UART0 {
    resources: UART0Resources,
}

impl UART0 {
    pub fn new(resources: UART0Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> UART0Resources {
        self.resources
    }
    /// Enable the UART0 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the UART0 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART0.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Release reset for UART0.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: UART1 (uart) from canonical block block.uart1 -> uart
pub const DRV_UART1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.uart1", name: "UART1", consumer_ref: "periph.uart1", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_UART1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.uart1", name: "UART1", target_ref: "periph.uart1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_UART1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.uart1", name: "UART1 interrupt source", source_ref: "periph.uart1", producer_ref: Some("periph.uart1"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_UART1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.uart1", name: "UART1 interrupt source route", source_ref: "isrc.uart1", interrupt_ref: "int.uart1", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_UART1_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_UART1_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_UART1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart1.rx.pd2", name: "UART1 RX on PD2", pin_ref: "pin.pd2", peripheral_ref: "periph.uart1", signal: "RX", route_type: "hardwired", control_refs: &["reg.gpiod.afsel", "reg.gpiod.den"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(false) }];
pub const DRV_UART1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart1.tx.pd3", name: "UART1 TX on PD3", pin_ref: "pin.pd3", peripheral_ref: "periph.uart1", signal: "TX", route_type: "hardwired", control_refs: &["reg.gpiod.afsel", "reg.gpiod.den"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(false) }];
pub const DRV_UART1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "rx", signal: "RX", routes: DRV_UART1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "tx", signal: "TX", routes: DRV_UART1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_UART1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_UART1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_UART1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct UART1Resources {
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

pub const DRV_UART1_RESOURCES: UART1Resources = UART1Resources {
    clocks: DRV_UART1_CLOCK_BINDINGS,
    resets: DRV_UART1_RESET_BINDINGS,
    interrupt_sources: DRV_UART1_INTERRUPT_SOURCES,
    interrupts: DRV_UART1_INTERRUPT_ROUTES,
    dma_channels: DRV_UART1_DMA_CHANNELS,
    dma: DRV_UART1_DMA_ROUTES,
    pins: DRV_UART1_PIN_ROLES,
    init_operations: DRV_UART1_INIT_OPERATIONS,
    state_machines: DRV_UART1_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_UART1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct UART1 {
    resources: UART1Resources,
}

impl UART1 {
    pub fn new(resources: UART1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> UART1Resources {
        self.resources
    }
    /// Enable the UART1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the UART1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Release reset for UART1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }


}

