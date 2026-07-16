//! Generated WCH/QingKe runtime support for CH32V203G6U6.

#[cfg(feature = "dma-async")]
use crate::dma::{DMA1, DMA1RuntimeResources};
#[cfg(feature = "gpio-async-wait")]
use crate::gpio::generated_drv_gpioa_signal_gpio_wait;
#[cfg(feature = "gpio-async-wait")]
use crate::gpio::generated_drv_gpiob_signal_gpio_wait;
#[cfg(feature = "gpio-async-wait")]
use crate::gpio::generated_drv_gpiod_signal_gpio_wait;
#[cfg(feature = "i2c-async")]
use crate::i2c::generated_drv_i2c1_signal_i2c_async;
#[cfg(feature = "i2c-async")]
use crate::i2c::generated_drv_i2c1_slave_on_i2c_slave_interrupt;
use crate::interrupt::{DRV_PFIC_RUNTIME_RESOURCES, Irq, PFIC};
use crate::metadata;
use crate::time::{DRV_TIME_RTC_RUNTIME_RESOURCES, RTCEmbassyTimeDriver};
use core::arch::{asm, global_asm};

pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {
    module_name: "wch",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

unsafe extern "C" {
    fn __hair_wch_hang_vector();
    #[cfg(feature = "gpio-async-wait")]
    fn __hair_wch_drv_gpioa_exti_exti0_vector();
    #[cfg(feature = "gpio-async-wait")]
    fn __hair_wch_drv_gpioa_exti_exti1_vector();
    #[cfg(feature = "gpio-async-wait")]
    fn __hair_wch_drv_gpioa_exti_exti2_vector();
    #[cfg(feature = "gpio-async-wait")]
    fn __hair_wch_drv_gpioa_exti_exti3_vector();
    #[cfg(feature = "gpio-async-wait")]
    fn __hair_wch_drv_gpioa_exti_exti4_vector();
    #[cfg(feature = "dma-async")]
    fn __hair_wch_drv_dma1_ch1_vector();
    #[cfg(feature = "gpio-async-wait")]
    fn __hair_wch_drv_gpioa_exti_exti9_5_vector();
    #[cfg(feature = "i2c-async")]
    fn __hair_wch_drv_i2c1_i2c1_ev_vector();
    #[cfg(feature = "i2c-async")]
    fn __hair_wch_drv_i2c1_i2c1_er_vector();
    #[cfg(feature = "gpio-async-wait")]
    fn __hair_wch_drv_gpioa_exti_exti15_10_vector();
    #[cfg(feature = "wch-runtime")]
    fn __hair_wch_embassy_time_driver_vector();
}

#[derive(Clone, Copy)]
#[repr(C)]
union WchVector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

const WCH_VECTOR_COUNT: usize = 63;
const WCH_RESERVED_VECTOR: WchVector = WchVector { reserved: 0 };
const WCH_HANG_VECTOR: WchVector = WchVector {
    handler: __hair_wch_hang_vector,
};
#[cfg(feature = "gpio-async-wait")]
const WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI0_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_gpioa_exti_exti0_vector,
};
#[cfg(feature = "gpio-async-wait")]
const WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI1_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_gpioa_exti_exti1_vector,
};
#[cfg(feature = "gpio-async-wait")]
const WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI2_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_gpioa_exti_exti2_vector,
};
#[cfg(feature = "gpio-async-wait")]
const WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI3_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_gpioa_exti_exti3_vector,
};
#[cfg(feature = "gpio-async-wait")]
const WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI4_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_gpioa_exti_exti4_vector,
};
#[cfg(feature = "dma-async")]
const WCH_RUNTIME_DRV_DMA1_CH1_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_dma1_ch1_vector,
};
#[cfg(feature = "gpio-async-wait")]
const WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI9_5_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_gpioa_exti_exti9_5_vector,
};
#[cfg(feature = "i2c-async")]
const WCH_RUNTIME_DRV_I2C1_I2C1_EV_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_i2c1_i2c1_ev_vector,
};
#[cfg(feature = "i2c-async")]
const WCH_RUNTIME_DRV_I2C1_I2C1_ER_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_i2c1_i2c1_er_vector,
};
#[cfg(feature = "gpio-async-wait")]
const WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI15_10_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_drv_gpioa_exti_exti15_10_vector,
};
#[cfg(feature = "wch-runtime")]
const WCH_TIME_DRIVER_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_embassy_time_driver_vector,
};

#[repr(C, align(64))]
struct WchVectorTable([WchVector; WCH_VECTOR_COUNT]);

#[cfg(feature = "gpio-async-wait")]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti0_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[22] = WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI0_HANDLER_VECTOR;
}

#[cfg(not(feature = "gpio-async-wait"))]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti0_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "gpio-async-wait")]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti1_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[23] = WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI1_HANDLER_VECTOR;
}

#[cfg(not(feature = "gpio-async-wait"))]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti1_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "gpio-async-wait")]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti2_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[24] = WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI2_HANDLER_VECTOR;
}

#[cfg(not(feature = "gpio-async-wait"))]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti2_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "gpio-async-wait")]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti3_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[25] = WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI3_HANDLER_VECTOR;
}

#[cfg(not(feature = "gpio-async-wait"))]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti3_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "gpio-async-wait")]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti4_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[26] = WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI4_HANDLER_VECTOR;
}

#[cfg(not(feature = "gpio-async-wait"))]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti4_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "dma-async")]
const fn __hair_assign_wch_runtime_drv_dma1_ch1_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[27] = WCH_RUNTIME_DRV_DMA1_CH1_HANDLER_VECTOR;
}

#[cfg(not(feature = "dma-async"))]
const fn __hair_assign_wch_runtime_drv_dma1_ch1_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "gpio-async-wait")]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti9_5_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[39] = WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI9_5_HANDLER_VECTOR;
}

#[cfg(not(feature = "gpio-async-wait"))]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti9_5_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "i2c-async")]
const fn __hair_assign_wch_runtime_drv_i2c1_i2c1_ev_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[47] = WCH_RUNTIME_DRV_I2C1_I2C1_EV_HANDLER_VECTOR;
}

#[cfg(not(feature = "i2c-async"))]
const fn __hair_assign_wch_runtime_drv_i2c1_i2c1_ev_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "i2c-async")]
const fn __hair_assign_wch_runtime_drv_i2c1_i2c1_er_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[48] = WCH_RUNTIME_DRV_I2C1_I2C1_ER_HANDLER_VECTOR;
}

#[cfg(not(feature = "i2c-async"))]
const fn __hair_assign_wch_runtime_drv_i2c1_i2c1_er_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "gpio-async-wait")]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti15_10_handler_vector(
    table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
    table[56] = WCH_RUNTIME_DRV_GPIOA_EXTI_EXTI15_10_HANDLER_VECTOR;
}

#[cfg(not(feature = "gpio-async-wait"))]
const fn __hair_assign_wch_runtime_drv_gpioa_exti_exti15_10_handler_vector(
    _table: &mut [WchVector; WCH_VECTOR_COUNT],
) {
}

#[cfg(feature = "wch-runtime")]
const fn __hair_assign_wch_time_driver_handler_vector(table: &mut [WchVector; WCH_VECTOR_COUNT]) {
    table[57] = WCH_TIME_DRIVER_HANDLER_VECTOR;
}

#[cfg(not(feature = "wch-runtime"))]
const fn __hair_assign_wch_time_driver_handler_vector(_table: &mut [WchVector; WCH_VECTOR_COUNT]) {}

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
    __hair_assign_wch_runtime_drv_gpioa_exti_exti0_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_gpioa_exti_exti1_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_gpioa_exti_exti2_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_gpioa_exti_exti3_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_gpioa_exti_exti4_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_dma1_ch1_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_gpioa_exti_exti9_5_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_i2c1_i2c1_ev_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_i2c1_i2c1_er_handler_vector(&mut table);
    __hair_assign_wch_runtime_drv_gpioa_exti_exti15_10_handler_vector(&mut table);
    __hair_assign_wch_time_driver_handler_vector(&mut table);
    WchVectorTable(table)
}

#[unsafe(link_section = ".vector")]
#[used]
static WCH_VECTOR_TABLE: WchVectorTable = build_wch_vector_table();

global_asm!(
    r#"
    .global __hair_wch_hang_vector
__hair_wch_hang_vector:
1:
    j 1b
"#
);

#[cfg(feature = "gpio-async-wait")]
global_asm!(
    r#"
    .global __hair_wch_drv_gpioa_exti_exti0_vector
__hair_wch_drv_gpioa_exti_exti0_vector:
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
    call __hair_wch_drv_gpioa_exti_exti0_irq_rust
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

#[cfg(feature = "gpio-async-wait")]
global_asm!(
    r#"
    .global __hair_wch_drv_gpioa_exti_exti1_vector
__hair_wch_drv_gpioa_exti_exti1_vector:
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
    call __hair_wch_drv_gpioa_exti_exti1_irq_rust
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

#[cfg(feature = "gpio-async-wait")]
global_asm!(
    r#"
    .global __hair_wch_drv_gpioa_exti_exti2_vector
__hair_wch_drv_gpioa_exti_exti2_vector:
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
    call __hair_wch_drv_gpioa_exti_exti2_irq_rust
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

#[cfg(feature = "gpio-async-wait")]
global_asm!(
    r#"
    .global __hair_wch_drv_gpioa_exti_exti3_vector
__hair_wch_drv_gpioa_exti_exti3_vector:
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
    call __hair_wch_drv_gpioa_exti_exti3_irq_rust
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

#[cfg(feature = "gpio-async-wait")]
global_asm!(
    r#"
    .global __hair_wch_drv_gpioa_exti_exti4_vector
__hair_wch_drv_gpioa_exti_exti4_vector:
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
    call __hair_wch_drv_gpioa_exti_exti4_irq_rust
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

#[cfg(feature = "dma-async")]
global_asm!(
    r#"
    .global __hair_wch_drv_dma1_ch1_vector
__hair_wch_drv_dma1_ch1_vector:
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
    call __hair_wch_drv_dma1_ch1_irq_rust
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

#[cfg(feature = "gpio-async-wait")]
global_asm!(
    r#"
    .global __hair_wch_drv_gpioa_exti_exti9_5_vector
__hair_wch_drv_gpioa_exti_exti9_5_vector:
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
    call __hair_wch_drv_gpioa_exti_exti9_5_irq_rust
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

#[cfg(feature = "i2c-async")]
global_asm!(
    r#"
    .global __hair_wch_drv_i2c1_i2c1_ev_vector
__hair_wch_drv_i2c1_i2c1_ev_vector:
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
    call __hair_wch_drv_i2c1_i2c1_ev_irq_rust
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

#[cfg(feature = "i2c-async")]
global_asm!(
    r#"
    .global __hair_wch_drv_i2c1_i2c1_er_vector
__hair_wch_drv_i2c1_i2c1_er_vector:
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
    call __hair_wch_drv_i2c1_i2c1_er_irq_rust
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

#[cfg(feature = "gpio-async-wait")]
global_asm!(
    r#"
    .global __hair_wch_drv_gpioa_exti_exti15_10_vector
__hair_wch_drv_gpioa_exti_exti15_10_vector:
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
    call __hair_wch_drv_gpioa_exti_exti15_10_irq_rust
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

#[cfg(feature = "wch-runtime")]
global_asm!(
    r#"
    .global __hair_wch_embassy_time_driver_vector
__hair_wch_embassy_time_driver_vector:
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
    call __hair_wch_embassy_time_driver_irq_rust
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
    PFIC::new(DRV_PFIC_RUNTIME_RESOURCES).expect("generated WCH PFIC resources")
}

fn time_driver() -> RTCEmbassyTimeDriver {
    RTCEmbassyTimeDriver::new(DRV_TIME_RTC_RUNTIME_RESOURCES)
        .expect("generated WCH time-driver resources")
}

#[cfg(feature = "gpio-async-wait")]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti0_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::EXTI0)?;
    Ok(())
}

#[cfg(not(feature = "gpio-async-wait"))]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti0_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "gpio-async-wait")]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti1_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::EXTI1)?;
    Ok(())
}

#[cfg(not(feature = "gpio-async-wait"))]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti1_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "gpio-async-wait")]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti2_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::EXTI2)?;
    Ok(())
}

#[cfg(not(feature = "gpio-async-wait"))]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti2_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "gpio-async-wait")]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti3_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::EXTI3)?;
    Ok(())
}

#[cfg(not(feature = "gpio-async-wait"))]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti3_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "gpio-async-wait")]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti4_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::EXTI4)?;
    Ok(())
}

#[cfg(not(feature = "gpio-async-wait"))]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti4_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "dma-async")]
fn __hair_enable_wch_runtime_drv_dma1_ch1_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::DMA1Channel1)?;
    Ok(())
}

#[cfg(not(feature = "dma-async"))]
fn __hair_enable_wch_runtime_drv_dma1_ch1_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "gpio-async-wait")]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti9_5_handler_vector() -> Result<(), metadata::Error>
{
    pfic().enable_irq(Irq::EXTI95)?;
    Ok(())
}

#[cfg(not(feature = "gpio-async-wait"))]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti9_5_handler_vector() -> Result<(), metadata::Error>
{
    Ok(())
}

#[cfg(feature = "i2c-async")]
fn __hair_enable_wch_runtime_drv_i2c1_i2c1_ev_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::I2C1EV)?;
    Ok(())
}

#[cfg(not(feature = "i2c-async"))]
fn __hair_enable_wch_runtime_drv_i2c1_i2c1_ev_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "i2c-async")]
fn __hair_enable_wch_runtime_drv_i2c1_i2c1_er_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::I2C1ER)?;
    Ok(())
}

#[cfg(not(feature = "i2c-async"))]
fn __hair_enable_wch_runtime_drv_i2c1_i2c1_er_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "gpio-async-wait")]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti15_10_handler_vector() -> Result<(), metadata::Error>
{
    pfic().enable_irq(Irq::EXTI1510)?;
    Ok(())
}

#[cfg(not(feature = "gpio-async-wait"))]
fn __hair_enable_wch_runtime_drv_gpioa_exti_exti15_10_handler_vector() -> Result<(), metadata::Error>
{
    Ok(())
}

#[cfg(feature = "wch-runtime")]
fn __hair_enable_wch_time_driver_handler_vector() -> Result<(), metadata::Error> {
    pfic().enable_irq(Irq::RTCAlarm)?;
    Ok(())
}

#[cfg(not(feature = "wch-runtime"))]
fn __hair_enable_wch_time_driver_handler_vector() -> Result<(), metadata::Error> {
    Ok(())
}

#[cfg(feature = "dma-async")]
const GENERATED_WCH_RUNTIME_DRV_DMA1_RUNTIME_RESOURCES: DMA1RuntimeResources =
    DMA1RuntimeResources {};

#[cfg(feature = "dma-async")]
fn generated_wch_runtime_drv_dma1() -> DMA1 {
    DMA1::new(GENERATED_WCH_RUNTIME_DRV_DMA1_RUNTIME_RESOURCES)
        .expect("generated WCH runtime driver resources")
}

pub fn init_embassy_time_runtime() -> Result<(), metadata::Error> {
    time_driver().init_time_driver()?;
    unsafe {
        asm!("csrw 0x804, {value}", value = in(reg) 0x3usize);
        asm!(
            "csrw mtvec, {value}",
            value = in(reg) ((&WCH_VECTOR_TABLE as *const WchVectorTable as usize) | 0x3)
        );
    }
    __hair_enable_wch_runtime_drv_gpioa_exti_exti0_handler_vector()?;
    __hair_enable_wch_runtime_drv_gpioa_exti_exti1_handler_vector()?;
    __hair_enable_wch_runtime_drv_gpioa_exti_exti2_handler_vector()?;
    __hair_enable_wch_runtime_drv_gpioa_exti_exti3_handler_vector()?;
    __hair_enable_wch_runtime_drv_gpioa_exti_exti4_handler_vector()?;
    __hair_enable_wch_runtime_drv_dma1_ch1_handler_vector()?;
    __hair_enable_wch_runtime_drv_gpioa_exti_exti9_5_handler_vector()?;
    __hair_enable_wch_runtime_drv_i2c1_i2c1_ev_handler_vector()?;
    __hair_enable_wch_runtime_drv_i2c1_i2c1_er_handler_vector()?;
    __hair_enable_wch_runtime_drv_gpioa_exti_exti15_10_handler_vector()?;
    __hair_enable_wch_time_driver_handler_vector()?;
    unsafe {
        asm!("csrs mie, {value}", value = in(reg) 0x800usize);
        asm!("csrs mstatus, {value}", value = in(reg) 0x88usize);
    }
    Ok(())
}

#[cfg(feature = "gpio-async-wait")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_gpioa_exti_exti0_irq_rust() {
    let _ = generated_drv_gpioa_signal_gpio_wait(0u32);
    let _ = generated_drv_gpiob_signal_gpio_wait(0u32);
    let _ = generated_drv_gpiod_signal_gpio_wait(0u32);
}

#[cfg(feature = "gpio-async-wait")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_gpioa_exti_exti1_irq_rust() {
    let _ = generated_drv_gpioa_signal_gpio_wait(1u32);
    let _ = generated_drv_gpiob_signal_gpio_wait(1u32);
    let _ = generated_drv_gpiod_signal_gpio_wait(1u32);
}

#[cfg(feature = "gpio-async-wait")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_gpioa_exti_exti2_irq_rust() {
    let _ = generated_drv_gpioa_signal_gpio_wait(2u32);
}

#[cfg(feature = "gpio-async-wait")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_gpioa_exti_exti3_irq_rust() {
    let _ = generated_drv_gpioa_signal_gpio_wait(3u32);
    let _ = generated_drv_gpiob_signal_gpio_wait(3u32);
}

#[cfg(feature = "gpio-async-wait")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_gpioa_exti_exti4_irq_rust() {
    let _ = generated_drv_gpioa_signal_gpio_wait(4u32);
    let _ = generated_drv_gpiob_signal_gpio_wait(4u32);
}

#[cfg(feature = "dma-async")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_dma1_ch1_irq_rust() {
    let _ = generated_wch_runtime_drv_dma1().on_interrupt(1);
}

#[cfg(feature = "gpio-async-wait")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_gpioa_exti_exti9_5_irq_rust() {
    let _ = generated_drv_gpioa_signal_gpio_wait(5u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(6u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(7u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(9u32);
    let _ = generated_drv_gpiob_signal_gpio_wait(5u32);
    let _ = generated_drv_gpiob_signal_gpio_wait(6u32);
    let _ = generated_drv_gpiob_signal_gpio_wait(7u32);
}

#[cfg(feature = "i2c-async")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_i2c1_i2c1_ev_irq_rust() {
    let _ = generated_drv_i2c1_signal_i2c_async();
    let _ = generated_drv_i2c1_slave_on_i2c_slave_interrupt();
}

#[cfg(feature = "i2c-async")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_i2c1_i2c1_er_irq_rust() {
    let _ = generated_drv_i2c1_signal_i2c_async();
    let _ = generated_drv_i2c1_slave_on_i2c_slave_interrupt();
}

#[cfg(feature = "gpio-async-wait")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_drv_gpioa_exti_exti15_10_irq_rust() {
    let _ = generated_drv_gpioa_signal_gpio_wait(10u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(11u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(12u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(13u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(14u32);
    let _ = generated_drv_gpioa_signal_gpio_wait(15u32);
}

#[cfg(feature = "wch-runtime")]
#[unsafe(no_mangle)]
extern "C" fn __hair_wch_embassy_time_driver_irq_rust() {
    time_driver().on_time_driver_interrupt();
}
