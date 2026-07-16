//! Generated Embassy-style gpio module for ESP32-C3FN4.

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

// Driver instance: GPIOPort (gpio-port) from canonical block block.gpio0 -> gpio-port
pub const DRV_GPIO_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[];
pub const DRV_GPIO_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_GPIO_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_GPIO_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_GPIO_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_GPIO_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_GPIO_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio0",
    name: "GPIO function on GPIO0",
    pin_ref: "pin.gpio0",
    peripheral_ref: "per.gpio",
    signal: "GPIO0",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio1",
    name: "GPIO function on GPIO1",
    pin_ref: "pin.gpio1",
    peripheral_ref: "per.gpio",
    signal: "GPIO1",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio2",
    name: "GPIO function on GPIO2",
    pin_ref: "pin.gpio2",
    peripheral_ref: "per.gpio",
    signal: "GPIO2",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio3",
    name: "GPIO function on GPIO3",
    pin_ref: "pin.gpio3",
    peripheral_ref: "per.gpio",
    signal: "GPIO3",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio4",
    name: "GPIO function on GPIO4",
    pin_ref: "pin.gpio4",
    peripheral_ref: "per.gpio",
    signal: "GPIO4",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio5",
    name: "GPIO function on GPIO5",
    pin_ref: "pin.gpio5",
    peripheral_ref: "per.gpio",
    signal: "GPIO5",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio6",
    name: "GPIO function on GPIO6",
    pin_ref: "pin.gpio6",
    peripheral_ref: "per.gpio",
    signal: "GPIO6",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio7",
    name: "GPIO function on GPIO7",
    pin_ref: "pin.gpio7",
    peripheral_ref: "per.gpio",
    signal: "GPIO7",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_8_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio8",
    name: "GPIO function on GPIO8",
    pin_ref: "pin.gpio8",
    peripheral_ref: "per.gpio",
    signal: "GPIO8",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_9_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio9",
    name: "GPIO function on GPIO9",
    pin_ref: "pin.gpio9",
    peripheral_ref: "per.gpio",
    signal: "GPIO9",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_10_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio10",
    name: "GPIO function on GPIO10",
    pin_ref: "pin.gpio10",
    peripheral_ref: "per.gpio",
    signal: "GPIO10",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_11_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio11",
    name: "GPIO function on GPIO11",
    pin_ref: "pin.gpio11",
    peripheral_ref: "per.gpio",
    signal: "GPIO11",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_12_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio12",
    name: "GPIO function on GPIO12",
    pin_ref: "pin.gpio12",
    peripheral_ref: "per.gpio",
    signal: "GPIO12",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_13_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio13",
    name: "GPIO function on GPIO13",
    pin_ref: "pin.gpio13",
    peripheral_ref: "per.gpio",
    signal: "GPIO13",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_14_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio14",
    name: "GPIO function on GPIO14",
    pin_ref: "pin.gpio14",
    peripheral_ref: "per.gpio",
    signal: "GPIO14",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_15_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio15",
    name: "GPIO function on GPIO15",
    pin_ref: "pin.gpio15",
    peripheral_ref: "per.gpio",
    signal: "GPIO15",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_16_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio16",
    name: "GPIO function on GPIO16",
    pin_ref: "pin.gpio16",
    peripheral_ref: "per.gpio",
    signal: "GPIO16",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_17_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio17",
    name: "GPIO function on GPIO17",
    pin_ref: "pin.gpio17",
    peripheral_ref: "per.gpio",
    signal: "GPIO17",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_18_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio18",
    name: "GPIO function on GPIO18",
    pin_ref: "pin.gpio18",
    peripheral_ref: "per.gpio",
    signal: "GPIO18",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_19_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio19",
    name: "GPIO function on GPIO19",
    pin_ref: "pin.gpio19",
    peripheral_ref: "per.gpio",
    signal: "GPIO19",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_20_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio20",
    name: "GPIO function on GPIO20",
    pin_ref: "pin.gpio20",
    peripheral_ref: "per.gpio",
    signal: "GPIO20",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLE_21_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.gpio.gpio21",
    name: "GPIO function on GPIO21",
    pin_ref: "pin.gpio21",
    peripheral_ref: "per.gpio",
    signal: "GPIO21",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_GPIO_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "gpio0",
        signal: "GPIO0",
        routes: DRV_GPIO_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio1",
        signal: "GPIO1",
        routes: DRV_GPIO_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio2",
        signal: "GPIO2",
        routes: DRV_GPIO_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio3",
        signal: "GPIO3",
        routes: DRV_GPIO_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio4",
        signal: "GPIO4",
        routes: DRV_GPIO_PIN_ROLE_4_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio5",
        signal: "GPIO5",
        routes: DRV_GPIO_PIN_ROLE_5_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio6",
        signal: "GPIO6",
        routes: DRV_GPIO_PIN_ROLE_6_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio7",
        signal: "GPIO7",
        routes: DRV_GPIO_PIN_ROLE_7_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio8",
        signal: "GPIO8",
        routes: DRV_GPIO_PIN_ROLE_8_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio9",
        signal: "GPIO9",
        routes: DRV_GPIO_PIN_ROLE_9_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio10",
        signal: "GPIO10",
        routes: DRV_GPIO_PIN_ROLE_10_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio11",
        signal: "GPIO11",
        routes: DRV_GPIO_PIN_ROLE_11_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio12",
        signal: "GPIO12",
        routes: DRV_GPIO_PIN_ROLE_12_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio13",
        signal: "GPIO13",
        routes: DRV_GPIO_PIN_ROLE_13_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio14",
        signal: "GPIO14",
        routes: DRV_GPIO_PIN_ROLE_14_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio15",
        signal: "GPIO15",
        routes: DRV_GPIO_PIN_ROLE_15_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio16",
        signal: "GPIO16",
        routes: DRV_GPIO_PIN_ROLE_16_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio17",
        signal: "GPIO17",
        routes: DRV_GPIO_PIN_ROLE_17_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio18",
        signal: "GPIO18",
        routes: DRV_GPIO_PIN_ROLE_18_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio19",
        signal: "GPIO19",
        routes: DRV_GPIO_PIN_ROLE_19_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio20",
        signal: "GPIO20",
        routes: DRV_GPIO_PIN_ROLE_20_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "gpio21",
        signal: "GPIO21",
        routes: DRV_GPIO_PIN_ROLE_21_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
];
pub const DRV_GPIO_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_GPIO_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_GPIO_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct GPIOPortRuntimeResources {}

pub const DRV_GPIO_RUNTIME_RESOURCES: GPIOPortRuntimeResources = GPIOPortRuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct GPIOPortMetadataResources {
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

pub const DRV_GPIO_METADATA_RESOURCES: GPIOPortMetadataResources = GPIOPortMetadataResources {
    clocks: DRV_GPIO_CLOCK_BINDINGS,
    resets: DRV_GPIO_RESET_BINDINGS,
    interrupt_sources: DRV_GPIO_INTERRUPT_SOURCES,
    interrupts: DRV_GPIO_INTERRUPT_ROUTES,
    dma_channels: DRV_GPIO_DMA_CHANNELS,
    dma: DRV_GPIO_DMA_ROUTES,
    pins: DRV_GPIO_PIN_ROLES,
    init_operations: DRV_GPIO_INIT_OPERATIONS,
    state_machines: DRV_GPIO_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_GPIO_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct GPIOPort;

impl GPIOPort {
    pub fn new(resources: GPIOPortRuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> GPIOPortMetadataResources {
        DRV_GPIO_METADATA_RESOURCES
    }
    /// Access the GPIO0 pin on GPIOPort.
    pub fn gpio0(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO0",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004554u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009004u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000001u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO1 pin on GPIOPort.
    pub fn gpio1(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO1",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004558u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009008u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000002u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO2 pin on GPIOPort.
    pub fn gpio2(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO2",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x6000455Cu64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x6000900Cu64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000004u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO3 pin on GPIOPort.
    pub fn gpio3(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO3",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004560u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009010u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000008u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO4 pin on GPIOPort.
    pub fn gpio4(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO4",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004564u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009014u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000010u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO5 pin on GPIOPort.
    pub fn gpio5(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO5",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004568u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009018u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000020u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO6 pin on GPIOPort.
    pub fn gpio6(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO6",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x6000456Cu64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x6000901Cu64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000040u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO7 pin on GPIOPort.
    pub fn gpio7(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO7",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004570u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009020u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000080u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO8 pin on GPIOPort.
    pub fn gpio8(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO8",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004574u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009024u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000100u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO9 pin on GPIOPort.
    pub fn gpio9(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO9",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004578u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009028u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000200u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO10 pin on GPIOPort.
    pub fn gpio10(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO10",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x6000457Cu64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x6000902Cu64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000400u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO11 pin on GPIOPort.
    pub fn gpio11(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO11",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004580u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009030u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00000800u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO12 pin on GPIOPort.
    pub fn gpio12(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO12",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004584u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009034u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00001000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO13 pin on GPIOPort.
    pub fn gpio13(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO13",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004588u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009038u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00002000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO14 pin on GPIOPort.
    pub fn gpio14(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO14",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x6000458Cu64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x6000903Cu64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00004000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO15 pin on GPIOPort.
    pub fn gpio15(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO15",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004590u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009040u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00008000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO16 pin on GPIOPort.
    pub fn gpio16(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO16",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004594u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009044u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00010000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO17 pin on GPIOPort.
    pub fn gpio17(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO17",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x60004598u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009048u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00020000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO18 pin on GPIOPort.
    pub fn gpio18(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO18",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x6000459Cu64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x6000904Cu64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00040000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO19 pin on GPIOPort.
    pub fn gpio19(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO19",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x600045A0u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009050u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00080000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO20 pin on GPIOPort.
    pub fn gpio20(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO20",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x600045A4u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009054u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00100000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }

    /// Access the GPIO21 pin on GPIOPort.
    pub fn gpio21(&self) -> GPIOPortFlex {
        GPIOPortFlex {
            pin_name: "GPIO21",
            out_addr: 0x60004004u64,
            out_w1ts_addr: 0x60004008u64,
            out_w1tc_addr: 0x6000400Cu64,
            enable_w1ts_addr: 0x60004024u64,
            enable_w1tc_addr: 0x60004028u64,
            input_addr: 0x6000403Cu64,
            out_sel_cfg_addr: 0x600045A8u64,
            out_sel_clear_mask: 0x000000FFu32,
            out_sel_gpio_mask: 0x00000080u32,
            inv_sel_mask: 0x00000100u32,
            oen_sel_mask: 0x00000200u32,
            oen_inv_sel_mask: 0x00000400u32,
            io_mux_addr: 0x60009058u64,
            mcu_sel_mask: 0x00007000u32,
            bit_mask: 0x00200000u32,
            fun_wpd_mask: 0x00000080u32,
            fun_wpu_mask: 0x00000100u32,
            fun_ie_mask: 0x00000200u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPIOPortFlex {
    pin_name: &'static str,
    out_addr: u64,
    out_w1ts_addr: u64,
    out_w1tc_addr: u64,
    enable_w1ts_addr: u64,
    enable_w1tc_addr: u64,
    input_addr: u64,
    out_sel_cfg_addr: u64,
    out_sel_clear_mask: u32,
    out_sel_gpio_mask: u32,
    inv_sel_mask: u32,
    oen_sel_mask: u32,
    oen_inv_sel_mask: u32,
    io_mux_addr: u64,
    mcu_sel_mask: u32,
    bit_mask: u32,
    fun_wpd_mask: u32,
    fun_wpu_mask: u32,
    fun_ie_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GPIOPortInput {
    pin: GPIOPortFlex,
}

#[derive(Debug, Clone)]
pub struct GPIOPortOutput {
    pin: GPIOPortFlex,
}

impl GPIOPortFlex {
    pub fn pin_name(&self) -> &'static str {
        self.pin_name
    }

    pub fn into_input(self, pull: Pull) -> Result<GPIOPortInput, metadata::Error> {
        self.set_as_input(pull)?;
        Ok(GPIOPortInput { pin: self })
    }

    pub fn into_output(self, initial_level: Level) -> Result<GPIOPortOutput, metadata::Error> {
        self.set_as_output(initial_level)?;
        Ok(GPIOPortOutput { pin: self })
    }

    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {
        self.set_pull(pull)?;
        modify_u32(
            self.io_mux_addr,
            self.mcu_sel_mask | self.fun_ie_mask,
            self.fun_ie_mask,
        )?;
        write_u32(self.enable_w1tc_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {
        modify_u32(
            self.out_sel_cfg_addr,
            self.out_sel_clear_mask | self.inv_sel_mask | self.oen_sel_mask | self.oen_inv_sel_mask,
            self.out_sel_gpio_mask,
        )?;
        self.set_level(initial_level)?;
        modify_u32(
            self.io_mux_addr,
            self.mcu_sel_mask | self.fun_ie_mask,
            0x00000000u32,
        )?;
        write_u32(self.enable_w1ts_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {
        let set_mask = match pull {
            Pull::None => 0x00000000u32,
            Pull::Up => self.fun_wpu_mask,
            Pull::Down => self.fun_wpd_mask,
        };
        modify_u32(
            self.io_mux_addr,
            self.fun_wpu_mask | self.fun_wpd_mask,
            set_mask,
        )?;
        Ok(())
    }

    pub fn is_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.input_addr)? & self.bit_mask) != 0)
    }

    pub fn is_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_high()?)
    }

    pub fn get_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_high()? {
            Level::High
        } else {
            Level::Low
        })
    }

    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(self.out_addr)? & self.bit_mask) != 0)
    }

    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {
        Ok(!self.is_set_high()?)
    }

    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {
        Ok(if self.is_set_high()? {
            Level::High
        } else {
            Level::Low
        })
    }

    pub fn set_high(&self) -> Result<(), metadata::Error> {
        write_u32(self.out_w1ts_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_low(&self) -> Result<(), metadata::Error> {
        write_u32(self.out_w1tc_addr, self.bit_mask)?;
        Ok(())
    }

    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }
}

impl GPIOPortInput {
    pub fn into_flex(self) -> GPIOPortFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
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

impl GPIOPortOutput {
    pub fn into_flex(self) -> GPIOPortFlex {
        self.pin
    }

    pub fn pin_name(&self) -> &'static str {
        self.pin.pin_name()
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
