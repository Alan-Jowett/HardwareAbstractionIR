//! Generated Embassy-style watchdog module for CH32V203G6U6.

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
    module_name: "watchdog",
    document_title: metadata::GENERATED_METADATA.document_title,
    document_version: metadata::GENERATED_METADATA.document_version,
    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,
    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,
};

// Driver instance: IWDG (watchdog) from canonical block block.iwdg -> watchdog
pub const DRV_IWDG_CLOCK_BINDINGS: &[metadata::ClockBinding] = &[];
pub const DRV_IWDG_RESET_BINDINGS: &[metadata::ResetBinding] = &[];
pub const DRV_IWDG_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &[];
pub const DRV_IWDG_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &[];
pub const DRV_IWDG_DMA_CHANNELS: &[metadata::DmaChannel] = &[];
pub const DRV_IWDG_DMA_ROUTES: &[metadata::DmaRoute] = &[];
pub const DRV_IWDG_PIN_ROLES: &[metadata::PinRole] = &[];
pub const DRV_IWDG_INIT_OPERATIONS: &[metadata::SemanticOperation] = &[
    metadata::SemanticOperation {
        id: "op.iwdg.unlock",
        name: "Unlock IWDG configuration",
        description: None,
        kind: Some("configuration"),
        target_refs: &["periph.iwdg"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.iwdg.ctlr"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Write KEY = 0x5555",
            }),
            value: None,
            description: Some(
                "Write the IWDG unlock key so prescaler and reload updates are accepted.",
            ),
        }],
        preconditions: &[],
        postconditions: &[],
    },
    metadata::SemanticOperation {
        id: "op.iwdg.feed",
        name: "Reload IWDG counter",
        description: None,
        kind: Some("mode-transition"),
        target_refs: &["periph.iwdg"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.iwdg.ctlr"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Write KEY = 0xAAAA",
            }),
            value: None,
            description: Some("Write the IWDG reload key to refresh the watchdog counter."),
        }],
        preconditions: &[],
        postconditions: &[],
    },
    metadata::SemanticOperation {
        id: "op.iwdg.start",
        name: "Start IWDG",
        description: None,
        kind: Some("mode-transition"),
        target_refs: &["periph.iwdg"],
        steps: &[metadata::SemanticOperationStep {
            index: 0,
            action: "write",
            target_ref: Some("reg.iwdg.ctlr"),
            expression: Some(metadata::SemanticExpression {
                language: Some("plain"),
                text: "Write KEY = 0xCCCC",
            }),
            value: None,
            description: Some("Write the IWDG start key to enable the watchdog."),
        }],
        preconditions: &[],
        postconditions: &[],
    },
];
pub const DRV_IWDG_STATE_MACHINES: &[metadata::SemanticStateMachine] = &[];
pub const DRV_IWDG_CAPABILITY_TAGS: &[&str] = &[];

#[derive(Debug, Clone, Copy)]
pub struct IWDGResources {
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

pub const DRV_IWDG_RESOURCES: IWDGResources = IWDGResources {
    clocks: DRV_IWDG_CLOCK_BINDINGS,
    resets: DRV_IWDG_RESET_BINDINGS,
    interrupt_sources: DRV_IWDG_INTERRUPT_SOURCES,
    interrupts: DRV_IWDG_INTERRUPT_ROUTES,
    dma_channels: DRV_IWDG_DMA_CHANNELS,
    dma: DRV_IWDG_DMA_ROUTES,
    pins: DRV_IWDG_PIN_ROLES,
    init_operations: DRV_IWDG_INIT_OPERATIONS,
    state_machines: DRV_IWDG_STATE_MACHINES,
    lowering_pattern: None,
    time_driver_source: None,
    capability_tags: DRV_IWDG_CAPABILITY_TAGS,
};

#[derive(Debug, Clone, Copy)]
pub struct IWDG {
    resources: IWDGResources,
}

impl IWDG {
    pub fn new(resources: IWDGResources) -> Result<Self, metadata::Error> {
        Ok(Self { resources })
    }

    pub fn resources(&self) -> IWDGResources {
        self.resources
    }
    pub fn unlock(&self) -> Result<(), metadata::Error> {
        self.apply_unlock()
    }

    pub fn feed_watchdog(&self) -> Result<(), metadata::Error> {
        self.apply_feed()
    }

    pub fn start_watchdog(&self) -> Result<(), metadata::Error> {
        self.apply_start()
    }

    /// Report whether the watchdog prescaler update is still pending.
    pub fn is_prescaler_update_pending(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x4000300Cu64)? & 0x00000001u32) != 0)
    }

    /// Report whether the watchdog reload update is still pending.
    pub fn is_reload_update_pending(&self) -> Result<bool, metadata::Error> {
        Ok((read_u32(0x4000300Cu64)? & 0x00000002u32) != 0)
    }

    pub fn is_configuration_update_pending(&self) -> Result<bool, metadata::Error> {
        Ok(((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0)
    }

    pub fn read_prescaler(&self) -> Result<u8, metadata::Error> {
        Ok(((read_u32(0x40003004u64)?) & 0x00000007u32) as u8)
    }

    pub fn read_reload(&self) -> Result<u16, metadata::Error> {
        Ok(((read_u32(0x40003008u64)?) & 0x00000FFFu32) as u16)
    }

    pub fn set_prescaler(&self, prescaler: u8) -> Result<(), metadata::Error> {
        if prescaler > 0x07u8 {
            return Err(metadata::Error::Unsupported(
                "watchdog prescaler exceeds the modeled field width",
            ));
        }
        while ((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0 {
            core::hint::spin_loop();
        }
        self.apply_unlock()?;
        modify_u32(
            0x40003004u64,
            0x00000007u32,
            (u32::from(prescaler)) & 0x00000007u32,
        )?;
        while ((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0 {
            core::hint::spin_loop();
        }
        Ok(())
    }

    pub fn set_reload(&self, reload: u16) -> Result<(), metadata::Error> {
        if reload > 0x0FFFu16 {
            return Err(metadata::Error::Unsupported(
                "watchdog reload exceeds the modeled field width",
            ));
        }
        while ((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0 {
            core::hint::spin_loop();
        }
        self.apply_unlock()?;
        modify_u32(
            0x40003008u64,
            0x00000FFFu32,
            (u32::from(reload)) & 0x00000FFFu32,
        )?;
        while ((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0 {
            core::hint::spin_loop();
        }
        Ok(())
    }

    pub fn configure(&self, config: IWDGConfig) -> Result<(), metadata::Error> {
        let prescaler = config.prescaler;
        let reload = config.reload;
        if prescaler > 0x07u8 {
            return Err(metadata::Error::Unsupported(
                "watchdog prescaler exceeds the modeled field width",
            ));
        }
        if reload > 0x0FFFu16 {
            return Err(metadata::Error::Unsupported(
                "watchdog reload exceeds the modeled field width",
            ));
        }
        while ((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0 {
            core::hint::spin_loop();
        }
        self.apply_unlock()?;
        modify_u32(
            0x40003004u64,
            0x00000007u32,
            (u32::from(prescaler)) & 0x00000007u32,
        )?;
        while ((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0 {
            core::hint::spin_loop();
        }
        self.apply_unlock()?;
        modify_u32(
            0x40003008u64,
            0x00000FFFu32,
            (u32::from(reload)) & 0x00000FFFu32,
        )?;
        while ((read_u32(0x4000300Cu64)?) & 0x00000003u32) != 0 {
            core::hint::spin_loop();
        }
        Ok(())
    }

    pub fn start_with_config(&self, config: IWDGConfig) -> Result<(), metadata::Error> {
        self.configure(config)?;
        self.apply_start()?;
        Ok(())
    }

    pub fn apply_unlock(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40003000u64, 0x0000FFFFu32, 0x00005555u32)?;
        Ok(())
    }

    pub fn apply_feed(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40003000u64, 0x0000FFFFu32, 0x0000AAAAu32)?;
        Ok(())
    }

    pub fn apply_start(&self) -> Result<(), metadata::Error> {
        modify_u32(0x40003000u64, 0x0000FFFFu32, 0x0000CCCCu32)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IWDGConfig {
    pub prescaler: u8,
    pub reload: u16,
}

impl IWDGConfig {
    pub const fn new(prescaler: u8, reload: u16) -> Self {
        Self { prescaler, reload }
    }
}

impl From<(u8, u16)> for IWDGConfig {
    fn from(value: (u8, u16)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl embedded_hal_02::watchdog::Watchdog for IWDG {
    fn feed(&mut self) {
        self.feed_watchdog().expect("generated watchdog feed")
    }
}

impl embedded_hal_02::watchdog::WatchdogEnable for IWDG {
    type Time = IWDGConfig;

    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        self.start_with_config(period.into())
            .expect("generated watchdog start")
    }
}
