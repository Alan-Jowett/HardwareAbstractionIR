//! Generated Embassy-style time module for CH32V203G6U6.

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
    module_name: "time",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: RTC Embassy time driver (rtc) from canonical block block.rtc -> rtc-controller
pub const DRV_TIME_RTC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[
    metadata::ClockBinding {
        id: "clk.pwr",
        name: "PWR clock binding",
        consumer_ref: "periph.pwr",
        clock_ref: "clk.pclk1",
        controller_ref: Some("block.rcc"),
        binding_kind: "gated",
        control_refs: &["reg.rcc.apb1pcenr"],
        enable_operation_refs: &[],
        disable_operation_refs: &[],
    },
    metadata::ClockBinding {
        id: "clk.bkp",
        name: "BKP clock binding",
        consumer_ref: "periph.bkp",
        clock_ref: "clk.pclk1",
        controller_ref: Some("block.rcc"),
        binding_kind: "gated",
        control_refs: &["reg.rcc.apb1pcenr"],
        enable_operation_refs: &[],
        disable_operation_refs: &[],
    },
];
pub const DRV_TIME_RTC_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_TIME_RTC_INTERRUPT_SOURCES: &[metadata::InterruptSource] =
    &[metadata::InterruptSource {
        id: "isrc.rtc.alarm",
        name: "RTC alarm interrupt source",
        source_ref: "periph.rtc",
        producer_ref: None,
        kind: "rtc",
        flag_refs: &[],
        clear_operation_refs: &["op.rtc.clear_alarm_flag"],
    }];
pub const DRV_TIME_RTC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] =
    &[metadata::InterruptRoute {
        id: "iroute.rtc.alarm",
        name: "RTC alarm interrupt route",
        source_ref: "isrc.rtc.alarm",
        interrupt_ref: "int.rtcalarm",
        controller_ref: "block.pfic",
        cpu_target_ref: None,
        line_index: None,
        route_type: "hardwired",
        control_refs: &["field.exti_intenr.mr17", "field.exti_rtenr.tr17"],
        acknowledge_operation_refs: &["op.exti.clear_line17_pending"],
        shared_group: None,
    }];
pub const DRV_TIME_RTC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIME_RTC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIME_RTC_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_TIME_RTC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_TIME_RTC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_TIME_RTC_CAPABILITY_TAGS: &[&str] = &["embassy-time-driver"];

#[derive(Debug, Clone, Copy)]
pub struct RTCEmbassyTimeDriverResources {
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

pub const DRV_TIME_RTC_RESOURCES: RTCEmbassyTimeDriverResources = RTCEmbassyTimeDriverResources {
    clocks: DRV_TIME_RTC_CLOCK_BINDINGS,
    resets: DRV_TIME_RTC_RESET_BINDINGS,
    interrupt_sources: DRV_TIME_RTC_INTERRUPT_SOURCES,
    interrupts: DRV_TIME_RTC_INTERRUPT_ROUTES,
    dma_channels: DRV_TIME_RTC_DMA_CHANNELS,
    dma: DRV_TIME_RTC_DMA_ROUTES,
    pins: DRV_TIME_RTC_PIN_ROLES,
    init_operations: DRV_TIME_RTC_INIT_OPERATIONS,
    state_machines: DRV_TIME_RTC_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: Some("rtc"),
    capability_tags: DRV_TIME_RTC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct RTCEmbassyTimeDriver {
    resources: RTCEmbassyTimeDriverResources,
}

impl RTCEmbassyTimeDriver {
    pub fn new(resources: RTCEmbassyTimeDriverResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> RTCEmbassyTimeDriverResources {
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

    pub fn init_time_driver(&self) -> Result<(), metadata::Error> {
        self.enable_pwr_clock()?;
        self.enable_bkp_clock()?;
        initialize_drv_time_rtc_time_driver()
    }

    pub fn now_ticks(&self) -> u64 {
        generated_drv_time_rtc_time_driver_now()
    }

    pub fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {
        generated_drv_time_rtc_time_driver_delay_ticks(ticks)
    }

    pub fn on_time_driver_interrupt(&self) {
        generated_drv_time_rtc_time_driver_interrupt();
    }
}

use core::cell::{Cell, RefCell};
use critical_section::Mutex as CriticalSectionMutex;
use embassy_time_driver::Driver as EmbassyTimeDriver;
use embassy_time_queue_utils::Queue as EmbassyTimeQueue;

const GENERATED_RTC_CNTH_ADDRESS: u64 = 0x40002818u64;
const GENERATED_RTC_CNTL_ADDRESS: u64 = 0x4000281Cu64;
const GENERATED_RTC_ALRMH_ADDRESS: u64 = 0x40002820u64;
const GENERATED_RTC_ALRML_ADDRESS: u64 = 0x40002824u64;
const GENERATED_RTC_PSCRH_ADDRESS: u64 = 0x40002808u64;
const GENERATED_RTC_PSCRL_ADDRESS: u64 = 0x4000280Cu64;
const GENERATED_RTC_CTLRH_ADDRESS: u64 = 0x40002800u64;
const GENERATED_RTC_CTLRH_ALRIE_MASK: u16 = 0x0002u16;
const GENERATED_RTC_CTLRL_ADDRESS: u64 = 0x40002804u64;
const GENERATED_RTC_CTLRL_ALRF_MASK: u16 = 0x0002u16;
const GENERATED_RTC_CTLRL_CNF_MASK: u16 = 0x0010u16;
const GENERATED_RTC_CTLRL_RTOFF_MASK: u16 = 0x0020u16;
const GENERATED_RTC_CTLRL_RSF_MASK: u16 = 0x0008u16;
const GENERATED_RTC_TICK_HZ: u64 = 1000u64;
const GENERATED_RTC_PRESCALER_RELOAD: u32 = 39u32;
const GENERATED_RCC_RSTSCKR_ADDRESS: u64 = 0x40021024u64;
const GENERATED_RCC_RSTSCKR_LSION_MASK: u32 = 0x00000001u32;
const GENERATED_RCC_RSTSCKR_LSIRDY_MASK: u32 = 0x00000002u32;
const GENERATED_RCC_BDCTLR_ADDRESS: u64 = 0x40021020u64;
const GENERATED_RCC_BDCTLR_RTCSEL_MASK: u32 = 0x00000300u32;
const GENERATED_RCC_BDCTLR_RTCSEL_LSI_MASK: u32 = 0x00000200u32;
const GENERATED_RCC_BDCTLR_RTCEN_MASK: u32 = 0x00008000u32;
const GENERATED_RCC_BDCTLR_BDRST_MASK: u32 = 0x00010000u32;
const GENERATED_PWR_CTLR_ADDRESS: u64 = 0x40007000u64;
const GENERATED_PWR_CTLR_DBP_MASK: u32 = 0x00000100u32;

struct GeneratedRtcTimeDriver {
    initialized: CriticalSectionMutex<Cell<bool>>,
    wraps: CriticalSectionMutex<Cell<u64>>,
    last_raw: CriticalSectionMutex<Cell<u32>>,
    queue: CriticalSectionMutex<RefCell<EmbassyTimeQueue>>,
}

impl GeneratedRtcTimeDriver {
    const fn new() -> Self {
        Self {
            initialized: CriticalSectionMutex::new(Cell::new(false)),
            wraps: CriticalSectionMutex::new(Cell::new(0)),
            last_raw: CriticalSectionMutex::new(Cell::new(0)),
            queue: CriticalSectionMutex::new(RefCell::new(EmbassyTimeQueue::new())),
        }
    }

    fn wait_for_lsi_ready(&self) {
        must_modify_u32(
            GENERATED_RCC_RSTSCKR_ADDRESS,
            GENERATED_RCC_RSTSCKR_LSION_MASK,
            GENERATED_RCC_RSTSCKR_LSION_MASK,
        );
        while (must_read_u32(GENERATED_RCC_RSTSCKR_ADDRESS) & GENERATED_RCC_RSTSCKR_LSIRDY_MASK)
            == 0
        {
            core::hint::spin_loop();
        }
    }

    fn wait_for_rtc_write_ready(&self) {
        while (must_read_u16(GENERATED_RTC_CTLRL_ADDRESS) & GENERATED_RTC_CTLRL_RTOFF_MASK) == 0 {
            core::hint::spin_loop();
        }
    }

    fn synchronize_rtc_registers(&self) {
        must_modify_u16(
            GENERATED_RTC_CTLRL_ADDRESS,
            GENERATED_RTC_CTLRL_RSF_MASK,
            0u16,
        );
        while (must_read_u16(GENERATED_RTC_CTLRL_ADDRESS) & GENERATED_RTC_CTLRL_RSF_MASK) == 0 {
            core::hint::spin_loop();
        }
    }

    fn enter_config_mode(&self) {
        self.wait_for_rtc_write_ready();
        must_modify_u16(
            GENERATED_RTC_CTLRL_ADDRESS,
            GENERATED_RTC_CTLRL_CNF_MASK,
            GENERATED_RTC_CTLRL_CNF_MASK,
        );
    }

    fn exit_config_mode(&self) {
        must_modify_u16(
            GENERATED_RTC_CTLRL_ADDRESS,
            GENERATED_RTC_CTLRL_CNF_MASK,
            0u16,
        );
        self.wait_for_rtc_write_ready();
    }

    fn read_split32(&self, high_address: u64, low_address: u64) -> u32 {
        loop {
            let high_1 = u32::from(must_read_u16(high_address));
            let low = u32::from(must_read_u16(low_address));
            let high_2 = u32::from(must_read_u16(high_address));
            if high_1 == high_2 {
                return (high_1 << 16) | low;
            }
        }
    }

    fn write_split32(&self, high_address: u64, low_address: u64, value: u32) {
        must_write_u16(high_address, (value >> 16) as u16);
        must_write_u16(low_address, value as u16);
    }

    fn read_raw_counter(&self) -> u32 {
        self.read_split32(GENERATED_RTC_CNTH_ADDRESS, GENERATED_RTC_CNTL_ADDRESS)
    }

    fn read_now(&self) -> u64 {
        critical_section::with(|cs| {
            let raw = self.read_raw_counter();
            let last = self.last_raw.borrow(cs).get();
            let mut wraps = self.wraps.borrow(cs).get();
            if raw < last {
                wraps = wraps.wrapping_add(1);
                self.wraps.borrow(cs).set(wraps);
            }
            self.last_raw.borrow(cs).set(raw);
            (wraps << 32) | u64::from(raw)
        })
    }

    fn set_alarm_enabled(&self, enabled: bool) {
        let set_mask = if enabled {
            GENERATED_RTC_CTLRH_ALRIE_MASK
        } else {
            0u16
        };
        must_modify_u16(
            GENERATED_RTC_CTLRH_ADDRESS,
            GENERATED_RTC_CTLRH_ALRIE_MASK,
            set_mask,
        );
    }

    fn clear_alarm_flag(&self) {
        must_modify_u16(0x40002804u64, 0x0002u16, 0x0000u16);
        must_modify_u32(0x40010414u64, 0x00020000u32, 0x00020000u32);
    }

    fn is_alarm_pending(&self) -> bool {
        (must_read_u16(GENERATED_RTC_CTLRL_ADDRESS) & GENERATED_RTC_CTLRL_ALRF_MASK) != 0
    }

    fn arm_alarm(&self, at: u64) {
        self.enter_config_mode();
        self.write_split32(
            GENERATED_RTC_ALRMH_ADDRESS,
            GENERATED_RTC_ALRML_ADDRESS,
            at as u32,
        );
        self.exit_config_mode();
        self.clear_alarm_flag();
        self.set_alarm_enabled(true);
    }

    fn init(&self) -> Result<(), metadata::Error> {
        critical_section::with(|cs| {
            if self.initialized.borrow(cs).get() {
                return Ok(());
            }
            self.wait_for_lsi_ready();
            must_modify_u32(
                GENERATED_PWR_CTLR_ADDRESS,
                GENERATED_PWR_CTLR_DBP_MASK,
                GENERATED_PWR_CTLR_DBP_MASK,
            );
            must_modify_u32(
                GENERATED_RCC_BDCTLR_ADDRESS,
                GENERATED_RCC_BDCTLR_BDRST_MASK,
                GENERATED_RCC_BDCTLR_BDRST_MASK,
            );
            must_modify_u32(
                GENERATED_RCC_BDCTLR_ADDRESS,
                GENERATED_RCC_BDCTLR_BDRST_MASK,
                0u32,
            );
            must_modify_u32(
                GENERATED_RCC_BDCTLR_ADDRESS,
                GENERATED_RCC_BDCTLR_RTCSEL_MASK,
                GENERATED_RCC_BDCTLR_RTCSEL_LSI_MASK,
            );
            must_modify_u32(
                GENERATED_RCC_BDCTLR_ADDRESS,
                GENERATED_RCC_BDCTLR_RTCEN_MASK,
                GENERATED_RCC_BDCTLR_RTCEN_MASK,
            );
            must_modify_u32(0x40010400u64, 0x00020000u32, 0x00020000u32);
            must_modify_u32(0x40010408u64, 0x00020000u32, 0x00020000u32);
            self.synchronize_rtc_registers();
            self.enter_config_mode();
            self.write_split32(
                GENERATED_RTC_PSCRH_ADDRESS,
                GENERATED_RTC_PSCRL_ADDRESS,
                GENERATED_RTC_PRESCALER_RELOAD,
            );
            self.write_split32(GENERATED_RTC_CNTH_ADDRESS, GENERATED_RTC_CNTL_ADDRESS, 0u32);
            self.write_split32(
                GENERATED_RTC_ALRMH_ADDRESS,
                GENERATED_RTC_ALRML_ADDRESS,
                u32::MAX,
            );
            self.exit_config_mode();
            self.clear_alarm_flag();
            self.set_alarm_enabled(false);
            self.wraps.borrow(cs).set(0);
            self.last_raw.borrow(cs).set(self.read_raw_counter());
            self.initialized.borrow(cs).set(true);
            Ok(())
        })
    }

    fn on_interrupt(&self) {
        if !critical_section::with(|cs| self.initialized.borrow(cs).get())
            || !self.is_alarm_pending()
        {
            return;
        }
        self.clear_alarm_flag();
        let now = self.read_now();
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            let next = queue.next_expiration(now);
            if next == u64::MAX {
                self.set_alarm_enabled(false);
            } else {
                self.arm_alarm(next);
            }
        });
    }

    fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {
        let start = self.read_now();
        let deadline = start.wrapping_add(ticks);
        loop {
            let now = self.read_now();
            if now.wrapping_sub(deadline) < (1u64 << 63) {
                break;
            }
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl EmbassyTimeDriver for GeneratedRtcTimeDriver {
    fn now(&self) -> u64 {
        self.read_now()
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        let should_rearm = critical_section::with(|cs| {
            self.queue.borrow(cs).borrow_mut().schedule_wake(at, waker)
        });
        if !should_rearm {
            return;
        }
        let now = self.read_now();
        critical_section::with(|cs| {
            let next = self.queue.borrow(cs).borrow_mut().next_expiration(now);
            if next == u64::MAX {
                self.set_alarm_enabled(false);
            } else {
                self.arm_alarm(next);
            }
        });
    }
}

#[allow(dead_code)]
fn must_read_u16(address: u64) -> u16 {
    read_u16(address).expect("generated rtc time-driver MMIO read")
}

#[allow(dead_code)]
fn must_read_u32(address: u64) -> u32 {
    read_u32(address).expect("generated rtc time-driver MMIO read")
}

#[allow(dead_code)]
fn must_modify_u16(address: u64, clear_mask: u16, set_mask: u16) {
    modify_u16(address, clear_mask, set_mask).expect("generated rtc time-driver MMIO write")
}

#[allow(dead_code)]
fn must_modify_u32(address: u64, clear_mask: u32, set_mask: u32) {
    modify_u32(address, clear_mask, set_mask).expect("generated rtc time-driver MMIO write")
}

#[allow(dead_code)]
fn must_write_u16(address: u64, value: u16) {
    write_u16(address, value).expect("generated rtc time-driver MMIO write")
}

embassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedRtcTimeDriver = GeneratedRtcTimeDriver::new());

#[allow(dead_code)]
pub fn generated_drv_time_rtc_time_driver_interrupt() {
    GENERATED_TIME_DRIVER.on_interrupt();
}

fn initialize_drv_time_rtc_time_driver() -> Result<(), metadata::Error> {
    let _ = GENERATED_RTC_TICK_HZ;
    GENERATED_TIME_DRIVER.init()
}

fn generated_drv_time_rtc_time_driver_now() -> u64 {
    GENERATED_TIME_DRIVER.now()
}

fn generated_drv_time_rtc_time_driver_delay_ticks(ticks: u64) -> Result<(), metadata::Error> {
    GENERATED_TIME_DRIVER.delay_ticks(ticks)
}
