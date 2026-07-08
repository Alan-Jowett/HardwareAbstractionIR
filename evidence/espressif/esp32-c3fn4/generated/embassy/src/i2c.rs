//! Generated Embassy-style i2c module for ESP32-C3FN4.

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
fn read_u32(address: u64) -> Result<u32, metadata::Error> {
    let address = checked_address(address, core::mem::align_of::<u32>())?;
    unsafe { Ok(read_volatile(address as *const u32)) }
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

// Driver instance: I2c0 (i2c) from canonical block block.i2c0 -> i2c
pub const DRV_I2C0_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clkbind.i2c0", name: "I2C_EXT0_CLK_EN", consumer_ref: "per.i2c0", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_I2C0_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rstbind.i2c0", name: "I2C_EXT0_RST", target_ref: "per.i2c0", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_I2C0_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.i2c0", name: "I2C_EXT0", source_ref: "per.i2c0", producer_ref: Some("block.i2c0"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_I2C0_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.i2c0", name: "I2C0 interrupt matrix source", source_ref: "isrc.i2c0", interrupt_ref: "irq.ets_i2c_ext0_intr_source", controller_ref: "block.interrupt_matrix0", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "matrix", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_I2C0_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_I2C0_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_I2C0_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.i2c0.scl.gpio0", name: "I2C0 SCL on GPIO0", pin_ref: "pin.gpio0", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio1", name: "I2C0 SCL on GPIO1", pin_ref: "pin.gpio1", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio2", name: "I2C0 SCL on GPIO2", pin_ref: "pin.gpio2", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio3", name: "I2C0 SCL on GPIO3", pin_ref: "pin.gpio3", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio4", name: "I2C0 SCL on GPIO4", pin_ref: "pin.gpio4", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio5", name: "I2C0 SCL on GPIO5", pin_ref: "pin.gpio5", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio6", name: "I2C0 SCL on GPIO6", pin_ref: "pin.gpio6", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio7", name: "I2C0 SCL on GPIO7", pin_ref: "pin.gpio7", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio8", name: "I2C0 SCL on GPIO8", pin_ref: "pin.gpio8", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio9", name: "I2C0 SCL on GPIO9", pin_ref: "pin.gpio9", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio10", name: "I2C0 SCL on GPIO10", pin_ref: "pin.gpio10", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio11", name: "I2C0 SCL on GPIO11", pin_ref: "pin.gpio11", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio12", name: "I2C0 SCL on GPIO12", pin_ref: "pin.gpio12", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio13", name: "I2C0 SCL on GPIO13", pin_ref: "pin.gpio13", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio14", name: "I2C0 SCL on GPIO14", pin_ref: "pin.gpio14", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio15", name: "I2C0 SCL on GPIO15", pin_ref: "pin.gpio15", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio16", name: "I2C0 SCL on GPIO16", pin_ref: "pin.gpio16", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio17", name: "I2C0 SCL on GPIO17", pin_ref: "pin.gpio17", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio18", name: "I2C0 SCL on GPIO18", pin_ref: "pin.gpio18", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio19", name: "I2C0 SCL on GPIO19", pin_ref: "pin.gpio19", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio20", name: "I2C0 SCL on GPIO20", pin_ref: "pin.gpio20", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.scl.gpio21", name: "I2C0 SCL on GPIO21", pin_ref: "pin.gpio21", peripheral_ref: "per.i2c0", signal: "SCL", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_I2C0_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.i2c0.sda.gpio0", name: "I2C0 SDA on GPIO0", pin_ref: "pin.gpio0", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio1", name: "I2C0 SDA on GPIO1", pin_ref: "pin.gpio1", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio2", name: "I2C0 SDA on GPIO2", pin_ref: "pin.gpio2", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio3", name: "I2C0 SDA on GPIO3", pin_ref: "pin.gpio3", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio4", name: "I2C0 SDA on GPIO4", pin_ref: "pin.gpio4", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio5", name: "I2C0 SDA on GPIO5", pin_ref: "pin.gpio5", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio6", name: "I2C0 SDA on GPIO6", pin_ref: "pin.gpio6", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio7", name: "I2C0 SDA on GPIO7", pin_ref: "pin.gpio7", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio8", name: "I2C0 SDA on GPIO8", pin_ref: "pin.gpio8", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio9", name: "I2C0 SDA on GPIO9", pin_ref: "pin.gpio9", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio10", name: "I2C0 SDA on GPIO10", pin_ref: "pin.gpio10", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio11", name: "I2C0 SDA on GPIO11", pin_ref: "pin.gpio11", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio12", name: "I2C0 SDA on GPIO12", pin_ref: "pin.gpio12", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio13", name: "I2C0 SDA on GPIO13", pin_ref: "pin.gpio13", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio14", name: "I2C0 SDA on GPIO14", pin_ref: "pin.gpio14", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio15", name: "I2C0 SDA on GPIO15", pin_ref: "pin.gpio15", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio16", name: "I2C0 SDA on GPIO16", pin_ref: "pin.gpio16", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio17", name: "I2C0 SDA on GPIO17", pin_ref: "pin.gpio17", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio18", name: "I2C0 SDA on GPIO18", pin_ref: "pin.gpio18", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio19", name: "I2C0 SDA on GPIO19", pin_ref: "pin.gpio19", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio20", name: "I2C0 SDA on GPIO20", pin_ref: "pin.gpio20", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }, metadata::PinRoute { id: "pinroute.i2c0.sda.gpio21", name: "I2C0 SDA on GPIO21", pin_ref: "pin.gpio21", peripheral_ref: "per.i2c0", signal: "SDA", route_type: "matrix", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: None }];
pub const DRV_I2C0_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "scl", signal: "SCL", routes: DRV_I2C0_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "sda", signal: "SDA", routes: DRV_I2C0_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_I2C0_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.i2c0.configure_controller", name: "Configure I2C0 controller mode", description: None, kind: Some("configuration"), target_refs: &["per.i2c0"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.i2c0.ctr"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set MS_MODE" }), value: None, description: Some("Place I2C0 into master mode.") }, metadata::SemanticOperationStep { index: 1, action: "write", target_ref: Some("reg.i2c0.ctr"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CLK_EN" }), value: None, description: Some("Enable the I2C controller clock.") }, metadata::SemanticOperationStep { index: 2, action: "write", target_ref: Some("reg.i2c0.ctr"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set CONF_UPGATE" }), value: None, description: Some("Commit the controller configuration to hardware.") }], preconditions: &[], postconditions: &[] }, metadata::SemanticOperation { id: "op.i2c0.start_transaction", name: "Start I2C0 transaction", description: None, kind: Some("transaction"), target_refs: &["per.i2c0"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.i2c0.ctr"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set TRANS_START" }), value: None, description: Some("Start the programmed I2C transaction.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_I2C0_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_I2C0_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct I2c0Resources {
    pub clocks: &'static [metadata::ClockBinding],
    pub resets: &'static [metadata::ResetBinding],
    pub interrupt_sources: &'static [metadata::InterruptSource],
    pub interrupts: &'static [metadata::InterruptRoute],
    pub dma_channels: &'static [metadata::DmaChannel],
    pub dma: &'static [metadata::DmaRoute],
    pub pins: &'static [metadata::PinRole],
    pub init_operations: &'static [metadata::SemanticOperation],
    pub state_machines: &'static [metadata::SemanticStateMachine],
    pub capability_tags: &'static [&'static str],
}

pub const DRV_I2C0_RESOURCES: I2c0Resources = I2c0Resources {
    clocks: DRV_I2C0_CLOCK_BINDINGS,
    resets: DRV_I2C0_RESET_BINDINGS,
    interrupt_sources: DRV_I2C0_INTERRUPT_SOURCES,
    interrupts: DRV_I2C0_INTERRUPT_ROUTES,
    dma_channels: DRV_I2C0_DMA_CHANNELS,
    dma: DRV_I2C0_DMA_ROUTES,
    pins: DRV_I2C0_PIN_ROLES,
    init_operations: DRV_I2C0_INIT_OPERATIONS,
    state_machines: DRV_I2C0_STATE_MACHINES,
    capability_tags: DRV_I2C0_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct I2c0 {
    resources: I2c0Resources,
}

impl I2c0 {
    pub fn new(resources: I2c0Resources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> I2c0Resources {
        self.resources
    }
    /// Enable the I2C0 clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Disable the I2C0 clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for I2C0.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000080u32, 0x00000080u32)?;
        Ok(())
    }

    /// Release reset for I2C0.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00000080u32, 0x00000000u32)?;
        Ok(())
    }

    pub fn apply_configure_controller(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60013004u64, 0x00000010u32, 0x00000010u32)?;
        modify_u32(0x60013004u64, 0x00000100u32, 0x00000100u32)?;
        modify_u32(0x60013004u64, 0x00000800u32, 0x00000800u32)?;
        Ok(())
    }

    pub fn apply_start_transaction(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60013004u64, 0x00000020u32, 0x00000020u32)?;
        Ok(())
    }


}

