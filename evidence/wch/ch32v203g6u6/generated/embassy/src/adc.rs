//! Generated Embassy-style adc module for CH32V203G6U6.

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
    module_name: "adc",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: ADC1 (adc) from canonical block block.adc1 -> adc
pub const DRV_ADC1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.adc1",
    name: "ADC1 clock binding",
    consumer_ref: "periph.adc1",
    clock_ref: "clk.adc",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.apb2pcenr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_ADC1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.adc1",
    name: "ADC1 reset binding",
    target_ref: "periph.adc1",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rst.apb2"),
    binding_kind: "local",
    control_refs: &["reg.rcc.apb2prstr"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_ADC1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource {
    id: "isrc.adc1.global",
    name: "ADC1 GLOBAL interrupt source",
    source_ref: "periph.adc1",
    producer_ref: None,
    kind: "peripheral",
    flag_refs: &[],
    clear_operation_refs: &[],
}];
pub const DRV_ADC1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.adc1.global",
    name: "ADC1 GLOBAL interrupt route",
    source_ref: "isrc.adc1.global",
    interrupt_ref: "int.adc12",
    controller_ref: "block.pfic",
    cpu_target_ref: None,
    line_index: None,
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_ADC1_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel {
    id: "dmach.dma1.ch1",
    name: "DMA1 Channel 1",
    controller_ref: "block.dma1",
    target_ref: None,
    channel_index: 1,
    capabilities: &[],
    priority_levels: &[],
}];
pub const DRV_ADC1_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute {
    id: "dmaroute.adc1.global",
    name: "ADC1 GLOBAL DMA route",
    peripheral_ref: "periph.adc1",
    signal: Some("GLOBAL"),
    channel_ref: "dmach.dma1.ch1",
    direction: "peripheral-to-memory",
    control_refs: &[],
    shared_channel_group_ref: None,
}];
pub const DRV_ADC1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in0.pa0",
    name: "ADC1 IN0 on PA0",
    pin_ref: "pin.pa0",
    peripheral_ref: "periph.adc1",
    signal: "IN0",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in1.pa1",
    name: "ADC1 IN1 on PA1",
    pin_ref: "pin.pa1",
    peripheral_ref: "periph.adc1",
    signal: "IN1",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in2.pa2",
    name: "ADC1 IN2 on PA2",
    pin_ref: "pin.pa2",
    peripheral_ref: "periph.adc1",
    signal: "IN2",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in3.pa3",
    name: "ADC1 IN3 on PA3",
    pin_ref: "pin.pa3",
    peripheral_ref: "periph.adc1",
    signal: "IN3",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in4.pa4",
    name: "ADC1 IN4 on PA4",
    pin_ref: "pin.pa4",
    peripheral_ref: "periph.adc1",
    signal: "IN4",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in5.pa5",
    name: "ADC1 IN5 on PA5",
    pin_ref: "pin.pa5",
    peripheral_ref: "periph.adc1",
    signal: "IN5",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in6.pa6",
    name: "ADC1 IN6 on PA6",
    pin_ref: "pin.pa6",
    peripheral_ref: "periph.adc1",
    signal: "IN6",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in7.pa7",
    name: "ADC1 IN7 on PA7",
    pin_ref: "pin.pa7",
    peripheral_ref: "periph.adc1",
    signal: "IN7",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_8_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in8.pb0",
    name: "ADC1 IN8 on PB0",
    pin_ref: "pin.pb0",
    peripheral_ref: "periph.adc1",
    signal: "IN8",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLE_9_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc1.in9.pb1",
    name: "ADC1 IN9 on PB1",
    pin_ref: "pin.pb1",
    peripheral_ref: "periph.adc1",
    signal: "IN9",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC1_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "in0",
        signal: "IN0",
        routes: DRV_ADC1_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in1",
        signal: "IN1",
        routes: DRV_ADC1_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in2",
        signal: "IN2",
        routes: DRV_ADC1_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in3",
        signal: "IN3",
        routes: DRV_ADC1_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in4",
        signal: "IN4",
        routes: DRV_ADC1_PIN_ROLE_4_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in5",
        signal: "IN5",
        routes: DRV_ADC1_PIN_ROLE_5_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in6",
        signal: "IN6",
        routes: DRV_ADC1_PIN_ROLE_6_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in7",
        signal: "IN7",
        routes: DRV_ADC1_PIN_ROLE_7_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in8",
        signal: "IN8",
        routes: DRV_ADC1_PIN_ROLE_8_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in9",
        signal: "IN9",
        routes: DRV_ADC1_PIN_ROLE_9_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_ADC1_INIT_OPERATIONS: &[metadata::SemanticOperation] =
    &[metadata::SemanticOperation {
        id: "op.adc1.calibrate",
        name: "ADC1 enable and calibrate",
        description: None,
        kind: Some("initialization"),
        target_refs: &["periph.adc1"],
        steps: &[
            metadata::SemanticOperationStep {
                index: 0,
                action: "write",
                target_ref: Some("reg.adc1.ctlr2"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set ADON = 1",
                }),
                value: None,
                description: Some("Enable the A/D converter by setting CTLR2.ADON."),
            },
            metadata::SemanticOperationStep {
                index: 1,
                action: "write",
                target_ref: Some("reg.adc1.ctlr2"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set RSTCAL = 1",
                }),
                value: None,
                description: Some("Reset the calibration logic by setting CTLR2.RSTCAL."),
            },
            metadata::SemanticOperationStep {
                index: 2,
                action: "write",
                target_ref: Some("reg.adc1.ctlr2"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set CAL = 1",
                }),
                value: None,
                description: Some("Start A/D calibration by setting CTLR2.CAL."),
            },
        ],
        preconditions: &[],
        postconditions: &[],
    }];
pub const DRV_ADC1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_ADC1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct ADC1Resources {
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

pub const DRV_ADC1_RESOURCES: ADC1Resources = ADC1Resources {
    clocks: DRV_ADC1_CLOCK_BINDINGS,
    resets: DRV_ADC1_RESET_BINDINGS,
    interrupt_sources: DRV_ADC1_INTERRUPT_SOURCES,
    interrupts: DRV_ADC1_INTERRUPT_ROUTES,
    dma_channels: DRV_ADC1_DMA_CHANNELS,
    dma: DRV_ADC1_DMA_ROUTES,
    pins: DRV_ADC1_PIN_ROLES,
    init_operations: DRV_ADC1_INIT_OPERATIONS,
    state_machines: DRV_ADC1_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_ADC1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct ADC1 {
    resources: ADC1Resources,
}

impl ADC1 {
    pub fn new(resources: ADC1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> ADC1Resources {
        self.resources
    }
    /// Enable the ADC1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    /// Disable the ADC1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000200u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for ADC1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    /// Release reset for ADC1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000200u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_calibrate(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40012408u64, 0x00000001u32, 0x00000001u32)?;
        modify_u32(0x40012408u64, 0x00000008u32, 0x00000008u32)?;
        modify_u32(0x40012408u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }
}
// Driver instance: ADC2 (adc) from canonical block block.adc2 -> adc
pub const DRV_ADC2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.adc2",
    name: "ADC2 clock binding",
    consumer_ref: "periph.adc2",
    clock_ref: "clk.adc",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.apb2pcenr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_ADC2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.adc2",
    name: "ADC2 reset binding",
    target_ref: "periph.adc2",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rst.apb2"),
    binding_kind: "local",
    control_refs: &["reg.rcc.apb2prstr"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_ADC2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource {
    id: "isrc.adc2.global",
    name: "ADC2 GLOBAL interrupt source",
    source_ref: "periph.adc2",
    producer_ref: None,
    kind: "peripheral",
    flag_refs: &[],
    clear_operation_refs: &[],
}];
pub const DRV_ADC2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.adc2.global",
    name: "ADC2 GLOBAL interrupt route",
    source_ref: "isrc.adc2.global",
    interrupt_ref: "int.adc12",
    controller_ref: "block.pfic",
    cpu_target_ref: None,
    line_index: None,
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_ADC2_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_ADC2_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_ADC2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in0.pa0",
    name: "ADC2 IN0 on PA0",
    pin_ref: "pin.pa0",
    peripheral_ref: "periph.adc2",
    signal: "IN0",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in1.pa1",
    name: "ADC2 IN1 on PA1",
    pin_ref: "pin.pa1",
    peripheral_ref: "periph.adc2",
    signal: "IN1",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in2.pa2",
    name: "ADC2 IN2 on PA2",
    pin_ref: "pin.pa2",
    peripheral_ref: "periph.adc2",
    signal: "IN2",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in3.pa3",
    name: "ADC2 IN3 on PA3",
    pin_ref: "pin.pa3",
    peripheral_ref: "periph.adc2",
    signal: "IN3",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in4.pa4",
    name: "ADC2 IN4 on PA4",
    pin_ref: "pin.pa4",
    peripheral_ref: "periph.adc2",
    signal: "IN4",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in5.pa5",
    name: "ADC2 IN5 on PA5",
    pin_ref: "pin.pa5",
    peripheral_ref: "periph.adc2",
    signal: "IN5",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_6_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in6.pa6",
    name: "ADC2 IN6 on PA6",
    pin_ref: "pin.pa6",
    peripheral_ref: "periph.adc2",
    signal: "IN6",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_7_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in7.pa7",
    name: "ADC2 IN7 on PA7",
    pin_ref: "pin.pa7",
    peripheral_ref: "periph.adc2",
    signal: "IN7",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_8_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in8.pb0",
    name: "ADC2 IN8 on PB0",
    pin_ref: "pin.pb0",
    peripheral_ref: "periph.adc2",
    signal: "IN8",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLE_9_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.in9.pb1",
    name: "ADC2 IN9 on PB1",
    pin_ref: "pin.pb1",
    peripheral_ref: "periph.adc2",
    signal: "IN9",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC2_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "in0",
        signal: "IN0",
        routes: DRV_ADC2_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in1",
        signal: "IN1",
        routes: DRV_ADC2_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in2",
        signal: "IN2",
        routes: DRV_ADC2_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in3",
        signal: "IN3",
        routes: DRV_ADC2_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in4",
        signal: "IN4",
        routes: DRV_ADC2_PIN_ROLE_4_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in5",
        signal: "IN5",
        routes: DRV_ADC2_PIN_ROLE_5_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in6",
        signal: "IN6",
        routes: DRV_ADC2_PIN_ROLE_6_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in7",
        signal: "IN7",
        routes: DRV_ADC2_PIN_ROLE_7_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in8",
        signal: "IN8",
        routes: DRV_ADC2_PIN_ROLE_8_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "in9",
        signal: "IN9",
        routes: DRV_ADC2_PIN_ROLE_9_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_ADC2_INIT_OPERATIONS: &[metadata::SemanticOperation] =
    &[metadata::SemanticOperation {
        id: "op.adc2.calibrate",
        name: "ADC2 enable and calibrate",
        description: None,
        kind: Some("initialization"),
        target_refs: &["periph.adc2"],
        steps: &[
            metadata::SemanticOperationStep {
                index: 0,
                action: "write",
                target_ref: Some("reg.adc2.ctlr2"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set ADON = 1",
                }),
                value: None,
                description: Some("Enable the A/D converter by setting CTLR2.ADON."),
            },
            metadata::SemanticOperationStep {
                index: 1,
                action: "write",
                target_ref: Some("reg.adc2.ctlr2"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set RSTCAL = 1",
                }),
                value: None,
                description: Some("Reset the calibration logic by setting CTLR2.RSTCAL."),
            },
            metadata::SemanticOperationStep {
                index: 2,
                action: "write",
                target_ref: Some("reg.adc2.ctlr2"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set CAL = 1",
                }),
                value: None,
                description: Some("Start A/D calibration by setting CTLR2.CAL."),
            },
        ],
        preconditions: &[],
        postconditions: &[],
    }];
pub const DRV_ADC2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_ADC2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct ADC2Resources {
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

pub const DRV_ADC2_RESOURCES: ADC2Resources = ADC2Resources {
    clocks: DRV_ADC2_CLOCK_BINDINGS,
    resets: DRV_ADC2_RESET_BINDINGS,
    interrupt_sources: DRV_ADC2_INTERRUPT_SOURCES,
    interrupts: DRV_ADC2_INTERRUPT_ROUTES,
    dma_channels: DRV_ADC2_DMA_CHANNELS,
    dma: DRV_ADC2_DMA_ROUTES,
    pins: DRV_ADC2_PIN_ROLES,
    init_operations: DRV_ADC2_INIT_OPERATIONS,
    state_machines: DRV_ADC2_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_ADC2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct ADC2 {
    resources: ADC2Resources,
}

impl ADC2 {
    pub fn new(resources: ADC2Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> ADC2Resources {
        self.resources
    }
    /// Enable the ADC2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000400u32, 0x00000400u32)?;
        Ok(())
    }

    /// Disable the ADC2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000400u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for ADC2.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000400u32, 0x00000400u32)?;
        Ok(())
    }

    /// Release reset for ADC2.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000400u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_calibrate(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40012808u64, 0x00000001u32, 0x00000001u32)?;
        modify_u32(0x40012808u64, 0x00000008u32, 0x00000008u32)?;
        modify_u32(0x40012808u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }
}
