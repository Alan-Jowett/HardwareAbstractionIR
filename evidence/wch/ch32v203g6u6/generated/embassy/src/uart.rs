//! Generated Embassy-style uart module for CH32V203G6U6.

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
    module_name: "uart",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: USART1 (usart) from canonical block block.usart1 -> usart
pub const DRV_USART1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.usart1", name: "USART1 clock binding", consumer_ref: "periph.usart1", clock_ref: "clk.pclk2", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb2pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_USART1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.usart1", name: "USART1 reset binding", target_ref: "periph.usart1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb2"), binding_kind: "local", control_refs: &["reg.rcc.apb2prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_USART1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.usart1.global", name: "USART1 GLOBAL interrupt source", source_ref: "periph.usart1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_USART1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.usart1.global", name: "USART1 GLOBAL interrupt route", source_ref: "isrc.usart1.global", interrupt_ref: "int.usart1", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_USART1_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel { id: "dmach.dma1.ch4", name: "DMA1 Channel 4", controller_ref: "block.dma1", target_ref: None, channel_index: 4, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dmach.dma1.ch5", name: "DMA1 Channel 5", controller_ref: "block.dma1", target_ref: None, channel_index: 5, capabilities: &[], priority_levels: &[] }];
pub const DRV_USART1_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute { id: "dmaroute.usart1.tx", name: "USART1 TX DMA route", peripheral_ref: "periph.usart1", signal: Some("TX"), channel_ref: "dmach.dma1.ch4", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.usart1.rx", name: "USART1 RX DMA route", peripheral_ref: "periph.usart1", signal: Some("RX"), channel_ref: "dmach.dma1.ch5", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: None }];
pub const DRV_USART1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart1.tx.pa9.r0", name: "USART1 TX on PA9 (remap 0)", pin_ref: "pin.pa9", peripheral_ref: "periph.usart1", signal: "TX", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.usart1.tx.pb6.r1", name: "USART1 TX on PB6 (remap 1)", pin_ref: "pin.pb6", peripheral_ref: "periph.usart1", signal: "TX", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.usart1.tx.pa6.r3", name: "USART1 TX on PA6 (remap 3)", pin_ref: "pin.pa6", peripheral_ref: "periph.usart1", signal: "TX", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart1.rx.pa10.r0", name: "USART1 RX on PA10 (remap 0)", pin_ref: "pin.pa10", peripheral_ref: "periph.usart1", signal: "RX", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.usart1.rx.pb7.r1", name: "USART1 RX on PB7 (remap 1)", pin_ref: "pin.pb7", peripheral_ref: "periph.usart1", signal: "RX", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.usart1.rx.pa7.r3", name: "USART1 RX on PA7 (remap 3)", pin_ref: "pin.pa7", peripheral_ref: "periph.usart1", signal: "RX", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart1.ck.pa10.r2", name: "USART1 CK on PA10 (remap 2)", pin_ref: "pin.pa10", peripheral_ref: "periph.usart1", signal: "CK", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.usart1.ck.pa5.r3", name: "USART1 CK on PA5 (remap 3)", pin_ref: "pin.pa5", peripheral_ref: "periph.usart1", signal: "CK", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART1_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart1.cts.pa11.r0", name: "USART1 CTS on PA11 (remap 0)", pin_ref: "pin.pa11", peripheral_ref: "periph.usart1", signal: "CTS", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.usart1.cts.pa11.r1", name: "USART1 CTS on PA11 (remap 1)", pin_ref: "pin.pa11", peripheral_ref: "periph.usart1", signal: "CTS", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.usart1.cts.pa5.r2", name: "USART1 CTS on PA5 (remap 2)", pin_ref: "pin.pa5", peripheral_ref: "periph.usart1", signal: "CTS", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART1_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart1.rts.pa12.r0", name: "USART1 RTS on PA12 (remap 0)", pin_ref: "pin.pa12", peripheral_ref: "periph.usart1", signal: "RTS", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.usart1.rts.pa12.r1", name: "USART1 RTS on PA12 (remap 1)", pin_ref: "pin.pa12", peripheral_ref: "periph.usart1", signal: "RTS", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.usart1.rts.pa9.r2", name: "USART1 RTS on PA9 (remap 2)", pin_ref: "pin.pa9", peripheral_ref: "periph.usart1", signal: "RTS", route_type: "selectable", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "tx", signal: "TX", routes: DRV_USART1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "rx", signal: "RX", routes: DRV_USART1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "ck", signal: "CK", routes: DRV_USART1_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "cts", signal: "CTS", routes: DRV_USART1_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "rts", signal: "RTS", routes: DRV_USART1_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_USART1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_USART1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USART1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct USART1Resources {
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

pub const DRV_USART1_RESOURCES: USART1Resources = USART1Resources {
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
pub struct USART1 {
    resources: USART1Resources,
}

impl USART1 {
    pub fn new(resources: USART1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> USART1Resources {
        self.resources
    }
    /// Enable the USART1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00004000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Disable the USART1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021018u64, 0x00004000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00004000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Release reset for USART1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002100Cu64, 0x00004000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable USART1.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x2000u16, 0x2000u16)?;
        Ok(())
    }

    /// Disable USART1.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x2000u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART1 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0008u16, 0x0008u16)?;
        Ok(())
    }

    /// Disable the USART1 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0008u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART1 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0004u16, 0x0004u16)?;
        Ok(())
    }

    /// Disable the USART1 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0004u16, 0x0000u16)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x2000u16, 0x0000u16)?;
        modify_u16(0x4001380Cu64, 0x0008u16, 0x0000u16)?;
        modify_u16(0x4001380Cu64, 0x0004u16, 0x0000u16)?;
        modify_u16(0x4001380Cu64, 0x8000u16, 0x0000u16)?;
        modify_u16(0x4001380Cu64, 0x1000u16, 0x0000u16)?;
        modify_u16(0x4001380Cu64, 0x0400u16, 0x0000u16)?;
        modify_u16(0x4001380Cu64, 0x0200u16, 0x0000u16)?;
        modify_u16(0x40013810u64, 0x3000u16, 0x0000u16)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported("USART baud mantissa exceeds modeled field width"));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported("USART baud fraction exceeds modeled field width"));
        }
        modify_u32(0x40013808u64, 0x0000FFFFu32, ((u32::from(mantissa) & 0xFFFu32) << 4) | ((u32::from(fraction) & 0xFu32) << 0))?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while (read_u32(0x40013800u64)? & 0x00000080u32) == 0 {}
        write_u32(0x40013804u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while (read_u32(0x40013800u64)? & 0x00000040u32) == 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x40013800u64)? & 0x00000020u32) == 0 {}
        Ok((read_u32(0x40013804u64)? & 0xFFu32) as u8)
    }

    /// Enable the USART1 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    /// Disable the USART1 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0080u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART1 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0020u16, 0x0020u16)?;
        Ok(())
    }

    /// Disable the USART1 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4001380Cu64, 0x0020u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART1 TX DMA path.
    pub fn enable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40013814u64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    /// Disable the USART1 TX DMA path.
    pub fn disable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40013814u64, 0x0080u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART1 RX DMA path.
    pub fn enable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40013814u64, 0x0040u16, 0x0040u16)?;
        Ok(())
    }

    /// Disable the USART1 RX DMA path.
    pub fn disable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40013814u64, 0x0040u16, 0x0000u16)?;
        Ok(())
    }
}
// Driver instance: USART2 (usart) from canonical block block.usart2 -> usart
pub const DRV_USART2_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.usart2", name: "USART2 clock binding", consumer_ref: "periph.usart2", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_USART2_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.usart2", name: "USART2 reset binding", target_ref: "periph.usart2", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_USART2_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.usart2.global", name: "USART2 GLOBAL interrupt source", source_ref: "periph.usart2", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_USART2_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.usart2.global", name: "USART2 GLOBAL interrupt route", source_ref: "isrc.usart2.global", interrupt_ref: "int.usart2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_USART2_DMA_CHANNELS: &[metadata::DmaChannel] = &[metadata::DmaChannel { id: "dmach.dma1.ch6", name: "DMA1 Channel 6", controller_ref: "block.dma1", target_ref: None, channel_index: 6, capabilities: &[], priority_levels: &[] }, metadata::DmaChannel { id: "dmach.dma1.ch7", name: "DMA1 Channel 7", controller_ref: "block.dma1", target_ref: None, channel_index: 7, capabilities: &[], priority_levels: &[] }];
pub const DRV_USART2_DMA_ROUTES: &[metadata::DmaRoute] = &[metadata::DmaRoute { id: "dmaroute.usart2.rx", name: "USART2 RX DMA route", peripheral_ref: "periph.usart2", signal: Some("RX"), channel_ref: "dmach.dma1.ch6", direction: "peripheral-to-memory", control_refs: &[], shared_channel_group_ref: None }, metadata::DmaRoute { id: "dmaroute.usart2.tx", name: "USART2 TX DMA route", peripheral_ref: "periph.usart2", signal: Some("TX"), channel_ref: "dmach.dma1.ch7", direction: "memory-to-peripheral", control_refs: &[], shared_channel_group_ref: None }];
pub const DRV_USART2_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart2.tx.pa2", name: "USART2 TX on PA2", pin_ref: "pin.pa2", peripheral_ref: "periph.usart2", signal: "TX", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART2_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart2.rx.pa3", name: "USART2 RX on PA3", pin_ref: "pin.pa3", peripheral_ref: "periph.usart2", signal: "RX", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART2_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart2.ck.pa4", name: "USART2 CK on PA4", pin_ref: "pin.pa4", peripheral_ref: "periph.usart2", signal: "CK", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART2_PIN_ROLE_3_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart2.cts.pa0", name: "USART2 CTS on PA0", pin_ref: "pin.pa0", peripheral_ref: "periph.usart2", signal: "CTS", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART2_PIN_ROLE_4_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usart2.rts.pa1", name: "USART2 RTS on PA1", pin_ref: "pin.pa1", peripheral_ref: "periph.usart2", signal: "RTS", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_USART2_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "tx", signal: "TX", routes: DRV_USART2_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "rx", signal: "RX", routes: DRV_USART2_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "ck", signal: "CK", routes: DRV_USART2_PIN_ROLE_2_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "cts", signal: "CTS", routes: DRV_USART2_PIN_ROLE_3_ROUTES, requirement: metadata::ResourceRequirement::Optional }, metadata::PinRole { role: "rts", signal: "RTS", routes: DRV_USART2_PIN_ROLE_4_ROUTES, requirement: metadata::ResourceRequirement::Optional }];
pub const DRV_USART2_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_USART2_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USART2_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct USART2Resources {
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

pub const DRV_USART2_RESOURCES: USART2Resources = USART2Resources {
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
pub struct USART2 {
    resources: USART2Resources,
}

impl USART2 {
    pub fn new(resources: USART2Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> USART2Resources {
        self.resources
    }
    /// Enable the USART2 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Disable the USART2 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USART2.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00020000u32, 0x00020000u32)?;
        Ok(())
    }

    /// Release reset for USART2.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00020000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable USART2.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x2000u16, 0x2000u16)?;
        Ok(())
    }

    /// Disable USART2.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x2000u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART2 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0008u16, 0x0008u16)?;
        Ok(())
    }

    /// Disable the USART2 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0008u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART2 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0004u16, 0x0004u16)?;
        Ok(())
    }

    /// Disable the USART2 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0004u16, 0x0000u16)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x2000u16, 0x0000u16)?;
        modify_u16(0x4000440Cu64, 0x0008u16, 0x0000u16)?;
        modify_u16(0x4000440Cu64, 0x0004u16, 0x0000u16)?;
        modify_u16(0x4000440Cu64, 0x8000u16, 0x0000u16)?;
        modify_u16(0x4000440Cu64, 0x1000u16, 0x0000u16)?;
        modify_u16(0x4000440Cu64, 0x0400u16, 0x0000u16)?;
        modify_u16(0x4000440Cu64, 0x0200u16, 0x0000u16)?;
        modify_u16(0x40004410u64, 0x3000u16, 0x0000u16)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported("USART baud mantissa exceeds modeled field width"));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported("USART baud fraction exceeds modeled field width"));
        }
        modify_u32(0x40004408u64, 0x0000FFFFu32, ((u32::from(mantissa) & 0xFFFu32) << 4) | ((u32::from(fraction) & 0xFu32) << 0))?;
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

    /// Enable the USART2 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    /// Disable the USART2 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0080u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART2 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0020u16, 0x0020u16)?;
        Ok(())
    }

    /// Disable the USART2 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x4000440Cu64, 0x0020u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART2 TX DMA path.
    pub fn enable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40004414u64, 0x0080u16, 0x0080u16)?;
        Ok(())
    }

    /// Disable the USART2 TX DMA path.
    pub fn disable_tx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40004414u64, 0x0080u16, 0x0000u16)?;
        Ok(())
    }

    /// Enable the USART2 RX DMA path.
    pub fn enable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40004414u64, 0x0040u16, 0x0040u16)?;
        Ok(())
    }

    /// Disable the USART2 RX DMA path.
    pub fn disable_rx_dma(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40004414u64, 0x0040u16, 0x0000u16)?;
        Ok(())
    }
}
