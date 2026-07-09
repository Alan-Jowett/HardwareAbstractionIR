//! Generated Embassy-style timer module for CH32V203G6U6.

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
    module_name: "timer",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: TIM1 (timer) from canonical block block.tim1 -> timer-advanced
pub const DRV_TIM1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim1", name: "TIM1 clock binding", consumer_ref: "periph.tim1", clock_ref: "clk.pclk2-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_TIM1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim1", name: "TIM1 reset binding", target_ref: "periph.tim1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_TIM1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim1.brk", name: "TIM1 BRK interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.up", name: "TIM1 UP interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.trg", name: "TIM1 TRG interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.com", name: "TIM1 COM interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.cc", name: "TIM1 CC interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_TIM1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim1.brk", name: "TIM1 BRK interrupt route", source_ref: "isrc.tim1.brk", interrupt_ref: "int.tim1brk", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.up", name: "TIM1 UP interrupt route", source_ref: "isrc.tim1.up", interrupt_ref: "int.tim1up", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.trg", name: "TIM1 TRG interrupt route", source_ref: "isrc.tim1.trg", interrupt_ref: "int.tim1trgcom", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.com", name: "TIM1 COM interrupt route", source_ref: "isrc.tim1.com", interrupt_ref: "int.tim1trgcom", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.cc", name: "TIM1 CC interrupt route", source_ref: "isrc.tim1.cc", interrupt_ref: "int.tim1cc", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_TIM1_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIM1_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIM1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch2.pa9.r0", name: "TIM1 CH2 on PA9 (remap 0)", pin_ref: "pin.pa9", peripheral_ref: "periph.tim1", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim1.ch2.pa9.r1", name: "TIM1 CH2 on PA9 (remap 1)", pin_ref: "pin.pa9", peripheral_ref: "periph.tim1", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch3.pa10.r0", name: "TIM1 CH3 on PA10 (remap 0)", pin_ref: "pin.pa10", peripheral_ref: "periph.tim1", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim1.ch3.pa10.r1", name: "TIM1 CH3 on PA10 (remap 1)", pin_ref: "pin.pa10", peripheral_ref: "periph.tim1", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch4.pa11.r0", name: "TIM1 CH4 on PA11 (remap 0)", pin_ref: "pin.pa11", peripheral_ref: "periph.tim1", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim1.ch4.pa11.r1", name: "TIM1 CH4 on PA11 (remap 1)", pin_ref: "pin.pa11", peripheral_ref: "periph.tim1", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.etr.pa12.r0", name: "TIM1 ETR on PA12 (remap 0)", pin_ref: "pin.pa12", peripheral_ref: "periph.tim1", signal: "ETR", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim1.etr.pa12.r1", name: "TIM1 ETR on PA12 (remap 1)", pin_ref: "pin.pa12", peripheral_ref: "periph.tim1", signal: "ETR", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.bkin.pa6.r1", name: "TIM1 BKIN on PA6 (remap 1)", pin_ref: "pin.pa6", peripheral_ref: "periph.tim1", signal: "BKIN", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch1n.pa7.r1", name: "TIM1 CH1N on PA7 (remap 1)", pin_ref: "pin.pa7", peripheral_ref: "periph.tim1", signal: "CH1N", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch2n.pb0.r1", name: "TIM1 CH2N on PB0 (remap 1)", pin_ref: "pin.pb0", peripheral_ref: "periph.tim1", signal: "CH2N", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch3n.pb1.r1", name: "TIM1 CH3N on PB1 (remap 1)", pin_ref: "pin.pb1", peripheral_ref: "periph.tim1", signal: "CH3N", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_TIM1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3", signal: "CH3", routes: DRV_TIM1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch4", signal: "CH4", routes: DRV_TIM1_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "etr", signal: "ETR", routes: DRV_TIM1_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "bkin", signal: "BKIN", routes: DRV_TIM1_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch1n", signal: "CH1N", routes: DRV_TIM1_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2n", signal: "CH2N", routes: DRV_TIM1_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3n", signal: "CH3N", routes: DRV_TIM1_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_TIM1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim1.enable", name: "TIM1 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim1"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim1.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_TIM1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim1", name: "TIM1 counter state", description: None, target_refs: &["periph.tim1"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim1.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim1.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_TIM1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM1Resources {
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

pub const DRV_TIM1_RESOURCES: TIM1Resources = TIM1Resources {
    clocks: DRV_TIM1_CLOCK_BINDINGS,
    resets: DRV_TIM1_RESET_BINDINGS,
    interrupt_sources: DRV_TIM1_INTERRUPT_SOURCES,
    interrupts: DRV_TIM1_INTERRUPT_ROUTES,
    dma_channels: DRV_TIM1_DMA_CHANNELS,
    dma: DRV_TIM1_DMA_ROUTES,
    pins: DRV_TIM1_PIN_ROLES,
    init_operations: DRV_TIM1_INIT_OPERATIONS,
    state_machines: DRV_TIM1_STATE_MACHINES,
    lowering_pattern: None,
    capability_tags: DRV_TIM1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM1 {
    resources: TIM1Resources,
}

impl TIM1 {
    pub fn new(resources: TIM1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM1Resources {
        self.resources
    }
    /// Enable the TIM1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000800u32, 0x00000800u32)?;
        Ok(())
    }

    /// Disable the TIM1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000800u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000800u32, 0x00000800u32)?;
        Ok(())
    }

    /// Release reset for TIM1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000800u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C00u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_disabled_to_enabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C00u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_enabled_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C00u64, 0x0001u16, 0x0000u16)?;
        Ok(())
    }


}

// Driver instance: TIM2 (timer) from canonical block block.tim2 -> timer-general
pub const DRV_TIM2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim2", name: "TIM2 clock binding", consumer_ref: "periph.tim2", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_TIM2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim2", name: "TIM2 reset binding", target_ref: "periph.tim2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_TIM2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim2.up", name: "TIM2 UP interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim2.trg", name: "TIM2 TRG interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim2.cc", name: "TIM2 CC interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_TIM2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim2.up", name: "TIM2 UP interrupt route", source_ref: "isrc.tim2.up", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim2.trg", name: "TIM2 TRG interrupt route", source_ref: "isrc.tim2.trg", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim2.cc", name: "TIM2 CC interrupt route", source_ref: "isrc.tim2.cc", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_TIM2_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIM2_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIM2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch1.pa0.r0", name: "TIM2 CH1 on PA0 (remap 0)", pin_ref: "pin.pa0", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch1.pa15.r1", name: "TIM2 CH1 on PA15 (remap 1)", pin_ref: "pin.pa15", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch1.pa0.r2", name: "TIM2 CH1 on PA0 (remap 2)", pin_ref: "pin.pa0", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch1.pa15.r3", name: "TIM2 CH1 on PA15 (remap 3)", pin_ref: "pin.pa15", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM2_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch2.pa1.r0", name: "TIM2 CH2 on PA1 (remap 0)", pin_ref: "pin.pa1", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch2.pb3.r1", name: "TIM2 CH2 on PB3 (remap 1)", pin_ref: "pin.pb3", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch2.pa1.r2", name: "TIM2 CH2 on PA1 (remap 2)", pin_ref: "pin.pa1", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch2.pb3.r3", name: "TIM2 CH2 on PB3 (remap 3)", pin_ref: "pin.pb3", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM2_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch3.pa2.r0", name: "TIM2 CH3 on PA2 (remap 0)", pin_ref: "pin.pa2", peripheral_ref: "periph.tim2", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch3.pa2.r1", name: "TIM2 CH3 on PA2 (remap 1)", pin_ref: "pin.pa2", peripheral_ref: "periph.tim2", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM2_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch4.pa3.r0", name: "TIM2 CH4 on PA3 (remap 0)", pin_ref: "pin.pa3", peripheral_ref: "periph.tim2", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch4.pa3.r1", name: "TIM2 CH4 on PA3 (remap 1)", pin_ref: "pin.pa3", peripheral_ref: "periph.tim2", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM2_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.etr.pa0.r0", name: "TIM2 ETR on PA0 (remap 0)", pin_ref: "pin.pa0", peripheral_ref: "periph.tim2", signal: "ETR", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.etr.pa15.r1", name: "TIM2 ETR on PA15 (remap 1)", pin_ref: "pin.pa15", peripheral_ref: "periph.tim2", signal: "ETR", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.etr.pa0.r2", name: "TIM2 ETR on PA0 (remap 2)", pin_ref: "pin.pa0", peripheral_ref: "periph.tim2", signal: "ETR", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.etr.pa15.r3", name: "TIM2 ETR on PA15 (remap 3)", pin_ref: "pin.pa15", peripheral_ref: "periph.tim2", signal: "ETR", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM2_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch1", signal: "CH1", routes: DRV_TIM2_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_TIM2_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3", signal: "CH3", routes: DRV_TIM2_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch4", signal: "CH4", routes: DRV_TIM2_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "etr", signal: "ETR", routes: DRV_TIM2_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_TIM2_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim2.enable", name: "TIM2 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim2"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim2.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_TIM2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim2", name: "TIM2 counter state", description: None, target_refs: &["periph.tim2"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim2.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim2.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_TIM2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM2Resources {
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

pub const DRV_TIM2_RESOURCES: TIM2Resources = TIM2Resources {
    clocks: DRV_TIM2_CLOCK_BINDINGS,
    resets: DRV_TIM2_RESET_BINDINGS,
    interrupt_sources: DRV_TIM2_INTERRUPT_SOURCES,
    interrupts: DRV_TIM2_INTERRUPT_ROUTES,
    dma_channels: DRV_TIM2_DMA_CHANNELS,
    dma: DRV_TIM2_DMA_ROUTES,
    pins: DRV_TIM2_PIN_ROLES,
    init_operations: DRV_TIM2_INIT_OPERATIONS,
    state_machines: DRV_TIM2_STATE_MACHINES,
    lowering_pattern: None,
    capability_tags: DRV_TIM2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM2 {
    resources: TIM2Resources,
}

impl TIM2 {
    pub fn new(resources: TIM2Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM2Resources {
        self.resources
    }
    /// Enable the TIM2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the TIM2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM2.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Release reset for TIM2.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000000u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_disabled_to_enabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000000u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_enabled_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000000u64, 0x0001u16, 0x0000u16)?;
        Ok(())
    }


}

// Driver instance: TIM3 (timer) from canonical block block.tim3 -> timer-general
pub const DRV_TIM3_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim3", name: "TIM3 clock binding", consumer_ref: "periph.tim3", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_TIM3_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim3", name: "TIM3 reset binding", target_ref: "periph.tim3", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_TIM3_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim3.up", name: "TIM3 UP interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim3.trg", name: "TIM3 TRG interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim3.cc", name: "TIM3 CC interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_TIM3_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim3.up", name: "TIM3 UP interrupt route", source_ref: "isrc.tim3.up", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim3.trg", name: "TIM3 TRG interrupt route", source_ref: "isrc.tim3.trg", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim3.cc", name: "TIM3 CC interrupt route", source_ref: "isrc.tim3.cc", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_TIM3_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIM3_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIM3_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch1.pa6.r0", name: "TIM3 CH1 on PA6 (remap 0)", pin_ref: "pin.pa6", peripheral_ref: "periph.tim3", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch1.pb4.r2", name: "TIM3 CH1 on PB4 (remap 2)", pin_ref: "pin.pb4", peripheral_ref: "periph.tim3", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM3_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch2.pa7.r0", name: "TIM3 CH2 on PA7 (remap 0)", pin_ref: "pin.pa7", peripheral_ref: "periph.tim3", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch2.pb5.r2", name: "TIM3 CH2 on PB5 (remap 2)", pin_ref: "pin.pb5", peripheral_ref: "periph.tim3", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM3_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch3.pb0.r0", name: "TIM3 CH3 on PB0 (remap 0)", pin_ref: "pin.pb0", peripheral_ref: "periph.tim3", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch3.pb0.r2", name: "TIM3 CH3 on PB0 (remap 2)", pin_ref: "pin.pb0", peripheral_ref: "periph.tim3", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM3_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch4.pb1.r0", name: "TIM3 CH4 on PB1 (remap 0)", pin_ref: "pin.pb1", peripheral_ref: "periph.tim3", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch4.pb1.r2", name: "TIM3 CH4 on PB1 (remap 2)", pin_ref: "pin.pb1", peripheral_ref: "periph.tim3", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM3_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch1", signal: "CH1", routes: DRV_TIM3_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_TIM3_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3", signal: "CH3", routes: DRV_TIM3_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch4", signal: "CH4", routes: DRV_TIM3_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_TIM3_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim3.enable", name: "TIM3 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim3"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim3.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_TIM3_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim3", name: "TIM3 counter state", description: None, target_refs: &["periph.tim3"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim3.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim3.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_TIM3_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM3Resources {
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

pub const DRV_TIM3_RESOURCES: TIM3Resources = TIM3Resources {
    clocks: DRV_TIM3_CLOCK_BINDINGS,
    resets: DRV_TIM3_RESET_BINDINGS,
    interrupt_sources: DRV_TIM3_INTERRUPT_SOURCES,
    interrupts: DRV_TIM3_INTERRUPT_ROUTES,
    dma_channels: DRV_TIM3_DMA_CHANNELS,
    dma: DRV_TIM3_DMA_ROUTES,
    pins: DRV_TIM3_PIN_ROLES,
    init_operations: DRV_TIM3_INIT_OPERATIONS,
    state_machines: DRV_TIM3_STATE_MACHINES,
    lowering_pattern: None,
    capability_tags: DRV_TIM3_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM3 {
    resources: TIM3Resources,
}

impl TIM3 {
    pub fn new(resources: TIM3Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM3Resources {
        self.resources
    }
    /// Enable the TIM3 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the TIM3 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM3.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Release reset for TIM3.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000400u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_disabled_to_enabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000400u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_enabled_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000400u64, 0x0001u16, 0x0000u16)?;
        Ok(())
    }


}

// Driver instance: TIM4 (timer) from canonical block block.tim4 -> timer-general
pub const DRV_TIM4_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim4", name: "TIM4 clock binding", consumer_ref: "periph.tim4", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_TIM4_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim4", name: "TIM4 reset binding", target_ref: "periph.tim4", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_TIM4_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim4.up", name: "TIM4 UP interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim4.trg", name: "TIM4 TRG interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim4.cc", name: "TIM4 CC interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_TIM4_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim4.up", name: "TIM4 UP interrupt route", source_ref: "isrc.tim4.up", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim4.trg", name: "TIM4 TRG interrupt route", source_ref: "isrc.tim4.trg", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim4.cc", name: "TIM4 CC interrupt route", source_ref: "isrc.tim4.cc", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_TIM4_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIM4_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIM4_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim4.ch1.pb6", name: "TIM4 CH1 on PB6", pin_ref: "pin.pb6", peripheral_ref: "periph.tim4", signal: "CH1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM4_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim4.ch2.pb7", name: "TIM4 CH2 on PB7", pin_ref: "pin.pb7", peripheral_ref: "periph.tim4", signal: "CH2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_TIM4_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch1", signal: "CH1", routes: DRV_TIM4_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_TIM4_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_TIM4_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim4.enable", name: "TIM4 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim4"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim4.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_TIM4_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim4", name: "TIM4 counter state", description: None, target_refs: &["periph.tim4"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim4.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim4.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_TIM4_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM4Resources {
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

pub const DRV_TIM4_RESOURCES: TIM4Resources = TIM4Resources {
    clocks: DRV_TIM4_CLOCK_BINDINGS,
    resets: DRV_TIM4_RESET_BINDINGS,
    interrupt_sources: DRV_TIM4_INTERRUPT_SOURCES,
    interrupts: DRV_TIM4_INTERRUPT_ROUTES,
    dma_channels: DRV_TIM4_DMA_CHANNELS,
    dma: DRV_TIM4_DMA_ROUTES,
    pins: DRV_TIM4_PIN_ROLES,
    init_operations: DRV_TIM4_INIT_OPERATIONS,
    state_machines: DRV_TIM4_STATE_MACHINES,
    lowering_pattern: None,
    capability_tags: DRV_TIM4_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM4 {
    resources: TIM4Resources,
}

impl TIM4 {
    pub fn new(resources: TIM4Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM4Resources {
        self.resources
    }
    /// Enable the TIM4 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the TIM4 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM4.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for TIM4.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000800u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_disabled_to_enabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000800u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_enabled_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000800u64, 0x0001u16, 0x0000u16)?;
        Ok(())
    }


}

