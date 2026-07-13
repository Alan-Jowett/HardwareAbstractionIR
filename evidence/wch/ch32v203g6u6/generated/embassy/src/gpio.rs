//! Generated Embassy-style gpio module for CH32V203G6U6.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pull {
    None,
    Up,
    Down,
}

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "gpio",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: GPIOA (gpio-port) from canonical block block.gpioa -> gpio-port
pub const DRV_GPIOA_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioa", name: "GPIOA clock binding", consumer_ref: "periph.gpioa", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOA_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioa", name: "GPIOA reset binding", target_ref: "periph.gpioa", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOA_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOA_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOA_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOA_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOA_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa0", name: "GPIOA PA0", pin_ref: "pin.pa0", peripheral_ref: "periph.gpioa", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa1", name: "GPIOA PA1", pin_ref: "pin.pa1", peripheral_ref: "periph.gpioa", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa10", name: "GPIOA PA10", pin_ref: "pin.pa10", peripheral_ref: "periph.gpioa", signal: "GPIO10", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa11", name: "GPIOA PA11", pin_ref: "pin.pa11", peripheral_ref: "periph.gpioa", signal: "GPIO11", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa12", name: "GPIOA PA12", pin_ref: "pin.pa12", peripheral_ref: "periph.gpioa", signal: "GPIO12", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa13", name: "GPIOA PA13", pin_ref: "pin.pa13", peripheral_ref: "periph.gpioa", signal: "GPIO13", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa14", name: "GPIOA PA14", pin_ref: "pin.pa14", peripheral_ref: "periph.gpioa", signal: "GPIO14", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa15", name: "GPIOA PA15", pin_ref: "pin.pa15", peripheral_ref: "periph.gpioa", signal: "GPIO15", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_8_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa2", name: "GPIOA PA2", pin_ref: "pin.pa2", peripheral_ref: "periph.gpioa", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_9_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa3", name: "GPIOA PA3", pin_ref: "pin.pa3", peripheral_ref: "periph.gpioa", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_10_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa4", name: "GPIOA PA4", pin_ref: "pin.pa4", peripheral_ref: "periph.gpioa", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_11_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa5", name: "GPIOA PA5", pin_ref: "pin.pa5", peripheral_ref: "periph.gpioa", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_12_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa6", name: "GPIOA PA6", pin_ref: "pin.pa6", peripheral_ref: "periph.gpioa", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_13_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa7", name: "GPIOA PA7", pin_ref: "pin.pa7", peripheral_ref: "periph.gpioa", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLE_14_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa9", name: "GPIOA PA9", pin_ref: "pin.pa9", peripheral_ref: "periph.gpioa", signal: "GPIO9", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOA_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOA_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOA_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio10", signal: "GPIO10", routes: DRV_GPIOA_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio11", signal: "GPIO11", routes: DRV_GPIOA_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio12", signal: "GPIO12", routes: DRV_GPIOA_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio13", signal: "GPIO13", routes: DRV_GPIOA_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio14", signal: "GPIO14", routes: DRV_GPIOA_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio15", signal: "GPIO15", routes: DRV_GPIOA_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOA_PIN_ROLE_8_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOA_PIN_ROLE_9_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOA_PIN_ROLE_10_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOA_PIN_ROLE_11_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOA_PIN_ROLE_12_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOA_PIN_ROLE_13_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio9", signal: "GPIO9", routes: DRV_GPIOA_PIN_ROLE_14_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOA_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOA_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOA_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GPIOAResources {
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

pub const DRV_GPIOA_RESOURCES: GPIOAResources = GPIOAResources {
    clocks: DRV_GPIOA_CLOCK_BINDINGS,
    resets: DRV_GPIOA_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOA_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOA_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOA_DMA_CHANNELS,
    dma: DRV_GPIOA_DMA_ROUTES,
    pins: DRV_GPIOA_PIN_ROLES,
    init_operations: DRV_GPIOA_INIT_OPERATIONS,
    state_machines: DRV_GPIOA_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIOA_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GPIOA {
    resources: GPIOAResources,
}

impl GPIOA {
    pub fn new(resources: GPIOAResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GPIOAResources {
        self.resources
    }
    /// Enable the GPIOA clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the GPIOA clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOA.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for GPIOA.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PA0 pin on GPIOA.
    pub fn pa0(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PA0",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0x0000000Fu32,
            cfg_input_float_mask: 0x00000004u32,
            cfg_input_pull_mask: 0x00000008u32,
            cfg_output_mask: 0x00000003u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000001u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000001u32,
        }
    }

    /// Access the PA1 pin on GPIOA.
    pub fn pa1(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PA1",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0x000000F0u32,
            cfg_input_float_mask: 0x00000040u32,
            cfg_input_pull_mask: 0x00000080u32,
            cfg_output_mask: 0x00000030u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000002u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000002u32,
        }
    }

    /// Access the PA10 pin on GPIOA.
    pub fn pa10(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PA10",
            cfg_addr: 0x40010804u64,
            cfg_clear_mask: 0x00000F00u32,
            cfg_input_float_mask: 0x00000400u32,
            cfg_input_pull_mask: 0x00000800u32,
            cfg_output_mask: 0x00000300u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000400u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000400u32,
        }
    }

    /// Access the PA11 pin on GPIOA.
    pub fn pa11(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PA11",
            cfg_addr: 0x40010804u64,
            cfg_clear_mask: 0x0000F000u32,
            cfg_input_float_mask: 0x00004000u32,
            cfg_input_pull_mask: 0x00008000u32,
            cfg_output_mask: 0x00003000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000800u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000800u32,
        }
    }

    /// Access the PA12 pin on GPIOA.
    pub fn pa12(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PA12",
            cfg_addr: 0x40010804u64,
            cfg_clear_mask: 0x000F0000u32,
            cfg_input_float_mask: 0x00040000u32,
            cfg_input_pull_mask: 0x00080000u32,
            cfg_output_mask: 0x00030000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00001000u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00001000u32,
        }
    }

    /// Access the PA13 pin on GPIOA.
    pub fn pa13(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PA13",
            cfg_addr: 0x40010804u64,
            cfg_clear_mask: 0x00F00000u32,
            cfg_input_float_mask: 0x00400000u32,
            cfg_input_pull_mask: 0x00800000u32,
            cfg_output_mask: 0x00300000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00002000u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00002000u32,
        }
    }

    /// Access the PA14 pin on GPIOA.
    pub fn pa14(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PA14",
            cfg_addr: 0x40010804u64,
            cfg_clear_mask: 0x0F000000u32,
            cfg_input_float_mask: 0x04000000u32,
            cfg_input_pull_mask: 0x08000000u32,
            cfg_output_mask: 0x03000000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00004000u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00004000u32,
        }
    }

    /// Access the PA15 pin on GPIOA.
    pub fn pa15(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PA15",
            cfg_addr: 0x40010804u64,
            cfg_clear_mask: 0xF0000000u32,
            cfg_input_float_mask: 0x40000000u32,
            cfg_input_pull_mask: 0x80000000u32,
            cfg_output_mask: 0x30000000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00008000u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00008000u32,
        }
    }

    /// Access the PA2 pin on GPIOA.
    pub fn pa2(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[8],
            pin_name: "PA2",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0x00000F00u32,
            cfg_input_float_mask: 0x00000400u32,
            cfg_input_pull_mask: 0x00000800u32,
            cfg_output_mask: 0x00000300u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000004u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000004u32,
        }
    }

    /// Access the PA3 pin on GPIOA.
    pub fn pa3(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[9],
            pin_name: "PA3",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0x0000F000u32,
            cfg_input_float_mask: 0x00004000u32,
            cfg_input_pull_mask: 0x00008000u32,
            cfg_output_mask: 0x00003000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000008u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000008u32,
        }
    }

    /// Access the PA4 pin on GPIOA.
    pub fn pa4(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[10],
            pin_name: "PA4",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0x000F0000u32,
            cfg_input_float_mask: 0x00040000u32,
            cfg_input_pull_mask: 0x00080000u32,
            cfg_output_mask: 0x00030000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000010u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000010u32,
        }
    }

    /// Access the PA5 pin on GPIOA.
    pub fn pa5(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[11],
            pin_name: "PA5",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0x00F00000u32,
            cfg_input_float_mask: 0x00400000u32,
            cfg_input_pull_mask: 0x00800000u32,
            cfg_output_mask: 0x00300000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000020u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000020u32,
        }
    }

    /// Access the PA6 pin on GPIOA.
    pub fn pa6(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[12],
            pin_name: "PA6",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0x0F000000u32,
            cfg_input_float_mask: 0x04000000u32,
            cfg_input_pull_mask: 0x08000000u32,
            cfg_output_mask: 0x03000000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000040u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000040u32,
        }
    }

    /// Access the PA7 pin on GPIOA.
    pub fn pa7(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[13],
            pin_name: "PA7",
            cfg_addr: 0x40010800u64,
            cfg_clear_mask: 0xF0000000u32,
            cfg_input_float_mask: 0x40000000u32,
            cfg_input_pull_mask: 0x80000000u32,
            cfg_output_mask: 0x30000000u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000080u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000080u32,
        }
    }

    /// Access the PA9 pin on GPIOA.
    pub fn pa9(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[14],
            pin_name: "PA9",
            cfg_addr: 0x40010804u64,
            cfg_clear_mask: 0x000000F0u32,
            cfg_input_float_mask: 0x00000040u32,
            cfg_input_pull_mask: 0x00000080u32,
            cfg_output_mask: 0x00000030u32,
            idr_addr: 0x40010808u64,
            idr_mask: 0x00000200u32,
            odr_addr: 0x4001080Cu64,
            odr_mask: 0x00000200u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOAFlex {
    resources: GPIOAResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    cfg_addr: u64,
    cfg_clear_mask: u32,
    cfg_input_float_mask: u32,
    cfg_input_pull_mask: u32,
    cfg_output_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GPIOAInput {
    pin: GPIOAFlex,
}

#[derive(Debug, Clone)]
pub struct GPIOAOutput {
    pin: GPIOAFlex,
}

impl GPIOAFlex {
    pub fn resources(&self) -> GPIOAResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GPIOAInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GPIOAInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GPIOAOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GPIOAOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_float_mask)?;
            }
            Pull::Up => {
                modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;
            }
            Pull::Down => {
                modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.idr_addr)? & self.idr_mask) != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.odr_addr)? & self.odr_mask) != 0)
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_set_high()?)
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_set_high()? { Level::High } else { Level::Low })
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GPIOAInput {
    pub fn into_flex(self) -> GPIOAFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.pin.role()
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.pin.set_pull(pull)
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        self.pin.is_high()
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        self.pin.is_low()
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        self.pin.get_level()
    }
}

impl GPIOAOutput {
    pub fn into_flex(self) -> GPIOAFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.pin.role()
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.pin.set_pull(pull)
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        self.pin.set_high()
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        self.pin.set_low()
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        self.pin.set_level(level)
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.pin.is_set_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.pin.is_set_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.pin.get_output_level()
    }
}
// Driver instance: GPIOB (gpio-port) from canonical block block.gpiob -> gpio-port
pub const DRV_GPIOB_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiob", name: "GPIOB clock binding", consumer_ref: "periph.gpiob", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOB_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpiob", name: "GPIOB reset binding", target_ref: "periph.gpiob", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOB_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOB_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOB_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOB_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOB_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb0", name: "GPIOB PB0", pin_ref: "pin.pb0", peripheral_ref: "periph.gpiob", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOB_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb1", name: "GPIOB PB1", pin_ref: "pin.pb1", peripheral_ref: "periph.gpiob", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOB_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb3", name: "GPIOB PB3", pin_ref: "pin.pb3", peripheral_ref: "periph.gpiob", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOB_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb4", name: "GPIOB PB4", pin_ref: "pin.pb4", peripheral_ref: "periph.gpiob", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOB_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb5", name: "GPIOB PB5", pin_ref: "pin.pb5", peripheral_ref: "periph.gpiob", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOB_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb6", name: "GPIOB PB6", pin_ref: "pin.pb6", peripheral_ref: "periph.gpiob", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOB_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb7", name: "GPIOB PB7", pin_ref: "pin.pb7", peripheral_ref: "periph.gpiob", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOB_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOB_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOB_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOB_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOB_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOB_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOB_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOB_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOB_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOB_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOB_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GPIOBResources {
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

pub const DRV_GPIOB_RESOURCES: GPIOBResources = GPIOBResources {
    clocks: DRV_GPIOB_CLOCK_BINDINGS,
    resets: DRV_GPIOB_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOB_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOB_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOB_DMA_CHANNELS,
    dma: DRV_GPIOB_DMA_ROUTES,
    pins: DRV_GPIOB_PIN_ROLES,
    init_operations: DRV_GPIOB_INIT_OPERATIONS,
    state_machines: DRV_GPIOB_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIOB_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GPIOB {
    resources: GPIOBResources,
}

impl GPIOB {
    pub fn new(resources: GPIOBResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GPIOBResources {
        self.resources
    }
    /// Enable the GPIOB clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the GPIOB clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOB.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Release reset for GPIOB.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PB0 pin on GPIOB.
    pub fn pb0(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PB0",
            cfg_addr: 0x40010C00u64,
            cfg_clear_mask: 0x0000000Fu32,
            cfg_input_float_mask: 0x00000004u32,
            cfg_input_pull_mask: 0x00000008u32,
            cfg_output_mask: 0x00000003u32,
            idr_addr: 0x40010C08u64,
            idr_mask: 0x00000001u32,
            odr_addr: 0x40010C0Cu64,
            odr_mask: 0x00000001u32,
        }
    }

    /// Access the PB1 pin on GPIOB.
    pub fn pb1(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PB1",
            cfg_addr: 0x40010C00u64,
            cfg_clear_mask: 0x000000F0u32,
            cfg_input_float_mask: 0x00000040u32,
            cfg_input_pull_mask: 0x00000080u32,
            cfg_output_mask: 0x00000030u32,
            idr_addr: 0x40010C08u64,
            idr_mask: 0x00000002u32,
            odr_addr: 0x40010C0Cu64,
            odr_mask: 0x00000002u32,
        }
    }

    /// Access the PB3 pin on GPIOB.
    pub fn pb3(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PB3",
            cfg_addr: 0x40010C00u64,
            cfg_clear_mask: 0x0000F000u32,
            cfg_input_float_mask: 0x00004000u32,
            cfg_input_pull_mask: 0x00008000u32,
            cfg_output_mask: 0x00003000u32,
            idr_addr: 0x40010C08u64,
            idr_mask: 0x00000008u32,
            odr_addr: 0x40010C0Cu64,
            odr_mask: 0x00000008u32,
        }
    }

    /// Access the PB4 pin on GPIOB.
    pub fn pb4(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PB4",
            cfg_addr: 0x40010C00u64,
            cfg_clear_mask: 0x000F0000u32,
            cfg_input_float_mask: 0x00040000u32,
            cfg_input_pull_mask: 0x00080000u32,
            cfg_output_mask: 0x00030000u32,
            idr_addr: 0x40010C08u64,
            idr_mask: 0x00000010u32,
            odr_addr: 0x40010C0Cu64,
            odr_mask: 0x00000010u32,
        }
    }

    /// Access the PB5 pin on GPIOB.
    pub fn pb5(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PB5",
            cfg_addr: 0x40010C00u64,
            cfg_clear_mask: 0x00F00000u32,
            cfg_input_float_mask: 0x00400000u32,
            cfg_input_pull_mask: 0x00800000u32,
            cfg_output_mask: 0x00300000u32,
            idr_addr: 0x40010C08u64,
            idr_mask: 0x00000020u32,
            odr_addr: 0x40010C0Cu64,
            odr_mask: 0x00000020u32,
        }
    }

    /// Access the PB6 pin on GPIOB.
    pub fn pb6(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PB6",
            cfg_addr: 0x40010C00u64,
            cfg_clear_mask: 0x0F000000u32,
            cfg_input_float_mask: 0x04000000u32,
            cfg_input_pull_mask: 0x08000000u32,
            cfg_output_mask: 0x03000000u32,
            idr_addr: 0x40010C08u64,
            idr_mask: 0x00000040u32,
            odr_addr: 0x40010C0Cu64,
            odr_mask: 0x00000040u32,
        }
    }

    /// Access the PB7 pin on GPIOB.
    pub fn pb7(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PB7",
            cfg_addr: 0x40010C00u64,
            cfg_clear_mask: 0xF0000000u32,
            cfg_input_float_mask: 0x40000000u32,
            cfg_input_pull_mask: 0x80000000u32,
            cfg_output_mask: 0x30000000u32,
            idr_addr: 0x40010C08u64,
            idr_mask: 0x00000080u32,
            odr_addr: 0x40010C0Cu64,
            odr_mask: 0x00000080u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOBFlex {
    resources: GPIOBResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    cfg_addr: u64,
    cfg_clear_mask: u32,
    cfg_input_float_mask: u32,
    cfg_input_pull_mask: u32,
    cfg_output_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GPIOBInput {
    pin: GPIOBFlex,
}

#[derive(Debug, Clone)]
pub struct GPIOBOutput {
    pin: GPIOBFlex,
}

impl GPIOBFlex {
    pub fn resources(&self) -> GPIOBResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GPIOBInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GPIOBInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GPIOBOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GPIOBOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_float_mask)?;
            }
            Pull::Up => {
                modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;
            }
            Pull::Down => {
                modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.idr_addr)? & self.idr_mask) != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.odr_addr)? & self.odr_mask) != 0)
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_set_high()?)
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_set_high()? { Level::High } else { Level::Low })
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GPIOBInput {
    pub fn into_flex(self) -> GPIOBFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.pin.role()
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.pin.set_pull(pull)
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        self.pin.is_high()
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        self.pin.is_low()
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        self.pin.get_level()
    }
}

impl GPIOBOutput {
    pub fn into_flex(self) -> GPIOBFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.pin.role()
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.pin.set_pull(pull)
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        self.pin.set_high()
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        self.pin.set_low()
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        self.pin.set_level(level)
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.pin.is_set_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.pin.is_set_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.pin.get_output_level()
    }
}
// Driver instance: GPIOD (gpio-port) from canonical block block.gpiod -> gpio-port
pub const DRV_GPIOD_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiod", name: "GPIOD clock binding", consumer_ref: "periph.gpiod", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOD_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpiod", name: "GPIOD reset binding", target_ref: "periph.gpiod", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOD_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOD_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOD_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOD_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOD_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd0", name: "GPIOD PD0", pin_ref: "pin.pd0", peripheral_ref: "periph.gpiod", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOD_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd1", name: "GPIOD PD1", pin_ref: "pin.pd1", peripheral_ref: "periph.gpiod", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_GPIOD_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOD_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOD_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOD_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOD_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOD_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GPIODResources {
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

pub const DRV_GPIOD_RESOURCES: GPIODResources = GPIODResources {
    clocks: DRV_GPIOD_CLOCK_BINDINGS,
    resets: DRV_GPIOD_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOD_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOD_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOD_DMA_CHANNELS,
    dma: DRV_GPIOD_DMA_ROUTES,
    pins: DRV_GPIOD_PIN_ROLES,
    init_operations: DRV_GPIOD_INIT_OPERATIONS,
    state_machines: DRV_GPIOD_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIOD_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GPIOD {
    resources: GPIODResources,
}

impl GPIOD {
    pub fn new(resources: GPIODResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GPIODResources {
        self.resources
    }
    /// Enable the GPIOD clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the GPIOD clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOD.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Release reset for GPIOD.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PD0 pin on GPIOD.
    pub fn pd0(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PD0",
            cfg_addr: 0x40011400u64,
            cfg_clear_mask: 0x0000000Fu32,
            cfg_input_float_mask: 0x00000004u32,
            cfg_input_pull_mask: 0x00000008u32,
            cfg_output_mask: 0x00000003u32,
            idr_addr: 0x40011408u64,
            idr_mask: 0x00000001u32,
            odr_addr: 0x4001140Cu64,
            odr_mask: 0x00000001u32,
        }
    }

    /// Access the PD1 pin on GPIOD.
    pub fn pd1(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PD1",
            cfg_addr: 0x40011400u64,
            cfg_clear_mask: 0x000000F0u32,
            cfg_input_float_mask: 0x00000040u32,
            cfg_input_pull_mask: 0x00000080u32,
            cfg_output_mask: 0x00000030u32,
            idr_addr: 0x40011408u64,
            idr_mask: 0x00000002u32,
            odr_addr: 0x4001140Cu64,
            odr_mask: 0x00000002u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIODFlex {
    resources: GPIODResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    cfg_addr: u64,
    cfg_clear_mask: u32,
    cfg_input_float_mask: u32,
    cfg_input_pull_mask: u32,
    cfg_output_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GPIODInput {
    pin: GPIODFlex,
}

#[derive(Debug, Clone)]
pub struct GPIODOutput {
    pin: GPIODFlex,
}

impl GPIODFlex {
    pub fn resources(&self) -> GPIODResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GPIODInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GPIODInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GPIODOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GPIODOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_float_mask)?;
            }
            Pull::Up => {
                modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;
            }
            Pull::Down => {
                modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;
                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.idr_addr)? & self.idr_mask) != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.odr_addr)? & self.odr_mask) != 0)
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_set_high()?)
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_set_high()? { Level::High } else { Level::Low })
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GPIODInput {
    pub fn into_flex(self) -> GPIODFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.pin.role()
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.pin.set_pull(pull)
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        self.pin.is_high()
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        self.pin.is_low()
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        self.pin.get_level()
    }
}

impl GPIODOutput {
    pub fn into_flex(self) -> GPIODFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.pin.role()
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.pin.set_pull(pull)
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        self.pin.set_high()
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        self.pin.set_low()
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        self.pin.set_level(level)
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.pin.is_set_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.pin.is_set_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.pin.get_output_level()
    }
}
