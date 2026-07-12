#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOA_RESOURCES, GPIOA, Level},
    interrupt::{DRV_PFIC_RESOURCES, Irq, PFIC},
    time::{DRV_TIME_TIM4_RESOURCES, TIM4EmbassyTimeDriver},
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

unsafe extern "C" {
    fn __wch_hang_vector();
    fn __wch_tim4_vector();
}

#[derive(Clone, Copy)]
#[repr(C)]
union WchVector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

const WCH_VECTOR_COUNT: usize = 63;
const WCH_TIM4_VECTOR_SLOT: usize = 46;
const WCH_RESERVED_VECTOR: WchVector = WchVector { reserved: 0 };
const WCH_HANG_VECTOR: WchVector = WchVector {
    handler: __wch_hang_vector,
};
const WCH_TIM4_HANDLER_VECTOR: WchVector = WchVector {
    handler: __wch_tim4_vector,
};

#[repr(C, align(64))]
struct WchVectorTable([WchVector; WCH_VECTOR_COUNT]);

const fn build_wch_vector_table() -> WchVectorTable {
    let mut table = [WCH_HANG_VECTOR; WCH_VECTOR_COUNT];
    table[1] = WCH_RESERVED_VECTOR;
    table[4] = WCH_RESERVED_VECTOR;
    table[6] = WCH_RESERVED_VECTOR;
    table[7] = WCH_RESERVED_VECTOR;
    table[10] = WCH_RESERVED_VECTOR;
    table[11] = WCH_RESERVED_VECTOR;
    table[13] = WCH_RESERVED_VECTOR;
    table[15] = WCH_RESERVED_VECTOR;
    table[WCH_TIM4_VECTOR_SLOT] = WCH_TIM4_HANDLER_VECTOR;
    WchVectorTable(table)
}

#[unsafe(link_section = ".vector")]
#[used]
static WCH_VECTOR_TABLE: WchVectorTable = build_wch_vector_table();

global_asm!(
    r#"
    .global __wch_hang_vector
__wch_hang_vector:
1:
    j 1b

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

fn pfic() -> PFIC {
    PFIC::new(DRV_PFIC_RESOURCES).unwrap()
}

fn tim4_time() -> TIM4EmbassyTimeDriver {
    TIM4EmbassyTimeDriver::new(DRV_TIME_TIM4_RESOURCES).unwrap()
}

fn init_generated_time_driver() {
    tim4_time().init_time_driver().unwrap();
    unsafe {
        asm!("csrw 0x804, {}", in(reg) 0x3usize);
        asm!(
            "csrw mtvec, {}",
            in(reg) ((&WCH_VECTOR_TABLE as *const WchVectorTable as usize) | 0x3)
        );
    }
    pfic().enable_irq(Irq::TIM4).unwrap();
    unsafe {
        riscv::register::mie::set_mext();
        riscv::interrupt::enable();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
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
