//! Generated Embassy-style usb module for CH32V203G6U6.

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
    module_name: "usb",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: USBD (usb-device) from canonical block block.usbd -> usb-device
pub const DRV_USBD_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clk.usbd", name: "USBD clock binding", consumer_ref: "periph.usbfsd", clock_ref: "clk.pclk1", controller_ref: Some("block.rcc"), binding_kind: "gated", control_refs: &["reg.rcc.apb1pcenr"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_USBD_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rst.usbd", name: "USBD reset binding", target_ref: "periph.usbfsd", controller_ref: Some("block.rcc"), reset_domain_ref: Some("rst.apb1"), binding_kind: "local", control_refs: &["reg.rcc.apb1prstr"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_USBD_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.usbd.hp", name: "USBD HP interrupt source", source_ref: "periph.usbfsd", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.usbd.lp", name: "USBD LP interrupt source", source_ref: "periph.usbfsd", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }, metadata::InterruptSource { id: "isrc.usbd.wkup", name: "USBD WKUP interrupt source", source_ref: "periph.usbfsd", producer_ref: None, kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_USBD_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.usbd.hp", name: "USBD HP interrupt route", source_ref: "isrc.usbd.hp", interrupt_ref: "int.usbhpcan1tx", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.usbd.lp", name: "USBD LP interrupt route", source_ref: "isrc.usbd.lp", interrupt_ref: "int.usblpcan1rx0", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }, metadata::InterruptRoute { id: "iroute.usbd.wkup", name: "USBD WKUP interrupt route", source_ref: "isrc.usbd.wkup", interrupt_ref: "int.usbwakeup", controller_ref: "block.pfic", cpu_target_ref: None, line_index: None, route_type: "hardwired", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_USBD_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_USBD_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_USBD_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usbd.dm.pa11", name: "USBD DM on PA11", pin_ref: "pin.pa11", peripheral_ref: "periph.usbfsd", signal: "DM", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_USBD_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usbd.dp.pa12", name: "USBD DP on PA12", pin_ref: "pin.pa12", peripheral_ref: "periph.usbfsd", signal: "DP", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_USBD_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "dm", signal: "DM", routes: DRV_USBD_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "dp", signal: "DP", routes: DRV_USBD_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_USBD_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[];
pub const DRV_USBD_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USBD_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct USBDResources {
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

pub const DRV_USBD_RESOURCES: USBDResources = USBDResources {
    clocks: DRV_USBD_CLOCK_BINDINGS,
    resets: DRV_USBD_RESET_BINDINGS,
    interrupt_sources: DRV_USBD_INTERRUPT_SOURCES,
    interrupts: DRV_USBD_INTERRUPT_ROUTES,
    dma_channels: DRV_USBD_DMA_CHANNELS,
    dma: DRV_USBD_DMA_ROUTES,
    pins: DRV_USBD_PIN_ROLES,
    init_operations: DRV_USBD_INIT_OPERATIONS,
    state_machines: DRV_USBD_STATE_MACHINES,
    lowering_pattern: Some("fsdev-pma-btable"),
    time_driver_source: None,
    capability_tags: DRV_USBD_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct USBD {
    resources: USBDResources,
}

impl USBD {
    pub fn new(resources: USBDResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> USBDResources {
        self.resources
    }
    /// Enable the USBD clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00800000u32, 0x00800000u32)?;
        Ok(())
    }

    /// Disable the USBD clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00800000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USBD.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00800000u32, 0x00800000u32)?;
        Ok(())
    }

    /// Release reset for USBD.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00800000u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn embassy_usb_driver(&self) -> USBDUsbDriver {
        USBDUsbDriver::new(self.resources)
    }
}


use core::{cell::RefCell, future::poll_fn, task::Poll};
use embassy_usb_driver::{Bus, ControlPipe, Direction, Driver, Endpoint, EndpointAddress, EndpointAllocError, EndpointError, EndpointInfo, EndpointIn, EndpointOut, EndpointType, Event, Unsupported};

const USB_BASE: usize = 0x4000_5C00;
const USBRAM_BASE: usize = 0x4000_6000;
const RCC_APB1PCENR: u64 = 0x4002_101C;
const RCC_APB1PRSTR: u64 = 0x4002_100C;
const EXTEN_CTR: u64 = 0x4002_3800;
const RCC_APB1_USB_EN: u32 = 1 << 23;
const RCC_APB1_USB_RST: u32 = 1 << 23;
const EXTEN_USBD_PU_EN: u32 = 1 << 1;
const USB_DYNAMIC_PMA_START: u16 = 0x00C0;
const USB_DETACH_DELAY_MS: u16 = 250;

const USB_CNTR_OFFSET: usize = 0x40;
const USB_ISTR_OFFSET: usize = 0x44;
const USB_DADDR_OFFSET: usize = 0x4C;
const USB_BTABLE_OFFSET: usize = 0x50;

const USB_CNTR_FRES: u16 = 1 << 0;
const USB_CNTR_PDWN: u16 = 1 << 1;
const USB_CNTR_LPMODE: u16 = 1 << 2;
const USB_CNTR_FSUSP: u16 = 1 << 3;
const USB_CNTR_ESOFM: u16 = 1 << 8;
const USB_CNTR_RESETM: u16 = 1 << 10;
const USB_CNTR_SUSPM: u16 = 1 << 11;
const USB_CNTR_WKUPM: u16 = 1 << 12;
const USB_CNTR_PMAOVRM: u16 = 1 << 14;
const USB_CNTR_CTRM: u16 = 1 << 15;
const USB_CNTR_INIT_MASK: u16 = USB_CNTR_RESETM
    | USB_CNTR_ESOFM
    | USB_CNTR_CTRM
    | USB_CNTR_SUSPM
    | USB_CNTR_WKUPM
    | USB_CNTR_PMAOVRM;

const USB_ISTR_PMAOVR: u16 = 1 << 14;
const USB_ISTR_ERR: u16 = 1 << 13;
const USB_ISTR_WKUP: u16 = 1 << 12;
const USB_ISTR_SUSP: u16 = 1 << 11;
const USB_ISTR_RESET: u16 = 1 << 10;
const USB_ISTR_SOF: u16 = 1 << 9;
const USB_ISTR_ESOF: u16 = 1 << 8;

const USB_DADDR_EF: u16 = 1 << 7;

const EP_CTR_RX: u16 = 1 << 15;
const EP_DTOG_RX: u16 = 1 << 14;
const EP_STAT_RX_MASK: u16 = 0b11 << 12;
const EP_SETUP: u16 = 1 << 11;
const EP_TYPE_MASK: u16 = 0b11 << 9;
const EP_KIND: u16 = 1 << 8;
const EP_CTR_TX: u16 = 1 << 7;
const EP_DTOG_TX: u16 = 1 << 6;
const EP_STAT_TX_MASK: u16 = 0b11 << 4;
const EP_ADDR_MASK: u16 = 0x000F;

const EP_TYPE_BULK: u16 = 0b00 << 9;
const EP_TYPE_CONTROL: u16 = 0b01 << 9;
const EP_TYPE_INTERRUPT: u16 = 0b11 << 9;

const EP_STAT_STALL: u8 = 1;
const EP_STAT_NAK: u8 = 2;
const EP_STAT_VALID: u8 = 3;

const BTABLE_FIELD_ADDR_TX: u16 = 0;
const BTABLE_FIELD_COUNT_TX: u16 = 2;
const BTABLE_FIELD_ADDR_RX: u16 = 4;
const BTABLE_FIELD_COUNT_RX: u16 = 6;

const EP0_PACKET_SIZE: u16 = 64;
const EP0_SETUP_SIZE: u16 = 8;
const PMA_EP0_RX_ADDR: u16 = 0x0040;
const PMA_EP0_TX_ADDR: u16 = 0x0080;

#[derive(Debug, Clone, Copy)]
struct FsdevDirConfig {
    allocated: bool,
    enabled: bool,
    stalled: bool,
    busy: bool,
    pma_addr: u16,
    max_packet_size: u16,
    interval_ms: u8,
}

impl FsdevDirConfig {
    const fn new() -> Self {
        Self {
            allocated: false,
            enabled: false,
            stalled: false,
            busy: false,
            pma_addr: 0,
            max_packet_size: 0,
            interval_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FsdevEndpointPair {
    ep_type: u16,
    in_dir: FsdevDirConfig,
    out_dir: FsdevDirConfig,
}

impl FsdevEndpointPair {
    const fn new() -> Self {
        Self {
            ep_type: 0,
            in_dir: FsdevDirConfig::new(),
            out_dir: FsdevDirConfig::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FsdevRuntimeState {
    pairs: [FsdevEndpointPair; 8],
    control_max_packet_size: u16,
}

impl FsdevRuntimeState {
    const fn new() -> Self {
        Self {
            pairs: [FsdevEndpointPair::new(); 8],
            control_max_packet_size: EP0_PACKET_SIZE,
        }
    }
}

static FSDEV_RUNTIME: critical_section::Mutex<RefCell<FsdevRuntimeState>> =
    critical_section::Mutex::new(RefCell::new(FsdevRuntimeState::new()));

fn with_fsdev_runtime<R>(f: impl FnOnce(&mut FsdevRuntimeState) -> R) -> R {
    critical_section::with(|cs| {
        let mut runtime = FSDEV_RUNTIME.borrow(cs).borrow_mut();
        f(&mut runtime)
    })
}

fn dir_config_mut(pair: &mut FsdevEndpointPair, direction: Direction) -> &mut FsdevDirConfig {
    match direction {
        Direction::In => &mut pair.in_dir,
        Direction::Out => &mut pair.out_dir,
    }
}

fn dir_config(pair: &FsdevEndpointPair, direction: Direction) -> &FsdevDirConfig {
    match direction {
        Direction::In => &pair.in_dir,
        Direction::Out => &pair.out_dir,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct USBDUsbDriver {
    pairs: [FsdevEndpointPair; 8],
    next_pma_addr: u16,
}

impl USBDUsbDriver {
    fn new(resources: USBDResources) -> Self {
        let _ = resources;
        Self {
            pairs: [FsdevEndpointPair::new(); 8],
            next_pma_addr: USB_DYNAMIC_PMA_START,
        }
    }

    fn allocate_endpoint(
        &mut self,
        direction: Direction,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<EndpointInfo, EndpointAllocError> {
        let type_bits = endpoint_type_bits(ep_type)?;
        let index = if let Some(addr) = ep_addr {
            if addr.direction() != direction || addr.index() == 0 || addr.index() >= self.pairs.len() {
                return Err(EndpointAllocError);
            }
            let pair = &self.pairs[addr.index()];
            if pair.ep_type != 0 && pair.ep_type != type_bits {
                return Err(EndpointAllocError);
            }
            if dir_config(pair, direction).allocated {
                return Err(EndpointAllocError);
            }
            addr.index()
        } else if let Some(index) = (1..self.pairs.len()).find(|&index| {
            let pair = &self.pairs[index];
            pair.ep_type == type_bits
                && !dir_config(pair, direction).allocated
                && dir_config(pair, opposite_direction(direction)).allocated
        }) {
            index
        } else if let Some(index) =
            (1..self.pairs.len()).find(|&index| self.pairs[index].ep_type == 0)
        {
            index
        } else {
            return Err(EndpointAllocError);
        };

        let pair = &mut self.pairs[index];
        if pair.ep_type == 0 {
            pair.ep_type = type_bits;
        }
        let dir = dir_config_mut(pair, direction);
        dir.allocated = true;
        dir.enabled = false;
        dir.stalled = false;
        dir.busy = false;
        dir.pma_addr = self.next_pma_addr;
        dir.max_packet_size = max_packet_size;
        dir.interval_ms = interval_ms;
        self.next_pma_addr = self
            .next_pma_addr
            .saturating_add(pma_allocation_size(max_packet_size));

        Ok(EndpointInfo {
            addr: EndpointAddress::from_parts(index, direction),
            ep_type,
            max_packet_size,
            interval_ms,
        })
    }
}

impl<'d> Driver<'d> for USBDUsbDriver {
    type EndpointOut = USBDEndpointOut;
    type EndpointIn = USBDEndpointIn;
    type ControlPipe = USBDControlPipe;
    type Bus = USBDUsbBus;

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        let info = self.allocate_endpoint(Direction::Out, ep_type, ep_addr, max_packet_size, interval_ms)?;
        Ok(USBDEndpointOut { info })
    }

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        let info = self.allocate_endpoint(Direction::In, ep_type, ep_addr, max_packet_size, interval_ms)?;
        Ok(USBDEndpointIn { info })
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        with_fsdev_runtime(|runtime| {
            runtime.pairs = self.pairs;
            runtime.control_max_packet_size = control_max_packet_size;
        });
        (
            USBDUsbBus { power_reported: false },
            USBDControlPipe {
                max_packet_size: control_max_packet_size as usize,
            },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct USBDUsbBus {
    power_reported: bool,
}

impl Bus for USBDUsbBus {
    async fn enable(&mut self) {
        let _ = modify_u32(RCC_APB1PCENR, 0, RCC_APB1_USB_EN);
        usb_disconnect();
        delay_ms(USB_DETACH_DELAY_MS);
        let _ = modify_u32(RCC_APB1PRSTR, 0, RCC_APB1_USB_RST);
        spin_delay(1_000);
        let _ = modify_u32(RCC_APB1PRSTR, RCC_APB1_USB_RST, 0);
        fsdev_core_reset();
        usb_write16(USB_CNTR_OFFSET, 0);
        usb_write16(USB_BTABLE_OFFSET, 0);
        usb_write16(USB_CNTR_OFFSET, USB_CNTR_INIT_MASK);
        handle_bus_reset_runtime();
        usb_connect();
    }

    async fn disable(&mut self) {
        usb_disconnect();
        usb_write16(USB_CNTR_OFFSET, USB_CNTR_FRES | USB_CNTR_PDWN);
        let _ = modify_u32(RCC_APB1PCENR, RCC_APB1_USB_EN, 0);
    }

    async fn poll(&mut self) -> Event {
        if !self.power_reported {
            self.power_reported = true;
            return Event::PowerDetected;
        }

        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            let status = usb_read16(USB_ISTR_OFFSET);
            if (status & USB_ISTR_RESET) != 0 {
                clear_istr_exact(USB_ISTR_RESET);
                handle_bus_reset_runtime();
                return Poll::Ready(Event::Reset);
            }
            if (status & USB_ISTR_WKUP) != 0 {
                usb_write16(
                    USB_CNTR_OFFSET,
                    usb_read16(USB_CNTR_OFFSET) & !(USB_CNTR_LPMODE | USB_CNTR_FSUSP),
                );
                clear_istr_exact(USB_ISTR_WKUP);
                return Poll::Ready(Event::Resume);
            }
            if (status & USB_ISTR_SUSP) != 0 {
                let mut cntr = usb_read16(USB_CNTR_OFFSET);
                cntr |= USB_CNTR_FSUSP | USB_CNTR_LPMODE;
                usb_write16(USB_CNTR_OFFSET, cntr);
                clear_istr_exact(USB_ISTR_SUSP);
                return Poll::Ready(Event::Suspend);
            }
            if (status & USB_ISTR_ESOF) != 0 {
                clear_istr_exact(USB_ISTR_ESOF);
            }
            if (status & USB_ISTR_SOF) != 0 {
                clear_istr_exact(USB_ISTR_SOF);
            }
            if (status & USB_ISTR_ERR) != 0 {
                clear_istr_exact(USB_ISTR_ERR);
            }
            if (status & USB_ISTR_PMAOVR) != 0 {
                clear_istr_exact(USB_ISTR_PMAOVR);
            }
            Poll::Pending
        })
        .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        let index = ep_addr.index() as u8;
        let direction = ep_addr.direction();
        with_fsdev_runtime(|runtime| {
            let pair = &mut runtime.pairs[index as usize];
            dir_config_mut(pair, direction).enabled = enabled;
            dir_config_mut(pair, direction).busy = false;
            dir_config_mut(pair, direction).stalled = false;
            if enabled {
                if direction == Direction::In {
                    open_in_endpoint(index, pair.ep_type, dir_config(pair, Direction::In).pma_addr, dir_config(pair, Direction::In).max_packet_size);
                } else {
                    open_out_endpoint(index, pair.ep_type, dir_config(pair, Direction::Out).pma_addr, dir_config(pair, Direction::Out).max_packet_size);
                }
            } else if dir_config(pair, opposite_direction(direction)).enabled {
                if direction == Direction::In {
                    open_out_endpoint(index, pair.ep_type, dir_config(pair, Direction::Out).pma_addr, dir_config(pair, Direction::Out).max_packet_size);
                } else {
                    open_in_endpoint(index, pair.ep_type, dir_config(pair, Direction::In).pma_addr, dir_config(pair, Direction::In).max_packet_size);
                }
            } else {
                epr_write(index, 0);
            }
        });
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        with_fsdev_runtime(|runtime| {
            let pair = &mut runtime.pairs[ep_addr.index()];
            dir_config_mut(pair, ep_addr.direction()).stalled = stalled;
        });
        if stalled {
            dcd_edpt_stall(u8::from(ep_addr));
        } else {
            dcd_edpt_clear_stall(u8::from(ep_addr));
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        with_fsdev_runtime(|runtime| dir_config(&runtime.pairs[ep_addr.index()], ep_addr.direction()).stalled)
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct USBDEndpointOut {
    info: EndpointInfo,
}

#[derive(Debug, Clone, Copy)]
pub struct USBDEndpointIn {
    info: EndpointInfo,
}

impl Endpoint for USBDEndpointOut {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        wait_endpoint_enabled(self.info.addr).await
    }
}

impl Endpoint for USBDEndpointIn {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        wait_endpoint_enabled(self.info.addr).await
    }
}

impl EndpointOut for USBDEndpointOut {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let index = self.info.addr.index() as u8;
        arm_out_transfer(index, self.info.max_packet_size);
        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            if !endpoint_enabled(self.info.addr) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            let ep_reg = epr_read(index);
            if (ep_reg & EP_CTR_RX) == 0 {
                return Poll::Pending;
            }
            ep_write_clear_ctr(index, false);
            let count = ep_rx_count(index);
            if count > buf.len() {
                arm_out_transfer(index, self.info.max_packet_size);
                return Poll::Ready(Err(EndpointError::BufferOverflow));
            }
            pma_read_bytes(ep_rx_addr(index), &mut buf[..count]);
            arm_out_transfer(index, self.info.max_packet_size);
            Poll::Ready(Ok(count))
        })
        .await
    }
}

impl EndpointIn for USBDEndpointIn {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        if buf.len() > self.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }
        let index = self.info.addr.index() as u8;
        let pma_addr = with_fsdev_runtime(|runtime| runtime.pairs[index as usize].in_dir.pma_addr);
        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            if !endpoint_enabled(self.info.addr) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            if endpoint_busy(self.info.addr) {
                if (epr_read(index) & EP_CTR_TX) != 0 {
                    ep_write_clear_ctr(index, true);
                    set_endpoint_busy(self.info.addr, false);
                    return Poll::Ready(Ok(()));
                }
                return Poll::Pending;
            }
            if !buf.is_empty() {
                unsafe { pma_write_bytes(pma_addr, buf) };
            }
            ep_tx_count(index, buf.len() as u16);
            let mut ep_reg = epr_read(index) | EP_CTR_TX | EP_CTR_RX;
            ep_change_status(&mut ep_reg, true, EP_STAT_VALID);
            ep_reg &= u_epreg_mask() | EP_STAT_TX_MASK;
            epr_write(index, ep_reg);
            set_endpoint_busy(self.info.addr, true);
            Poll::Pending
        })
        .await
    }
}

#[derive(Debug, Clone, Copy)]
pub struct USBDControlPipe {
    max_packet_size: usize,
}

impl ControlPipe for USBDControlPipe {
    fn max_packet_size(&self) -> usize {
        self.max_packet_size
    }

    async fn setup(&mut self) -> [u8; 8] {
        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            let ep_reg = epr_read(0);
            if (ep_reg & EP_CTR_RX) == 0 || (ep_reg & EP_SETUP) == 0 {
                return Poll::Pending;
            }
            let mut setup = [0u8; 8];
            pma_read_bytes(ep_rx_addr(0), &mut setup);
            ep_write_clear_ctr(0, false);
            let direction_in = (setup[0] & 0x80) != 0;
            let has_data = u16::from_le_bytes([setup[6], setup[7]]) != 0;
            if !direction_in && has_data {
                ep0_set_type(EP_TYPE_BULK);
            } else {
                ep0_set_type(EP_TYPE_CONTROL);
            }
            Poll::Ready(setup)
        })
        .await
    }

    async fn data_out(&mut self, buf: &mut [u8], _first: bool, _last: bool) -> Result<usize, EndpointError> {
        arm_ep0_out_data(buf.len() as u16);
        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            let ep_reg = epr_read(0);
            if (ep_reg & EP_CTR_RX) == 0 {
                return Poll::Pending;
            }
            if (ep_reg & EP_SETUP) != 0 {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            let count = ep_rx_count(0);
            ep_write_clear_ctr(0, false);
            if count > buf.len() {
                return Poll::Ready(Err(EndpointError::BufferOverflow));
            }
            pma_read_bytes(ep_rx_addr(0), &mut buf[..count]);
            Poll::Ready(Ok(count))
        })
        .await
    }

    async fn data_in(&mut self, data: &[u8], _first: bool, last: bool) -> Result<(), EndpointError> {
        if data.len() > self.max_packet_size {
            return Err(EndpointError::BufferOverflow);
        }
        if !data.is_empty() {
            unsafe { pma_write_bytes(ep_tx_addr(0), data) };
        }
        ep_tx_count(0, data.len() as u16);
        let mut ep_reg = epr_read(0) | EP_CTR_TX | EP_CTR_RX;
        ep_change_status(&mut ep_reg, true, EP_STAT_VALID);
        ep_reg &= u_epreg_mask() | EP_STAT_TX_MASK;
        epr_write(0, ep_reg);
        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            let ep_reg = epr_read(0);
            if (ep_reg & EP_CTR_TX) == 0 {
                return Poll::Pending;
            }
            ep_write_clear_ctr(0, true);
            if !last {
                return Poll::Ready(Ok(()));
            }
            ep0_set_type(EP_TYPE_BULK);
            arm_ep0_status_out();
            Poll::Ready(Ok(()))
        })
        .await?;

        if !last {
            return Ok(());
        }

        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            let ep_reg = epr_read(0);
            if (ep_reg & EP_CTR_RX) == 0 {
                return Poll::Pending;
            }
            if (ep_reg & EP_SETUP) != 0 {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            ep_write_clear_ctr(0, false);
            ep0_set_type(EP_TYPE_CONTROL);
            edpt0_prepare_setup();
            Poll::Ready(Ok(()))
        })
        .await
    }

    async fn accept(&mut self) {
        ep0_set_type(EP_TYPE_CONTROL);
        ep_tx_count(0, 0);
        let mut ep_reg = epr_read(0) | EP_CTR_TX | EP_CTR_RX;
        ep_change_status(&mut ep_reg, true, EP_STAT_VALID);
        ep_reg &= u_epreg_mask() | EP_STAT_TX_MASK;
        epr_write(0, ep_reg);
        poll_fn(|cx| {
            cx.waker().wake_by_ref();
            if (epr_read(0) & EP_CTR_TX) == 0 {
                return Poll::Pending;
            }
            ep_write_clear_ctr(0, true);
            edpt0_prepare_setup();
            Poll::Ready(())
        })
        .await
    }

    async fn reject(&mut self) {
        dcd_edpt_stall(0x00);
        dcd_edpt_stall(0x80);
    }

    async fn accept_set_address(&mut self, addr: u8) {
        self.accept().await;
        usb_write16(USB_DADDR_OFFSET, USB_DADDR_EF | u16::from(addr & 0x7F));
    }
}

async fn wait_endpoint_enabled(ep_addr: EndpointAddress) {
    poll_fn(|cx| {
        cx.waker().wake_by_ref();
        if endpoint_enabled(ep_addr) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await
}

fn endpoint_enabled(ep_addr: EndpointAddress) -> bool {
    with_fsdev_runtime(|runtime| dir_config(&runtime.pairs[ep_addr.index()], ep_addr.direction()).enabled)
}

fn endpoint_busy(ep_addr: EndpointAddress) -> bool {
    with_fsdev_runtime(|runtime| dir_config(&runtime.pairs[ep_addr.index()], ep_addr.direction()).busy)
}

fn set_endpoint_busy(ep_addr: EndpointAddress, busy: bool) {
    with_fsdev_runtime(|runtime| {
        dir_config_mut(&mut runtime.pairs[ep_addr.index()], ep_addr.direction()).busy = busy;
    });
}

fn opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::In => Direction::Out,
        Direction::Out => Direction::In,
    }
}

fn endpoint_type_bits(ep_type: EndpointType) -> Result<u16, EndpointAllocError> {
    match ep_type {
        EndpointType::Bulk => Ok(EP_TYPE_BULK),
        EndpointType::Interrupt => Ok(EP_TYPE_INTERRUPT),
        EndpointType::Control => Ok(EP_TYPE_CONTROL),
        EndpointType::Isochronous => Err(EndpointAllocError),
    }
}

fn pma_allocation_size(max_packet_size: u16) -> u16 {
    (max_packet_size + 1) & !1
}

fn spin_delay(iterations: u32) {
    for _ in 0..iterations {
        core::hint::spin_loop();
    }
}

fn delay_ms(ms: u16) {
    for _ in 0..ms {
        spin_delay(48_000);
    }
}

fn usb_connect() {
    let _ = modify_u32(EXTEN_CTR, 0, EXTEN_USBD_PU_EN);
}

fn usb_disconnect() {
    let _ = modify_u32(EXTEN_CTR, EXTEN_USBD_PU_EN, 0);
}

fn fsdev_core_reset() {
    usb_write16(USB_CNTR_OFFSET, USB_CNTR_FRES | USB_CNTR_PDWN);
    spin_delay(200);
    usb_write16(USB_CNTR_OFFSET, USB_CNTR_FRES);
    spin_delay(200);
    usb_write16(USB_ISTR_OFFSET, 0);
}

fn handle_bus_reset_runtime() {
    with_fsdev_runtime(|runtime| {
        for pair in &mut runtime.pairs {
            pair.in_dir.enabled = false;
            pair.in_dir.busy = false;
            pair.out_dir.enabled = false;
            pair.out_dir.busy = false;
            pair.in_dir.stalled = false;
            pair.out_dir.stalled = false;
        }
    });
    for index in 1..8 {
        epr_write(index, 0);
    }
    usb_write16(USB_DADDR_OFFSET, 0);
    edpt0_open();
    usb_write16(USB_DADDR_OFFSET, USB_DADDR_EF);
}

fn edpt0_open() {
    btable_write(0, BTABLE_FIELD_ADDR_RX, PMA_EP0_RX_ADDR);
    btable_write(0, BTABLE_FIELD_ADDR_TX, PMA_EP0_TX_ADDR);
    let mut ep_reg = epr_read(0) & !u_epreg_mask();
    ep_reg |= EP_TYPE_CONTROL;
    ep_change_status(&mut ep_reg, true, EP_STAT_NAK);
    ep_change_status(&mut ep_reg, false, EP_STAT_NAK);
    edpt0_prepare_setup();
    epr_write(0, ep_reg);
}

fn edpt0_prepare_setup() {
    btable_set_rx_bufsize(0, EP0_SETUP_SIZE);
}

fn arm_ep0_out_data(size: u16) {
    btable_set_rx_bufsize(0, size.min(EP0_PACKET_SIZE));
    let mut ep_reg = epr_read(0) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask() | EP_STAT_RX_MASK;
    ep_change_status(&mut ep_reg, false, EP_STAT_VALID);
    epr_write(0, ep_reg);
}

fn arm_ep0_status_out() {
    edpt0_prepare_setup();
    let mut ep_reg = epr_read(0) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask() | EP_STAT_RX_MASK;
    ep_change_status(&mut ep_reg, false, EP_STAT_VALID);
    epr_write(0, ep_reg);
}

fn arm_out_transfer(index: u8, max_packet_size: u16) {
    btable_set_rx_bufsize(index, max_packet_size);
    let mut ep_reg = epr_read(index) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask() | EP_STAT_RX_MASK;
    ep_change_status(&mut ep_reg, false, EP_STAT_VALID);
    epr_write(index, ep_reg);
}

fn open_in_endpoint(ep_num: u8, ep_type: u16, pma_addr: u16, packet_size: u16) {
    btable_write(ep_num, BTABLE_FIELD_ADDR_TX, pma_addr);
    let mut ep_reg = epr_read(ep_num) & !u_epreg_mask();
    ep_reg |= u16::from(ep_num) | ep_type;
    ep_change_status(&mut ep_reg, true, EP_STAT_NAK);
    ep_reg &= !(EP_STAT_RX_MASK | EP_DTOG_RX);
    ep_tx_count(ep_num, 0);
    let _ = packet_size;
    epr_write(ep_num, ep_reg);
}

fn open_out_endpoint(ep_num: u8, ep_type: u16, pma_addr: u16, packet_size: u16) {
    btable_write(ep_num, BTABLE_FIELD_ADDR_RX, pma_addr);
    let mut ep_reg = epr_read(ep_num) & !u_epreg_mask();
    ep_reg |= u16::from(ep_num) | ep_type;
    ep_change_status(&mut ep_reg, false, EP_STAT_NAK);
    ep_reg &= !(EP_STAT_TX_MASK | EP_DTOG_TX);
    btable_set_rx_bufsize(ep_num, packet_size);
    epr_write(ep_num, ep_reg);
}

fn dcd_edpt_stall(ep_addr: u8) {
    let ep_num = ep_addr & 0x7F;
    let dir_in = (ep_addr & 0x80) != 0;
    let mut ep_reg = epr_read(ep_num) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask() | if dir_in { EP_STAT_TX_MASK } else { EP_STAT_RX_MASK };
    if dir_in {
        ep_change_status(&mut ep_reg, true, EP_STAT_STALL);
    } else {
        ep_change_status(&mut ep_reg, false, EP_STAT_STALL);
    }
    if ep_num == 0 {
        ep_reg = (ep_reg & !EP_TYPE_MASK) | EP_TYPE_CONTROL;
    }
    epr_write(ep_num, ep_reg);
}

fn dcd_edpt_clear_stall(ep_addr: u8) {
    let ep_num = ep_addr & 0x7F;
    let dir_in = (ep_addr & 0x80) != 0;
    let mut ep_reg = epr_read(ep_num) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask()
        | if dir_in {
            EP_STAT_TX_MASK | EP_DTOG_TX
        } else {
            EP_STAT_RX_MASK | EP_DTOG_RX
        };
    if dir_in {
        ep_change_status(&mut ep_reg, true, EP_STAT_NAK);
    } else {
        ep_change_status(&mut ep_reg, false, EP_STAT_VALID);
    }
    if ep_num == 0 {
        ep_reg = (ep_reg & !EP_TYPE_MASK) | EP_TYPE_CONTROL;
    }
    epr_write(ep_num, ep_reg);
}

fn ep0_set_type(ep_type: u16) {
    let mut ep_reg = epr_read(0) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask();
    ep_reg = (ep_reg & !EP_TYPE_MASK) | ep_type;
    epr_write(0, ep_reg);
}

#[inline(always)]
const fn u_epreg_mask() -> u16 {
    EP_CTR_RX | EP_SETUP | EP_TYPE_MASK | EP_KIND | EP_CTR_TX | EP_ADDR_MASK
}

fn ep_change_status(reg: &mut u16, dir_in: bool, state: u8) {
    *reg ^= u16::from(state) << if dir_in { 4 } else { 12 };
}

fn ep_write_clear_ctr(ep: u8, dir_in: bool) {
    let mut reg = epr_read(ep);
    reg |= EP_CTR_TX | EP_CTR_RX;
    reg &= u_epreg_mask();
    reg &= !(if dir_in { EP_CTR_TX } else { EP_CTR_RX });
    epr_write(ep, reg);
}

fn clear_istr_exact(mask: u16) {
    usb_write16(USB_ISTR_OFFSET, !mask);
}

fn btable_write(ep: u8, field_offset: u16, value: u16) {
    pma_write16(u16::from(ep) * 8 + field_offset, value);
}

fn btable_read(ep: u8, field_offset: u16) -> u16 {
    pma_read16(u16::from(ep) * 8 + field_offset)
}

fn btable_set_rx_bufsize(ep: u8, size: u16) {
    let (blsize, num_block) = if size > 62 {
        (1u16, size.div_ceil(32))
    } else {
        (0u16, size.div_ceil(2))
    };
    let mut bl_nb = (blsize << 15) | ((num_block - blsize) << 10);
    if bl_nb == 0 {
        bl_nb = 1 << 15;
    }
    btable_write(ep, BTABLE_FIELD_COUNT_RX, bl_nb);
}

fn ep_tx_count(ep: u8, value: u16) {
    btable_write(ep, BTABLE_FIELD_COUNT_TX, value);
}

fn ep_tx_addr(ep: u8) -> u16 {
    btable_read(ep, BTABLE_FIELD_ADDR_TX)
}

fn ep_rx_addr(ep: u8) -> u16 {
    btable_read(ep, BTABLE_FIELD_ADDR_RX)
}

fn ep_rx_count(ep: u8) -> usize {
    usize::from(btable_read(ep, BTABLE_FIELD_COUNT_RX) & 0x03FF)
}

#[inline(always)]
fn usb_reg_ptr(offset: usize) -> *mut u16 {
    (USB_BASE + offset) as *mut u16
}

fn usb_read16(offset: usize) -> u16 {
    unsafe { usb_reg_ptr(offset).read_volatile() }
}

fn usb_write16(offset: usize, value: u16) {
    unsafe { usb_reg_ptr(offset).write_volatile(value) }
}

#[inline(always)]
fn epr_read(ep: u8) -> u16 {
    usb_read16(usize::from(ep) * 4)
}

#[inline(always)]
fn epr_write(ep: u8, value: u16) {
    usb_write16(usize::from(ep) * 4, value)
}

#[inline(always)]
fn pma_word_ptr(offset: u16) -> *mut u16 {
    (USBRAM_BASE + usize::from(offset) * 2) as *mut u16
}

fn pma_read16(offset: u16) -> u16 {
    unsafe { pma_word_ptr(offset).read_volatile() }
}

fn pma_write16(offset: u16, value: u16) {
    unsafe { pma_word_ptr(offset).write_volatile(value) }
}

unsafe fn pma_write_bytes(offset: u16, bytes: &[u8]) {
    let mut pma_offset = offset;
    let mut index = 0;
    while index < bytes.len() {
        let lo = bytes[index];
        let hi = if index + 1 < bytes.len() { bytes[index + 1] } else { 0 };
        pma_write16(pma_offset, u16::from(lo) | (u16::from(hi) << 8));
        pma_offset += 2;
        index += 2;
    }
}

fn pma_read_bytes(offset: u16, dest: &mut [u8]) {
    let mut pma_offset = offset;
    let mut index = 0;
    while index < dest.len() {
        let word = pma_read16(pma_offset).to_le_bytes();
        dest[index] = word[0];
        if index + 1 < dest.len() {
            dest[index + 1] = word[1];
        }
        pma_offset += 2;
        index += 2;
    }
}
