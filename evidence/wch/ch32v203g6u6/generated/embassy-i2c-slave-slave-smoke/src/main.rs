#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use core::panic::PanicInfo;
use core::{
    ptr::addr_of_mut,
    sync::atomic::{AtomicBool, Ordering},
};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOB_RUNTIME_RESOURCES, GPIOB},
    i2c::{DRV_I2C1_SLAVE_RUNTIME_RESOURCES, I2C1Slave, I2C1SlavePacketDirection},
    rcc::{DRV_RCC_RUNTIME_RESOURCES, RCC},
    wch,
};

const SLAVE_ADDRESS: u8 = 0x42;
const NORMAL_REQUEST: [u8; 3] = [0x10, 0x20, 0x30];
const NORMAL_RESPONSE: [u8; 4] = [0x30, 0x20, 0x10, 0x03];
const ISR_REQUEST: [u8; 3] = [0xA5, 0x5A, 0x01];
const STATUS_QUERY: [u8; 1] = [0x55];

const GPIOB_CFGLR: u32 = 0x40010C00;
const GPIOB_BSHR: u32 = 0x40010C10;
const PB6_MODE_SHIFT: u32 = 24;
const PB7_MODE_SHIFT: u32 = 28;
const GPIO_ALT_OPEN_DRAIN_50MHZ: u32 = 0xF;

static ISR_PACKET_SEEN: AtomicBool = AtomicBool::new(false);
static mut ISR_BUFFER: [u8; 32] = [0; 32];

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    let _rcc = RCC::new(DRV_RCC_RUNTIME_RESOURCES).unwrap();
    wch::init_embassy_time_runtime().unwrap();

    let gpiob = GPIOB::new(DRV_GPIOB_RUNTIME_RESOURCES).unwrap();
    gpiob.enable_clock().unwrap();
    gpiob.release_reset().unwrap();
    configure_board_i2c_pins();

    let i2c1 = I2C1Slave::new(DRV_I2C1_SLAVE_RUNTIME_RESOURCES).unwrap();
    i2c1.enable_clock().unwrap();
    i2c1.release_reset().unwrap();
    i2c1.init_slave().unwrap();
    i2c1.set_own_address_7bit(SLAVE_ADDRESS).unwrap();
    i2c1.enable_rx_packet_isr_dispatch(unsafe { &mut *addr_of_mut!(ISR_BUFFER) }, on_isr_packet)
        .unwrap();

    let mut rx = [0u8; 32];
    let mut tx = [0u8; 32];
    let mut tx_len = 0usize;

    loop {
        match i2c1.blocking_wait_packet_direction().unwrap() {
            I2C1SlavePacketDirection::ReceiveFromMaster => {
                let received = i2c1.blocking_read_packet(&mut rx).unwrap();
                if received == NORMAL_REQUEST.len() && rx[..received] == NORMAL_REQUEST {
                    tx[..NORMAL_RESPONSE.len()].copy_from_slice(&NORMAL_RESPONSE);
                    tx_len = NORMAL_RESPONSE.len();
                } else if received == STATUS_QUERY.len() && rx[..received] == STATUS_QUERY {
                    let callback_seen = ISR_PACKET_SEEN.load(Ordering::Acquire);
                    ISR_PACKET_SEEN.store(false, Ordering::Release);
                    tx[0] = u8::from(callback_seen);
                    tx_len = 1;
                } else {
                    tx[0] = 0xEE;
                    tx_len = 1;
                }
            }
            I2C1SlavePacketDirection::TransmitToMaster => {
                let _ = i2c1.blocking_write_packet(&tx[..tx_len]).unwrap();
                tx_len = 0;
            }
        }
    }
}

fn on_isr_packet(bytes: &[u8], truncated: bool) {
    if !truncated && bytes == ISR_REQUEST {
        ISR_PACKET_SEEN.store(true, Ordering::Release);
    }
}

fn configure_board_i2c_pins() {
    write_u32(GPIOB_BSHR, (1u32 << 6) | (1u32 << 7));
    modify_u32(
        GPIOB_CFGLR,
        (0xFu32 << PB6_MODE_SHIFT) | (0xFu32 << PB7_MODE_SHIFT),
        (GPIO_ALT_OPEN_DRAIN_50MHZ << PB6_MODE_SHIFT)
            | (GPIO_ALT_OPEN_DRAIN_50MHZ << PB7_MODE_SHIFT),
    );
}

fn write_u32(address: u32, value: u32) {
    unsafe {
        (address as *mut u32).write_volatile(value);
    }
}

fn modify_u32(address: u32, clear_mask: u32, set_mask: u32) {
    unsafe {
        let ptr = address as *mut u32;
        let current = ptr.read_volatile();
        ptr.write_volatile((current & !clear_mask) | set_mask);
    }
}
