//! Generated Embassy-style interrupt module for LM3S6965.

use crate::metadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Irq {
    SysTick = -1,
    GPIOA = 0,
    GPIOB = 1,
    GPIOC = 2,
    GPIOD = 3,
    GPIOE = 4,
    UART0 = 5,
    UART1 = 6,
    SSI0 = 7,
    I2C0 = 8,
    WATCHDOG0 = 18,
    TIMER0A = 19,
    TIMER0B = 20,
    TIMER1A = 21,
    TIMER1B = 22,
    TIMER2A = 23,
    TIMER2B = 24,
    SYSCTL = 28,
    FLASHCTRL = 29,
    GPIOF = 30,
    TIMER3A = 35,
    TIMER3B = 36,
}

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "interrupt",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: NVIC (interrupt) from canonical block block.nvic -> interrupt-controller
pub const DRV_NVIC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[];
pub const DRV_NVIC_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_NVIC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.systick", name: "SysTick interrupt source", source_ref: "periph.systick", producer_ref: Some("periph.systick"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.gpioa", name: "GPIOA interrupt source", source_ref: "periph.gpioa", producer_ref: Some("periph.gpioa"), kind: "gpio", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.gpiob", name: "GPIOB interrupt source", source_ref: "periph.gpiob", producer_ref: Some("periph.gpiob"), kind: "gpio", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.gpioc", name: "GPIOC interrupt source", source_ref: "periph.gpioc", producer_ref: Some("periph.gpioc"), kind: "gpio", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.gpiod", name: "GPIOD interrupt source", source_ref: "periph.gpiod", producer_ref: Some("periph.gpiod"), kind: "gpio", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.gpioe", name: "GPIOE interrupt source", source_ref: "periph.gpioe", producer_ref: Some("periph.gpioe"), kind: "gpio", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.gpiof", name: "GPIOF interrupt source", source_ref: "periph.gpiof", producer_ref: Some("periph.gpiof"), kind: "gpio", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.uart0", name: "UART0 interrupt source", source_ref: "periph.uart0", producer_ref: Some("periph.uart0"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.uart1", name: "UART1 interrupt source", source_ref: "periph.uart1", producer_ref: Some("periph.uart1"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.ssi0", name: "SSI0 interrupt source", source_ref: "periph.ssi0", producer_ref: Some("periph.ssi0"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.i2c0", name: "I2C0 interrupt source", source_ref: "periph.i2c0", producer_ref: Some("periph.i2c0"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.watchdog0", name: "WATCHDOG0 interrupt source", source_ref: "periph.watchdog0", producer_ref: Some("periph.watchdog0"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer0a", name: "TIMER0A interrupt source", source_ref: "periph.timer0", producer_ref: Some("periph.timer0"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer0b", name: "TIMER0B interrupt source", source_ref: "periph.timer0", producer_ref: Some("periph.timer0"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer1a", name: "TIMER1A interrupt source", source_ref: "periph.timer1", producer_ref: Some("periph.timer1"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer1b", name: "TIMER1B interrupt source", source_ref: "periph.timer1", producer_ref: Some("periph.timer1"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer2a", name: "TIMER2A interrupt source", source_ref: "periph.timer2", producer_ref: Some("periph.timer2"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer2b", name: "TIMER2B interrupt source", source_ref: "periph.timer2", producer_ref: Some("periph.timer2"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer3a", name: "TIMER3A interrupt source", source_ref: "periph.timer3", producer_ref: Some("periph.timer3"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.timer3b", name: "TIMER3B interrupt source", source_ref: "periph.timer3", producer_ref: Some("periph.timer3"), kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.sysctl", name: "SYSCTL interrupt source", source_ref: "periph.sysctl", producer_ref: Some("periph.sysctl"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.flash", name: "FLASH interrupt source", source_ref: "periph.flash_ctrl", producer_ref: Some("periph.flash_ctrl"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_NVIC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.systick", name: "SysTick interrupt source route", source_ref: "isrc.systick", interrupt_ref: "int.systick", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.gpioa", name: "GPIOA interrupt source route", source_ref: "isrc.gpioa", interrupt_ref: "int.gpioa", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.gpiob", name: "GPIOB interrupt source route", source_ref: "isrc.gpiob", interrupt_ref: "int.gpiob", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.gpioc", name: "GPIOC interrupt source route", source_ref: "isrc.gpioc", interrupt_ref: "int.gpioc", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.gpiod", name: "GPIOD interrupt source route", source_ref: "isrc.gpiod", interrupt_ref: "int.gpiod", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.gpioe", name: "GPIOE interrupt source route", source_ref: "isrc.gpioe", interrupt_ref: "int.gpioe", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.gpiof", name: "GPIOF interrupt source route", source_ref: "isrc.gpiof", interrupt_ref: "int.gpiof", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.uart0", name: "UART0 interrupt source route", source_ref: "isrc.uart0", interrupt_ref: "int.uart0", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.uart1", name: "UART1 interrupt source route", source_ref: "isrc.uart1", interrupt_ref: "int.uart1", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.ssi0", name: "SSI0 interrupt source route", source_ref: "isrc.ssi0", interrupt_ref: "int.ssi0", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.i2c0", name: "I2C0 interrupt source route", source_ref: "isrc.i2c0", interrupt_ref: "int.i2c0", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.watchdog0", name: "WATCHDOG0 interrupt source route", source_ref: "isrc.watchdog0", interrupt_ref: "int.watchdog0", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer0a", name: "TIMER0A interrupt source route", source_ref: "isrc.timer0a", interrupt_ref: "int.timer0a", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer0b", name: "TIMER0B interrupt source route", source_ref: "isrc.timer0b", interrupt_ref: "int.timer0b", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer1a", name: "TIMER1A interrupt source route", source_ref: "isrc.timer1a", interrupt_ref: "int.timer1a", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer1b", name: "TIMER1B interrupt source route", source_ref: "isrc.timer1b", interrupt_ref: "int.timer1b", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer2a", name: "TIMER2A interrupt source route", source_ref: "isrc.timer2a", interrupt_ref: "int.timer2a", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer2b", name: "TIMER2B interrupt source route", source_ref: "isrc.timer2b", interrupt_ref: "int.timer2b", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer3a", name: "TIMER3A interrupt source route", source_ref: "isrc.timer3a", interrupt_ref: "int.timer3a", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.timer3b", name: "TIMER3B interrupt source route", source_ref: "isrc.timer3b", interrupt_ref: "int.timer3b", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.sysctl", name: "SYSCTL interrupt source route", source_ref: "isrc.sysctl", interrupt_ref: "int.sysctl", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.flash", name: "FLASH interrupt source route", source_ref: "isrc.flash", interrupt_ref: "int.flash", controller_ref: "block.nvic", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_NVIC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_NVIC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_NVIC_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_NVIC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_NVIC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_NVIC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct NVICResources {
    pub clocks: &'static [metadata::ClockBinding],
    pub resets: &'static [metadata::ResetBinding],
    pub interrupt_sources: &'static [metadata::InterruptSource],
    pub interrupts: &'static [metadata::InterruptRoute],
    pub dma_channels: &'static [metadata::DmaChannel],
    pub dma: &'static [metadata::DmaRoute],
    pub pins: &'static [metadata::PinRole],
    pub init_operations: &'static [metadata::SemanticOperation],
    pub state_machines: &'static [metadata::SemanticStateMachine],
    pub capability_tags: &'static [&'static str],
}

pub const DRV_NVIC_RESOURCES: NVICResources = NVICResources {
    clocks: DRV_NVIC_CLOCK_BINDINGS,
    resets: DRV_NVIC_RESET_BINDINGS,
    interrupt_sources: DRV_NVIC_INTERRUPT_SOURCES,
    interrupts: DRV_NVIC_INTERRUPT_ROUTES,
    dma_channels: DRV_NVIC_DMA_CHANNELS,
    dma: DRV_NVIC_DMA_ROUTES,
    pins: DRV_NVIC_PIN_ROLES,
    init_operations: DRV_NVIC_INIT_OPERATIONS,
    state_machines: DRV_NVIC_STATE_MACHINES,
    capability_tags: DRV_NVIC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct NVIC {
    resources: NVICResources,
}

impl NVIC {
    pub fn new(resources: NVICResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> NVICResources {
        self.resources
    }
    pub fn bind(&self) -> &'static [metadata::InterruptRoute] {
        self.resources.interrupts
    }

}

