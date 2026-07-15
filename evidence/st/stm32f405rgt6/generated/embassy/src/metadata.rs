//! Generated Embassy-style HAL metadata for ST STM32F405RGT6.

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
    "olimex-stm32-h405-product-page",
    "st-stm32f405-datasheet",
    "st-rm0090",
    "st-cmsis-device-header",
    "st-cmsis-startup",
    "st-stm32f4-svd-bundle",
    "stm32-rs-stm32f405-patch",
    "embassy-rs-stm32f405rg-json",
    "embassy-rs-embassy-stm32-cargo",
];
pub const GENERATED_PROVENANCE_EVIDENCE_IDS: &[&str] = &[
    "e_olimex_target",
    "e_cmsis_cpu",
    "e_cmsis_irq_inventory",
    "e_startup_vectors",
    "e_st_svd_register_model",
    "e_stm32_rs_patch",
    "e_embassy_chip_topology",
    "e_embassy_feature_flag",
];

pub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {
    document_title: "ST STM32F405RGT6",
    document_version: "0.1.0",
    device_name: "STM32F405RGT6",
    target_architecture: None,
    executor_idle_strategy: None,
    feature_flags: &["stm32f405rg"],
    provenance_sources: &[
        ProvenanceSource {
            id: "olimex-stm32-h405-product-page",
            name: "STM32-H405 product page",
            kind: Some("other"),
            path: None,
        },
        ProvenanceSource {
            id: "st-stm32f405-datasheet",
            name: "DM00037051",
            kind: Some("datasheet"),
            path: None,
        },
        ProvenanceSource {
            id: "st-rm0090",
            name: "DM00031020",
            kind: Some("reference-manual"),
            path: None,
        },
        ProvenanceSource {
            id: "st-cmsis-device-header",
            name: "stm32f405xx.h",
            kind: Some("vendor-header"),
            path: None,
        },
        ProvenanceSource {
            id: "st-cmsis-startup",
            name: "startup_stm32f405xx.s",
            kind: Some("source-code"),
            path: None,
        },
        ProvenanceSource {
            id: "st-stm32f4-svd-bundle",
            name: "en.stm32f4-svd.zip",
            kind: Some("svd"),
            path: None,
        },
        ProvenanceSource {
            id: "stm32-rs-stm32f405-patch",
            name: "stm32f405.yaml",
            kind: Some("generated"),
            path: None,
        },
        ProvenanceSource {
            id: "embassy-rs-stm32f405rg-json",
            name: "STM32F405RG.json",
            kind: Some("generated"),
            path: None,
        },
        ProvenanceSource {
            id: "embassy-rs-embassy-stm32-cargo",
            name: "embassy-stm32/Cargo.toml",
            kind: Some("hal"),
            path: None,
        },
    ],
    provenance_evidence: &[
        ProvenanceEvidence {
            id: "e_olimex_target",
            name: "Olimex STM32-H405 target page",
            source_ref: "olimex-stm32-h405-product-page",
            normalized_claim: Some(
                "The Olimex STM32-H405 product page identifies the board as an STM32-H405 Cortex-M4 board and separately notes a later GD32 substitution path.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.9f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_cmsis_cpu",
            name: "CMSIS CPU macros",
            source_ref: "st-cmsis-device-header",
            normalized_claim: Some(
                "The official STM32F405 header defines __CM4_REV=0x0001U, __MPU_PRESENT=1, __FPU_PRESENT=1, __NVIC_PRIO_BITS=4, and __Vendor_SysTickConfig=0.",
            ),
            extraction_method: Some("parser"),
            confidence: Some(0.99f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_cmsis_irq_inventory",
            name: "CMSIS IRQ inventory",
            source_ref: "st-cmsis-device-header",
            normalized_claim: Some(
                "The official header enumerates the external IRQ inventory for STM32F405xx-class devices.",
            ),
            extraction_method: Some("parser"),
            confidence: Some(0.98f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_startup_vectors",
            name: "Startup vector ordering",
            source_ref: "st-cmsis-startup",
            normalized_claim: Some(
                "The official startup file lists the external vector ordering from WWDG through FPU for STM32F405xx.",
            ),
            extraction_method: Some("parser"),
            confidence: Some(0.97f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_st_svd_register_model",
            name: "Official ST SVD register model",
            source_ref: "st-stm32f4-svd-bundle",
            normalized_claim: Some(
                "STM32F405.svd in the official ST SVD bundle provides the primary register-bearing structural model for the device family.",
            ),
            extraction_method: Some("imported"),
            confidence: Some(0.95f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_stm32_rs_patch",
            name: "stm32-rs STM32F405 patch notes",
            source_ref: "stm32-rs-stm32f405-patch",
            normalized_claim: Some(
                "The stm32-rs device patch documents known family-level SVD cleanup for STM32F405, including Ethernet removals and naming fixes.",
            ),
            extraction_method: Some("manual"),
            confidence: Some(0.88f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_embassy_chip_topology",
            name: "Exact-chip package topology",
            source_ref: "embassy-rs-stm32f405rg-json",
            normalized_claim: Some(
                "The exact-chip STM32F405RG JSON provides package-filtered pin mappings, peripheral RCC bindings, interrupt links, and DMA route candidates.",
            ),
            extraction_method: Some("imported"),
            confidence: Some(0.9f64),
            locator: None,
        },
        ProvenanceEvidence {
            id: "e_embassy_feature_flag",
            name: "Embassy stm32f405rg feature",
            source_ref: "embassy-rs-embassy-stm32-cargo",
            normalized_claim: Some(
                "The embassy-stm32 crate exposes the stm32f405rg feature in its supported chip list.",
            ),
            extraction_method: Some("parser"),
            confidence: Some(0.96f64),
            locator: None,
        },
    ],
};
