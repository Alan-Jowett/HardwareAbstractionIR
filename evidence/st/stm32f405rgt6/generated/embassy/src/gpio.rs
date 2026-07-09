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

// Driver instance: GpioA (gpio-port) from canonical block block.gpioa -> gpio-port
pub const DRV_GPIOA_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioa", name: "GPIOA clock", consumer_ref: "periph.gpioa", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOA_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioa", name: "GPIOA reset", target_ref: "periph.gpioa", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.ahb1"), binding_kind: "software", control_refs: &["reg.rcc.ahb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
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
pub struct GpioAResources {
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

pub const DRV_GPIOA_RESOURCES: GpioAResources = GpioAResources {
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
pub struct GpioA {
    resources: GpioAResources,
}

impl GpioA {
    pub fn new(resources: GpioAResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpioAResources {
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

    /// Assert reset for GPIOA.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Release reset for GPIOA.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PA0 pin on GpioA.
    pub fn pa0(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PA0",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00000003u32,
            moder_output_mask: 0x00000001u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00000003u32,
            pupdr_up_mask: 0x00000001u32,
            pupdr_down_mask: 0x00000002u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000001u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000001u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000001u32,
            bsrr_reset_mask: 0x00010000u32,
        }
    }

    /// Access the PA1 pin on GpioA.
    pub fn pa1(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PA1",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x0000000Cu32,
            moder_output_mask: 0x00000004u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x0000000Cu32,
            pupdr_up_mask: 0x00000004u32,
            pupdr_down_mask: 0x00000008u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000002u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000002u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000002u32,
            bsrr_reset_mask: 0x00020000u32,
        }
    }

    /// Access the PA10 pin on GpioA.
    pub fn pa10(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PA10",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00300000u32,
            moder_output_mask: 0x00100000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00300000u32,
            pupdr_up_mask: 0x00100000u32,
            pupdr_down_mask: 0x00200000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000400u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000400u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000400u32,
            bsrr_reset_mask: 0x04000000u32,
        }
    }

    /// Access the PA11 pin on GpioA.
    pub fn pa11(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PA11",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00C00000u32,
            moder_output_mask: 0x00400000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00C00000u32,
            pupdr_up_mask: 0x00400000u32,
            pupdr_down_mask: 0x00800000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000800u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000800u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000800u32,
            bsrr_reset_mask: 0x08000000u32,
        }
    }

    /// Access the PA12 pin on GpioA.
    pub fn pa12(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PA12",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x03000000u32,
            moder_output_mask: 0x01000000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x03000000u32,
            pupdr_up_mask: 0x01000000u32,
            pupdr_down_mask: 0x02000000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00001000u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00001000u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00001000u32,
            bsrr_reset_mask: 0x10000000u32,
        }
    }

    /// Access the PA13 pin on GpioA.
    pub fn pa13(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PA13",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x0C000000u32,
            moder_output_mask: 0x04000000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x0C000000u32,
            pupdr_up_mask: 0x04000000u32,
            pupdr_down_mask: 0x08000000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00002000u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00002000u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00002000u32,
            bsrr_reset_mask: 0x20000000u32,
        }
    }

    /// Access the PA14 pin on GpioA.
    pub fn pa14(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PA14",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x30000000u32,
            moder_output_mask: 0x10000000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x30000000u32,
            pupdr_up_mask: 0x10000000u32,
            pupdr_down_mask: 0x20000000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00004000u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00004000u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00004000u32,
            bsrr_reset_mask: 0x40000000u32,
        }
    }

    /// Access the PA15 pin on GpioA.
    pub fn pa15(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PA15",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0xC0000000u32,
            moder_output_mask: 0x40000000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0xC0000000u32,
            pupdr_up_mask: 0x40000000u32,
            pupdr_down_mask: 0x80000000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00008000u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00008000u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00008000u32,
            bsrr_reset_mask: 0x80000000u32,
        }
    }

    /// Access the PA2 pin on GpioA.
    pub fn pa2(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[8],
            pin_name: "PA2",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00000030u32,
            moder_output_mask: 0x00000010u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00000030u32,
            pupdr_up_mask: 0x00000010u32,
            pupdr_down_mask: 0x00000020u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000004u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000004u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000004u32,
            bsrr_reset_mask: 0x00040000u32,
        }
    }

    /// Access the PA3 pin on GpioA.
    pub fn pa3(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[9],
            pin_name: "PA3",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x000000C0u32,
            moder_output_mask: 0x00000040u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x000000C0u32,
            pupdr_up_mask: 0x00000040u32,
            pupdr_down_mask: 0x00000080u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000008u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000008u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000008u32,
            bsrr_reset_mask: 0x00080000u32,
        }
    }

    /// Access the PA4 pin on GpioA.
    pub fn pa4(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[10],
            pin_name: "PA4",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00000300u32,
            moder_output_mask: 0x00000100u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00000300u32,
            pupdr_up_mask: 0x00000100u32,
            pupdr_down_mask: 0x00000200u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000010u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000010u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000010u32,
            bsrr_reset_mask: 0x00100000u32,
        }
    }

    /// Access the PA5 pin on GpioA.
    pub fn pa5(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[11],
            pin_name: "PA5",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00000C00u32,
            moder_output_mask: 0x00000400u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00000C00u32,
            pupdr_up_mask: 0x00000400u32,
            pupdr_down_mask: 0x00000800u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000020u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000020u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000020u32,
            bsrr_reset_mask: 0x00200000u32,
        }
    }

    /// Access the PA6 pin on GpioA.
    pub fn pa6(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[12],
            pin_name: "PA6",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00003000u32,
            moder_output_mask: 0x00001000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00003000u32,
            pupdr_up_mask: 0x00001000u32,
            pupdr_down_mask: 0x00002000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000040u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000040u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000040u32,
            bsrr_reset_mask: 0x00400000u32,
        }
    }

    /// Access the PA7 pin on GpioA.
    pub fn pa7(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[13],
            pin_name: "PA7",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x0000C000u32,
            moder_output_mask: 0x00004000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x0000C000u32,
            pupdr_up_mask: 0x00004000u32,
            pupdr_down_mask: 0x00008000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000080u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000080u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000080u32,
            bsrr_reset_mask: 0x00800000u32,
        }
    }

    /// Access the PA8 pin on GpioA.
    pub fn pa8(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[14],
            pin_name: "PA8",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x00030000u32,
            moder_output_mask: 0x00010000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x00030000u32,
            pupdr_up_mask: 0x00010000u32,
            pupdr_down_mask: 0x00020000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000100u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000100u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000100u32,
            bsrr_reset_mask: 0x01000000u32,
        }
    }

    /// Access the PA9 pin on GpioA.
    pub fn pa9(&self) -> GpioAFlex {
        GpioAFlex {
            resources: self.resources,
            role: &self.resources.pins[15],
            pin_name: "PA9",
            moder_addr: 0x40020000u64,
            moder_clear_mask: 0x000C0000u32,
            moder_output_mask: 0x00040000u32,
            pupdr_addr: 0x4002000Cu64,
            pupdr_clear_mask: 0x000C0000u32,
            pupdr_up_mask: 0x00040000u32,
            pupdr_down_mask: 0x00080000u32,
            idr_addr: 0x40020010u64,
            idr_mask: 0x00000200u32,
            odr_addr: 0x40020014u64,
            odr_mask: 0x00000200u32,
            bsrr_addr: 0x40020018u64,
            bsrr_set_mask: 0x00000200u32,
            bsrr_reset_mask: 0x02000000u32,
        }
    }


}

#[derive(Debug, Clone)]
pub struct GpioAFlex {
    resources: GpioAResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    moder_addr: u64,
    moder_clear_mask: u32,
    moder_output_mask: u32,
    pupdr_addr: u64,
    pupdr_clear_mask: u32,
    pupdr_up_mask: u32,
    pupdr_down_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
    bsrr_addr: u64,
    bsrr_set_mask: u32,
    bsrr_reset_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GpioAInput {
    pin: GpioAFlex,
}

#[derive(Debug, Clone)]
pub struct GpioAOutput {
    pin: GpioAFlex,
}

impl GpioAFlex {
    pub fn resources(&self) -> GpioAResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GpioAInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GpioAInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GpioAOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GpioAOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, self.moder_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        let set_mask = match pull {
            Pull::None => 0x00000000u32,
            Pull::Up => self.pupdr_up_mask,
            Pull::Down => self.pupdr_down_mask,
        };
        modify_u32(self.pupdr_addr, self.pupdr_clear_mask, set_mask)?;
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
        write_u32(self.bsrr_addr, self.bsrr_set_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.bsrr_addr, self.bsrr_reset_mask)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GpioAInput {
    pub fn into_flex(self) -> GpioAFlex {
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

impl GpioAOutput {
    pub fn into_flex(self) -> GpioAFlex {
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

// Driver instance: GpioB (gpio-port) from canonical block block.gpiob -> gpio-port
pub const DRV_GPIOB_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiob", name: "GPIOB clock", consumer_ref: "periph.gpiob", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOB_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpiob", name: "GPIOB reset", target_ref: "periph.gpiob", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.ahb1"), binding_kind: "software", control_refs: &["reg.rcc.ahb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
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
pub struct GpioBResources {
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

pub const DRV_GPIOB_RESOURCES: GpioBResources = GpioBResources {
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
pub struct GpioB {
    resources: GpioBResources,
}

impl GpioB {
    pub fn new(resources: GpioBResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpioBResources {
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

    /// Assert reset for GPIOB.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Release reset for GPIOB.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PB0 pin on GpioB.
    pub fn pb0(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PB0",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00000003u32,
            moder_output_mask: 0x00000001u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00000003u32,
            pupdr_up_mask: 0x00000001u32,
            pupdr_down_mask: 0x00000002u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000001u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000001u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000001u32,
            bsrr_reset_mask: 0x00010000u32,
        }
    }

    /// Access the PB1 pin on GpioB.
    pub fn pb1(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PB1",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x0000000Cu32,
            moder_output_mask: 0x00000004u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x0000000Cu32,
            pupdr_up_mask: 0x00000004u32,
            pupdr_down_mask: 0x00000008u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000002u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000002u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000002u32,
            bsrr_reset_mask: 0x00020000u32,
        }
    }

    /// Access the PB10 pin on GpioB.
    pub fn pb10(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PB10",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00300000u32,
            moder_output_mask: 0x00100000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00300000u32,
            pupdr_up_mask: 0x00100000u32,
            pupdr_down_mask: 0x00200000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000400u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000400u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000400u32,
            bsrr_reset_mask: 0x04000000u32,
        }
    }

    /// Access the PB11 pin on GpioB.
    pub fn pb11(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PB11",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00C00000u32,
            moder_output_mask: 0x00400000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00C00000u32,
            pupdr_up_mask: 0x00400000u32,
            pupdr_down_mask: 0x00800000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000800u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000800u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000800u32,
            bsrr_reset_mask: 0x08000000u32,
        }
    }

    /// Access the PB12 pin on GpioB.
    pub fn pb12(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PB12",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x03000000u32,
            moder_output_mask: 0x01000000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x03000000u32,
            pupdr_up_mask: 0x01000000u32,
            pupdr_down_mask: 0x02000000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00001000u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00001000u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00001000u32,
            bsrr_reset_mask: 0x10000000u32,
        }
    }

    /// Access the PB13 pin on GpioB.
    pub fn pb13(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PB13",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x0C000000u32,
            moder_output_mask: 0x04000000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x0C000000u32,
            pupdr_up_mask: 0x04000000u32,
            pupdr_down_mask: 0x08000000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00002000u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00002000u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00002000u32,
            bsrr_reset_mask: 0x20000000u32,
        }
    }

    /// Access the PB14 pin on GpioB.
    pub fn pb14(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PB14",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x30000000u32,
            moder_output_mask: 0x10000000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x30000000u32,
            pupdr_up_mask: 0x10000000u32,
            pupdr_down_mask: 0x20000000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00004000u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00004000u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00004000u32,
            bsrr_reset_mask: 0x40000000u32,
        }
    }

    /// Access the PB15 pin on GpioB.
    pub fn pb15(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PB15",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0xC0000000u32,
            moder_output_mask: 0x40000000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0xC0000000u32,
            pupdr_up_mask: 0x40000000u32,
            pupdr_down_mask: 0x80000000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00008000u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00008000u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00008000u32,
            bsrr_reset_mask: 0x80000000u32,
        }
    }

    /// Access the PB2 pin on GpioB.
    pub fn pb2(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[8],
            pin_name: "PB2",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00000030u32,
            moder_output_mask: 0x00000010u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00000030u32,
            pupdr_up_mask: 0x00000010u32,
            pupdr_down_mask: 0x00000020u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000004u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000004u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000004u32,
            bsrr_reset_mask: 0x00040000u32,
        }
    }

    /// Access the PB3 pin on GpioB.
    pub fn pb3(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[9],
            pin_name: "PB3",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x000000C0u32,
            moder_output_mask: 0x00000040u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x000000C0u32,
            pupdr_up_mask: 0x00000040u32,
            pupdr_down_mask: 0x00000080u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000008u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000008u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000008u32,
            bsrr_reset_mask: 0x00080000u32,
        }
    }

    /// Access the PB4 pin on GpioB.
    pub fn pb4(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[10],
            pin_name: "PB4",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00000300u32,
            moder_output_mask: 0x00000100u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00000300u32,
            pupdr_up_mask: 0x00000100u32,
            pupdr_down_mask: 0x00000200u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000010u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000010u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000010u32,
            bsrr_reset_mask: 0x00100000u32,
        }
    }

    /// Access the PB5 pin on GpioB.
    pub fn pb5(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[11],
            pin_name: "PB5",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00000C00u32,
            moder_output_mask: 0x00000400u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00000C00u32,
            pupdr_up_mask: 0x00000400u32,
            pupdr_down_mask: 0x00000800u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000020u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000020u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000020u32,
            bsrr_reset_mask: 0x00200000u32,
        }
    }

    /// Access the PB6 pin on GpioB.
    pub fn pb6(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[12],
            pin_name: "PB6",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00003000u32,
            moder_output_mask: 0x00001000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00003000u32,
            pupdr_up_mask: 0x00001000u32,
            pupdr_down_mask: 0x00002000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000040u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000040u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000040u32,
            bsrr_reset_mask: 0x00400000u32,
        }
    }

    /// Access the PB7 pin on GpioB.
    pub fn pb7(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[13],
            pin_name: "PB7",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x0000C000u32,
            moder_output_mask: 0x00004000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x0000C000u32,
            pupdr_up_mask: 0x00004000u32,
            pupdr_down_mask: 0x00008000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000080u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000080u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000080u32,
            bsrr_reset_mask: 0x00800000u32,
        }
    }

    /// Access the PB8 pin on GpioB.
    pub fn pb8(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[14],
            pin_name: "PB8",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x00030000u32,
            moder_output_mask: 0x00010000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x00030000u32,
            pupdr_up_mask: 0x00010000u32,
            pupdr_down_mask: 0x00020000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000100u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000100u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000100u32,
            bsrr_reset_mask: 0x01000000u32,
        }
    }

    /// Access the PB9 pin on GpioB.
    pub fn pb9(&self) -> GpioBFlex {
        GpioBFlex {
            resources: self.resources,
            role: &self.resources.pins[15],
            pin_name: "PB9",
            moder_addr: 0x40020400u64,
            moder_clear_mask: 0x000C0000u32,
            moder_output_mask: 0x00040000u32,
            pupdr_addr: 0x4002040Cu64,
            pupdr_clear_mask: 0x000C0000u32,
            pupdr_up_mask: 0x00040000u32,
            pupdr_down_mask: 0x00080000u32,
            idr_addr: 0x40020410u64,
            idr_mask: 0x00000200u32,
            odr_addr: 0x40020414u64,
            odr_mask: 0x00000200u32,
            bsrr_addr: 0x40020418u64,
            bsrr_set_mask: 0x00000200u32,
            bsrr_reset_mask: 0x02000000u32,
        }
    }


}

#[derive(Debug, Clone)]
pub struct GpioBFlex {
    resources: GpioBResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    moder_addr: u64,
    moder_clear_mask: u32,
    moder_output_mask: u32,
    pupdr_addr: u64,
    pupdr_clear_mask: u32,
    pupdr_up_mask: u32,
    pupdr_down_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
    bsrr_addr: u64,
    bsrr_set_mask: u32,
    bsrr_reset_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GpioBInput {
    pin: GpioBFlex,
}

#[derive(Debug, Clone)]
pub struct GpioBOutput {
    pin: GpioBFlex,
}

impl GpioBFlex {
    pub fn resources(&self) -> GpioBResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GpioBInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GpioBInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GpioBOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GpioBOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, self.moder_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        let set_mask = match pull {
            Pull::None => 0x00000000u32,
            Pull::Up => self.pupdr_up_mask,
            Pull::Down => self.pupdr_down_mask,
        };
        modify_u32(self.pupdr_addr, self.pupdr_clear_mask, set_mask)?;
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
        write_u32(self.bsrr_addr, self.bsrr_set_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.bsrr_addr, self.bsrr_reset_mask)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GpioBInput {
    pub fn into_flex(self) -> GpioBFlex {
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

impl GpioBOutput {
    pub fn into_flex(self) -> GpioBFlex {
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

// Driver instance: GpioC (gpio-port) from canonical block block.gpioc -> gpio-port
pub const DRV_GPIOC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioc", name: "GPIOC clock", consumer_ref: "periph.gpioc", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOC_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioc", name: "GPIOC reset", target_ref: "periph.gpioc", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.ahb1"), binding_kind: "software", control_refs: &["reg.rcc.ahb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
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
pub struct GpioCResources {
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

pub const DRV_GPIOC_RESOURCES: GpioCResources = GpioCResources {
    clocks: DRV_GPIOC_CLOCK_BINDINGS,
    resets: DRV_GPIOC_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOC_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOC_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOC_DMA_CHANNELS,
    dma: DRV_GPIOC_DMA_ROUTES,
    pins: DRV_GPIOC_PIN_ROLES,
    init_operations: DRV_GPIOC_INIT_OPERATIONS,
    state_machines: DRV_GPIOC_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIOC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GpioC {
    resources: GpioCResources,
}

impl GpioC {
    pub fn new(resources: GpioCResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpioCResources {
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

    /// Assert reset for GPIOC.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for GPIOC.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PC0 pin on GpioC.
    pub fn pc0(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PC0",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00000003u32,
            moder_output_mask: 0x00000001u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00000003u32,
            pupdr_up_mask: 0x00000001u32,
            pupdr_down_mask: 0x00000002u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000001u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000001u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000001u32,
            bsrr_reset_mask: 0x00010000u32,
        }
    }

    /// Access the PC1 pin on GpioC.
    pub fn pc1(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PC1",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x0000000Cu32,
            moder_output_mask: 0x00000004u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x0000000Cu32,
            pupdr_up_mask: 0x00000004u32,
            pupdr_down_mask: 0x00000008u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000002u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000002u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000002u32,
            bsrr_reset_mask: 0x00020000u32,
        }
    }

    /// Access the PC10 pin on GpioC.
    pub fn pc10(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PC10",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00300000u32,
            moder_output_mask: 0x00100000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00300000u32,
            pupdr_up_mask: 0x00100000u32,
            pupdr_down_mask: 0x00200000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000400u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000400u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000400u32,
            bsrr_reset_mask: 0x04000000u32,
        }
    }

    /// Access the PC11 pin on GpioC.
    pub fn pc11(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PC11",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00C00000u32,
            moder_output_mask: 0x00400000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00C00000u32,
            pupdr_up_mask: 0x00400000u32,
            pupdr_down_mask: 0x00800000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000800u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000800u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000800u32,
            bsrr_reset_mask: 0x08000000u32,
        }
    }

    /// Access the PC12 pin on GpioC.
    pub fn pc12(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PC12",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x03000000u32,
            moder_output_mask: 0x01000000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x03000000u32,
            pupdr_up_mask: 0x01000000u32,
            pupdr_down_mask: 0x02000000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00001000u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00001000u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00001000u32,
            bsrr_reset_mask: 0x10000000u32,
        }
    }

    /// Access the PC13 pin on GpioC.
    pub fn pc13(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PC13",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x0C000000u32,
            moder_output_mask: 0x04000000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x0C000000u32,
            pupdr_up_mask: 0x04000000u32,
            pupdr_down_mask: 0x08000000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00002000u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00002000u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00002000u32,
            bsrr_reset_mask: 0x20000000u32,
        }
    }

    /// Access the PC14 pin on GpioC.
    pub fn pc14(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PC14",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x30000000u32,
            moder_output_mask: 0x10000000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x30000000u32,
            pupdr_up_mask: 0x10000000u32,
            pupdr_down_mask: 0x20000000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00004000u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00004000u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00004000u32,
            bsrr_reset_mask: 0x40000000u32,
        }
    }

    /// Access the PC15 pin on GpioC.
    pub fn pc15(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PC15",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0xC0000000u32,
            moder_output_mask: 0x40000000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0xC0000000u32,
            pupdr_up_mask: 0x40000000u32,
            pupdr_down_mask: 0x80000000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00008000u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00008000u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00008000u32,
            bsrr_reset_mask: 0x80000000u32,
        }
    }

    /// Access the PC2 pin on GpioC.
    pub fn pc2(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[8],
            pin_name: "PC2",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00000030u32,
            moder_output_mask: 0x00000010u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00000030u32,
            pupdr_up_mask: 0x00000010u32,
            pupdr_down_mask: 0x00000020u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000004u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000004u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000004u32,
            bsrr_reset_mask: 0x00040000u32,
        }
    }

    /// Access the PC3 pin on GpioC.
    pub fn pc3(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[9],
            pin_name: "PC3",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x000000C0u32,
            moder_output_mask: 0x00000040u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x000000C0u32,
            pupdr_up_mask: 0x00000040u32,
            pupdr_down_mask: 0x00000080u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000008u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000008u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000008u32,
            bsrr_reset_mask: 0x00080000u32,
        }
    }

    /// Access the PC4 pin on GpioC.
    pub fn pc4(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[10],
            pin_name: "PC4",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00000300u32,
            moder_output_mask: 0x00000100u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00000300u32,
            pupdr_up_mask: 0x00000100u32,
            pupdr_down_mask: 0x00000200u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000010u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000010u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000010u32,
            bsrr_reset_mask: 0x00100000u32,
        }
    }

    /// Access the PC5 pin on GpioC.
    pub fn pc5(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[11],
            pin_name: "PC5",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00000C00u32,
            moder_output_mask: 0x00000400u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00000C00u32,
            pupdr_up_mask: 0x00000400u32,
            pupdr_down_mask: 0x00000800u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000020u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000020u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000020u32,
            bsrr_reset_mask: 0x00200000u32,
        }
    }

    /// Access the PC6 pin on GpioC.
    pub fn pc6(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[12],
            pin_name: "PC6",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00003000u32,
            moder_output_mask: 0x00001000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00003000u32,
            pupdr_up_mask: 0x00001000u32,
            pupdr_down_mask: 0x00002000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000040u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000040u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000040u32,
            bsrr_reset_mask: 0x00400000u32,
        }
    }

    /// Access the PC7 pin on GpioC.
    pub fn pc7(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[13],
            pin_name: "PC7",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x0000C000u32,
            moder_output_mask: 0x00004000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x0000C000u32,
            pupdr_up_mask: 0x00004000u32,
            pupdr_down_mask: 0x00008000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000080u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000080u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000080u32,
            bsrr_reset_mask: 0x00800000u32,
        }
    }

    /// Access the PC8 pin on GpioC.
    pub fn pc8(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[14],
            pin_name: "PC8",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x00030000u32,
            moder_output_mask: 0x00010000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x00030000u32,
            pupdr_up_mask: 0x00010000u32,
            pupdr_down_mask: 0x00020000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000100u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000100u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000100u32,
            bsrr_reset_mask: 0x01000000u32,
        }
    }

    /// Access the PC9 pin on GpioC.
    pub fn pc9(&self) -> GpioCFlex {
        GpioCFlex {
            resources: self.resources,
            role: &self.resources.pins[15],
            pin_name: "PC9",
            moder_addr: 0x40020800u64,
            moder_clear_mask: 0x000C0000u32,
            moder_output_mask: 0x00040000u32,
            pupdr_addr: 0x4002080Cu64,
            pupdr_clear_mask: 0x000C0000u32,
            pupdr_up_mask: 0x00040000u32,
            pupdr_down_mask: 0x00080000u32,
            idr_addr: 0x40020810u64,
            idr_mask: 0x00000200u32,
            odr_addr: 0x40020814u64,
            odr_mask: 0x00000200u32,
            bsrr_addr: 0x40020818u64,
            bsrr_set_mask: 0x00000200u32,
            bsrr_reset_mask: 0x02000000u32,
        }
    }


}

#[derive(Debug, Clone)]
pub struct GpioCFlex {
    resources: GpioCResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    moder_addr: u64,
    moder_clear_mask: u32,
    moder_output_mask: u32,
    pupdr_addr: u64,
    pupdr_clear_mask: u32,
    pupdr_up_mask: u32,
    pupdr_down_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
    bsrr_addr: u64,
    bsrr_set_mask: u32,
    bsrr_reset_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GpioCInput {
    pin: GpioCFlex,
}

#[derive(Debug, Clone)]
pub struct GpioCOutput {
    pin: GpioCFlex,
}

impl GpioCFlex {
    pub fn resources(&self) -> GpioCResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GpioCInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GpioCInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GpioCOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GpioCOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, self.moder_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        let set_mask = match pull {
            Pull::None => 0x00000000u32,
            Pull::Up => self.pupdr_up_mask,
            Pull::Down => self.pupdr_down_mask,
        };
        modify_u32(self.pupdr_addr, self.pupdr_clear_mask, set_mask)?;
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
        write_u32(self.bsrr_addr, self.bsrr_set_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.bsrr_addr, self.bsrr_reset_mask)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GpioCInput {
    pub fn into_flex(self) -> GpioCFlex {
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

impl GpioCOutput {
    pub fn into_flex(self) -> GpioCFlex {
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

// Driver instance: GpioD (gpio-port) from canonical block block.gpiod -> gpio-port
pub const DRV_GPIOD_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiod", name: "GPIOD clock", consumer_ref: "periph.gpiod", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOD_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpiod", name: "GPIOD reset", target_ref: "periph.gpiod", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.ahb1"), binding_kind: "software", control_refs: &["reg.rcc.ahb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
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
pub struct GpioDResources {
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

pub const DRV_GPIOD_RESOURCES: GpioDResources = GpioDResources {
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
pub struct GpioD {
    resources: GpioDResources,
}

impl GpioD {
    pub fn new(resources: GpioDResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpioDResources {
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

    /// Assert reset for GPIOD.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Release reset for GPIOD.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PD2 pin on GpioD.
    pub fn pd2(&self) -> GpioDFlex {
        GpioDFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PD2",
            moder_addr: 0x40020C00u64,
            moder_clear_mask: 0x00000030u32,
            moder_output_mask: 0x00000010u32,
            pupdr_addr: 0x40020C0Cu64,
            pupdr_clear_mask: 0x00000030u32,
            pupdr_up_mask: 0x00000010u32,
            pupdr_down_mask: 0x00000020u32,
            idr_addr: 0x40020C10u64,
            idr_mask: 0x00000004u32,
            odr_addr: 0x40020C14u64,
            odr_mask: 0x00000004u32,
            bsrr_addr: 0x40020C18u64,
            bsrr_set_mask: 0x00000004u32,
            bsrr_reset_mask: 0x00040000u32,
        }
    }


}

#[derive(Debug, Clone)]
pub struct GpioDFlex {
    resources: GpioDResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    moder_addr: u64,
    moder_clear_mask: u32,
    moder_output_mask: u32,
    pupdr_addr: u64,
    pupdr_clear_mask: u32,
    pupdr_up_mask: u32,
    pupdr_down_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
    bsrr_addr: u64,
    bsrr_set_mask: u32,
    bsrr_reset_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GpioDInput {
    pin: GpioDFlex,
}

#[derive(Debug, Clone)]
pub struct GpioDOutput {
    pin: GpioDFlex,
}

impl GpioDFlex {
    pub fn resources(&self) -> GpioDResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GpioDInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GpioDInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GpioDOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GpioDOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, self.moder_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        let set_mask = match pull {
            Pull::None => 0x00000000u32,
            Pull::Up => self.pupdr_up_mask,
            Pull::Down => self.pupdr_down_mask,
        };
        modify_u32(self.pupdr_addr, self.pupdr_clear_mask, set_mask)?;
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
        write_u32(self.bsrr_addr, self.bsrr_set_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.bsrr_addr, self.bsrr_reset_mask)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GpioDInput {
    pub fn into_flex(self) -> GpioDFlex {
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

impl GpioDOutput {
    pub fn into_flex(self) -> GpioDFlex {
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

// Driver instance: GpioH (gpio-port) from canonical block block.gpioh -> gpio-port
pub const DRV_GPIOH_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioh", name: "GPIOH clock", consumer_ref: "periph.gpioh", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOH_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioh", name: "GPIOH reset", target_ref: "periph.gpioh", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.ahb1"), binding_kind: "software", control_refs: &["reg.rcc.ahb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
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
pub struct GpioHResources {
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

pub const DRV_GPIOH_RESOURCES: GpioHResources = GpioHResources {
    clocks: DRV_GPIOH_CLOCK_BINDINGS,
    resets: DRV_GPIOH_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOH_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOH_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOH_DMA_CHANNELS,
    dma: DRV_GPIOH_DMA_ROUTES,
    pins: DRV_GPIOH_PIN_ROLES,
    init_operations: DRV_GPIOH_INIT_OPERATIONS,
    state_machines: DRV_GPIOH_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIOH_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GpioH {
    resources: GpioHResources,
}

impl GpioH {
    pub fn new(resources: GpioHResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GpioHResources {
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

    /// Assert reset for GPIOH.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Release reset for GPIOH.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023810u64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PH0 pin on GpioH.
    pub fn ph0(&self) -> GpioHFlex {
        GpioHFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PH0",
            moder_addr: 0x40021C00u64,
            moder_clear_mask: 0x00000003u32,
            moder_output_mask: 0x00000001u32,
            pupdr_addr: 0x40021C0Cu64,
            pupdr_clear_mask: 0x00000003u32,
            pupdr_up_mask: 0x00000001u32,
            pupdr_down_mask: 0x00000002u32,
            idr_addr: 0x40021C10u64,
            idr_mask: 0x00000001u32,
            odr_addr: 0x40021C14u64,
            odr_mask: 0x00000001u32,
            bsrr_addr: 0x40021C18u64,
            bsrr_set_mask: 0x00000001u32,
            bsrr_reset_mask: 0x00010000u32,
        }
    }

    /// Access the PH1 pin on GpioH.
    pub fn ph1(&self) -> GpioHFlex {
        GpioHFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PH1",
            moder_addr: 0x40021C00u64,
            moder_clear_mask: 0x0000000Cu32,
            moder_output_mask: 0x00000004u32,
            pupdr_addr: 0x40021C0Cu64,
            pupdr_clear_mask: 0x0000000Cu32,
            pupdr_up_mask: 0x00000004u32,
            pupdr_down_mask: 0x00000008u32,
            idr_addr: 0x40021C10u64,
            idr_mask: 0x00000002u32,
            odr_addr: 0x40021C14u64,
            odr_mask: 0x00000002u32,
            bsrr_addr: 0x40021C18u64,
            bsrr_set_mask: 0x00000002u32,
            bsrr_reset_mask: 0x00020000u32,
        }
    }


}

#[derive(Debug, Clone)]
pub struct GpioHFlex {
    resources: GpioHResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    moder_addr: u64,
    moder_clear_mask: u32,
    moder_output_mask: u32,
    pupdr_addr: u64,
    pupdr_clear_mask: u32,
    pupdr_up_mask: u32,
    pupdr_down_mask: u32,
    idr_addr: u64,
    idr_mask: u32,
    odr_addr: u64,
    odr_mask: u32,
    bsrr_addr: u64,
    bsrr_set_mask: u32,
    bsrr_reset_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GpioHInput {
    pin: GpioHFlex,
}

#[derive(Debug, Clone)]
pub struct GpioHOutput {
    pin: GpioHFlex,
}

impl GpioHFlex {
    pub fn resources(&self) -> GpioHResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GpioHInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GpioHInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GpioHOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GpioHOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        self.set_level(initial_level)?;
        modify_u32(self.moder_addr, self.moder_clear_mask, self.moder_output_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        let set_mask = match pull {
            Pull::None => 0x00000000u32,
            Pull::Up => self.pupdr_up_mask,
            Pull::Down => self.pupdr_down_mask,
        };
        modify_u32(self.pupdr_addr, self.pupdr_clear_mask, set_mask)?;
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
        write_u32(self.bsrr_addr, self.bsrr_set_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.bsrr_addr, self.bsrr_reset_mask)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GpioHInput {
    pub fn into_flex(self) -> GpioHFlex {
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

impl GpioHOutput {
    pub fn into_flex(self) -> GpioHFlex {
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

