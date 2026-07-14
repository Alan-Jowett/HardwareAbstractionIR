//! Generated Embassy-style HAL metadata for WCH CH32V203G6U6.

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

pub const GENERATED_PROVENANCE_SOURCE_IDS: &[&str] = &["wch-ch32v203-datasheet", "wch-ch32fv2x-v3x-rm", "openwch-ch32v20x-sdk", "openwch-ch32v20x-header", "openwch-ch32v20x-startup-d6", "openwch-ch32v20x-adc-dma-example", "openwch-ch32v20x-tim-dma-example", "qingke-v4-processor-manual", "ch32-rs-ch32v203xx-svd", "ch32-rs-ch32v203g6u6-yaml", "adafruit-qt-py-ch32v203-schematic", "adafruit-qt-py-ch32v203-overview", "adafruit-qt-py-ch32v203-pinouts"];
pub const GENERATED_PROVENANCE_EVIDENCE_IDS: &[&str] = &["e_scope_metadata", "e_model_table", "e_architecture", "e_clock_power", "e_memory_map_figure", "e_package_ordering", "e_pin_notes", "e_linker_memory", "e_header_d6_group", "e_header_base_addresses", "e_header_irqs", "e_startup_vector", "e_rm_overview", "e_core_manual", "e_sdk_toolchain", "e_core_priority_bits", "e_core_vendor_systick", "e_pin_table_qfn28", "e_header_register_typedefs", "e_header_register_bitdefs", "e_rm_tim1_register_model", "e_rm_tim234_register_model", "e_rm_tim_aux_availability", "e_rm_can_filter_extent", "e_header_usb_register_typedefs", "e_header_register_window_extents", "e_core_io_qualifiers", "e_ch32rs_ch32v203xx_svd_metadata", "e_ch32rs_ch32v203g6u6_yaml_variant", "e_ch32rs_ch32v203g6u6_yaml_usb_fragments", "e_ch32rs_ch32v203g6u6_yaml_clock_reset_topology", "e_ch32rs_ch32v203g6u6_yaml_interrupt_topology", "e_ch32rs_ch32v203g6u6_yaml_dma_topology", "e_ch32rs_ch32v203g6u6_yaml_pin_topology", "e_header_tim_counter_enable", "e_header_adc_enable_calibration_bits", "e_adc_dma_example", "e_tim_dma_example", "e_adafruit_usb_c_usage", "e_adafruit_usb_native_cdc", "e_adafruit_schematic_usb_native_not_uart", "e_adafruit_schematic_usb_cc_termination", "e_adafruit_pinouts_uart_pa23", "e_adafruit_pinouts_neopixel_pa4", "e_adafruit_schematic_neopixel_pa4"];

pub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {
    document_title: "WCH CH32V203G6U6",
    document_version: "0.1.0",
    device_name: "CH32V203G6U6",
    target_architecture: Some("rv32imacxw"),
    executor_idle_strategy: Some("spin"),
    feature_flags: &["adc", "dma", "i2c", "interrupt", "pwm", "rcc", "spi", "timer", "uart", "usb"],
    provenance_sources: &[ProvenanceSource { id: "wch-ch32v203-datasheet", name: "CH32V203DS0.PDF", kind: Some("datasheet"), path: None }, ProvenanceSource { id: "wch-ch32fv2x-v3x-rm", name: "CH32FV2x_V3xRM.PDF", kind: Some("reference-manual"), path: None }, ProvenanceSource { id: "openwch-ch32v20x-sdk", name: "openwch/ch32v20x", kind: Some("sdk"), path: None }, ProvenanceSource { id: "openwch-ch32v20x-header", name: "ch32v20x.h", kind: Some("vendor-header"), path: None }, ProvenanceSource { id: "openwch-ch32v20x-startup-d6", name: "startup_ch32v20x_D6.S", kind: Some("source-code"), path: None }, ProvenanceSource { id: "openwch-ch32v20x-adc-dma-example", name: "ADC_DMA main.c", kind: Some("source-code"), path: None }, ProvenanceSource { id: "openwch-ch32v20x-tim-dma-example", name: "TIM_DMA main.c", kind: Some("source-code"), path: None }, ProvenanceSource { id: "qingke-v4-processor-manual", name: "QingKeV4_Processor_Manual.PDF", kind: Some("other"), path: None }, ProvenanceSource { id: "ch32-rs-ch32v203xx-svd", name: "CH32V203xx.svd", kind: Some("svd"), path: None }, ProvenanceSource { id: "ch32-rs-ch32v203g6u6-yaml", name: "CH32V203G6U6.yaml", kind: Some("generated"), path: None }, ProvenanceSource { id: "adafruit-qt-py-ch32v203-schematic", name: "Adafruit QT Py CH32V203.sch", kind: Some("source-code"), path: None }, ProvenanceSource { id: "adafruit-qt-py-ch32v203-overview", name: "Adafruit QT Py CH32V203 overview", kind: Some("other"), path: None }, ProvenanceSource { id: "adafruit-qt-py-ch32v203-pinouts", name: "Adafruit QT Py CH32V203 pinouts", kind: Some("other"), path: None }],
    provenance_evidence: &[ProvenanceEvidence { id: "e_scope_metadata", name: "Datasheet scope metadata", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("The official WCH CH32V203DS0 datasheet v3.0 explicitly covers CH32V203G6U6 as part of the CH32V203 family model table."), extraction_method: Some("manual"), confidence: Some(0.99f64), locator: Some("fragment=CH32V203DS0_PDF / download/file?id=354 metadata") }, ProvenanceEvidence { id: "e_model_table", name: "Model comparison table", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("CH32V203G6U6 provides 32KB flash, 10KB SRAM, 24 GPIOs, 2 USART/UART, 1 SPI, 1 I2C, 1 CAN, USBD present, USBHD absent, and two 10-channel ADC/TKey units."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("page=7") }, ProvenanceEvidence { id: "e_architecture", name: "Architecture overview", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("CH32V203 is based on the QingKe V4B 32-bit RISC-V core and supports up to 144MHz system frequency."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("page=1") }, ProvenanceEvidence { id: "e_clock_power", name: "Clock and supply facts", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("The device uses 2.4V to 3.6V system supply, built-in 8MHz HSI RC, built-in 40KHz LSI RC, external 3MHz to 25MHz HSE, external 32.768KHz LSE, and PLL-derived SYSCLK up to 144MHz."), extraction_method: Some("manual"), confidence: Some(0.94f64), locator: Some("page=1") }, ProvenanceEvidence { id: "e_memory_map_figure", name: "Memory map figure", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("The family memory map places SRAM at 0x20000000, APB/AHB peripheral space at 0x40000000, and USBFS at 0x50000000."), extraction_method: Some("manual"), confidence: Some(0.9f64), locator: Some("page=9") }, ProvenanceEvidence { id: "e_package_ordering", name: "Package and ordering table", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("CH32V203G6U6 is the exact QFN28 package variant in the CH32V203 family model table and package pin-definition table."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("page=7") }, ProvenanceEvidence { id: "e_pin_notes", name: "G6U6 OSC pin remap note", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("For CH32V203G6U6, the OSC_IN and OSC_OUT package pads can be remapped to GPIO PD0 and PD1 by software."), extraction_method: Some("manual"), confidence: Some(0.92f64), locator: Some("pages=29-30") }, ProvenanceEvidence { id: "e_linker_memory", name: "Generic D6 linker memory layout", source_ref: "openwch-ch32v20x-sdk", normalized_claim: Some("The official openwch generic D6 linker script documents the low-density CH32V203F6/G6/C6 memory layout as flash at the 0x00000000 boot alias with 32KB length and RAM at 0x20000000 with 10KB length."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("fragment=EVT/EXAM/SRC/Ld/Link.ld") }, ProvenanceEvidence { id: "e_header_d6_group", name: "Header D6 grouping", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official header maps CH32V20x_D6 to low/medium-density CH32V203 parts including CH32V203G6 class devices."), extraction_method: Some("imported"), confidence: Some(0.96f64), locator: Some("fragment=EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h, lines=19-24") }, ProvenanceEvidence { id: "e_header_base_addresses", name: "Peripheral base macros", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official header defines the CH32V20x D6 peripheral base addresses used for the structural peripheral map."), extraction_method: Some("parser"), confidence: Some(0.96f64), locator: Some("fragment=EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h, lines=879-926") }, ProvenanceEvidence { id: "e_header_irqs", name: "IRQ enumeration", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official header enumerates the CH32V20x_D6 interrupt identifiers used by the G6U6 variant."), extraction_method: Some("imported"), confidence: Some(0.92f64), locator: Some("fragment=EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h, lines=112-115") }, ProvenanceEvidence { id: "e_startup_vector", name: "Startup vector table", source_ref: "openwch-ch32v20x-startup-d6", normalized_claim: Some("The official D6 startup file defines the G6U6-class external interrupt vector ordering from WWDG through DMA1_Channel8."), extraction_method: Some("imported"), confidence: Some(0.98f64), locator: Some("fragment=EVT/EXAM/SRC/Startup/startup_ch32v20x_D6.S, lines=40-87") }, ProvenanceEvidence { id: "e_rm_overview", name: "Reference manual overview", source_ref: "wch-ch32fv2x-v3x-rm", normalized_claim: Some("The CH32FV2x_V3xRM reference manual is the official register and peripheral manual for CH32V203G6U6-class devices."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("pages=1-3") }, ProvenanceEvidence { id: "e_core_manual", name: "QingKe V4 processor scope", source_ref: "qingke-v4-processor-manual", normalized_claim: Some("The QingKe V4 processor manual covers V4A/V4B/V4C/V4F cores and provides ISA-level details for the CH32V203G6U6 QingKe V4B core."), extraction_method: Some("manual"), confidence: Some(0.9f64), locator: Some("page=1") }, ProvenanceEvidence { id: "e_sdk_toolchain", name: "SDK compiler architecture flags", source_ref: "openwch-ch32v20x-sdk", normalized_claim: Some("The official CH32V203G6U6 example build compiles for -march=rv32imacxw and -mabi=ilp32, with no floating-point ISA extension enabled."), extraction_method: Some("imported"), confidence: Some(0.9f64), locator: Some("fragment=C++/Use MRS Create C++ project-example/CH32V203G6U6++/obj/Core/subdir.mk, line=18") }, ProvenanceEvidence { id: "e_core_priority_bits", name: "PFIC priority bit layout", source_ref: "openwch-ch32v20x-sdk", normalized_claim: Some("The CH32V20x core header documents interrupt priority using bit7 as pre-emption priority and bits6-bit4 as subpriority, implying four implemented interrupt priority bits."), extraction_method: Some("manual"), confidence: Some(0.88f64), locator: Some("fragment=C++/Use MRS Create C++ project-example/CH32V203G6U6++/Core/core_riscv.h, lines=259-271") }, ProvenanceEvidence { id: "e_core_vendor_systick", name: "Vendor system timer block", source_ref: "openwch-ch32v20x-sdk", normalized_claim: Some("The CH32V20x core support code exposes a vendor-defined SysTick register block at 0xE000F000 and the official CH32V203G6U6 debug support code configures it directly for delay timing, so the device uses vendor system-timer configuration rather than a CMSIS-standard SysTick abstraction."), extraction_method: Some("manual"), confidence: Some(0.86f64), locator: Some("fragment=C++/Use MRS Create C++ project-example/CH32V203G6U6++/Core/core_riscv.h and Debug/debug.c, lines=102-118") }, ProvenanceEvidence { id: "e_pin_table_qfn28", name: "QFN28 pin definition table", source_ref: "wch-ch32v203-datasheet", normalized_claim: Some("Table 3-1-3 defines the CH32V203 QFN28 package pad assignments, default functions, and alternate/remap functions used for the CH32V203G6U6 pin model."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("pages=29-31") }, ProvenanceEvidence { id: "e_header_register_typedefs", name: "Vendor header register structs", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official CH32V20x header defines typed register structs, member order, widths, and offsets for the major CH32V203G6U6 peripheral families, including USB, CAN, DMA channels, and timers."), extraction_method: Some("parser"), confidence: Some(0.96f64), locator: Some("fragment=ch32v20x.h, lines=136-680") }, ProvenanceEvidence { id: "e_header_register_bitdefs", name: "Vendor header bit-definition sections", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official CH32V20x header provides register bit masks and bit-position macros for ADC, AFIO, BKP, CAN core registers, CRC, DMA, EXTI, FLASH, GPIO, I2C, IWDG, PWR, RCC, RTC, SPI, TIM, USART, WWDG, and related blocks."), extraction_method: Some("parser"), confidence: Some(0.95f64), locator: Some("fragment=ch32v20x.h, lines=890-4598") }, ProvenanceEvidence { id: "e_rm_tim1_register_model", name: "RM TIM1 advanced timer register model", source_ref: "wch-ch32fv2x-v3x-rm", normalized_claim: Some("The reference manual models TIM1 as an advanced-control timer on CH32V20x_D6, with RPTCR, BDTR, DMACFGR, and DMAADR in the TIM1 register block."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("pages=237-242") }, ProvenanceEvidence { id: "e_rm_tim234_register_model", name: "RM TIM2-TIM4 register tables", source_ref: "wch-ch32fv2x-v3x-rm", normalized_claim: Some("The CH32V20x_D6 TIM2, TIM3, and TIM4 register tables omit BDTR and RPTCR, retain DMACFGR and DMAADR, and widen TIM2 CNT/ATRLR/CCR1-4 to 32 bits."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("pages=253-254") }, ProvenanceEvidence { id: "e_rm_tim_aux_availability", name: "RM TIMx_AUX availability note", source_ref: "wch-ch32fv2x-v3x-rm", normalized_claim: Some("The TIMx_AUX register is only available for selected CH32F20x_D8 / CH32V30x_D8 / CH32V31x_D8C products, excluding CH32V20x_D6 devices such as CH32V203G6U6."), extraction_method: Some("manual"), confidence: Some(0.94f64), locator: Some("page=243") }, ProvenanceEvidence { id: "e_rm_can_filter_extent", name: "RM CAN1 filter-bank extent", source_ref: "wch-ch32fv2x-v3x-rm", normalized_claim: Some("The CH32V20x CAN1 filter register table extends from F0R1/F0R2 through F27R1/F27R2, so CH32V203G6U6 exposes 28 filter banks even though the header bit-definition sections stop at bank 13."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("pages=430-432") }, ProvenanceEvidence { id: "e_header_usb_register_typedefs", name: "Vendor header USB register structs", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official CH32V20x header defines concrete USBFS device and USBFS host register structs at the shared USBFS base address, establishing register presence and offsets even though matching bit-definition sections are absent."), extraction_method: Some("parser"), confidence: Some(0.94f64), locator: Some("fragment=ch32v20x.h, lines=587-680") }, ProvenanceEvidence { id: "e_header_register_window_extents", name: "Vendor header register window extents", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official CH32V20x header's typed peripheral structs, nested mailbox/filter structs, and reserved padding bound the contiguous register-window extent of each modeled peripheral block, allowing address-block coverage to be derived from the struct layout."), extraction_method: Some("parser"), confidence: Some(0.9f64), locator: Some("fragment=ch32v20x.h, lines=136-680") }, ProvenanceEvidence { id: "e_core_io_qualifiers", name: "Core access qualifier definitions", source_ref: "openwch-ch32v20x-sdk", normalized_claim: Some("The official CH32V20x core header defines __I as read-only, __O as write-only, and __IO as read/write, providing the access-permission mapping used by the peripheral access-layer structs."), extraction_method: Some("manual"), confidence: Some(0.92f64), locator: Some("fragment=EVT/EXAM/SRC/Core/core_riscv.h, lines=15-18") }, ProvenanceEvidence { id: "e_ch32rs_ch32v203xx_svd_metadata", name: "Community CH32V203 SVD metadata overlap", source_ref: "ch32-rs-ch32v203xx-svd", normalized_claim: Some("The community CH32V203xx SVD provides register descriptions, many register reset values, field descriptions, and some field access annotations for CH32V203-family peripherals. When those metadata are imported only on exact peripheral-name, register-name-plus-offset, and field-name-plus-bit-range matches against the official-source-derived HAIR topology, they serve as an auditable metadata gap-filler without reshaping the recovered register structure."), extraction_method: Some("imported"), confidence: Some(0.8f64), locator: Some("fragment=device CH32V20xxx / peripherals section") }, ProvenanceEvidence { id: "e_ch32rs_ch32v203g6u6_yaml_variant", name: "Community CH32V203G6U6 YAML exact-variant fit", source_ref: "ch32-rs-ch32v203g6u6-yaml", normalized_claim: Some("The community CH32V203G6U6 YAML names the exact CH32V203G6U6 variant, records the QFN28 package and 32KB flash / 10KB RAM sizing, and includes CH32V203 peripheral fragments such as USART1/2, TIM1, TIM2/3/4, ADC1/2, SPI1, I2C1, USBD, and CAN1 for conservative exact-variant topology reuse."), extraction_method: Some("manual"), confidence: Some(0.75f64), locator: Some("fragment=CH32V203G6U6.yaml, lines=1-34") }, ProvenanceEvidence { id: "e_ch32rs_ch32v203g6u6_yaml_usb_fragments", name: "Community CH32V203G6U6 USBD register fragment", source_ref: "ch32-rs-ch32v203g6u6-yaml", normalized_claim: Some("The pinned CH32V203G6U6 ch32-data source includes the USBD peripheral fragment, which resolves to the shared USB register definitions and supports conservative device-mode register metadata reuse without importing the absent USBFS host overlay."), extraction_method: Some("manual"), confidence: Some(0.82f64), locator: Some("fragment=CH32V203G6U6.yaml -> ../peripherals/FV2x_V3x_USBD.yaml") }, ProvenanceEvidence { id: "e_ch32rs_ch32v203g6u6_yaml_clock_reset_topology", name: "Community CH32V203G6U6 clock/reset topology", source_ref: "ch32-rs-ch32v203g6u6-yaml", normalized_claim: Some("The pinned CH32V203G6U6 YAML and its included family/peripheral fragments identify the HCLK, PCLK1, PCLK2, PCLK1_TIM, PCLK2_TIM, and ADC clock domains plus the RCC enable/reset register bindings for DMA1, GPIOA-D, AFIO, USART1/2, SPI1, I2C1, TIM1/2/3/4, ADC1/2, CAN1, and USBD."), extraction_method: Some("manual"), confidence: Some(0.86f64), locator: Some("fragment=CH32V203G6U6.yaml -> ../family/CH32V2.yaml and included peripheral YAML rcc sections") }, ProvenanceEvidence { id: "e_ch32rs_ch32v203g6u6_yaml_interrupt_topology", name: "Community CH32V203G6U6 interrupt topology", source_ref: "ch32-rs-ch32v203g6u6-yaml", normalized_claim: Some("The pinned CH32V203G6U6 YAML and the included CH32V2_D6 interrupt map define the interrupt signal to vector bindings for DMA1 channels, ADC1/2, USART1/2, SPI1, I2C1, TIM1/2/3/4, CAN1, USBD shared lines, and the common RCC/EXTI/RTC family signals for this subgroup; absent peripherals stay pruned from the exact-variant extraction."), extraction_method: Some("manual"), confidence: Some(0.86f64), locator: Some("fragment=CH32V203G6U6.yaml -> ../interrupts/CH32V2_D6.yaml and included peripheral YAML interrupt sections") }, ProvenanceEvidence { id: "e_ch32rs_ch32v203g6u6_yaml_dma_topology", name: "Community CH32V203G6U6 DMA channel map", source_ref: "ch32-rs-ch32v203g6u6-yaml", normalized_claim: Some("The pinned CH32V203G6U6 YAML includes the CH32V_V4B DMA map assigning DMA1 channels to ADC1, USART1/2, SPI1, I2C1, and timer request lines for the QingKe V4B family."), extraction_method: Some("manual"), confidence: Some(0.84f64), locator: Some("fragment=CH32V203G6U6.yaml -> ../dma/CH32V_V4B.yaml") }, ProvenanceEvidence { id: "e_ch32rs_ch32v203g6u6_yaml_pin_topology", name: "Community CH32V203G6U6 pin and remap topology", source_ref: "ch32-rs-ch32v203g6u6-yaml", normalized_claim: Some("The pinned CH32V203G6U6 peripheral YAML fragments enumerate USART, SPI, I2C, TIM, ADC, CAN, and USBD signal-to-pin routes and the AFIO remap controls that select alternate routes; routes landing on pins absent from the QFN28 package must be excluded for the exact variant."), extraction_method: Some("manual"), confidence: Some(0.85f64), locator: Some("fragment=CH32V203G6U6.yaml -> included peripheral YAML pin/remap sections") }, ProvenanceEvidence { id: "e_header_tim_counter_enable", name: "Vendor header TIM counter enable bit", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official CH32V20x header defines TIM_CEN as the CTLR1 counter-enable control bit for the general and advanced timer blocks."), extraction_method: Some("manual"), confidence: Some(0.9f64), locator: Some("fragment=ch32v20x.h, line=4252") }, ProvenanceEvidence { id: "e_header_adc_enable_calibration_bits", name: "Vendor header ADC enable and calibration bits", source_ref: "openwch-ch32v20x-header", normalized_claim: Some("The official CH32V20x header defines ADC_ADON, ADC_CAL, and ADC_RSTCAL in CTLR2, establishing the documented enable and calibration controls used by ADC1 and ADC2."), extraction_method: Some("manual"), confidence: Some(0.9f64), locator: Some("fragment=ch32v20x.h, lines=982-985") }, ProvenanceEvidence { id: "e_adc_dma_example", name: "Official ADC1 DMA example", source_ref: "openwch-ch32v20x-adc-dma-example", normalized_claim: Some("The official WCH ADC_DMA example enables ADC1 DMA, configures DMA1_Channel1 with DMA_DIR_PeripheralSRC from ADC1->RDATAR to memory, and waits for DMA1 transfer complete on channel 1."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("fragment=EVT/EXAM/ADC/ADC_DMA/User/main.c") }, ProvenanceEvidence { id: "e_tim_dma_example", name: "Official TIM1 update DMA example", source_ref: "openwch-ch32v20x-tim-dma-example", normalized_claim: Some("The official WCH TIM_DMA example initializes DMA1_Channel5 with DMA_DIR_PeripheralDST targeting TIM1_CH1CVR_ADDRESS, enables TIM_DMA_Update on TIM1, and uses that route to update PWM output values."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("fragment=EVT/EXAM/TIM/TIM_DMA/User/main.c") }, ProvenanceEvidence { id: "e_adafruit_usb_c_usage", name: "Adafruit USB-C board usage", source_ref: "adafruit-qt-py-ch32v203-pinouts", normalized_claim: Some("Adafruit documents the QT Py CH32V203 USB-C port as used for both powering and programming the board."), extraction_method: Some("manual"), confidence: Some(0.93f64), locator: Some("fragment=pinouts.md / Power") }, ProvenanceEvidence { id: "e_adafruit_usb_native_cdc", name: "Adafruit native USB device note", source_ref: "adafruit-qt-py-ch32v203-overview", normalized_claim: Some("Adafruit describes the board as exposing the CH32V203 native USB device interface and notes USB CDC support via TinyUSB at board level."), extraction_method: Some("manual"), confidence: Some(0.88f64), locator: Some("fragment=overview.md / feature list") }, ProvenanceEvidence { id: "e_adafruit_schematic_usb_native_not_uart", name: "Adafruit schematic USB nets and separate UART nets", source_ref: "adafruit-qt-py-ch32v203-schematic", normalized_claim: Some("The Adafruit schematic routes the USB-C D+ and D- nets directly to IC2 USBD-labeled pins, while the board's RX and TX nets are separately routed to PA3 and PA2; no discrete USB-UART bridge is shown."), extraction_method: Some("manual"), confidence: Some(0.86f64), locator: Some("fragment=USB-C and UART nets, lines=8233-8499") }, ProvenanceEvidence { id: "e_adafruit_schematic_usb_cc_termination", name: "Adafruit schematic USB-C CC termination", source_ref: "adafruit-qt-py-ch32v203-schematic", normalized_claim: Some("The Adafruit schematic places 5.1 kOhm resistors on both USB-C CC1 and CC2, consistent with a device-side sink attachment on the USB-C receptacle."), extraction_method: Some("manual"), confidence: Some(0.9f64), locator: Some("fragment=USB-C CC network, lines=7907-8371") }, ProvenanceEvidence { id: "e_adafruit_pinouts_uart_pa23", name: "Adafruit board UART header roles", source_ref: "adafruit-qt-py-ch32v203-pinouts", normalized_claim: Some("Adafruit documents the board UART header as RX on PA3 and TX on PA2."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("fragment=pinouts.md / UART") }, ProvenanceEvidence { id: "e_adafruit_pinouts_neopixel_pa4", name: "Adafruit board NeoPixel pinout", source_ref: "adafruit-qt-py-ch32v203-pinouts", normalized_claim: Some("Adafruit documents the onboard RGB NeoPixel LED as connected to PA4."), extraction_method: Some("manual"), confidence: Some(0.97f64), locator: Some("fragment=pinouts.md / NeoPixel LED") }, ProvenanceEvidence { id: "e_adafruit_schematic_neopixel_pa4", name: "Adafruit schematic NeoPixel net", source_ref: "adafruit-qt-py-ch32v203-schematic", normalized_claim: Some("The Adafruit schematic names the onboard RGB LED data net NEOPIX and routes it to IC2 pin PA4."), extraction_method: Some("manual"), confidence: Some(0.95f64), locator: Some("fragment=NeoPixel net, lines=8433-8440") }],
};
