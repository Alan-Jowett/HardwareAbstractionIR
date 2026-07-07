//! Generated Embassy-style dma module for STM32F405RGT6.

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
    module_name: "dma",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: Dma1 (dma) from canonical block block.dma1 -> dma-controller
pub const DRV_DMA1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.dma1", name: "DMA1 clock", consumer_ref: "periph.dma1", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_DMA1_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_DMA1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_DMA1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_DMA1_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel { id: "dma.dma1_ch0", name: "DMA1 CH0", controller_ref: "block.dma1", target_ref: None, channel_index: 0, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma1_ch5", name: "DMA1 CH5", controller_ref: "block.dma1", target_ref: None, channel_index: 5, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma1_ch6", name: "DMA1 CH6", controller_ref: "block.dma1", target_ref: None, channel_index: 6, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma1_ch7", name: "DMA1 CH7", controller_ref: "block.dma1", target_ref: None, channel_index: 7, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma1_ch2", name: "DMA1 CH2", controller_ref: "block.dma1", target_ref: None, channel_index: 2, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma1_ch3", name: "DMA1 CH3", controller_ref: "block.dma1", target_ref: None, channel_index: 3, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma1_ch4", name: "DMA1 CH4", controller_ref: "block.dma1", target_ref: None, channel_index: 4, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma1_ch1", name: "DMA1 CH1", controller_ref: "block.dma1", target_ref: None, channel_index: 1, capabilities: &[], priority_levels: &[] }];
pub const DRV_DMA1_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute { id: "dmaroute.i2c1.rx.dma1_ch0.1", name: "I2C1 RX via DMA1_CH0", peripheral_ref: "periph.i2c1", signal: Some("RX"), channel_ref: "dma.dma1_ch0", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch0") }, metadata::DmaRoute { id: "dmaroute.i2c1.rx.dma1_ch5.1", name: "I2C1 RX via DMA1_CH5", peripheral_ref: "periph.i2c1", signal: Some("RX"), channel_ref: "dma.dma1_ch5", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch5") }, metadata::DmaRoute { id: "dmaroute.i2c1.tx.dma1_ch6.1", name: "I2C1 TX via DMA1_CH6", peripheral_ref: "periph.i2c1", signal: Some("TX"), channel_ref: "dma.dma1_ch6", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch6") }, metadata::DmaRoute { id: "dmaroute.i2c1.tx.dma1_ch7.1", name: "I2C1 TX via DMA1_CH7", peripheral_ref: "periph.i2c1", signal: Some("TX"), channel_ref: "dma.dma1_ch7", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch7") }, metadata::DmaRoute { id: "dmaroute.i2c2.rx.dma1_ch2.7", name: "I2C2 RX via DMA1_CH2", peripheral_ref: "periph.i2c2", signal: Some("RX"), channel_ref: "dma.dma1_ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch2") }, metadata::DmaRoute { id: "dmaroute.i2c2.rx.dma1_ch3.7", name: "I2C2 RX via DMA1_CH3", peripheral_ref: "periph.i2c2", signal: Some("RX"), channel_ref: "dma.dma1_ch3", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch3") }, metadata::DmaRoute { id: "dmaroute.i2c2.tx.dma1_ch7.7", name: "I2C2 TX via DMA1_CH7", peripheral_ref: "periph.i2c2", signal: Some("TX"), channel_ref: "dma.dma1_ch7", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch7") }, metadata::DmaRoute { id: "dmaroute.i2c3.rx.dma1_ch2.3", name: "I2C3 RX via DMA1_CH2", peripheral_ref: "periph.i2c3", signal: Some("RX"), channel_ref: "dma.dma1_ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch2") }, metadata::DmaRoute { id: "dmaroute.i2c3.tx.dma1_ch4.3", name: "I2C3 TX via DMA1_CH4", peripheral_ref: "periph.i2c3", signal: Some("TX"), channel_ref: "dma.dma1_ch4", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch4") }, metadata::DmaRoute { id: "dmaroute.spi2.rx.dma1_ch3.0", name: "SPI2 RX via DMA1_CH3", peripheral_ref: "periph.spi2", signal: Some("RX"), channel_ref: "dma.dma1_ch3", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch3") }, metadata::DmaRoute { id: "dmaroute.spi2.tx.dma1_ch4.0", name: "SPI2 TX via DMA1_CH4", peripheral_ref: "periph.spi2", signal: Some("TX"), channel_ref: "dma.dma1_ch4", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch4") }, metadata::DmaRoute { id: "dmaroute.spi3.rx.dma1_ch0.0", name: "SPI3 RX via DMA1_CH0", peripheral_ref: "periph.spi3", signal: Some("RX"), channel_ref: "dma.dma1_ch0", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch0") }, metadata::DmaRoute { id: "dmaroute.spi3.rx.dma1_ch2.0", name: "SPI3 RX via DMA1_CH2", peripheral_ref: "periph.spi3", signal: Some("RX"), channel_ref: "dma.dma1_ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch2") }, metadata::DmaRoute { id: "dmaroute.spi3.tx.dma1_ch5.0", name: "SPI3 TX via DMA1_CH5", peripheral_ref: "periph.spi3", signal: Some("TX"), channel_ref: "dma.dma1_ch5", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch5") }, metadata::DmaRoute { id: "dmaroute.spi3.tx.dma1_ch7.0", name: "SPI3 TX via DMA1_CH7", peripheral_ref: "periph.spi3", signal: Some("TX"), channel_ref: "dma.dma1_ch7", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch7") }, metadata::DmaRoute { id: "dmaroute.uart4.rx.dma1_ch2.4", name: "UART4 RX via DMA1_CH2", peripheral_ref: "periph.uart4", signal: Some("RX"), channel_ref: "dma.dma1_ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch2") }, metadata::DmaRoute { id: "dmaroute.uart4.tx.dma1_ch4.4", name: "UART4 TX via DMA1_CH4", peripheral_ref: "periph.uart4", signal: Some("TX"), channel_ref: "dma.dma1_ch4", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch4") }, metadata::DmaRoute { id: "dmaroute.uart5.rx.dma1_ch0.4", name: "UART5 RX via DMA1_CH0", peripheral_ref: "periph.uart5", signal: Some("RX"), channel_ref: "dma.dma1_ch0", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch0") }, metadata::DmaRoute { id: "dmaroute.uart5.tx.dma1_ch7.4", name: "UART5 TX via DMA1_CH7", peripheral_ref: "periph.uart5", signal: Some("TX"), channel_ref: "dma.dma1_ch7", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch7") }, metadata::DmaRoute { id: "dmaroute.usart2.rx.dma1_ch5.4", name: "USART2 RX via DMA1_CH5", peripheral_ref: "periph.usart2", signal: Some("RX"), channel_ref: "dma.dma1_ch5", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch5") }, metadata::DmaRoute { id: "dmaroute.usart2.tx.dma1_ch6.4", name: "USART2 TX via DMA1_CH6", peripheral_ref: "periph.usart2", signal: Some("TX"), channel_ref: "dma.dma1_ch6", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch6") }, metadata::DmaRoute { id: "dmaroute.usart3.rx.dma1_ch1.4", name: "USART3 RX via DMA1_CH1", peripheral_ref: "periph.usart3", signal: Some("RX"), channel_ref: "dma.dma1_ch1", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.usart3.tx.dma1_ch3.4", name: "USART3 TX via DMA1_CH3", peripheral_ref: "periph.usart3", signal: Some("TX"), channel_ref: "dma.dma1_ch3", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch3") }, metadata::DmaRoute { id: "dmaroute.usart3.tx.dma1_ch4.7", name: "USART3 TX via DMA1_CH4", peripheral_ref: "periph.usart3", signal: Some("TX"), channel_ref: "dma.dma1_ch4", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma1_ch4") }];
pub const DRV_DMA1_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_DMA1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_DMA1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_DMA1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Dma1Resources {
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

pub const DRV_DMA1_RESOURCES: Dma1Resources = Dma1Resources {
    clocks: DRV_DMA1_CLOCK_BINDINGS,
    resets: DRV_DMA1_RESET_BINDINGS,
    interrupt_sources: DRV_DMA1_INTERRUPT_SOURCES,
    interrupts: DRV_DMA1_INTERRUPT_ROUTES,
    dma_channels: DRV_DMA1_DMA_CHANNELS,
    dma: DRV_DMA1_DMA_ROUTES,
    pins: DRV_DMA1_PIN_ROLES,
    init_operations: DRV_DMA1_INIT_OPERATIONS,
    state_machines: DRV_DMA1_STATE_MACHINES,
    capability_tags: DRV_DMA1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Dma1 {
    resources: Dma1Resources,
}

impl Dma1 {
    pub fn new(resources: Dma1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Dma1Resources {
        self.resources
    }
    /// Enable the DMA1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00200000u32, 0x00200000u32)?;
        Ok(())
    }

    /// Disable the DMA1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00200000u32, 0x00000000u32)?;
        Ok(())
    }


}

// Driver instance: Dma2 (dma) from canonical block block.dma2 -> dma-controller
pub const DRV_DMA2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.dma2", name: "DMA2 clock", consumer_ref: "periph.dma2", clock_ref: "clk.hclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_DMA2_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_DMA2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_DMA2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_DMA2_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel { id: "dma.dma2_ch3", name: "DMA2 CH3", controller_ref: "block.dma2", target_ref: None, channel_index: 3, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma2_ch6", name: "DMA2 CH6", controller_ref: "block.dma2", target_ref: None, channel_index: 6, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma2_ch0", name: "DMA2 CH0", controller_ref: "block.dma2", target_ref: None, channel_index: 0, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma2_ch2", name: "DMA2 CH2", controller_ref: "block.dma2", target_ref: None, channel_index: 2, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma2_ch5", name: "DMA2 CH5", controller_ref: "block.dma2", target_ref: None, channel_index: 5, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma2_ch7", name: "DMA2 CH7", controller_ref: "block.dma2", target_ref: None, channel_index: 7, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dma.dma2_ch1", name: "DMA2 CH1", controller_ref: "block.dma2", target_ref: None, channel_index: 1, capabilities: &[], priority_levels: &[] }];
pub const DRV_DMA2_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute { id: "dmaroute.sdio.rx.dma2_ch3.4", name: "SDIO RX via DMA2_CH3", peripheral_ref: "periph.sdio", signal: Some("RX"), channel_ref: "dma.dma2_ch3", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch3") }, metadata::DmaRoute { id: "dmaroute.sdio.tx.dma2_ch3.4", name: "SDIO TX via DMA2_CH3", peripheral_ref: "periph.sdio", signal: Some("TX"), channel_ref: "dma.dma2_ch3", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch3") }, metadata::DmaRoute { id: "dmaroute.sdio.rx.dma2_ch6.4", name: "SDIO RX via DMA2_CH6", peripheral_ref: "periph.sdio", signal: Some("RX"), channel_ref: "dma.dma2_ch6", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch6") }, metadata::DmaRoute { id: "dmaroute.sdio.tx.dma2_ch6.4", name: "SDIO TX via DMA2_CH6", peripheral_ref: "periph.sdio", signal: Some("TX"), channel_ref: "dma.dma2_ch6", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch6") }, metadata::DmaRoute { id: "dmaroute.spi1.rx.dma2_ch0.3", name: "SPI1 RX via DMA2_CH0", peripheral_ref: "periph.spi1", signal: Some("RX"), channel_ref: "dma.dma2_ch0", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.spi1.rx.dma2_ch2.3", name: "SPI1 RX via DMA2_CH2", peripheral_ref: "periph.spi1", signal: Some("RX"), channel_ref: "dma.dma2_ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch2") }, metadata::DmaRoute { id: "dmaroute.spi1.tx.dma2_ch3.3", name: "SPI1 TX via DMA2_CH3", peripheral_ref: "periph.spi1", signal: Some("TX"), channel_ref: "dma.dma2_ch3", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch3") }, metadata::DmaRoute { id: "dmaroute.spi1.tx.dma2_ch5.3", name: "SPI1 TX via DMA2_CH5", peripheral_ref: "periph.spi1", signal: Some("TX"), channel_ref: "dma.dma2_ch5", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch5") }, metadata::DmaRoute { id: "dmaroute.usart1.rx.dma2_ch2.4", name: "USART1 RX via DMA2_CH2", peripheral_ref: "periph.usart1", signal: Some("RX"), channel_ref: "dma.dma2_ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch2") }, metadata::DmaRoute { id: "dmaroute.usart1.rx.dma2_ch5.4", name: "USART1 RX via DMA2_CH5", peripheral_ref: "periph.usart1", signal: Some("RX"), channel_ref: "dma.dma2_ch5", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch5") }, metadata::DmaRoute { id: "dmaroute.usart1.tx.dma2_ch7.4", name: "USART1 TX via DMA2_CH7", peripheral_ref: "periph.usart1", signal: Some("TX"), channel_ref: "dma.dma2_ch7", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch7") }, metadata::DmaRoute { id: "dmaroute.usart6.rx.dma2_ch1.5", name: "USART6 RX via DMA2_CH1", peripheral_ref: "periph.usart6", signal: Some("RX"), channel_ref: "dma.dma2_ch1", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.usart6.rx.dma2_ch2.5", name: "USART6 RX via DMA2_CH2", peripheral_ref: "periph.usart6", signal: Some("RX"), channel_ref: "dma.dma2_ch2", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch2") }, metadata::DmaRoute { id: "dmaroute.usart6.tx.dma2_ch6.5", name: "USART6 TX via DMA2_CH6", peripheral_ref: "periph.usart6", signal: Some("TX"), channel_ref: "dma.dma2_ch6", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch6") }, metadata::DmaRoute { id: "dmaroute.usart6.tx.dma2_ch7.5", name: "USART6 TX via DMA2_CH7", peripheral_ref: "periph.usart6", signal: Some("TX"), channel_ref: "dma.dma2_ch7", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: Some("dmagroup.dma_dma2_ch7") }];
pub const DRV_DMA2_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_DMA2_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_DMA2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_DMA2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Dma2Resources {
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

pub const DRV_DMA2_RESOURCES: Dma2Resources = Dma2Resources {
    clocks: DRV_DMA2_CLOCK_BINDINGS,
    resets: DRV_DMA2_RESET_BINDINGS,
    interrupt_sources: DRV_DMA2_INTERRUPT_SOURCES,
    interrupts: DRV_DMA2_INTERRUPT_ROUTES,
    dma_channels: DRV_DMA2_DMA_CHANNELS,
    dma: DRV_DMA2_DMA_ROUTES,
    pins: DRV_DMA2_PIN_ROLES,
    init_operations: DRV_DMA2_INIT_OPERATIONS,
    state_machines: DRV_DMA2_STATE_MACHINES,
    capability_tags: DRV_DMA2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Dma2 {
    resources: Dma2Resources,
}

impl Dma2 {
    pub fn new(resources: Dma2Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Dma2Resources {
        self.resources
    }
    /// Enable the DMA2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00400000u32, 0x00400000u32)?;
        Ok(())
    }

    /// Disable the DMA2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023830u64, 0x00400000u32, 0x00000000u32)?;
        Ok(())
    }


}

