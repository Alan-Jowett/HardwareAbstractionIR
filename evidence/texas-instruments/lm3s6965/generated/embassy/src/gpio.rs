//! Generated Embassy-style gpio module for LM3S6965.

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
pub const DRV_GPIOA_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioa", name: "GPIOA", consumer_ref: "periph.gpioa", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOA_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioa", name: "GPIOA", target_ref: "periph.gpioa", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOA_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOA_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOA_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOA_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOA_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa0", name: "GPIOA PA0", pin_ref: "pin.pa0", peripheral_ref: "periph.gpioa", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa1", name: "GPIOA PA1", pin_ref: "pin.pa1", peripheral_ref: "periph.gpioa", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa2", name: "GPIOA PA2", pin_ref: "pin.pa2", peripheral_ref: "periph.gpioa", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa3", name: "GPIOA PA3", pin_ref: "pin.pa3", peripheral_ref: "periph.gpioa", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa4", name: "GPIOA PA4", pin_ref: "pin.pa4", peripheral_ref: "periph.gpioa", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa5", name: "GPIOA PA5", pin_ref: "pin.pa5", peripheral_ref: "periph.gpioa", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa6", name: "GPIOA PA6", pin_ref: "pin.pa6", peripheral_ref: "periph.gpioa", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioa.pa7", name: "GPIOA PA7", pin_ref: "pin.pa7", peripheral_ref: "periph.gpioa", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOA_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOA_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOA_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOA_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOA_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOA_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOA_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOA_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOA_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }];
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
        modify_u32(0x400FE100u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the GPIOA clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOA.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Release reset for GPIOA.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PA0 pin on GPIOA.
    pub fn pa0(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PA0",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004004u64,
            bit_mask: 0x00000001u32,
        }
    }

    /// Access the PA1 pin on GPIOA.
    pub fn pa1(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PA1",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004008u64,
            bit_mask: 0x00000002u32,
        }
    }

    /// Access the PA2 pin on GPIOA.
    pub fn pa2(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PA2",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004010u64,
            bit_mask: 0x00000004u32,
        }
    }

    /// Access the PA3 pin on GPIOA.
    pub fn pa3(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PA3",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004020u64,
            bit_mask: 0x00000008u32,
        }
    }

    /// Access the PA4 pin on GPIOA.
    pub fn pa4(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PA4",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004040u64,
            bit_mask: 0x00000010u32,
        }
    }

    /// Access the PA5 pin on GPIOA.
    pub fn pa5(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PA5",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004080u64,
            bit_mask: 0x00000020u32,
        }
    }

    /// Access the PA6 pin on GPIOA.
    pub fn pa6(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PA6",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004100u64,
            bit_mask: 0x00000040u32,
        }
    }

    /// Access the PA7 pin on GPIOA.
    pub fn pa7(&self) -> GPIOAFlex {
        GPIOAFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PA7",
            dir_addr: 0x40004400u64,
            afsel_addr: 0x40004420u64,
            den_addr: 0x4000451Cu64,
            pur_addr: 0x40004510u64,
            pdr_addr: 0x40004514u64,
            data_alias_addr: 0x40004200u64,
            bit_mask: 0x00000080u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOAFlex {
    resources: GPIOAResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    dir_addr: u64,
    afsel_addr: u64,
    den_addr: u64,
    pur_addr: u64,
    pdr_addr: u64,
    data_alias_addr: u64,
    bit_mask: u32,
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
        self.set_pull(pull)?;
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        modify_u32(self.dir_addr, self.bit_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        self.set_level(initial_level)?;
        modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
            }
            Pull::Up => {
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pur_addr, self.bit_mask, self.bit_mask)?;
            }
            Pull::Down => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, self.bit_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok(read_u32(self.data_alias_addr)? != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.is_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.is_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.get_level()
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, 0x00000000u32)?;
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
pub const DRV_GPIOB_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiob", name: "GPIOB", consumer_ref: "periph.gpiob", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOB_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpiob", name: "GPIOB", target_ref: "periph.gpiob", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOB_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOB_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOB_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOB_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOB_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb0", name: "GPIOB PB0", pin_ref: "pin.pb0", peripheral_ref: "periph.gpiob", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb1", name: "GPIOB PB1", pin_ref: "pin.pb1", peripheral_ref: "periph.gpiob", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb2", name: "GPIOB PB2", pin_ref: "pin.pb2", peripheral_ref: "periph.gpiob", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb3", name: "GPIOB PB3", pin_ref: "pin.pb3", peripheral_ref: "periph.gpiob", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb4", name: "GPIOB PB4", pin_ref: "pin.pb4", peripheral_ref: "periph.gpiob", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb5", name: "GPIOB PB5", pin_ref: "pin.pb5", peripheral_ref: "periph.gpiob", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb6", name: "GPIOB PB6", pin_ref: "pin.pb6", peripheral_ref: "periph.gpiob", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiob.pb7", name: "GPIOB PB7", pin_ref: "pin.pb7", peripheral_ref: "periph.gpiob", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOB_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOB_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOB_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOB_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOB_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOB_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOB_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOB_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOB_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }];
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
        modify_u32(0x400FE100u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the GPIOB clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOB.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Release reset for GPIOB.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PB0 pin on GPIOB.
    pub fn pb0(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PB0",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005004u64,
            bit_mask: 0x00000001u32,
        }
    }

    /// Access the PB1 pin on GPIOB.
    pub fn pb1(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PB1",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005008u64,
            bit_mask: 0x00000002u32,
        }
    }

    /// Access the PB2 pin on GPIOB.
    pub fn pb2(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PB2",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005010u64,
            bit_mask: 0x00000004u32,
        }
    }

    /// Access the PB3 pin on GPIOB.
    pub fn pb3(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PB3",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005020u64,
            bit_mask: 0x00000008u32,
        }
    }

    /// Access the PB4 pin on GPIOB.
    pub fn pb4(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PB4",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005040u64,
            bit_mask: 0x00000010u32,
        }
    }

    /// Access the PB5 pin on GPIOB.
    pub fn pb5(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PB5",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005080u64,
            bit_mask: 0x00000020u32,
        }
    }

    /// Access the PB6 pin on GPIOB.
    pub fn pb6(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PB6",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005100u64,
            bit_mask: 0x00000040u32,
        }
    }

    /// Access the PB7 pin on GPIOB.
    pub fn pb7(&self) -> GPIOBFlex {
        GPIOBFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PB7",
            dir_addr: 0x40005400u64,
            afsel_addr: 0x40005420u64,
            den_addr: 0x4000551Cu64,
            pur_addr: 0x40005510u64,
            pdr_addr: 0x40005514u64,
            data_alias_addr: 0x40005200u64,
            bit_mask: 0x00000080u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOBFlex {
    resources: GPIOBResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    dir_addr: u64,
    afsel_addr: u64,
    den_addr: u64,
    pur_addr: u64,
    pdr_addr: u64,
    data_alias_addr: u64,
    bit_mask: u32,
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
        self.set_pull(pull)?;
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        modify_u32(self.dir_addr, self.bit_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        self.set_level(initial_level)?;
        modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
            }
            Pull::Up => {
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pur_addr, self.bit_mask, self.bit_mask)?;
            }
            Pull::Down => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, self.bit_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok(read_u32(self.data_alias_addr)? != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.is_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.is_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.get_level()
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, 0x00000000u32)?;
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
// Driver instance: GPIOC (gpio-port) from canonical block block.gpioc -> gpio-port
pub const DRV_GPIOC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioc", name: "GPIOC", consumer_ref: "periph.gpioc", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOC_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioc", name: "GPIOC", target_ref: "periph.gpioc", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOC_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc0", name: "GPIOC PC0", pin_ref: "pin.pc0", peripheral_ref: "periph.gpioc", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc1", name: "GPIOC PC1", pin_ref: "pin.pc1", peripheral_ref: "periph.gpioc", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc2", name: "GPIOC PC2", pin_ref: "pin.pc2", peripheral_ref: "periph.gpioc", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc3", name: "GPIOC PC3", pin_ref: "pin.pc3", peripheral_ref: "periph.gpioc", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc4", name: "GPIOC PC4", pin_ref: "pin.pc4", peripheral_ref: "periph.gpioc", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc5", name: "GPIOC PC5", pin_ref: "pin.pc5", peripheral_ref: "periph.gpioc", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc6", name: "GPIOC PC6", pin_ref: "pin.pc6", peripheral_ref: "periph.gpioc", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioc.pc7", name: "GPIOC PC7", pin_ref: "pin.pc7", peripheral_ref: "periph.gpioc", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOC_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOC_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOC_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOC_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOC_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOC_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOC_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOC_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOC_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GPIOCResources {
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

pub const DRV_GPIOC_RESOURCES: GPIOCResources = GPIOCResources {
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
pub struct GPIOC {
    resources: GPIOCResources,
}

impl GPIOC {
    pub fn new(resources: GPIOCResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GPIOCResources {
        self.resources
    }
    /// Enable the GPIOC clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the GPIOC clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOC.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for GPIOC.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PC0 pin on GPIOC.
    pub fn pc0(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PC0",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006004u64,
            bit_mask: 0x00000001u32,
        }
    }

    /// Access the PC1 pin on GPIOC.
    pub fn pc1(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PC1",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006008u64,
            bit_mask: 0x00000002u32,
        }
    }

    /// Access the PC2 pin on GPIOC.
    pub fn pc2(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PC2",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006010u64,
            bit_mask: 0x00000004u32,
        }
    }

    /// Access the PC3 pin on GPIOC.
    pub fn pc3(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PC3",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006020u64,
            bit_mask: 0x00000008u32,
        }
    }

    /// Access the PC4 pin on GPIOC.
    pub fn pc4(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PC4",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006040u64,
            bit_mask: 0x00000010u32,
        }
    }

    /// Access the PC5 pin on GPIOC.
    pub fn pc5(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PC5",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006080u64,
            bit_mask: 0x00000020u32,
        }
    }

    /// Access the PC6 pin on GPIOC.
    pub fn pc6(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PC6",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006100u64,
            bit_mask: 0x00000040u32,
        }
    }

    /// Access the PC7 pin on GPIOC.
    pub fn pc7(&self) -> GPIOCFlex {
        GPIOCFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PC7",
            dir_addr: 0x40006400u64,
            afsel_addr: 0x40006420u64,
            den_addr: 0x4000651Cu64,
            pur_addr: 0x40006510u64,
            pdr_addr: 0x40006514u64,
            data_alias_addr: 0x40006200u64,
            bit_mask: 0x00000080u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOCFlex {
    resources: GPIOCResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    dir_addr: u64,
    afsel_addr: u64,
    den_addr: u64,
    pur_addr: u64,
    pdr_addr: u64,
    data_alias_addr: u64,
    bit_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GPIOCInput {
    pin: GPIOCFlex,
}

#[derive(Debug, Clone)]
pub struct GPIOCOutput {
    pin: GPIOCFlex,
}

impl GPIOCFlex {
    pub fn resources(&self) -> GPIOCResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GPIOCInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GPIOCInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GPIOCOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GPIOCOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        modify_u32(self.dir_addr, self.bit_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        self.set_level(initial_level)?;
        modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
            }
            Pull::Up => {
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pur_addr, self.bit_mask, self.bit_mask)?;
            }
            Pull::Down => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, self.bit_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok(read_u32(self.data_alias_addr)? != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.is_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.is_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.get_level()
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GPIOCInput {
    pub fn into_flex(self) -> GPIOCFlex {
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

impl GPIOCOutput {
    pub fn into_flex(self) -> GPIOCFlex {
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
pub const DRV_GPIOD_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiod", name: "GPIOD", consumer_ref: "periph.gpiod", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOD_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpiod", name: "GPIOD", target_ref: "periph.gpiod", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOD_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOD_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOD_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOD_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOD_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd0", name: "GPIOD PD0", pin_ref: "pin.pd0", peripheral_ref: "periph.gpiod", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd1", name: "GPIOD PD1", pin_ref: "pin.pd1", peripheral_ref: "periph.gpiod", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd2", name: "GPIOD PD2", pin_ref: "pin.pd2", peripheral_ref: "periph.gpiod", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd3", name: "GPIOD PD3", pin_ref: "pin.pd3", peripheral_ref: "periph.gpiod", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd4", name: "GPIOD PD4", pin_ref: "pin.pd4", peripheral_ref: "periph.gpiod", signal: "GPIO4", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd5", name: "GPIOD PD5", pin_ref: "pin.pd5", peripheral_ref: "periph.gpiod", signal: "GPIO5", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd6", name: "GPIOD PD6", pin_ref: "pin.pd6", peripheral_ref: "periph.gpiod", signal: "GPIO6", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiod.pd7", name: "GPIOD PD7", pin_ref: "pin.pd7", peripheral_ref: "periph.gpiod", signal: "GPIO7", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOD_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOD_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOD_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOD_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOD_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio4", signal: "GPIO4", routes: DRV_GPIOD_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio5", signal: "GPIO5", routes: DRV_GPIOD_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio6", signal: "GPIO6", routes: DRV_GPIOD_PIN_ROLE_6_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio7", signal: "GPIO7", routes: DRV_GPIOD_PIN_ROLE_7_ROUTES, requirement: metadata::ResourceRequirement::Required }];
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
        modify_u32(0x400FE100u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the GPIOD clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOD.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Release reset for GPIOD.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PD0 pin on GPIOD.
    pub fn pd0(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PD0",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007004u64,
            bit_mask: 0x00000001u32,
        }
    }

    /// Access the PD1 pin on GPIOD.
    pub fn pd1(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PD1",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007008u64,
            bit_mask: 0x00000002u32,
        }
    }

    /// Access the PD2 pin on GPIOD.
    pub fn pd2(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PD2",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007010u64,
            bit_mask: 0x00000004u32,
        }
    }

    /// Access the PD3 pin on GPIOD.
    pub fn pd3(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PD3",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007020u64,
            bit_mask: 0x00000008u32,
        }
    }

    /// Access the PD4 pin on GPIOD.
    pub fn pd4(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[4],
            pin_name: "PD4",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007040u64,
            bit_mask: 0x00000010u32,
        }
    }

    /// Access the PD5 pin on GPIOD.
    pub fn pd5(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[5],
            pin_name: "PD5",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007080u64,
            bit_mask: 0x00000020u32,
        }
    }

    /// Access the PD6 pin on GPIOD.
    pub fn pd6(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[6],
            pin_name: "PD6",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007100u64,
            bit_mask: 0x00000040u32,
        }
    }

    /// Access the PD7 pin on GPIOD.
    pub fn pd7(&self) -> GPIODFlex {
        GPIODFlex {
            resources: self.resources,
            role: &self.resources.pins[7],
            pin_name: "PD7",
            dir_addr: 0x40007400u64,
            afsel_addr: 0x40007420u64,
            den_addr: 0x4000751Cu64,
            pur_addr: 0x40007510u64,
            pdr_addr: 0x40007514u64,
            data_alias_addr: 0x40007200u64,
            bit_mask: 0x00000080u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIODFlex {
    resources: GPIODResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    dir_addr: u64,
    afsel_addr: u64,
    den_addr: u64,
    pur_addr: u64,
    pdr_addr: u64,
    data_alias_addr: u64,
    bit_mask: u32,
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
        self.set_pull(pull)?;
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        modify_u32(self.dir_addr, self.bit_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        self.set_level(initial_level)?;
        modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
            }
            Pull::Up => {
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pur_addr, self.bit_mask, self.bit_mask)?;
            }
            Pull::Down => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, self.bit_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok(read_u32(self.data_alias_addr)? != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.is_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.is_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.get_level()
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, 0x00000000u32)?;
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
// Driver instance: GPIOE (gpio-port) from canonical block block.gpioe -> gpio-port
pub const DRV_GPIOE_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioe", name: "GPIOE", consumer_ref: "periph.gpioe", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOE_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioe", name: "GPIOE", target_ref: "periph.gpioe", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOE_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOE_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOE_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOE_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOE_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioe.pe0", name: "GPIOE PE0", pin_ref: "pin.pe0", peripheral_ref: "periph.gpioe", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOE_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioe.pe1", name: "GPIOE PE1", pin_ref: "pin.pe1", peripheral_ref: "periph.gpioe", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOE_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioe.pe2", name: "GPIOE PE2", pin_ref: "pin.pe2", peripheral_ref: "periph.gpioe", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOE_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpioe.pe3", name: "GPIOE PE3", pin_ref: "pin.pe3", peripheral_ref: "periph.gpioe", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOE_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOE_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOE_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOE_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOE_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOE_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOE_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOE_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GPIOEResources {
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

pub const DRV_GPIOE_RESOURCES: GPIOEResources = GPIOEResources {
    clocks: DRV_GPIOE_CLOCK_BINDINGS,
    resets: DRV_GPIOE_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOE_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOE_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOE_DMA_CHANNELS,
    dma: DRV_GPIOE_DMA_ROUTES,
    pins: DRV_GPIOE_PIN_ROLES,
    init_operations: DRV_GPIOE_INIT_OPERATIONS,
    state_machines: DRV_GPIOE_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIOE_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GPIOE {
    resources: GPIOEResources,
}

impl GPIOE {
    pub fn new(resources: GPIOEResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GPIOEResources {
        self.resources
    }
    /// Enable the GPIOE clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Disable the GPIOE clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOE.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Release reset for GPIOE.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PE0 pin on GPIOE.
    pub fn pe0(&self) -> GPIOEFlex {
        GPIOEFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PE0",
            dir_addr: 0x40024400u64,
            afsel_addr: 0x40024420u64,
            den_addr: 0x4002451Cu64,
            pur_addr: 0x40024510u64,
            pdr_addr: 0x40024514u64,
            data_alias_addr: 0x40024004u64,
            bit_mask: 0x00000001u32,
        }
    }

    /// Access the PE1 pin on GPIOE.
    pub fn pe1(&self) -> GPIOEFlex {
        GPIOEFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PE1",
            dir_addr: 0x40024400u64,
            afsel_addr: 0x40024420u64,
            den_addr: 0x4002451Cu64,
            pur_addr: 0x40024510u64,
            pdr_addr: 0x40024514u64,
            data_alias_addr: 0x40024008u64,
            bit_mask: 0x00000002u32,
        }
    }

    /// Access the PE2 pin on GPIOE.
    pub fn pe2(&self) -> GPIOEFlex {
        GPIOEFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PE2",
            dir_addr: 0x40024400u64,
            afsel_addr: 0x40024420u64,
            den_addr: 0x4002451Cu64,
            pur_addr: 0x40024510u64,
            pdr_addr: 0x40024514u64,
            data_alias_addr: 0x40024010u64,
            bit_mask: 0x00000004u32,
        }
    }

    /// Access the PE3 pin on GPIOE.
    pub fn pe3(&self) -> GPIOEFlex {
        GPIOEFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PE3",
            dir_addr: 0x40024400u64,
            afsel_addr: 0x40024420u64,
            den_addr: 0x4002451Cu64,
            pur_addr: 0x40024510u64,
            pdr_addr: 0x40024514u64,
            data_alias_addr: 0x40024020u64,
            bit_mask: 0x00000008u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOEFlex {
    resources: GPIOEResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    dir_addr: u64,
    afsel_addr: u64,
    den_addr: u64,
    pur_addr: u64,
    pdr_addr: u64,
    data_alias_addr: u64,
    bit_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GPIOEInput {
    pin: GPIOEFlex,
}

#[derive(Debug, Clone)]
pub struct GPIOEOutput {
    pin: GPIOEFlex,
}

impl GPIOEFlex {
    pub fn resources(&self) -> GPIOEResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GPIOEInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GPIOEInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GPIOEOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GPIOEOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        modify_u32(self.dir_addr, self.bit_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        self.set_level(initial_level)?;
        modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
            }
            Pull::Up => {
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pur_addr, self.bit_mask, self.bit_mask)?;
            }
            Pull::Down => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, self.bit_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok(read_u32(self.data_alias_addr)? != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.is_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.is_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.get_level()
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GPIOEInput {
    pub fn into_flex(self) -> GPIOEFlex {
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

impl GPIOEOutput {
    pub fn into_flex(self) -> GPIOEFlex {
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
// Driver instance: GPIOF (gpio-port) from canonical block block.gpiof -> gpio-port
pub const DRV_GPIOF_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpiof", name: "GPIOF", consumer_ref: "periph.gpiof", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_GPIOF_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpiof", name: "GPIOF", target_ref: "periph.gpiof", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_GPIOF_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIOF_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIOF_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIOF_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIOF_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiof.pf0", name: "GPIOF PF0", pin_ref: "pin.pf0", peripheral_ref: "periph.gpiof", signal: "GPIO0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOF_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiof.pf1", name: "GPIOF PF1", pin_ref: "pin.pf1", peripheral_ref: "periph.gpiof", signal: "GPIO1", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOF_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiof.pf2", name: "GPIOF PF2", pin_ref: "pin.pf2", peripheral_ref: "periph.gpiof", signal: "GPIO2", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOF_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.gpiof.pf3", name: "GPIOF PF3", pin_ref: "pin.pf3", peripheral_ref: "periph.gpiof", signal: "GPIO3", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_GPIOF_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "gpio0", signal: "GPIO0", routes: DRV_GPIOF_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio1", signal: "GPIO1", routes: DRV_GPIOF_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio2", signal: "GPIO2", routes: DRV_GPIOF_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "gpio3", signal: "GPIO3", routes: DRV_GPIOF_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_GPIOF_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIOF_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIOF_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GPIOFResources {
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

pub const DRV_GPIOF_RESOURCES: GPIOFResources = GPIOFResources {
    clocks: DRV_GPIOF_CLOCK_BINDINGS,
    resets: DRV_GPIOF_RESET_BINDINGS,
    interrupt_sources: DRV_GPIOF_INTERRUPT_SOURCES,
    interrupts: DRV_GPIOF_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIOF_DMA_CHANNELS,
    dma: DRV_GPIOF_DMA_ROUTES,
    pins: DRV_GPIOF_PIN_ROLES,
    init_operations: DRV_GPIOF_INIT_OPERATIONS,
    state_machines: DRV_GPIOF_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIOF_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GPIOF {
    resources: GPIOFResources,
}

impl GPIOF {
    pub fn new(resources: GPIOFResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> GPIOFResources {
        self.resources
    }
    /// Enable the GPIOF clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the GPIOF clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOF.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Release reset for GPIOF.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Access the PF0 pin on GPIOF.
    pub fn pf0(&self) -> GPIOFFlex {
        GPIOFFlex {
            resources: self.resources,
            role: &self.resources.pins[0],
            pin_name: "PF0",
            dir_addr: 0x40025400u64,
            afsel_addr: 0x40025420u64,
            den_addr: 0x4002551Cu64,
            pur_addr: 0x40025510u64,
            pdr_addr: 0x40025514u64,
            data_alias_addr: 0x40025004u64,
            bit_mask: 0x00000001u32,
        }
    }

    /// Access the PF1 pin on GPIOF.
    pub fn pf1(&self) -> GPIOFFlex {
        GPIOFFlex {
            resources: self.resources,
            role: &self.resources.pins[1],
            pin_name: "PF1",
            dir_addr: 0x40025400u64,
            afsel_addr: 0x40025420u64,
            den_addr: 0x4002551Cu64,
            pur_addr: 0x40025510u64,
            pdr_addr: 0x40025514u64,
            data_alias_addr: 0x40025008u64,
            bit_mask: 0x00000002u32,
        }
    }

    /// Access the PF2 pin on GPIOF.
    pub fn pf2(&self) -> GPIOFFlex {
        GPIOFFlex {
            resources: self.resources,
            role: &self.resources.pins[2],
            pin_name: "PF2",
            dir_addr: 0x40025400u64,
            afsel_addr: 0x40025420u64,
            den_addr: 0x4002551Cu64,
            pur_addr: 0x40025510u64,
            pdr_addr: 0x40025514u64,
            data_alias_addr: 0x40025010u64,
            bit_mask: 0x00000004u32,
        }
    }

    /// Access the PF3 pin on GPIOF.
    pub fn pf3(&self) -> GPIOFFlex {
        GPIOFFlex {
            resources: self.resources,
            role: &self.resources.pins[3],
            pin_name: "PF3",
            dir_addr: 0x40025400u64,
            afsel_addr: 0x40025420u64,
            den_addr: 0x4002551Cu64,
            pur_addr: 0x40025510u64,
            pdr_addr: 0x40025514u64,
            data_alias_addr: 0x40025020u64,
            bit_mask: 0x00000008u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOFFlex {
    resources: GPIOFResources,
    role: &'static metadata::PinRole,
    pin_name: &'static str,
    dir_addr: u64,
    afsel_addr: u64,
    den_addr: u64,
    pur_addr: u64,
    pdr_addr: u64,
    data_alias_addr: u64,
    bit_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GPIOFInput {
    pin: GPIOFFlex,
}

#[derive(Debug, Clone)]
pub struct GPIOFOutput {
    pin: GPIOFFlex,
}

impl GPIOFFlex {
    pub fn resources(&self) -> GPIOFResources {
        self.resources
    }

    pub fn role(&self) -> &'static metadata::PinRole {
        self.role
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GPIOFInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GPIOFInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GPIOFOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GPIOFOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        modify_u32(self.dir_addr, self.bit_mask, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;
        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;
        self.set_level(initial_level)?;
        modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        match pull {
            Pull::None => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
            }
            Pull::Up => {
                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pur_addr, self.bit_mask, self.bit_mask)?;
            }
            Pull::Down => {
                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;
                modify_u32(self.pdr_addr, self.bit_mask, self.bit_mask)?;
            }
        }
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok(read_u32(self.data_alias_addr)? != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? { Level::High } else { Level::Low })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        self.is_high()
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        self.is_low()
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        self.get_level()
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.data_alias_addr, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GPIOFInput {
    pub fn into_flex(self) -> GPIOFFlex {
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

impl GPIOFOutput {
    pub fn into_flex(self) -> GPIOFFlex {
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
