//! Generated Embassy-style uart module for ESP32-C3FN4.

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
    module_name: "uart",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: Uart0 (uart) from canonical block block.uart0 -> uart
pub const DRV_UART0_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clkbind.uart0",
    name: "UART_CLK_EN",
    consumer_ref: "per.uart0",
    clock_ref: "clk.apb",
    controller_ref: Some("block.system"),
    binding_kind: "gated",
    control_refs: &["reg.system.perip_clk_en0"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_UART0_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rstbind.uart0",
    name: "UART_RST",
    target_ref: "per.uart0",
    controller_ref: Some("block.system"),
    reset_domain_ref: Some("rst.system"),
    binding_kind: "local",
    control_refs: &["reg.system.perip_rst_en0"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_UART0_INTERRUPT_SOURCES: &[metadata::InterruptSource] =
    &[metadata::InterruptSource {
        id: "isrc.uart0",
        name: "UART0",
        source_ref: "per.uart0",
        producer_ref: Some("block.uart0"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    }];
pub const DRV_UART0_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.uart0",
    name: "UART0 interrupt matrix source",
    source_ref: "isrc.uart0",
    interrupt_ref: "irq.ets_uart0_intr_source",
    controller_ref: "block.interrupt_matrix0",
    cpu_target_ref: Some("block.cpu0"),
    line_index: None,
    route_type: "matrix",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_UART0_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_UART0_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_UART0_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.uart0.rx.gpio20",
    name: "UART0 RX on GPIO20",
    pin_ref: "pin.gpio20",
    peripheral_ref: "per.uart0",
    signal: "U0RXD",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_UART0_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.uart0.tx.gpio21",
    name: "UART0 TX on GPIO21",
    pin_ref: "pin.gpio21",
    peripheral_ref: "per.uart0",
    signal: "U0TXD",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_UART0_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "rx",
        signal: "U0RXD",
        routes: DRV_UART0_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "tx",
        signal: "U0TXD",
        routes: DRV_UART0_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
];
pub const DRV_UART0_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_UART0_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_UART0_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct Uart0Resources {
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

pub const DRV_UART0_RESOURCES: Uart0Resources = Uart0Resources {
    clocks: DRV_UART0_CLOCK_BINDINGS,
    resets: DRV_UART0_RESET_BINDINGS,
    interrupt_sources: DRV_UART0_INTERRUPT_SOURCES,
    interrupts: DRV_UART0_INTERRUPT_ROUTES,
    dma_channels: DRV_UART0_DMA_CHANNELS,
    dma: DRV_UART0_DMA_ROUTES,
    pins: DRV_UART0_PIN_ROLES,
    init_operations: DRV_UART0_INIT_OPERATIONS,
    state_machines: DRV_UART0_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_UART0_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Uart0 {
    resources: Uart0Resources,
}

impl Uart0 {
    pub fn new(resources: Uart0Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> Uart0Resources {
        self.resources
    }
    /// Enable the UART0 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the UART0 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for UART0.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for UART0.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable Uart0.
    pub fn enable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60000020u64, 0x02000000u32, 0x02000000u32)?;
        Ok(())
    }

    /// Disable Uart0.
    pub fn disable(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60000020u64, 0x02000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60000020u64, 0x00000002u32, 0x00000000u32)?;
        modify_u32(0x60000020u64, 0x00000001u32, 0x00000000u32)?;
        modify_u32(0x60000020u64, 0x0000000Cu32, 0x0000000Cu32)?;
        modify_u32(0x60000020u64, 0x00000030u32, 0x00000010u32)?;
        modify_u32(0x60000020u64, 0x10000000u32, 0x10000000u32)?;
        modify_u32(0x60000020u64, 0x02000000u32, 0x02000000u32)?;
        Ok(())
    }

    pub fn set_baud_divider(&self, divider: u16, fraction: u8) -> Result<(), metadata::Error> {
        if u32::from(divider) > 0xFFFu32 {
            return Err(metadata::Error::Unsupported(
                "UART baud divider exceeds modeled field width",
            ));
        }
        if u32::from(fraction) > 0xFu32 {
            return Err(metadata::Error::Unsupported(
                "UART baud fraction exceeds modeled field width",
            ));
        }
        modify_u32(
            0x60000014u64,
            0x00F00FFFu32,
            u32::from(divider) & 0xFFFu32 | (u32::from(fraction) & 0xFu32) << 20,
        )?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        write_u32(0x60000000u64, u32::from(byte))?;
        Ok(())
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<(), metadata::Error> {
        while ((read_u32(0x6000001Cu64)?) & 0x03FF0000u32) >> 16 != 0 {}
        Ok(())
    }

    pub fn read_byte(&self) -> Result<u8, metadata::Error> {
        while (read_u32(0x6000001Cu64)?) & 0x000003FFu32 == 0 {}
        Ok((read_u32(0x60000000u64)? & 0xFFu32) as u8)
    }

    /// Enable the Uart0 TXFIFO-empty interrupt.
    pub fn enable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x6000000Cu64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    /// Disable the Uart0 TXFIFO-empty interrupt.
    pub fn disable_txe_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x6000000Cu64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the Uart0 RXFIFO-full interrupt.
    pub fn enable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x6000000Cu64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the Uart0 RXFIFO-full interrupt.
    pub fn disable_rxne_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x6000000Cu64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }
}
