//! Generated Embassy-style interrupt module for CH32V203G6U6.

use crate::metadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Irq {
    WWDG = 16,
    PVD = 17,
    TAMPER = 18,
    RTC = 19,
    FLASH = 20,
    RCC = 21,
    EXTI0 = 22,
    EXTI1 = 23,
    EXTI2 = 24,
    EXTI3 = 25,
    EXTI4 = 26,
    DMA1Channel1 = 27,
    DMA1Channel2 = 28,
    DMA1Channel3 = 29,
    DMA1Channel4 = 30,
    DMA1Channel5 = 31,
    DMA1Channel6 = 32,
    DMA1Channel7 = 33,
    ADC12 = 34,
    USBHPCAN1TX = 35,
    USBLPCAN1RX0 = 36,
    CAN1RX1 = 37,
    CAN1SCE = 38,
    EXTI95 = 39,
    TIM1BRK = 40,
    TIM1UP = 41,
    TIM1TRGCOM = 42,
    TIM1CC = 43,
    TIM2 = 44,
    TIM3 = 45,
    TIM4 = 46,
    I2C1EV = 47,
    I2C1ER = 48,
    SPI1 = 51,
    USART1 = 53,
    USART2 = 54,
    EXTI1510 = 56,
    RTCAlarm = 57,
    USBWakeUp = 58,
    DMA1Channel8 = 62,
}

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "interrupt",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: PFIC (interrupt) from canonical block block.pfic -> interrupt-controller
pub const DRV_PFIC_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[];
pub const DRV_PFIC_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_PFIC_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.usart1.global", name: "USART1 GLOBAL interrupt source", source_ref: "periph.usart1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.usart2.global", name: "USART2 GLOBAL interrupt source", source_ref: "periph.usart2", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.spi1.global", name: "SPI1 GLOBAL interrupt source", source_ref: "periph.spi1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.i2c1.er", name: "I2C1 ER interrupt source", source_ref: "periph.i2c1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.i2c1.ev", name: "I2C1 EV interrupt source", source_ref: "periph.i2c1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.brk", name: "TIM1 BRK interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.up", name: "TIM1 UP interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.trg", name: "TIM1 TRG interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.com", name: "TIM1 COM interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim1.cc", name: "TIM1 CC interrupt source", source_ref: "periph.tim1", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim2.up", name: "TIM2 UP interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim2.trg", name: "TIM2 TRG interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim2.cc", name: "TIM2 CC interrupt source", source_ref: "periph.tim2", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim3.up", name: "TIM3 UP interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim3.trg", name: "TIM3 TRG interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim3.cc", name: "TIM3 CC interrupt source", source_ref: "periph.tim3", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim4.up", name: "TIM4 UP interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim4.trg", name: "TIM4 TRG interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.tim4.cc", name: "TIM4 CC interrupt source", source_ref: "periph.tim4", producer_ref: None, kind: "timer", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.adc1.global", name: "ADC1 GLOBAL interrupt source", source_ref: "periph.adc1", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.adc2.global", name: "ADC2 GLOBAL interrupt source", source_ref: "periph.adc2", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel1", name: "DMA1 channel 1 interrupt source", source_ref: "dmach.dma1.ch1", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel2", name: "DMA1 channel 2 interrupt source", source_ref: "dmach.dma1.ch2", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel3", name: "DMA1 channel 3 interrupt source", source_ref: "dmach.dma1.ch3", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel4", name: "DMA1 channel 4 interrupt source", source_ref: "dmach.dma1.ch4", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel5", name: "DMA1 channel 5 interrupt source", source_ref: "dmach.dma1.ch5", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel6", name: "DMA1 channel 6 interrupt source", source_ref: "dmach.dma1.ch6", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel7", name: "DMA1 channel 7 interrupt source", source_ref: "dmach.dma1.ch7", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.dma1.channel8", name: "DMA1 channel 8 interrupt source", source_ref: "dmach.dma1.ch8", producer_ref: None, kind: "dma", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_PFIC_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.usart1.global", name: "USART1 GLOBAL interrupt route", source_ref: "isrc.usart1.global", interrupt_ref: "int.usart1", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.usart2.global", name: "USART2 GLOBAL interrupt route", source_ref: "isrc.usart2.global", interrupt_ref: "int.usart2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.spi1.global", name: "SPI1 GLOBAL interrupt route", source_ref: "isrc.spi1.global", interrupt_ref: "int.spi1", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.i2c1.er", name: "I2C1 ER interrupt route", source_ref: "isrc.i2c1.er", interrupt_ref: "int.i2c1er", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.i2c1.ev", name: "I2C1 EV interrupt route", source_ref: "isrc.i2c1.ev", interrupt_ref: "int.i2c1ev", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.brk", name: "TIM1 BRK interrupt route", source_ref: "isrc.tim1.brk", interrupt_ref: "int.tim1brk", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.up", name: "TIM1 UP interrupt route", source_ref: "isrc.tim1.up", interrupt_ref: "int.tim1up", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.trg", name: "TIM1 TRG interrupt route", source_ref: "isrc.tim1.trg", interrupt_ref: "int.tim1trgcom", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.com", name: "TIM1 COM interrupt route", source_ref: "isrc.tim1.com", interrupt_ref: "int.tim1trgcom", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim1.cc", name: "TIM1 CC interrupt route", source_ref: "isrc.tim1.cc", interrupt_ref: "int.tim1cc", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim2.up", name: "TIM2 UP interrupt route", source_ref: "isrc.tim2.up", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim2.trg", name: "TIM2 TRG interrupt route", source_ref: "isrc.tim2.trg", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim2.cc", name: "TIM2 CC interrupt route", source_ref: "isrc.tim2.cc", interrupt_ref: "int.tim2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim3.up", name: "TIM3 UP interrupt route", source_ref: "isrc.tim3.up", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim3.trg", name: "TIM3 TRG interrupt route", source_ref: "isrc.tim3.trg", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim3.cc", name: "TIM3 CC interrupt route", source_ref: "isrc.tim3.cc", interrupt_ref: "int.tim3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim4.up", name: "TIM4 UP interrupt route", source_ref: "isrc.tim4.up", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim4.trg", name: "TIM4 TRG interrupt route", source_ref: "isrc.tim4.trg", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.tim4.cc", name: "TIM4 CC interrupt route", source_ref: "isrc.tim4.cc", interrupt_ref: "int.tim4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.adc1.global", name: "ADC1 GLOBAL interrupt route", source_ref: "isrc.adc1.global", interrupt_ref: "int.adc12", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.adc2.global", name: "ADC2 GLOBAL interrupt route", source_ref: "isrc.adc2.global", interrupt_ref: "int.adc12", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel1", name: "DMA1 channel 1 interrupt route", source_ref: "isrc.dma1.channel1", interrupt_ref: "int.dma1channel1", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel2", name: "DMA1 channel 2 interrupt route", source_ref: "isrc.dma1.channel2", interrupt_ref: "int.dma1channel2", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel3", name: "DMA1 channel 3 interrupt route", source_ref: "isrc.dma1.channel3", interrupt_ref: "int.dma1channel3", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel4", name: "DMA1 channel 4 interrupt route", source_ref: "isrc.dma1.channel4", interrupt_ref: "int.dma1channel4", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel5", name: "DMA1 channel 5 interrupt route", source_ref: "isrc.dma1.channel5", interrupt_ref: "int.dma1channel5", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel6", name: "DMA1 channel 6 interrupt route", source_ref: "isrc.dma1.channel6", interrupt_ref: "int.dma1channel6", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel7", name: "DMA1 channel 7 interrupt route", source_ref: "isrc.dma1.channel7", interrupt_ref: "int.dma1channel7", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.dma1.channel8", name: "DMA1 channel 8 interrupt route", source_ref: "isrc.dma1.channel8", interrupt_ref: "int.dma1channel8", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_PFIC_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_PFIC_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_PFIC_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_PFIC_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_PFIC_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_PFIC_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct PFICResources {
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
    pub capability_tags: &'static [&'static str],
}

pub const DRV_PFIC_RESOURCES: PFICResources = PFICResources {
    clocks: DRV_PFIC_CLOCK_BINDINGS,
    resets: DRV_PFIC_RESET_BINDINGS,
    interrupt_sources: DRV_PFIC_INTERRUPT_SOURCES,
    interrupts: DRV_PFIC_INTERRUPT_ROUTES,
    dma_channels: DRV_PFIC_DMA_CHANNELS,
    dma: DRV_PFIC_DMA_ROUTES,
    pins: DRV_PFIC_PIN_ROLES,
    init_operations: DRV_PFIC_INIT_OPERATIONS,
    state_machines: DRV_PFIC_STATE_MACHINES,
    lowering_pattern: None,
    capability_tags: DRV_PFIC_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct PFIC {
    resources: PFICResources,
}

impl PFIC {
    pub fn new(resources: PFICResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> PFICResources {
        self.resources
    }
    pub fn bind(&self) -> &'static [metadata::InterruptRoute] {
        self.resources.interrupts
    }


}

