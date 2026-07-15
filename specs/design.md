# HAIR Repository Design

## Purpose

This document describes the current repository design that satisfies the HAIR
governing requirements. It covers the schema architecture, workflow
composition, artifact layout, and Rust CLI implementation boundaries.

## 1. Repository architecture overview

The repository is organized around five cooperating parts:

1. **Schema layers** under `schema\` define the HAIR document model, the
   evidence-manifest input model, and optional profile layers.
2. **Human-oriented design references** under `docs\` explain the current
   schema, CLI, MCU profile, and Embassy profile contracts.
3. **Workflow skills** under `.github\skills\` formalize staged source
   discovery, extraction, audit, bootstrap, evolution, and maintenance
   processes.
4. **Rust tooling** under `src\main.rs` implements the current repository
   CLI.
5. **Reference evidence and generated artifacts** under `evidence\` provide
   concrete device examples, reports, and generator outputs used as reality
   checks.

This structure keeps the semantic model, operational workflows, and generated
outputs separate while allowing them to reinforce one another.

## 2. Top-level HAIR document composition

The top-level schema is `schema\hair.json`. It composes the repository model
through these sections:

| Section | Responsibility |
| --- | --- |
| `metadata` | document identity and human metadata |
| `imports` | explicit composition of shared HAIR material |
| `provenance` | source artifacts, evidence, and review traceability |
| `structure` | concrete device, peripherals, registers, fields, and interrupts |
| `semantics` | behaviors, operations, state machines, and semantic relationships |
| `physical` | clocks, timing, power, reset, pins, packages, and electrical facts |
| `normalization` | canonical names, mappings, and vendor-quirk handling |
| `validation` | declarative rules and validation profiles |
| `profiles` | optional domain-specific specialization layers |

The top-level composition enforces the one-device-per-document rule while
leaving optional layers off unless the relevant evidence or workflow scope
requires them.

**Supports:** RQ-001, RQ-002, RQ-003

## 3. Layered schema design

### 3.1 Common layer

`schema\common.json` centralizes the shared vocabulary used by the rest of
the schema set. It provides identifiers, qualified references, expressions,
value literals, numeric ranges, named entities, and array helpers.

This prevents low-level type drift across the other schema layers.

**Supports:** RQ-004

### 3.2 Provenance layer

`schema\provenance.json` models source artifacts, evidence records, review
records, and per-entity provenance references. Provenance is embedded in HAIR
documents so that generated or reviewed facts remain traceable to approved
evidence.

**Supports:** RQ-005, RQ-006

### 3.3 Structural layer

`schema\structure.json` defines the concrete device graph: architectures,
device families, peripheral templates, the concrete `device`, memory regions,
interrupts, peripherals, registers, clusters, fields, and enumerated values.

Two structural design decisions are central:

1. `structure.device.interrupts[]` is the authoritative device interrupt
   inventory.
2. `register.alternateOfRef` is reserved for same-offset alternate register
   views, not for general shared-base overlays.

The structural layer is the main SVD-overlapping slice of HAIR, but it is not
the full model.

**Supports:** RQ-001, RQ-002, RQ-003, RQ-013

### 3.4 Semantic layer

`schema\semantics.json` captures behaviors, operations, state machines, and
semantic relationships that are not expressible in plain register topology.

This layer gives the repository a place to represent executable intent such as
initialization sequences, mode transitions, control semantics, and side
effects.

**Supports:** RQ-003, RQ-010, RQ-014

### 3.5 Physical layer

`schema\physical.json` captures clocks, timing constraints, power domains,
reset domains, pins, packages, electrical constraints, address spaces, and
interrupt-controller facts.

The physical layer exists because downstream tooling frequently depends on
non-register constraints that are absent from SVD-like formats.

**Supports:** RQ-003, RQ-010, RQ-014

### 3.6 Normalization layer

`schema\normalization.json` defines peripheral families, canonical terms,
normalized mappings, naming rules, and vendor quirks. This layer separates
vendor-native structure from the canonical model that downstream tooling can
depend on across device families.

For first-cut terminology standardization, the normalization layer may carry a
seeded catalog of canonical peripheral, register, and field terms plus
per-entity mappings from structural entities to one or more of those terms.
This keeps the source-derived vendor names in `structure` while making
cross-vendor comparability explicit and reviewable.

Repository-managed generators may also consume those mappings as additive
semantic lookup hints when a supported lowering path needs to recognize
equivalent cross-vendor concepts without rewriting the source-derived
structural names. In that role, normalization remains secondary to explicit
profile, topology, semantic, and structural reachability rules; it does not
become a replacement executable schema.

**Supports:** RQ-003, RQ-009, RQ-010

### 3.7 Validation layer

`schema\validation.json` defines validation rules and validation profiles as
data inside the HAIR model. The layer is part of the document contract even
though the current CLI only enforces schema conformance.

This preserves an explicit place for richer validation logic without requiring
the current tooling to invent partial execution semantics.

**Supports:** RQ-012

## 4. Optional profile design

### 4.1 MCU/SoC profile

`schema\profiles\mcu.json` provides a canonical architecture layer over the
core model. Its major responsibilities are:

- canonical block classification
- execution-model binding
- routing-fabric description
- interrupt topology
- clock/reset topology
- DMA topology
- pin topology
- memory, bus, low-power, and security topology

This profile gives generators and validators stable architectural names and
topology records without relocating the underlying hardware facts out of the
core layers.

**Supports:** RQ-008, RQ-014

### 4.2 Embassy HAL profile

`schema\profiles\embassy-hal.json` defines a generator-facing lowering
contract. Driver instances reference canonical blocks, clock/reset bindings,
interrupt routes, DMA routes, pin roles, semantic operations, and state
machines.

The design intent is explicit: generated APIs are derived from approved
lowering inputs, not from a fixed placeholder method list per `driverKind`.
The profile also preserves a structured metadata surface for downstream use.
The same contract must accommodate more than one hardware-lowering family for a
given driver kind. In particular, GPIO lowering may come either from a classic
single-block register layout or from a composite route/control path such as
ESP32-C3 GPIO + IO MUX + GPIO Matrix, provided the approved HAIR records make
the emitted behavior structurally reachable without guesswork. Likewise,
interrupt-driven and DMA-backed UART/I2C/SPI/ADC behavior is part of the
supported subset only when the driver instance names the full interrupt, DMA,
pin, and semantic closure required for real lowering.

USB device lowering follows the same evidence-bounded rule. A `usb-device`
driver instance may expose endpoint-oriented helpers, serial-style byte-stream
helpers, or both, but only for the subset whose control and data paths are
explicitly modeled by approved clock/reset bindings, interrupt routes, D+/D-
pin routes, semantic operations/state machines, and structurally reachable
register/field data. Device-specific USB transport shapes such as the
ESP32-C3 USB Serial/JTAG FIFO path are valid first-cut lowering families when
the approved HAIR records make that path explicit; the generator must not
pretend that one vendor-specific path implies a universal generic USB stack.
When a supported USB lowering family also depends on family-specific bring-up
policy, that policy must be selected explicitly in `profiles.embassyHal`
rather than hidden inside generator heuristics. For example, a
`serial-jtag-preserve-link` lowering family may require the generator to
preserve a boot-established USB Serial/JTAG link instead of emitting a generic
reset-and-reattach sequence; that distinction belongs in the approved profile
contract plus referenced semantic operations, not in ad hoc code generation.

Time-base support follows the same explicit-contract rule. The existing
SysTick-backed `embassy-time-driver` path remains valid, but it is not the only
allowed architecture. When a document claims a hardware-timer-backed or
rtc-backed Embassy time base, that choice must also be selected explicitly in
`profiles.embassyHal` rather than inferred from `driverKind` or capability tags
alone. The non-SysTick path must remain evidence-bounded: the approved HAIR
inputs need to justify how the timer or rtc source is configured, started,
acknowledged, advanced, and connected to the wake interrupt path. If the
generated driver also exposes blocking delay helpers or rtc-specific raw control
helpers, those helpers must come from the same approved counter/alarm semantics
instead of from repository-invented timing behavior.

When one supported hardware-timer time-base architecture still has materially
different lowering families, that family choice must also stay explicit. For
example, a free-running counter plus compare/alarm timer path that derives async
timing from prescaler, reload, counter, compare/alarm, event, and
status/acknowledge registers must be selected through
`driverInstances[].loweringPattern` rather than by silent generator probing.
That same path must also narrow any multi-cause timer interrupt inventory to one
approved interrupt source/route/clear sequence for the generated time base,
even when the device shares several timer causes on one vector.

The generated core contract for that hardware-timer path must stay
runtime-agnostic. In practice, that means the generated Embassy module may emit
timer initialization, blocking delay helpers, the wake-handler entry point, and
metadata describing the unique approved interrupt route, but it must not assume
one specific board runtime owns interrupt binding. A board-level runtime layer
may use that contract to map and enable the interrupt on a concrete platform
such as ESP32-C3 without making `esp-hal` part of the core generated HAL model.
When the runtime layer or generated helper methods must translate
`structure.device.interrupts[].number` into controller-facing vector slots or
enable/pending/active register indices, that translation must come from the
referenced `physical.interruptControllers[]` record rather than from
architecture-specific generator heuristics. The controller model is therefore
the authoritative home for vector-table numbering and controller-register
indexing semantics. If the controller declares that the interrupt number is only
an external-interrupt index for vector placement, any core exception-slot prefix
must come from the CPU/runtime contract rather than from controller-local guess
logic.
That same contract must also preserve the timer tick frequency explicitly so the
generated crate can select the matching `embassy-time-driver` tick-rate feature
instead of silently falling back to Embassy's 1 MHz default.

For non-SysTick Embassy time bases whose lowering family depends on one directly
named counter/alarm/interrupt path, the profile must also name the exact
generator-facing binding handles rather than relying on generator-side register
name probing. The contract uses
`timeDriverBindings` on the driver instance to name:

- the free-running counter register or field
- the compare/alarm register or field
- any semantic apply operations needed after alarm reprogramming
- the interrupt-enable field
- the interrupt-pending field
- the semantic clear/ack operation for that wake cause

This keeps the emitted time-driver lowering auditable and prevents the
generator from silently re-discovering timing semantics from vendor-native
register names that the approved HAIR profile did not explicitly bind.

Higher-level ADC DMA lowering needs the same discipline. Some MCU families can
support buffered regular-group sampling through a stable combination of ADC
sequence programming, sample-time programming, a data register, and one linked
DMA channel, but the exact control path is still family-specific. For that
case, the profile uses `driverInstances[].loweringPattern` on an `adc` driver
instance to select an approved lowering family instead of letting the generator
probe register names heuristically. The first such family is
`regular-sequence-adc-dma`, which is intentionally limited to software-started
regular-group buffered sampling and does not imply injected or dual-ADC modes.

When that ADC family is selected, the same driver instance must also carry an
explicit `adcDmaBindings` map naming the direct roles the generated API needs:

- the regular-sequence length field
- the ordered regular-sequence slot fields
- the per-channel sample-time fields
- the ADC data register
- either one direct software-start control or an explicit semantic start sequence
- the DMA transfer-count register or field
- the DMA peripheral-address and memory-address registers or fields
- the DMA channel-enable control
- the DMA half-transfer and transfer-complete status flags
- the matching DMA interrupt-enable handles when interrupt-driven circular
  sampling is claimed
- the semantic setup operations for one-shot and circular mode
- the semantic clear/ack operations for DMA half-transfer and
  transfer-complete events

The DMA controller bring-up for this family is intentionally not repeated in
`adcDmaBindings`. The ADC lowering already names its DMA path through
`dmaRouteRefs`, and the matching `dma` driver instance already owns the
controller-local clock/reset bindings for that path. Reusing that
route-to-controller closure keeps the ADC family explicit about
sequence/sample/data roles while avoiding a second, drifting source of truth
for the same DMA controller gate or reset path.

IRQ-driven DMA futures sit one layer lower. Instead of letting each peripheral
family invent its own wake/clear/interrupt bookkeeping, the `dma` driver
instance now owns an explicit `dmaAsyncBindings` map for the DMA channels that
may wake async tasks. Peripheral families such as
`regular-sequence-adc-dma` compose on top of that controller-local async DMA
surface rather than embedding a second copy of the same interrupt logic.

Separately, ADC families may require init/calibration helpers to wait for
ready-status bits to clear before software start or DMA sampling is valid. That
wait must stay in the approved semantic operation itself as explicit field polls
rather than as an undocumented generator-side delay or ad hoc busy loop.

This keeps the CH32-style buffered ADC path auditable while still allowing the
generated API to expose higher-level one-shot and circular sampling helpers.
The family-specific setup operations carry the fixed register writes whose
encoding differs across vendors, while the binding refs name the dynamic roles
that the generated code must program per call.

RTC lowering follows the same evidence-bounded rule as the other supported
driver kinds, but aligns with the way Embassy commonly exposes RTC hardware:
as a HAL-specific rtc module rather than a universal cross-platform trait.
Within HAIR, an `rtc` driver instance may therefore serve two related but
distinct roles from the same approved control path:

1. provide an rtc-backed `embassy-time-driver` source when the document carries
   the explicit counter/alarm/interrupt/tick-rate closure needed for async wake
   scheduling
2. expose HAL-specific raw RTC helpers for counter, prescaler, alarm, and
   flag/interrupt handling that do not fit the generic `embassy-time` contract

Watchdog lowering follows the same explicit-contract rule. Because
`embedded-hal` 1.0 does not ship watchdog traits, a `watchdog` driver instance
may expose portable feed/start support through an aliased `embedded-hal` 0.2
dependency such as `embedded_hal_02::watchdog::{Watchdog, WatchdogEnable}`,
plus HAL-specific raw configuration/status helpers, only when the approved
HAIR inputs justify that same watchdog control/status path explicitly. For
watchdog families such as CH32 IWDG whose approved path does not justify a
real disable sequence, the generator must omit `WatchdogDisable` rather than
synthesizing disable behavior.

Flash lowering follows the same evidence-bounded rule, but uses the portable
`embedded-storage` NOR traits rather than a device-family-specific HAL trait.
A `flash` driver instance stays rooted in a `flash-controller` canonical block
and a named managed storage region. The corresponding `flashBindings` map names
the controller-local erase/program geometry, busy/completion and optional error
status handles, program/page-erase/address/start controls, and the semantic
unlock/lock/flag-clear operations required by the selected sequencer family.
This keeps the generated `ReadNorFlash` / `NorFlash` surface traceable to one
approved memory-mapped flash path without inventing option-byte, mass-erase, or
fast-program behavior. Because internal flash controllers vary materially across
families, that family choice must stay explicit when needed; the first such
selector is `loweringPattern = "stm32f1-page-flash"` for STM32F1-class
page-erase/program controllers, which also covers CH32V203's compatible FLASH
control block when the approved bindings close the full path explicitly.

When a document also carries explicit normalization canonical mappings, Embassy
lowering may use those mappings as secondary resolution hints for supported
register, field, or peripheral concepts that recur across vendors. This is a
variance-reduction aid for the lowering implementation, not permission to infer
new behavior, bypass required profile data, or rename the source-derived
structural model. Ambiguous canonical mappings, or canonicalized lowering paths
that conflict with the explicit profile/structural contract, must fail
explicitly. Absent canonical mappings are not by themselves an error when some
other supported explicit lowering path still resolves the concept safely.

**Supports:** RQ-008, RQ-010, RQ-014

## 5. Workflow architecture

The repository uses staged workflows rather than a monolithic extraction
pipeline.

### 5.1 Source discovery

`find-mcu-sources` identifies one exact target device variant, gathers
approved evidence, and writes an evidence manifest plus discovery report.

### 5.2 Extraction

`extract` transforms one approved evidence manifest into one HAIR document and
an extraction report. It is provenance-first, rejects unsupported invention,
and treats full-device coverage as a real completeness bar.

### 5.3 Audit

`audit` challenges a HAIR document against its approved evidence and claimed
scope. It exists to falsify unsupported claims before downstream generation is
trusted.

### 5.4 Repository-governance workflows

`bootstrap` establishes the governing requirements, design, and validation
baseline. `evolve` changes that baseline under explicit review gates.
`maintain` audits for drift between the baseline and the implementation.

Together, these workflows separate evidence approval, extraction, challenge,
and repository governance into explicit phases with human review gates.

**Supports:** RQ-006, RQ-007

## 6. Rust CLI design

### 6.1 Command surface

The Rust crate exposes the current repository-managed CLI:

- `validate`
- `generate svd`
- `generate embassy`
- `diff`

The CLI is implemented in a single root crate and uses `clap` subcommands to
dispatch to each operation.

**Supports:** RQ-011

### 6.2 Schema loading and validation

The CLI locates the repository schema root by walking ancestor directories
until it finds `schema\hair.json`. It then loads the complete schema set,
normalizes layered `allOf` composition, resolves schema references to the
repository document set, and compiles a Draft 2020-12 validator.

This design allows `hair validate` and generator entry points to operate on
the repository schema set as a coherent whole rather than relying on ad hoc
single-file validation.

**Supports:** RQ-003, RQ-012

### 6.3 `validate`

`validate` loads one JSON document, compiles the repository schema set, and
checks conformance to the HAIR document schema. It reports path-oriented
validation errors and does not execute declarative validation-rule logic.

**Supports:** RQ-012, RQ-016

### 6.4 `generate svd`

`generate svd` first validates the input document, then lowers the validated
HAIR model to SVD output. Output is written to stdout by default or to
`--output` when specified.

The SVD path depends on explicit CPU metadata and preserves the device-level
interrupt inventory only when each declared interrupt can be attributed either
through explicit `interruptRefs` or through one unambiguous same-name
peripheral match. Otherwise the generator fails explicitly rather than
inventing or silently dropping SVD interrupt attribution.

SVD lowering remains vendor-faithful: normalization canonical terminology may
improve cross-vendor reasoning elsewhere in the repository, but it does not
silently rename emitted SVD peripherals, registers, or fields.

**Supports:** RQ-010, RQ-013, RQ-016

### 6.5 `generate embassy`

`generate embassy` validates the input document, resolves the Embassy
generation model, and emits a multi-file Rust crate rooted at `--output-dir`.
It generates `Cargo.toml`, `src\lib.rs`, `src\metadata.rs`, and one or more
module files derived from the resolved driver set.

The generator enforces driver-kind support, scope checks, reference
resolution, and profile-specific failure contracts before writing output. Its
lowering path is intentionally family-aware rather than vendor-name-driven:
multiple register-layout or routing-fabric strategies may satisfy the same
driver kind when they all preserve the same evidence-bounded API contract.
Explicit canonical normalization mappings may further reduce internal lowering
variance by helping the generator recognize equivalent supported concepts
across vendor naming schemes without changing the document's structural names.
That family-aware path may also accept exact vendor aliases within a supported
layout family when the approved document still carries the explicit register and
field closure required for the emitted methods; for example, a CH32 USART that
uses `STATR` / `DATAR` / `CTLR1-3` names may still lower through the
STM32-class USART path only when the same supported baud, control, status, and
data fields are explicitly modeled.

**Supports:** RQ-010, RQ-014, RQ-016

### 6.6 `diff`

`diff` accepts filesystem paths and `git:<rev>:<repo-path>` selectors, loads
both operands as JSON, compares parsed structures, and reports stable path
differences as additions, removals, or changed values.

**Supports:** RQ-015, RQ-016

## 7. Artifact layout and responsibilities

| Path | Responsibility |
| --- | --- |
| `schema\` | governing HAIR, manifest, and profile schemas |
| `docs\` | repository design references and command/profile contracts |
| `.github\skills\` | staged workflow contracts for source discovery, extraction, audit, and repository governance |
| `src\main.rs` | Rust CLI implementation and tests |
| `evidence\` | reference manifests, HAIR documents, reports, and generated outputs |
| `specs\` | governing repository baseline documents |

Within `evidence\`, each device directory acts as a self-contained reference
bundle that may include:

- `evidence-manifest.json`
- `hair.json`
- `source-discovery.md`
- `extraction-report.md`
- `audit-report.md`
- generated outputs such as SVD, PAC, Embassy crates, and smoke projects

**Supports:** RQ-006, RQ-007, RQ-009

## 8. Reference-device role in the design

The repository uses concrete device directories as reality checks for both the
schema and the generators. Current examples demonstrate:

- evidence-manifest driven extraction scope
- HAIR document structure for concrete devices
- extraction and audit reporting
- generated SVD and Embassy outputs
- smoke-project packaging for selected generated Embassy crates

These examples are part of the repository design because they keep the schema,
workflows, and tooling grounded in real device artifacts.

**Supports:** RQ-006, RQ-007, RQ-009

## 9. Requirement-specific design coverage

### 9.1 RQ-001 design coverage

RQ-001 is realized by the top-level HAIR composition in Section 2 and the
structural-layer rules in Section 3.3. The design keeps one concrete device in
`structure.device` and treats the enclosing document as the unit of ownership.

### 9.2 RQ-002 design coverage

RQ-002 is realized by the explicit `imports` composition model in Section 2 and
the structural reuse model in Section 3.3. Shared hardware can be referenced,
but the importing document remains the concrete device definition.

### 9.3 RQ-003 design coverage

RQ-003 is realized by the layered schema architecture described in Sections 2
and 3. The repository separates provenance, structure, semantics, physical,
normalization, validation, and optional profiles into distinct layers.

### 9.4 RQ-004 design coverage

RQ-004 is realized by the common-layer design in Section 3.1. Shared
identifiers, references, expressions, literals, and entity metadata are
defined once and reused across the schema set.

### 9.5 RQ-005 design coverage

RQ-005 is realized by the provenance-layer design in Section 3.2 and by the
workflow architecture in Section 5. The repository embeds provenance inside
HAIR documents and carries it through extraction and audit workflows.

### 9.6 RQ-006 design coverage

RQ-006 is realized by the evidence-manifest input model in Sections 1, 2, and
5.1-5.2, plus the artifact layout in Section 7. The repository starts
extraction from one approved manifest and carries that evidence boundary into
device bundles under `evidence\`.

### 9.7 RQ-007 design coverage

RQ-007 is realized by the staged workflow architecture in Section 5 and the
artifact responsibilities in Section 7. Source discovery, extraction, audit,
bootstrap, evolve, and maintain are distinct review-gated steps.

### 9.8 RQ-008 design coverage

RQ-008 is realized by the optional-profile design in Section 4. The MCU/SoC
and Embassy profile layers specialize the core model without replacing the core
hardware facts.

### 9.9 RQ-009 design coverage

RQ-009 is realized by the normalization-layer design in Section 3.6. Canonical
terminology is modeled as additive normalization metadata rather than as a
rewrite of source-derived structural names, and mappings may bind one entity to
more than one canonical term when the vendor-facing entity bundles multiple
concepts. When generators consume these mappings, they do so only as additive
semantic hints layered on top of the explicit lowering contract.

### 9.10 RQ-010 design coverage

RQ-010 is realized by the generator-facing schema and CLI design in Sections
3.4-3.7, 4.2, 6.4, and 6.5. Lowering depends on explicit topology, semantics,
and structural reachability, and unsupported cases fail explicitly.

### 9.11 RQ-011 design coverage

RQ-011 is realized by the Rust CLI command-surface design in Section 6.1 and
by the command-specific designs in Sections 6.3-6.6. The implemented command
surface is intentionally limited to validate, generate svd, generate embassy,
and diff.

### 9.12 RQ-012 design coverage

RQ-012 is realized by the validation-layer design in Section 3.7 and the CLI
schema-loader design in Sections 6.2-6.3. Declarative validation rules exist
in the model, while the implemented CLI currently enforces schema conformance.

### 9.13 RQ-013 design coverage

RQ-013 is realized by the structural-layer interrupt and CPU metadata design in
Section 3.3 and the SVD lowering design in Section 6.4. The SVD path depends
on explicit device-level interrupt inventory and explicit CPU metadata, while
requiring safe peripheral attribution for emitted SVD interrupt blocks.

### 9.14 RQ-014 design coverage

RQ-014 is realized by the MCU/SoC and Embassy profile designs in Section 4 and
the Embassy CLI path in Section 6.5. Embassy generation resolves driver
instances from explicit canonical topology and profile-declared lowering
contracts, with optional normalization canonical mappings available as
secondary variance-reduction hints when they are explicit and unambiguous.

### 9.15 RQ-015 design coverage

RQ-015 is realized by the diff command design in Section 6.6. The repository
loads JSON operands from either filesystem paths or git selectors and reports
stable structural differences.

### 9.16 RQ-016 design coverage

RQ-016 is realized by the CLI command architecture in Section 6, where commands
return normalized success, check-failure, and operational-failure exit
semantics.

## 10. Requirement traceability

| Design area | Primary requirements |
| --- | --- |
| Top-level document composition | RQ-001, RQ-002, RQ-003 |
| Common and provenance layers | RQ-004, RQ-005, RQ-006 |
| Structural, semantic, physical, normalization layers | RQ-003, RQ-009, RQ-010, RQ-013, RQ-014 |
| Optional profiles | RQ-008, RQ-014 |
| Workflow architecture | RQ-006, RQ-007, RQ-009 |
| CLI architecture | RQ-011, RQ-012, RQ-013, RQ-014, RQ-015, RQ-016 |
| Evidence and generated artifacts | RQ-006, RQ-007, RQ-009 |

## 11. Current design limits

The current design intentionally leaves these areas outside the stable
baseline:

1. complete import merge and override semantics across shared documents
2. execution of declarative validation rules from the CLI
3. repository-managed CLI commands for extraction or normalization
4. generator contracts beyond the currently implemented SVD and Embassy paths
5. blanket claims that every valid HAIR document is Embassy-generation-ready
