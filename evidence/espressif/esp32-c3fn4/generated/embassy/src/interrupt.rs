//! Generated Embassy-style interrupt module for ESP32-C3FN4.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Irq {
    ETSWIFIMACINTRSOURCE = 0,
    ETSWIFIMACNMISOURCE = 1,
    ETSWIFIPWRINTRSOURCE = 2,
    ETSWIFIBBINTRSOURCE = 3,
    ETSBTMACINTRSOURCE = 4,
    ETSBTBBINTRSOURCE = 5,
    ETSBTBBNMISOURCE = 6,
    ETSRWBTINTRSOURCE = 7,
    ETSRWBLEINTRSOURCE = 8,
    ETSRWBTNMISOURCE = 9,
    ETSRWBLENMISOURCE = 10,
    ETSI2CMASTERSOURCE = 11,
    ETSSLC0INTRSOURCE = 12,
    ETSSLC1INTRSOURCE = 13,
    ETSAPBCTRLINTRSOURCE = 14,
    ETSUHCI0INTRSOURCE = 15,
    ETSGPIOINTRSOURCE = 16,
    ETSGPIONMISOURCE = 17,
    ETSSPI1INTRSOURCE = 18,
    ETSSPI2INTRSOURCE = 19,
    ETSI2S0INTRSOURCE = 20,
    ETSUART0INTRSOURCE = 21,
    ETSUART1INTRSOURCE = 22,
    ETSLEDCINTRSOURCE = 23,
    ETSEFUSEINTRSOURCE = 24,
    ETSTWAIINTRSOURCE = 25,
    ETSUSBSERIALJTAGINTRSOURCE = 26,
    ETSRTCCOREINTRSOURCE = 27,
    ETSRMTINTRSOURCE = 28,
    ETSI2CEXT0INTRSOURCE = 29,
    ETSTIMER1INTRSOURCE = 30,
    ETSTIMER2INTRSOURCE = 31,
    ETSTG0T0LEVELINTRSOURCE = 32,
    ETSTG0WDTLEVELINTRSOURCE = 33,
    ETSTG1T0LEVELINTRSOURCE = 34,
    ETSTG1WDTLEVELINTRSOURCE = 35,
    ETSCACHEIAINTRSOURCE = 36,
    ETSSYSTIMERTARGET0INTRSOURCE = 37,
    ETSSYSTIMERTARGET1INTRSOURCE = 38,
    ETSSYSTIMERTARGET2INTRSOURCE = 39,
    ETSSPIMEMREJECTCACHEINTRSOURCE = 40,
    ETSICACHEPRELOAD0INTRSOURCE = 41,
    ETSICACHESYNC0INTRSOURCE = 42,
    ETSAPBADCINTRSOURCE = 43,
    ETSDMACH0INTRSOURCE = 44,
    ETSDMACH1INTRSOURCE = 45,
    ETSDMACH2INTRSOURCE = 46,
    ETSRSAINTRSOURCE = 47,
    ETSAESINTRSOURCE = 48,
    ETSSHAINTRSOURCE = 49,
    ETSFROMCPUINTR0SOURCE = 50,
    ETSFROMCPUINTR1SOURCE = 51,
    ETSFROMCPUINTR2SOURCE = 52,
    ETSFROMCPUINTR3SOURCE = 53,
    ETSASSISTDEBUGINTRSOURCE = 54,
    ETSDMAAPBPERIPMSINTRSOURCE = 55,
    ETSCORE0IRAM0PMSINTRSOURCE = 56,
    ETSCORE0DRAM0PMSINTRSOURCE = 57,
    ETSCORE0PIFPMSINTRSOURCE = 58,
    ETSCORE0PIFPMSSIZEINTRSOURCE = 59,
    ETSBAKPMSVIOLATEINTRSOURCE = 60,
    ETSCACHECORE0ACSINTRSOURCE = 61,
}

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "interrupt",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: InterruptMatrix (interrupt) from canonical block block.interrupt_matrix0 -> interrupt-matrix
pub const DRV_IRQ_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[];
pub const DRV_IRQ_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_IRQ_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[
    metadata::InterruptSource {
        id: "isrc.gpio",
        name: "GPIO",
        source_ref: "per.gpio",
        producer_ref: Some("block.gpio0"),
        kind: "gpio",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.uart0",
        name: "UART0",
        source_ref: "per.uart0",
        producer_ref: Some("block.uart0"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.uart1",
        name: "UART1",
        source_ref: "per.uart1",
        producer_ref: Some("block.uart1"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.spi2",
        name: "SPI2",
        source_ref: "per.spi2",
        producer_ref: Some("block.spi2"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.i2c0",
        name: "I2C_EXT0",
        source_ref: "per.i2c0",
        producer_ref: Some("block.i2c0"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.apb_adc",
        name: "APB ADC",
        source_ref: "per.apb_saradc",
        producer_ref: Some("block.apb_saradc"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.usb_device",
        name: "USB_SERIAL_JTAG",
        source_ref: "per.usb_device",
        producer_ref: Some("block.usb_device"),
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.systimer0",
        name: "SYSTIMER_TARGET0",
        source_ref: "per.systimer",
        producer_ref: Some("block.systimer0"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &["op.systimer.clear_target0_interrupt"],
    },
    metadata::InterruptSource {
        id: "isrc.systimer1",
        name: "SYSTIMER_TARGET1",
        source_ref: "per.systimer",
        producer_ref: Some("block.systimer0"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.systimer2",
        name: "SYSTIMER_TARGET2",
        source_ref: "per.systimer",
        producer_ref: Some("block.systimer0"),
        kind: "timer",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.dma0",
        name: "DMA_CH0",
        source_ref: "per.dma",
        producer_ref: Some("block.gdma0"),
        kind: "dma",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.dma1",
        name: "DMA_CH1",
        source_ref: "per.dma",
        producer_ref: Some("block.gdma0"),
        kind: "dma",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.dma2",
        name: "DMA_CH2",
        source_ref: "per.dma",
        producer_ref: Some("block.gdma0"),
        kind: "dma",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
];
pub const DRV_IRQ_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[
    metadata::InterruptRoute {
        id: "iroute.gpio",
        name: "GPIO interrupt matrix source",
        source_ref: "isrc.gpio",
        interrupt_ref: "irq.ets_gpio_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
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
    },
    metadata::InterruptRoute {
        id: "iroute.uart1",
        name: "UART1 interrupt matrix source",
        source_ref: "isrc.uart1",
        interrupt_ref: "irq.ets_uart1_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.spi2",
        name: "SPI2 interrupt matrix source",
        source_ref: "isrc.spi2",
        interrupt_ref: "irq.ets_spi2_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.i2c0",
        name: "I2C0 interrupt matrix source",
        source_ref: "isrc.i2c0",
        interrupt_ref: "irq.ets_i2c_ext0_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.apb_adc",
        name: "APB ADC interrupt matrix source",
        source_ref: "isrc.apb_adc",
        interrupt_ref: "irq.ets_apb_adc_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.usb_device",
        name: "USB Serial/JTAG interrupt matrix source",
        source_ref: "isrc.usb_device",
        interrupt_ref: "irq.ets_usb_serial_jtag_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.systimer0",
        name: "SYSTIMER target0 interrupt matrix source",
        source_ref: "isrc.systimer0",
        interrupt_ref: "irq.ets_systimer_target0_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.systimer1",
        name: "SYSTIMER target1 interrupt matrix source",
        source_ref: "isrc.systimer1",
        interrupt_ref: "irq.ets_systimer_target1_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.systimer2",
        name: "SYSTIMER target2 interrupt matrix source",
        source_ref: "isrc.systimer2",
        interrupt_ref: "irq.ets_systimer_target2_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.dma0",
        name: "GDMA channel0 interrupt matrix source",
        source_ref: "isrc.dma0",
        interrupt_ref: "irq.ets_dma_ch0_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.dma1",
        name: "GDMA channel1 interrupt matrix source",
        source_ref: "isrc.dma1",
        interrupt_ref: "irq.ets_dma_ch1_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.dma2",
        name: "GDMA channel2 interrupt matrix source",
        source_ref: "isrc.dma2",
        interrupt_ref: "irq.ets_dma_ch2_intr_source",
        controller_ref: "block.interrupt_matrix0",
        cpu_target_ref: Some("block.cpu0"),
        line_index: None,
        route_type: "matrix",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
];
pub const DRV_IRQ_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_IRQ_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_IRQ_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_IRQ_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_IRQ_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_IRQ_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct InterruptMatrixResources {
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

pub const DRV_IRQ_RESOURCES: InterruptMatrixResources = InterruptMatrixResources {
    clocks: DRV_IRQ_CLOCK_BINDINGS,
    resets: DRV_IRQ_RESET_BINDINGS,
    interrupt_sources: DRV_IRQ_INTERRUPT_SOURCES,
    interrupts: DRV_IRQ_INTERRUPT_ROUTES,
    dma_channels: DRV_IRQ_DMA_CHANNELS,
    dma: DRV_IRQ_DMA_ROUTES,
    pins: DRV_IRQ_PIN_ROLES,
    init_operations: DRV_IRQ_INIT_OPERATIONS,
    state_machines: DRV_IRQ_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_IRQ_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct InterruptMatrix {
    resources: InterruptMatrixResources,
}

impl InterruptMatrix {
    pub fn new(resources: InterruptMatrixResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> InterruptMatrixResources {
        self.resources
    }
    pub fn bind(&self) -> &'static [metadata::InterruptRoute] {
        self.resources.interrupts
    }
}
