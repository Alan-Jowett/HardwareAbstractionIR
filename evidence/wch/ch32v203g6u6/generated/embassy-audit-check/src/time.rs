//! Generated Embassy-style time module for CH32V203G6U6.

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
    module_name: "time",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: TIM4 Embassy time base (timer) from canonical block block.tim4 -> timer-general
pub const DRV_TIM4_TIME_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.tim4", name: "TIM4 clock binding", consumer_ref: "periph.tim4", clock_ref: "clk.pclk1-tim", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_TIM4_TIME_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.tim4", name: "TIM4 reset binding", target_ref: "periph.tim4", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_TIM4_TIME_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.tim4.cc", name: "TIM4 CC interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &["op.tim4.clear_cc1"] }];
pub const DRV_TIM4_TIME_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.tim4.cc", name: "TIM4 CC interrupt route", source_ref: "isrc.tim4.cc", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_TIM4_TIME_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIM4_TIME_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIM4_TIME_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_TIM4_TIME_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.tim4.configure_counter_compare_timebase", name: "Configure TIM4 counter/compare time base", description: None, kind: Some("initialization"), target_refs: &["periph.tim4"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim4.psc"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Write PSC = 7999" }), value: None, description: Some("Divide the reset-default 8 MHz timer clock down to a 1 kHz timer tick.") }, metadata::SemanticOperationStep { index: 1, action: "write", target_ref: Some("reg.tim4.atrlr"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Write ARR = 65535" }), value: None, description: Some("Run the free-running counter across the full 16-bit range.") }, metadata::SemanticOperationStep { index: 2, action: "write", target_ref: Some("reg.tim4.cnt"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Write CNT = 0" }), value: None, description: Some("Start the time base from zero on initialization.") }, metadata::SemanticOperationStep { index: 3, action: "write", target_ref: Some("reg.tim4.ch1cvr"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Write CCR1 = 0" }), value: None, description: Some("Initialize the compare register before arming alarms.") }, metadata::SemanticOperationStep { index: 4, action: "write", target_ref: Some("reg.tim4.swevgr"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set UG = 1" }), value: None, description: Some("Apply the prescaler and reload configuration immediately.") }], preconditions: &[], postconditions: &[] }, metadata::SemanticOperation { id: "op.tim4.enable", name: "TIM4 counter enable", description: None, kind: Some("mode-transition"), target_refs: &["periph.tim4"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.tim4.ctlr1"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CEN = 1" }), value: None, description: Some("Set CTLR1.CEN to enable the counter.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_TIM4_TIME_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.tim4", name: "TIM4 counter state", description: None, target_refs: &["periph.tim4"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: Some("CTLR1.CEN is cleared and the counter is stopped."), invariants: &[] }, metadata::SemanticState { name: "enabled", description: Some("CTLR1.CEN is set and the counter runs."), invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("Set CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.tim4.ctlr1.cen"), description: Some("Counter starts when CEN is asserted.") }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("Clear CTLR1.CEN"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.tim4.ctlr1.cen"), description: Some("Counter stops when CEN is cleared.") }] }] }];
pub const DRV_TIM4_TIME_CAPABILITY_TAGS: &[&str] = &["embassy-time-driver"];

#[derive(Debug, Clone, Copy)]
pub struct TIM4EmbassyTimeBaseResources {
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

pub const DRV_TIM4_TIME_RESOURCES: TIM4EmbassyTimeBaseResources = TIM4EmbassyTimeBaseResources {
    clocks: DRV_TIM4_TIME_CLOCK_BINDINGS,
    resets: DRV_TIM4_TIME_RESET_BINDINGS,
    interrupt_sources: DRV_TIM4_TIME_INTERRUPT_SOURCES,
    interrupts: DRV_TIM4_TIME_INTERRUPT_ROUTES,
    dma_channels: DRV_TIM4_TIME_DMA_CHANNELS,
    dma: DRV_TIM4_TIME_DMA_ROUTES,
    pins: DRV_TIM4_TIME_PIN_ROLES,
    init_operations: DRV_TIM4_TIME_INIT_OPERATIONS,
    state_machines: DRV_TIM4_TIME_STATE_MACHINES,
    lowering_pattern: Some("counter-compare-timer"),
    time_driver_source: Some("hardware-timer"),
    capability_tags: DRV_TIM4_TIME_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct TIM4EmbassyTimeBase {
    resources: TIM4EmbassyTimeBaseResources,
}

impl TIM4EmbassyTimeBase {
    pub fn new(resources: TIM4EmbassyTimeBaseResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TIM4EmbassyTimeBaseResources {
        self.resources
    }
    /// Enable the TIM4 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the TIM4 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for TIM4.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Release reset for TIM4.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_configure_counter_compare_timebase(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000828u64, 0xFFFFu16, 0x1F3Fu16)?;
        modify_u16(0x4000082Cu64, 0xFFFFu16, 0xFFFFu16)?;
        modify_u16(0x40000824u64, 0xFFFFu16, 0x0000u16)?;
        modify_u16(0x40000834u64, 0xFFFFu16, 0x0000u16)?;
        modify_u16(0x40000814u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn apply_enable(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000800u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_disabled_to_enabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000800u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }

    pub fn transition_enabled_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40000800u64, 0x0001u16, 0x0000u16)?;
        Ok(())
    }

    pub fn init_time_driver(&self) -> Result<(), metadata::Error> {
        self.enable_clock()?;
        self.release_reset()?;
        self.apply_configure_counter_compare_timebase()?;
        self.apply_enable()?;
        initialize_drv_tim4_time_time_driver()
    }

    pub fn now_ticks(&self) -> u64 {
        generated_drv_tim4_time_time_driver_now()
    }

    pub fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {
        generated_drv_tim4_time_time_driver_delay_ticks(ticks)
    }

    pub fn on_time_driver_interrupt(&self) {
        generated_drv_tim4_time_time_driver_interrupt();
    }


}


use core::cell::{Cell, RefCell};
use critical_section::Mutex as CriticalSectionMutex;
use embassy_time_driver::Driver as EmbassyTimeDriver;
use embassy_time_queue_utils::Queue as EmbassyTimeQueue;

const GENERATED_TIME_COUNTER_ADDRESS: u64 = 0x40000824u64;
const GENERATED_TIME_COUNTER_MASK: u32 = 0x0000FFFFu32;
const GENERATED_TIME_COUNTER_SHIFT: u32 = 0;
const GENERATED_TIME_COUNTER_BITS: u32 = 16;
const GENERATED_TIME_COMPARE_ADDRESS: u64 = 0x40000834u64;
const GENERATED_TIME_COMPARE_MASK: u32 = 0x0000FFFFu32;
const GENERATED_TIME_COMPARE_SHIFT: u32 = 0;
const GENERATED_TIME_INTERRUPT_ENABLE_ADDRESS: u64 = 0x4000080Cu64;
const GENERATED_TIME_INTERRUPT_ENABLE_MASK: u32 = 0x00000002u32;
const GENERATED_TIME_INTERRUPT_PENDING_ADDRESS: u64 = 0x40000810u64;
const GENERATED_TIME_INTERRUPT_PENDING_MASK: u32 = 0x00000002u32;

struct GeneratedCounterCompareTimeDriver {
    initialized: CriticalSectionMutex<Cell<bool>>,
    wraps: CriticalSectionMutex<Cell<u64>>,
    last_raw: CriticalSectionMutex<Cell<u32>>,
    queue: CriticalSectionMutex<RefCell<EmbassyTimeQueue>>,
}

impl GeneratedCounterCompareTimeDriver {
    const fn new() -> Self {
        Self {
            initialized: CriticalSectionMutex::new(Cell::new(false)),
            wraps: CriticalSectionMutex::new(Cell::new(0)),
            last_raw: CriticalSectionMutex::new(Cell::new(0)),
            queue: CriticalSectionMutex::new(RefCell::new(EmbassyTimeQueue::new())),
        }
    }

    fn init(&self) -> Result<(), metadata::Error> {
        critical_section::with(|cs| {
            if self.initialized.borrow(cs).get() {
                return Ok(());
            }
            self.set_alarm_enabled(false);
        must_modify_u16(0x40000810u64, 0x0002u16, 0x0000u16);
            self.wraps.borrow(cs).set(0);
            self.last_raw.borrow(cs).set(self.read_raw_counter());
            self.initialized.borrow(cs).set(true);
            Ok(())
        })
    }

    fn read_raw_counter(&self) -> u32 {
        let raw = must_read_u16(GENERATED_TIME_COUNTER_ADDRESS) as u32;
        ((raw & GENERATED_TIME_COUNTER_MASK) >> GENERATED_TIME_COUNTER_SHIFT) as u32
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
            (wraps << GENERATED_TIME_COUNTER_BITS) | u64::from(raw)
        })
    }

    fn arm_alarm(&self, at: u64) {
        let raw = (((at as u32) << GENERATED_TIME_COMPARE_SHIFT) & GENERATED_TIME_COMPARE_MASK) as u16;
        must_modify_u16(GENERATED_TIME_COMPARE_ADDRESS, GENERATED_TIME_COMPARE_MASK as u16, raw);
        self.set_alarm_enabled(true);
    }

    fn set_alarm_enabled(&self, enabled: bool) {
        let set_mask = enabled.then_some(GENERATED_TIME_INTERRUPT_ENABLE_MASK as u16).unwrap_or(0u16);
        must_modify_u16(GENERATED_TIME_INTERRUPT_ENABLE_ADDRESS, GENERATED_TIME_INTERRUPT_ENABLE_MASK as u16, set_mask);
    }

    fn acknowledge_interrupt(&self) {
        must_modify_u16(0x40000810u64, 0x0002u16, 0x0000u16);
    }

    fn is_alarm_pending(&self) -> bool {
        (must_read_u16(GENERATED_TIME_INTERRUPT_PENDING_ADDRESS) & (GENERATED_TIME_INTERRUPT_PENDING_MASK as u16)) != 0
    }
        
    fn on_interrupt(&self) {
        if !critical_section::with(|cs| self.initialized.borrow(cs).get()) || !self.is_alarm_pending() {
            return;
        }
        self.acknowledge_interrupt();
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

impl EmbassyTimeDriver for GeneratedCounterCompareTimeDriver {
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
fn must_read_u8(address: u64) -> u8 {
    read_u8(address).expect("generated time-driver MMIO read")
}

#[allow(dead_code)]
fn must_read_u16(address: u64) -> u16 {
    read_u16(address).expect("generated time-driver MMIO read")
}

#[allow(dead_code)]
fn must_read_u32(address: u64) -> u32 {
    read_u32(address).expect("generated time-driver MMIO read")
}

#[allow(dead_code)]
fn must_modify_u8(address: u64, clear_mask: u8, set_mask: u8) {
    modify_u8(address, clear_mask, set_mask).expect("generated time-driver MMIO write")
}

#[allow(dead_code)]
fn must_modify_u16(address: u64, clear_mask: u16, set_mask: u16) {
    modify_u16(address, clear_mask, set_mask).expect("generated time-driver MMIO write")
}

#[allow(dead_code)]
fn must_modify_u32(address: u64, clear_mask: u32, set_mask: u32) {
    modify_u32(address, clear_mask, set_mask).expect("generated time-driver MMIO write")
}

#[allow(dead_code)]
fn must_write_u8(address: u64, value: u8) {
    write_u8(address, value).expect("generated time-driver MMIO write")
}

#[allow(dead_code)]
fn must_write_u16(address: u64, value: u16) {
    write_u16(address, value).expect("generated time-driver MMIO write")
}

#[allow(dead_code)]
fn must_write_u32(address: u64, value: u32) {
    write_u32(address, value).expect("generated time-driver MMIO write")
}

embassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedCounterCompareTimeDriver = GeneratedCounterCompareTimeDriver::new());

#[allow(dead_code)]
pub fn generated_drv_tim4_time_time_driver_interrupt() {
    GENERATED_TIME_DRIVER.on_interrupt();
}

fn initialize_drv_tim4_time_time_driver() -> Result<(), metadata::Error> {
    GENERATED_TIME_DRIVER.init()
}

fn generated_drv_tim4_time_time_driver_now() -> u64 {
    GENERATED_TIME_DRIVER.now()
}

fn generated_drv_tim4_time_time_driver_delay_ticks(ticks: u64) -> Result<(), metadata::Error> {
    GENERATED_TIME_DRIVER.delay_ticks(ticks)
}
