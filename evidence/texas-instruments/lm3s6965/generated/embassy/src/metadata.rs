//! Generated Embassy-style HAL metadata for Texas Instruments LM3S6965.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceRequirement {
    Required,
    Optional,
    MutuallyExclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unsupported(&'static str),
    InvalidReference(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueLiteral {
    Integer(i64),
    Unsigned(u64),
    Number(f64),
    String(&'static str),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticExpression {
    pub language: Option<&'static str>,
    pub text: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Predicate {
    pub kind: &'static str,
    pub target_ref: Option<&'static str>,
    pub expression: Option<SemanticExpression>,
    pub expected_value: Option<ValueLiteral>,
    pub description: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticOperationStep {
    pub index: u32,
    pub action: &'static str,
    pub target_ref: Option<&'static str>,
    pub expression: Option<SemanticExpression>,
    pub value: Option<ValueLiteral>,
    pub description: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticOperation {
    pub id: &'static str,
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub kind: Option<&'static str>,
    pub target_refs: &'static [&'static str],
    pub steps: &'static [SemanticOperationStep],
    pub preconditions: &'static [Predicate],
    pub postconditions: &'static [Predicate],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticState {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub invariants: &'static [Predicate],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticSideEffect {
    pub kind: &'static str,
    pub target_ref: Option<&'static str>,
    pub description: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticTransition {
    pub from: &'static str,
    pub to: &'static str,
    pub trigger: Option<&'static str>,
    pub conditions: &'static [Predicate],
    pub effects: &'static [SemanticSideEffect],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticStateMachine {
    pub id: &'static str,
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub target_refs: &'static [&'static str],
    pub initial_state: Option<&'static str>,
    pub states: &'static [SemanticState],
    pub transitions: &'static [SemanticTransition],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockBinding {
    pub id: &'static str,
    pub name: &'static str,
    pub consumer_ref: &'static str,
    pub clock_ref: &'static str,
    pub controller_ref: Option<&'static str>,
    pub binding_kind: &'static str,
    pub control_refs: &'static [&'static str],
    pub enable_operation_refs: &'static [&'static str],
    pub disable_operation_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetBinding {
    pub id: &'static str,
    pub name: &'static str,
    pub target_ref: &'static str,
    pub controller_ref: Option<&'static str>,
    pub reset_domain_ref: Option<&'static str>,
    pub binding_kind: &'static str,
    pub control_refs: &'static [&'static str],
    pub assert_operation_refs: &'static [&'static str],
    pub release_operation_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptSource {
    pub id: &'static str,
    pub name: &'static str,
    pub source_ref: &'static str,
    pub producer_ref: Option<&'static str>,
    pub kind: &'static str,
    pub flag_refs: &'static [&'static str],
    pub clear_operation_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptRoute {
    pub id: &'static str,
    pub name: &'static str,
    pub source_ref: &'static str,
    pub interrupt_ref: &'static str,
    pub controller_ref: &'static str,
    pub cpu_target_ref: Option<&'static str>,
    pub line_index: Option<u32>,
    pub route_type: &'static str,
    pub control_refs: &'static [&'static str],
    pub acknowledge_operation_refs: &'static [&'static str],
    pub shared_group: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaChannel {
    pub id: &'static str,
    pub name: &'static str,
    pub controller_ref: &'static str,
    pub target_ref: Option<&'static str>,
    pub channel_index: u32,
    pub capabilities: &'static [&'static str],
    pub priority_levels: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaRoute {
    pub id: &'static str,
    pub name: &'static str,
    pub peripheral_ref: &'static str,
    pub signal: Option<&'static str>,
    pub channel_ref: &'static str,
    pub direction: &'static str,
    pub control_refs: &'static [&'static str],
    pub shared_channel_group_ref: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinRoute {
    pub id: &'static str,
    pub name: &'static str,
    pub pin_ref: &'static str,
    pub peripheral_ref: &'static str,
    pub signal: &'static str,
    pub route_type: &'static str,
    pub control_refs: &'static [&'static str],
    pub electrical_constraint_refs: &'static [&'static str],
    pub conflict_refs: &'static [&'static str],
    pub default_after_reset: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinRole {
    pub role: &'static str,
    pub signal: &'static str,
    pub routes: &'static [PinRoute],
    pub requirement: ResourceRequirement,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProvenanceSource {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: Option<&'static str>,
    pub path: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProvenanceEvidence {
    pub id: &'static str,
    pub name: &'static str,
    pub source_ref: &'static str,
    pub normalized_claim: Option<&'static str>,
    pub extraction_method: Option<&'static str>,
    pub confidence: Option<f64>,
    pub locator: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleProvenance {
    pub module_name: &'static str,
    pub document_title: &'static str,
    pub document_version: &'static str,
    pub source_ids: &'static [&'static str],
    pub evidence_ids: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedMetadata {
    pub document_title: &'static str,
    pub document_version: &'static str,
    pub device_name: &'static str,
    pub target_architecture: Option<&'static str>,
    pub feature_flags: &'static [&'static str],
    pub provenance_sources: &'static [ProvenanceSource],
    pub provenance_evidence: &'static [ProvenanceEvidence],
}

pub const GENERATED_PROVENANCE_SOURCE_IDS: &[&str] = &["ti-lm3s6965-product-page", "ti-lm3s6965-datasheet", "arm-cortex-m3-trm", "ti-sw-drl", "ti-stellaris-driverlib-guide", "ti-lm3s6965evb-user-guide", "ti-lm3s-class-errata", "cmsis-lm3s6965-header-mirror", "ti-driverlib-mirror-hw-gpio", "ti-driverlib-mirror-hw-sysctl", "ti-driverlib-mirror-hw-uart", "ti-driverlib-mirror-hw-ssi", "ti-driverlib-mirror-hw-i2c", "ti-driverlib-mirror-hw-timer", "ti-driverlib-mirror-hw-watchdog", "ti-driverlib-mirror-hw-nvic", "qemu-stellaris-docs"];
pub const GENERATED_PROVENANCE_EVIDENCE_IDS: &[&str] = &["e_target_identity", "e_cpu_core", "e_irq_inventory", "e_memory_sizes", "e_memory_map", "e_systick_offsets", "e_register_models", "e_gpio_offsets", "e_uart_offsets", "e_ssi_offsets", "e_i2c_offsets", "e_timer_offsets", "e_watchdog_offsets", "e_sysctl_offsets", "e_pin_table", "e_timer_semantics", "e_timer_driverlib", "e_clock_enable_semantics", "e_package_type"];

pub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {
    document_title: "Texas Instruments LM3S6965",
    document_version: "0.1.0",
    device_name: "LM3S6965",
    target_architecture: Some("thumbv7m-none-eabi"),
    feature_flags: &["gpio", "interrupt", "rcc", "uart", "spi", "i2c", "timer"],
    provenance_sources: &[ProvenanceSource { id: "ti-lm3s6965-product-page", name: "ti-lm3s6965-product-page", kind: Some("other"), path: None }, ProvenanceSource { id: "ti-lm3s6965-datasheet", name: "ti-lm3s6965-datasheet", kind: Some("datasheet"), path: None }, ProvenanceSource { id: "arm-cortex-m3-trm", name: "arm-cortex-m3-trm", kind: Some("reference-manual"), path: None }, ProvenanceSource { id: "ti-sw-drl", name: "ti-sw-drl", kind: Some("sdk"), path: None }, ProvenanceSource { id: "ti-stellaris-driverlib-guide", name: "ti-stellaris-driverlib-guide", kind: Some("other"), path: None }, ProvenanceSource { id: "ti-lm3s6965evb-user-guide", name: "ti-lm3s6965evb-user-guide", kind: Some("other"), path: None }, ProvenanceSource { id: "ti-lm3s-class-errata", name: "ti-lm3s-class-errata", kind: Some("errata"), path: None }, ProvenanceSource { id: "cmsis-lm3s6965-header-mirror", name: "cmsis-lm3s6965-header-mirror", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-gpio", name: "ti-driverlib-mirror-hw-gpio", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-sysctl", name: "ti-driverlib-mirror-hw-sysctl", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-uart", name: "ti-driverlib-mirror-hw-uart", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-ssi", name: "ti-driverlib-mirror-hw-ssi", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-i2c", name: "ti-driverlib-mirror-hw-i2c", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-timer", name: "ti-driverlib-mirror-hw-timer", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-watchdog", name: "ti-driverlib-mirror-hw-watchdog", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "ti-driverlib-mirror-hw-nvic", name: "ti-driverlib-mirror-hw-nvic", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "qemu-stellaris-docs", name: "qemu-stellaris-docs", kind: Some("other"), path: None }],
    provenance_evidence: &[ProvenanceEvidence { id: "e_target_identity", name: "Exact target identity", source_ref: "ti-lm3s6965-datasheet", normalized_claim: Some("The device is the Texas Instruments Stellaris LM3S6965 Cortex-M3 microcontroller used by the lm3s6965evb board context."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("page=45") }, ProvenanceEvidence { id: "e_cpu_core", name: "Cortex-M3 core facts", source_ref: "cmsis-lm3s6965-header-mirror", normalized_claim: Some("The device header defines Cortex-M3 revision 0x0202, MPU present, 3 NVIC priority bits, and standard SysTick configuration."), extraction_method: Some("imported"), confidence: Some(0.98f64), locator: Some("lines=103-107") }, ProvenanceEvidence { id: "e_irq_inventory", name: "Device IRQ inventory", source_ref: "cmsis-lm3s6965-header-mirror", normalized_claim: Some("The LM3S6965 device header enumerates the device IRQ inventory including GPIOA-F, UART0/1, SSI0, I2C0, WATCHDOG0, TIMER0-3A/B, SYSCTL, FLASH_CTRL, and SysTick."), extraction_method: Some("parser"), confidence: Some(0.98f64), locator: Some("lines=36-91") }, ProvenanceEvidence { id: "e_memory_sizes", name: "Flash and SRAM capacities", source_ref: "ti-lm3s6965-datasheet", normalized_claim: Some("LM3S6965 provides 256KB flash and 64KB SRAM."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("page=45") }, ProvenanceEvidence { id: "e_memory_map", name: "Core memory map slices", source_ref: "ti-lm3s6965-datasheet", normalized_claim: Some("The device memory map places flash at 0x00000000, SRAM at 0x20000000, peripheral space at 0x40000000, and Cortex-M3 private peripherals at 0xE000E000."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("page=72") }, ProvenanceEvidence { id: "e_systick_offsets", name: "SysTick offsets", source_ref: "ti-lm3s6965-datasheet", normalized_claim: Some("SysTick control, reload, and current registers are located at offsets 0x010, 0x014, and 0x018 within the Cortex-M3 private peripheral region."), extraction_method: Some("manual"), confidence: Some(0.94f64), locator: Some("page=103") }, ProvenanceEvidence { id: "e_register_models", name: "CMSIS register blocks", source_ref: "cmsis-lm3s6965-header-mirror", normalized_claim: Some("The LM3S6965 CMSIS header provides register-bearing structural layouts for WATCHDOG0, GPIOA, SSI0, UART0, I2C0, TIMER0, FLASH_CTRL, and SYSCTL, and clones GPIO/UART/TIMER instance types across the family."), extraction_method: Some("parser"), confidence: Some(0.96f64), locator: Some("lines=151-713") }, ProvenanceEvidence { id: "e_gpio_offsets", name: "GPIO offsets and fields", source_ref: "ti-driverlib-mirror-hw-gpio", normalized_claim: Some("TI-derived GPIO hardware header provides register offsets and bit masks for GPIO direction, interrupt, pad configuration, alternate-function select, and digital enable registers."), extraction_method: Some("parser"), confidence: Some(0.92f64), locator: Some("lines=48-151") }, ProvenanceEvidence { id: "e_uart_offsets", name: "UART offsets and fields", source_ref: "ti-driverlib-mirror-hw-uart", normalized_claim: Some("TI-derived UART hardware header provides register offsets and bit masks for UART data, status, baud, line control, control, and interrupt registers."), extraction_method: Some("parser"), confidence: Some(0.92f64), locator: Some("lines=48-518") }, ProvenanceEvidence { id: "e_ssi_offsets", name: "SSI offsets and fields", source_ref: "ti-driverlib-mirror-hw-ssi", normalized_claim: Some("TI-derived SSI hardware header provides register offsets and bit masks for SSI control, data, status, prescale, and interrupt registers."), extraction_method: Some("parser"), confidence: Some(0.92f64), locator: Some("lines=48-240") }, ProvenanceEvidence { id: "e_i2c_offsets", name: "I2C offsets and fields", source_ref: "ti-driverlib-mirror-hw-i2c", normalized_claim: Some("TI-derived I2C hardware header provides register offsets and bit masks for I2C master/slave address, control/status, data, timing, configuration, and interrupt registers."), extraction_method: Some("parser"), confidence: Some(0.92f64), locator: Some("lines=48-485") }, ProvenanceEvidence { id: "e_timer_offsets", name: "GPTM offsets and fields", source_ref: "ti-driverlib-mirror-hw-timer", normalized_claim: Some("TI-derived GPTM hardware header provides register offsets and bit masks for configuration, mode, control, load, match, prescale, counter, and interrupt registers."), extraction_method: Some("parser"), confidence: Some(0.93f64), locator: Some("lines=48-676") }, ProvenanceEvidence { id: "e_watchdog_offsets", name: "Watchdog offsets and fields", source_ref: "ti-driverlib-mirror-hw-watchdog", normalized_claim: Some("TI-derived watchdog hardware header provides register offsets and bit masks for watchdog load, control, interrupt status/clear, test, and lock registers."), extraction_method: Some("parser"), confidence: Some(0.9f64), locator: None }, ProvenanceEvidence { id: "e_sysctl_offsets", name: "System control offsets and fields", source_ref: "ti-driverlib-mirror-hw-sysctl", normalized_claim: Some("TI-derived system control header provides register offsets and gating/reset field masks for GPIOA-F, UART0/1, SSI0, I2C0, TIMER0-3, and WDT0 along with RCC and RCC2 fields."), extraction_method: Some("parser"), confidence: Some(0.94f64), locator: Some("lines=62-1407") }, ProvenanceEvidence { id: "e_pin_table", name: "Requested peripheral pin routes", source_ref: "ti-lm3s6965-datasheet", normalized_claim: Some("The datasheet pin tables place UART0 on PA0/PA1, SSI0 on PA2/PA3/PA4/PA5, I2C0 on PB2/PB3, UART1 on PD2/PD3, and CCP0-3 on PD4/PD7/PD5/PC6 for the 100-pin LQFP package."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("pages=688-690") }, ProvenanceEvidence { id: "e_timer_semantics", name: "Basic timer modes", source_ref: "ti-lm3s6965-datasheet", normalized_claim: Some("Each GPTM supports 32-bit and split 16-bit one-shot and periodic timers, along with capture, PWM, and RTC modes; LM3S6965 timer timeout behavior is explicit for one-shot and periodic down-counter operation."), extraction_method: Some("manual"), confidence: Some(0.94f64), locator: Some("pages=334-345") }, ProvenanceEvidence { id: "e_timer_driverlib", name: "Timer API semantics", source_ref: "ti-stellaris-driverlib-guide", normalized_claim: Some("The Driver Library guide documents GPTM configure, enable/disable, interval load, and interrupt behavior and notes that some up-count variants are not available on all parts."), extraction_method: Some("manual"), confidence: Some(0.9f64), locator: Some("pages=317-336") }, ProvenanceEvidence { id: "e_clock_enable_semantics", name: "Clock enable timing caveat", source_ref: "ti-stellaris-driverlib-guide", normalized_claim: Some("Software must allow the peripheral clock gate to propagate before first access after peripheral enable."), extraction_method: Some("manual"), confidence: Some(0.85f64), locator: Some("pages=279-310") }, ProvenanceEvidence { id: "e_package_type", name: "Package type", source_ref: "ti-lm3s6965-datasheet", normalized_claim: Some("LM3S6965 is offered in a 100-pin RoHS-compliant LQFP package in the referenced datasheet package tables."), extraction_method: Some("manual"), confidence: Some(0.93f64), locator: Some("page=45") }],
};
