//! Generated Embassy-style uart module for STM32F405RGT6.

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

// Driver instance: Uart4 (uart) from canonical block block.uart4 -> uart
pub const DRV_UART4_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.uart4", name: "UART4 clock", consumer_ref: "periph.uart4", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_UART4_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.uart4", name: "UART4 reset", target_ref: "periph.uart4", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.apb1"), binding_kind: "software", control_refs: &["reg.rcc.apb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_UART4_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.uart4.global", name: "UART4 GLOBAL source", source_ref: "periph.uart4", producer_ref: Some("block.uart4"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_UART4_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.uart4.global", name: "UART4 GLOBAL route", source_ref: "isrc.uart4.global", interrupt_ref: "irq.uart4", controller_ref: "block.nvic", cpu_target_ref: None, line_index: Some(52), route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_UART4_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_UART4_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_UART4_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart4.tx.pa0", name: "UART4 TX on PA0", pin_ref: "pin.pa0", peripheral_ref: "periph.uart4", signal: "TX", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.uart4.tx.pc10", name: "UART4 TX on PC10", pin_ref: "pin.pc10", peripheral_ref: "periph.uart4", signal: "TX", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_UART4_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart4.rx.pa1", name: "UART4 RX on PA1", pin_ref: "pin.pa1", peripheral_ref: "periph.uart4", signal: "RX", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.uart4.rx.pc11", name: "UART4 RX on PC11", pin_ref: "pin.pc11", peripheral_ref: "periph.uart4", signal: "RX", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_UART4_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "tx", signal: "TX", routes: DRV_UART4_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "rx", signal: "RX", routes: DRV_UART4_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_UART4_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_UART4_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_UART4_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Uart4Resources {
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

pub const DRV_UART4_RESOURCES: Uart4Resources = Uart4Resources {
    clocks: DRV_UART4_CLOCK_BINDINGS,
    resets: DRV_UART4_RESET_BINDINGS,
    interrupt_sources: DRV_UART4_INTERRUPT_SOURCES,
    interrupts: DRV_UART4_INTERRUPT_ROUTES,
    dma_channels: DRV_UART4_DMA_CHANNELS,
    dma: DRV_UART4_DMA_ROUTES,
    pins: DRV_UART4_PIN_ROLES,
    init_operations: DRV_UART4_INIT_OPERATIONS,
    state_machines: DRV_UART4_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_UART4_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Uart4 {
    resources: Uart4Resources,
}

impl Uart4 {
    pub fn new(resources: Uart4Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Uart4Resources {
        self.resources
    }
    /// Enable the UART4 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00080000u32, 0x00080000u32)?;
        Ok(())
    }

    /// Disable the UART4 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00080000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART4.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00080000u32, 0x00080000u32)?;
        Ok(())
    }

    /// Release reset for UART4.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00080000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable Uart4.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00002000u32, 0x00002000u32)?;
        Ok(())
    }

    /// Disable Uart4.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00002000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Uart4 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the Uart4 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Uart4 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the Uart4 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00002000u32, 0x00000000u32)?;
        modify_u32(0x40004C0Cu64, 0x00000008u32, 0x00000000u32)?;
        modify_u32(0x40004C0Cu64, 0x00000004u32, 0x00000000u32)?;
        modify_u32(0x40004C0Cu64, 0x00008000u32, 0x00000000u32)?;
        modify_u32(0x40004C0Cu64, 0x00001000u32, 0x00000000u32)?;
        modify_u32(0x40004C0Cu64, 0x00000400u32, 0x00000000u32)?;
        modify_u32(0x40004C0Cu64, 0x00000200u32, 0x00000000u32)?;
        modify_u32(0x40004C10u64, 0x00003000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported("USART baud mantissa exceeds modeled field width"));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported("USART baud fraction exceeds modeled field width"));
        }
        modify_u32(0x40004C08u64, 0x0000FFFFu32, ((u32::from(mantissa) & 0xFFFu32) << 4) | ((u32::from(fraction) & 0xFu32) << 0))?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while (read_u32(0x40004C00u64)? & 0x00000080u32) == 0 {}
        write_u32(0x40004C04u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while (read_u32(0x40004C00u64)? & 0x00000040u32) == 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x40004C00u64)? & 0x00000020u32) == 0 {}
        Ok((read_u32(0x40004C04u64)? & 0xFFu32) as u8)
    }

    /// Enable the Uart4 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the Uart4 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Uart4 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the Uart4 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40004C0Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }
}
// Driver instance: Uart5 (uart) from canonical block block.uart5 -> uart
pub const DRV_UART5_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.uart5", name: "UART5 clock", consumer_ref: "periph.uart5", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1enr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_UART5_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.uart5", name: "UART5 reset", target_ref: "periph.uart5", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rdom.apb1"), binding_kind: "software", control_refs: &["reg.rcc.apb1rstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_UART5_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.uart5.global", name: "UART5 GLOBAL source", source_ref: "periph.uart5", producer_ref: Some("block.uart5"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_UART5_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.uart5.global", name: "UART5 GLOBAL route", source_ref: "isrc.uart5.global", interrupt_ref: "irq.uart5", controller_ref: "block.nvic", cpu_target_ref: None, line_index: Some(53), route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_UART5_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_UART5_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_UART5_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart5.tx.pc12", name: "UART5 TX on PC12", pin_ref: "pin.pc12", peripheral_ref: "periph.uart5", signal: "TX", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_UART5_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.uart5.rx.pd2", name: "UART5 RX on PD2", pin_ref: "pin.pd2", peripheral_ref: "periph.uart5", signal: "RX", route_type: "muxed", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_UART5_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "tx", signal: "TX", routes: DRV_UART5_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "rx", signal: "RX", routes: DRV_UART5_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_UART5_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_UART5_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_UART5_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Uart5Resources {
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

pub const DRV_UART5_RESOURCES: Uart5Resources = Uart5Resources {
    clocks: DRV_UART5_CLOCK_BINDINGS,
    resets: DRV_UART5_RESET_BINDINGS,
    interrupt_sources: DRV_UART5_INTERRUPT_SOURCES,
    interrupts: DRV_UART5_INTERRUPT_ROUTES,
    dma_channels: DRV_UART5_DMA_CHANNELS,
    dma: DRV_UART5_DMA_ROUTES,
    pins: DRV_UART5_PIN_ROLES,
    init_operations: DRV_UART5_INIT_OPERATIONS,
    state_machines: DRV_UART5_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_UART5_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Uart5 {
    resources: Uart5Resources,
}

impl Uart5 {
    pub fn new(resources: Uart5Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Uart5Resources {
        self.resources
    }
    /// Enable the UART5 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00100000u32, 0x00100000u32)?;
        Ok(())
    }

    /// Disable the UART5 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023840u64, 0x00100000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART5.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00100000u32, 0x00100000u32)?;
        Ok(())
    }

    /// Release reset for UART5.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40023820u64, 0x00100000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable Uart5.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00002000u32, 0x00002000u32)?;
        Ok(())
    }

    /// Disable Uart5.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00002000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Uart5 transmitter.
    pub fn enable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the Uart5 transmitter.
    pub fn disable_transmitter(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Uart5 receiver.
    pub fn enable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the Uart5 receiver.
    pub fn disable_receiver(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00002000u32, 0x00000000u32)?;
        modify_u32(0x4000500Cu64, 0x00000008u32, 0x00000000u32)?;
        modify_u32(0x4000500Cu64, 0x00000004u32, 0x00000000u32)?;
        modify_u32(0x4000500Cu64, 0x00008000u32, 0x00000000u32)?;
        modify_u32(0x4000500Cu64, 0x00001000u32, 0x00000000u32)?;
        modify_u32(0x4000500Cu64, 0x00000400u32, 0x00000000u32)?;
        modify_u32(0x4000500Cu64, 0x00000200u32, 0x00000000u32)?;
        modify_u32(0x40005010u64, 0x00003000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(mantissa) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported("USART baud mantissa exceeds modeled field width"));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported("USART baud fraction exceeds modeled field width"));
        }
        modify_u32(0x40005008u64, 0x0000FFFFu32, ((u32::from(mantissa) & 0xFFFu32) << 4) | ((u32::from(fraction) & 0xFu32) << 0))?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while (read_u32(0x40005000u64)? & 0x00000080u32) == 0 {}
        write_u32(0x40005004u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while (read_u32(0x40005000u64)? & 0x00000040u32) == 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x40005000u64)? & 0x00000020u32) == 0 {}
        Ok((read_u32(0x40005004u64)? & 0xFFu32) as u8)
    }

    /// Enable the Uart5 TXE interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the Uart5 TXE interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Uart5 RXNE interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }

    /// Disable the Uart5 RXNE interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4000500Cu64, 0x00000020u32, 0x00000000u32)?;
        Ok(())
    }
}
