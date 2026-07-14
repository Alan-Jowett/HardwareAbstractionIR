//! Generated Embassy-style rcc module for CH32V203G6U6.

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

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "rcc",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: RCC (rcc) from canonical block block.rcc -> clock-controller
pub const DRV_RCC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioa", name: "GPIOA clock binding", consumer_ref: "periph.gpioa", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpiob", name: "GPIOB clock binding", consumer_ref: "periph.gpiob", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpioc", name: "GPIOC clock binding", consumer_ref: "periph.gpioc", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpiod", name: "GPIOD clock binding", consumer_ref: "periph.gpiod", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.usart1", name: "USART1 clock binding", consumer_ref: "periph.usart1", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.usart2", name: "USART2 clock binding", consumer_ref: "periph.usart2", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.spi1", name: "SPI1 clock binding", consumer_ref: "periph.spi1", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.i2c1", name: "I2C1 clock binding", consumer_ref: "periph.i2c1", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.tim1", name: "TIM1 clock binding", consumer_ref: "periph.tim1", clock_ref: "clk.pclk2-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.tim2", name: "TIM2 clock binding", consumer_ref: "periph.tim2", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.tim3", name: "TIM3 clock binding", consumer_ref: "periph.tim3", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.tim4", name: "TIM4 clock binding", consumer_ref: "periph.tim4", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.adc1", name: "ADC1 clock binding", consumer_ref: "periph.adc1", clock_ref: "clk.adc", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.adc2", name: "ADC2 clock binding", consumer_ref: "periph.adc2", clock_ref: "clk.adc", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.dma1", name: "DMA1 clock binding", consumer_ref: "periph.dma1", clock_ref: "clk.hclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.ahbpcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_RCC_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioa", name: "GPIOA reset binding", target_ref: "periph.gpioa", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpiob", name: "GPIOB reset binding", target_ref: "periph.gpiob", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpioc", name: "GPIOC reset binding", target_ref: "periph.gpioc", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpiod", name: "GPIOD reset binding", target_ref: "periph.gpiod", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.usart1", name: "USART1 reset binding", target_ref: "periph.usart1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.usart2", name: "USART2 reset binding", target_ref: "periph.usart2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.spi1", name: "SPI1 reset binding", target_ref: "periph.spi1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.i2c1", name: "I2C1 reset binding", target_ref: "periph.i2c1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.tim1", name: "TIM1 reset binding", target_ref: "periph.tim1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.tim2", name: "TIM2 reset binding", target_ref: "periph.tim2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.tim3", name: "TIM3 reset binding", target_ref: "periph.tim3", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.tim4", name: "TIM4 reset binding", target_ref: "periph.tim4", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.adc1", name: "ADC1 reset binding", target_ref: "periph.adc1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.adc2", name: "ADC2 reset binding", target_ref: "periph.adc2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_RCC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_RCC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_RCC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_RCC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_RCC_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_RCC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_RCC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_RCC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct RCCResources {
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

pub const DRV_RCC_RESOURCES: RCCResources = RCCResources {
    clocks: DRV_RCC_CLOCK_BINDINGS,
    resets: DRV_RCC_RESET_BINDINGS,
    interrupt_sources: DRV_RCC_INTERRUPT_SOURCES,
    interrupts: DRV_RCC_INTERRUPT_ROUTES,
    dma_channels: DRV_RCC_DMA_CHANNELS,
    dma: DRV_RCC_DMA_ROUTES,
    pins: DRV_RCC_PIN_ROLES,
    init_operations: DRV_RCC_INIT_OPERATIONS,
    state_machines: DRV_RCC_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_RCC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct RCC {
    resources: RCCResources,
}

impl RCC {
    pub fn new(resources: RCCResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> RCCResources {
        self.resources
    }
    /// Enable the GPIOA clock gate.
    pub fn enable_gpioa_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the GPIOA clock gate.
    pub fn disable_gpioa_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOB clock gate.
    pub fn enable_gpiob_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the GPIOB clock gate.
    pub fn disable_gpiob_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOC clock gate.
    pub fn enable_gpioc_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Disable the GPIOC clock gate.
    pub fn disable_gpioc_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOD clock gate.
    pub fn enable_gpiod_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the GPIOD clock gate.
    pub fn disable_gpiod_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the USART1 clock gate.
    pub fn enable_usart1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00004000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Disable the USART1 clock gate.
    pub fn disable_usart1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00004000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the USART2 clock gate.
    pub fn enable_usart2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Disable the USART2 clock gate.
    pub fn disable_usart2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the SPI1 clock gate.
    pub fn enable_spi1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Disable the SPI1 clock gate.
    pub fn disable_spi1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the I2C1 clock gate.
    pub fn enable_i2c1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00200000u32, 0x00200000u32)?;
        Ok(())
    }

    /// Disable the I2C1 clock gate.
    pub fn disable_i2c1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00200000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIM1 clock gate.
    pub fn enable_tim1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000800u32, 0x00000800u32)?;
        Ok(())
    }

    /// Disable the TIM1 clock gate.
    pub fn disable_tim1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000800u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIM2 clock gate.
    pub fn enable_tim2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the TIM2 clock gate.
    pub fn disable_tim2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIM3 clock gate.
    pub fn enable_tim3_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the TIM3 clock gate.
    pub fn disable_tim3_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIM4 clock gate.
    pub fn enable_tim4_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the TIM4 clock gate.
    pub fn disable_tim4_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the ADC1 clock gate.
    pub fn enable_adc1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    /// Disable the ADC1 clock gate.
    pub fn disable_adc1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000200u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the ADC2 clock gate.
    pub fn enable_adc2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000400u32, 0x00000400u32)?;
        Ok(())
    }

    /// Disable the ADC2 clock gate.
    pub fn disable_adc2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00000400u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the DMA1 clock gate.
    pub fn enable_dma1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021014u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the DMA1 clock gate.
    pub fn disable_dma1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021014u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOA.
    pub fn assert_gpioa_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for GPIOA.
    pub fn release_gpioa_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOB.
    pub fn assert_gpiob_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Release reset for GPIOB.
    pub fn release_gpiob_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOC.
    pub fn assert_gpioc_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Release reset for GPIOC.
    pub fn release_gpioc_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOD.
    pub fn assert_gpiod_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Release reset for GPIOD.
    pub fn release_gpiod_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART1.
    pub fn assert_usart1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00004000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Release reset for USART1.
    pub fn release_usart1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00004000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART2.
    pub fn assert_usart2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Release reset for USART2.
    pub fn release_usart2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SPI1.
    pub fn assert_spi1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Release reset for SPI1.
    pub fn release_spi1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for I2C1.
    pub fn assert_i2c1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00200000u32, 0x00200000u32)?;
        Ok(())
    }

    /// Release reset for I2C1.
    pub fn release_i2c1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00200000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM1.
    pub fn assert_tim1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000800u32, 0x00000800u32)?;
        Ok(())
    }

    /// Release reset for TIM1.
    pub fn release_tim1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000800u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM2.
    pub fn assert_tim2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Release reset for TIM2.
    pub fn release_tim2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM3.
    pub fn assert_tim3_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Release reset for TIM3.
    pub fn release_tim3_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM4.
    pub fn assert_tim4_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for TIM4.
    pub fn release_tim4_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for ADC1.
    pub fn assert_adc1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    /// Release reset for ADC1.
    pub fn release_adc1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000200u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for ADC2.
    pub fn assert_adc2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000400u32, 0x00000400u32)?;
        Ok(())
    }

    /// Release reset for ADC2.
    pub fn release_adc2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00000400u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_usb_fsdev_clock_48mhz(&self) -> Result<(), metadata::Error> {
        const RCC_CTLR: u64 = 0x4002_1000;
        const RCC_CFGR0: u64 = 0x4002_1004;
        const RCC_APB1PCENR: u64 = 0x4002_101C;
        const EXTEN_CTR: u64 = 0x4002_3800;
        const RCC_PLLON: u32 = 1 << 24;
        const RCC_PLLRDY: u32 = 1 << 25;
        const RCC_HSEON: u32 = 1 << 16;
        const RCC_HSERDY: u32 = 1 << 17;
        const RCC_SW_MASK: u32 = 0b11;
        const RCC_SW_PLL: u32 = 0b10;
        const RCC_SWS_MASK: u32 = 0b11 << 2;
        const RCC_SWS_PLL: u32 = 0b10 << 2;
        const RCC_HPRE_MASK: u32 = 0b1111 << 4;
        const RCC_PPRE1_MASK: u32 = 0b111 << 8;
        const RCC_PPRE2_MASK: u32 = 0b111 << 11;
        const RCC_PLLSRC: u32 = 1 << 16;
        const RCC_PLLXTPRE: u32 = 1 << 17;
        const RCC_PLLMULL_MASK: u32 = 0b1111 << 18;
        const RCC_USBPRE_MASK: u32 = 0b11 << 22;
        const RCC_PLLMULL6: u32 = 4 << 18;
        const RCC_PPRE1_DIV2: u32 = 0b100 << 8;
        const RCC_APB1_USB_EN: u32 = 1 << 23;
        const EXTEN_PLL_HSI_PRE: u32 = 1 << 4;
        const HSE_TIMEOUT: u32 = 200_000;
        const PLL_TIMEOUT: u32 = 200_000;
        const SWITCH_TIMEOUT: u32 = 200_000;

        modify_u32(EXTEN_CTR, EXTEN_PLL_HSI_PRE, 0)?;
        let mut cfgr0 = read_u32(RCC_CFGR0)?;
        cfgr0 &= !(RCC_SW_MASK | RCC_HPRE_MASK | RCC_PPRE1_MASK | RCC_PPRE2_MASK | RCC_PLLSRC | RCC_PLLXTPRE | RCC_PLLMULL_MASK | RCC_USBPRE_MASK);
        cfgr0 |= RCC_PPRE1_DIV2 | RCC_PLLMULL6;
        write_u32(RCC_CFGR0, cfgr0)?;

        write_u32(RCC_CTLR, read_u32(RCC_CTLR)? | RCC_HSEON)?;
        for _ in 0..HSE_TIMEOUT {
            if (read_u32(RCC_CTLR)? & RCC_HSERDY) != 0 {
                let mut pll_cfg = read_u32(RCC_CFGR0)?;
                pll_cfg &= !(RCC_PLLSRC | RCC_PLLXTPRE | RCC_PLLMULL_MASK | RCC_USBPRE_MASK);
                pll_cfg |= RCC_PLLSRC | RCC_PLLMULL6;
                write_u32(RCC_CFGR0, pll_cfg)?;
                write_u32(RCC_CTLR, read_u32(RCC_CTLR)? | RCC_PLLON)?;
                for _ in 0..PLL_TIMEOUT {
                    if (read_u32(RCC_CTLR)? & RCC_PLLRDY) != 0 {
                        let mut switched = read_u32(RCC_CFGR0)?;
                        switched &= !RCC_SW_MASK;
                        switched |= RCC_SW_PLL;
                        write_u32(RCC_CFGR0, switched)?;
                        for _ in 0..SWITCH_TIMEOUT {
                            if (read_u32(RCC_CFGR0)? & RCC_SWS_MASK) == RCC_SWS_PLL {
                                write_u32(RCC_APB1PCENR, read_u32(RCC_APB1PCENR)? | RCC_APB1_USB_EN)?;
                                return Ok(());
                            }
                        }
                        break;
                    }
                }
                break;
            }
        }

        modify_u32(EXTEN_CTR, 0, EXTEN_PLL_HSI_PRE)?;
        let mut cfgr0 = read_u32(RCC_CFGR0)?;
        cfgr0 &= !(RCC_SW_MASK | RCC_HPRE_MASK | RCC_PPRE1_MASK | RCC_PPRE2_MASK | RCC_PLLSRC | RCC_PLLXTPRE | RCC_PLLMULL_MASK | RCC_USBPRE_MASK);
        cfgr0 |= RCC_PPRE1_DIV2 | RCC_PLLMULL6;
        write_u32(RCC_CFGR0, cfgr0)?;
        write_u32(RCC_CTLR, read_u32(RCC_CTLR)? | RCC_PLLON)?;
        for _ in 0..PLL_TIMEOUT {
            if (read_u32(RCC_CTLR)? & RCC_PLLRDY) != 0 {
                let mut switched = read_u32(RCC_CFGR0)?;
                switched &= !RCC_SW_MASK;
                switched |= RCC_SW_PLL;
                write_u32(RCC_CFGR0, switched)?;
                for _ in 0..SWITCH_TIMEOUT {
                    if (read_u32(RCC_CFGR0)? & RCC_SWS_MASK) == RCC_SWS_PLL {
                        write_u32(RCC_APB1PCENR, read_u32(RCC_APB1PCENR)? | RCC_APB1_USB_EN)?;
                        return Ok(());
                    }
                }
                break;
            }
        }

        Err(metadata::Error::Unsupported("failed to configure CH32 FSDEV USB clock to 48 MHz"))
    }
}
