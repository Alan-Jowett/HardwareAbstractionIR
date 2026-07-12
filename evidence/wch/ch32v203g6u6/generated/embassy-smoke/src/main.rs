#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOA_RESOURCES, GPIOA, Level},
    interrupt::{DRV_PFIC_RESOURCES, Irq, PFIC},
    time::{DRV_TIME_TIM4_RESOURCES, TIM4EmbassyTimeDriver},
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

const GPIOA_CFGLR_ADDR: usize = 0x4001_0800;
const GPIOA_OUTDR_ADDR: usize = 0x4001_080C;
const RCC_APB2PCENR_ADDR: usize = 0x4002_1018;
const GPIOA_CLOCK_EN_BIT: u32 = 0x0000_0004;
const GPIO_CFG_MASK: u32 = 0xF;
const GPIO_MODE_OUTPUT_50MHZ_PUSH_PULL: u32 = 0x3;
const PA7_CFG_SHIFT: u32 = 28;
const PA7_BIT: u32 = 1 << 7;
const FAULT_PULSE_ON_CYCLES: u32 = 200_000;
const FAULT_PULSE_OFF_CYCLES: u32 = 200_000;
const FAULT_GROUP_GAP_CYCLES: u32 = 8_000_000;

unsafe extern "C" {
    fn __wch_fault_vector();
    fn __wch_tim4_vector();
    fn __wch_unexpected_irq_vector();
}

#[repr(C)]
union WchVector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[repr(C, align(64))]
struct WchVectorTable([WchVector; 63]);

#[unsafe(link_section = ".vector")]
#[used]
static WCH_VECTOR_TABLE: WchVectorTable = WchVectorTable([
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector { reserved: 0 },
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector { reserved: 0 },
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector { reserved: 0 },
    WchVector { reserved: 0 },
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector { reserved: 0 },
    WchVector { reserved: 0 },
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector { reserved: 0 },
    WchVector {
        handler: __wch_fault_vector,
    },
    WchVector { reserved: 0 },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_tim4_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
    WchVector {
        handler: __wch_unexpected_irq_vector,
    },
]);

global_asm!(
    r#"
    .global __wch_fault_vector
__wch_fault_vector:
    li a0, 13
    tail __wch_signal_fault_rust

    .global __wch_unexpected_irq_vector
__wch_unexpected_irq_vector:
    li a0, 14
    tail __wch_signal_fault_rust

    .global __wch_tim4_vector
__wch_tim4_vector:
    addi sp, sp, -64
    sw ra, 0(sp)
    sw t0, 4(sp)
    sw t1, 8(sp)
    sw t2, 12(sp)
    sw t3, 16(sp)
    sw t4, 20(sp)
    sw t5, 24(sp)
    sw t6, 28(sp)
    sw a0, 32(sp)
    sw a1, 36(sp)
    sw a2, 40(sp)
    sw a3, 44(sp)
    sw a4, 48(sp)
    sw a5, 52(sp)
    sw a6, 56(sp)
    sw a7, 60(sp)
    call __wch_tim4_irq_rust
    lw ra, 0(sp)
    lw t0, 4(sp)
    lw t1, 8(sp)
    lw t2, 12(sp)
    lw t3, 16(sp)
    lw t4, 20(sp)
    lw t5, 24(sp)
    lw t6, 28(sp)
    lw a0, 32(sp)
    lw a1, 36(sp)
    lw a2, 40(sp)
    lw a3, 44(sp)
    lw a4, 48(sp)
    lw a5, 52(sp)
    lw a6, 56(sp)
    lw a7, 60(sp)
    addi sp, sp, 64
    mret
"#
);

fn modify_u32(address: usize, clear_mask: u32, set_mask: u32) {
    unsafe {
        let current = read_volatile(address as *const u32);
        write_volatile(address as *mut u32, (current & !clear_mask) | set_mask);
    }
}

fn set_status_pin_raw(high: bool) {
    unsafe {
        let current = read_volatile(GPIOA_OUTDR_ADDR as *const u32);
        let next = if high {
            current | PA7_BIT
        } else {
            current & !PA7_BIT
        };
        write_volatile(GPIOA_OUTDR_ADDR as *mut u32, next);
    }
}

fn init_status_pin_raw() {
    modify_u32(RCC_APB2PCENR_ADDR, GPIOA_CLOCK_EN_BIT, GPIOA_CLOCK_EN_BIT);
    modify_u32(
        GPIOA_CFGLR_ADDR,
        GPIO_CFG_MASK << PA7_CFG_SHIFT,
        GPIO_MODE_OUTPUT_50MHZ_PUSH_PULL << PA7_CFG_SHIFT,
    );
    set_status_pin_raw(false);
}

fn busy_wait(cycles: u32) {
    for _ in 0..cycles {
        core::hint::spin_loop();
    }
}

fn signal_fault(code: u8) -> ! {
    init_status_pin_raw();
    let pulses = code.max(1);
    loop {
        for _ in 0..pulses {
            set_status_pin_raw(true);
            busy_wait(FAULT_PULSE_ON_CYCLES);
            set_status_pin_raw(false);
            busy_wait(FAULT_PULSE_OFF_CYCLES);
        }
        busy_wait(FAULT_GROUP_GAP_CYCLES);
    }
}

fn pfic() -> PFIC {
    PFIC::new(DRV_PFIC_RESOURCES).unwrap()
}

fn tim4_time() -> TIM4EmbassyTimeDriver {
    TIM4EmbassyTimeDriver::new(DRV_TIME_TIM4_RESOURCES).unwrap()
}

fn install_wch_vectors() {
    unsafe {
        asm!("csrw 0x804, {}", in(reg) 0x3usize);
        asm!(
            "csrw mtvec, {}",
            in(reg) ((&WCH_VECTOR_TABLE as *const WchVectorTable as usize) | 0x3)
        );
    }
}

fn enable_tim4_irq() {
    pfic().enable_irq(Irq::TIM4).unwrap();
}

fn init_generated_time_driver() {
    tim4_time().init_time_driver().unwrap();
    install_wch_vectors();
    enable_tim4_irq();
    unsafe {
        riscv::register::mie::set_mext();
        riscv::interrupt::enable();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    signal_fault(15)
}

#[unsafe(no_mangle)]
extern "C" fn __wch_signal_fault_rust(code: usize) -> ! {
    signal_fault(code as u8)
}

#[unsafe(no_mangle)]
extern "C" fn __wch_tim4_irq_rust() {
    tim4_time().on_time_driver_interrupt();
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let gpioa = GPIOA::new(DRV_GPIOA_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();
    let led = gpioa.pa7().into_output(Level::Low).unwrap();
    init_generated_time_driver();
    loop {
        led.set_high().unwrap();
        Timer::after(Duration::from_secs(1)).await;
        led.set_low().unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
