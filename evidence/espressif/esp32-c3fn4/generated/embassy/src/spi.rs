//! Generated Embassy-style spi module for ESP32-C3FN4.

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
    module_name: "spi",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: Spi2 (spi) from canonical block block.spi2 -> spi
pub const DRV_SPI2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clkbind.spi2", name: "SPI2_CLK_EN", consumer_ref: "per.spi2", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_SPI2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rstbind.spi2", name: "SPI2_RST", target_ref: "per.spi2", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_SPI2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.spi2", name: "SPI2", source_ref: "per.spi2", producer_ref: Some("block.spi2"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_SPI2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.spi2", name: "SPI2 interrupt matrix source", source_ref: "isrc.spi2", interrupt_ref: "irq.ets_spi2_intr_source", controller_ref: "block.interrupt_matrix0", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "matrix", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_SPI2_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel { id: "dma.tx_ch0", name: "GDMA TX channel 0", controller_ref: "block.gdma0", target_ref: None, channel_index: 0, capabilities: &["tx", "peripheral"], priority_levels: &[] }, metadata::DmaChannel { id: "dma.rx_ch0", name: "GDMA RX channel 0", controller_ref: "block.gdma0", target_ref: None, channel_index: 0, capabilities: &["rx", "peripheral"], priority_levels: &[] }, metadata::DmaChannel { id: "dma.tx_ch1", name: "GDMA TX channel 1", controller_ref: "block.gdma0", target_ref: None, channel_index: 1, capabilities: &["tx", "peripheral"], priority_levels: &[] }, metadata::DmaChannel { id: "dma.rx_ch1", name: "GDMA RX channel 1", controller_ref: "block.gdma0", target_ref: None, channel_index: 1, capabilities: &["rx", "peripheral"], priority_levels: &[] }, metadata::DmaChannel { id: "dma.tx_ch2", name: "GDMA TX channel 2", controller_ref: "block.gdma0", target_ref: None, channel_index: 2, capabilities: &["tx", "peripheral"], priority_levels: &[] }, metadata::DmaChannel { id: "dma.rx_ch2", name: "GDMA RX channel 2", controller_ref: "block.gdma0", target_ref: None, channel_index: 2, capabilities: &["rx", "peripheral"], priority_levels: &[] }];
pub const DRV_SPI2_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute { id: "dmaroute.spi2.tx.ch0", name: "SPI2 TX via GDMA channel 0", peripheral_ref: "per.spi2", signal: Some("FSPID"), channel_ref: "dma.tx_ch0", direction: "memory-to-peripheral", control_refs: &["reg.dma.out_peri_sel_ch0"], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.spi2.rx.ch0", name: "SPI2 RX via GDMA channel 0", peripheral_ref: "per.spi2", signal: Some("FSPIQ"), channel_ref: "dma.rx_ch0", direction: "peripheral-to-memory", control_refs: &["reg.dma.in_peri_sel_ch0"], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.spi2.tx.ch1", name: "SPI2 TX via GDMA channel 1", peripheral_ref: "per.spi2", signal: Some("FSPID"), channel_ref: "dma.tx_ch1", direction: "memory-to-peripheral", control_refs: &["reg.dma.out_peri_sel_ch1"], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.spi2.rx.ch1", name: "SPI2 RX via GDMA channel 1", peripheral_ref: "per.spi2", signal: Some("FSPIQ"), channel_ref: "dma.rx_ch1", direction: "peripheral-to-memory", control_refs: &["reg.dma.in_peri_sel_ch1"], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.spi2.tx.ch2", name: "SPI2 TX via GDMA channel 2", peripheral_ref: "per.spi2", signal: Some("FSPID"), channel_ref: "dma.tx_ch2", direction: "memory-to-peripheral", control_refs: &["reg.dma.out_peri_sel_ch2"], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.spi2.rx.ch2", name: "SPI2 RX via GDMA channel 2", peripheral_ref: "per.spi2", signal: Some("FSPIQ"), channel_ref: "dma.rx_ch2", direction: "peripheral-to-memory", control_refs: &["reg.dma.in_peri_sel_ch2"], shared_channel_group_ref: None }];
pub const DRV_SPI2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.clk.gpio6", name: "SPI2 CLK on GPIO6", pin_ref: "pin.gpio6", peripheral_ref: "per.spi2", signal: "FSPICLK", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.d.gpio7", name: "SPI2 D on GPIO7", pin_ref: "pin.gpio7", peripheral_ref: "per.spi2", signal: "FSPID", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.q.gpio2", name: "SPI2 Q on GPIO2", pin_ref: "pin.gpio2", peripheral_ref: "per.spi2", signal: "FSPIQ", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.cs.gpio10", name: "SPI2 CS0 on GPIO10", pin_ref: "pin.gpio10", peripheral_ref: "per.spi2", signal: "FSPICS0", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.wp.gpio5", name: "SPI2 WP on GPIO5", pin_ref: "pin.gpio5", peripheral_ref: "per.spi2", signal: "FSPIWP", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLE_5_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.spi2.hd.gpio4", name: "SPI2 HD on GPIO4", pin_ref: "pin.gpio4", peripheral_ref: "per.spi2", signal: "FSPIHD", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_SPI2_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "sck", signal: "FSPICLK", routes: DRV_SPI2_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "mosi", signal: "FSPID", routes: DRV_SPI2_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "miso", signal: "FSPIQ", routes: DRV_SPI2_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "cs", signal: "FSPICS0", routes: DRV_SPI2_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "wp", signal: "FSPIWP", routes: DRV_SPI2_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "hd", signal: "FSPIHD", routes: DRV_SPI2_PIN_ROLE_5_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_SPI2_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.spi2.enable_clock_gate", name: "Enable SPI2 clock gate", description: None, kind: Some("initialization"), target_refs: &["per.spi2"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.spi2.clk_gate"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CLK_EN" }), value: None, description: Some("Enable the SPI2 internal functional clock.") }], preconditions: &[], postconditions: &[] }, metadata::SemanticOperation { id: "op.spi2.start_transaction", name: "Start SPI2 user transaction", description: None, kind: Some("transaction"), target_refs: &["per.spi2"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.spi2.cmd"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set USR" }), value: None, description: Some("Start the programmed SPI2 user transaction.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_SPI2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_SPI2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Spi2Resources {
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

pub const DRV_SPI2_RESOURCES: Spi2Resources = Spi2Resources {
    clocks: DRV_SPI2_CLOCK_BINDINGS,
    resets: DRV_SPI2_RESET_BINDINGS,
    interrupt_sources: DRV_SPI2_INTERRUPT_SOURCES,
    interrupts: DRV_SPI2_INTERRUPT_ROUTES,
    dma_channels: DRV_SPI2_DMA_CHANNELS,
    dma: DRV_SPI2_DMA_ROUTES,
    pins: DRV_SPI2_PIN_ROLES,
    init_operations: DRV_SPI2_INIT_OPERATIONS,
    state_machines: DRV_SPI2_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_SPI2_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Spi2 {
    resources: Spi2Resources,
}

impl Spi2 {
    pub fn new(resources: Spi2Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Spi2Resources {
        self.resources
    }
    /// Enable the SPI2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    /// Disable the SPI2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SPI2.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    /// Release reset for SPI2.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Spi2 TX DMA path.
    pub fn enable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60024030u64, 0x10000000u32, 0x10000000u32)?;
        Ok(())
    }

    /// Disable the Spi2 TX DMA path.
    pub fn disable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60024030u64, 0x10000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Spi2 RX DMA path.
    pub fn enable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60024030u64, 0x08000000u32, 0x08000000u32)?;
        Ok(())
    }

    /// Disable the Spi2 RX DMA path.
    pub fn disable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60024030u64, 0x08000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Spi2 DMA segment-done interrupt.
    pub fn enable_dma_segment_done_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60024034u64, 0x00002000u32, 0x00002000u32)?;
        Ok(())
    }

    /// Disable the Spi2 DMA segment-done interrupt.
    pub fn disable_dma_segment_done_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60024034u64, 0x00002000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable_clock_gate(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600240E8u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn apply_start_transaction(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60024000u64, 0x01000000u32, 0x01000000u32)?;
        Ok(())
    }


}

