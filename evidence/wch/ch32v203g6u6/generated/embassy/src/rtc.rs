//! Generated Embassy-style rtc module for CH32V203G6U6.

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
    module_name: "rtc",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: RTC (rtc) from canonical block block.rtc -> rtc-controller
pub const DRV_RTC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.pwr", name: "PWR clock binding", consumer_ref: "periph.pwr", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }, metadata::ClockBinding { id: "clk.bkp", name: "BKP clock binding", consumer_ref: "periph.bkp", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_RTC_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_RTC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.rtc.global", name: "RTC global interrupt source", source_ref: "periph.rtc", producer_ref: None, kind: "rtc", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.rtc.alarm", name: "RTC alarm interrupt source", source_ref: "periph.rtc", producer_ref: None, kind: "rtc", flag_refs: &[], clear_operation_refs: &["op.rtc.clear_alarm_flag"] }];
pub const DRV_RTC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.rtc.global", name: "RTC global interrupt route", source_ref: "isrc.rtc.global", interrupt_ref: "int.rtc", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.rtc.alarm", name: "RTC alarm interrupt route", source_ref: "isrc.rtc.alarm", interrupt_ref: "int.rtcalarm", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &["field.exti_intenr.mr17", "field.exti_rtenr.tr17"], acknowledge_operation_refs: &["op.exti.clear_line17_pending"], shared_group: None }];
pub const DRV_RTC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_RTC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_RTC_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_RTC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_RTC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_RTC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct RTCResources {
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

pub const DRV_RTC_RESOURCES: RTCResources = RTCResources {
    clocks: DRV_RTC_CLOCK_BINDINGS,
    resets: DRV_RTC_RESET_BINDINGS,
    interrupt_sources: DRV_RTC_INTERRUPT_SOURCES,
    interrupts: DRV_RTC_INTERRUPT_ROUTES,
    dma_channels: DRV_RTC_DMA_CHANNELS,
    dma: DRV_RTC_DMA_ROUTES,
    pins: DRV_RTC_PIN_ROLES,
    init_operations: DRV_RTC_INIT_OPERATIONS,
    state_machines: DRV_RTC_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_RTC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct RTC {
    resources: RTCResources,
}

impl RTC {
    pub fn new(resources: RTCResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> RTCResources {
        self.resources
    }
    /// Enable the PWR clock gate.
    pub fn enable_pwr_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x10000000u32, 0x10000000u32)?;
        Ok(())
    }

    /// Disable the PWR clock gate.
    pub fn disable_pwr_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x10000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the BKP clock gate.
    pub fn enable_bkp_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x08000000u32, 0x08000000u32)?;
        Ok(())
    }

    /// Disable the BKP clock gate.
    pub fn disable_bkp_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x08000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn read_counter(&self) -> Result<u32, metadata::Error> {
        read_generated_rtc_split32(GENERATED_RTC_CNTH_ADDRESS, GENERATED_RTC_CNTL_ADDRESS)
    }

    pub fn set_counter(&self, value: u32) -> Result<(), metadata::Error> {
        write_generated_rtc_split32_with_config(
            GENERATED_RTC_CNTH_ADDRESS,
            GENERATED_RTC_CNTL_ADDRESS,
            value,
        )
    }

    pub fn read_prescaler(&self) -> Result<u32, metadata::Error> {
        read_generated_rtc_split32(GENERATED_RTC_PSCRH_ADDRESS, GENERATED_RTC_PSCRL_ADDRESS)
    }

    pub fn set_prescaler(&self, value: u32) -> Result<(), metadata::Error> {
        write_generated_rtc_split32_with_config(
            GENERATED_RTC_PSCRH_ADDRESS,
            GENERATED_RTC_PSCRL_ADDRESS,
            value,
        )
    }

    pub fn read_alarm(&self) -> Result<u32, metadata::Error> {
        read_generated_rtc_split32(GENERATED_RTC_ALRMH_ADDRESS, GENERATED_RTC_ALRML_ADDRESS)
    }

    pub fn set_alarm(&self, value: u32) -> Result<(), metadata::Error> {
        write_generated_rtc_split32_with_config(
            GENERATED_RTC_ALRMH_ADDRESS,
            GENERATED_RTC_ALRML_ADDRESS,
            value,
        )
    }

    pub fn enable_second_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002800u64, 0x0001u16, 0x0001u16)
    }

    pub fn disable_second_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002800u64, 0x0001u16, 0x0000u16)
    }

    pub fn enable_alarm_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002800u64, 0x0002u16, 0x0002u16)
    }

    pub fn disable_alarm_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002800u64, 0x0002u16, 0x0000u16)
    }

    pub fn enable_overflow_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002800u64, 0x0004u16, 0x0004u16)
    }

    pub fn disable_overflow_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002800u64, 0x0004u16, 0x0000u16)
    }

    pub fn is_second_flag_set(&self) -> Result<bool, metadata::Error> {
        Ok((read_u16(0x40002804u64)? & 0x0001u16) != 0)
    }

    pub fn clear_second_flag(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002804u64, 0x0001u16, 0x0000u16)
    }

    pub fn is_alarm_flag_set(&self) -> Result<bool, metadata::Error> {
        Ok((read_u16(0x40002804u64)? & 0x0002u16) != 0)
    }

    pub fn clear_alarm_flag(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002804u64, 0x0002u16, 0x0000u16)
    }

    pub fn is_overflow_flag_set(&self) -> Result<bool, metadata::Error> {
        Ok((read_u16(0x40002804u64)? & 0x0004u16) != 0)
    }

    pub fn clear_overflow_flag(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40002804u64, 0x0004u16, 0x0000u16)
    }
}


const GENERATED_RTC_CTLRL_ADDRESS: u64 = 0x40002804u64;
const GENERATED_RTC_CTLRL_CNF_MASK: u16 = 0x0010u16;
const GENERATED_RTC_CTLRL_RTOFF_MASK: u16 = 0x0020u16;
const GENERATED_RTC_PSCRH_ADDRESS: u64 = 0x40002808u64;
const GENERATED_RTC_PSCRL_ADDRESS: u64 = 0x4000280Cu64;
const GENERATED_RTC_CNTH_ADDRESS: u64 = 0x40002818u64;
const GENERATED_RTC_CNTL_ADDRESS: u64 = 0x4000281Cu64;
const GENERATED_RTC_ALRMH_ADDRESS: u64 = 0x40002820u64;
const GENERATED_RTC_ALRML_ADDRESS: u64 = 0x40002824u64;

fn wait_generated_rtc_write_ready() -> Result<(), metadata::Error> {
    while (read_u16(GENERATED_RTC_CTLRL_ADDRESS)? & GENERATED_RTC_CTLRL_RTOFF_MASK) == 0 {
        core::hint::spin_loop();
    }
    Ok(())
}

fn read_generated_rtc_split32(high_address: u64, low_address: u64) -> Result<u32, metadata::Error> {
    loop {
        let high_1 = u32::from(read_u16(high_address)?);
        let low = u32::from(read_u16(low_address)?);
        let high_2 = u32::from(read_u16(high_address)?);
        if high_1 == high_2 {
            return Ok((high_1 << 16) | low);
        }
    }
}

fn write_generated_rtc_split32_with_config(
    high_address: u64,
    low_address: u64,
    value: u32,
) -> Result<(), metadata::Error> {
    wait_generated_rtc_write_ready()?;
    modify_u16(
        GENERATED_RTC_CTLRL_ADDRESS,
        GENERATED_RTC_CTLRL_CNF_MASK,
        GENERATED_RTC_CTLRL_CNF_MASK,
    )?;
    write_u16(high_address, (value >> 16) as u16)?;
    write_u16(low_address, value as u16)?;
    modify_u16(GENERATED_RTC_CTLRL_ADDRESS, GENERATED_RTC_CTLRL_CNF_MASK, 0u16)?;
    wait_generated_rtc_write_ready()
}
