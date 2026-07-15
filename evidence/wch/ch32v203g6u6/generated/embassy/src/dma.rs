//! Generated Embassy-style dma module for CH32V203G6U6.

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
    module_name: "dma",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: DMA1 (dma) from canonical block block.dma1 -> dma-controller
pub const DRV_DMA1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.dma1",
    name: "DMA1 clock binding",
    consumer_ref: "periph.dma1",
    clock_ref: "clk.hclk",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.ahbpcenr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_DMA1_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_DMA1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource {
    id: "isrc.dma1.channel1",
    name: "DMA1 channel 1 interrupt source",
    source_ref: "dmach.dma1.ch1",
    producer_ref: None,
    kind: "dma",
    flag_refs: &[],
    clear_operation_refs: &[],
}];
pub const DRV_DMA1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute {
    id: "iroute.dma1.channel1",
    name: "DMA1 channel 1 interrupt route",
    source_ref: "isrc.dma1.channel1",
    interrupt_ref: "int.dma1channel1",
    controller_ref: "block.pfic",
    cpu_target_ref: None,
    line_index: None,
    route_type: "hardwired",
    control_refs: &[],
    acknowledge_operation_refs: &[],
    shared_group: None,
}];
pub const DRV_DMA1_DMA_CHANNELS: &[metadata::DmaChannel] = &[
    metadata::DmaChannel {
        id: "dmach.dma1.ch1",
        name: "DMA1 Channel 1",
        controller_ref: "block.dma1",
        target_ref: None,
        channel_index: 1,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dmach.dma1.ch2",
        name: "DMA1 Channel 2",
        controller_ref: "block.dma1",
        target_ref: None,
        channel_index: 2,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dmach.dma1.ch3",
        name: "DMA1 Channel 3",
        controller_ref: "block.dma1",
        target_ref: None,
        channel_index: 3,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dmach.dma1.ch4",
        name: "DMA1 Channel 4",
        controller_ref: "block.dma1",
        target_ref: None,
        channel_index: 4,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dmach.dma1.ch5",
        name: "DMA1 Channel 5",
        controller_ref: "block.dma1",
        target_ref: None,
        channel_index: 5,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dmach.dma1.ch6",
        name: "DMA1 Channel 6",
        controller_ref: "block.dma1",
        target_ref: None,
        channel_index: 6,
        capabilities: &[],
        priority_levels: &[],
    },
    metadata::DmaChannel {
        id: "dmach.dma1.ch7",
        name: "DMA1 Channel 7",
        controller_ref: "block.dma1",
        target_ref: None,
        channel_index: 7,
        capabilities: &[],
        priority_levels: &[],
    },
];
pub const DRV_DMA1_DMA_ROUTES: &[metadata::DmaRoute] = &[
    metadata::DmaRoute {
        id: "dmaroute.adc1.global",
        name: "ADC1 GLOBAL DMA route",
        peripheral_ref: "periph.adc1",
        signal: Some("GLOBAL"),
        channel_ref: "dmach.dma1.ch1",
        direction: "peripheral-to-memory",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.spi1.rx",
        name: "SPI1 RX DMA route",
        peripheral_ref: "periph.spi1",
        signal: Some("RX"),
        channel_ref: "dmach.dma1.ch2",
        direction: "peripheral-to-memory",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.spi1.tx",
        name: "SPI1 TX DMA route",
        peripheral_ref: "periph.spi1",
        signal: Some("TX"),
        channel_ref: "dmach.dma1.ch3",
        direction: "memory-to-peripheral",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.usart1.tx",
        name: "USART1 TX DMA route",
        peripheral_ref: "periph.usart1",
        signal: Some("TX"),
        channel_ref: "dmach.dma1.ch4",
        direction: "memory-to-peripheral",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.usart1.rx",
        name: "USART1 RX DMA route",
        peripheral_ref: "periph.usart1",
        signal: Some("RX"),
        channel_ref: "dmach.dma1.ch5",
        direction: "peripheral-to-memory",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.usart2.rx",
        name: "USART2 RX DMA route",
        peripheral_ref: "periph.usart2",
        signal: Some("RX"),
        channel_ref: "dmach.dma1.ch6",
        direction: "peripheral-to-memory",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.usart2.tx",
        name: "USART2 TX DMA route",
        peripheral_ref: "periph.usart2",
        signal: Some("TX"),
        channel_ref: "dmach.dma1.ch7",
        direction: "memory-to-peripheral",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.i2c1.tx",
        name: "I2C1 TX DMA route",
        peripheral_ref: "periph.i2c1",
        signal: Some("TX"),
        channel_ref: "dmach.dma1.ch6",
        direction: "memory-to-peripheral",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
    metadata::DmaRoute {
        id: "dmaroute.i2c1.rx",
        name: "I2C1 RX DMA route",
        peripheral_ref: "periph.i2c1",
        signal: Some("RX"),
        channel_ref: "dmach.dma1.ch7",
        direction: "peripheral-to-memory",
        control_refs: &[],
        shared_channel_group_ref: None,
    },
];
pub const DRV_DMA1_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_DMA1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_DMA1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_DMA1_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct DMA1Resources {
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

pub const DRV_DMA1_RESOURCES: DMA1Resources = DMA1Resources {
    clocks: DRV_DMA1_CLOCK_BINDINGS,
    resets: DRV_DMA1_RESET_BINDINGS,
    interrupt_sources: DRV_DMA1_INTERRUPT_SOURCES,
    interrupts: DRV_DMA1_INTERRUPT_ROUTES,
    dma_channels: DRV_DMA1_DMA_CHANNELS,
    dma: DRV_DMA1_DMA_ROUTES,
    pins: DRV_DMA1_PIN_ROLES,
    init_operations: DRV_DMA1_INIT_OPERATIONS,
    state_machines: DRV_DMA1_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_DMA1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct DMA1 {
    resources: DMA1Resources,
}

impl DMA1 {
    pub fn new(resources: DMA1Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> DMA1Resources {
        self.resources
    }
    /// Enable the DMA1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021014u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Disable the DMA1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021014u64, 0x00000001u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn prepare_transfer_complete_wait(
        &self,
        channel_index: u32,
    ) -> Result<(), metadata::Error> {
        generated_drv_dma1_prepare_transfer_complete_wait(channel_index)
    }

    pub async fn wait_transfer_complete(&self, channel_index: u32) -> Result<(), metadata::Error> {
        generated_drv_dma1_wait_transfer_complete(channel_index).await
    }

    pub fn enable_transfer_complete_interrupt(
        &self,
        channel_index: u32,
    ) -> Result<(), metadata::Error> {
        match channel_index {
            1 => {
                modify_u32(0x40020008u64, 0x00000002u32, 0x00000002u32)?;
                Ok(())
            }
            _ => Err(metadata::Error::InvalidReference(
                "DMA async completion is not bound for the requested channel",
            )),
        }
    }

    pub fn disable_transfer_complete_interrupt(
        &self,
        channel_index: u32,
    ) -> Result<(), metadata::Error> {
        match channel_index {
            1 => {
                modify_u32(0x40020008u64, 0x00000002u32, 0x00000000u32)?;
                Ok(())
            }
            _ => Err(metadata::Error::InvalidReference(
                "DMA async completion is not bound for the requested channel",
            )),
        }
    }

    pub fn is_transfer_complete(&self, channel_index: u32) -> Result<bool, metadata::Error> {
        match channel_index {
            1 => Ok((read_u32(0x40020000u64)? & 0x00000002u32) != 0),
            _ => Err(metadata::Error::InvalidReference(
                "DMA async completion is not bound for the requested channel",
            )),
        }
    }

    pub fn clear_transfer_complete(&self, channel_index: u32) -> Result<(), metadata::Error> {
        match channel_index {
            1 => {
                modify_u32(0x40020004u64, 0x00000002u32, 0x00000002u32)?;
                Ok(())
            }
            _ => Err(metadata::Error::InvalidReference(
                "DMA async completion is not bound for the requested channel",
            )),
        }
    }

    pub fn on_interrupt(&self, channel_index: u32) -> Result<(), metadata::Error> {
        match channel_index {
            1 => {
                if (read_u32(0x40020000u64)? & 0x00000002u32) != 0 {
                    modify_u32(0x40020004u64, 0x00000002u32, 0x00000002u32)?;
                    generated_drv_dma1_signal_transfer_complete(1)?;
                }
                Ok(())
            }
            _ => Err(metadata::Error::InvalidReference(
                "DMA async completion is not bound for the requested channel",
            )),
        }
    }
}

#[derive(Debug)]
struct GeneratedDMA1DmaWaitState {
    transfer_complete_ready: bool,
    transfer_complete_waker: Option<core::task::Waker>,
}

impl GeneratedDMA1DmaWaitState {
    const fn new() -> Self {
        Self {
            transfer_complete_ready: false,
            transfer_complete_waker: None,
        }
    }
}

const GENERATED_DRV_DMA1_DMA_WAIT_STATE: &str =
    "DMA async completion is not bound for the requested channel";
static GENERATED_DRV_DMA1_DMA_CH1_WAIT_STATE: critical_section::Mutex<
    core::cell::RefCell<GeneratedDMA1DmaWaitState>,
> = critical_section::Mutex::new(core::cell::RefCell::new(GeneratedDMA1DmaWaitState::new()));

fn generated_drv_dma1_prepare_transfer_complete_wait(
    channel_index: u32,
) -> Result<(), metadata::Error> {
    match channel_index {
        1 => critical_section::with(|cs| {
            let mut state = GENERATED_DRV_DMA1_DMA_CH1_WAIT_STATE
                .borrow(cs)
                .borrow_mut();
            state.transfer_complete_ready = false;
            state.transfer_complete_waker = None;
            Ok(())
        }),
        _ => Err(metadata::Error::InvalidReference(
            GENERATED_DRV_DMA1_DMA_WAIT_STATE,
        )),
    }
}

async fn generated_drv_dma1_wait_transfer_complete(
    channel_index: u32,
) -> Result<(), metadata::Error> {
    core::future::poll_fn(|cx| match channel_index {
        1 => {
            let ready = critical_section::with(|cs| {
                let mut state = GENERATED_DRV_DMA1_DMA_CH1_WAIT_STATE
                    .borrow(cs)
                    .borrow_mut();
                if state.transfer_complete_ready {
                    state.transfer_complete_ready = false;
                    true
                } else {
                    state.transfer_complete_waker = Some(cx.waker().clone());
                    false
                }
            });
            if ready {
                core::task::Poll::Ready(Ok(()))
            } else {
                core::task::Poll::Pending
            }
        }
        _ => core::task::Poll::Ready(Err(metadata::Error::InvalidReference(
            GENERATED_DRV_DMA1_DMA_WAIT_STATE,
        ))),
    })
    .await
}

fn generated_drv_dma1_signal_transfer_complete(channel_index: u32) -> Result<(), metadata::Error> {
    match channel_index {
        1 => {
            let waker = critical_section::with(|cs| {
                let mut state = GENERATED_DRV_DMA1_DMA_CH1_WAIT_STATE
                    .borrow(cs)
                    .borrow_mut();
                state.transfer_complete_ready = true;
                state.transfer_complete_waker.take()
            });
            if let Some(waker) = waker {
                waker.wake();
            }
            Ok(())
        }
        _ => Err(metadata::Error::InvalidReference(
            GENERATED_DRV_DMA1_DMA_WAIT_STATE,
        )),
    }
}
