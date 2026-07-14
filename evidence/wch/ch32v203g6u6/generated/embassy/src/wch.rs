//! Generated WCH/QingKe runtime support for CH32V203G6U6.

use crate::interrupt::{DRV_PFIC_RESOURCES, Irq, PFIC};
use crate::metadata;
use crate::time::{DRV_TIME_RTC_RESOURCES, RTCEmbassyTimeDriver};
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
    fn __hair_wch_embassy_time_driver_vector();
}

#[derive(Clone, Copy)]
#[repr(C)]
union WchVector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

const WCH_VECTOR_COUNT: usize = 63;
const WCH_TIME_DRIVER_VECTOR_SLOT: usize = 57;
const WCH_RESERVED_VECTOR: WchVector = WchVector { reserved: 0 };
const WCH_HANG_VECTOR: WchVector = WchVector {
    handler: __hair_wch_hang_vector,
};
const WCH_TIME_DRIVER_HANDLER_VECTOR: WchVector = WchVector {
    handler: __hair_wch_embassy_time_driver_vector,
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
    table[WCH_TIME_DRIVER_VECTOR_SLOT] = WCH_TIME_DRIVER_HANDLER_VECTOR;
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
    PFIC::new(DRV_PFIC_RESOURCES).expect("generated WCH PFIC resources")
}

fn time_driver() -> RTCEmbassyTimeDriver {
    RTCEmbassyTimeDriver::new(DRV_TIME_RTC_RESOURCES).expect("generated WCH time-driver resources")
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
    pfic().enable_irq(Irq::RTCAlarm)?;
            unsafe {
        asm!("csrs mie, {value}", value = in(reg) 0x800usize);
        asm!("csrs mstatus, {value}", value = in(reg) 0x88usize);
    }
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn __hair_wch_embassy_time_driver_irq_rust() {
    time_driver().on_time_driver_interrupt();
}
