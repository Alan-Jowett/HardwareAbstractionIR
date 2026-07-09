//! Generated Embassy-style time module for ESP32-C3FN4.

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
    module_name: "time",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: SystemTimer (timer) from canonical block block.systimer0 -> timer-general
pub const DRV_TIME_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clkbind.systimer", name: "SYSTIMER_CLK_EN", consumer_ref: "per.systimer", clock_ref: "clk.systimer", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_TIME_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rstbind.systimer", name: "SYSTIMER_RST", target_ref: "per.systimer", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_TIME_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.systimer0", name: "SYSTIMER_TARGET0", source_ref: "per.systimer", producer_ref: Some("block.systimer0"), kind: "timer", flag_refs: &[], clear_operation_refs: &["op.systimer.clear_target0_interrupt"] }];
pub const DRV_TIME_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.systimer0", name: "SYSTIMER target0 interrupt matrix source", source_ref: "isrc.systimer0", interrupt_ref: "irq.ets_systimer_target0_intr_source", controller_ref: "block.interrupt_matrix0", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "matrix", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_TIME_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIME_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIME_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_TIME_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.systimer.enable_target0", name: "Enable SYSTIMER target0 time base", description: None, kind: Some("initialization"), target_refs: &["per.systimer"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.systimer.conf"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CLK_EN" }), value: None, description: Some("Enable the SYSTIMER register clock.") }, metadata::SemanticOperationStep { index: 1, action: "write", target_ref: Some("reg.systimer.conf"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set TIMER_UNIT0_WORK_EN" }), value: None, description: Some("Start counter unit 0 so the generated timer-backed delay and time base can observe a running counter.") }, metadata::SemanticOperationStep { index: 2, action: "write", target_ref: Some("reg.systimer.conf"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set TARGET0_WORK_EN" }), value: None, description: Some("Enable comparator target 0 so the generated async time base can arm wake alarms.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_TIME_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[metadata::SemanticStateMachine { id: "sm.systimer", name: "SYSTIMER target0 running state", description: None, target_refs: &["per.systimer"], initial_state: Some("disabled"), states: &[metadata::SemanticState { name: "disabled", description: None, invariants: &[] }, metadata::SemanticState { name: "enabled", description: None, invariants: &[] }], transitions: &[metadata::SemanticTransition { from: "disabled", to: "enabled", trigger: Some("enable requested"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "starts-hardware", target_ref: Some("field.systimer.conf.timer_unit0_work_en"), description: None }] }, metadata::SemanticTransition { from: "enabled", to: "disabled", trigger: Some("disable requested"), conditions: &[], effects: &[metadata::SemanticSideEffect { kind: "stops-hardware", target_ref: Some("field.systimer.conf.timer_unit0_work_en"), description: None }] }] }];
pub const DRV_TIME_CAPABILITY_TAGS: &[&str] = &["embassy-time-driver"];

#[derive(Debug, Clone, Copy)]
pub struct SystemTimerResources {
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

pub const DRV_TIME_RESOURCES: SystemTimerResources = SystemTimerResources {
    clocks: DRV_TIME_CLOCK_BINDINGS,
    resets: DRV_TIME_RESET_BINDINGS,
    interrupt_sources: DRV_TIME_INTERRUPT_SOURCES,
    interrupts: DRV_TIME_INTERRUPT_ROUTES,
    dma_channels: DRV_TIME_DMA_CHANNELS,
    dma: DRV_TIME_DMA_ROUTES,
    pins: DRV_TIME_PIN_ROLES,
    init_operations: DRV_TIME_INIT_OPERATIONS,
    state_machines: DRV_TIME_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: Some("hardware-timer"),
    capability_tags: DRV_TIME_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct SystemTimer {
    resources: SystemTimerResources,
}

impl SystemTimer {
    pub fn new(resources: SystemTimerResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> SystemTimerResources {
        self.resources
    }
    /// Enable the SYSTIMER clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x20000000u32, 0x20000000u32)?;
        Ok(())
    }

    /// Disable the SYSTIMER clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x20000000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for SYSTIMER.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x20000000u32, 0x20000000u32)?;
        Ok(())
    }

    /// Release reset for SYSTIMER.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x20000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_enable_target0(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60023000u64, 0x80000000u32, 0x80000000u32)?;
        modify_u32(0x60023000u64, 0x40000000u32, 0x40000000u32)?;
        modify_u32(0x60023000u64, 0x01000000u32, 0x01000000u32)?;
        Ok(())
    }

    pub fn transition_disabled_to_enabled(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60023000u64, 0x40000000u32, 0x40000000u32)?;
        Ok(())
    }

    pub fn transition_enabled_to_disabled(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60023000u64, 0x40000000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn init_time_driver(&self) -> Result<(), metadata::Error> {
        self.enable_clock()?;
        self.release_reset()?;
        self.apply_enable_target0()?;
        initialize_drv_time_time_driver()
    }

    pub fn now_ticks(&self) -> u64 {
        generated_drv_time_time_driver_now()
    }

    pub fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {
        generated_drv_time_time_driver_delay_ticks(ticks)
    }

    pub fn on_time_driver_interrupt(&self) {
        generated_drv_time_time_driver_interrupt();
    }


}


use core::cell::{Cell, RefCell};
use critical_section::Mutex as CriticalSectionMutex;
use embassy_time_driver::Driver as EmbassyTimeDriver;
use embassy_time_queue_utils::Queue as EmbassyTimeQueue;

const GENERATED_TIME_CONF_ADDRESS: u64 = 0x60023000u64;
const GENERATED_TIME_CONF_CLK_EN_MASK: u32 = 0x80000000u32;
const GENERATED_TIME_CONF_TARGET0_WORK_EN_MASK: u32 = 0x01000000u32;
const GENERATED_TIME_CONF_UNIT0_WORK_EN_MASK: u32 = 0x40000000u32;
const GENERATED_TIME_UNIT0_OP_ADDRESS: u64 = 0x60023004u64;
const GENERATED_TIME_UNIT0_OP_VALUE_VALID_MASK: u32 = 0x20000000u32;
const GENERATED_TIME_UNIT0_OP_UPDATE_MASK: u32 = 0x40000000u32;
const GENERATED_TIME_UNIT0_VALUE_HI_ADDRESS: u64 = 0x60023040u64;
const GENERATED_TIME_UNIT0_VALUE_HI_MASK: u32 = 0x000FFFFFu32;
const GENERATED_TIME_UNIT0_VALUE_HI_SHIFT: u32 = 0;
const GENERATED_TIME_UNIT0_VALUE_LO_ADDRESS: u64 = 0x60023044u64;
const GENERATED_TIME_UNIT0_VALUE_LO_MASK: u32 = 0xFFFFFFFFu32;
const GENERATED_TIME_UNIT0_VALUE_LO_SHIFT: u32 = 0;
const GENERATED_TIME_TARGET0_HI_ADDRESS: u64 = 0x6002301Cu64;
const GENERATED_TIME_TARGET0_HI_MASK: u32 = 0x000FFFFFu32;
const GENERATED_TIME_TARGET0_HI_SHIFT: u32 = 0;
const GENERATED_TIME_TARGET0_LO_ADDRESS: u64 = 0x60023020u64;
const GENERATED_TIME_TARGET0_LO_MASK: u32 = 0xFFFFFFFFu32;
const GENERATED_TIME_TARGET0_LO_SHIFT: u32 = 0;
const GENERATED_TIME_TARGET0_CONF_ADDRESS: u64 = 0x60023034u64;
const GENERATED_TIME_TARGET0_PERIOD_MASK: u32 = 0x03FFFFFFu32;
const GENERATED_TIME_TARGET0_PERIOD_MODE_MASK: u32 = 0x40000000u32;
const GENERATED_TIME_TARGET0_UNIT_SEL_MASK: u32 = 0x80000000u32;
const GENERATED_TIME_COMP0_LOAD_ADDRESS: u64 = 0x60023050u64;
const GENERATED_TIME_COMP0_LOAD_MASK: u32 = 0x00000001u32;
const GENERATED_TIME_COMP0_LOAD_SHIFT: u32 = 0;
const GENERATED_TIME_INT_ENA_ADDRESS: u64 = 0x60023064u64;
const GENERATED_TIME_INT_ENA_TARGET0_MASK: u32 = 0x00000001u32;

struct GeneratedHardwareTimerTimeDriver {
    initialized: CriticalSectionMutex<Cell<bool>>,
    queue: CriticalSectionMutex<RefCell<EmbassyTimeQueue>>,
}

impl GeneratedHardwareTimerTimeDriver {
    const fn new() -> Self {
        Self {
            initialized: CriticalSectionMutex::new(Cell::new(false)),
            queue: CriticalSectionMutex::new(RefCell::new(EmbassyTimeQueue::new())),
        }
    }

    fn init(&self) -> Result<(), metadata::Error> {
        critical_section::with(|cs| {
            if self.initialized.borrow(cs).get() {
                return Ok(());
            }
            modify_u32(
                GENERATED_TIME_CONF_ADDRESS,
                0u32,
                GENERATED_TIME_CONF_CLK_EN_MASK
                    | GENERATED_TIME_CONF_TARGET0_WORK_EN_MASK
                    | GENERATED_TIME_CONF_UNIT0_WORK_EN_MASK,
            )?;
            modify_u32(
                GENERATED_TIME_TARGET0_CONF_ADDRESS,
                GENERATED_TIME_TARGET0_PERIOD_MASK
                    | GENERATED_TIME_TARGET0_PERIOD_MODE_MASK
                    | GENERATED_TIME_TARGET0_UNIT_SEL_MASK,
                0u32,
            )?;
            modify_u32(GENERATED_TIME_INT_ENA_ADDRESS, GENERATED_TIME_INT_ENA_TARGET0_MASK, 0u32)?;
        modify_u32(0x6002306Cu64, 0x00000001u32, 0x00000001u32)?;
            self.initialized.borrow(cs).set(true);
            Ok(())
        })
    }

    fn read_now(&self) -> u64 {
        must_modify_u32(
            GENERATED_TIME_UNIT0_OP_ADDRESS,
            GENERATED_TIME_UNIT0_OP_UPDATE_MASK,
            GENERATED_TIME_UNIT0_OP_UPDATE_MASK,
        );
        loop {
            if (must_read_u32(GENERATED_TIME_UNIT0_OP_ADDRESS) & GENERATED_TIME_UNIT0_OP_VALUE_VALID_MASK) != 0 {
                let high = ((must_read_u32(GENERATED_TIME_UNIT0_VALUE_HI_ADDRESS) & GENERATED_TIME_UNIT0_VALUE_HI_MASK) >> GENERATED_TIME_UNIT0_VALUE_HI_SHIFT) as u64;
                let low = ((must_read_u32(GENERATED_TIME_UNIT0_VALUE_LO_ADDRESS) & GENERATED_TIME_UNIT0_VALUE_LO_MASK) >> GENERATED_TIME_UNIT0_VALUE_LO_SHIFT) as u64;
                return (high << 32) | low;
            }
        }
    }

    fn arm_alarm(&self, at: u64) {
        let high = ((at >> 32) as u32) << GENERATED_TIME_TARGET0_HI_SHIFT;
        let low = (at as u32) << GENERATED_TIME_TARGET0_LO_SHIFT;
        must_modify_u32(GENERATED_TIME_TARGET0_HI_ADDRESS, GENERATED_TIME_TARGET0_HI_MASK, high & GENERATED_TIME_TARGET0_HI_MASK);
        must_modify_u32(GENERATED_TIME_TARGET0_LO_ADDRESS, GENERATED_TIME_TARGET0_LO_MASK, low & GENERATED_TIME_TARGET0_LO_MASK);
        must_modify_u32(
            GENERATED_TIME_TARGET0_CONF_ADDRESS,
            GENERATED_TIME_TARGET0_PERIOD_MASK | GENERATED_TIME_TARGET0_PERIOD_MODE_MASK | GENERATED_TIME_TARGET0_UNIT_SEL_MASK,
            0u32,
        );
        must_modify_u32(
            GENERATED_TIME_COMP0_LOAD_ADDRESS,
            GENERATED_TIME_COMP0_LOAD_MASK,
            (1u32 << GENERATED_TIME_COMP0_LOAD_SHIFT) & GENERATED_TIME_COMP0_LOAD_MASK,
        );
        must_modify_u32(
            GENERATED_TIME_INT_ENA_ADDRESS,
            GENERATED_TIME_INT_ENA_TARGET0_MASK,
            GENERATED_TIME_INT_ENA_TARGET0_MASK,
        );
    }

    fn disarm_alarm(&self) {
        must_modify_u32(GENERATED_TIME_INT_ENA_ADDRESS, GENERATED_TIME_INT_ENA_TARGET0_MASK, 0u32);
    }

    fn acknowledge_interrupt(&self) {
        must_modify_u32(0x6002306Cu64, 0x00000001u32, 0x00000001u32);
    }

    fn on_interrupt(&self) {
        critical_section::with(|cs| {
            if !self.initialized.borrow(cs).get() {
                return;
            }
            self.acknowledge_interrupt();
            let now = self.read_now();
            let mut queue = self.queue.borrow(cs).borrow_mut();
            let next = queue.next_expiration(now);
            if next == u64::MAX {
                self.disarm_alarm();
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

impl EmbassyTimeDriver for GeneratedHardwareTimerTimeDriver {
    fn now(&self) -> u64 {
        self.read_now()
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let now = self.read_now();
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let next = queue.next_expiration(now);
                if next == u64::MAX {
                    self.disarm_alarm();
                } else {
                    self.arm_alarm(next);
                }
            }
        });
    }
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
fn must_write_u32(address: u64, value: u32) {
    write_u32(address, value).expect("generated time-driver MMIO write")
}

embassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedHardwareTimerTimeDriver = GeneratedHardwareTimerTimeDriver::new());

#[allow(dead_code)]
pub fn generated_drv_time_time_driver_interrupt() {
    GENERATED_TIME_DRIVER.on_interrupt();
}

fn initialize_drv_time_time_driver() -> Result<(), metadata::Error> {
    GENERATED_TIME_DRIVER.init()
}

fn generated_drv_time_time_driver_now() -> u64 {
    GENERATED_TIME_DRIVER.now()
}

fn generated_drv_time_time_driver_delay_ticks(ticks: u64) -> Result<(), metadata::Error> {
    GENERATED_TIME_DRIVER.delay_ticks(ticks)
}
