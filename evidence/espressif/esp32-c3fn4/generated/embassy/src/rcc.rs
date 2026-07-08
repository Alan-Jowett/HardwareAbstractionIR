//! Generated Embassy-style rcc module for ESP32-C3FN4.

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

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "rcc",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: SystemClockReset (rcc) from canonical block block.system -> system-controller
pub const DRV_RCC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clkbind.uart0", name: "UART_CLK_EN", consumer_ref: "per.uart0", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clkbind.uart1", name: "UART1_CLK_EN", consumer_ref: "per.uart1", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clkbind.spi2", name: "SPI2_CLK_EN", consumer_ref: "per.spi2", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clkbind.i2c0", name: "I2C_EXT0_CLK_EN", consumer_ref: "per.i2c0", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clkbind.apb_saradc", name: "APB_SARADC_CLK_EN", consumer_ref: "per.apb_saradc", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clkbind.usb_device", name: "USB_DEVICE_CLK_EN", consumer_ref: "per.usb_device", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clkbind.systimer", name: "SYSTIMER_CLK_EN", consumer_ref: "per.systimer", clock_ref: "clk.systimer", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clkbind.dma", name: "DMA_CLK_EN", consumer_ref: "per.dma", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en1"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_RCC_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rstbind.uart0", name: "UART_RST", target_ref: "per.uart0", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rstbind.uart1", name: "UART1_RST", target_ref: "per.uart1", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rstbind.spi2", name: "SPI2_RST", target_ref: "per.spi2", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rstbind.i2c0", name: "I2C_EXT0_RST", target_ref: "per.i2c0", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rstbind.apb_saradc", name: "APB_SARADC_RST", target_ref: "per.apb_saradc", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rstbind.usb_device", name: "USB_DEVICE_RST", target_ref: "per.usb_device", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rstbind.systimer", name: "SYSTIMER_RST", target_ref: "per.systimer", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rstbind.dma", name: "DMA_RST", target_ref: "per.dma", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en1"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_RCC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_RCC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_RCC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_RCC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_RCC_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_RCC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_RCC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_RCC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct SystemClockResetResources {
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
    pub capability_tags: &'static [&'static str],
}

pub const DRV_RCC_RESOURCES: SystemClockResetResources = SystemClockResetResources {
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
    capability_tags: DRV_RCC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct SystemClockReset {
    resources: SystemClockResetResources,
}

impl SystemClockReset {
    pub fn new(resources: SystemClockResetResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> SystemClockResetResources {
        self.resources
    }
    /// Enable the UART0 clock gate.
    pub fn enable_uart0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the UART0 clock gate.
    pub fn disable_uart0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the UART1 clock gate.
    pub fn enable_uart1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the UART1 clock gate.
    pub fn disable_uart1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the SPI2 clock gate.
    pub fn enable_spi2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    /// Disable the SPI2 clock gate.
    pub fn disable_spi2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the I2C0 clock gate.
    pub fn enable_i2c0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the I2C0 clock gate.
    pub fn disable_i2c0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the APB_SARADC clock gate.
    pub fn enable_apb_saradc_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x10000000u32, 0x10000000u32)?;
        Ok(())
    }

    /// Disable the APB_SARADC clock gate.
    pub fn disable_apb_saradc_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x10000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the USB_DEVICE clock gate.
    pub fn enable_usb_device_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00800000u32, 0x00800000u32)?;
        Ok(())
    }

    /// Disable the USB_DEVICE clock gate.
    pub fn disable_usb_device_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00800000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the SYSTIMER clock gate.
    pub fn enable_systimer_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x20000000u32, 0x20000000u32)?;
        Ok(())
    }

    /// Disable the SYSTIMER clock gate.
    pub fn disable_systimer_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x20000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the DMA clock gate.
    pub fn enable_dma_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0014u64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    /// Disable the DMA clock gate.
    pub fn disable_dma_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0014u64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART0.
    pub fn assert_uart0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for UART0.
    pub fn release_uart0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART1.
    pub fn assert_uart1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Release reset for UART1.
    pub fn release_uart1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SPI2.
    pub fn assert_spi2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    /// Release reset for SPI2.
    pub fn release_spi2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for I2C0.
    pub fn assert_i2c0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Release reset for I2C0.
    pub fn release_i2c0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for APB_SARADC.
    pub fn assert_apb_saradc_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x10000000u32, 0x10000000u32)?;
        Ok(())
    }

    /// Release reset for APB_SARADC.
    pub fn release_apb_saradc_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x10000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USB_DEVICE.
    pub fn assert_usb_device_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00800000u32, 0x00800000u32)?;
        Ok(())
    }

    /// Release reset for USB_DEVICE.
    pub fn release_usb_device_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00800000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SYSTIMER.
    pub fn assert_systimer_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x20000000u32, 0x20000000u32)?;
        Ok(())
    }

    /// Release reset for SYSTIMER.
    pub fn release_systimer_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x20000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for DMA.
    pub fn assert_dma_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C001Cu64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    /// Release reset for DMA.
    pub fn release_dma_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C001Cu64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }


}

