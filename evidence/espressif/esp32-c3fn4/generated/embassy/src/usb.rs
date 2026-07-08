//! Generated Embassy-style usb module for ESP32-C3FN4.

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
    module_name: "usb",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: UsbSerialJtag (usb-device) from canonical block block.usb_device -> usb-device
pub const DRV_USB_DEVICE_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[metadata::ClockBinding { id: "clkbind.usb_device", name: "USB_DEVICE_CLK_EN", consumer_ref: "per.usb_device", clock_ref: "clk.apb", controller_ref: Some("block.system"), binding_kind: "gated", control_refs: &["reg.system.perip_clk_en0"], enable_operation_refs: &[], disable_operation_refs: &[] }];
pub const DRV_USB_DEVICE_RESET_BINDINGS: &[metadata::ResetBinding] = &[metadata::ResetBinding { id: "rstbind.usb_device", name: "USB_DEVICE_RST", target_ref: "per.usb_device", controller_ref: Some("block.system"), reset_domain_ref: Some("rst.system"), binding_kind: "local", control_refs: &["reg.system.perip_rst_en0"], assert_operation_refs: &[], release_operation_refs: &[] }];
pub const DRV_USB_DEVICE_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[metadata::InterruptSource { id: "isrc.usb_device", name: "USB_SERIAL_JTAG", source_ref: "per.usb_device", producer_ref: Some("block.usb_device"), kind: "peripheral", flag_refs: &[], clear_operation_refs: &[] }];
pub const DRV_USB_DEVICE_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[metadata::InterruptRoute { id: "iroute.usb_device", name: "USB Serial/JTAG interrupt matrix source", source_ref: "isrc.usb_device", interrupt_ref: "irq.ets_usb_serial_jtag_intr_source", controller_ref: "block.interrupt_matrix0", cpu_target_ref: Some("block.cpu0"), line_index: None, route_type: "matrix", control_refs: &[], acknowledge_operation_refs: &[], shared_group: None }];
pub const DRV_USB_DEVICE_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_USB_DEVICE_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_USB_DEVICE_PIN_ROLE_0_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usb.dm.gpio18", name: "USB D- on GPIO18", pin_ref: "pin.gpio18", peripheral_ref: "per.usb_device", signal: "USB_D-", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_USB_DEVICE_PIN_ROLE_1_ROUTES: &[metadata::PinRoute] = &[metadata::PinRoute { id: "pinroute.usb.dp.gpio19", name: "USB D+ on GPIO19", pin_ref: "pin.gpio19", peripheral_ref: "per.usb_device", signal: "USB_D+", route_type: "hardwired", control_refs: &[], electrical_constraint_refs: &[], conflict_refs: &[], default_after_reset: Some(true) }];
pub const DRV_USB_DEVICE_PIN_ROLES: &[metadata::PinRole] = &[metadata::PinRole { role: "dm", signal: "USB_D-", routes: DRV_USB_DEVICE_PIN_ROLE_0_ROUTES, requirement: metadata::ResourceRequirement::Required }, metadata::PinRole { role: "dp", signal: "USB_D+", routes: DRV_USB_DEVICE_PIN_ROLE_1_ROUTES, requirement: metadata::ResourceRequirement::Required }];
pub const DRV_USB_DEVICE_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[metadata::SemanticOperation { id: "op.usb_device.attach_serial_jtag", name: "Attach USB Serial/JTAG pads", description: None, kind: Some("initialization"), target_refs: &["per.usb_device"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.usb_device.conf0"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set PHY_SEL" }), value: None, description: Some("Select the on-chip USB device PHY.") }, metadata::SemanticOperationStep { index: 1, action: "write", target_ref: Some("reg.usb_device.conf0"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set PAD_PULL_OVERRIDE" }), value: None, description: Some("Let software drive the D+ and D- pull resistors for attach signaling.") }, metadata::SemanticOperationStep { index: 2, action: "write", target_ref: Some("reg.usb_device.conf0"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Clear DP_PULLDOWN" }), value: None, description: Some("Release the D+ pull-down before advertising the device.") }, metadata::SemanticOperationStep { index: 3, action: "write", target_ref: Some("reg.usb_device.conf0"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Clear DM_PULLDOWN" }), value: None, description: Some("Release the D- pull-down before advertising the device.") }, metadata::SemanticOperationStep { index: 4, action: "write", target_ref: Some("reg.usb_device.conf0"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set USB_PAD_ENABLE" }), value: None, description: Some("Enable the USB pad drivers on GPIO18 and GPIO19.") }, metadata::SemanticOperationStep { index: 5, action: "write", target_ref: Some("reg.usb_device.conf0"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set DP_PULLUP" }), value: None, description: Some("Advertise a full-speed USB device attach on D+.") }], preconditions: &[], postconditions: &[] }, metadata::SemanticOperation { id: "op.usb_device.complete_serial_in_packet", name: "Commit USB Serial/JTAG IN packet", description: None, kind: Some("transaction"), target_refs: &["per.usb_device"], steps: &[metadata::SemanticOperationStep { index: 0, action: "write", target_ref: Some("reg.usb_device.ep1_conf"), expression: Some(metadata::SemanticExpression { language: Some("plain"), text: "Set WR_DONE" }), value: None, description: Some("Hand the queued Serial IN bytes to the host-visible endpoint.") }], preconditions: &[], postconditions: &[] }];
pub const DRV_USB_DEVICE_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_USB_DEVICE_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct UsbSerialJtagResources {
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

pub const DRV_USB_DEVICE_RESOURCES: UsbSerialJtagResources = UsbSerialJtagResources {
    clocks: DRV_USB_DEVICE_CLOCK_BINDINGS,
    resets: DRV_USB_DEVICE_RESET_BINDINGS,
    interrupt_sources: DRV_USB_DEVICE_INTERRUPT_SOURCES,
    interrupts: DRV_USB_DEVICE_INTERRUPT_ROUTES,
    dma_channels: DRV_USB_DEVICE_DMA_CHANNELS,
    dma: DRV_USB_DEVICE_DMA_ROUTES,
    pins: DRV_USB_DEVICE_PIN_ROLES,
    init_operations: DRV_USB_DEVICE_INIT_OPERATIONS,
    state_machines: DRV_USB_DEVICE_STATE_MACHINES,
    capability_tags: DRV_USB_DEVICE_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct UsbSerialJtag {
    resources: UsbSerialJtagResources,
}

impl UsbSerialJtag {
    pub fn new(resources: UsbSerialJtagResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> UsbSerialJtagResources {
        self.resources
    }
    /// Enable the USB_DEVICE clock gate.
    pub fn enable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00800000u32, 0x00800000u32)?;
        Ok(())
    }

    /// Disable the USB_DEVICE clock gate.
    pub fn disable_clock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0010u64, 0x00800000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Assert reset for USB_DEVICE.
    pub fn assert_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00800000u32, 0x00800000u32)?;
        Ok(())
    }

    /// Release reset for USB_DEVICE.
    pub fn release_reset(&self) -> Result<(), metadata::Error> {
        modify_u32(0x600C0018u64, 0x00800000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the UsbSerialJtag USB pad drivers.
    pub fn enable_usb_pad(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043018u64, 0x00004000u32, 0x00004000u32)?;
        Ok(())
    }

    /// Disable the UsbSerialJtag USB pad drivers.
    pub fn disable_usb_pad(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043018u64, 0x00004000u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the UsbSerialJtag D+ pull-up.
    pub fn enable_dp_pullup(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043018u64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    /// Disable the UsbSerialJtag D+ pull-up.
    pub fn disable_dp_pullup(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043018u64, 0x00000200u32, 0x00000000u32)?;
        Ok(())
    }

    /// Let UsbSerialJtag pad pull control follow software override bits.
    pub fn enable_pad_pull_override(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043018u64, 0x00000100u32, 0x00000100u32)?;
        Ok(())
    }

    /// Release UsbSerialJtag pad pull control back to hardware defaults.
    pub fn disable_pad_pull_override(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043018u64, 0x00000100u32, 0x00000000u32)?;
        Ok(())
    }

    /// Return whether the UsbSerialJtag Serial IN endpoint can accept another byte.
    pub fn serial_in_ready(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x60043004u64)? & 0x00000002u32) != 0)
    }

    /// Return whether the UsbSerialJtag Serial OUT endpoint currently holds unread data.
    pub fn serial_out_data_available(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x60043004u64)? & 0x00000004u32) != 0)
    }

    /// Queue one byte for the UsbSerialJtag Serial IN endpoint.
    pub fn write_serial_byte(&self, byte: u8) -> Result<(), metadata::Error> {
        while !self.serial_in_ready()? {}
        write_u32(0x60043000u64, u32::from(byte))?;
        Ok(())
    }

    /// Queue a byte slice for the UsbSerialJtag Serial IN endpoint.
    pub fn write_serial_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        for &byte in bytes {
            self.write_serial_byte(byte)?;
        }
        Ok(())
    }

    /// Read one byte from the UsbSerialJtag Serial OUT endpoint.
    pub fn read_serial_byte(&self) -> Result<u8, metadata::Error> {
        while !self.serial_out_data_available()? {}
        Ok((read_u32(0x60043000u64)? & 0x000000FFu32) as u8)
    }

    /// Read bytes from the UsbSerialJtag Serial OUT endpoint into the supplied buffer.
    pub fn read_serial_bytes(&self, buffer: &mut [u8]) -> Result<(), metadata::Error> {
        for slot in buffer {
            *slot = self.read_serial_byte()?;
        }
        Ok(())
    }

    /// Commit the currently queued UsbSerialJtag Serial IN packet to the host-facing endpoint.
    pub fn complete_write_packet(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043004u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }

    /// Queue a packet and commit it on the UsbSerialJtag Serial IN endpoint.
    pub fn write_serial_packet(&self, bytes: &[u8]) -> Result<(), metadata::Error> {
        self.write_serial_bytes(bytes)?;
        self.complete_write_packet()
    }

    /// Enable the UsbSerialJtag Serial OUT packet-received interrupt.
    pub fn enable_serial_out_receive_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043010u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Disable the UsbSerialJtag Serial OUT packet-received interrupt.
    pub fn disable_serial_out_receive_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043010u64, 0x00000004u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the UsbSerialJtag Serial IN empty interrupt.
    pub fn enable_serial_in_empty_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043010u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Disable the UsbSerialJtag Serial IN empty interrupt.
    pub fn disable_serial_in_empty_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043010u64, 0x00000008u32, 0x00000000u32)?;
        Ok(())
    }

    /// Enable the UsbSerialJtag USB bus-reset interrupt.
    pub fn enable_usb_bus_reset_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043010u64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    /// Disable the UsbSerialJtag USB bus-reset interrupt.
    pub fn disable_usb_bus_reset_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043010u64, 0x00000200u32, 0x00000000u32)?;
        Ok(())
    }

    /// Clear the UsbSerialJtag Serial OUT packet-received interrupt.
    pub fn clear_serial_out_receive_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043014u64, 0x00000004u32, 0x00000004u32)?;
        Ok(())
    }

    /// Clear the UsbSerialJtag Serial IN empty interrupt.
    pub fn clear_serial_in_empty_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043014u64, 0x00000008u32, 0x00000008u32)?;
        Ok(())
    }

    /// Clear the UsbSerialJtag USB bus-reset interrupt.
    pub fn clear_usb_bus_reset_interrupt(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043014u64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    /// Return whether the UsbSerialJtag USB bus-reset interrupt is currently pending.
    pub fn is_usb_bus_reset_pending(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x6004300Cu64)? & 0x00000200u32) != 0)
    }

    pub fn apply_attach_serial_jtag(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043018u64, 0x00000001u32, 0x00000001u32)?;
        modify_u32(0x60043018u64, 0x00000100u32, 0x00000100u32)?;
        modify_u32(0x60043018u64, 0x00000400u32, 0x00000000u32)?;
        modify_u32(0x60043018u64, 0x00001000u32, 0x00000000u32)?;
        modify_u32(0x60043018u64, 0x00004000u32, 0x00004000u32)?;
        modify_u32(0x60043018u64, 0x00000200u32, 0x00000200u32)?;
        Ok(())
    }

    pub fn apply_complete_serial_in_packet(&self) -> Result<(), metadata::Error> {
        modify_u32(0x60043004u64, 0x00000001u32, 0x00000001u32)?;
        Ok(())
    }


}

