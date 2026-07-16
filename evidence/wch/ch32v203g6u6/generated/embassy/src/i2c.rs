//! Generated Embassy-style i2c module for CH32V203G6U6.

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
    module_name: "i2c",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: I2C1 (i2c) from canonical block block.i2c1 -> i2c
pub const DRV_I2C1_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding {
    id: "clk.i2c1",
    name: "I2C1 clock binding",
    consumer_ref: "periph.i2c1",
    clock_ref: "clk.pclk1",
    controller_ref: Some("block.rcc"),
    binding_kind: "gated",
    control_refs: &["reg.rcc.apb1pcenr"],
    enable_operation_refs: &[],
    disable_operation_refs: &[],
}];
pub const DRV_I2C1_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding {
    id: "rst.i2c1",
    name: "I2C1 reset binding",
    target_ref: "periph.i2c1",
    controller_ref: Some("block.rcc"),
    reset_domain_ref: Some("rst.apb1"),
    binding_kind: "local",
    control_refs: &["reg.rcc.apb1prstr"],
    assert_operation_refs: &[],
    release_operation_refs: &[],
}];
pub const DRV_I2C1_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[
    metadata::InterruptSource {
        id: "isrc.i2c1.er",
        name: "I2C1 ER interrupt source",
        source_ref: "periph.i2c1",
        producer_ref: None,
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
    metadata::InterruptSource {
        id: "isrc.i2c1.ev",
        name: "I2C1 EV interrupt source",
        source_ref: "periph.i2c1",
        producer_ref: None,
        kind: "peripheral",
        flag_refs: &[],
        clear_operation_refs: &[],
    },
];
pub const DRV_I2C1_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[
    metadata::InterruptRoute {
        id: "iroute.i2c1.er",
        name: "I2C1 ER interrupt route",
        source_ref: "isrc.i2c1.er",
        interrupt_ref: "int.i2c1er",
        controller_ref: "block.pfic",
        cpu_target_ref: None,
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
    metadata::InterruptRoute {
        id: "iroute.i2c1.ev",
        name: "I2C1 EV interrupt route",
        source_ref: "isrc.i2c1.ev",
        interrupt_ref: "int.i2c1ev",
        controller_ref: "block.pfic",
        cpu_target_ref: None,
        line_index: None,
        route_type: "hardwired",
        control_refs: &[],
        acknowledge_operation_refs: &[],
        shared_group: None,
    },
];
pub const DRV_I2C1_DMA_CHANNELS: &[metadata::DmaChannel] = &[
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
pub const DRV_I2C1_DMA_ROUTES: &[metadata::DmaRoute] = &[
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
pub const DRV_I2C1_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.i2c1.scl.pb6.r0",
    name: "I2C1 SCL on PB6 (remap 0)",
    pin_ref: "pin.pb6",
    peripheral_ref: "periph.i2c1",
    signal: "SCL",
    route_type: "selectable",
    control_refs: &["reg.afio.pcfr1"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_I2C1_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.i2c1.sda.pb7.r0",
    name: "I2C1 SDA on PB7 (remap 0)",
    pin_ref: "pin.pb7",
    peripheral_ref: "periph.i2c1",
    signal: "SDA",
    route_type: "selectable",
    control_refs: &["reg.afio.pcfr1"],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: Some(true),
}];
pub const DRV_I2C1_PIN_ROLE_2_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute {
    id: "pinroute.i2c1.smba.pb5",
    name: "I2C1 SMBA on PB5",
    pin_ref: "pin.pb5",
    peripheral_ref: "periph.i2c1",
    signal: "SMBA",
    route_type: "hardwired",
    control_refs: &[],
    electrical_constraint_refs: &[],
    conflict_refs: &[],
    default_after_reset: None,
}];
pub const DRV_I2C1_PIN_ROLES: &[metadata::PinRole] = &[
    metadata::PinRole {
        role: "scl",
        signal: "SCL",
        routes: DRV_I2C1_PIN_ROLE_0_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "sda",
        signal: "SDA",
        routes: DRV_I2C1_PIN_ROLE_1_ROUTES,
        requirement: metadata::ResourceRequirement::Required,
    },
    metadata::PinRole {
        role: "smba",
        signal: "SMBA",
        routes: DRV_I2C1_PIN_ROLE_2_ROUTES,
        requirement: metadata::ResourceRequirement::Optional,
    },
];
pub const DRV_I2C1_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[
    metadata::SemanticOperation {
        id: "op.i2c1.init_master_100khz",
        name: "I2C1 initialize fixed 100 kHz master timing",
        description: None,
        kind: Some("initialization"),
        target_refs: &["periph.i2c1"],
        steps: &[
            metadata::SemanticOperationStep {
                index: 0,
                action: "write",
                target_ref: Some("reg.i2c1.ctlr1"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Clear PE",
                }),
                value: None,
                description: Some(
                    "Disable the peripheral before reprogramming the fixed timing profile.",
                ),
            },
            metadata::SemanticOperationStep {
                index: 1,
                action: "write",
                target_ref: Some("reg.i2c1.ctlr1"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set SWRST = 1",
                }),
                value: None,
                description: Some(
                    "Assert the documented software reset bit before reinitialization.",
                ),
            },
            metadata::SemanticOperationStep {
                index: 2,
                action: "write",
                target_ref: Some("reg.i2c1.ctlr1"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Clear SWRST",
                }),
                value: None,
                description: Some("Release the software reset bit."),
            },
            metadata::SemanticOperationStep {
                index: 3,
                action: "write",
                target_ref: Some("reg.i2c1.ctlr2"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Write FREQ = 36",
                }),
                value: None,
                description: Some(
                    "Model the fixed APB1 = 36 MHz kernel clock selected for the first-cut generated helper.",
                ),
            },
            metadata::SemanticOperationStep {
                index: 4,
                action: "write",
                target_ref: Some("reg.i2c1.ckcfgr"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Clear FS",
                }),
                value: None,
                description: Some("Use standard-mode timing rather than fast mode."),
            },
            metadata::SemanticOperationStep {
                index: 5,
                action: "write",
                target_ref: Some("reg.i2c1.ckcfgr"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Clear DUTY",
                }),
                value: None,
                description: Some(
                    "Keep the reset-default duty selection for the fixed standard-mode profile.",
                ),
            },
            metadata::SemanticOperationStep {
                index: 6,
                action: "write",
                target_ref: Some("reg.i2c1.ckcfgr"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Write CCR = 180",
                }),
                value: None,
                description: Some(
                    "Program the standard-mode clock divider for a 100 kHz bus with a 36 MHz APB1 clock.",
                ),
            },
            metadata::SemanticOperationStep {
                index: 7,
                action: "write",
                target_ref: Some("reg.i2c1.rtr"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Write TRISE = 37",
                }),
                value: None,
                description: Some(
                    "Program the standard-mode maximum rise-time value for the same fixed timing profile.",
                ),
            },
            metadata::SemanticOperationStep {
                index: 8,
                action: "write",
                target_ref: Some("reg.i2c1.ctlr1"),
                expression: Some(metadata::SemanticExpression {
                    language: Some("plain"),
                    text: "Set PE = 1",
                }),
                value: None,
                description: Some(
                    "Re-enable the peripheral after the fixed timing profile is loaded.",
                ),
            },
        ],
        preconditions: &[],
        postconditions: &[],
    },
];
pub const DRV_I2C1_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_I2C1_CAPABILITY_TAGS: &[&str] = &["embedded-hal-async-i2c-master"];

#[derive(Debug, Clone, Copy)]
pub struct I2C1RuntimeResources {}

pub const DRV_I2C1_RUNTIME_RESOURCES: I2C1RuntimeResources = I2C1RuntimeResources {};

#[derive(Debug, Clone, Copy)]
pub struct I2C1MetadataResources {
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

pub const DRV_I2C1_METADATA_RESOURCES: I2C1MetadataResources = I2C1MetadataResources {
    clocks: DRV_I2C1_CLOCK_BINDINGS,
    resets: DRV_I2C1_RESET_BINDINGS,
    interrupt_sources: DRV_I2C1_INTERRUPT_SOURCES,
    interrupts: DRV_I2C1_INTERRUPT_ROUTES,
    dma_channels: DRV_I2C1_DMA_CHANNELS,
    dma: DRV_I2C1_DMA_ROUTES,
    pins: DRV_I2C1_PIN_ROLES,
    init_operations: DRV_I2C1_INIT_OPERATIONS,
    state_machines: DRV_I2C1_STATE_MACHINES,
    lowering_pattern: Some("legacy-event-i2c-master"),
    time_driver_source: None,
    capability_tags: DRV_I2C1_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct I2C1;

impl I2C1 {
    pub fn new(resources: I2C1RuntimeResources) -> Result<Self, metadata::Error> {
        let _ = resources;
        Ok(Self)
    }

    pub fn metadata_resources() -> I2C1MetadataResources {
        DRV_I2C1_METADATA_RESOURCES
    }
    /// Enable the I2C1 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00200000u32, 0x00200000u32)?;
        Ok(())
    }

    /// Disable the I2C1 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x4002101Cu64, 0x00200000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for I2C1.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00200000u32, 0x00200000u32)?;
        Ok(())
    }

    /// Release reset for I2C1.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40021010u64, 0x00200000u32, 0x00000000u32)?;
        Ok(())
    }

    fn generated_validate_7bit_address(&self, address: u8) -> Result<u8, metadata::Error> {
        if address > 0x7Fu8 {
            return Err(metadata::Error::Unsupported(
                "I2C address exceeds the modeled 7-bit master subset",
            ));
        }
        Ok(address)
    }

    pub fn init_master(&self) -> Result<(), metadata::Error> {
        self.apply_init_master_100khz()?;
        Ok(())
    }

    fn generated_check_and_clear_i2c_error_flags(&self) -> Result<(), metadata::Error> {
        if ((u32::from(read_u16(0x40005414u64)?) & 0x00000400u32) >> 10) != 0u32 {
            modify_u16(0x40005414u64, 0x0400u16, 0x0000u16)?;
            return Err(metadata::Error::NoAcknowledge);
        }
        if ((u32::from(read_u16(0x40005414u64)?) & 0x00000200u32) >> 9) != 0u32 {
            modify_u16(0x40005414u64, 0x0200u16, 0x0000u16)?;
            return Err(metadata::Error::ArbitrationLoss);
        }
        if ((u32::from(read_u16(0x40005414u64)?) & 0x00000100u32) >> 8) != 0u32 {
            modify_u16(0x40005414u64, 0x0100u16, 0x0000u16)?;
            return Err(metadata::Error::Bus);
        }
        Ok(())
    }

    fn generated_wait_until_bus_free(&self) -> Result<(), metadata::Error> {
        while ((u32::from(read_u16(0x40005418u64)?) & 0x00000002u32) >> 1) != 0u32 {
            self.generated_check_and_clear_i2c_error_flags()?;
            core::hint::spin_loop();
        }
        Ok(())
    }

    fn generated_send_start(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40005400u64, 0x0100u16, 0x0100u16)?;
        while (u32::from(read_u16(0x40005414u64)?) & 0x00000001u32) == 0u32 {
            self.generated_check_and_clear_i2c_error_flags()?;
            core::hint::spin_loop();
        }
        Ok(())
    }

    fn generated_set_ack(&self, enabled: bool) -> Result<(), metadata::Error> {
        if enabled {
            modify_u16(0x40005400u64, 0x0400u16, 0x0400u16)?;
        } else {
            modify_u16(0x40005400u64, 0x0400u16, 0x0000u16)?;
        }
        Ok(())
    }

    fn generated_set_ack_position(&self, enabled: bool) -> Result<(), metadata::Error> {
        if enabled {
            modify_u16(0x40005400u64, 0x0800u16, 0x0800u16)?;
        } else {
            modify_u16(0x40005400u64, 0x0800u16, 0x0000u16)?;
        }
        Ok(())
    }

    fn generated_send_stop(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40005400u64, 0x0200u16, 0x0200u16)?;
        Ok(())
    }

    fn generated_send_address(&self, address: u8, read: bool) -> Result<(), metadata::Error> {
        let address = self.generated_validate_7bit_address(address)?;
        let header = (address << 1) | u8::from(read);
        modify_u16(0x40005410u64, 0x00FFu16, (u16::from(header)) & 0x00FFu16)?;

        while ((u32::from(read_u16(0x40005414u64)?) & 0x00000002u32) >> 1) == 0u32 {
            self.generated_check_and_clear_i2c_error_flags()?;
            core::hint::spin_loop();
        }
        let _ = u32::from(read_u16(0x40005414u64)?);
        let _ = u32::from(read_u16(0x40005418u64)?);
        Ok(())
    }

    fn generated_send_data_byte(&self, value: u8) -> Result<(), metadata::Error> {
        while ((u32::from(read_u16(0x40005414u64)?) & 0x00000080u32) >> 7) == 0u32 {
            self.generated_check_and_clear_i2c_error_flags()?;
            core::hint::spin_loop();
        }
        modify_u16(0x40005410u64, 0x00FFu16, (u16::from(value)) & 0x00FFu16)?;

        while ((u32::from(read_u16(0x40005414u64)?) & 0x00000004u32) >> 2) == 0u32 {
            self.generated_check_and_clear_i2c_error_flags()?;
            core::hint::spin_loop();
        }
        Ok(())
    }

    fn generated_receive_data_byte(&self) -> Result<u8, metadata::Error> {
        while ((u32::from(read_u16(0x40005414u64)?) & 0x00000040u32) >> 6) == 0u32 {
            self.generated_check_and_clear_i2c_error_flags()?;
            core::hint::spin_loop();
        }
        let value = u32::from(read_u16(0x40005410u64)?) & 0x000000FFu32;
        u8::try_from(value)
            .map_err(|_| metadata::Error::Unsupported("generated I2C data field exceeds u8"))
    }

    fn generated_write_frame(
        &self,
        address: u8,
        write: &[u8],
        send_start: bool,
        send_stop: bool,
    ) -> Result<(), metadata::Error> {
        if write.is_empty() {
            return Ok(());
        }
        if send_start {
            self.generated_send_start()?;
            self.generated_send_address(address, false)?;
        }
        for &value in write {
            self.generated_send_data_byte(value)?;
        }
        while ((u32::from(read_u16(0x40005414u64)?) & 0x00000004u32) >> 2) == 0u32 {
            self.generated_check_and_clear_i2c_error_flags()?;
            core::hint::spin_loop();
        }
        if send_stop {
            self.generated_send_stop()?;
        }
        Ok(())
    }

    fn generated_read_frame(
        &self,
        address: u8,
        read: &mut [u8],
        send_start: bool,
        send_nack: bool,
        send_stop: bool,
    ) -> Result<(), metadata::Error> {
        if read.is_empty() {
            return Ok(());
        }
        let Some((last, prefix)) = read.split_last_mut() else {
            return Ok(());
        };
        if send_start {
            self.generated_set_ack_position(false)?;
            self.generated_set_ack(true)?;
            self.generated_send_start()?;
            self.generated_send_address(address, true)?;
        }
        for value in prefix {
            *value = self.generated_receive_data_byte()?;
        }
        if send_nack {
            self.generated_set_ack(false)?;
        }
        if send_stop {
            self.generated_send_stop()?;
        }
        *last = self.generated_receive_data_byte()?;
        Ok(())
    }

    pub fn blocking_write_7bit(&self, address: u8, write: &[u8]) -> Result<(), metadata::Error> {
        if write.is_empty() {
            return Ok(());
        }
        self.generated_wait_until_bus_free()?;
        self.generated_write_frame(address, write, true, true)
    }

    pub fn blocking_read_7bit(&self, address: u8, read: &mut [u8]) -> Result<(), metadata::Error> {
        if read.is_empty() {
            return Ok(());
        }
        self.generated_wait_until_bus_free()?;
        self.generated_read_frame(address, read, true, true, true)
    }

    pub fn blocking_write_read_7bit(
        &self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), metadata::Error> {
        if write.is_empty() {
            return self.blocking_read_7bit(address, read);
        }
        if read.is_empty() {
            return self.blocking_write_7bit(address, write);
        }
        self.generated_wait_until_bus_free()?;
        self.generated_write_frame(address, write, true, false)?;
        self.generated_read_frame(address, read, true, true, true)
    }

    pub fn blocking_transaction_7bit(
        &self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), metadata::Error> {
        let mut previous_kind: Option<bool> = None;
        let mut last_non_empty_index = None;
        for (index, operation) in operations.iter().enumerate() {
            let is_empty = match operation {
                embedded_hal::i2c::Operation::Write(write) => write.is_empty(),
                embedded_hal::i2c::Operation::Read(read) => read.is_empty(),
            };
            if !is_empty {
                last_non_empty_index = Some(index);
            }
        }
        let Some(last_non_empty_index) = last_non_empty_index else {
            return Ok(());
        };
        self.generated_wait_until_bus_free()?;
        for index in 0..operations.len() {
            let current_kind = match &operations[index] {
                embedded_hal::i2c::Operation::Write(write) if !write.is_empty() => Some(false),
                embedded_hal::i2c::Operation::Read(read) if !read.is_empty() => Some(true),
                _ => None,
            };
            let Some(current_kind) = current_kind else {
                continue;
            };
            let send_start = previous_kind != Some(current_kind);
            let is_last = index == last_non_empty_index;
            let next_kind = operations[index + 1..]
                .iter()
                .find_map(|operation| match operation {
                    embedded_hal::i2c::Operation::Write(write) if !write.is_empty() => Some(false),
                    embedded_hal::i2c::Operation::Read(read) if !read.is_empty() => Some(true),
                    _ => None,
                });
            let next_changes_kind = next_kind != Some(current_kind);
            match &mut operations[index] {
                embedded_hal::i2c::Operation::Write(write) => {
                    self.generated_write_frame(address, write, send_start, is_last)?;
                }
                embedded_hal::i2c::Operation::Read(read) => {
                    self.generated_read_frame(
                        address,
                        read,
                        send_start,
                        next_changes_kind,
                        is_last,
                    )?;
                }
            }
            previous_kind = Some(current_kind);
        }
        Ok(())
    }

    #[cfg(feature = "i2c-async")]
    async fn generated_wait_i2c_async_until<F>(&self, mut ready: F) -> Result<(), metadata::Error>
    where
        F: FnMut(&Self) -> Result<bool, metadata::Error>,
    {
        loop {
            generated_drv_i2c1_prepare_i2c_async_wait();
            self.generated_check_and_clear_i2c_error_flags()?;
            if ready(self)? {
                return Ok(());
            }
            self.generated_enable_i2c_async_interrupts()?;
            generated_drv_i2c1_wait_i2c_async().await?;
        }
    }

    #[cfg(feature = "i2c-async")]
    fn generated_enable_i2c_async_interrupts(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40005404u64, 0x0200u16, 0x0200u16)?;
        modify_u16(0x40005404u64, 0x0400u16, 0x0400u16)?;
        modify_u16(0x40005404u64, 0x0100u16, 0x0100u16)?;
        Ok(())
    }

    #[cfg(feature = "i2c-async")]
    async fn generated_send_start_async(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40005400u64, 0x0100u16, 0x0100u16)?;
        self.generated_wait_i2c_async_until(|_| {
            Ok((u32::from(read_u16(0x40005414u64)?) & 0x00000001u32) != 0u32)
        })
        .await
    }

    #[cfg(feature = "i2c-async")]
    async fn generated_send_address_async(
        &self,
        address: u8,
        read: bool,
    ) -> Result<(), metadata::Error> {
        let address = self.generated_validate_7bit_address(address)?;
        let header = (address << 1) | u8::from(read);
        modify_u16(0x40005410u64, 0x00FFu16, (u16::from(header)) & 0x00FFu16)?;

        self.generated_wait_i2c_async_until(|_| {
            Ok(((u32::from(read_u16(0x40005414u64)?) & 0x00000002u32) >> 1) != 0u32)
        })
        .await?;
        let _ = u32::from(read_u16(0x40005414u64)?);
        let _ = u32::from(read_u16(0x40005418u64)?);
        Ok(())
    }

    #[cfg(feature = "i2c-async")]
    async fn generated_send_data_byte_async(&self, value: u8) -> Result<(), metadata::Error> {
        self.generated_wait_i2c_async_until(|_| {
            Ok(((u32::from(read_u16(0x40005414u64)?) & 0x00000080u32) >> 7) != 0u32)
        })
        .await?;
        modify_u16(0x40005410u64, 0x00FFu16, (u16::from(value)) & 0x00FFu16)?;

        self.generated_wait_i2c_async_until(|_| {
            Ok(((u32::from(read_u16(0x40005414u64)?) & 0x00000004u32) >> 2) != 0u32)
        })
        .await
    }

    #[cfg(feature = "i2c-async")]
    async fn generated_receive_data_byte_async(&self) -> Result<u8, metadata::Error> {
        self.generated_wait_i2c_async_until(|_| {
            Ok(((u32::from(read_u16(0x40005414u64)?) & 0x00000040u32) >> 6) != 0u32)
        })
        .await?;
        let value = u32::from(read_u16(0x40005410u64)?) & 0x000000FFu32;
        u8::try_from(value)
            .map_err(|_| metadata::Error::Unsupported("generated I2C data field exceeds u8"))
    }

    #[cfg(feature = "i2c-async")]
    async fn generated_write_frame_async(
        &self,
        address: u8,
        write: &[u8],
        send_start: bool,
        send_stop: bool,
    ) -> Result<(), metadata::Error> {
        if write.is_empty() {
            return Ok(());
        }
        if send_start {
            self.generated_send_start_async().await?;
            self.generated_send_address_async(address, false).await?;
        }
        for &value in write {
            self.generated_send_data_byte_async(value).await?;
        }
        self.generated_wait_i2c_async_until(|_| {
            Ok(((u32::from(read_u16(0x40005414u64)?) & 0x00000004u32) >> 2) != 0u32)
        })
        .await?;
        if send_stop {
            self.generated_send_stop()?;
        }
        Ok(())
    }

    #[cfg(feature = "i2c-async")]
    async fn generated_read_frame_async(
        &self,
        address: u8,
        read: &mut [u8],
        send_start: bool,
        send_nack: bool,
        send_stop: bool,
    ) -> Result<(), metadata::Error> {
        if read.is_empty() {
            return Ok(());
        }
        let Some((last, prefix)) = read.split_last_mut() else {
            return Ok(());
        };
        if send_start {
            self.generated_set_ack_position(false)?;
            self.generated_set_ack(true)?;
            self.generated_send_start_async().await?;
            self.generated_send_address_async(address, true).await?;
        }
        for value in prefix {
            *value = self.generated_receive_data_byte_async().await?;
        }
        if send_nack {
            self.generated_set_ack(false)?;
        }
        if send_stop {
            self.generated_send_stop()?;
        }
        *last = self.generated_receive_data_byte_async().await?;
        Ok(())
    }

    #[cfg(feature = "i2c-async")]
    pub async fn write_async_7bit(&self, address: u8, write: &[u8]) -> Result<(), metadata::Error> {
        if write.is_empty() {
            return Ok(());
        }
        self.generated_wait_until_bus_free()?;
        self.generated_write_frame_async(address, write, true, true)
            .await
    }

    #[cfg(feature = "i2c-async")]
    pub async fn read_async_7bit(
        &self,
        address: u8,
        read: &mut [u8],
    ) -> Result<(), metadata::Error> {
        if read.is_empty() {
            return Ok(());
        }
        self.generated_wait_until_bus_free()?;
        self.generated_read_frame_async(address, read, true, true, true)
            .await
    }

    #[cfg(feature = "i2c-async")]
    pub async fn write_read_async_7bit(
        &self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), metadata::Error> {
        if write.is_empty() {
            return self.read_async_7bit(address, read).await;
        }
        if read.is_empty() {
            return self.write_async_7bit(address, write).await;
        }
        self.generated_wait_until_bus_free()?;
        self.generated_write_frame_async(address, write, true, false)
            .await?;
        self.generated_read_frame_async(address, read, true, true, true)
            .await
    }

    #[cfg(feature = "i2c-async")]
    pub async fn transaction_async_7bit(
        &self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), metadata::Error> {
        let mut previous_kind: Option<bool> = None;
        let mut last_non_empty_index = None;
        for (index, operation) in operations.iter().enumerate() {
            let is_empty = match operation {
                embedded_hal::i2c::Operation::Write(write) => write.is_empty(),
                embedded_hal::i2c::Operation::Read(read) => read.is_empty(),
            };
            if !is_empty {
                last_non_empty_index = Some(index);
            }
        }
        let Some(last_non_empty_index) = last_non_empty_index else {
            return Ok(());
        };
        self.generated_wait_until_bus_free()?;
        for index in 0..operations.len() {
            let current_kind = match &operations[index] {
                embedded_hal::i2c::Operation::Write(write) if !write.is_empty() => Some(false),
                embedded_hal::i2c::Operation::Read(read) if !read.is_empty() => Some(true),
                _ => None,
            };
            let Some(current_kind) = current_kind else {
                continue;
            };
            let send_start = previous_kind != Some(current_kind);
            let is_last = index == last_non_empty_index;
            let next_kind = operations[index + 1..]
                .iter()
                .find_map(|operation| match operation {
                    embedded_hal::i2c::Operation::Write(write) if !write.is_empty() => Some(false),
                    embedded_hal::i2c::Operation::Read(read) if !read.is_empty() => Some(true),
                    _ => None,
                });
            let next_changes_kind = next_kind != Some(current_kind);
            match &mut operations[index] {
                embedded_hal::i2c::Operation::Write(write) => {
                    self.generated_write_frame_async(address, write, send_start, is_last)
                        .await?;
                }
                embedded_hal::i2c::Operation::Read(read) => {
                    self.generated_read_frame_async(
                        address,
                        read,
                        send_start,
                        next_changes_kind,
                        is_last,
                    )
                    .await?;
                }
            }
            previous_kind = Some(current_kind);
        }
        Ok(())
    }

    pub fn apply_init_master_100khz(&self) -> Result<(), metadata::Error> {
        modify_u16(0x40005400u64, 0x0001u16, 0x0000u16)?;
        modify_u16(0x40005400u64, 0x8000u16, 0x8000u16)?;
        modify_u16(0x40005400u64, 0x8000u16, 0x0000u16)?;
        modify_u16(0x40005404u64, 0x003Fu16, 0x0024u16)?;
        modify_u16(0x4000541Cu64, 0x8000u16, 0x0000u16)?;
        modify_u16(0x4000541Cu64, 0x4000u16, 0x0000u16)?;
        modify_u16(0x4000541Cu64, 0x0FFFu16, 0x00B4u16)?;
        modify_u16(0x40005420u64, 0x003Fu16, 0x0025u16)?;
        modify_u16(0x40005400u64, 0x0001u16, 0x0001u16)?;
        Ok(())
    }
}

#[cfg(feature = "i2c")]
impl embedded_hal::i2c::ErrorType for I2C1 {
    type Error = metadata::Error;
}

#[cfg(feature = "i2c")]
impl embedded_hal::i2c::I2c<embedded_hal::i2c::SevenBitAddress> for I2C1 {
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.blocking_read_7bit(address, read)
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.blocking_write_7bit(address, write)
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.blocking_write_read_7bit(address, write, read)
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.blocking_transaction_7bit(address, operations)
    }
}

#[cfg(feature = "i2c-async")]
#[derive(Debug)]
struct GeneratedI2C1I2cAsyncState {
    ready: bool,
    waker: Option<core::task::Waker>,
}

#[cfg(feature = "i2c-async")]
impl GeneratedI2C1I2cAsyncState {
    const fn new() -> Self {
        Self {
            ready: false,
            waker: None,
        }
    }
}

#[cfg(feature = "i2c-async")]
static GENERATED_DRV_I2C1_I2C_ASYNC_STATE: critical_section::Mutex<
    core::cell::RefCell<GeneratedI2C1I2cAsyncState>,
> = critical_section::Mutex::new(core::cell::RefCell::new(GeneratedI2C1I2cAsyncState::new()));

#[cfg(feature = "i2c-async")]
fn generated_drv_i2c1_prepare_i2c_async_wait() {
    critical_section::with(|cs| {
        let mut state = GENERATED_DRV_I2C1_I2C_ASYNC_STATE.borrow(cs).borrow_mut();
        state.ready = false;
        state.waker = None;
    });
}

#[cfg(feature = "i2c-async")]
async fn generated_drv_i2c1_wait_i2c_async() -> Result<(), metadata::Error> {
    core::future::poll_fn(|cx| {
        critical_section::with(|cs| {
            let mut state = GENERATED_DRV_I2C1_I2C_ASYNC_STATE.borrow(cs).borrow_mut();
            if state.ready {
                state.ready = false;
                core::task::Poll::Ready(Ok(()))
            } else {
                state.waker = Some(cx.waker().clone());
                core::task::Poll::Pending
            }
        })
    })
    .await
}

#[cfg(feature = "i2c-async")]
pub(crate) fn generated_drv_i2c1_signal_i2c_async() -> Result<(), metadata::Error> {
    modify_u16(0x40005404u64, 0x0200u16, 0x0000u16)?;
    modify_u16(0x40005404u64, 0x0400u16, 0x0000u16)?;
    modify_u16(0x40005404u64, 0x0100u16, 0x0000u16)?;
    let waker = critical_section::with(|cs| {
        let mut state = GENERATED_DRV_I2C1_I2C_ASYNC_STATE.borrow(cs).borrow_mut();
        state.ready = true;
        state.waker.take()
    });
    if let Some(waker) = waker {
        waker.wake();
    }
    Ok(())
}

#[cfg(feature = "i2c-async")]
impl embedded_hal_async::i2c::I2c<embedded_hal::i2c::SevenBitAddress> for I2C1 {
    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.read_async_7bit(address, read).await
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.write_async_7bit(address, write).await
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write_read_async_7bit(address, write, read).await
    }

    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.transaction_async_7bit(address, operations).await
    }
}
