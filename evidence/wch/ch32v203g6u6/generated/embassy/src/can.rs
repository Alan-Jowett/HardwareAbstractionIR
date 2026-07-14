//! Generated Embassy-style can module for CH32V203G6U6.

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


use core::hint::spin_loop;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanError {
    Metadata(metadata::Error),
    Peripheral {
        kind: embedded_can::ErrorKind,
        detail: &'static str,
    },
    InvalidFrame(&'static str),
}

impl From<metadata::Error> for CanError {
    fn from(error: metadata::Error) -> Self {
        Self::Metadata(error)
    }
}

impl embedded_can::Error for CanError {
    fn kind(&self) -> embedded_can::ErrorKind {
        match *self {
            Self::Metadata(_) | Self::InvalidFrame(_) => embedded_can::ErrorKind::Other,
            Self::Peripheral { kind, .. } => kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanFrame {
    id: embedded_can::Id,
    data: [u8; 8],
    dlc: u8,
    is_remote: bool,
}

impl CanFrame {
    fn from_parts(
        id: embedded_can::Id,
        is_remote: bool,
        dlc: usize,
        data: [u8; 8],
    ) -> Result<Self, CanError> {
        if dlc > 8 {
            return Err(CanError::InvalidFrame("CAN DLC must be in the range 0..=8"));
        }
        Ok(Self {
            id,
            data: if is_remote { [0; 8] } else { data },
            dlc: dlc as u8,
            is_remote,
        })
    }

    fn data_words(&self) -> (u32, u32) {
        let bytes = self.data;
        (
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        )
    }
}

impl embedded_can::Frame for CanFrame {
    fn new(
        id: impl Into<embedded_can::Id>,
        data: &[u8],
    ) -> Option<Self> {
        if data.len() > 8 {
            return None;
        }
        let mut frame_data = [0u8; 8];
        frame_data[..data.len()].copy_from_slice(data);
        Some(Self {
            id: id.into(),
            data: frame_data,
            dlc: data.len() as u8,
            is_remote: false,
        })
    }

    fn new_remote(
        id: impl Into<embedded_can::Id>,
        dlc: usize,
    ) -> Option<Self> {
        (dlc <= 8).then_some(Self {
            id: id.into(),
            data: [0; 8],
            dlc: dlc as u8,
            is_remote: true,
        })
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, embedded_can::Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        self.is_remote
    }

    fn id(&self) -> embedded_can::Id {
        self.id
    }

    fn dlc(&self) -> usize {
        usize::from(self.dlc)
    }

    fn data(&self) -> &[u8] {
        let len = if self.is_remote {
            0
        } else {
            usize::from(self.dlc)
        };
        &self.data[..len]
    }
}

fn decode_can_error_kind(lec: u32, bus_off: bool, fifo_overrun: bool) -> embedded_can::ErrorKind {
    if fifo_overrun {
        return embedded_can::ErrorKind::Overrun;
    }
    if bus_off {
        return embedded_can::ErrorKind::Other;
    }
    match lec {
        1 => embedded_can::ErrorKind::Stuff,
        2 => embedded_can::ErrorKind::Form,
        3 => embedded_can::ErrorKind::Acknowledge,
        4 | 5 => embedded_can::ErrorKind::Bit,
        6 => embedded_can::ErrorKind::Crc,
        _ => embedded_can::ErrorKind::Other,
    }
}

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "can",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: CAN1 (can) from canonical block block.can1 -> can
pub const DRV_CAN1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.can1", name: "CAN1 clock binding", consumer_ref: "periph.can1", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_CAN1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.can1", name: "CAN1 reset binding", target_ref: "periph.can1", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_CAN1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.can1.tx", name: "CAN1 TX interrupt source", source_ref: "periph.can1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.can1.rx0", name: "CAN1 RX0 interrupt source", source_ref: "periph.can1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.can1.rx1", name: "CAN1 RX1 interrupt source", source_ref: "periph.can1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.can1.sce", name: "CAN1 SCE interrupt source", source_ref: "periph.can1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_CAN1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.can1.tx", name: "CAN1 TX interrupt route", source_ref: "isrc.can1.tx", interrupt_ref: "int.usbhpcan1tx", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.can1.rx0", name: "CAN1 RX0 interrupt route", source_ref: "isrc.can1.rx0", interrupt_ref: "int.usblpcan1rx0", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.can1.rx1", name: "CAN1 RX1 interrupt route", source_ref: "isrc.can1.rx1", interrupt_ref: "int.can1rx1", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.can1.sce", name: "CAN1 SCE interrupt route", source_ref: "isrc.can1.sce", interrupt_ref: "int.can1sce", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_CAN1_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_CAN1_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_CAN1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.can1.tx.pa12.r0", name: "CAN1 TX on PA12 (remap 0)", pin_ref: "pin.pa12", peripheral_ref: "periph.can1", signal: "TX", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.can1.tx.pd1.r3", name: "CAN1 TX on PD1 (remap 3)", pin_ref: "pin.pd1", peripheral_ref: "periph.can1", signal: "TX", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_CAN1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.can1.rx.pa11.r0", name: "CAN1 RX on PA11 (remap 0)", pin_ref: "pin.pa11", peripheral_ref: "periph.can1", signal: "RX", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }, metadata::PinRoute { id: "pinroute.can1.rx.pd0.r3", name: "CAN1 RX on PD0 (remap 3)", pin_ref: "pin.pd0", peripheral_ref: "periph.can1", signal: "RX", route_type: "selectable", control_refs: &["reg.afio.pcfr1"], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_CAN1_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "tx", signal: "TX", routes: DRV_CAN1_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "rx", signal: "RX", routes: DRV_CAN1_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_CAN1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_CAN1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_CAN1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct CAN1Resources {
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

pub const DRV_CAN1_RESOURCES: CAN1Resources = CAN1Resources {
    clocks: DRV_CAN1_CLOCK_BINDINGS,
    resets: DRV_CAN1_RESET_BINDINGS,
    interrupt_sources: DRV_CAN1_INTERRUPT_SOURCES,
    interrupts: DRV_CAN1_INTERRUPT_ROUTES,
    dma_channels: DRV_CAN1_DMA_CHANNELS,
    dma: DRV_CAN1_DMA_ROUTES,
    pins: DRV_CAN1_PIN_ROLES,
    init_operations: DRV_CAN1_INIT_OPERATIONS,
    state_machines: DRV_CAN1_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_CAN1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct CAN1 {
    resources: CAN1Resources,
}

impl CAN1 {
    pub fn new(resources: CAN1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> CAN1Resources {
        self.resources
    }
    /// Enable the CAN1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00800000u32, 0x00800000u32)?;
        Ok(())
    }

    /// Disable the CAN1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00800000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for CAN1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x02000000u32, 0x02000000u32)?;
        Ok(())
    }

    /// Release reset for CAN1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x02000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Configure the CAN TX route on PA12.
    pub fn configure_tx_pa12_route(&self) -> Result<(), CanError> {
        modify_u32(0x40010004u64, 0x00006000u32, 0x00000000u32)?;
        modify_u32(0x40010804u64, 0x000F0000u32, 0x000B0000u32)?;
        Ok(())
    }

    /// Configure the CAN TX route on PD1.
    pub fn configure_tx_pd1_route(&self) -> Result<(), CanError> {
        modify_u32(0x40010004u64, 0x00006000u32, 0x00006000u32)?;
        modify_u32(0x40011400u64, 0x000000F0u32, 0x000000B0u32)?;
        Ok(())
    }

    /// Configure the CAN RX route on PA11.
    pub fn configure_rx_pa11_route(&self) -> Result<(), CanError> {
        modify_u32(0x40010004u64, 0x00006000u32, 0x00000000u32)?;
        modify_u32(0x40010804u64, 0x0000F000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Configure the CAN RX route on PD0.
    pub fn configure_rx_pd0_route(&self) -> Result<(), CanError> {
        modify_u32(0x40010004u64, 0x00006000u32, 0x00006000u32)?;
        modify_u32(0x40011400u64, 0x0000000Fu32, 0x00000004u32)?;
        Ok(())
    }

    pub fn enter_initialization_mode(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000001u32 | 0x00000002u32, 0x00000001u32)?;
        self.wait_for_initialization_state(true)
    }

    pub fn leave_initialization_mode(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000001u32 | 0x00000002u32, 0x00000000u32)?;
        self.wait_for_initialization_state(false)
    }

    pub fn configure_tx_priority_by_identifier(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_tx_priority_by_request_order(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    pub fn enable_automatic_retransmission(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn disable_automatic_retransmission(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    pub fn enable_automatic_bus_off_management(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000040u32, 0x00000040u32)?;
        Ok(())
    }

    pub fn disable_automatic_bus_off_management(&self) -> Result<(), CanError> {
        modify_u32(0x40006400u64, 0x00000040u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_bit_timing(&self, brp: u16, ts1: u8, ts2: u8, sjw: u8) -> Result<(), CanError> {
        if u64::from(brp) > 0x3FFu64 {
            return Err(CanError::InvalidFrame("CAN BRP exceeds modeled field width"));
        }
        if u64::from(ts1) > 0xFu64 {
            return Err(CanError::InvalidFrame("CAN TS1 exceeds modeled field width"));
        }
        if u64::from(ts2) > 0x7u64 {
            return Err(CanError::InvalidFrame("CAN TS2 exceeds modeled field width"));
        }
        if u64::from(sjw) > 0x3u64 {
            return Err(CanError::InvalidFrame("CAN SJW exceeds modeled field width"));
        }
        let set_mask = ((u32::from(brp) << 0) & 0x000003FFu32)
            | ((u32::from(ts1) << 16) & 0x000F0000u32)
            | ((u32::from(ts2) << 20) & 0x00700000u32)
            | ((u32::from(sjw) << 24) & 0x03000000u32);
        modify_u32(0x4000641Cu64, 0x000003FFu32 | 0x000F0000u32 | 0x00700000u32 | 0x03000000u32, set_mask)?;
        Ok(())
    }

    pub fn enable_loopback(&self) -> Result<(), CanError> {
        modify_u32(0x4000641Cu64, 0x40000000u32, 0x40000000u32)?;
        Ok(())
    }

    pub fn disable_loopback(&self) -> Result<(), CanError> {
        modify_u32(0x4000641Cu64, 0x40000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn enable_silent(&self) -> Result<(), CanError> {
        modify_u32(0x4000641Cu64, 0x80000000u32, 0x80000000u32)?;
        Ok(())
    }

    pub fn disable_silent(&self) -> Result<(), CanError> {
        modify_u32(0x4000641Cu64, 0x80000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn enable_tx_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn disable_tx_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn enable_rx0_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00000002u32, 0x00000002u32)?;
        Ok(())
    }

    pub fn disable_rx0_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00000002u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn enable_rx1_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00000010u32, 0x00000010u32)?;
        Ok(())
    }

    pub fn disable_rx1_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00000010u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn enable_status_change_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00008000u32, 0x00008000u32)?;
        Ok(())
    }

    pub fn disable_status_change_interrupt(&self) -> Result<(), CanError> {
        modify_u32(0x40006414u64, 0x00008000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn pending_fifo0_frames(&self) -> Result<u8, CanError> {
        let value = (read_u32(0x4000640Cu64)? & 0x00000003u32) >> 0;
        Ok(value as u8)
    }

    pub fn pending_fifo1_frames(&self) -> Result<u8, CanError> {
        let value = (read_u32(0x40006410u64)? & 0x00000003u32) >> 0;
        Ok(value as u8)
    }

    pub fn last_error_kind(&self) -> Result<embedded_can::ErrorKind, CanError> {
        let errsr = read_u32(0x40006418u64)?;
        let fifo0 = read_u32(0x4000640Cu64)?;
        let fifo1 = read_u32(0x40006410u64)?;
        let lec = (errsr & 0x00000070u32) >> 4;
        let bus_off = (errsr & 0x00000004u32) != 0;
        let overrun = (fifo0 & 0x00000010u32) != 0 || (fifo1 & 0x00000010u32) != 0;
        Ok(decode_can_error_kind(lec, bus_off, overrun))
    }

    pub fn transmit_frame(&self, frame: &CanFrame) -> Result<(), CanError> {
        while (read_u32(0x40006408u64)? & 0x04000000u32) == 0 {
            if self.protocol_error_pending()? {
                let kind = self.last_error_kind()?;
                return Err(CanError::Peripheral { kind, detail: "CAN controller reported an error while waiting for mailbox 0" });
            }
            spin_loop();
        }
        let (identifier_register, data_length_register, data_low_register, data_high_register) = self.encode_frame_words(frame)?;
        write_u32(0x40006588u64, data_low_register)?;
        write_u32(0x4000658Cu64, data_high_register)?;
        write_u32(0x40006584u64, data_length_register)?;
        write_u32(0x40006580u64, identifier_register | 0x00000001u32)?;
        while (read_u32(0x40006408u64)? & 0x00000001u32) == 0 {
            if self.protocol_error_pending()? {
                let kind = self.last_error_kind()?;
                return Err(CanError::Peripheral { kind, detail: "CAN controller reported an error while transmitting on mailbox 0" });
            }
            spin_loop();
        }
        let status = read_u32(0x40006408u64)?;
        write_u32(0x40006408u64, 0x00000001u32)?;
        if (status & 0x00000002u32) != 0 {
            return Ok(());
        }
        if (status & 0x00000004u32) != 0 {
            return Err(CanError::Peripheral { kind: embedded_can::ErrorKind::Other, detail: "CAN arbitration was lost on mailbox 0" });
        }
        if (status & 0x00000008u32) != 0 {
            let kind = self.last_error_kind()?;
            return Err(CanError::Peripheral { kind, detail: "CAN transmission failed on mailbox 0" });
        }
        Err(CanError::Peripheral { kind: embedded_can::ErrorKind::Other, detail: "CAN mailbox 0 completed without TXOK0" })
    }

    pub fn receive_fifo0_frame(&self) -> Result<CanFrame, CanError> {
        loop {
            let fifo_status = read_u32(0x4000640Cu64)?;
            let pending = (fifo_status & 0x00000003u32) >> 0;
            if pending != 0 {
                let identifier_register = read_u32(0x400065B0u64)?;
                let data_length_register = read_u32(0x400065B4u64)?;
                let data_low_register = read_u32(0x400065B8u64)?;
                let data_high_register = read_u32(0x400065BCu64)?;
                let is_extended = (identifier_register & 0x00000004u32) != 0;
                let id = if is_extended {
                    let raw = (((identifier_register & 0xFFE00000u32) >> 21) << 18) | ((identifier_register & 0x001FFFF8u32) >> 3);
                    let id = embedded_can::ExtendedId::new(raw)
                        .ok_or(CanError::InvalidFrame("received CAN extended identifier exceeds 29 bits"))?;
                    embedded_can::Id::Extended(id)
                } else {
                    let raw = ((identifier_register & 0xFFE00000u32) >> 21) as u16;
                    let id = embedded_can::StandardId::new(raw)
                        .ok_or(CanError::InvalidFrame("received CAN standard identifier exceeds 11 bits"))?;
                    embedded_can::Id::Standard(id)
                };
                let is_remote = (identifier_register & 0x00000002u32) != 0;
                let dlc = ((data_length_register & 0x0000000Fu32) >> 0) as usize;
                let low_bytes = data_low_register.to_le_bytes();
                let high_bytes = data_high_register.to_le_bytes();
                let mut data = [0u8; 8];
                data[..4].copy_from_slice(&low_bytes);
                data[4..].copy_from_slice(&high_bytes);
                let frame = CanFrame::from_parts(id, is_remote, dlc, data);
                write_u32(0x4000640Cu64, 0x00000020u32)?;
                return frame;
            }
            if (fifo_status & 0x00000010u32) != 0 {
                let kind = self.last_error_kind()?;
                return Err(CanError::Peripheral { kind, detail: "CAN FIFO0 overrun" });
            }
            if self.protocol_error_pending()? {
                let kind = self.last_error_kind()?;
                return Err(CanError::Peripheral { kind, detail: "CAN controller reported an error while waiting on FIFO0" });
            }
            spin_loop();
        }
    }

    pub fn receive_fifo1_frame(&self) -> Result<CanFrame, CanError> {
        loop {
            let fifo_status = read_u32(0x40006410u64)?;
            let pending = (fifo_status & 0x00000003u32) >> 0;
            if pending != 0 {
                let identifier_register = read_u32(0x400065C0u64)?;
                let data_length_register = read_u32(0x400065C4u64)?;
                let data_low_register = read_u32(0x400065C8u64)?;
                let data_high_register = read_u32(0x400065CCu64)?;
                let is_extended = (identifier_register & 0x00000004u32) != 0;
                let id = if is_extended {
                    let raw = (((identifier_register & 0xFFE00000u32) >> 21) << 18) | ((identifier_register & 0x001FFFF8u32) >> 3);
                    let id = embedded_can::ExtendedId::new(raw)
                        .ok_or(CanError::InvalidFrame("received CAN extended identifier exceeds 29 bits"))?;
                    embedded_can::Id::Extended(id)
                } else {
                    let raw = ((identifier_register & 0xFFE00000u32) >> 21) as u16;
                    let id = embedded_can::StandardId::new(raw)
                        .ok_or(CanError::InvalidFrame("received CAN standard identifier exceeds 11 bits"))?;
                    embedded_can::Id::Standard(id)
                };
                let is_remote = (identifier_register & 0x00000002u32) != 0;
                let dlc = ((data_length_register & 0x0000000Fu32) >> 0) as usize;
                let low_bytes = data_low_register.to_le_bytes();
                let high_bytes = data_high_register.to_le_bytes();
                let mut data = [0u8; 8];
                data[..4].copy_from_slice(&low_bytes);
                data[4..].copy_from_slice(&high_bytes);
                let frame = CanFrame::from_parts(id, is_remote, dlc, data);
                write_u32(0x40006410u64, 0x00000020u32)?;
                return frame;
            }
            if (fifo_status & 0x00000010u32) != 0 {
                let kind = self.last_error_kind()?;
                return Err(CanError::Peripheral { kind, detail: "CAN FIFO1 overrun" });
            }
            if self.protocol_error_pending()? {
                let kind = self.last_error_kind()?;
                return Err(CanError::Peripheral { kind, detail: "CAN controller reported an error while waiting on FIFO1" });
            }
            spin_loop();
        }
    }

    pub fn enter_filter_init_mode(&self) -> Result<(), CanError> {
        modify_u32(0x40006600u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    pub fn leave_filter_init_mode(&self) -> Result<(), CanError> {
        modify_u32(0x40006600u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_filter_bank_mask_mode(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x40006604u64, mask & 0x0FFFFFFFu32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_filter_bank_list_mode(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x40006604u64, mask & 0x0FFFFFFFu32, mask & 0x0FFFFFFFu32)?;
        Ok(())
    }

    pub fn configure_filter_bank_scale_16bit(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x4000660Cu64, mask & 0x0FFFFFFFu32, 0x00000000u32)?;
        Ok(())
    }

    pub fn configure_filter_bank_scale_32bit(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x4000660Cu64, mask & 0x0FFFFFFFu32, mask & 0x0FFFFFFFu32)?;
        Ok(())
    }

    pub fn assign_filter_bank_to_fifo0(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x40006614u64, mask & 0x0FFFFFFFu32, 0x00000000u32)?;
        Ok(())
    }

    pub fn assign_filter_bank_to_fifo1(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x40006614u64, mask & 0x0FFFFFFFu32, mask & 0x0FFFFFFFu32)?;
        Ok(())
    }

    pub fn enable_filter_bank(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x4000661Cu64, mask & 0x0FFFFFFFu32, mask & 0x0FFFFFFFu32)?;
        Ok(())
    }

    pub fn disable_filter_bank(&self, filter_bank: u8) -> Result<(), CanError> {
        let mask = self.filter_bank_mask(filter_bank)?;
        modify_u32(0x4000661Cu64, mask & 0x0FFFFFFFu32, 0x00000000u32)?;
        Ok(())
    }

    pub fn write_filter_bank_words(&self, filter_bank: u8, fr1: u32, fr2: u32) -> Result<(), CanError> {
        let offset = self.filter_bank_offset(filter_bank)?;
        write_u32(0x40006640u64 + offset, fr1)?;
        write_u32(0x40006644u64 + offset, fr2)?;
        Ok(())
    }

    fn wait_for_initialization_state(&self, expected: bool) -> Result<(), CanError> {
        loop {
            let status = read_u32(0x40006404u64)?;
            let active = (status & 0x00000001u32) != 0;
            if active == expected {
                return Ok(());
            }
            spin_loop();
        }
    }

    fn protocol_error_pending(&self) -> Result<bool, CanError> {
        Ok((read_u32(0x40006404u64)? & 0x00000004u32) != 0)
    }

    fn filter_bank_mask(&self, filter_bank: u8) -> Result<u32, CanError> {
        if usize::from(filter_bank) >= 28 {
            return Err(CanError::InvalidFrame("CAN filter bank index exceeds modeled extent"));
        }
        Ok(1u32 << u32::from(filter_bank))
    }

    fn filter_bank_offset(&self, filter_bank: u8) -> Result<u64, CanError> {
        Ok(u64::from(filter_bank) * 8)
    }

    fn encode_frame_words(&self, frame: &CanFrame) -> Result<(u32, u32, u32, u32), CanError> {
        let mut identifier_register = 0u32;
        match embedded_can::Frame::id(frame) {
            embedded_can::Id::Standard(id) => {
                let raw = u32::from(id.as_raw());
                if u64::from(raw) > 0x7FFu64 {
                    return Err(CanError::InvalidFrame("CAN standard identifier exceeds modeled field width"));
                }
                identifier_register |= (raw << 21) & 0xFFE00000u32;
            }
            embedded_can::Id::Extended(id) => {
                let raw = id.as_raw();
                let standard_component = raw >> 18;
                let extended_component = raw & 0x3FFFFu32;
                if u64::from(standard_component) > 0x7FFu64 {
                    return Err(CanError::InvalidFrame("CAN extended identifier standard component exceeds modeled field width"));
                }
                identifier_register |= (standard_component << 21) & 0xFFE00000u32;
                identifier_register |= (extended_component << 3) & 0x001FFFF8u32;
                identifier_register |= 0x00000004u32;
            }
        }
        if embedded_can::Frame::is_remote_frame(frame) {
            identifier_register |= 0x00000002u32;
        }
        let dlc = embedded_can::Frame::dlc(frame);
        if dlc > 8 || dlc > 0xFusize {
            return Err(CanError::InvalidFrame("CAN DLC exceeds modeled field width"));
        }
        let data_length_register = ((dlc as u32) << 0) & 0x0000000Fu32;
        let (data_low_register, data_high_register) = frame.data_words();
        Ok((identifier_register, data_length_register, data_low_register, data_high_register))
    }
}


impl embedded_can::blocking::Can for CAN1 {
    type Frame = CanFrame;
    type Error = CanError;

    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        self.transmit_frame(frame)
    }

    fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        self.receive_fifo0_frame()
    }
}
