//! Generated Embassy-style HAL metadata for Espressif ESP32-C3FN4.

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
    pub executor_idle_strategy: Option<&'static str>,
    pub feature_flags: &'static [&'static str],
    pub provenance_sources: &'static [ProvenanceSource],
    pub provenance_evidence: &'static [ProvenanceEvidence],
}

pub const GENERATED_PROVENANCE_SOURCE_IDS: &[&str] = &[
    "espressif-esp32-c3-datasheet",
    "espressif-esp32-c3-trm",
    "espressif-esp32-c3-errata",
    "espressif-esp-idf-esp32c3",
    "espressif-esp32c3-svd",
];
pub const GENERATED_PROVENANCE_EVIDENCE_IDS: &[&str] = &[
    "e_variant_identity",
    "e_cpu_summary",
    "e_interrupt_matrix_summary",
    "e_systimer_summary",
    "e_memory_summary",
    "e_pin_overview",
    "e_iomux_functions",
    "e_peripheral_pin_assignment",
    "e_analog_functions",
    "e_sdk_memory_ranges",
    "e_sdk_interrupt_sources",
    "e_system_clock_reset_bits",
    "e_svd_register_model",
];

pub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {
    document_title: "Espressif ESP32-C3FN4",
    document_version: "0.1.0",
    device_name: "ESP32-C3FN4",
    target_architecture: Some("riscv32imc"),
    executor_idle_strategy: None,
    feature_flags: &["esp32c3fn4", "executable-profile", "dma"],
    provenance_sources: &[
        ProvenanceSource {
            id: "espressif-esp32-c3-datasheet",
            name: "ESP32-C3 Series Datasheet v2.4",
            kind: Some("datasheet"),
            path: None,
        },
        ProvenanceSource {
            id: "espressif-esp32-c3-trm",
            name: "ESP32-C3 Technical Reference Manual v1.4",
            kind: Some("reference-manual"),
            path: None,
        },
        ProvenanceSource {
            id: "espressif-esp32-c3-errata",
            name: "ESP32-C3 Chip Errata",
            kind: Some("errata"),
            path: None,
        },
        ProvenanceSource {
            id: "espressif-esp-idf-esp32c3",
            name: "ESP-IDF esp32c3 headers",
            kind: Some("sdk"),
            path: None,
        },
        ProvenanceSource {
            id: "espressif-esp32c3-svd",
            name: "esp32c3.svd",
            kind: Some("svd"),
            path: None,
        },
    ],
    provenance_evidence: &[
        ProvenanceEvidence {
            id: "e_variant_identity",
            name: "ESP32-C3FN4 variant identity",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "The ESP32-C3 datasheet v2.4 includes the concrete ESP32-C3FN4 variant with 4 MB in-package flash, QFN32 5x5 mm package, 22 GPIO, and chip revision v0.4.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.95f64),
            locator: Some("section=1.2 Comparison"),
        },
        ProvenanceEvidence {
            id: "e_cpu_summary",
            name: "CPU and interrupt summary",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "ESP32-C3 has a 32-bit single-core RV32IMC RISC-V CPU up to 160 MHz with up to 32 vectored interrupts at seven priority levels.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.93f64),
            locator: Some("section=4.1.1.1 High-Performance CPU"),
        },
        ProvenanceEvidence {
            id: "e_interrupt_matrix_summary",
            name: "Interrupt matrix summary",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "The ESP32-C3 interrupt matrix accepts 62 peripheral interrupt sources and generates 31 CPU peripheral interrupts.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.93f64),
            locator: Some("section=4.1.3.4 Interrupt Matrix"),
        },
        ProvenanceEvidence {
            id: "e_systimer_summary",
            name: "System timer summary",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "The system timer has two 52-bit counters, three comparators, and a fixed 16 MHz counter clock.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.93f64),
            locator: Some("section=4.1.3.5 System Timer"),
        },
        ProvenanceEvidence {
            id: "e_memory_summary",
            name: "Integrated memory summary",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "ESP32-C3 integrates 384 KB ROM, 400 KB SRAM of which 16 KB is cache, 8 KB RTC FAST SRAM, and 4 Kbit eFuse.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.94f64),
            locator: Some("section=4.1.2.1 Internal Memory"),
        },
        ProvenanceEvidence {
            id: "e_pin_overview",
            name: "QFN32 pin overview",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "Table 2-1 defines the QFN32 package pad naming, power domains, reset defaults, and distinguishes GPIO0-21 plus dedicated analog and power pins.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.94f64),
            locator: Some("section=2.2 Pin Overview"),
        },
        ProvenanceEvidence {
            id: "e_iomux_functions",
            name: "IO MUX pin functions",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "Table 2-4 defines direct IO MUX function sets for GPIO0-21 including UART0 and SPI2 fixed routes and JTAG pin defaults.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.94f64),
            locator: Some("section=2.3.1 IO MUX Functions"),
        },
        ProvenanceEvidence {
            id: "e_peripheral_pin_assignment",
            name: "Peripheral pin assignment priorities",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "Table 2-7 states that UART0, SPI0/1, and SPI2 also remain reachable through the GPIO matrix and classifies safe versus restricted GPIO candidates for UART1, I2C, I2S, TWAI, LED PWM, and RMT.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.92f64),
            locator: Some("section=2.3.4 Peripheral Pin Assignment"),
        },
        ProvenanceEvidence {
            id: "e_analog_functions",
            name: "Analog-capable GPIO summary",
            source_ref: "espressif-esp32-c3-datasheet",
            normalized_claim: Some(
                "GPIO0-4 provide ADC1 channels, GPIO5 provides ADC2_CH0, GPIO18 and GPIO19 default to USB_D-/USB_D+, and GPIO0/GPIO1 also expose XTAL_32K functions.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.93f64),
            locator: Some("section=2.3.2 Analog Functions"),
        },
        ProvenanceEvidence {
            id: "e_sdk_memory_ranges",
            name: "ESP-IDF memory map macros",
            source_ref: "espressif-esp-idf-esp32c3",
            normalized_claim: Some(
                "Official ESP-IDF headers define the DRAM0, IRAM0, cached flash views, RTC memory window, and peripheral window used by ESP32-C3 software.",
            ),
            extraction_method: Some("parser"),
            confidence: Some(0.98f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_sdk_interrupt_sources",
            name: "Interrupt source enumeration",
            source_ref: "espressif-esp-idf-esp32c3",
            normalized_claim: Some(
                "interrupts.h enumerates the peripheral interrupt source inventory consumed by the ESP32-C3 interrupt matrix.",
            ),
            extraction_method: Some("parser"),
            confidence: Some(0.98f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_system_clock_reset_bits",
            name: "SYSTEM clock/reset gating fields",
            source_ref: "espressif-esp32c3-svd",
            normalized_claim: Some(
                "The official SVD names per-peripheral clock-gating and reset fields for UART, SPI2, I2C_EXT0, APB_SARADC, SYSTIMER, DMA, timer groups, and related blocks in SYSTEM.",
            ),
            extraction_method: Some("parser"),
            confidence: Some(0.97f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_svd_register_model",
            name: "Official ESP32-C3 SVD register model",
            source_ref: "espressif-esp32c3-svd",
            normalized_claim: Some(
                "The official esp32c3.svd provides the primary machine-readable structural scaffold for 37 peripherals, their registers, fields, base addresses, and declared peripheral interrupt links.",
            ),
            extraction_method: Some("imported"),
            confidence: Some(0.95f64),
            locator: None,
        },
    ],
};
