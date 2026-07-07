//! Generated Embassy-style timer module for LM3S6965.

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

// Driver instance: TIMER0 (timer) from canonical block block.timer0 -> timer-general
pub const DRV_TIMER0_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.timer0",
    name: "TIMER0",
    consumer_ref: "periph.timer0",
    clock_ref: "clock.sysclk",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.sysctl.rcgc1"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_TIMER0_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.timer0",
    name: "TIMER0",
    target_ref: "periph.timer0",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rd.software"),
    binding_kind: "software",
    control_refs: &["reg.sysctl.srcr1"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_TIMER0_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[
    metadata::InterruptSource {
        id: "isrc.timer0a",
        name: "TIMER0A interrupt source",
        source_ref: "periph.timer0",
        producer_ref: Some("periph.timer0"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.timer0b",
        name: "TIMER0B interrupt source",
        source_ref: "periph.timer0",
        producer_ref: Some("periph.timer0"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
];
pub const DRV_TIMER0_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[
    metadata::InterruptRoute {
        id: "iroute.timer0a",
        name: "TIMER0A interrupt source route",
        source_ref: "isrc.timer0a",
        interrupt_ref: "int.timer0a",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.timer0b",
        name: "TIMER0B interrupt source route",
        source_ref: "isrc.timer0b",
        interrupt_ref: "int.timer0b",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
];
pub const DRV_TIMER0_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIMER0_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIMER0_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.timer0.ccp0.pd4",
    name: "CCP0 on PD4",
    pin_ref: "pin.pd4",
    peripheral_ref: "periph.timer0",
    signal: "CCP0",
    route_type: "hardwired",
    control_refs: &["reg.gpiod.afsel", "reg.gpiod.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_TIMER0_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole {
    role: "ccp0",
    signal: "CCP0",
    routes: DRV_TIMER0_PIN_ROLE_0_ROUTES,
    requirement: metadata::ResourceRequirement::Optional,
}];
pub const DRV_TIMER0_INIT_OPERATIONS: &[metadata::SemanticOperation] =
    &[metadata::SemanticOperation {
        id: "op.timer0.enable",
        name: "Enable TIMER0A",
        description: None,
        kind: Some("initialization"),
        target_refs: &["periph.timer0"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.timer0.ctl"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Set TAEN = 1",
            }),
            value: None,
            description: Some("Set TAEN to start Timer A."),
        }],
        preconditions: &[],
        postconditions: &[],
    }];
pub const DRV_TIMER0_STATE_MACHINES: &[metadata::SemanticStateMachine] =
    &[metadata::SemanticStateMachine {
        id: "sm.timer0.a",
        name: "TIMER0A enable state",
        description: None,
        target_refs: &["periph.timer0"],
        initial_state: Some("disabled"),
        states: &[
            metadata::SemanticState {
                name: "disabled",
                description: Some("Timer A is not enabled."),
                invariants: &[],
            },
            metadata::SemanticState {
                name: "running",
                description: Some("Timer A is enabled and counting."),
                invariants: &[],
            },
        ],
        transitions: &[
            metadata::SemanticTransition {
                from: "disabled",
                to: "running",
                trigger: Some("Set CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "starts-hardware",
                    target_ref: Some("field.timer0.ctl.taen"),
                    description: Some("Timer A starts counting when TAEN is set."),
                }],
            },
            metadata::SemanticTransition {
                from: "running",
                to: "disabled",
                trigger: Some("Clear CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "stops-hardware",
                    target_ref: Some("field.timer0.ctl.taen"),
                    description: Some("Timer A stops when TAEN is cleared."),
                }],
            },
        ],
    }];
pub const DRV_TIMER0_CAPABILITY_TAGS: &[&str] = &["timeout", "one-shot", "periodic", "basic-timer"];

#[derive(Debug, Clone, Copy)]
pub struct TIMER0Resources {
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

pub const DRV_TIMER0_RESOURCES: TIMER0Resources = TIMER0Resources {
    clocks: DRV_TIMER0_CLOCK_BINDINGS,
    resets: DRV_TIMER0_RESET_BINDINGS,
    interrupt_sources: DRV_TIMER0_INTERRUPT_SOURCES,
    interrupts: DRV_TIMER0_INTERRUPT_ROUTES,
    dma_channels: DRV_TIMER0_DMA_CHANNELS,
    dma: DRV_TIMER0_DMA_ROUTES,
    pins: DRV_TIMER0_PIN_ROLES,
    init_operations: DRV_TIMER0_INIT_OPERATIONS,
    state_machines: DRV_TIMER0_STATE_MACHINES,
    capability_tags: DRV_TIMER0_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIMER0 {
    resources: TIMER0Resources,
}

impl TIMER0 {
    pub fn new(resources: TIMER0Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIMER0Resources {
        self.resources
    }
    /// Enable the TIMER0 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00010000u32, 0x00010000u32)?;
        Ok(())
    }

    /// Disable the TIMER0 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00010000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER0.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00010000u32, 0x00010000u32)?;
        Ok(())
    }

    /// Release reset for TIMER0.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00010000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003000Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_disabled_to_running(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003000Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_running_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003000Cu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }
}

// Driver instance: TIMER1 (timer) from canonical block block.timer1 -> timer-general
pub const DRV_TIMER1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.timer1",
    name: "TIMER1",
    consumer_ref: "periph.timer1",
    clock_ref: "clock.sysclk",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.sysctl.rcgc1"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_TIMER1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.timer1",
    name: "TIMER1",
    target_ref: "periph.timer1",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rd.software"),
    binding_kind: "software",
    control_refs: &["reg.sysctl.srcr1"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_TIMER1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[
    metadata::InterruptSource {
        id: "isrc.timer1a",
        name: "TIMER1A interrupt source",
        source_ref: "periph.timer1",
        producer_ref: Some("periph.timer1"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.timer1b",
        name: "TIMER1B interrupt source",
        source_ref: "periph.timer1",
        producer_ref: Some("periph.timer1"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
];
pub const DRV_TIMER1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[
    metadata::InterruptRoute {
        id: "iroute.timer1a",
        name: "TIMER1A interrupt source route",
        source_ref: "isrc.timer1a",
        interrupt_ref: "int.timer1a",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.timer1b",
        name: "TIMER1B interrupt source route",
        source_ref: "isrc.timer1b",
        interrupt_ref: "int.timer1b",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
];
pub const DRV_TIMER1_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIMER1_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIMER1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.timer1.ccp1.pd7",
    name: "CCP1 on PD7",
    pin_ref: "pin.pd7",
    peripheral_ref: "periph.timer1",
    signal: "CCP1",
    route_type: "hardwired",
    control_refs: &["reg.gpiod.afsel", "reg.gpiod.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_TIMER1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole {
    role: "ccp1",
    signal: "CCP1",
    routes: DRV_TIMER1_PIN_ROLE_0_ROUTES,
    requirement: metadata::ResourceRequirement::Optional,
}];
pub const DRV_TIMER1_INIT_OPERATIONS: &[metadata::SemanticOperation] =
    &[metadata::SemanticOperation {
        id: "op.timer1.enable",
        name: "Enable TIMER1A",
        description: None,
        kind: Some("initialization"),
        target_refs: &["periph.timer1"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.timer1.ctl"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Set TAEN = 1",
            }),
            value: None,
            description: Some("Set TAEN to start Timer A."),
        }],
        preconditions: &[],
        postconditions: &[],
    }];
pub const DRV_TIMER1_STATE_MACHINES: &[metadata::SemanticStateMachine] =
    &[metadata::SemanticStateMachine {
        id: "sm.timer1.a",
        name: "TIMER1A enable state",
        description: None,
        target_refs: &["periph.timer1"],
        initial_state: Some("disabled"),
        states: &[
            metadata::SemanticState {
                name: "disabled",
                description: Some("Timer A is not enabled."),
                invariants: &[],
            },
            metadata::SemanticState {
                name: "running",
                description: Some("Timer A is enabled and counting."),
                invariants: &[],
            },
        ],
        transitions: &[
            metadata::SemanticTransition {
                from: "disabled",
                to: "running",
                trigger: Some("Set CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "starts-hardware",
                    target_ref: Some("field.timer1.ctl.taen"),
                    description: Some("Timer A starts counting when TAEN is set."),
                }],
            },
            metadata::SemanticTransition {
                from: "running",
                to: "disabled",
                trigger: Some("Clear CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "stops-hardware",
                    target_ref: Some("field.timer1.ctl.taen"),
                    description: Some("Timer A stops when TAEN is cleared."),
                }],
            },
        ],
    }];
pub const DRV_TIMER1_CAPABILITY_TAGS: &[&str] = &["timeout", "one-shot", "periodic", "basic-timer"];

#[derive(Debug, Clone, Copy)]
pub struct TIMER1Resources {
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

pub const DRV_TIMER1_RESOURCES: TIMER1Resources = TIMER1Resources {
    clocks: DRV_TIMER1_CLOCK_BINDINGS,
    resets: DRV_TIMER1_RESET_BINDINGS,
    interrupt_sources: DRV_TIMER1_INTERRUPT_SOURCES,
    interrupts: DRV_TIMER1_INTERRUPT_ROUTES,
    dma_channels: DRV_TIMER1_DMA_CHANNELS,
    dma: DRV_TIMER1_DMA_ROUTES,
    pins: DRV_TIMER1_PIN_ROLES,
    init_operations: DRV_TIMER1_INIT_OPERATIONS,
    state_machines: DRV_TIMER1_STATE_MACHINES,
    capability_tags: DRV_TIMER1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIMER1 {
    resources: TIMER1Resources,
}

impl TIMER1 {
    pub fn new(resources: TIMER1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIMER1Resources {
        self.resources
    }
    /// Enable the TIMER1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Disable the TIMER1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Release reset for TIMER1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003100Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_disabled_to_running(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003100Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_running_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003100Cu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }
}

// Driver instance: TIMER2 (timer) from canonical block block.timer2 -> timer-general
pub const DRV_TIMER2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.timer2",
    name: "TIMER2",
    consumer_ref: "periph.timer2",
    clock_ref: "clock.sysclk",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.sysctl.rcgc1"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_TIMER2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.timer2",
    name: "TIMER2",
    target_ref: "periph.timer2",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rd.software"),
    binding_kind: "software",
    control_refs: &["reg.sysctl.srcr1"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_TIMER2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[
    metadata::InterruptSource {
        id: "isrc.timer2a",
        name: "TIMER2A interrupt source",
        source_ref: "periph.timer2",
        producer_ref: Some("periph.timer2"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.timer2b",
        name: "TIMER2B interrupt source",
        source_ref: "periph.timer2",
        producer_ref: Some("periph.timer2"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
];
pub const DRV_TIMER2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[
    metadata::InterruptRoute {
        id: "iroute.timer2a",
        name: "TIMER2A interrupt source route",
        source_ref: "isrc.timer2a",
        interrupt_ref: "int.timer2a",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.timer2b",
        name: "TIMER2B interrupt source route",
        source_ref: "isrc.timer2b",
        interrupt_ref: "int.timer2b",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
];
pub const DRV_TIMER2_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIMER2_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIMER2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.timer2.ccp2.pd5",
    name: "CCP2 on PD5",
    pin_ref: "pin.pd5",
    peripheral_ref: "periph.timer2",
    signal: "CCP2",
    route_type: "hardwired",
    control_refs: &["reg.gpiod.afsel", "reg.gpiod.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_TIMER2_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole {
    role: "ccp2",
    signal: "CCP2",
    routes: DRV_TIMER2_PIN_ROLE_0_ROUTES,
    requirement: metadata::ResourceRequirement::Optional,
}];
pub const DRV_TIMER2_INIT_OPERATIONS: &[metadata::SemanticOperation] =
    &[metadata::SemanticOperation {
        id: "op.timer2.enable",
        name: "Enable TIMER2A",
        description: None,
        kind: Some("initialization"),
        target_refs: &["periph.timer2"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.timer2.ctl"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Set TAEN = 1",
            }),
            value: None,
            description: Some("Set TAEN to start Timer A."),
        }],
        preconditions: &[],
        postconditions: &[],
    }];
pub const DRV_TIMER2_STATE_MACHINES: &[metadata::SemanticStateMachine] =
    &[metadata::SemanticStateMachine {
        id: "sm.timer2.a",
        name: "TIMER2A enable state",
        description: None,
        target_refs: &["periph.timer2"],
        initial_state: Some("disabled"),
        states: &[
            metadata::SemanticState {
                name: "disabled",
                description: Some("Timer A is not enabled."),
                invariants: &[],
            },
            metadata::SemanticState {
                name: "running",
                description: Some("Timer A is enabled and counting."),
                invariants: &[],
            },
        ],
        transitions: &[
            metadata::SemanticTransition {
                from: "disabled",
                to: "running",
                trigger: Some("Set CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "starts-hardware",
                    target_ref: Some("field.timer2.ctl.taen"),
                    description: Some("Timer A starts counting when TAEN is set."),
                }],
            },
            metadata::SemanticTransition {
                from: "running",
                to: "disabled",
                trigger: Some("Clear CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "stops-hardware",
                    target_ref: Some("field.timer2.ctl.taen"),
                    description: Some("Timer A stops when TAEN is cleared."),
                }],
            },
        ],
    }];
pub const DRV_TIMER2_CAPABILITY_TAGS: &[&str] = &["timeout", "one-shot", "periodic", "basic-timer"];

#[derive(Debug, Clone, Copy)]
pub struct TIMER2Resources {
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

pub const DRV_TIMER2_RESOURCES: TIMER2Resources = TIMER2Resources {
    clocks: DRV_TIMER2_CLOCK_BINDINGS,
    resets: DRV_TIMER2_RESET_BINDINGS,
    interrupt_sources: DRV_TIMER2_INTERRUPT_SOURCES,
    interrupts: DRV_TIMER2_INTERRUPT_ROUTES,
    dma_channels: DRV_TIMER2_DMA_CHANNELS,
    dma: DRV_TIMER2_DMA_ROUTES,
    pins: DRV_TIMER2_PIN_ROLES,
    init_operations: DRV_TIMER2_INIT_OPERATIONS,
    state_machines: DRV_TIMER2_STATE_MACHINES,
    capability_tags: DRV_TIMER2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIMER2 {
    resources: TIMER2Resources,
}

impl TIMER2 {
    pub fn new(resources: TIMER2Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIMER2Resources {
        self.resources
    }
    /// Enable the TIMER2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00040000u32, 0x00040000u32)?;
        Ok(())
    }

    /// Disable the TIMER2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00040000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER2.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00040000u32, 0x00040000u32)?;
        Ok(())
    }

    /// Release reset for TIMER2.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00040000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003200Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_disabled_to_running(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003200Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_running_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003200Cu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }
}

// Driver instance: TIMER3 (timer) from canonical block block.timer3 -> timer-general
pub const DRV_TIMER3_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.timer3",
    name: "TIMER3",
    consumer_ref: "periph.timer3",
    clock_ref: "clock.sysclk",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.sysctl.rcgc1"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_TIMER3_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.timer3",
    name: "TIMER3",
    target_ref: "periph.timer3",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rd.software"),
    binding_kind: "software",
    control_refs: &["reg.sysctl.srcr1"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_TIMER3_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[
    metadata::InterruptSource {
        id: "isrc.timer3a",
        name: "TIMER3A interrupt source",
        source_ref: "periph.timer3",
        producer_ref: Some("periph.timer3"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.timer3b",
        name: "TIMER3B interrupt source",
        source_ref: "periph.timer3",
        producer_ref: Some("periph.timer3"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
];
pub const DRV_TIMER3_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[
    metadata::InterruptRoute {
        id: "iroute.timer3a",
        name: "TIMER3A interrupt source route",
        source_ref: "isrc.timer3a",
        interrupt_ref: "int.timer3a",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.timer3b",
        name: "TIMER3B interrupt source route",
        source_ref: "isrc.timer3b",
        interrupt_ref: "int.timer3b",
        controller_ref: "block.nvic",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
];
pub const DRV_TIMER3_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIMER3_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIMER3_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.timer3.ccp3.pc6",
    name: "CCP3 on PC6",
    pin_ref: "pin.pc6",
    peripheral_ref: "periph.timer3",
    signal: "CCP3",
    route_type: "hardwired",
    control_refs: &["reg.gpioc.afsel", "reg.gpioc.den"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(false),
}];
pub const DRV_TIMER3_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole {
    role: "ccp3",
    signal: "CCP3",
    routes: DRV_TIMER3_PIN_ROLE_0_ROUTES,
    requirement: metadata::ResourceRequirement::Optional,
}];
pub const DRV_TIMER3_INIT_OPERATIONS: &[metadata::SemanticOperation] =
    &[metadata::SemanticOperation {
        id: "op.timer3.enable",
        name: "Enable TIMER3A",
        description: None,
        kind: Some("initialization"),
        target_refs: &["periph.timer3"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.timer3.ctl"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Set TAEN = 1",
            }),
            value: None,
            description: Some("Set TAEN to start Timer A."),
        }],
        preconditions: &[],
        postconditions: &[],
    }];
pub const DRV_TIMER3_STATE_MACHINES: &[metadata::SemanticStateMachine] =
    &[metadata::SemanticStateMachine {
        id: "sm.timer3.a",
        name: "TIMER3A enable state",
        description: None,
        target_refs: &["periph.timer3"],
        initial_state: Some("disabled"),
        states: &[
            metadata::SemanticState {
                name: "disabled",
                description: Some("Timer A is not enabled."),
                invariants: &[],
            },
            metadata::SemanticState {
                name: "running",
                description: Some("Timer A is enabled and counting."),
                invariants: &[],
            },
        ],
        transitions: &[
            metadata::SemanticTransition {
                from: "disabled",
                to: "running",
                trigger: Some("Set CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "starts-hardware",
                    target_ref: Some("field.timer3.ctl.taen"),
                    description: Some("Timer A starts counting when TAEN is set."),
                }],
            },
            metadata::SemanticTransition {
                from: "running",
                to: "disabled",
                trigger: Some("Clear CTL.TAEN"),
                conditions: &[],
                effects: &[metadata::SemanticSideEffect {
                    kind: "stops-hardware",
                    target_ref: Some("field.timer3.ctl.taen"),
                    description: Some("Timer A stops when TAEN is cleared."),
                }],
            },
        ],
    }];
pub const DRV_TIMER3_CAPABILITY_TAGS: &[&str] = &["timeout", "one-shot", "periodic", "basic-timer"];

#[derive(Debug, Clone, Copy)]
pub struct TIMER3Resources {
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

pub const DRV_TIMER3_RESOURCES: TIMER3Resources = TIMER3Resources {
    clocks: DRV_TIMER3_CLOCK_BINDINGS,
    resets: DRV_TIMER3_RESET_BINDINGS,
    interrupt_sources: DRV_TIMER3_INTERRUPT_SOURCES,
    interrupts: DRV_TIMER3_INTERRUPT_ROUTES,
    dma_channels: DRV_TIMER3_DMA_CHANNELS,
    dma: DRV_TIMER3_DMA_ROUTES,
    pins: DRV_TIMER3_PIN_ROLES,
    init_operations: DRV_TIMER3_INIT_OPERATIONS,
    state_machines: DRV_TIMER3_STATE_MACHINES,
    capability_tags: DRV_TIMER3_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIMER3 {
    resources: TIMER3Resources,
}

impl TIMER3 {
    pub fn new(resources: TIMER3Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIMER3Resources {
        self.resources
    }
    /// Enable the TIMER3 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00080000u32, 0x00080000u32)?;
        Ok(())
    }

    /// Disable the TIMER3 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00080000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER3.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00080000u32, 0x00080000u32)?;
        Ok(())
    }

    /// Release reset for TIMER3.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00080000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003300Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_disabled_to_running(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003300Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn transition_running_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4003300Cu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }
}
