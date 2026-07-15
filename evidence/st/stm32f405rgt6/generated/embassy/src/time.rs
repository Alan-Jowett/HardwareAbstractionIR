//! Generated Embassy-style time module for STM32F405RGT6.

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

// Driver instance: Time (interrupt) from canonical block block.nvic -> interrupt-controller
pub const DRV_TIME_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[];
pub const DRV_TIME_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_TIME_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource {
    id: "isrc.systick",
    name: "SysTick source",
    source_ref: "block.cpu0",
    producer_ref: Some("block.cpu0"),
    kind: "timer",
    flag_refs: &[],
    clear_operation_refs: &[],
}];
pub const DRV_TIME_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.systick",
    name: "SysTick route",
    source_ref: "isrc.systick",
    interrupt_ref: "irq.systick",
    controller_ref: "block.nvic",
    cpu_target_ref: Some("block.cpu0"),
    line_index: None,
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_TIME_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_TIME_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_TIME_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_TIME_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_TIME_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_TIME_CAPABILITY_TAGS: &[&str] = &["embassy-time-driver"];

#[derive(Debug, Clone, Copy)]
pub struct TimeResources {
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

pub const DRV_TIME_RESOURCES: TimeResources = TimeResources {
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
    time_driver_source: Some("systick"),
    capability_tags: DRV_TIME_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct Time {
    resources: TimeResources,
}

impl Time {
    pub fn new(resources: TimeResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> TimeResources {
        self.resources
    }
    pub fn bind(&self) -> &'static [metadata::InterruptRoute] {
        self.resources.interrupts
    }

    pub fn init_time_driver(&self) -> Result<(), metadata::Error> {
        initialize_drv_time_time_driver()
    }
}

use core::cell::{Cell, RefCell};
use critical_section::Mutex as CriticalSectionMutex;
use embassy_time_driver::Driver as EmbassyTimeDriver;
use embassy_time_queue_utils::Queue as EmbassyTimeQueue;

const SYST_CSR_ADDRESS: u64 = 0xE000_E010;
const SYST_RVR_ADDRESS: u64 = 0xE000_E014;
const SYST_CVR_ADDRESS: u64 = 0xE000_E018;
const SYST_CSR_ENABLE: u32 = 1 << 0;
const SYST_CSR_TICKINT: u32 = 1 << 1;
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;
const SYST_RELOAD_VALUE: u32 = 15;

struct GeneratedSystickTimeDriver {
    initialized: CriticalSectionMutex<Cell<bool>>,
    ticks: CriticalSectionMutex<Cell<u64>>,
    queue: CriticalSectionMutex<RefCell<EmbassyTimeQueue>>,
}

impl GeneratedSystickTimeDriver {
    const fn new() -> Self {
        Self {
            initialized: CriticalSectionMutex::new(Cell::new(false)),
            ticks: CriticalSectionMutex::new(Cell::new(0)),
            queue: CriticalSectionMutex::new(RefCell::new(EmbassyTimeQueue::new())),
        }
    }

    fn init(&self) -> Result<(), metadata::Error> {
        critical_section::with(|cs| {
            if self.initialized.borrow(cs).get() {
                return Ok(());
            }
            self.ticks.borrow(cs).set(0);
            write_u32(SYST_CSR_ADDRESS, 0)?;
            write_u32(SYST_RVR_ADDRESS, SYST_RELOAD_VALUE)?;
            write_u32(SYST_CVR_ADDRESS, 0)?;
            write_u32(
                SYST_CSR_ADDRESS,
                SYST_CSR_ENABLE | SYST_CSR_TICKINT | SYST_CSR_CLKSOURCE,
            )?;
            self.initialized.borrow(cs).set(true);
            Ok(())
        })
    }

    fn on_systick(&self) {
        critical_section::with(|cs| {
            if !self.initialized.borrow(cs).get() {
                return;
            }
            let now = self.ticks.borrow(cs).get().wrapping_add(1);
            self.ticks.borrow(cs).set(now);
            let _ = self.queue.borrow(cs).borrow_mut().next_expiration(now);
        });
    }
}

impl EmbassyTimeDriver for GeneratedSystickTimeDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| self.ticks.borrow(cs).get())
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let now = self.ticks.borrow(cs).get();
            let mut queue = self.queue.borrow(cs).borrow_mut();
            let _ = queue.schedule_wake(at, waker);
            let _ = queue.next_expiration(now);
        });
    }
}

embassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedSystickTimeDriver = GeneratedSystickTimeDriver::new());

#[allow(dead_code)]
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn SysTick() {
    GENERATED_TIME_DRIVER.on_systick();
}

fn initialize_drv_time_time_driver() -> Result<(), metadata::Error> {
    GENERATED_TIME_DRIVER.init()
}
