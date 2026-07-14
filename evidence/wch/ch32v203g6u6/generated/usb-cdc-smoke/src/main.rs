#![no_std]
#![no_main]

use core::{
    arch::asm,
    cmp::min,
    panic::PanicInfo,
    ptr::{copy_nonoverlapping, null_mut},
};

use riscv::asm::delay;

const EP0_SIZE: usize = 64;
const EP0_SETUP_SIZE: usize = 8;

const RCC_CTLR: *mut u32 = 0x4002_1000 as *mut u32;
const RCC_CFGR0: *mut u32 = 0x4002_1004 as *mut u32;
const RCC_APB2PCENR: *mut u32 = 0x4002_1018 as *mut u32;
const RCC_APB1PCENR: *mut u32 = 0x4002_101c as *mut u32;
const RCC_APB1PRSTR: *mut u32 = 0x4002_100c as *mut u32;
const EXTEN_CTR: *mut u32 = 0x4002_3800 as *mut u32;
const GPIOA_CFGLR: *mut u32 = 0x4001_0800 as *mut u32;
const GPIOA_BSHR: usize = 0x4001_0810;
const GPIOA_BCR: usize = 0x4001_0814;

const RCC_PLLON: u32 = 1 << 24;
const RCC_PLLRDY: u32 = 1 << 25;
const RCC_HSEON: u32 = 1 << 16;
const RCC_HSERDY: u32 = 1 << 17;
const RCC_SW_MASK: u32 = 0b11;
const RCC_SW_PLL: u32 = 0b10;
const RCC_SWS_MASK: u32 = 0b11 << 2;
const RCC_SWS_PLL: u32 = 0b10 << 2;
const RCC_HPRE_MASK: u32 = 0b1111 << 4;
const RCC_PPRE1_MASK: u32 = 0b111 << 8;
const RCC_PPRE2_MASK: u32 = 0b111 << 11;
const RCC_PLLSRC: u32 = 1 << 16;
const RCC_PLLXTPRE: u32 = 1 << 17;
const RCC_PLLMULL_MASK: u32 = 0b1111 << 18;
const RCC_USBPRE_MASK: u32 = 0b11 << 22;
const RCC_PLLMULL6: u32 = 4 << 18;
const RCC_PPRE1_DIV2: u32 = 0b100 << 8;
const RCC_APB2_GPIOA_EN: u32 = 1 << 2;
const RCC_APB1_USB_EN: u32 = 1 << 23;
const RCC_APB1_USB_RST: u32 = 1 << 23;
const EXTEN_USBD_PU_EN: u32 = 1 << 1;
const EXTEN_PLL_HSI_PRE: u32 = 1 << 4;

const PA7_MASK: u32 = 1 << 7;
const GPIO_MODE_OUTPUT_50MHZ: u32 = 0b0011;

const USB_BASE: usize = 0x4000_5c00;
const USBRAM_BASE: usize = 0x4000_6000;

const USB_CNTR_OFFSET: usize = 0x40;
const USB_ISTR_OFFSET: usize = 0x44;
const USB_FNR_OFFSET: usize = 0x48;
const USB_DADDR_OFFSET: usize = 0x4c;
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

const USB_ISTR_CTR: u16 = 1 << 15;
const USB_ISTR_PMAOVR: u16 = 1 << 14;
const USB_ISTR_ERR: u16 = 1 << 13;
const USB_ISTR_WKUP: u16 = 1 << 12;
const USB_ISTR_SUSP: u16 = 1 << 11;
const USB_ISTR_RESET: u16 = 1 << 10;
const USB_ISTR_SOF: u16 = 1 << 9;
const USB_ISTR_ESOF: u16 = 1 << 8;
const USB_ISTR_EP_ID_MASK: u16 = 0x000f;

const USB_FNR_RXDM: u16 = 1 << 14;
const USB_FNR_RXDP: u16 = 1 << 15;

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
const EP_ADDR_MASK: u16 = 0x000f;

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

const PMA_EP0_RX_ADDR: u16 = 0x40;
const PMA_EP0_TX_ADDR: u16 = 0x80;

const USB_REQ_TYP_IN: u8 = 0x80;
const USB_REQ_TYP_MASK: u8 = 0x60;
const USB_REQ_TYP_STANDARD: u8 = 0x00;
const USB_REQ_TYP_CLASS: u8 = 0x20;
const USB_REQ_RECIP_MASK: u8 = 0x1f;
const USB_REQ_RECIP_DEVICE: u8 = 0x00;
const USB_REQ_RECIP_INTERFACE: u8 = 0x01;
const USB_REQ_RECIP_ENDPOINT: u8 = 0x02;
const USB_REQ_FEAT_REMOTE_WAKEUP: u16 = 0x0001;
const USB_REQ_FEAT_ENDP_HALT: u16 = 0x0000;

const USB_GET_STATUS: u8 = 0x00;
const USB_CLEAR_FEATURE: u8 = 0x01;
const USB_SET_FEATURE: u8 = 0x03;
const USB_SET_ADDRESS: u8 = 0x05;
const USB_GET_DESCRIPTOR: u8 = 0x06;
const USB_GET_CONFIGURATION: u8 = 0x08;
const USB_SET_CONFIGURATION: u8 = 0x09;
const USB_GET_INTERFACE: u8 = 0x0a;
const USB_SET_INTERFACE: u8 = 0x0b;

const USB_DESCR_TYP_DEVICE: u8 = 0x01;
const USB_DESCR_TYP_CONFIG: u8 = 0x02;
const USB_DESCR_TYP_STRING: u8 = 0x03;

const CDC_REQ_SET_LINE_CODING: u8 = 0x20;
const CDC_REQ_GET_LINE_CODING: u8 = 0x21;
const CDC_REQ_SET_CONTROL_LINE_STATE: u8 = 0x22;
const CDC_REQ_SEND_BREAK: u8 = 0x23;

const CDC_NOTIF_EP_ADDR: u8 = 0x81;
const CDC_DATA_OUT_EP_ADDR: u8 = 0x02;
const CDC_DATA_IN_EP_ADDR: u8 = 0x82;
const CDC_COMM_ITF_NUM: u16 = 0;

const PMA_CDC_NOTIF_TX_ADDR: u16 = 0xC0;
const PMA_CDC_DATA_RX_ADDR: u16 = 0x100;
const PMA_CDC_DATA_TX_ADDR: u16 = 0x140;

const CDC_NOTIF_PACKET_SIZE: u16 = 8;
const CDC_DATA_PACKET_SIZE: u16 = 64;

const HELLO_INTERVAL_TICKS: u32 = 200_000;
const HELLO_MESSAGE: &[u8] = b"Hello From ch32v203\r\n";

const DEVICE_DESC: [u8; 18] = [
    0x12,
    0x01,
    0x00,
    0x02,
    0xEF,
    0x02,
    0x01,
    EP0_SIZE as u8,
    0xfe,
    0xca,
    0x04,
    0x40,
    0x00,
    0x01,
    0x01,
    0x02,
    0x03,
    0x01,
];

const CONFIG_DESC: [u8; 75] = [
    0x09,
    0x02,
    0x4B,
    0x00,
    0x02,
    0x01,
    0x00,
    0x80,
    0x32,
    0x08,
    0x0B,
    0x00,
    0x02,
    0x02,
    0x02,
    0x00,
    0x00,
    0x09,
    0x04,
    0x00,
    0x00,
    0x01,
    0x02,
    0x02,
    0x00,
    0x00,
    0x05,
    0x24,
    0x00,
    0x20,
    0x01,
    0x05,
    0x24,
    0x01,
    0x00,
    0x01,
    0x04,
    0x24,
    0x02,
    0x06,
    0x05,
    0x24,
    0x06,
    0x00,
    0x01,
    0x07,
    0x05,
    CDC_NOTIF_EP_ADDR,
    0x03,
    0x08,
    0x00,
    0x01,
    0x09,
    0x04,
    0x01,
    0x00,
    0x02,
    0x0A,
    0x00,
    0x00,
    0x00,
    0x07,
    0x05,
    CDC_DATA_OUT_EP_ADDR,
    0x02,
    0x40,
    0x00,
    0x00,
    0x07,
    0x05,
    CDC_DATA_IN_EP_ADDR,
    0x02,
    0x40,
    0x00,
    0x00,
];

const LANG_DESC: [u8; 4] = [0x04, 0x03, 0x09, 0x04];
const MANUFACTURER_DESC: [u8; 16] = [
    0x10, 0x03, b'T', 0, b'i', 0, b'n', 0, b'y', 0, b'R', 0, b'S', 0, b'T', 0,
];
const PRODUCT_DESC: [u8; 30] = [
    0x1e, 0x03, b'F', 0, b'S', 0, b'D', 0, b'E', 0, b'V', 0, b' ', 0, b'S', 0, b'm', 0, b'o', 0,
    b'k', 0, b'e', 0, b' ', 0, b'R', 0, b'S', 0,
];
const SERIAL_DESC: [u8; 10] = [0x0a, 0x03, b'0', 0, b'0', 0, b'0', 0, b'1', 0];

const MARK_PULSE_CYCLES: u32 = 48_000;
const MARK_GAP_CYCLES: u32 = 48_000;
const MARK_GROUP_GAP_CYCLES: u32 = 160_000;
const RUNTIME_MARK_PULSE_CYCLES: u32 = 4_800;
const RUNTIME_MARK_GAP_CYCLES: u32 = 4_800;
const RUNTIME_MARK_GROUP_GAP_CYCLES: u32 = 16_000;
const USB_DETACH_DELAY_MS: u16 = 250;
const SYSCLK_48MHZ_1MS_CYCLES: u32 = 48_000;
const POLL_DELAY_CYCLES: u32 = 240;
const POST_RESET_SAMPLE_DELAY_TICKS: u16 = 48;

const MARK_BOOT: u8 = 1;
const MARK_USB_ENABLED: u8 = 2;
const MARK_IDLE_J: u8 = 3;
const MARK_BUS_RESET: u8 = 4;
const MARK_SE0: u8 = 5;
const MARK_K_STATE: u8 = 6;
const MARK_SE1: u8 = 7;
const MARK_CTR: u8 = 8;
const MARK_EP0_RX: u8 = 9;
const MARK_EP0_TX: u8 = 10;
const MARK_SETUP: u8 = 11;
const MARK_ADDRESSED: u8 = 12;
const MARK_CONFIGURED: u8 = 13;
const MARK_STALL: u8 = 14;
const MARK_SUSPEND: u8 = 15;
const MARK_WAKE: u8 = 16;
const MARK_ERROR: u8 = 17;
const MARK_PMAOVR: u8 = 18;
const MARK_ERR_ISTR_CTR: u8 = 19;
const MARK_ERR_ISTR_RESET: u8 = 20;
const MARK_ERR_FNR_RXDP: u8 = 21;
const MARK_ERR_FNR_RXDM: u8 = 22;
const MARK_ERR_EP0_CTR_RX: u8 = 23;
const MARK_ERR_EP0_CTR_TX: u8 = 24;
const MARK_ERR_EP0_SETUP: u8 = 25;
const MARK_ERR_EP0_RX_VALID: u8 = 26;
const MARK_ERR_EP0_TX_VALID: u8 = 27;
const MARK_HSE_FALLBACK: u8 = 28;

const EP0_OUT_ADDR: u8 = 0x00;
const EP0_IN_ADDR: u8 = 0x80;

static mut EP0_SETUP_BUF: [u8; 8] = [0; 8];
static mut CTRL_EP_BUF: [u8; EP0_SIZE] = [0; EP0_SIZE];
static mut CTRL_INLINE: [u8; EP0_SIZE] = [0; EP0_SIZE];
static mut CDC_RX_BUF: [u8; CDC_DATA_PACKET_SIZE as usize] = [0; CDC_DATA_PACKET_SIZE as usize];

#[inline(always)]
fn ctrl_ep_buf_ptr() -> *mut u8 {
    core::ptr::addr_of_mut!(CTRL_EP_BUF).cast::<u8>()
}

#[inline(always)]
fn ctrl_inline_ptr() -> *mut u8 {
    core::ptr::addr_of_mut!(CTRL_INLINE).cast::<u8>()
}

#[inline(always)]
fn ep0_setup_buf_ptr() -> *mut u8 {
    core::ptr::addr_of_mut!(EP0_SETUP_BUF).cast::<u8>()
}

#[inline(always)]
fn cdc_rx_buf_ptr() -> *mut u8 {
    core::ptr::addr_of_mut!(CDC_RX_BUF).cast::<u8>()
}

#[derive(Clone, Copy, Default)]
struct SetupRequest {
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
}

impl SetupRequest {
    fn direction_in(&self) -> bool {
        (self.bm_request_type & USB_REQ_TYP_IN) != 0
    }

    fn req_type(&self) -> u8 {
        self.bm_request_type & USB_REQ_TYP_MASK
    }

    fn recipient(&self) -> u8 {
        self.bm_request_type & USB_REQ_RECIP_MASK
    }
}

#[derive(Clone, Copy)]
struct XferCtl {
    buffer: *mut u8,
    total_len: u16,
    queued_len: u16,
    max_packet_size: u16,
    ep_idx: u8,
}

impl XferCtl {
    const fn new() -> Self {
        Self {
            buffer: null_mut(),
            total_len: 0,
            queued_len: 0,
            max_packet_size: 0,
            ep_idx: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct CtrlXfer {
    request: SetupRequest,
    buffer: *mut u8,
    data_len: u16,
    total_xferred: u16,
}

impl CtrlXfer {
    const fn new() -> Self {
        Self {
            request: SetupRequest {
                bm_request_type: 0,
                b_request: 0,
                w_value: 0,
                w_index: 0,
                w_length: 0,
            },
            buffer: null_mut(),
            data_len: 0,
            total_xferred: 0,
        }
    }
}

struct State {
    connected: bool,
    addressed: bool,
    suspended: bool,
    remote_wakeup_en: bool,
    cfg_num: u8,
    ep0_ctrl_dir_in: bool,
    ep0_ctrl_has_data: bool,
    ctrl_xfer: CtrlXfer,
    xfer_out: XferCtl,
    xfer_in: XferCtl,
    cdc_notif_in: XferCtl,
    cdc_data_out: XferCtl,
    cdc_data_in: XferCtl,
    cdc_line_coding: [u8; 7],
    cdc_line_state: u16,
    cdc_endpoints_open: bool,
    cdc_hello_ticks: u32,
    bus_reset_seen: bool,
    post_reset_sample_delay: u16,
    idle_j_seen: bool,
    se0_seen: bool,
    k_seen: bool,
    se1_seen: bool,
    ctr_seen: bool,
    ep0_rx_seen: bool,
    ep0_tx_seen: bool,
    suspend_seen: bool,
    wake_seen: bool,
    error_seen: bool,
    pmaovr_seen: bool,
    first_setup_seen: bool,
    address_seen: bool,
    configured_seen: bool,
}

impl State {
    const fn new() -> Self {
        Self {
            connected: false,
            addressed: false,
            suspended: false,
            remote_wakeup_en: false,
            cfg_num: 0,
            ep0_ctrl_dir_in: false,
            ep0_ctrl_has_data: false,
            ctrl_xfer: CtrlXfer::new(),
            xfer_out: XferCtl::new(),
            xfer_in: XferCtl::new(),
            cdc_notif_in: XferCtl::new(),
            cdc_data_out: XferCtl::new(),
            cdc_data_in: XferCtl::new(),
            cdc_line_coding: [0x00, 0xC2, 0x01, 0x00, 0x00, 0x00, 0x08],
            cdc_line_state: 0,
            cdc_endpoints_open: false,
            cdc_hello_ticks: 0,
            bus_reset_seen: false,
            post_reset_sample_delay: 0,
            idle_j_seen: false,
            se0_seen: false,
            k_seen: false,
            se1_seen: false,
            ctr_seen: false,
            ep0_rx_seen: false,
            ep0_tx_seen: false,
            suspend_seen: false,
            wake_seen: false,
            error_seen: false,
            pmaovr_seen: false,
            first_setup_seen: false,
            address_seen: false,
            configured_seen: false,
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    configure_debug_outputs();
    let using_hse = configure_sysclk_48mhz();

    let mut state = State::new();
    if !using_hse {
        pulse_marker(MARK_HSE_FALLBACK);
    }
    pulse_marker(MARK_BOOT);
    usb_init(&mut state);
    pulse_marker(MARK_USB_ENABLED);

    loop {
        dcd_int_handler(&mut state);
        cdc_poll(&mut state);
        update_debug_state(&mut state);
        delay(POLL_DELAY_CYCLES);
    }
}

fn configure_debug_outputs() {
    unsafe {
        RCC_APB2PCENR.write_volatile(RCC_APB2PCENR.read_volatile() | RCC_APB2_GPIOA_EN);
        let mut cfglr = GPIOA_CFGLR.read_volatile();
        cfglr &= !(0x0f << (7 * 4));
        cfglr |= GPIO_MODE_OUTPUT_50MHZ << (7 * 4);
        GPIOA_CFGLR.write_volatile(cfglr);
        write_gpio_low(PA7_MASK);
    }
}

#[inline(always)]
fn write_gpio_high(mask: u32) {
    unsafe {
        asm!(
            "sw {mask}, 0({addr})",
            addr = in(reg) GPIOA_BSHR,
            mask = in(reg) mask,
            options(nostack)
        );
    }
}

#[inline(always)]
fn write_gpio_low(mask: u32) {
    unsafe {
        asm!(
            "sw {mask}, 0({addr})",
            addr = in(reg) GPIOA_BCR,
            mask = in(reg) mask,
            options(nostack)
        );
    }
}

fn pulse_marker(count: u8) {
    pulse_marker_timed(
        count,
        MARK_PULSE_CYCLES,
        MARK_GAP_CYCLES,
        MARK_GROUP_GAP_CYCLES,
    );
}

fn pulse_runtime_marker(count: u8) {
    pulse_marker_timed(
        count,
        RUNTIME_MARK_PULSE_CYCLES,
        RUNTIME_MARK_GAP_CYCLES,
        RUNTIME_MARK_GROUP_GAP_CYCLES,
    );
}

fn pulse_marker_timed(count: u8, pulse_cycles: u32, gap_cycles: u32, group_gap_cycles: u32) {
    for _ in 0..count {
        write_gpio_high(PA7_MASK);
        delay(pulse_cycles);
        write_gpio_low(PA7_MASK);
        delay(gap_cycles);
    }
    delay(group_gap_cycles);
}

fn configure_sysclk_48mhz() -> bool {
    if configure_sysclk_48mhz_from_hse() {
        return true;
    }
    configure_sysclk_48mhz_from_hsi();
    false
}

fn configure_sysclk_48mhz_from_hse() -> bool {
    const HSE_TIMEOUT: u32 = 200_000;
    const PLL_TIMEOUT: u32 = 200_000;
    const SWITCH_TIMEOUT: u32 = 200_000;

    unsafe {
        EXTEN_CTR.write_volatile(EXTEN_CTR.read_volatile() & !EXTEN_PLL_HSI_PRE);

        let mut cfgr0 = RCC_CFGR0.read_volatile();
        cfgr0 &= !(RCC_SW_MASK
            | RCC_HPRE_MASK
            | RCC_PPRE1_MASK
            | RCC_PPRE2_MASK
            | RCC_PLLSRC
            | RCC_PLLXTPRE
            | RCC_PLLMULL_MASK
            | RCC_USBPRE_MASK);
        cfgr0 |= RCC_PPRE1_DIV2 | RCC_PLLMULL6;
        RCC_CFGR0.write_volatile(cfgr0);

        RCC_CTLR.write_volatile(RCC_CTLR.read_volatile() | RCC_HSEON);
        for _ in 0..HSE_TIMEOUT {
            if (RCC_CTLR.read_volatile() & RCC_HSERDY) != 0 {
                break;
            }
        }
        if (RCC_CTLR.read_volatile() & RCC_HSERDY) == 0 {
            return false;
        }

        let mut pll_cfg = RCC_CFGR0.read_volatile();
        pll_cfg &= !(RCC_PLLSRC | RCC_PLLXTPRE | RCC_PLLMULL_MASK | RCC_USBPRE_MASK);
        pll_cfg |= RCC_PLLSRC | RCC_PLLMULL6;
        RCC_CFGR0.write_volatile(pll_cfg);

        RCC_CTLR.write_volatile(RCC_CTLR.read_volatile() | RCC_PLLON);
        for _ in 0..PLL_TIMEOUT {
            if (RCC_CTLR.read_volatile() & RCC_PLLRDY) != 0 {
                break;
            }
        }
        if (RCC_CTLR.read_volatile() & RCC_PLLRDY) == 0 {
            return false;
        }

        let mut switched = RCC_CFGR0.read_volatile();
        switched &= !RCC_SW_MASK;
        switched |= RCC_SW_PLL;
        RCC_CFGR0.write_volatile(switched);
        for _ in 0..SWITCH_TIMEOUT {
            if (RCC_CFGR0.read_volatile() & RCC_SWS_MASK) == RCC_SWS_PLL {
                RCC_APB1PCENR.write_volatile(RCC_APB1PCENR.read_volatile() | RCC_APB1_USB_EN);
                return true;
            }
        }
    }
    false
}

fn configure_sysclk_48mhz_from_hsi() {
    unsafe {
        EXTEN_CTR.write_volatile(EXTEN_CTR.read_volatile() | EXTEN_PLL_HSI_PRE);

        let mut cfgr0 = RCC_CFGR0.read_volatile();
        cfgr0 &= !(RCC_SW_MASK
            | RCC_HPRE_MASK
            | RCC_PPRE1_MASK
            | RCC_PPRE2_MASK
            | RCC_PLLSRC
            | RCC_PLLXTPRE
            | RCC_PLLMULL_MASK
            | RCC_USBPRE_MASK);
        cfgr0 |= RCC_PPRE1_DIV2 | RCC_PLLMULL6;
        RCC_CFGR0.write_volatile(cfgr0);

        RCC_CTLR.write_volatile(RCC_CTLR.read_volatile() | RCC_PLLON);
        while (RCC_CTLR.read_volatile() & RCC_PLLRDY) == 0 {}

        let mut switched = RCC_CFGR0.read_volatile();
        switched &= !RCC_SW_MASK;
        switched |= RCC_SW_PLL;
        RCC_CFGR0.write_volatile(switched);
        while (RCC_CFGR0.read_volatile() & RCC_SWS_MASK) != RCC_SWS_PLL {}
        RCC_APB1PCENR.write_volatile(RCC_APB1PCENR.read_volatile() | RCC_APB1_USB_EN);
    }
}

fn usb_init(state: &mut State) {
    usb_disconnect();
    delay_ms(USB_DETACH_DELAY_MS);

    unsafe {
        RCC_APB1PRSTR.write_volatile(RCC_APB1PRSTR.read_volatile() | RCC_APB1_USB_RST);
        delay(1_000);
        RCC_APB1PRSTR.write_volatile(RCC_APB1PRSTR.read_volatile() & !RCC_APB1_USB_RST);
    }

    reset_device_state(state);
    dcd_init(state);
}

fn delay_ms(ms: u16) {
    for _ in 0..ms {
        delay(SYSCLK_48MHZ_1MS_CYCLES);
    }
}

fn reset_device_state(state: &mut State) {
    *state = State::new();
}

fn dcd_init(state: &mut State) {
    fsdev_core_reset();
    usb_write16(USB_CNTR_OFFSET, 0);
    usb_write16(USB_BTABLE_OFFSET, 0);
    usb_write16(USB_CNTR_OFFSET, USB_CNTR_INIT_MASK);
    handle_bus_reset(state);
    usb_connect();
}

fn fsdev_core_reset() {
    usb_write16(USB_CNTR_OFFSET, USB_CNTR_FRES | USB_CNTR_PDWN);
    delay(200);
    usb_write16(USB_CNTR_OFFSET, USB_CNTR_FRES);
    delay(200);
    usb_write16(USB_ISTR_OFFSET, 0);
}

fn handle_bus_reset(state: &mut State) {
    epr_write(1, 0);
    epr_write(2, 0);
    usb_write16(USB_DADDR_OFFSET, 0);
    state.ep0_ctrl_dir_in = false;
    state.ep0_ctrl_has_data = false;
    state.cdc_endpoints_open = false;
    state.cdc_line_state = 0;
    state.cdc_hello_ticks = 0;
    edpt0_open(state);
    usb_write16(USB_DADDR_OFFSET, USB_DADDR_EF);
}

fn edpt0_open(state: &mut State) {
    state.xfer_out.max_packet_size = EP0_SIZE as u16;
    state.xfer_out.ep_idx = 0;
    state.xfer_out.total_len = 0;
    state.xfer_out.queued_len = 0;
    state.xfer_out.buffer = null_mut();

    state.xfer_in.max_packet_size = EP0_SIZE as u16;
    state.xfer_in.ep_idx = 0;
    state.xfer_in.total_len = 0;
    state.xfer_in.queued_len = 0;
    state.xfer_in.buffer = null_mut();

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
    btable_set_rx_bufsize(0, EP0_SETUP_SIZE as u16);
}

fn dcd_int_handler(state: &mut State) {
    let int_status = usb_read16(USB_ISTR_OFFSET);

    if (int_status & USB_ISTR_SOF) != 0 && (usb_read16(USB_CNTR_OFFSET) & USB_CNTR_ESOFM) != 0 {
        clear_istr_exact(USB_ISTR_SOF);
    }

    if (int_status & USB_ISTR_RESET) != 0 {
        clear_istr_exact(USB_ISTR_RESET);
        reset_device_state(state);
        state.bus_reset_seen = true;
        state.post_reset_sample_delay = POST_RESET_SAMPLE_DELAY_TICKS;
        handle_bus_reset(state);
        pulse_runtime_marker(MARK_BUS_RESET);
        return;
    }

    if (int_status & USB_ISTR_WKUP) != 0 {
        usb_write16(
            USB_CNTR_OFFSET,
            usb_read16(USB_CNTR_OFFSET) & !(USB_CNTR_LPMODE | USB_CNTR_FSUSP),
        );
        clear_istr_exact(USB_ISTR_WKUP);
        state.suspended = false;
        if !state.wake_seen {
            state.wake_seen = true;
            pulse_runtime_marker(MARK_WAKE);
        }
    }

    if (int_status & USB_ISTR_SUSP) != 0 {
        let mut cntr = usb_read16(USB_CNTR_OFFSET);
        cntr |= USB_CNTR_FSUSP | USB_CNTR_LPMODE;
        usb_write16(USB_CNTR_OFFSET, cntr);
        clear_istr_exact(USB_ISTR_SUSP);
        state.suspended = true;
        if !state.suspend_seen {
            state.suspend_seen = true;
            pulse_runtime_marker(MARK_SUSPEND);
        }
    }

    if (int_status & USB_ISTR_ESOF) != 0 {
        clear_istr_exact(USB_ISTR_ESOF);
    }

    while (usb_read16(USB_ISTR_OFFSET) & USB_ISTR_CTR) != 0 {
        let ep_id = (usb_read16(USB_ISTR_OFFSET) & USB_ISTR_EP_ID_MASK) as u8;
        let ep_reg = epr_read(ep_id);

        if !state.ctr_seen {
            state.ctr_seen = true;
            pulse_runtime_marker(MARK_CTR);
        }

        if (ep_reg & EP_CTR_RX) != 0 {
            if !state.ep0_rx_seen {
                state.ep0_rx_seen = true;
                pulse_runtime_marker(MARK_EP0_RX);
            }
            if (ep_reg & EP_SETUP) != 0 {
                handle_ctr_setup(state, ep_id);
            } else {
                ep_write_clear_ctr(ep_id, false);
                handle_ctr_rx(state, ep_id);
            }
        }

        if (ep_reg & EP_CTR_TX) != 0 {
            if !state.ep0_tx_seen {
                state.ep0_tx_seen = true;
                pulse_runtime_marker(MARK_EP0_TX);
            }
            ep_write_clear_ctr(ep_id, true);
            handle_ctr_tx(state, ep_id);
        }
    }

    if (int_status & USB_ISTR_ERR) != 0 {
        if !state.error_seen {
            state.error_seen = true;
            pulse_runtime_marker(MARK_ERROR);
            emit_error_snapshot(state, int_status, epr_read(0), usb_read16(USB_FNR_OFFSET));
        }
        clear_istr_exact(USB_ISTR_ERR);
    }

    if (int_status & USB_ISTR_PMAOVR) != 0 {
        if !state.pmaovr_seen {
            state.pmaovr_seen = true;
            pulse_runtime_marker(MARK_PMAOVR);
        }
        clear_istr_exact(USB_ISTR_PMAOVR);
    }
}

fn handle_ctr_setup(state: &mut State, ep_id: u8) {
    let rx_count = ep_rx_count(ep_id);
    unsafe {
        pma_read_bytes(
            ep_rx_addr(ep_id),
            core::slice::from_raw_parts_mut(ep0_setup_buf_ptr(), rx_count),
        );
    }
    ep_write_clear_ctr(ep_id, false);

    if rx_count == 8 {
        let mut setup = [0u8; 8];
        unsafe {
            copy_nonoverlapping(ep0_setup_buf_ptr(), setup.as_mut_ptr(), 8);
        }
        let request = parse_setup(&setup);
        state.ep0_ctrl_dir_in = request.direction_in();
        state.ep0_ctrl_has_data = request.w_length != 0;
        if !state.ep0_ctrl_dir_in && state.ep0_ctrl_has_data {
            ep0_set_type(EP_TYPE_BULK);
        }
        if !state.first_setup_seen {
            state.first_setup_seen = true;
            pulse_runtime_marker(MARK_SETUP);
        }
        if !process_setup_received(state, request) {
            dcd_edpt_stall(EP0_OUT_ADDR);
            dcd_edpt_stall(EP0_IN_ADDR);
            pulse_runtime_marker(MARK_STALL);
        }
    } else {
        edpt0_prepare_setup();
    }
}

fn handle_ctr_rx(state: &mut State, ep_id: u8) {
    if ep_id != 0 {
        handle_data_out_ctr(state, ep_id);
        return;
    }

    let mut ep_reg = epr_read(ep_id) | EP_CTR_TX | EP_CTR_RX;
    if !state.ep0_ctrl_dir_in && state.ep0_ctrl_has_data {
        ep0_set_type(EP_TYPE_BULK);
        ep_reg = (ep_reg & !EP_TYPE_MASK) | EP_TYPE_BULK;
    }

    let rx_count = ep_rx_count(ep_id) as u16;
    if rx_count != 0 {
        let dst = state.xfer_out.buffer;
        if !dst.is_null() {
            unsafe {
                pma_read_bytes(
                    ep_rx_addr(ep_id),
                    core::slice::from_raw_parts_mut(
                        dst.add(state.xfer_out.queued_len as usize),
                        rx_count as usize,
                    ),
                );
            }
        }
    }
    state.xfer_out.queued_len = state.xfer_out.queued_len.saturating_add(rx_count);

    if rx_count < state.xfer_out.max_packet_size
        || state.xfer_out.queued_len >= state.xfer_out.total_len
    {
        btable_set_rx_bufsize(ep_id, state.xfer_out.max_packet_size);
        usbd_control_xfer_cb(state, EP0_OUT_ADDR, state.xfer_out.queued_len);
        state.xfer_out.total_len = 0;
        state.xfer_out.queued_len = 0;
    } else {
        let cnt = min(
            state
                .xfer_out
                .total_len
                .saturating_sub(state.xfer_out.queued_len),
            state.xfer_out.max_packet_size,
        );
        btable_set_rx_bufsize(ep_id, cnt);
        ep_reg &= u_epreg_mask() | EP_STAT_RX_MASK;
        ep_change_status(&mut ep_reg, false, EP_STAT_VALID);
        epr_write(ep_id, ep_reg);
    }
}

fn handle_ctr_tx(state: &mut State, ep_id: u8) {
    if ep_id != 0 {
        handle_data_in_ctr(state, ep_id);
        return;
    }

    if state.xfer_in.total_len != state.xfer_in.queued_len {
        dcd_transmit_packet(state, ep_id);
    } else {
        if state.ep0_ctrl_dir_in && state.ep0_ctrl_has_data {
            ep0_set_type(EP_TYPE_BULK);
        }
        usbd_control_xfer_cb(state, EP0_IN_ADDR, state.xfer_in.queued_len);
    }
}

fn dcd_transmit_packet(state: &mut State, ep_idx: u8) {
    let Some(xfer) = xfer_in_mut(state, ep_idx) else {
        return;
    };
    let len = min(
        xfer.total_len.saturating_sub(xfer.queued_len),
        xfer.max_packet_size,
    ) as usize;
    if len != 0 {
        unsafe {
            let src = xfer.buffer.add(xfer.queued_len as usize);
            pma_write_bytes(ep_tx_addr(ep_idx), core::slice::from_raw_parts(src, len));
        }
    }
    xfer.queued_len = xfer.queued_len.saturating_add(len as u16);
    ep_tx_count(ep_idx, len as u16);

    let mut ep_reg = epr_read(ep_idx) | EP_CTR_TX | EP_CTR_RX;
    ep_change_status(&mut ep_reg, true, EP_STAT_VALID);
    ep_reg &= u_epreg_mask() | EP_STAT_TX_MASK;
    epr_write(ep_idx, ep_reg);
}

fn dcd_edpt_xfer(state: &mut State, ep_addr: u8, buffer: *mut u8, total_bytes: u16) -> bool {
    let dir_in = (ep_addr & 0x80) != 0;
    let ep_num = ep_addr & 0x7f;
    let Some(xfer) = (if dir_in {
        xfer_in_mut(state, ep_num)
    } else {
        xfer_out_mut(state, ep_num)
    }) else {
        return false;
    };
    xfer.buffer = buffer;
    xfer.total_len = total_bytes;
    xfer.queued_len = 0;

    if dir_in {
        if ep_num == 0 {
            ep0_set_type(EP_TYPE_CONTROL);
        }
        dcd_transmit_packet(state, ep_num);
    } else {
        let mut ep_reg = epr_read(ep_num) | EP_CTR_TX | EP_CTR_RX;
        ep_reg &= u_epreg_mask() | EP_STAT_RX_MASK;
        let cnt = min(total_bytes, xfer.max_packet_size);
        btable_set_rx_bufsize(ep_num, cnt);
        if ep_num == 0 {
            ep_reg = (ep_reg & !EP_TYPE_MASK) | EP_TYPE_CONTROL;
        }
        ep_change_status(&mut ep_reg, false, EP_STAT_VALID);
        epr_write(ep_num, ep_reg);
    }

    true
}

fn dcd_edpt_stall(ep_addr: u8) {
    let ep_num = ep_addr & 0x7f;
    let dir_in = (ep_addr & 0x80) != 0;
    let mut ep_reg = epr_read(ep_num) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask()
        | if dir_in {
            EP_STAT_TX_MASK
        } else {
            EP_STAT_RX_MASK
        };
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
    let ep_num = ep_addr & 0x7f;
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
        ep_change_status(&mut ep_reg, false, EP_STAT_NAK);
    }
    epr_write(ep_num, ep_reg);
}

fn process_setup_received(state: &mut State, request: SetupRequest) -> bool {
    state.ctrl_xfer.request = request;
    state.ctrl_xfer.buffer = null_mut();
    state.ctrl_xfer.data_len = 0;
    state.ctrl_xfer.total_xferred = 0;
    state.connected = true;

    match request.recipient() {
        USB_REQ_RECIP_DEVICE => {
            if request.req_type() != USB_REQ_TYP_STANDARD {
                return false;
            }
            process_std_device_request(state, request)
        }
        USB_REQ_RECIP_INTERFACE => {
            if request.req_type() == USB_REQ_TYP_CLASS {
                process_cdc_class_request(state, request)
            } else {
                match request.b_request {
                    USB_GET_INTERFACE => {
                        unsafe {
                            CTRL_INLINE[0] = 0;
                        }
                        tud_control_xfer(state, request, ctrl_inline_ptr(), 1)
                    }
                    USB_SET_INTERFACE => tud_control_status(state, request),
                    _ => false,
                }
            }
        }
        USB_REQ_RECIP_ENDPOINT => process_std_endpoint_request(state, request),
        _ => false,
    }
}

fn process_std_device_request(state: &mut State, request: SetupRequest) -> bool {
    match request.b_request {
        USB_SET_ADDRESS => {
            dcd_set_address(state, request.w_value as u8);
            state.addressed = true;
            if !state.address_seen {
                state.address_seen = true;
                pulse_runtime_marker(MARK_ADDRESSED);
            }
            true
        }
        USB_GET_CONFIGURATION => {
            unsafe {
                CTRL_INLINE[0] = state.cfg_num;
            }
            tud_control_xfer(state, request, ctrl_inline_ptr(), 1)
        }
        USB_SET_CONFIGURATION => {
            if request.w_value > 1 {
                return false;
            }
            state.cfg_num = request.w_value as u8;
            if state.cfg_num == 1 {
                cdc_open_endpoints(state);
            } else {
                cdc_close_endpoints(state);
            }
            if state.cfg_num == 1 && !state.configured_seen {
                state.configured_seen = true;
                pulse_runtime_marker(MARK_CONFIGURED);
            }
            tud_control_status(state, request)
        }
        USB_GET_DESCRIPTOR => process_get_descriptor(state, request),
        USB_SET_FEATURE => {
            if request.w_value == USB_REQ_FEAT_REMOTE_WAKEUP {
                state.remote_wakeup_en = true;
                tud_control_status(state, request)
            } else {
                false
            }
        }
        USB_CLEAR_FEATURE => {
            if request.w_value == USB_REQ_FEAT_REMOTE_WAKEUP {
                state.remote_wakeup_en = false;
                tud_control_status(state, request)
            } else {
                false
            }
        }

        USB_GET_STATUS => {
            let mut status = 0u16;
            if state.remote_wakeup_en {
                status |= 0x0002;
            }
            unsafe {
                CTRL_INLINE[0] = status as u8;
                CTRL_INLINE[1] = (status >> 8) as u8;
            }
            tud_control_xfer(state, request, ctrl_inline_ptr(), 2)
        }
        _ => false,
    }
}

fn process_cdc_class_request(state: &mut State, request: SetupRequest) -> bool {
    if request.w_index != CDC_COMM_ITF_NUM {
        return false;
    }

    let line_coding_ptr = state.cdc_line_coding.as_mut_ptr();
    let line_coding_len = state.cdc_line_coding.len() as u16;

    match request.b_request {
        CDC_REQ_SET_LINE_CODING | CDC_REQ_GET_LINE_CODING => {
            tud_control_xfer(state, request, line_coding_ptr, line_coding_len)
        }
        CDC_REQ_SET_CONTROL_LINE_STATE => {
            state.cdc_line_state = request.w_value;
            tud_control_status(state, request)
        }
        CDC_REQ_SEND_BREAK => tud_control_status(state, request),
        _ => false,
    }
}

fn process_std_endpoint_request(state: &mut State, request: SetupRequest) -> bool {
    let ep_addr = request.w_index as u8;
    match request.b_request {
        USB_GET_STATUS => {
            let status = if usbd_edpt_stalled(ep_addr) {
                1u16
            } else {
                0u16
            };
            unsafe {
                CTRL_INLINE[0] = status as u8;
                CTRL_INLINE[1] = (status >> 8) as u8;
            }
            tud_control_xfer(state, request, ctrl_inline_ptr(), 2)
        }
        USB_CLEAR_FEATURE if request.w_value == USB_REQ_FEAT_ENDP_HALT => {
            dcd_edpt_clear_stall(ep_addr);
            tud_control_status(state, request)
        }
        USB_SET_FEATURE if request.w_value == USB_REQ_FEAT_ENDP_HALT => {
            dcd_edpt_stall(ep_addr);
            tud_control_status(state, request)
        }
        _ => false,
    }
}

fn process_get_descriptor(state: &mut State, request: SetupRequest) -> bool {
    let desc_type = (request.w_value >> 8) as u8;
    let desc_index = (request.w_value & 0xff) as u8;
    let descriptor: Option<&'static [u8]> = match (desc_type, desc_index) {
        (USB_DESCR_TYP_DEVICE, _) => Some(&DEVICE_DESC),
        (USB_DESCR_TYP_CONFIG, _) => Some(&CONFIG_DESC),
        (USB_DESCR_TYP_STRING, 0) => Some(&LANG_DESC),
        (USB_DESCR_TYP_STRING, 1) => Some(&MANUFACTURER_DESC),
        (USB_DESCR_TYP_STRING, 2) => Some(&PRODUCT_DESC),
        (USB_DESCR_TYP_STRING, 3) => Some(&SERIAL_DESC),
        _ => None,
    };
    let Some(descriptor) = descriptor else {
        return false;
    };
    tud_control_xfer(
        state,
        request,
        descriptor.as_ptr() as *mut u8,
        descriptor.len() as u16,
    )
}

fn tud_control_status(state: &mut State, request: SetupRequest) -> bool {
    let ep_status = status_stage_ep(request);
    status_stage_xact(state, ep_status)
}

fn tud_control_xfer(state: &mut State, request: SetupRequest, buffer: *mut u8, len: u16) -> bool {
    state.ctrl_xfer.request = request;
    state.ctrl_xfer.buffer = buffer;
    state.ctrl_xfer.data_len = min(len, request.w_length);
    state.ctrl_xfer.total_xferred = 0;

    if request.w_length != 0 {
        data_stage_xact(state)
    } else {
        status_stage_xact(state, EP0_IN_ADDR)
    }
}

fn status_stage_ep(request: SetupRequest) -> u8 {
    if request.w_length != 0 && request.direction_in() {
        EP0_OUT_ADDR
    } else {
        EP0_IN_ADDR
    }
}

fn status_stage_xact(state: &mut State, ep_addr: u8) -> bool {
    dcd_edpt_xfer(state, ep_addr, null_mut(), 0)
}

fn data_stage_xact(state: &mut State) -> bool {
    let remaining = state
        .ctrl_xfer
        .data_len
        .saturating_sub(state.ctrl_xfer.total_xferred);
    let xact_len = min(remaining, EP0_SIZE as u16);
    let ep_addr = if state.ctrl_xfer.request.direction_in() {
        EP0_IN_ADDR
    } else {
        EP0_OUT_ADDR
    };

    let buffer = if state.ctrl_xfer.request.direction_in() {
        if xact_len != 0 {
            unsafe {
                copy_nonoverlapping(
                    state
                        .ctrl_xfer
                        .buffer
                        .add(state.ctrl_xfer.total_xferred as usize),
                    ctrl_ep_buf_ptr(),
                    xact_len as usize,
                );
            }
            ctrl_ep_buf_ptr()
        } else {
            null_mut()
        }
    } else {
        ctrl_ep_buf_ptr()
    };

    dcd_edpt_xfer(state, ep_addr, buffer, xact_len)
}

fn usbd_control_xfer_cb(state: &mut State, ep_addr: u8, xferred_bytes: u16) {
    let ep_status = status_stage_ep(state.ctrl_xfer.request);
    if ep_addr == ep_status {
        dcd_edpt0_status_complete(state.ctrl_xfer.request);
        state.ep0_ctrl_has_data = false;
        return;
    }

    if !state.ctrl_xfer.request.direction_in()
        && xferred_bytes != 0
        && !state.ctrl_xfer.buffer.is_null()
    {
        unsafe {
            copy_nonoverlapping(
                ctrl_ep_buf_ptr().cast_const(),
                state
                    .ctrl_xfer
                    .buffer
                    .add(state.ctrl_xfer.total_xferred as usize),
                xferred_bytes as usize,
            );
        }
    }

    state.ctrl_xfer.total_xferred = state.ctrl_xfer.total_xferred.saturating_add(xferred_bytes);
    if !state.ctrl_xfer.request.direction_in() && !state.ctrl_xfer.buffer.is_null() {
        unsafe {
            state.ctrl_xfer.buffer = state.ctrl_xfer.buffer.add(xferred_bytes as usize);
        }
    }

    if state.ctrl_xfer.request.w_length == state.ctrl_xfer.total_xferred
        || xferred_bytes < EP0_SIZE as u16
    {
        let _ = status_stage_xact(state, ep_status);
    } else {
        let _ = data_stage_xact(state);
    }
}

fn dcd_set_address(state: &mut State, _dev_addr: u8) {
    let _ = dcd_edpt_xfer(state, EP0_IN_ADDR, null_mut(), 0);
}

fn dcd_edpt0_status_complete(request: SetupRequest) {
    edpt0_prepare_setup();
    if request.recipient() == USB_REQ_RECIP_DEVICE
        && request.req_type() == USB_REQ_TYP_STANDARD
        && request.b_request == USB_SET_ADDRESS
    {
        usb_write16(
            USB_DADDR_OFFSET,
            USB_DADDR_EF | (request.w_value as u16 & 0x7f),
        );
    }
}

fn usbd_edpt_stalled(ep_addr: u8) -> bool {
    let ep_num = ep_addr & 0x7f;
    if (ep_addr & 0x80) != 0 {
        ((epr_read(ep_num) & EP_STAT_TX_MASK) >> 4) as u8 == EP_STAT_STALL
    } else {
        ((epr_read(ep_num) & EP_STAT_RX_MASK) >> 12) as u8 == EP_STAT_STALL
    }
}

fn cdc_open_endpoints(state: &mut State) {
    open_in_endpoint(
        &mut state.cdc_notif_in,
        CDC_NOTIF_EP_ADDR & 0x7f,
        EP_TYPE_INTERRUPT,
        PMA_CDC_NOTIF_TX_ADDR,
        CDC_NOTIF_PACKET_SIZE,
    );
    open_out_endpoint(
        &mut state.cdc_data_out,
        CDC_DATA_OUT_EP_ADDR & 0x7f,
        EP_TYPE_BULK,
        PMA_CDC_DATA_RX_ADDR,
        CDC_DATA_PACKET_SIZE,
    );
    open_in_endpoint(
        &mut state.cdc_data_in,
        CDC_DATA_IN_EP_ADDR & 0x7f,
        EP_TYPE_BULK,
        PMA_CDC_DATA_TX_ADDR,
        CDC_DATA_PACKET_SIZE,
    );
    state.cdc_endpoints_open = true;
    state.cdc_hello_ticks = 0;
    arm_cdc_out_receive(state);
}

fn cdc_close_endpoints(state: &mut State) {
    epr_write(CDC_NOTIF_EP_ADDR & 0x7f, 0);
    epr_write(CDC_DATA_OUT_EP_ADDR & 0x7f, 0);
    state.cdc_endpoints_open = false;
    state.cdc_line_state = 0;
    state.cdc_hello_ticks = 0;
    state.cdc_notif_in = XferCtl::new();
    state.cdc_data_out = XferCtl::new();
    state.cdc_data_in = XferCtl::new();
}

fn arm_cdc_out_receive(state: &mut State) {
    let _ = dcd_edpt_xfer(
        state,
        CDC_DATA_OUT_EP_ADDR,
        cdc_rx_buf_ptr(),
        CDC_DATA_PACKET_SIZE,
    );
}

fn cdc_poll(state: &mut State) {
    if !state.cdc_endpoints_open || state.cfg_num != 1 || (state.cdc_line_state & 0x0001) == 0 {
        state.cdc_hello_ticks = 0;
        return;
    }

    state.cdc_hello_ticks = state.cdc_hello_ticks.saturating_add(1);
    if state.cdc_hello_ticks < HELLO_INTERVAL_TICKS {
        return;
    }
    state.cdc_hello_ticks -= HELLO_INTERVAL_TICKS;

    if state.cdc_data_in.total_len != 0 {
        return;
    }

    let _ = dcd_edpt_xfer(
        state,
        CDC_DATA_IN_EP_ADDR,
        HELLO_MESSAGE.as_ptr() as *mut u8,
        HELLO_MESSAGE.len() as u16,
    );
}

fn handle_data_out_ctr(state: &mut State, ep_id: u8) {
    let Some(xfer) = xfer_out_mut(state, ep_id) else {
        return;
    };

    let mut ep_reg = epr_read(ep_id) | EP_CTR_TX | EP_CTR_RX;
    let rx_count = ep_rx_count(ep_id) as u16;
    if rx_count != 0 && !xfer.buffer.is_null() {
        unsafe {
            pma_read_bytes(
                ep_rx_addr(ep_id),
                core::slice::from_raw_parts_mut(
                    xfer.buffer.add(xfer.queued_len as usize),
                    rx_count as usize,
                ),
            );
        }
    }
    xfer.queued_len = xfer.queued_len.saturating_add(rx_count);

    if rx_count < xfer.max_packet_size || xfer.queued_len >= xfer.total_len {
        btable_set_rx_bufsize(ep_id, xfer.max_packet_size);
        xfer.total_len = 0;
        xfer.queued_len = 0;

        if ep_id == (CDC_DATA_OUT_EP_ADDR & 0x7f) {
            arm_cdc_out_receive(state);
        }
    } else {
        let cnt = min(
            xfer.total_len.saturating_sub(xfer.queued_len),
            xfer.max_packet_size,
        );
        btable_set_rx_bufsize(ep_id, cnt);
        ep_reg &= u_epreg_mask() | EP_STAT_RX_MASK;
        ep_change_status(&mut ep_reg, false, EP_STAT_VALID);
        epr_write(ep_id, ep_reg);
    }
}

fn handle_data_in_ctr(state: &mut State, ep_id: u8) {
    let Some(xfer) = xfer_in_mut(state, ep_id) else {
        return;
    };

    if xfer.total_len != xfer.queued_len {
        dcd_transmit_packet(state, ep_id);
    } else {
        xfer.total_len = 0;
        xfer.queued_len = 0;
    }
}

fn open_in_endpoint(xfer: &mut XferCtl, ep_num: u8, ep_type: u16, pma_addr: u16, packet_size: u16) {
    xfer.max_packet_size = packet_size;
    xfer.ep_idx = ep_num;
    xfer.total_len = 0;
    xfer.queued_len = 0;
    xfer.buffer = null_mut();

    btable_write(ep_num, BTABLE_FIELD_ADDR_TX, pma_addr);

    let mut ep_reg = epr_read(ep_num) & !u_epreg_mask();
    ep_reg |= u16::from(ep_num) | ep_type;
    ep_change_status(&mut ep_reg, true, EP_STAT_NAK);
    ep_reg &= !(EP_STAT_RX_MASK | EP_DTOG_RX);
    epr_write(ep_num, ep_reg);
}

fn open_out_endpoint(
    xfer: &mut XferCtl,
    ep_num: u8,
    ep_type: u16,
    pma_addr: u16,
    packet_size: u16,
) {
    xfer.max_packet_size = packet_size;
    xfer.ep_idx = ep_num;
    xfer.total_len = 0;
    xfer.queued_len = 0;
    xfer.buffer = null_mut();

    btable_write(ep_num, BTABLE_FIELD_ADDR_RX, pma_addr);

    let mut ep_reg = epr_read(ep_num) & !u_epreg_mask();
    ep_reg |= u16::from(ep_num) | ep_type;
    ep_change_status(&mut ep_reg, false, EP_STAT_NAK);
    ep_reg &= !(EP_STAT_TX_MASK | EP_DTOG_TX);
    epr_write(ep_num, ep_reg);
}

fn xfer_in_mut(state: &mut State, ep_num: u8) -> Option<&mut XferCtl> {
    match ep_num {
        0 => Some(&mut state.xfer_in),
        1 => Some(&mut state.cdc_notif_in),
        2 => Some(&mut state.cdc_data_in),
        _ => None,
    }
}

fn xfer_out_mut(state: &mut State, ep_num: u8) -> Option<&mut XferCtl> {
    match ep_num {
        0 => Some(&mut state.xfer_out),
        2 => Some(&mut state.cdc_data_out),
        _ => None,
    }
}

fn parse_setup(bytes: &[u8; 8]) -> SetupRequest {
    SetupRequest {
        bm_request_type: bytes[0],
        b_request: bytes[1],
        w_value: u16::from_le_bytes([bytes[2], bytes[3]]),
        w_index: u16::from_le_bytes([bytes[4], bytes[5]]),
        w_length: u16::from_le_bytes([bytes[6], bytes[7]]),
    }
}

fn update_debug_state(state: &mut State) {
    let fnr = usb_read16(USB_FNR_OFFSET);
    if (fnr & USB_FNR_RXDP) != 0 && (fnr & USB_FNR_RXDM) == 0 && !state.idle_j_seen {
        state.idle_j_seen = true;
        pulse_runtime_marker(MARK_IDLE_J);
    }
    if !state.bus_reset_seen {
        return;
    }
    if state.post_reset_sample_delay != 0 {
        state.post_reset_sample_delay -= 1;
        return;
    }

    let dp = (fnr & USB_FNR_RXDP) != 0;
    let dm = (fnr & USB_FNR_RXDM) != 0;
    match (dp, dm) {
        (false, false) if !state.se0_seen => {
            state.se0_seen = true;
            pulse_runtime_marker(MARK_SE0);
        }
        (false, true) if !state.k_seen => {
            state.k_seen = true;
            pulse_runtime_marker(MARK_K_STATE);
        }
        (true, true) if !state.se1_seen => {
            state.se1_seen = true;
            pulse_runtime_marker(MARK_SE1);
        }
        _ => {}
    }
}

fn emit_error_snapshot(state: &State, istr: u16, ep0: u16, fnr: u16) {
    let _ = state;
    if (istr & USB_ISTR_CTR) != 0 {
        pulse_runtime_marker(MARK_ERR_ISTR_CTR);
    }
    if (istr & USB_ISTR_RESET) != 0 {
        pulse_runtime_marker(MARK_ERR_ISTR_RESET);
    }
    if (fnr & USB_FNR_RXDP) != 0 {
        pulse_runtime_marker(MARK_ERR_FNR_RXDP);
    }
    if (fnr & USB_FNR_RXDM) != 0 {
        pulse_runtime_marker(MARK_ERR_FNR_RXDM);
    }
    if (ep0 & EP_CTR_RX) != 0 {
        pulse_runtime_marker(MARK_ERR_EP0_CTR_RX);
    }
    if (ep0 & EP_CTR_TX) != 0 {
        pulse_runtime_marker(MARK_ERR_EP0_CTR_TX);
    }
    if (ep0 & EP_SETUP) != 0 {
        pulse_runtime_marker(MARK_ERR_EP0_SETUP);
    }
    if ((ep0 & EP_STAT_RX_MASK) >> 12) as u8 == EP_STAT_VALID {
        pulse_runtime_marker(MARK_ERR_EP0_RX_VALID);
    }
    if ((ep0 & EP_STAT_TX_MASK) >> 4) as u8 == EP_STAT_VALID {
        pulse_runtime_marker(MARK_ERR_EP0_TX_VALID);
    }
}

fn usb_connect() {
    unsafe {
        EXTEN_CTR.write_volatile(EXTEN_CTR.read_volatile() | EXTEN_USBD_PU_EN);
    }
}

fn usb_disconnect() {
    unsafe {
        EXTEN_CTR.write_volatile(EXTEN_CTR.read_volatile() & !EXTEN_USBD_PU_EN);
    }
}

#[inline(always)]
fn usb_read16(offset: usize) -> u16 {
    unsafe { ((USB_BASE + offset) as *const u16).read_volatile() }
}

#[inline(always)]
fn usb_write16(offset: usize, value: u16) {
    unsafe { ((USB_BASE + offset) as *mut u16).write_volatile(value) }
}

#[inline(always)]
fn epr_ptr(ep: u8) -> *mut u16 {
    (USB_BASE + usize::from(ep) * 4) as *mut u16
}

#[inline(always)]
fn epr_read(ep: u8) -> u16 {
    unsafe { epr_ptr(ep).read_volatile() }
}

#[inline(always)]
fn epr_write(ep: u8, value: u16) {
    unsafe { epr_ptr(ep).write_volatile(value) }
}

#[inline(always)]
const fn u_epreg_mask() -> u16 {
    EP_CTR_RX | EP_SETUP | EP_TYPE_MASK | EP_KIND | EP_CTR_TX | EP_ADDR_MASK
}

fn ep_change_status(reg: &mut u16, dir_in: bool, state: u8) {
    *reg ^= u16::from(state) << if dir_in { 4 } else { 12 };
}

fn ep0_set_type(ep_type: u16) {
    let mut ep_reg = epr_read(0) | EP_CTR_TX | EP_CTR_RX;
    ep_reg &= u_epreg_mask();
    ep_reg = (ep_reg & !EP_TYPE_MASK) | ep_type;
    epr_write(0, ep_reg);
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
    usize::from(btable_read(ep, BTABLE_FIELD_COUNT_RX) & 0x03ff)
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
        let hi = if index + 1 < bytes.len() {
            bytes[index + 1]
        } else {
            0
        };
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
