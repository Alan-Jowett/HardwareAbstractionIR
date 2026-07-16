//! Generated Embassy-style usart module for STM32F405RGT6.

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
    module_name: "usart",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: Usart1 (usart) from canonical block block.usart1 -> usart
pub const DRV_USART1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.usart1",
    name: "USART1 clock",
    consumer_ref: "periph.usart1",
    clock_ref: "clk.pclk2",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.apb2enr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_USART1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.usart1",
    name: "USART1 reset",
    target_ref: "periph.usart1",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rdom.apb2"),
    binding_kind: "software",
    control_refs: &["reg.rcc.apb2rstr"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_USART1_INTERRUPT_SOURCES: &[metadata::InterruptSource] =
    &[metadata::InterruptSource {
        id: "isrc.usart1.global",
        name: "USART1 GLOBAL source",
        source_ref: "periph.usart1",
        producer_ref: Some("block.usart1"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    }];
pub const DRV_USART1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.usart1.global",
    name: "USART1 GLOBAL route",
    source_ref: "isrc.usart1.global",
    interrupt_ref: "irq.usart1",
    controller_ref: "block.nvic",
    cpu_target_ref: None,
    line_index: Some(37),
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_USART1_DMA_CHANNELS: &[metadata::DmaChannel] = &[
    metadata::DmaChannel {
        id: "dma.dma2_ch2",
        name: "DMA2 CH2",
        controller_ref: "block.dma2",
        target_ref: None,
        channel_index: 2,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dma.dma2_ch5",
        name: "DMA2 CH5",
        controller_ref: "block.dma2",
        target_ref: None,
        channel_index: 5,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dma.dma2_ch7",
        name: "DMA2 CH7",
        controller_ref: "block.dma2",
        target_ref: None,
        channel_index: 7,
        capabilities: &[],
        priority_levels: &[],
    },
];
pub const DRV_USART1_DMA_ROUTES: &[metadata::DmaRoute] = &[
    metadata::DmaRoute {
        id: "dmaroute.usart1.rx.dma2_ch2.4",
        name: "USART1 RX via DMA2_CH2",
        peripheral_ref: "periph.usart1",
        signal: Some("RX"),
        channel_ref: "dma.dma2_ch2",
        direction: "peripheral-to-memory",
        control_refs: &["reg.dma2.s2cr"],
        shared_channel_group_ref: Some("dmagroup.dma_dma2_ch2"),
    },
    metadata::DmaRoute {
        id: "dmaroute.usart1.rx.dma2_ch5.4",
        name: "USART1 RX via DMA2_CH5",
        peripheral_ref: "periph.usart1",
        signal: Some("RX"),
        channel_ref: "dma.dma2_ch5",
        direction: "peripheral-to-memory",
        control_refs: &["reg.dma2.s5cr"],
        shared_channel_group_ref: Some("dmagroup.dma_dma2_ch5"),
    },
    metadata::DmaRoute {
        id: "dmaroute.usart1.tx.dma2_ch7.4",
        name: "USART1 TX via DMA2_CH7",
        peripheral_ref: "periph.usart1",
        signal: Some("TX"),
        channel_ref: "dma.dma2_ch7",
        direction: "memory-to-peripheral",
        control_refs: &["reg.dma2.s7cr"],
        shared_channel_group_ref: Some("dmagroup.dma_dma2_ch7"),
    },
];
pub const DRV_USART1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[
    metadata::PinRoute {
        id: "pinroute.usart1.tx.pa9",
        name: "USART1 TX on PA9",
        pin_ref: "pin.pa9",
        peripheral_ref: "periph.usart1",
        signal: "TX",
        route_type: "muxed",
        control_refs: &["reg.gpioa.moder", "reg.gpioa.afrh"],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
    metadata::PinRoute {
        id: "pinroute.usart1.tx.pb6",
        name: "USART1 TX on PB6",
        pin_ref: "pin.pb6",
        peripheral_ref: "periph.usart1",
        signal: "TX",
        route_type: "muxed",
        control_refs: &["reg.gpiob.moder", "reg.gpiob.afrl"],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
];
pub const DRV_USART1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[
    metadata::PinRoute {
        id: "pinroute.usart1.rx.pa10",
        name: "USART1 RX on PA10",
        pin_ref: "pin.pa10",
        peripheral_ref: "periph.usart1",
        signal: "RX",
        route_type: "muxed",
        control_refs: &["reg.gpioa.moder", "reg.gpioa.afrh"],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
    metadata::PinRoute {
        id: "pinroute.usart1.rx.pb7",
        name: "USART1 RX on PB7",
        pin_ref: "pin.pb7",
        peripheral_ref: "periph.usart1",
        signal: "RX",
        route_type: "muxed",
        control_refs: &["reg.gpiob.moder", "reg.gpiob.afrl"],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
];
pub const DRV_USART1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart1.ck.pa8",
    name: "USART1 CK on PA8",
    pin_ref: "pin.pa8",
    peripheral_ref: "periph.usart1",
    signal: "CK",
    route_type: "muxed",
    control_refs: &["reg.gpioa.moder", "reg.gpioa.afrh"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART1_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart1.cts.pa11",
    name: "USART1 CTS on PA11",
    pin_ref: "pin.pa11",
    peripheral_ref: "periph.usart1",
    signal: "CTS",
    route_type: "muxed",
    control_refs: &["reg.gpioa.moder", "reg.gpioa.afrh"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART1_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart1.rts.pa12",
    name: "USART1 RTS on PA12",
    pin_ref: "pin.pa12",
    peripheral_ref: "periph.usart1",
    signal: "RTS",
    route_type: "muxed",
    control_refs: &["reg.gpioa.moder", "reg.gpioa.afrh"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART1_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "tx",
        signal: "TX",
        routes: DRV_USART1_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "rx",
        signal: "RX",
        routes: DRV_USART1_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "ck",
        signal: "CK",
        routes: DRV_USART1_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "cts",
        signal: "CTS",
        routes: DRV_USART1_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "rts",
        signal: "RTS",
        routes: DRV_USART1_PIN_ROLE_4_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_USART1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_USART1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USART1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Usart1RuntimeResources {}

pub const DRV_USART1_RUNTIME_RESOURCES: Usart1RuntimeResources = Usart1RuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct Usart1MetadataResources {
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

pub const DRV_USART1_METADATA_RESOURCES: Usart1MetadataResources = Usart1MetadataResources {
    clocks: DRV_USART1_CLOCK_BINDINGS,
    resets: DRV_USART1_RESET_BINDINGS,
    interrupt_sources: DRV_USART1_INTERRUPT_SOURCES,
    interrupts: DRV_USART1_INTERRUPT_ROUTES,
    dma_channels: DRV_USART1_DMA_CHANNELS,
    dma: DRV_USART1_DMA_ROUTES,
    pins: DRV_USART1_PIN_ROLES,
    init_operations: DRV_USART1_INIT_OPERATIONS,
    state_machines: DRV_USART1_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_USART1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Usart1;

impl Usart1 {
    pub fn new(resources: Usart1RuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> Usart1MetadataResources {
        DRV_USART1_METADATA_RESOURCES
    }
    /// Enable the USART1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023844u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Disable the USART1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023844u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023824u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Release reset for USART1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023824u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Configure the Usart1 TX route on PA9.
    pub fn configure_tx_pa9_route(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40020000u64, 0x000C0000u32, 0x00080000u32)?;
        modify_u32(0x40020024u64, 0x000000F0u32, 0x00000070u32)?;
        Ok(())
    }

    /// Configure the Usart1 TX route on PB6.
    pub fn configure_tx_pb6_route(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40020400u64, 0x00003000u32, 0x00002000u32)?;
        modify_u32(0x40020420u64, 0x0F000000u32, 0x07000000u32)?;
        Ok(())
    }

    /// Configure the Usart1 RX route on PA10.
    pub fn configure_rx_pa10_route(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40020000u64, 0x00300000u32, 0x00200000u32)?;
        modify_u32(0x40020024u64, 0x00000F00u32, 0x00000700u32)?;
        Ok(())
    }

    /// Configure the Usart1 RX route on PB7.
    pub fn configure_rx_pb7_route(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40020400u64, 0x0000C000u32, 0x00008000u32)?;
        modify_u32(0x40020420u64, 0xF0000000u32, 0x70000000u32)?;
        Ok(())
    }

    /// Configure the Usart1 CK route on PA8.
    pub fn configure_ck_pa8_route(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40020000u64, 0x00030000u32, 0x00020000u32)?;
        modify_u32(0x40020024u64, 0x0000000Fu32, 0x00000007u32)?;
        Ok(())
    }

    /// Configure the Usart1 CTS route on PA11.
    pub fn configure_cts_pa11_route(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40020000u64, 0x00C00000u32, 0x00800000u32)?;
        modify_u32(0x40020024u64, 0x0000F000u32, 0x00007000u32)?;
        Ok(())
    }

    /// Configure the Usart1 RTS route on PA12.
    pub fn configure_rts_pa12_route(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40020000u64, 0x03000000u32, 0x02000000u32)?;
        modify_u32(0x40020024u64, 0x000F0000u32, 0x00070000u32)?;
        Ok(())
    }

    /// Enable Usart1.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00002000u32, 0x00002000u32)?;
        Ok(())
    }

    /// Disable Usart1.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00002000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart1 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the Usart1 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart1 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the Usart1 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00002000u32, 0x00000000u32)?;
        modify_u32(0x4001100Cu64, 0x00000008u32, 0x00000000u32)?;
        modify_u32(0x4001100Cu64, 0x00000004u32, 0x00000000u32)?;
        modify_u32(0x4001100Cu64, 0x00008000u32, 0x00000000u32)?;
        modify_u32(0x4001100Cu64, 0x00001000u32, 0x00000000u32)?;
        modify_u32(0x4001100Cu64, 0x00000400u32, 0x00000000u32)?;
        modify_u32(0x4001100Cu64, 0x00000200u32, 0x00000000u32)?;
        modify_u32(0x40011010u64, 0x00003000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud mantissa exceeds modeled field width",
            ));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud fraction exceeds modeled field width",
            ));
        }
        modify_u32(
            0x40011008u64,
            0x0000FFFFu32,
            (u32::from(mantissa) & 0xFFFu32) << 4 | u32::from(fraction) & 0xFu32,
        )?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while (read_u32(0x40011000u64)? & 0x00000080u32) == 0 {}
        write_u32(0x40011004u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while (read_u32(0x40011000u64)? & 0x00000040u32) == 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x40011000u64)? & 0x00000020u32) == 0 {}
        Ok((read_u32(0x40011004u64)? & 0xFFu32) as u8)
    }

    /// Enable the Usart1 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the Usart1 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart1 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the Usart1 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001100Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart1 TX DMA path.
    pub fn enable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40011014u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the Usart1 TX DMA path.
    pub fn disable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40011014u64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart1 RX DMA path.
    pub fn enable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40011014u64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    /// Disable the Usart1 RX DMA path.
    pub fn disable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40011014u64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }
}
// Driver instance: Usart2 (usart) from canonical block block.usart2 -> usart
pub const DRV_USART2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.usart2",
    name: "USART2 clock",
    consumer_ref: "periph.usart2",
    clock_ref: "clk.pclk1",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.apb1enr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_USART2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.usart2",
    name: "USART2 reset",
    target_ref: "periph.usart2",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rdom.apb1"),
    binding_kind: "software",
    control_refs: &["reg.rcc.apb1rstr"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_USART2_INTERRUPT_SOURCES: &[metadata::InterruptSource] =
    &[metadata::InterruptSource {
        id: "isrc.usart2.global",
        name: "USART2 GLOBAL source",
        source_ref: "periph.usart2",
        producer_ref: Some("block.usart2"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    }];
pub const DRV_USART2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.usart2.global",
    name: "USART2 GLOBAL route",
    source_ref: "isrc.usart2.global",
    interrupt_ref: "irq.usart2",
    controller_ref: "block.nvic",
    cpu_target_ref: None,
    line_index: Some(38),
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_USART2_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_USART2_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_USART2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart2.tx.pa2",
    name: "USART2 TX on PA2",
    pin_ref: "pin.pa2",
    peripheral_ref: "periph.usart2",
    signal: "TX",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART2_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart2.rx.pa3",
    name: "USART2 RX on PA3",
    pin_ref: "pin.pa3",
    peripheral_ref: "periph.usart2",
    signal: "RX",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART2_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart2.ck.pa4",
    name: "USART2 CK on PA4",
    pin_ref: "pin.pa4",
    peripheral_ref: "periph.usart2",
    signal: "CK",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART2_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart2.cts.pa0",
    name: "USART2 CTS on PA0",
    pin_ref: "pin.pa0",
    peripheral_ref: "periph.usart2",
    signal: "CTS",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART2_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart2.rts.pa1",
    name: "USART2 RTS on PA1",
    pin_ref: "pin.pa1",
    peripheral_ref: "periph.usart2",
    signal: "RTS",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART2_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "tx",
        signal: "TX",
        routes: DRV_USART2_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "rx",
        signal: "RX",
        routes: DRV_USART2_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "ck",
        signal: "CK",
        routes: DRV_USART2_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "cts",
        signal: "CTS",
        routes: DRV_USART2_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "rts",
        signal: "RTS",
        routes: DRV_USART2_PIN_ROLE_4_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_USART2_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_USART2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USART2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Usart2RuntimeResources {}

pub const DRV_USART2_RUNTIME_RESOURCES: Usart2RuntimeResources = Usart2RuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct Usart2MetadataResources {
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

pub const DRV_USART2_METADATA_RESOURCES: Usart2MetadataResources = Usart2MetadataResources {
    clocks: DRV_USART2_CLOCK_BINDINGS,
    resets: DRV_USART2_RESET_BINDINGS,
    interrupt_sources: DRV_USART2_INTERRUPT_SOURCES,
    interrupts: DRV_USART2_INTERRUPT_ROUTES,
    dma_channels: DRV_USART2_DMA_CHANNELS,
    dma: DRV_USART2_DMA_ROUTES,
    pins: DRV_USART2_PIN_ROLES,
    init_operations: DRV_USART2_INIT_OPERATIONS,
    state_machines: DRV_USART2_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_USART2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Usart2;

impl Usart2 {
    pub fn new(resources: Usart2RuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> Usart2MetadataResources {
        DRV_USART2_METADATA_RESOURCES
    }
    /// Enable the USART2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Disable the USART2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART2.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Release reset for USART2.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable Usart2.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00002000u32, 0x00002000u32)?;
        Ok(())
    }

    /// Disable Usart2.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00002000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart2 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the Usart2 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart2 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the Usart2 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00002000u32, 0x00000000u32)?;
        modify_u32(0x4000440Cu64, 0x00000008u32, 0x00000000u32)?;
        modify_u32(0x4000440Cu64, 0x00000004u32, 0x00000000u32)?;
        modify_u32(0x4000440Cu64, 0x00008000u32, 0x00000000u32)?;
        modify_u32(0x4000440Cu64, 0x00001000u32, 0x00000000u32)?;
        modify_u32(0x4000440Cu64, 0x00000400u32, 0x00000000u32)?;
        modify_u32(0x4000440Cu64, 0x00000200u32, 0x00000000u32)?;
        modify_u32(0x40004410u64, 0x00003000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud mantissa exceeds modeled field width",
            ));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud fraction exceeds modeled field width",
            ));
        }
        modify_u32(
            0x40004408u64,
            0x0000FFFFu32,
            (u32::from(mantissa) & 0xFFFu32) << 4 | u32::from(fraction) & 0xFu32,
        )?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while (read_u32(0x40004400u64)? & 0x00000080u32) == 0 {}
        write_u32(0x40004404u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while (read_u32(0x40004400u64)? & 0x00000040u32) == 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x40004400u64)? & 0x00000020u32) == 0 {}
        Ok((read_u32(0x40004404u64)? & 0xFFu32) as u8)
    }

    /// Enable the Usart2 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the Usart2 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart2 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the Usart2 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000440Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }
}
// Driver instance: Usart3 (usart) from canonical block block.usart3 -> usart
pub const DRV_USART3_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.usart3",
    name: "USART3 clock",
    consumer_ref: "periph.usart3",
    clock_ref: "clk.pclk1",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.apb1enr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_USART3_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.usart3",
    name: "USART3 reset",
    target_ref: "periph.usart3",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rdom.apb1"),
    binding_kind: "software",
    control_refs: &["reg.rcc.apb1rstr"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_USART3_INTERRUPT_SOURCES: &[metadata::InterruptSource] =
    &[metadata::InterruptSource {
        id: "isrc.usart3.global",
        name: "USART3 GLOBAL source",
        source_ref: "periph.usart3",
        producer_ref: Some("block.usart3"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    }];
pub const DRV_USART3_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.usart3.global",
    name: "USART3 GLOBAL route",
    source_ref: "isrc.usart3.global",
    interrupt_ref: "irq.usart3",
    controller_ref: "block.nvic",
    cpu_target_ref: None,
    line_index: Some(39),
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_USART3_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_USART3_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_USART3_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[
    metadata::PinRoute {
        id: "pinroute.usart3.tx.pb10",
        name: "USART3 TX on PB10",
        pin_ref: "pin.pb10",
        peripheral_ref: "periph.usart3",
        signal: "TX",
        route_type: "muxed",
        control_refs: &[],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
    metadata::PinRoute {
        id: "pinroute.usart3.tx.pc10",
        name: "USART3 TX on PC10",
        pin_ref: "pin.pc10",
        peripheral_ref: "periph.usart3",
        signal: "TX",
        route_type: "muxed",
        control_refs: &[],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
];
pub const DRV_USART3_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[
    metadata::PinRoute {
        id: "pinroute.usart3.rx.pb11",
        name: "USART3 RX on PB11",
        pin_ref: "pin.pb11",
        peripheral_ref: "periph.usart3",
        signal: "RX",
        route_type: "muxed",
        control_refs: &[],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
    metadata::PinRoute {
        id: "pinroute.usart3.rx.pc11",
        name: "USART3 RX on PC11",
        pin_ref: "pin.pc11",
        peripheral_ref: "periph.usart3",
        signal: "RX",
        route_type: "muxed",
        control_refs: &[],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
];
pub const DRV_USART3_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[
    metadata::PinRoute {
        id: "pinroute.usart3.ck.pb12",
        name: "USART3 CK on PB12",
        pin_ref: "pin.pb12",
        peripheral_ref: "periph.usart3",
        signal: "CK",
        route_type: "muxed",
        control_refs: &[],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
    metadata::PinRoute {
        id: "pinroute.usart3.ck.pc12",
        name: "USART3 CK on PC12",
        pin_ref: "pin.pc12",
        peripheral_ref: "periph.usart3",
        signal: "CK",
        route_type: "muxed",
        control_refs: &[],
        electrical_constraint_refs: &[],
        conflict_refs: &[],
        default_after_reset: None,
    },
];
pub const DRV_USART3_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart3.cts.pb13",
    name: "USART3 CTS on PB13",
    pin_ref: "pin.pb13",
    peripheral_ref: "periph.usart3",
    signal: "CTS",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART3_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart3.rts.pb14",
    name: "USART3 RTS on PB14",
    pin_ref: "pin.pb14",
    peripheral_ref: "periph.usart3",
    signal: "RTS",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART3_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "tx",
        signal: "TX",
        routes: DRV_USART3_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "rx",
        signal: "RX",
        routes: DRV_USART3_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "ck",
        signal: "CK",
        routes: DRV_USART3_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "cts",
        signal: "CTS",
        routes: DRV_USART3_PIN_ROLE_3_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
    metadata::PinRole {
        role: "rts",
        signal: "RTS",
        routes: DRV_USART3_PIN_ROLE_4_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_USART3_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_USART3_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USART3_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Usart3RuntimeResources {}

pub const DRV_USART3_RUNTIME_RESOURCES: Usart3RuntimeResources = Usart3RuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct Usart3MetadataResources {
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

pub const DRV_USART3_METADATA_RESOURCES: Usart3MetadataResources = Usart3MetadataResources {
    clocks: DRV_USART3_CLOCK_BINDINGS,
    resets: DRV_USART3_RESET_BINDINGS,
    interrupt_sources: DRV_USART3_INTERRUPT_SOURCES,
    interrupts: DRV_USART3_INTERRUPT_ROUTES,
    dma_channels: DRV_USART3_DMA_CHANNELS,
    dma: DRV_USART3_DMA_ROUTES,
    pins: DRV_USART3_PIN_ROLES,
    init_operations: DRV_USART3_INIT_OPERATIONS,
    state_machines: DRV_USART3_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_USART3_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Usart3;

impl Usart3 {
    pub fn new(resources: Usart3RuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> Usart3MetadataResources {
        DRV_USART3_METADATA_RESOURCES
    }
    /// Enable the USART3 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00040000u32, 0x00040000u32)?;
        Ok(())
    }

    /// Disable the USART3 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00040000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART3.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00040000u32, 0x00040000u32)?;
        Ok(())
    }

    /// Release reset for USART3.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00040000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable Usart3.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00002000u32, 0x00002000u32)?;
        Ok(())
    }

    /// Disable Usart3.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00002000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart3 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the Usart3 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart3 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the Usart3 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00002000u32, 0x00000000u32)?;
        modify_u32(0x4000480Cu64, 0x00000008u32, 0x00000000u32)?;
        modify_u32(0x4000480Cu64, 0x00000004u32, 0x00000000u32)?;
        modify_u32(0x4000480Cu64, 0x00008000u32, 0x00000000u32)?;
        modify_u32(0x4000480Cu64, 0x00001000u32, 0x00000000u32)?;
        modify_u32(0x4000480Cu64, 0x00000400u32, 0x00000000u32)?;
        modify_u32(0x4000480Cu64, 0x00000200u32, 0x00000000u32)?;
        modify_u32(0x40004810u64, 0x00003000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud mantissa exceeds modeled field width",
            ));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud fraction exceeds modeled field width",
            ));
        }
        modify_u32(
            0x40004808u64,
            0x0000FFFFu32,
            (u32::from(mantissa) & 0xFFFu32) << 4 | u32::from(fraction) & 0xFu32,
        )?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while (read_u32(0x40004800u64)? & 0x00000080u32) == 0 {}
        write_u32(0x40004804u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while (read_u32(0x40004800u64)? & 0x00000040u32) == 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x40004800u64)? & 0x00000020u32) == 0 {}
        Ok((read_u32(0x40004804u64)? & 0xFFu32) as u8)
    }

    /// Enable the Usart3 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the Usart3 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart3 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the Usart3 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000480Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }
}
// Driver instance: Usart6 (usart) from canonical block block.usart6 -> usart
pub const DRV_USART6_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.usart6",
    name: "USART6 clock",
    consumer_ref: "periph.usart6",
    clock_ref: "clk.pclk2",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.apb2enr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_USART6_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.usart6",
    name: "USART6 reset",
    target_ref: "periph.usart6",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rdom.apb2"),
    binding_kind: "software",
    control_refs: &["reg.rcc.apb2rstr"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_USART6_INTERRUPT_SOURCES: &[metadata::InterruptSource] =
    &[metadata::InterruptSource {
        id: "isrc.usart6.global",
        name: "USART6 GLOBAL source",
        source_ref: "periph.usart6",
        producer_ref: Some("block.usart6"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    }];
pub const DRV_USART6_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.usart6.global",
    name: "USART6 GLOBAL route",
    source_ref: "isrc.usart6.global",
    interrupt_ref: "irq.usart6",
    controller_ref: "block.nvic",
    cpu_target_ref: None,
    line_index: Some(71),
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_USART6_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_USART6_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_USART6_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart6.tx.pc6",
    name: "USART6 TX on PC6",
    pin_ref: "pin.pc6",
    peripheral_ref: "periph.usart6",
    signal: "TX",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART6_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart6.rx.pc7",
    name: "USART6 RX on PC7",
    pin_ref: "pin.pc7",
    peripheral_ref: "periph.usart6",
    signal: "RX",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART6_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.usart6.ck.pc8",
    name: "USART6 CK on PC8",
    pin_ref: "pin.pc8",
    peripheral_ref: "periph.usart6",
    signal: "CK",
    route_type: "muxed",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_USART6_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "tx",
        signal: "TX",
        routes: DRV_USART6_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "rx",
        signal: "RX",
        routes: DRV_USART6_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "ck",
        signal: "CK",
        routes: DRV_USART6_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_USART6_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_USART6_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USART6_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Usart6RuntimeResources {}

pub const DRV_USART6_RUNTIME_RESOURCES: Usart6RuntimeResources = Usart6RuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct Usart6MetadataResources {
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

pub const DRV_USART6_METADATA_RESOURCES: Usart6MetadataResources = Usart6MetadataResources {
    clocks: DRV_USART6_CLOCK_BINDINGS,
    resets: DRV_USART6_RESET_BINDINGS,
    interrupt_sources: DRV_USART6_INTERRUPT_SOURCES,
    interrupts: DRV_USART6_INTERRUPT_ROUTES,
    dma_channels: DRV_USART6_DMA_CHANNELS,
    dma: DRV_USART6_DMA_ROUTES,
    pins: DRV_USART6_PIN_ROLES,
    init_operations: DRV_USART6_INIT_OPERATIONS,
    state_machines: DRV_USART6_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_USART6_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Usart6;

impl Usart6 {
    pub fn new(resources: Usart6RuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> Usart6MetadataResources {
        DRV_USART6_METADATA_RESOURCES
    }
    /// Enable the USART6 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023844u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the USART6 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023844u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART6.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023824u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Release reset for USART6.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023824u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable Usart6.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00002000u32, 0x00002000u32)?;
        Ok(())
    }

    /// Disable Usart6.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00002000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart6 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the Usart6 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart6 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the Usart6 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00002000u32, 0x00000000u32)?;
        modify_u32(0x4001140Cu64, 0x00000008u32, 0x00000000u32)?;
        modify_u32(0x4001140Cu64, 0x00000004u32, 0x00000000u32)?;
        modify_u32(0x4001140Cu64, 0x00008000u32, 0x00000000u32)?;
        modify_u32(0x4001140Cu64, 0x00001000u32, 0x00000000u32)?;
        modify_u32(0x4001140Cu64, 0x00000400u32, 0x00000000u32)?;
        modify_u32(0x4001140Cu64, 0x00000200u32, 0x00000000u32)?;
        modify_u32(0x40011410u64, 0x00003000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud mantissa exceeds modeled field width",
            ));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported(
                "USART baud fraction exceeds modeled field width",
            ));
        }
        modify_u32(
            0x40011408u64,
            0x0000FFFFu32,
            (u32::from(mantissa) & 0xFFFu32) << 4 | u32::from(fraction) & 0xFu32,
        )?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while (read_u32(0x40011400u64)? & 0x00000080u32) == 0 {}
        write_u32(0x40011404u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while (read_u32(0x40011400u64)? & 0x00000040u32) == 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x40011400u64)? & 0x00000020u32) == 0 {}
        Ok((read_u32(0x40011404u64)? & 0xFFu32) as u8)
    }

    /// Enable the Usart6 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the Usart6 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Usart6 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the Usart6 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4001140Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }
}
