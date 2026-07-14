# HAIR Repository Validation Baseline

## Purpose

This document defines how to verify the current HAIR repository baseline using
existing commands, reference artifacts, generated outputs, and review
workflows.

The validation baseline is intentionally split between:

1. **automated checks** provided by the Rust crate and its tests
2. **artifact-level checks** using reference device bundles under `evidence\`
3. **workflow reviews** for evidence approval, extraction claims, and audit
   verdicts

Validation promises only what the repository currently supports.

## 1. Validation principles

1. Validate the schema contract before trusting generation outputs.
2. Validate generators against real reference artifacts, not only synthetic
   fixtures.
3. Preserve a manual review gate where repository workflows are intentionally
   adversarial or evidence-dependent.
4. Treat unsupported or ambiguous inputs as failures or review blockers rather
   than as acceptable silent degradation.

## 2. Automated repository checks

### V-001 Rust regression suite

**Purpose:** verify CLI behavior, schema-loader behavior, generator
invariants, and profile-specific failure cases.

**Command**

```powershell
cargo test
```

**Expected result**
- The test suite passes.
- The suite covers schema validation, structural diffing, SVD lowering, and
  Embassy-generation contracts and failure paths.

### V-002 HAIR schema validation

**Purpose:** verify that a HAIR document conforms to the repository schema
set.

**Command**

```powershell
cargo run -- validate evidence\st\stm32f405rgt6\hair.json
```

**Expected result**
- The command succeeds.
- Failure output, when present, identifies the failing HAIR path.

### V-003 Structural diff behavior

**Purpose:** verify that diff is structural and stable.

**Command**

```powershell
cargo run -- diff evidence\st\stm32f405rgt6\hair.json evidence\st\stm32f405rgt6\hair.json
```

**Expected result**
- The command succeeds and reports no differences for identical inputs.

### V-004 SVD generation path

**Purpose:** verify that repository-managed SVD lowering still operates on a
passing reference HAIR document under the current CLI contract.

**Command**

```powershell
cargo run -- generate svd evidence\wch\ch32v203c8t6\hair.json --output <svd-output-path>
```

**Expected result**
- The command writes one SVD document.
- Generation succeeds when the reference document carries the required CPU
  metadata and the declared device interrupts can be attributed to peripherals
  through explicit links or one safe same-name match.

### V-005 Embassy generation path

**Purpose:** verify that repository-managed Embassy lowering still operates on
in-scope reference HAIR documents, including a composite-routing ESP32-C3
bundle.

**Command**

```powershell
cargo run -- generate embassy evidence\texas-instruments\lm3s6965\hair.json --output-dir <crate-output-dir>
cargo run -- generate embassy evidence\espressif\esp32-c3fn4\hair.json --output-dir <crate-output-dir>
```

**Expected result**
- The command writes a Rust crate rooted at `<crate-output-dir>`.
- The emitted crate contains `Cargo.toml`, `src\lib.rs`, `src\metadata.rs`,
  and generated driver modules justified by the reference document's profile
  scope.
- When a reference document carries explicit canonical normalization mappings,
  the generator may consume them only as additive lowering hints for supported
  equivalent concepts; missing or ambiguous mappings must not mask unsupported
  lowering inputs.
- The ESP32-C3 reference bundle succeeds without weakening the executable
  lowering contract, demonstrating support for a non-STM32/TM4C GPIO/routing
  family plus any claimed async/DMA-backed UART/I2C/SPI/ADC paths.
- If the ESP32-C3 reference bundle claims `usb-device` lowering, generation
  succeeds only when the approved HAIR inputs explicitly support the claimed USB
  control/data path. Generated USB artifacts must remain traceable either to
  endpoint/FIFO-oriented device records or to an approved device-specific
  serial-style USB path such as ESP32-C3 USB Serial/JTAG; generation must fail
  explicitly rather than synthesizing a generic USB stack from partial evidence.
- If a `usb-device` driver instance selects a lowering family whose bring-up
  behavior is materially specific (for example a boot-link-preserving USB
  Serial/JTAG path), generation succeeds only when the profile carries that
  explicit selector and the referenced semantic operations justify the emitted
  attach/reset-preservation sequence; otherwise generation fails explicitly.
- If a driver instance claims `embassy-time-driver`, generation succeeds only
  when the profile carries an explicit `timeDriverSource` selector and the
  referenced interrupt, semantic, and structural inputs justify that exact
  time-base architecture. Existing SysTick-backed paths must remain valid, while
  hardware-timer-backed paths must fail explicitly unless they justify both the
  async wake behavior and any claimed blocking delay helpers from approved HAIR
  data. For approved hardware-timer paths, the emitted core contract must also
  preserve the unique interrupt-route metadata and wake-handler hook needed for
  a downstream runtime layer to bind the concrete trap symbol explicitly, plus
  any generated interrupt-controller helper methods justified by the approved
  interrupt identities and the explicit Embassy tick rate needed to keep
  generated async durations aligned with the modeled hardware timer.
- If a hardware-timer time-driver path uses a lowering family whose generated
  code depends on directly named counter/alarm/interrupt roles, such as
  `counter-compare-timer`, generation also succeeds only when the driver
  instance carries explicit `timeDriverBindings` for the
  counter read, alarm/compare write, interrupt-enable, interrupt-pending, and
  interrupt-clear roles used by the generated time-driver lowering; the
  generator must fail explicitly rather than recovering those roles from vendor
  register names alone. If the selected timer family requires a separate
  event/reload/latch step after alarm writes, that same binding map must also
  name the required semantic apply operation(s).
- If a hardware-timer time-driver path uses a timer family whose supported
  lowering shape is materially distinct from other supported timer families,
  generation succeeds only when the driver instance also carries the explicit
  `loweringPattern` for that family. The first such family selector is
  `counter-compare-timer` for free-running counter plus compare/alarm timer
  paths.
- If a hardware-timer time-driver path is sourced from a timer with multiple
  interrupt causes or shared vectors, generation succeeds only when the profile
  and interrupt topology narrow the generated time base to one explicit
  interrupt route/source pair and one explicit clear operation; a timer that
  merely exposes update/trigger/compare inventory is not yet async-ready.

## 3. Artifact-level validation

### V-006 Evidence-manifest validation

**Purpose:** verify that extraction inputs remain explicit and single-target.

**Artifacts**
- `evidence\st\stm32f405rgt6\evidence-manifest.json`
- `evidence\texas-instruments\lm3s6965\evidence-manifest.json`
- `evidence\wch\ch32v203c8t6\evidence-manifest.json`

**Check**
- Each manifest names one concrete target device.
- Each manifest carries at least one approved source.
- Source entries are classified with repository-supported source kinds.

### V-007 Reference HAIR document review

**Purpose:** verify that example HAIR documents reflect the repository model
in realistic device bundles.

**Artifacts**
- `evidence\st\stm32f405rgt6\hair.json`
- `evidence\texas-instruments\lm3s6965\hair.json`
- `evidence\wch\ch32v203c8t6\hair.json`

**Check**
- Each document represents one concrete device variant.
- Provenance, structure, and any included profiles remain internally
  consistent.
- When `normalization.canonicalTerms[]` or `normalization.mappings[]` are
  present, mappings remain additive metadata: vendor-facing entity names are
  still preserved in `structure`, and each mapping resolves to one or more
  document-defined canonical terms.
- When repository-managed lowering consumes canonical mappings, it does so as an
  additive resolution aid; the document is not treated as if normalization
  silently replaced the explicit structural or profile contract.
- Example documents exercise different portions of the schema and generator
  surface.

### V-008 Generated-output inspection

**Purpose:** verify that reference generated artifacts remain aligned with the
repository contracts.

**Artifacts**
- `evidence\st\stm32f405rgt6\generated\stm32f405rgt6.svd`
- `evidence\st\stm32f405rgt6\generated\embassy\`
- `evidence\st\stm32f405rgt6\generated\embassy-smoke\`
- `evidence\wch\ch32v203g6u6\generated\ch32v203g6u6.svd`
- `evidence\wch\ch32v203g6u6\generated\pac\`
- `evidence\wch\ch32v203g6u6\generated\embassy\`
- `evidence\wch\ch32v203g6u6\generated\embassy-smoke\`
- `evidence\espressif\esp32-c3fn4\`
- `evidence\texas-instruments\lm3s6965\embassy-out\`
- `evidence\texas-instruments\lm3s6965\embassy-smoke\`

**Check**
- Checked-in SVD output is present for the STM32F405RGT6 reference bundle, and
  V-004 separately exercises the current passing SVD command path on
  `evidence\wch\ch32v203c8t6\hair.json`.
- The CH32V203G6U6 reference bundle keeps its checked-in SVD, PAC, Embassy HAL,
  and smoke-project artifacts aligned with the same HAIR document.
- Embassy output directories contain a complete generated crate layout.
- Smoke-project directories consume generated crates using normal Rust package
  boundaries rather than ad hoc post-processing.

## 4. Workflow validation

### V-009 Source-discovery review

**Purpose:** verify that source discovery remains evidence-first and
single-target.

**Artifacts**
- `evidence\...\source-discovery.md`
- `schema\evidence-manifest.json`
- `.github\skills\find-mcu-sources\SKILL.md`

**Check**
- Discovery reports justify the chosen evidence set.
- Approved sources are distinguished from rejected candidates and coverage
  gaps.
- Manifest content matches the documented discovery outcome.

### V-010 Extraction review

**Purpose:** verify that extraction remains provenance-first and scoped to
approved evidence.

**Artifacts**
- `evidence\...\extraction-report.md`
- `evidence\...\hair.json`
- `.github\skills\extract\SKILL.md`

**Check**
- Extraction reports identify target scope, source inventory, discovered
  hardware areas, metadata coverage, rejected claims, and limitations.
- When canonical terminology is in scope, extraction reports identify which
  peripheral/register/field entities received canonical term mappings, which
  candidates were deferred, and why.
- When `usb-device` executable lowering is claimed, extraction reports identify
  the supporting clock/reset bindings, interrupt routes, D+/D- pin routes, and
  the explicit endpoint/FIFO or serial-style semantic path that justifies each
  claimed generated behavior.
- HAIR documents align with the extraction scope claimed in the report.

### V-011 Audit review

**Purpose:** verify that full-device or generator-readiness claims survive
adversarial challenge.

**Artifacts**
- `evidence\wch\ch32v203c8t6\audit-report.md`
- `.github\skills\audit\SKILL.md`

**Check**
- Audit reports state approved evidence, completeness findings, unsupported
  claims, root-cause classification, and a final verdict.
- A document is not treated as downstream-ready solely because it is
  schema-valid.

### V-012 Governing-spec review

**Purpose:** verify that repository governance changes stay traceable.

**Artifacts**
- `specs\requirements.md`
- `specs\design.md`
- `specs\validation.md`
- `.github\skills\bootstrap\SKILL.md`
- `.github\skills\evolve\SKILL.md`
- `.github\skills\maintain\SKILL.md`

**Check**
- Requirements, design, and validation remain mutually traceable.
- Specification changes occur before implementation changes that would alter
  repository guarantees.
- Repository-owned canonical term guidance and normalization schema changes stay
  aligned with the extraction workflow contract.
- Maintenance work treats missing or drifting governing specs as a first-class
  problem.

## 5. Optional smoke validation

### V-013 QEMU smoke execution for the LM3S6965 Embassy example

**Purpose:** provide an executable sanity check for the LM3S6965 Embassy smoke
project.

**Command**

```powershell
powershell -ExecutionPolicy Bypass -File evidence\texas-instruments\lm3s6965\embassy-smoke\run-qemu-smoke.ps1
```

**Expected result**
- The smoke firmware builds for `thumbv7m-none-eabi`.
- QEMU output includes the expected UART0 smoke confirmation text.

**Note**
- This is an environment-dependent smoke check, not a universal repository
  precondition.

### V-014 QEMU smoke execution for the ESP32-C3 Embassy example

**Purpose:** provide an executable sanity check for the ESP32-C3 generated
Embassy smoke project under the pinned `sonde-esp-dev` container's
`qemu-system-riscv32`.

**Command**

```powershell
powershell -ExecutionPolicy Bypass -File evidence\espressif\esp32-c3fn4\generated\embassy-smoke\run-qemu-smoke.ps1
```

**Expected result**
- The smoke firmware builds for `riscv32imc-unknown-none-elf`.
- QEMU output includes the expected `PASS` confirmation text after exercising
  generated UART0 and interrupt-facing HAL paths.

**Note**
- This is an environment-dependent smoke check, not a universal repository
  precondition.
- The current ESP32-C3 QEMU path validates boot, UART, and interrupt behavior.
  GPIO output state is not asserted there because the emulator does not expose
  enough observable GPIO state for a reliable runtime check.

### V-015 Hardware smoke packaging for the CH32V203G6U6 Embassy example

**Purpose:** provide a hardware-flashable Embassy smoke image for the
CH32V203G6U6 reference bundle that exercises the generated HAL Embassy
time-driver path on the physical device with an externally observable signal.

**Command**

```powershell
powershell -ExecutionPolicy Bypass -File evidence\wch\ch32v203g6u6\generated\embassy-smoke\build-smoke-bin.ps1 -Release
```

**Expected result**
- The smoke firmware builds for `riscv32imc-unknown-none-elf`.
- The packaging step writes a flashable `.bin` beside the release ELF.
- When flashed to the physical QT Py CH32V203 hardware, the firmware emits a
  NeoPixel-compatible bitstream on `PA4` and alternates the onboard RGB LED
  between off and dim white with one-second `embassy_time::Timer::after(...)`
  delays.

**Note**
- This is a hardware-dependent smoke check, not a universal repository
  precondition.
- The smoke application shall consume the generated Embassy HAL crate through
  normal Rust package boundaries for RCC, GPIO, and Embassy time-driver setup.
- The QT Py onboard RGB LED uses `PA4` as a NeoPixel data input rather than as
  a simple GPIO-driven LED pin.

## 6. Requirement-specific validation coverage

### 6.1 RQ-001 validation coverage

RQ-001 is validated by V-002, V-006, and V-007. Together these checks confirm
that HAIR examples validate as one-document device descriptions and that the
reference manifests and HAIR documents identify one concrete target variant.

### 6.2 RQ-002 validation coverage

RQ-002 is validated by V-007 and V-012. Reference HAIR review checks the
composition model in concrete device bundles, and governing-spec review checks
that shared-document composition remains traceable in the repository baseline.

### 6.3 RQ-003 validation coverage

RQ-003 is validated by V-001, V-002, and V-007. The regression suite and HAIR
schema validation exercise the layered model structurally, while reference HAIR
review confirms that real device bundles use the layered document shape.

### 6.4 RQ-004 validation coverage

RQ-004 is validated by V-001 and V-002. The schema-loader and validation path
depend on the shared common vocabulary remaining consistent across the schema
set.

### 6.5 RQ-005 validation coverage

RQ-005 is validated by V-007, V-010, and V-011. Reference HAIR review,
extraction review, and audit review together check that provenance remains
embedded, evidence-backed, and usable for adversarial challenge.

### 6.6 RQ-006 validation coverage

RQ-006 is validated by V-006, V-009, and V-010. These activities check that
manifests remain explicit and single-target, that source discovery justifies
the approved evidence set, and that extraction stays scoped to that evidence.

### 6.7 RQ-007 validation coverage

RQ-007 is validated by V-009, V-010, V-011, and V-012. These reviews verify
that the staged workflow outputs and governance reviews remain distinct and
traceable.

### 6.8 RQ-008 validation coverage

RQ-008 is validated by V-001, V-007, and V-010. The regression suite and
reference artifact reviews confirm that optional profiles remain additive to
the core model rather than replacements for it.

### 6.9 RQ-009 validation coverage

RQ-009 is validated by V-007, V-010, V-011, and V-012. Reference artifact
review, extraction review, audit review, and governing-spec review together
check that canonical terminology remains additive normalization metadata rather
than a silent rewrite of vendor-facing structure, even when generator logic
uses canonical mappings as secondary hints.

### 6.10 RQ-010 validation coverage

RQ-010 is validated by V-001, V-004, V-005, V-008, and V-011. The regression
suite, generator runs, generated-output inspection, and audit review together
check deterministic lowering and explicit failure behavior.

### 6.11 RQ-011 validation coverage

RQ-011 is validated by V-001, V-002, V-003, V-004, and V-005. These automated
checks exercise the implemented CLI surface and its command-specific contracts.

### 6.12 RQ-012 validation coverage

RQ-012 is validated by V-001, V-002, and V-011. Schema validation confirms the
implemented CLI boundary, while audit review reinforces that schema validity
alone is not treated as full semantic readiness.

### 6.13 RQ-013 validation coverage

RQ-013 is validated by V-001, V-004, V-008. The regression suite, SVD
generation path, and generated-output inspection verify the current SVD
lowering contract on reference artifacts, including explicit failure when
interrupt attribution is incomplete or ambiguous.

### 6.14 RQ-014 validation coverage

RQ-014 is validated by V-001, V-005, V-008, V-010, and V-011. The regression
suite, Embassy generation path, generated crate inspection, extraction review,
and audit review jointly test the profile-derived Embassy contract across both
classic register-layout MCUs and composite-routing MCUs such as ESP32-C3,
including any approved use of canonical mappings as additive variance-reduction
hints rather than replacements for explicit lowering inputs.

### 6.15 RQ-015 validation coverage

RQ-015 is validated by V-001 and V-003. The regression suite and identical-doc
diff command check the structural diff behavior and stable path reporting
contract.

### 6.16 RQ-016 validation coverage

RQ-016 is validated by V-001, V-002, V-003, V-004, and V-005. These command
checks cover successful execution plus the current split between direct
check-failure outcomes and generator/operational failures across the
implemented CLI surface.

## 7. Requirement traceability matrix

| Requirement | Validation activities |
| --- | --- |
| RQ-001 | V-002, V-006, V-007 |
| RQ-002 | V-007, V-012 |
| RQ-003 | V-001, V-002, V-007 |
| RQ-004 | V-001, V-002 |
| RQ-005 | V-007, V-010, V-011 |
| RQ-006 | V-006, V-009, V-010 |
| RQ-007 | V-009, V-010, V-011, V-012 |
| RQ-008 | V-001, V-007, V-010 |
| RQ-009 | V-007, V-010, V-011, V-012 |
| RQ-010 | V-001, V-004, V-005, V-008, V-011 |
| RQ-011 | V-001, V-002, V-003, V-004, V-005 |
| RQ-012 | V-001, V-002, V-011 |
| RQ-013 | V-001, V-004, V-008 |
| RQ-014 | V-001, V-005, V-008, V-010, V-011 |
| RQ-015 | V-001, V-003 |
| RQ-016 | V-001, V-002, V-003, V-004, V-005 |

## 8. Current validation limits

The current validation baseline intentionally does not claim:

1. execution of declarative `validation.rules` from `hair validate`
2. fully automated verification of cross-document import resolution or merge
   semantics
3. automatic proof that every schema-valid document is generation-ready
4. repository-managed automated checks for unimplemented generator families
5. replacement of evidence review and adversarial audit with schema checks
   alone
