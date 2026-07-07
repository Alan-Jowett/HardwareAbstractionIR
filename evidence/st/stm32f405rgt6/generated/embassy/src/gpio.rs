//! Generated Embassy-style gpio module for STM32F405RGT6.

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
    module_name: "gpio",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: Gpioa (gpio-port) from canonical block block.gpioa -> gpio-port
pub const DRV_GPIOA_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioa", name: "GPIOA clock", consumer_ref: "periph.gpioa", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOA_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_GPIOA_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOA_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOA_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOA_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOA_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio0.pa0", name: "GPIOA GPIO0 on PA0", pin_ref: "pin.pa0", peripheral_ref: "periph.gpioa", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio1.pa1", name: "GPIOA GPIO1 on PA1", pin_ref: "pin.pa1", peripheral_ref: "periph.gpioa", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio10.pa10", name: "GPIOA GPIO10 on PA10", pin_ref: "pin.pa10", peripheral_ref: "periph.gpioa", signal: "GPIO10", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio11.pa11", name: "GPIOA GPIO11 on PA11", pin_ref: "pin.pa11", peripheral_ref: "periph.gpioa", signal: "GPIO11", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio12.pa12", name: "GPIOA GPIO12 on PA12", pin_ref: "pin.pa12", peripheral_ref: "periph.gpioa", signal: "GPIO12", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio13.pa13", name: "GPIOA GPIO13 on PA13", pin_ref: "pin.pa13", peripheral_ref: "periph.gpioa", signal: "GPIO13", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio14.pa14", name: "GPIOA GPIO14 on PA14", pin_ref: "pin.pa14", peripheral_ref: "periph.gpioa", signal: "GPIO14", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio15.pa15", name: "GPIOA GPIO15 on PA15", pin_ref: "pin.pa15", peripheral_ref: "periph.gpioa", signal: "GPIO15", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_8_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio2.pa2", name: "GPIOA GPIO2 on PA2", pin_ref: "pin.pa2", peripheral_ref: "periph.gpioa", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_9_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio3.pa3", name: "GPIOA GPIO3 on PA3", pin_ref: "pin.pa3", peripheral_ref: "periph.gpioa", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_10_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio4.pa4", name: "GPIOA GPIO4 on PA4", pin_ref: "pin.pa4", peripheral_ref: "periph.gpioa", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_11_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio5.pa5", name: "GPIOA GPIO5 on PA5", pin_ref: "pin.pa5", peripheral_ref: "periph.gpioa", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_12_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio6.pa6", name: "GPIOA GPIO6 on PA6", pin_ref: "pin.pa6", peripheral_ref: "periph.gpioa", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_13_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio7.pa7", name: "GPIOA GPIO7 on PA7", pin_ref: "pin.pa7", peripheral_ref: "periph.gpioa", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_14_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio8.pa8", name: "GPIOA GPIO8 on PA8", pin_ref: "pin.pa8", peripheral_ref: "periph.gpioa", signal: "GPIO8", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_15_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.gpio9.pa9", name: "GPIOA GPIO9 on PA9", pin_ref: "pin.pa9", peripheral_ref: "periph.gpioa", signal: "GPIO9", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOA_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOA_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio10", signal: "GPIO10", routes: DRV_GPIOA_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio11", signal: "GPIO11", routes: DRV_GPIOA_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio12", signal: "GPIO12", routes: DRV_GPIOA_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio13", signal: "GPIO13", routes: DRV_GPIOA_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio14", signal: "GPIO14", routes: DRV_GPIOA_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio15", signal: "GPIO15", routes: DRV_GPIOA_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOA_PIN_ROLE_8_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOA_PIN_ROLE_9_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOA_PIN_ROLE_10_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOA_PIN_ROLE_11_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOA_PIN_ROLE_12_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOA_PIN_ROLE_13_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio8", signal: "GPIO8", routes: DRV_GPIOA_PIN_ROLE_14_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio9", signal: "GPIO9", routes: DRV_GPIOA_PIN_ROLE_15_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOA_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOA_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOA_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GpioaResources {
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

pub const DRV_GPIOA_RESOURCES: GpioaResources = GpioaResources {
    clocks: DRV_GPIOA_CLOCK_BINDINGS,
    resets: DRV_GPIOA_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOA_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOA_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOA_DMA_CHANNELS,
    dma: DRV_GPIOA_DMA_ROUTES,
    pins: DRV_GPIOA_PIN_ROLES,
    init_operations: DRV_GPIOA_INIT_OPERATIONS,
    state_machines: DRV_GPIOA_STATE_MACHINES,
    capability_tags: DRV_GPIOA_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Gpioa {
    resources: GpioaResources,
}

impl Gpioa {
    pub fn new(resources: GpioaResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpioaResources {
        self.resources
    }
    /// Enable the GPIOA clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the GPIOA clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: Gpiob (gpio-port) from canonical block block.gpiob -> gpio-port
pub const DRV_GPIOB_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiob", name: "GPIOB clock", consumer_ref: "periph.gpiob", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOB_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_GPIOB_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOB_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOB_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOB_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOB_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio0.pb0", name: "GPIOB GPIO0 on PB0", pin_ref: "pin.pb0", peripheral_ref: "periph.gpiob", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio1.pb1", name: "GPIOB GPIO1 on PB1", pin_ref: "pin.pb1", peripheral_ref: "periph.gpiob", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio10.pb10", name: "GPIOB GPIO10 on PB10", pin_ref: "pin.pb10", peripheral_ref: "periph.gpiob", signal: "GPIO10", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio11.pb11", name: "GPIOB GPIO11 on PB11", pin_ref: "pin.pb11", peripheral_ref: "periph.gpiob", signal: "GPIO11", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio12.pb12", name: "GPIOB GPIO12 on PB12", pin_ref: "pin.pb12", peripheral_ref: "periph.gpiob", signal: "GPIO12", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio13.pb13", name: "GPIOB GPIO13 on PB13", pin_ref: "pin.pb13", peripheral_ref: "periph.gpiob", signal: "GPIO13", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio14.pb14", name: "GPIOB GPIO14 on PB14", pin_ref: "pin.pb14", peripheral_ref: "periph.gpiob", signal: "GPIO14", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio15.pb15", name: "GPIOB GPIO15 on PB15", pin_ref: "pin.pb15", peripheral_ref: "periph.gpiob", signal: "GPIO15", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_8_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio2.pb2", name: "GPIOB GPIO2 on PB2", pin_ref: "pin.pb2", peripheral_ref: "periph.gpiob", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_9_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio3.pb3", name: "GPIOB GPIO3 on PB3", pin_ref: "pin.pb3", peripheral_ref: "periph.gpiob", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_10_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio4.pb4", name: "GPIOB GPIO4 on PB4", pin_ref: "pin.pb4", peripheral_ref: "periph.gpiob", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_11_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio5.pb5", name: "GPIOB GPIO5 on PB5", pin_ref: "pin.pb5", peripheral_ref: "periph.gpiob", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_12_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio6.pb6", name: "GPIOB GPIO6 on PB6", pin_ref: "pin.pb6", peripheral_ref: "periph.gpiob", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_13_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio7.pb7", name: "GPIOB GPIO7 on PB7", pin_ref: "pin.pb7", peripheral_ref: "periph.gpiob", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_14_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio8.pb8", name: "GPIOB GPIO8 on PB8", pin_ref: "pin.pb8", peripheral_ref: "periph.gpiob", signal: "GPIO8", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_15_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.gpio9.pb9", name: "GPIOB GPIO9 on PB9", pin_ref: "pin.pb9", peripheral_ref: "periph.gpiob", signal: "GPIO9", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOB_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOB_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio10", signal: "GPIO10", routes: DRV_GPIOB_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio11", signal: "GPIO11", routes: DRV_GPIOB_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio12", signal: "GPIO12", routes: DRV_GPIOB_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio13", signal: "GPIO13", routes: DRV_GPIOB_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio14", signal: "GPIO14", routes: DRV_GPIOB_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio15", signal: "GPIO15", routes: DRV_GPIOB_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOB_PIN_ROLE_8_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOB_PIN_ROLE_9_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOB_PIN_ROLE_10_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOB_PIN_ROLE_11_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOB_PIN_ROLE_12_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOB_PIN_ROLE_13_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio8", signal: "GPIO8", routes: DRV_GPIOB_PIN_ROLE_14_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio9", signal: "GPIO9", routes: DRV_GPIOB_PIN_ROLE_15_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOB_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOB_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOB_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GpiobResources {
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

pub const DRV_GPIOB_RESOURCES: GpiobResources = GpiobResources {
    clocks: DRV_GPIOB_CLOCK_BINDINGS,
    resets: DRV_GPIOB_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOB_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOB_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOB_DMA_CHANNELS,
    dma: DRV_GPIOB_DMA_ROUTES,
    pins: DRV_GPIOB_PIN_ROLES,
    init_operations: DRV_GPIOB_INIT_OPERATIONS,
    state_machines: DRV_GPIOB_STATE_MACHINES,
    capability_tags: DRV_GPIOB_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Gpiob {
    resources: GpiobResources,
}

impl Gpiob {
    pub fn new(resources: GpiobResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpiobResources {
        self.resources
    }
    /// Enable the GPIOB clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the GPIOB clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: Gpioc (gpio-port) from canonical block block.gpioc -> gpio-port
pub const DRV_GPIOC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioc", name: "GPIOC clock", consumer_ref: "periph.gpioc", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOC_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_GPIOC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOC_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio0.pc0", name: "GPIOC GPIO0 on PC0", pin_ref: "pin.pc0", peripheral_ref: "periph.gpioc", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio1.pc1", name: "GPIOC GPIO1 on PC1", pin_ref: "pin.pc1", peripheral_ref: "periph.gpioc", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio10.pc10", name: "GPIOC GPIO10 on PC10", pin_ref: "pin.pc10", peripheral_ref: "periph.gpioc", signal: "GPIO10", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio11.pc11", name: "GPIOC GPIO11 on PC11", pin_ref: "pin.pc11", peripheral_ref: "periph.gpioc", signal: "GPIO11", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio12.pc12", name: "GPIOC GPIO12 on PC12", pin_ref: "pin.pc12", peripheral_ref: "periph.gpioc", signal: "GPIO12", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio13.pc13", name: "GPIOC GPIO13 on PC13", pin_ref: "pin.pc13", peripheral_ref: "periph.gpioc", signal: "GPIO13", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio14.pc14", name: "GPIOC GPIO14 on PC14", pin_ref: "pin.pc14", peripheral_ref: "periph.gpioc", signal: "GPIO14", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio15.pc15", name: "GPIOC GPIO15 on PC15", pin_ref: "pin.pc15", peripheral_ref: "periph.gpioc", signal: "GPIO15", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_8_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio2.pc2", name: "GPIOC GPIO2 on PC2", pin_ref: "pin.pc2", peripheral_ref: "periph.gpioc", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_9_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio3.pc3", name: "GPIOC GPIO3 on PC3", pin_ref: "pin.pc3", peripheral_ref: "periph.gpioc", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_10_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio4.pc4", name: "GPIOC GPIO4 on PC4", pin_ref: "pin.pc4", peripheral_ref: "periph.gpioc", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_11_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio5.pc5", name: "GPIOC GPIO5 on PC5", pin_ref: "pin.pc5", peripheral_ref: "periph.gpioc", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_12_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio6.pc6", name: "GPIOC GPIO6 on PC6", pin_ref: "pin.pc6", peripheral_ref: "periph.gpioc", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_13_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio7.pc7", name: "GPIOC GPIO7 on PC7", pin_ref: "pin.pc7", peripheral_ref: "periph.gpioc", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_14_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio8.pc8", name: "GPIOC GPIO8 on PC8", pin_ref: "pin.pc8", peripheral_ref: "periph.gpioc", signal: "GPIO8", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_15_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.gpio9.pc9", name: "GPIOC GPIO9 on PC9", pin_ref: "pin.pc9", peripheral_ref: "periph.gpioc", signal: "GPIO9", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOC_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOC_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio10", signal: "GPIO10", routes: DRV_GPIOC_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio11", signal: "GPIO11", routes: DRV_GPIOC_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio12", signal: "GPIO12", routes: DRV_GPIOC_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio13", signal: "GPIO13", routes: DRV_GPIOC_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio14", signal: "GPIO14", routes: DRV_GPIOC_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio15", signal: "GPIO15", routes: DRV_GPIOC_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOC_PIN_ROLE_8_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOC_PIN_ROLE_9_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOC_PIN_ROLE_10_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOC_PIN_ROLE_11_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOC_PIN_ROLE_12_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOC_PIN_ROLE_13_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio8", signal: "GPIO8", routes: DRV_GPIOC_PIN_ROLE_14_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio9", signal: "GPIO9", routes: DRV_GPIOC_PIN_ROLE_15_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GpiocResources {
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

pub const DRV_GPIOC_RESOURCES: GpiocResources = GpiocResources {
    clocks: DRV_GPIOC_CLOCK_BINDINGS,
    resets: DRV_GPIOC_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOC_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOC_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOC_DMA_CHANNELS,
    dma: DRV_GPIOC_DMA_ROUTES,
    pins: DRV_GPIOC_PIN_ROLES,
    init_operations: DRV_GPIOC_INIT_OPERATIONS,
    state_machines: DRV_GPIOC_STATE_MACHINES,
    capability_tags: DRV_GPIOC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Gpioc {
    resources: GpiocResources,
}

impl Gpioc {
    pub fn new(resources: GpiocResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpiocResources {
        self.resources
    }
    /// Enable the GPIOC clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the GPIOC clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: Gpiod (gpio-port) from canonical block block.gpiod -> gpio-port
pub const DRV_GPIOD_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiod", name: "GPIOD clock", consumer_ref: "periph.gpiod", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOD_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_GPIOD_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOD_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOD_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOD_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOD_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.gpio2.pd2", name: "GPIOD GPIO2 on PD2", pin_ref: "pin.pd2", peripheral_ref: "periph.gpiod", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOD_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOD_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOD_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOD_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GpiodResources {
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

pub const DRV_GPIOD_RESOURCES: GpiodResources = GpiodResources {
    clocks: DRV_GPIOD_CLOCK_BINDINGS,
    resets: DRV_GPIOD_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOD_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOD_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOD_DMA_CHANNELS,
    dma: DRV_GPIOD_DMA_ROUTES,
    pins: DRV_GPIOD_PIN_ROLES,
    init_operations: DRV_GPIOD_INIT_OPERATIONS,
    state_machines: DRV_GPIOD_STATE_MACHINES,
    capability_tags: DRV_GPIOD_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Gpiod {
    resources: GpiodResources,
}

impl Gpiod {
    pub fn new(resources: GpiodResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpiodResources {
        self.resources
    }
    /// Enable the GPIOD clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the GPIOD clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: Gpioh (gpio-port) from canonical block block.gpioh -> gpio-port
pub const DRV_GPIOH_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioh", name: "GPIOH clock", consumer_ref: "periph.gpioh", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOH_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_GPIOH_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOH_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOH_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOH_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOH_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioh.gpio0.ph0", name: "GPIOH GPIO0 on PH0", pin_ref: "pin.ph0", peripheral_ref: "periph.gpioh", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOH_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioh.gpio1.ph1", name: "GPIOH GPIO1 on PH1", pin_ref: "pin.ph1", peripheral_ref: "periph.gpioh", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOH_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOH_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOH_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOH_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOH_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOH_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GpiohResources {
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

pub const DRV_GPIOH_RESOURCES: GpiohResources = GpiohResources {
    clocks: DRV_GPIOH_CLOCK_BINDINGS,
    resets: DRV_GPIOH_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOH_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOH_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOH_DMA_CHANNELS,
    dma: DRV_GPIOH_DMA_ROUTES,
    pins: DRV_GPIOH_PIN_ROLES,
    init_operations: DRV_GPIOH_INIT_OPERATIONS,
    state_machines: DRV_GPIOH_STATE_MACHINES,
    capability_tags: DRV_GPIOH_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Gpioh {
    resources: GpiohResources,
}

impl Gpioh {
    pub fn new(resources: GpiohResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpiohResources {
        self.resources
    }
    /// Enable the GPIOH clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the GPIOH clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }


}

