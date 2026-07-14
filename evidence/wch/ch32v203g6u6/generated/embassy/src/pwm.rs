//! Generated Embassy-style pwm module for CH32V203G6U6.

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

impl embedded_hal::pwm::Error for metadata::Error {
    fn kind(&self) -> embedded_hal::pwm::ErrorKind {
        embedded_hal::pwm::ErrorKind::Other
    }
}

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "pwm",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: TIM1 PWM (pwm) from canonical block block.pwm-tim1 -> pwm
pub const DRV_PWM_TIM1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim1", name: "TIM1 clock binding", consumer_ref: "periph.tim1", clock_ref: "clk.pclk2-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_PWM_TIM1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim1", name: "TIM1 reset binding", target_ref: "periph.tim1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_PWM_TIM1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim1.brk", name: "TIM1 BRK interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.up", name: "TIM1 UP interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.trg", name: "TIM1 TRG interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.com", name: "TIM1 COM interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.cc", name: "TIM1 CC interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_PWM_TIM1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim1.brk", name: "TIM1 BRK interrupt route", source_ref: "isrc.tim1.brk", interrupt_ref: "int.tim1brk", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.up", name: "TIM1 UP interrupt route", source_ref: "isrc.tim1.up", interrupt_ref: "int.tim1up", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.trg", name: "TIM1 TRG interrupt route", source_ref: "isrc.tim1.trg", interrupt_ref: "int.tim1trgcom", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.com", name: "TIM1 COM interrupt route", source_ref: "isrc.tim1.com", interrupt_ref: "int.tim1trgcom", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.cc", name: "TIM1 CC interrupt route", source_ref: "isrc.tim1.cc", interrupt_ref: "int.tim1cc", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_PWM_TIM1_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_PWM_TIM1_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_PWM_TIM1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch2.pa9.r0", name: "TIM1 CH2 on PA9 (remap 0)", pin_ref: "pin.pa9", peripheral_ref: "periph.tim1", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim1.ch2.pa9.r1", name: "TIM1 CH2 on PA9 (remap 1)", pin_ref: "pin.pa9", peripheral_ref: "periph.tim1", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch3.pa10.r0", name: "TIM1 CH3 on PA10 (remap 0)", pin_ref: "pin.pa10", peripheral_ref: "periph.tim1", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim1.ch3.pa10.r1", name: "TIM1 CH3 on PA10 (remap 1)", pin_ref: "pin.pa10", peripheral_ref: "periph.tim1", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch4.pa11.r0", name: "TIM1 CH4 on PA11 (remap 0)", pin_ref: "pin.pa11", peripheral_ref: "periph.tim1", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim1.ch4.pa11.r1", name: "TIM1 CH4 on PA11 (remap 1)", pin_ref: "pin.pa11", peripheral_ref: "periph.tim1", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM1_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch1n.pa7.r1", name: "TIM1 CH1N on PA7 (remap 1)", pin_ref: "pin.pa7", peripheral_ref: "periph.tim1", signal: "CH1N", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM1_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch2n.pb0.r1", name: "TIM1 CH2N on PB0 (remap 1)", pin_ref: "pin.pb0", peripheral_ref: "periph.tim1", signal: "CH2N", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM1_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim1.ch3n.pb1.r1", name: "TIM1 CH3N on PB1 (remap 1)", pin_ref: "pin.pb1", peripheral_ref: "periph.tim1", signal: "CH3N", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_PWM_TIM1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3", signal: "CH3", routes: DRV_PWM_TIM1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch4", signal: "CH4", routes: DRV_PWM_TIM1_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch1n", signal: "CH1N", routes: DRV_PWM_TIM1_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2n", signal: "CH2N", routes: DRV_PWM_TIM1_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3n", signal: "CH3N", routes: DRV_PWM_TIM1_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_PWM_TIM1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim1.enable", name: "TIM1 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim1"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim1.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_PWM_TIM1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim1", name: "TIM1 counter state", description: None, target_refs: &["periph.tim1"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim1.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim1.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_PWM_TIM1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM1PWMResources {
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

pub const DRV_PWM_TIM1_RESOURCES: TIM1PWMResources = TIM1PWMResources {
    clocks: DRV_PWM_TIM1_CLOCK_BINDINGS,
    resets: DRV_PWM_TIM1_RESET_BINDINGS,
    interrupt_sources: DRV_PWM_TIM1_INTERRUPT_SOURCES,
    interrupts: DRV_PWM_TIM1_INTERRUPT_ROUTES,
    dma_channels: DRV_PWM_TIM1_DMA_CHANNELS,
    dma: DRV_PWM_TIM1_DMA_ROUTES,
    pins: DRV_PWM_TIM1_PIN_ROLES,
    init_operations: DRV_PWM_TIM1_INIT_OPERATIONS,
    state_machines: DRV_PWM_TIM1_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_PWM_TIM1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM1PWM {
    resources: TIM1PWMResources,
}

impl TIM1PWM {
    pub fn new(resources: TIM1PWMResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM1PWMResources {
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

    pub fn set_prescaler(&self, prescaler: u16) -> Result<(), metadata::Error> {
        write_u16(0x40012C28u64, prescaler)?;
        Ok(())
    }

    pub fn set_auto_reload(&self, reload: u16) -> Result<(), metadata::Error> {
        write_u16(0x40012C2Cu64, reload)?;
        Ok(())
    }

    pub fn set_counter(&self, counter: u16) -> Result<(), metadata::Error> {
        write_u16(0x40012C24u64, counter)?;
        Ok(())
    }

    pub fn generate_update(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C14u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    /// Enable auto-reload buffering for TIM1 PWM.
    pub fn enable_auto_reload_preload(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C00u64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    /// Enable the main output gate for TIM1 PWM.
    pub fn enable_main_output(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C44u64, 0x8000u16, 0x8000u16)?;
        Ok(())
    }

    /// Disable the main output gate for TIM1 PWM.
    pub fn disable_main_output(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C44u64, 0x8000u16, 0x0000u16)?;
        Ok(())
    }

    pub fn configure_ch2_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C18u64, 0x0300u16, 0x0000u16)?;
        modify_u16(0x40012C18u64, 0x0800u16, 0x0800u16)?;
        modify_u16(0x40012C18u64, 0x7000u16, 0x6000u16)?;
        Ok(())
    }

    pub fn channel_ch2(&self) -> TIM1PWMCh2 {
        TIM1PWMCh2 {
            compare_addr: 0x40012C38u64,
            auto_reload_addr: 0x40012C2Cu64,
            enable_addr: 0x40012C20u64,
            enable_clear_mask: 0x0010u16,
            enable_set_mask: 0x0010u16,
        }
    }

    /// Configure PA9 for the TIM1 PWM CH2 output.
    pub fn configure_ch2_pa9_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x000000C0u32, 0x00000000u32)?;
        modify_u32(0x40010804u64, 0x000000F0u32, 0x000000B0u32)?;
        Ok(())
    }

    pub fn configure_ch3_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C1Cu64, 0x0003u16, 0x0000u16)?;
        modify_u16(0x40012C1Cu64, 0x0008u16, 0x0008u16)?;
        modify_u16(0x40012C1Cu64, 0x0070u16, 0x0060u16)?;
        Ok(())
    }

    pub fn channel_ch3(&self) -> TIM1PWMCh3 {
        TIM1PWMCh3 {
            compare_addr: 0x40012C3Cu64,
            auto_reload_addr: 0x40012C2Cu64,
            enable_addr: 0x40012C20u64,
            enable_clear_mask: 0x0100u16,
            enable_set_mask: 0x0100u16,
        }
    }

    /// Configure PA10 for the TIM1 PWM CH3 output.
    pub fn configure_ch3_pa10_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x000000C0u32, 0x00000000u32)?;
        modify_u32(0x40010804u64, 0x00000F00u32, 0x00000B00u32)?;
        Ok(())
    }

    pub fn configure_ch4_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40012C1Cu64, 0x0300u16, 0x0000u16)?;
        modify_u16(0x40012C1Cu64, 0x0800u16, 0x0800u16)?;
        modify_u16(0x40012C1Cu64, 0x7000u16, 0x6000u16)?;
        Ok(())
    }

    pub fn channel_ch4(&self) -> TIM1PWMCh4 {
        TIM1PWMCh4 {
            compare_addr: 0x40012C40u64,
            auto_reload_addr: 0x40012C2Cu64,
            enable_addr: 0x40012C20u64,
            enable_clear_mask: 0x1000u16,
            enable_set_mask: 0x1000u16,
        }
    }

    /// Configure PA11 for the TIM1 PWM CH4 output.
    pub fn configure_ch4_pa11_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x000000C0u32, 0x00000000u32)?;
        modify_u32(0x40010804u64, 0x0000F000u32, 0x0000B000u32)?;
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

#[derive(Debug, Clone, Copy)]
pub struct TIM1PWMCh2 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM1PWMCh2 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM1PWMCh2 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM1PWMCh2 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM1PWMCh3 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM1PWMCh3 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM1PWMCh3 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM1PWMCh3 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM1PWMCh4 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM1PWMCh4 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM1PWMCh4 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM1PWMCh4 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}
// Driver instance: TIM2 PWM (pwm) from canonical block block.pwm-tim2 -> pwm
pub const DRV_PWM_TIM2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim2", name: "TIM2 clock binding", consumer_ref: "periph.tim2", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_PWM_TIM2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim2", name: "TIM2 reset binding", target_ref: "periph.tim2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_PWM_TIM2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim2.up", name: "TIM2 UP interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim2.trg", name: "TIM2 TRG interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim2.cc", name: "TIM2 CC interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_PWM_TIM2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim2.up", name: "TIM2 UP interrupt route", source_ref: "isrc.tim2.up", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim2.trg", name: "TIM2 TRG interrupt route", source_ref: "isrc.tim2.trg", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim2.cc", name: "TIM2 CC interrupt route", source_ref: "isrc.tim2.cc", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_PWM_TIM2_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_PWM_TIM2_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_PWM_TIM2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch1.pa0.r0", name: "TIM2 CH1 on PA0 (remap 0)", pin_ref: "pin.pa0", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch1.pa15.r1", name: "TIM2 CH1 on PA15 (remap 1)", pin_ref: "pin.pa15", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch1.pa0.r2", name: "TIM2 CH1 on PA0 (remap 2)", pin_ref: "pin.pa0", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch1.pa15.r3", name: "TIM2 CH1 on PA15 (remap 3)", pin_ref: "pin.pa15", peripheral_ref: "periph.tim2", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM2_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch2.pa1.r0", name: "TIM2 CH2 on PA1 (remap 0)", pin_ref: "pin.pa1", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch2.pb3.r1", name: "TIM2 CH2 on PB3 (remap 1)", pin_ref: "pin.pb3", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch2.pa1.r2", name: "TIM2 CH2 on PA1 (remap 2)", pin_ref: "pin.pa1", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.tim2.ch2.pb3.r3", name: "TIM2 CH2 on PB3 (remap 3)", pin_ref: "pin.pb3", peripheral_ref: "periph.tim2", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM2_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch3.pa2.r0", name: "TIM2 CH3 on PA2 (remap 0)", pin_ref: "pin.pa2", peripheral_ref: "periph.tim2", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch3.pa2.r1", name: "TIM2 CH3 on PA2 (remap 1)", pin_ref: "pin.pa2", peripheral_ref: "periph.tim2", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM2_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim2.ch4.pa3.r0", name: "TIM2 CH4 on PA3 (remap 0)", pin_ref: "pin.pa3", peripheral_ref: "periph.tim2", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim2.ch4.pa3.r1", name: "TIM2 CH4 on PA3 (remap 1)", pin_ref: "pin.pa3", peripheral_ref: "periph.tim2", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM2_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch1", signal: "CH1", routes: DRV_PWM_TIM2_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_PWM_TIM2_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3", signal: "CH3", routes: DRV_PWM_TIM2_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch4", signal: "CH4", routes: DRV_PWM_TIM2_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_PWM_TIM2_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim2.enable", name: "TIM2 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim2"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim2.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_PWM_TIM2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim2", name: "TIM2 counter state", description: None, target_refs: &["periph.tim2"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim2.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim2.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_PWM_TIM2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM2PWMResources {
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

pub const DRV_PWM_TIM2_RESOURCES: TIM2PWMResources = TIM2PWMResources {
    clocks: DRV_PWM_TIM2_CLOCK_BINDINGS,
    resets: DRV_PWM_TIM2_RESET_BINDINGS,
    interrupt_sources: DRV_PWM_TIM2_INTERRUPT_SOURCES,
    interrupts: DRV_PWM_TIM2_INTERRUPT_ROUTES,
    dma_channels: DRV_PWM_TIM2_DMA_CHANNELS,
    dma: DRV_PWM_TIM2_DMA_ROUTES,
    pins: DRV_PWM_TIM2_PIN_ROLES,
    init_operations: DRV_PWM_TIM2_INIT_OPERATIONS,
    state_machines: DRV_PWM_TIM2_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_PWM_TIM2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM2PWM {
    resources: TIM2PWMResources,
}

impl TIM2PWM {
    pub fn new(resources: TIM2PWMResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM2PWMResources {
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

    pub fn set_prescaler(&self, prescaler: u16) -> Result<(), metadata::Error> {
        write_u16(0x40000028u64, prescaler)?;
        Ok(())
    }

    pub fn set_auto_reload(&self, reload: u16) -> Result<(), metadata::Error> {
        write_u32(0x4000002Cu64, u32::from(reload))?;
        Ok(())
    }

    pub fn set_counter(&self, counter: u16) -> Result<(), metadata::Error> {
        write_u32(0x40000024u64, u32::from(counter))?;
        Ok(())
    }

    pub fn generate_update(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000014u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    /// Enable auto-reload buffering for TIM2 PWM.
    pub fn enable_auto_reload_preload(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000000u64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    pub fn configure_ch1_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000018u64, 0x0003u16, 0x0000u16)?;
        modify_u16(0x40000018u64, 0x0008u16, 0x0008u16)?;
        modify_u16(0x40000018u64, 0x0070u16, 0x0060u16)?;
        Ok(())
    }

    pub fn channel_ch1(&self) -> TIM2PWMCh1 {
        TIM2PWMCh1 {
            compare_addr: 0x40000034u64,
            auto_reload_addr: 0x4000002Cu64,
            enable_addr: 0x40000020u64,
            enable_clear_mask: 0x0001u16,
            enable_set_mask: 0x0001u16,
        }
    }

    /// Configure PA0 for the TIM2 PWM CH1 output.
    pub fn configure_ch1_pa0_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000300u32, 0x00000000u32)?;
        modify_u32(0x40010800u64, 0x0000000Fu32, 0x0000000Bu32)?;
        Ok(())
    }

    pub fn configure_ch2_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000018u64, 0x0300u16, 0x0000u16)?;
        modify_u16(0x40000018u64, 0x0800u16, 0x0800u16)?;
        modify_u16(0x40000018u64, 0x7000u16, 0x6000u16)?;
        Ok(())
    }

    pub fn channel_ch2(&self) -> TIM2PWMCh2 {
        TIM2PWMCh2 {
            compare_addr: 0x40000038u64,
            auto_reload_addr: 0x4000002Cu64,
            enable_addr: 0x40000020u64,
            enable_clear_mask: 0x0010u16,
            enable_set_mask: 0x0010u16,
        }
    }

    /// Configure PA1 for the TIM2 PWM CH2 output.
    pub fn configure_ch2_pa1_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000300u32, 0x00000000u32)?;
        modify_u32(0x40010800u64, 0x000000F0u32, 0x000000B0u32)?;
        Ok(())
    }

    pub fn configure_ch3_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000001Cu64, 0x0003u16, 0x0000u16)?;
        modify_u16(0x4000001Cu64, 0x0008u16, 0x0008u16)?;
        modify_u16(0x4000001Cu64, 0x0070u16, 0x0060u16)?;
        Ok(())
    }

    pub fn channel_ch3(&self) -> TIM2PWMCh3 {
        TIM2PWMCh3 {
            compare_addr: 0x4000003Cu64,
            auto_reload_addr: 0x4000002Cu64,
            enable_addr: 0x40000020u64,
            enable_clear_mask: 0x0100u16,
            enable_set_mask: 0x0100u16,
        }
    }

    /// Configure PA2 for the TIM2 PWM CH3 output.
    pub fn configure_ch3_pa2_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000300u32, 0x00000000u32)?;
        modify_u32(0x40010800u64, 0x00000F00u32, 0x00000B00u32)?;
        Ok(())
    }

    pub fn configure_ch4_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000001Cu64, 0x0300u16, 0x0000u16)?;
        modify_u16(0x4000001Cu64, 0x0800u16, 0x0800u16)?;
        modify_u16(0x4000001Cu64, 0x7000u16, 0x6000u16)?;
        Ok(())
    }

    pub fn channel_ch4(&self) -> TIM2PWMCh4 {
        TIM2PWMCh4 {
            compare_addr: 0x40000040u64,
            auto_reload_addr: 0x4000002Cu64,
            enable_addr: 0x40000020u64,
            enable_clear_mask: 0x1000u16,
            enable_set_mask: 0x1000u16,
        }
    }

    /// Configure PA3 for the TIM2 PWM CH4 output.
    pub fn configure_ch4_pa3_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000300u32, 0x00000000u32)?;
        modify_u32(0x40010800u64, 0x0000F000u32, 0x0000B000u32)?;
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

#[derive(Debug, Clone, Copy)]
pub struct TIM2PWMCh1 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM2PWMCh1 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM2PWMCh1 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM2PWMCh1 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u32>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u32) } as u16;
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u32(self.compare_addr, u32::from(duty))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM2PWMCh2 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM2PWMCh2 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM2PWMCh2 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM2PWMCh2 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u32>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u32) } as u16;
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u32(self.compare_addr, u32::from(duty))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM2PWMCh3 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM2PWMCh3 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM2PWMCh3 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM2PWMCh3 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u32>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u32) } as u16;
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u32(self.compare_addr, u32::from(duty))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM2PWMCh4 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM2PWMCh4 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM2PWMCh4 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM2PWMCh4 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u32>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u32) } as u16;
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u32(self.compare_addr, u32::from(duty))?;
        Ok(())
    }
}
// Driver instance: TIM3 PWM (pwm) from canonical block block.pwm-tim3 -> pwm
pub const DRV_PWM_TIM3_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim3", name: "TIM3 clock binding", consumer_ref: "periph.tim3", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_PWM_TIM3_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim3", name: "TIM3 reset binding", target_ref: "periph.tim3", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_PWM_TIM3_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim3.up", name: "TIM3 UP interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim3.trg", name: "TIM3 TRG interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim3.cc", name: "TIM3 CC interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_PWM_TIM3_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim3.up", name: "TIM3 UP interrupt route", source_ref: "isrc.tim3.up", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim3.trg", name: "TIM3 TRG interrupt route", source_ref: "isrc.tim3.trg", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim3.cc", name: "TIM3 CC interrupt route", source_ref: "isrc.tim3.cc", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_PWM_TIM3_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_PWM_TIM3_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_PWM_TIM3_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch1.pa6.r0", name: "TIM3 CH1 on PA6 (remap 0)", pin_ref: "pin.pa6", peripheral_ref: "periph.tim3", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch1.pb4.r2", name: "TIM3 CH1 on PB4 (remap 2)", pin_ref: "pin.pb4", peripheral_ref: "periph.tim3", signal: "CH1", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM3_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch2.pa7.r0", name: "TIM3 CH2 on PA7 (remap 0)", pin_ref: "pin.pa7", peripheral_ref: "periph.tim3", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch2.pb5.r2", name: "TIM3 CH2 on PB5 (remap 2)", pin_ref: "pin.pb5", peripheral_ref: "periph.tim3", signal: "CH2", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM3_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch3.pb0.r0", name: "TIM3 CH3 on PB0 (remap 0)", pin_ref: "pin.pb0", peripheral_ref: "periph.tim3", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch3.pb0.r2", name: "TIM3 CH3 on PB0 (remap 2)", pin_ref: "pin.pb0", peripheral_ref: "periph.tim3", signal: "CH3", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM3_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim3.ch4.pb1.r0", name: "TIM3 CH4 on PB1 (remap 0)", pin_ref: "pin.pb1", peripheral_ref: "periph.tim3", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.tim3.ch4.pb1.r2", name: "TIM3 CH4 on PB1 (remap 2)", pin_ref: "pin.pb1", peripheral_ref: "periph.tim3", signal: "CH4", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM3_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch1", signal: "CH1", routes: DRV_PWM_TIM3_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_PWM_TIM3_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch3", signal: "CH3", routes: DRV_PWM_TIM3_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch4", signal: "CH4", routes: DRV_PWM_TIM3_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_PWM_TIM3_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim3.enable", name: "TIM3 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim3"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim3.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_PWM_TIM3_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim3", name: "TIM3 counter state", description: None, target_refs: &["periph.tim3"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim3.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim3.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_PWM_TIM3_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM3PWMResources {
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

pub const DRV_PWM_TIM3_RESOURCES: TIM3PWMResources = TIM3PWMResources {
    clocks: DRV_PWM_TIM3_CLOCK_BINDINGS,
    resets: DRV_PWM_TIM3_RESET_BINDINGS,
    interrupt_sources: DRV_PWM_TIM3_INTERRUPT_SOURCES,
    interrupts: DRV_PWM_TIM3_INTERRUPT_ROUTES,
    dma_channels: DRV_PWM_TIM3_DMA_CHANNELS,
    dma: DRV_PWM_TIM3_DMA_ROUTES,
    pins: DRV_PWM_TIM3_PIN_ROLES,
    init_operations: DRV_PWM_TIM3_INIT_OPERATIONS,
    state_machines: DRV_PWM_TIM3_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_PWM_TIM3_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM3PWM {
    resources: TIM3PWMResources,
}

impl TIM3PWM {
    pub fn new(resources: TIM3PWMResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM3PWMResources {
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

    pub fn set_prescaler(&self, prescaler: u16) -> Result<(), metadata::Error> {
        write_u16(0x40000428u64, prescaler)?;
        Ok(())
    }

    pub fn set_auto_reload(&self, reload: u16) -> Result<(), metadata::Error> {
        write_u16(0x4000042Cu64, reload)?;
        Ok(())
    }

    pub fn set_counter(&self, counter: u16) -> Result<(), metadata::Error> {
        write_u16(0x40000424u64, counter)?;
        Ok(())
    }

    pub fn generate_update(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000414u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    /// Enable auto-reload buffering for TIM3 PWM.
    pub fn enable_auto_reload_preload(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000400u64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    pub fn configure_ch1_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000418u64, 0x0003u16, 0x0000u16)?;
        modify_u16(0x40000418u64, 0x0008u16, 0x0008u16)?;
        modify_u16(0x40000418u64, 0x0070u16, 0x0060u16)?;
        Ok(())
    }

    pub fn channel_ch1(&self) -> TIM3PWMCh1 {
        TIM3PWMCh1 {
            compare_addr: 0x40000434u64,
            auto_reload_addr: 0x4000042Cu64,
            enable_addr: 0x40000420u64,
            enable_clear_mask: 0x0001u16,
            enable_set_mask: 0x0001u16,
        }
    }

    /// Configure PA6 for the TIM3 PWM CH1 output.
    pub fn configure_ch1_pa6_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000C00u32, 0x00000000u32)?;
        modify_u32(0x40010800u64, 0x0F000000u32, 0x0B000000u32)?;
        Ok(())
    }

    pub fn configure_ch2_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000418u64, 0x0300u16, 0x0000u16)?;
        modify_u16(0x40000418u64, 0x0800u16, 0x0800u16)?;
        modify_u16(0x40000418u64, 0x7000u16, 0x6000u16)?;
        Ok(())
    }

    pub fn channel_ch2(&self) -> TIM3PWMCh2 {
        TIM3PWMCh2 {
            compare_addr: 0x40000438u64,
            auto_reload_addr: 0x4000042Cu64,
            enable_addr: 0x40000420u64,
            enable_clear_mask: 0x0010u16,
            enable_set_mask: 0x0010u16,
        }
    }

    /// Configure PA7 for the TIM3 PWM CH2 output.
    pub fn configure_ch2_pa7_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000C00u32, 0x00000000u32)?;
        modify_u32(0x40010800u64, 0xF0000000u32, 0xB0000000u32)?;
        Ok(())
    }

    pub fn configure_ch3_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000041Cu64, 0x0003u16, 0x0000u16)?;
        modify_u16(0x4000041Cu64, 0x0008u16, 0x0008u16)?;
        modify_u16(0x4000041Cu64, 0x0070u16, 0x0060u16)?;
        Ok(())
    }

    pub fn channel_ch3(&self) -> TIM3PWMCh3 {
        TIM3PWMCh3 {
            compare_addr: 0x4000043Cu64,
            auto_reload_addr: 0x4000042Cu64,
            enable_addr: 0x40000420u64,
            enable_clear_mask: 0x0100u16,
            enable_set_mask: 0x0100u16,
        }
    }

    /// Configure PB0 for the TIM3 PWM CH3 output.
    pub fn configure_ch3_pb0_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000C00u32, 0x00000000u32)?;
        modify_u32(0x40010C00u64, 0x0000000Fu32, 0x0000000Bu32)?;
        Ok(())
    }

    pub fn configure_ch4_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000041Cu64, 0x0300u16, 0x0000u16)?;
        modify_u16(0x4000041Cu64, 0x0800u16, 0x0800u16)?;
        modify_u16(0x4000041Cu64, 0x7000u16, 0x6000u16)?;
        Ok(())
    }

    pub fn channel_ch4(&self) -> TIM3PWMCh4 {
        TIM3PWMCh4 {
            compare_addr: 0x40000440u64,
            auto_reload_addr: 0x4000042Cu64,
            enable_addr: 0x40000420u64,
            enable_clear_mask: 0x1000u16,
            enable_set_mask: 0x1000u16,
        }
    }

    /// Configure PB1 for the TIM3 PWM CH4 output.
    pub fn configure_ch4_pb1_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010004u64, 0x00000C00u32, 0x00000000u32)?;
        modify_u32(0x40010C00u64, 0x000000F0u32, 0x000000B0u32)?;
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

#[derive(Debug, Clone, Copy)]
pub struct TIM3PWMCh1 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM3PWMCh1 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM3PWMCh1 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM3PWMCh1 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM3PWMCh2 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM3PWMCh2 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM3PWMCh2 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM3PWMCh2 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM3PWMCh3 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM3PWMCh3 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM3PWMCh3 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM3PWMCh3 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM3PWMCh4 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM3PWMCh4 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM3PWMCh4 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM3PWMCh4 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}
// Driver instance: TIM4 PWM (pwm) from canonical block block.pwm-tim4 -> pwm
pub const DRV_PWM_TIM4_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim4", name: "TIM4 clock binding", consumer_ref: "periph.tim4", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_PWM_TIM4_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim4", name: "TIM4 reset binding", target_ref: "periph.tim4", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_PWM_TIM4_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim4.up", name: "TIM4 UP interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim4.trg", name: "TIM4 TRG interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim4.cc", name: "TIM4 CC interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &["op.tim4.clear_cc1"] }];
pub const DRV_PWM_TIM4_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim4.up", name: "TIM4 UP interrupt route", source_ref: "isrc.tim4.up", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim4.trg", name: "TIM4 TRG interrupt route", source_ref: "isrc.tim4.trg", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim4.cc", name: "TIM4 CC interrupt route", source_ref: "isrc.tim4.cc", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_PWM_TIM4_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_PWM_TIM4_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_PWM_TIM4_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim4.ch1.pb6", name: "TIM4 CH1 on PB6", pin_ref: "pin.pb6", peripheral_ref: "periph.tim4", signal: "CH1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM4_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.tim4.ch2.pb7", name: "TIM4 CH2 on PB7", pin_ref: "pin.pb7", peripheral_ref: "periph.tim4", signal: "CH2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_PWM_TIM4_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "ch1", signal: "CH1", routes: DRV_PWM_TIM4_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "ch2", signal: "CH2", routes: DRV_PWM_TIM4_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_PWM_TIM4_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim4.enable", name: "TIM4 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim4"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim4.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_PWM_TIM4_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim4", name: "TIM4 counter state", description: None, target_refs: &["periph.tim4"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim4.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim4.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_PWM_TIM4_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct TIM4PWMResources {
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

pub const DRV_PWM_TIM4_RESOURCES: TIM4PWMResources = TIM4PWMResources {
    clocks: DRV_PWM_TIM4_CLOCK_BINDINGS,
    resets: DRV_PWM_TIM4_RESET_BINDINGS,
    interrupt_sources: DRV_PWM_TIM4_INTERRUPT_SOURCES,
    interrupts: DRV_PWM_TIM4_INTERRUPT_ROUTES,
    dma_channels: DRV_PWM_TIM4_DMA_CHANNELS,
    dma: DRV_PWM_TIM4_DMA_ROUTES,
    pins: DRV_PWM_TIM4_PIN_ROLES,
    init_operations: DRV_PWM_TIM4_INIT_OPERATIONS,
    state_machines: DRV_PWM_TIM4_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_PWM_TIM4_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM4PWM {
    resources: TIM4PWMResources,
}

impl TIM4PWM {
    pub fn new(resources: TIM4PWMResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM4PWMResources {
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

    pub fn set_prescaler(&self, prescaler: u16) -> Result<(), metadata::Error> {
        write_u16(0x40000828u64, prescaler)?;
        Ok(())
    }

    pub fn set_auto_reload(&self, reload: u16) -> Result<(), metadata::Error> {
        write_u16(0x4000082Cu64, reload)?;
        Ok(())
    }

    pub fn set_counter(&self, counter: u16) -> Result<(), metadata::Error> {
        write_u16(0x40000824u64, counter)?;
        Ok(())
    }

    pub fn generate_update(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000814u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    /// Enable auto-reload buffering for TIM4 PWM.
    pub fn enable_auto_reload_preload(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000800u64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    pub fn configure_ch1_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000818u64, 0x0003u16, 0x0000u16)?;
        modify_u16(0x40000818u64, 0x0008u16, 0x0008u16)?;
        modify_u16(0x40000818u64, 0x0070u16, 0x0060u16)?;
        Ok(())
    }

    pub fn channel_ch1(&self) -> TIM4PWMCh1 {
        TIM4PWMCh1 {
            compare_addr: 0x40000834u64,
            auto_reload_addr: 0x4000082Cu64,
            enable_addr: 0x40000820u64,
            enable_clear_mask: 0x0001u16,
            enable_set_mask: 0x0001u16,
        }
    }

    /// Configure PB6 for the TIM4 PWM CH1 output.
    pub fn configure_ch1_pb6_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010C00u64, 0x0F000000u32, 0x0B000000u32)?;
        Ok(())
    }

    pub fn configure_ch2_as_pwm_mode_1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000818u64, 0x0300u16, 0x0000u16)?;
        modify_u16(0x40000818u64, 0x0800u16, 0x0800u16)?;
        modify_u16(0x40000818u64, 0x7000u16, 0x6000u16)?;
        Ok(())
    }

    pub fn channel_ch2(&self) -> TIM4PWMCh2 {
        TIM4PWMCh2 {
            compare_addr: 0x40000838u64,
            auto_reload_addr: 0x4000082Cu64,
            enable_addr: 0x40000820u64,
            enable_clear_mask: 0x0010u16,
            enable_set_mask: 0x0010u16,
        }
    }

    /// Configure PB7 for the TIM4 PWM CH2 output.
    pub fn configure_ch2_pb7_as_pwm_output(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40010C00u64, 0xF0000000u32, 0xB0000000u32)?;
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

#[derive(Debug, Clone, Copy)]
pub struct TIM4PWMCh1 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM4PWMCh1 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM4PWMCh1 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM4PWMCh1 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TIM4PWMCh2 {
    compare_addr: u64,
    auto_reload_addr: u64,
    enable_addr: u64,
    enable_clear_mask: u16,
    enable_set_mask: u16,
}

impl TIM4PWMCh2 {
    pub fn enable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, self.enable_set_mask)?;
        Ok(())
    }

    pub fn disable_output(&self) -> Result<(), metadata::Error> {
        modify_u16(self.enable_addr, self.enable_clear_mask, 0x0000u16)?;
        Ok(())
    }
}

impl embedded_hal::pwm::ErrorType for TIM4PWMCh2 {
    type Error = metadata::Error;
}

impl embedded_hal::pwm::SetDutyCycle for TIM4PWMCh2 {
    fn max_duty_cycle(&self) -> u16 {
        let address = checked_address(self.auto_reload_addr, core::mem::align_of::<u16>())
            .expect("modeled PWM auto-reload register address must be aligned");
        let reload = unsafe { read_volatile(address as *const u16) };
        reload.saturating_add(1)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > self.max_duty_cycle() {
            return Err(metadata::Error::Unsupported("PWM duty exceeds the configured auto-reload period"));
        }
        write_u16(self.compare_addr, duty)?;
        Ok(())
    }
}
