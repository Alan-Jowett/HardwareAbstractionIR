//! Generated Embassy-style adc module for ESP32-C3FN4.

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

// Driver instance: ApbSarAdc (adc) from canonical block block.apb_saradc -> adc
pub const DRV_ADC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clkbind.apb_saradc",
    name: "APB_SARADC_CLK_EN",
    consumer_ref: "per.apb_saradc",
    clock_ref: "clk.apb",
    controller_ref: Some("block.system"),
    binding_kind: "gated",
    control_refs: &["reg.system.perip_clk_en0"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_ADC_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rstbind.apb_saradc",
    name: "APB_SARADC_RST",
    target_ref: "per.apb_saradc",
    controller_ref: Some("block.system"),
    reset_domain_ref: Some("rst.system"),
    binding_kind: "local",
    control_refs: &["reg.system.perip_rst_en0"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_ADC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource {
    id: "isrc.apb_adc",
    name: "APB ADC",
    source_ref: "per.apb_saradc",
    producer_ref: Some("block.apb_saradc"),
    kind: "peripheral",
    flag_refs: &[],
    clear_operation_refs: &[],
}];
pub const DRV_ADC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.apb_adc",
    name: "APB ADC interrupt matrix source",
    source_ref: "isrc.apb_adc",
    interrupt_ref: "irq.ets_apb_adc_intr_source",
    controller_ref: "block.interrupt_matrix0",
    cpu_target_ref: Some("block.cpu0"),
    line_index: None,
    route_type: "matrix",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_ADC_DMA_CHANNELS: &[metadata::DmaChannel] = &[
    metadata::DmaChannel {
        id: "dma.rx_ch0",
        name: "GDMA RX channel 0",
        controller_ref: "block.gdma0",
        target_ref: None,
        channel_index: 0,
        capabilities: &["rx", "peripheral"],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dma.rx_ch1",
        name: "GDMA RX channel 1",
        controller_ref: "block.gdma0",
        target_ref: None,
        channel_index: 1,
        capabilities: &["rx", "peripheral"],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dma.rx_ch2",
        name: "GDMA RX channel 2",
        controller_ref: "block.gdma0",
        target_ref: None,
        channel_index: 2,
        capabilities: &["rx", "peripheral"],
        priority_levels: &[],
    },
];
pub const DRV_ADC_DMA_ROUTES: &[metadata::DmaRoute] = &[
    metadata::DmaRoute {
        id: "dmaroute.apb_saradc.rx.ch0",
        name: "APB_SARADC via GDMA channel 0",
        peripheral_ref: "per.apb_saradc",
        signal: Some("ADC1"),
        channel_ref: "dma.rx_ch0",
        direction: "peripheral-to-memory",
        control_refs: &["reg.dma.in_peri_sel_ch0"],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.apb_saradc.rx.ch1",
        name: "APB_SARADC via GDMA channel 1",
        peripheral_ref: "per.apb_saradc",
        signal: Some("ADC1"),
        channel_ref: "dma.rx_ch1",
        direction: "peripheral-to-memory",
        control_refs: &["reg.dma.in_peri_sel_ch1"],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.apb_saradc.rx.ch2",
        name: "APB_SARADC via GDMA channel 2",
        peripheral_ref: "per.apb_saradc",
        signal: Some("ADC1"),
        channel_ref: "dma.rx_ch2",
        direction: "peripheral-to-memory",
        control_refs: &["reg.dma.in_peri_sel_ch2"],
        shared_channel_group_ref: None,
    },
];
pub const DRV_ADC_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc.ch0.gpio0",
    name: "ADC1_CH0 on GPIO0",
    pin_ref: "pin.gpio0",
    peripheral_ref: "per.apb_saradc",
    signal: "ADC1_CH0",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc.ch1.gpio1",
    name: "ADC1_CH1 on GPIO1",
    pin_ref: "pin.gpio1",
    peripheral_ref: "per.apb_saradc",
    signal: "ADC1_CH1",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc.ch2.gpio2",
    name: "ADC1_CH2 on GPIO2",
    pin_ref: "pin.gpio2",
    peripheral_ref: "per.apb_saradc",
    signal: "ADC1_CH2",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc.ch3.gpio3",
    name: "ADC1_CH3 on GPIO3",
    pin_ref: "pin.gpio3",
    peripheral_ref: "per.apb_saradc",
    signal: "ADC1_CH3",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc.ch4.gpio4",
    name: "ADC1_CH4 on GPIO4",
    pin_ref: "pin.gpio4",
    peripheral_ref: "per.apb_saradc",
    signal: "ADC1_CH4",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.adc2.ch0.gpio5",
    name: "ADC2_CH0 on GPIO5",
    pin_ref: "pin.gpio5",
    peripheral_ref: "per.apb_saradc",
    signal: "ADC2_CH0",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_ADC_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "ch0",
        signal: "ADC1_CH0",
        routes: DRV_ADC_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "ch1",
        signal: "ADC1_CH1",
        routes: DRV_ADC_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "ch2",
        signal: "ADC1_CH2",
        routes: DRV_ADC_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "ch3",
        signal: "ADC1_CH3",
        routes: DRV_ADC_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "ch4",
        signal: "ADC1_CH4",
        routes: DRV_ADC_PIN_ROLE_4_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "ch5",
        signal: "ADC2_CH0",
        routes: DRV_ADC_PIN_ROLE_5_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_ADC_INIT_OPERATIONS: &[metadata::SemanticOperation] =
    &[metadata::SemanticOperation {
        id: "op.adc.configure_onetime_ch0",
        name: "Configure one-shot ADC1 channel 0 sampling",
        description: None,
        kind: Some("initialization"),
        target_refs: &["per.apb_saradc"],
        steps: &[
            metadata::SemanticOperationStep {
                index: 0,
                action: "write",
                target_ref: Some("reg.apb_saradc.onetime_sample"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Write SARADC_ONETIME_ATTEN = 0",
                }),
                value: None,
                description: Some(
                    "Use 0 dB attenuation for the initial reference conversion path.",
                ),
            },
            metadata::SemanticOperationStep {
                index: 1,
                action: "write",
                target_ref: Some("reg.apb_saradc.onetime_sample"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Write SARADC_ONETIME_CHANNEL = 0",
                }),
                value: None,
                description: Some("Select ADC1 channel 0."),
            },
            metadata::SemanticOperationStep {
                index: 2,
                action: "write",
                target_ref: Some("reg.apb_saradc.onetime_sample"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set SARADC_ONETIME_START",
                }),
                value: None,
                description: Some("Start one-shot conversion using the configured channel."),
            },
        ],
        preconditions: &[],
        postconditions: &[],
    }];
pub const DRV_ADC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_ADC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct ApbSarAdcRuntimeResources {}

pub const DRV_ADC_RUNTIME_RESOURCES: ApbSarAdcRuntimeResources = ApbSarAdcRuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct ApbSarAdcMetadataResources {
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

pub const DRV_ADC_METADATA_RESOURCES: ApbSarAdcMetadataResources = ApbSarAdcMetadataResources {
    clocks: DRV_ADC_CLOCK_BINDINGS,
    resets: DRV_ADC_RESET_BINDINGS,
    interrupt_sources: DRV_ADC_INTERRUPT_SOURCES,
    interrupts: DRV_ADC_INTERRUPT_ROUTES,
    dma_channels: DRV_ADC_DMA_CHANNELS,
    dma: DRV_ADC_DMA_ROUTES,
    pins: DRV_ADC_PIN_ROLES,
    init_operations: DRV_ADC_INIT_OPERATIONS,
    state_machines: DRV_ADC_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_ADC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct ApbSarAdc;

impl ApbSarAdc {
    pub fn new(resources: ApbSarAdcRuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> ApbSarAdcMetadataResources {
        DRV_ADC_METADATA_RESOURCES
    }
    /// Enable the APB_SARADC clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x10000000u32, 0x10000000u32)?;
        Ok(())
    }

    /// Disable the APB_SARADC clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x10000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for APB_SARADC.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x10000000u32, 0x10000000u32)?;
        Ok(())
    }

    /// Release reset for APB_SARADC.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x10000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the ApbSarAdc DMA path.
    pub fn enable_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60040050u64, 0x80000000u32, 0x80000000u32)?;
        Ok(())
    }

    /// Disable the ApbSarAdc DMA path.
    pub fn disable_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60040050u64, 0x80000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn reset_dma_fsm(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60040050u64, 0x40000000u32, 0x40000000u32)?;
        modify_u32(0x60040050u64, 0x40000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the ApbSarAdc conversion-done interrupt.
    pub fn enable_done_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60040040u64, 0x80000000u32, 0x80000000u32)?;
        Ok(())
    }

    /// Disable the ApbSarAdc conversion-done interrupt.
    pub fn disable_done_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60040040u64, 0x80000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_configure_onetime_ch0(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60040020u64, 0x01800000u32, 0x00000000u32)?;
        modify_u32(0x60040020u64, 0x1E000000u32, 0x00000000u32)?;
        modify_u32(0x60040020u64, 0x20000000u32, 0x20000000u32)?;
        Ok(())
    }
}
