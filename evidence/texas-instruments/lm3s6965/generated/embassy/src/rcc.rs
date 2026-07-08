//! Generated Embassy-style rcc module for LM3S6965.

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

// Driver instance: SYSCTL (rcc) from canonical block block.rcc -> clock-controller
pub const DRV_RCC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.gpioa", name: "GPIOA", consumer_ref: "periph.gpioa", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpiob", name: "GPIOB", consumer_ref: "periph.gpiob", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpioc", name: "GPIOC", consumer_ref: "periph.gpioc", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpiod", name: "GPIOD", consumer_ref: "periph.gpiod", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpioe", name: "GPIOE", consumer_ref: "periph.gpioe", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.gpiof", name: "GPIOF", consumer_ref: "periph.gpiof", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc2"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.uart0", name: "UART0", consumer_ref: "periph.uart0", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.uart1", name: "UART1", consumer_ref: "periph.uart1", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.ssi0", name: "SSI0", consumer_ref: "periph.ssi0", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.i2c0", name: "I2C0", consumer_ref: "periph.i2c0", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.timer0", name: "TIMER0", consumer_ref: "periph.timer0", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.timer1", name: "TIMER1", consumer_ref: "periph.timer1", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.timer2", name: "TIMER2", consumer_ref: "periph.timer2", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.timer3", name: "TIMER3", consumer_ref: "periph.timer3", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc1"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.watchdog0", name: "WATCHDOG0", consumer_ref: "periph.watchdog0", clock_ref: "clock.sysclk", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.sysctl.rcgc0"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_RCC_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.gpioa", name: "GPIOA", target_ref: "periph.gpioa", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpiob", name: "GPIOB", target_ref: "periph.gpiob", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpioc", name: "GPIOC", target_ref: "periph.gpioc", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpiod", name: "GPIOD", target_ref: "periph.gpiod", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpioe", name: "GPIOE", target_ref: "periph.gpioe", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.gpiof", name: "GPIOF", target_ref: "periph.gpiof", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr2"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.uart0", name: "UART0", target_ref: "periph.uart0", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.uart1", name: "UART1", target_ref: "periph.uart1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.ssi0", name: "SSI0", target_ref: "periph.ssi0", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.i2c0", name: "I2C0", target_ref: "periph.i2c0", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.timer0", name: "TIMER0", target_ref: "periph.timer0", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.timer1", name: "TIMER1", target_ref: "periph.timer1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.timer2", name: "TIMER2", target_ref: "periph.timer2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }, metadata::ResetBinding { id: "rst.timer3", name: "TIMER3", target_ref: "periph.timer3", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rd.software"), binding_kind: "software", control_refs: &["reg.sysctl.srcr1"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_RCC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_RCC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_RCC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_RCC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_RCC_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_RCC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_RCC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_RCC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct SYSCTLResources {
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

pub const DRV_RCC_RESOURCES: SYSCTLResources = SYSCTLResources {
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
pub struct SYSCTL {
    resources: SYSCTLResources,
}

impl SYSCTL {
    pub fn new(resources: SYSCTLResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> SYSCTLResources {
        self.resources
    }
    /// Enable the GPIOA clock gate.
    pub fn enable_gpioa_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the GPIOA clock gate.
    pub fn disable_gpioa_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOB clock gate.
    pub fn enable_gpiob_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the GPIOB clock gate.
    pub fn disable_gpiob_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOC clock gate.
    pub fn enable_gpioc_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the GPIOC clock gate.
    pub fn disable_gpioc_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOD clock gate.
    pub fn enable_gpiod_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the GPIOD clock gate.
    pub fn disable_gpiod_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOE clock gate.
    pub fn enable_gpioe_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Disable the GPIOE clock gate.
    pub fn disable_gpioe_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the GPIOF clock gate.
    pub fn enable_gpiof_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the GPIOF clock gate.
    pub fn disable_gpiof_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE100u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the UART0 clock gate.
    pub fn enable_uart0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the UART0 clock gate.
    pub fn disable_uart0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the UART1 clock gate.
    pub fn enable_uart1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the UART1 clock gate.
    pub fn disable_uart1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the SSI0 clock gate.
    pub fn enable_ssi0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Disable the SSI0 clock gate.
    pub fn disable_ssi0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the I2C0 clock gate.
    pub fn enable_i2c0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Disable the I2C0 clock gate.
    pub fn disable_i2c0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIMER0 clock gate.
    pub fn enable_timer0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00010000u32, 0x00010000u32)?;
        Ok(())
    }

    /// Disable the TIMER0 clock gate.
    pub fn disable_timer0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00010000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIMER1 clock gate.
    pub fn enable_timer1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Disable the TIMER1 clock gate.
    pub fn disable_timer1_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIMER2 clock gate.
    pub fn enable_timer2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00040000u32, 0x00040000u32)?;
        Ok(())
    }

    /// Disable the TIMER2 clock gate.
    pub fn disable_timer2_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00040000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the TIMER3 clock gate.
    pub fn enable_timer3_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00080000u32, 0x00080000u32)?;
        Ok(())
    }

    /// Disable the TIMER3 clock gate.
    pub fn disable_timer3_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0FCu64, 0x00080000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the WATCHDOG0 clock gate.
    pub fn enable_watchdog0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0F8u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the WATCHDOG0 clock gate.
    pub fn disable_watchdog0_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE0F8u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOA.
    pub fn assert_gpioa_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Release reset for GPIOA.
    pub fn release_gpioa_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOB.
    pub fn assert_gpiob_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Release reset for GPIOB.
    pub fn release_gpiob_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOC.
    pub fn assert_gpioc_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for GPIOC.
    pub fn release_gpioc_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOD.
    pub fn assert_gpiod_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Release reset for GPIOD.
    pub fn release_gpiod_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOE.
    pub fn assert_gpioe_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Release reset for GPIOE.
    pub fn release_gpioe_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for GPIOF.
    pub fn assert_gpiof_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Release reset for GPIOF.
    pub fn release_gpiof_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE044u64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART0.
    pub fn assert_uart0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Release reset for UART0.
    pub fn release_uart0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART1.
    pub fn assert_uart1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Release reset for UART1.
    pub fn release_uart1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SSI0.
    pub fn assert_ssi0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    /// Release reset for SSI0.
    pub fn release_ssi0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for I2C0.
    pub fn assert_i2c0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00001000u32, 0x00001000u32)?;
        Ok(())
    }

    /// Release reset for I2C0.
    pub fn release_i2c0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00001000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER0.
    pub fn assert_timer0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00010000u32, 0x00010000u32)?;
        Ok(())
    }

    /// Release reset for TIMER0.
    pub fn release_timer0_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00010000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER1.
    pub fn assert_timer1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Release reset for TIMER1.
    pub fn release_timer1_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER2.
    pub fn assert_timer2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00040000u32, 0x00040000u32)?;
        Ok(())
    }

    /// Release reset for TIMER2.
    pub fn release_timer2_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00040000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIMER3.
    pub fn assert_timer3_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00080000u32, 0x00080000u32)?;
        Ok(())
    }

    /// Release reset for TIMER3.
    pub fn release_timer3_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x400FE040u64, 0x00080000u32, 0x00000000u32)?;
        Ok(())
    }


}

