# HAIR Repository Requirements

## Purpose and scope

This document defines the governing repository requirements for HAIR. It
covers the current HAIR document model, repository workflows, implemented
CLI surface, and the validation boundary that must hold before downstream
generation is trusted.

The repository baseline is intentionally conservative. Only implemented
commands, documented workflow contracts, and schema-backed invariants are
treated as current requirements.

## Repository requirements

### RQ-001 One device per HAIR document

The repository shall treat one top-level HAIR document as the description of
exactly one concrete device variant.

**Acceptance criteria**
- `documentKind` is `hair`.
- `metadata.role` is `device-variant`.
- `structure.device` describes a single concrete device variant.

### RQ-002 Shared hardware is composed by explicit imports

The repository shall support reuse of shared hardware definitions through
explicit imported HAIR documents or fragments without changing the one-device
scope of the importing document.

**Acceptance criteria**
- Imports are declared through `imports[]`.
- Shared structural reuse may be referenced through `structure.sharedRefs[]`
  and `structure.device.extendsRefs[]`.
- The importing document remains the authoritative description of one concrete
  device variant.

### RQ-003 HAIR is a layered semantic IR

The repository shall model hardware through distinct semantic layers rather
than a single flat register schema.

**Acceptance criteria**
- The repository defines separate provenance, structure, semantics, physical,
  normalization, and validation layers.
- The top-level document composes those layers through `schema\hair.json`.
- SVD-like register structure is treated as one layer of the model, not the
  whole model.

### RQ-004 Common vocabulary remains centralized

The repository shall define shared primitive types and reusable low-level
vocabulary in a common schema layer used by the rest of the schema set.

**Acceptance criteria**
- Cross-layer primitives such as identifiers, references, expressions, value
  literals, numeric ranges, and named entities are defined in
  `schema\common.json`.
- Other schema layers reference those definitions rather than redefining them.

### RQ-005 Provenance is mandatory for auditable extraction

The repository shall preserve source traceability and evidence-backed claims
inside HAIR documents.

**Acceptance criteria**
- HAIR documents include `provenance.sources[]` and `provenance.evidence[]`.
- Schema entities that support provenance records use explicit source,
  evidence, review, confidence, or derivation references.
- Repository workflows treat provenance as part of the document itself rather
  than as an external sidecar.

### RQ-006 Extraction is evidence-first

The repository shall drive extraction from an explicit evidence manifest that
names one target device variant and the approved source set.

**Acceptance criteria**
- Evidence manifests conform to `schema\evidence-manifest.json`.
- A manifest identifies one target device and one approved source inventory.
- Extraction workflows use the manifest as the allowed evidence boundary.

### RQ-007 Extraction and audit are gated workflows

The repository shall treat source discovery, extraction, and audit as staged
workflows with explicit review gates rather than as implicit side effects of
generation.

**Acceptance criteria**
- Source discovery produces an evidence manifest and discovery report.
- Extraction produces a HAIR document and extraction report.
- Audit produces an audit report and an explicit readiness verdict.
- Workflow phases must stop on blocking ambiguity rather than silently
  inventing missing facts.

### RQ-008 Optional profiles are additive, not replacements

The repository shall treat `profiles.mcuSoc` and `profiles.embassyHal` as
optional layers that specialize the core HAIR model without replacing it.

**Acceptance criteria**
- `profiles.mcuSoc` provides canonical MCU/SoC topology and block
  classification.
- `profiles.embassyHal` provides a generator-facing lowering contract for
  Embassy-style output.
- Profile data may only narrow and bind hardware facts already represented in
  the core layers.

### RQ-009 Canonical cross-vendor terminology is normalization-owned

The repository shall represent cross-vendor peripheral, register, and field
concepts through explicit normalization data without overwriting vendor-native
names in the structural model.

**Acceptance criteria**
- `normalization` may define a seeded catalog of canonical terms for
  `peripheral`, `register`, and `field` concepts.
- Per-entity normalization mappings bind a target structural entity to one or
  more canonical term references.
- Vendor-facing names remain on the structural entities themselves; canonical
  mappings are additive normalization metadata rather than replacements for the
  source-derived names.
- Repository-managed generators may consume explicit canonical mappings only as
  additive semantic hints; those mappings must not silently rewrite structural
  names or replace required profile, topology, or semantic lowering inputs.
- First-cut coverage may be partial and seeded from the repository's current
  reference bundles and workflows rather than requiring an immediate universal
  vocabulary.

### RQ-010 Generator behavior is deterministic and explicit

The repository shall require deterministic lowering from HAIR and explicit
failure when required data is missing or unsupported.

**Acceptance criteria**
- Generators may only emit behavior justified by the approved HAIR inputs.
- Unsupported required data must cause explicit failure rather than silent
  omission or placeholder behavior.
- Generator outputs are treated as lowerings from HAIR, not as alternate
  sources of truth.

### RQ-011 The repository-managed CLI surface is limited

The repository shall define the current CLI contract as the implemented `hair`
commands only.

**Acceptance criteria**
- The supported command surface is:
  - `hair validate <input>`
  - `hair generate svd <input> [--output <path>]`
  - `hair generate embassy <input> --output-dir <path>`
  - `hair diff <left> <right>`
- `extract` and `normalize` are workflow contracts, not CLI commands in the
  current baseline.

### RQ-012 Validation is currently schema-conformance-first

The repository shall treat CLI validation as schema conformance against the
repository schema set, not as full semantic or profile execution.

**Acceptance criteria**
- `hair validate` checks conformance to `schema\hair.json` and referenced
  layered schemas.
- Declarative `validation.rules` and `validation.profiles` are part of the
  document model.
- Executing declarative validation rules is outside the current CLI guarantee.

### RQ-013 SVD generation preserves required device structure

The repository shall lower HAIR device documents to CMSIS-SVD-style output
when the required SVD-representable data is present.

**Acceptance criteria**
- SVD generation preserves the device, peripherals, interrupts, registers,
  fields, enumerated values, reset values, and access metadata when those
  concepts are representable in SVD.
- Device interrupt inventory is taken from `structure.device.interrupts[]`.
- Peripheral `interruptRefs` are the primary attribution path for emitting SVD
  peripheral interrupt blocks. If a declared device interrupt is not linked,
  generation may fall back only to one unambiguous same-name peripheral match;
  otherwise SVD generation fails explicitly rather than silently dropping or
  ambiguously attributing the interrupt.
- The input document must provide explicit CPU revision, endianness, interrupt
  priority width, and core feature flags needed by CMSIS-SVD output.

### RQ-014 Embassy generation is profile-derived

The repository shall allow Embassy-style HAL generation only from HAIR
documents that include the required canonical MCU topology and an explicit
Embassy profile contract.

**Acceptance criteria**
- Embassy generation requires `profiles.mcuSoc` and `profiles.embassyHal`.
- Generated driver APIs are derived from referenced topology, semantics, and
  reachable structural data rather than a fixed universal method list.
- Embassy generation may use explicit and unambiguous normalization canonical
  mappings as secondary cross-vendor resolution hints for supported lowering
  concepts, but those mappings do not replace explicit profile/topology/semantic
  closure and must not silently widen the supported subset.
- Supported lowering may bind through either direct per-block register layouts
  or composite MCU-specific routing/control fabrics when the approved HAIR
  document makes that path explicit and structurally reachable.
- Async and DMA-backed generated APIs are allowed only when the document
  carries the interrupt, DMA, pin-routing, and semantic-operation/state-machine
  facts needed to lower those behaviors deterministically.
- If a `gpio-port` driver instance claims capability tag
  `embedded-hal-async-wait`, the same profile entry shall carry explicit
  `gpioExtiWaitBindings` naming the exact per-line port-select,
  interrupt-mask, rising-trigger, falling-trigger, pending-flag,
  pending-clear, and interrupt-route handles used by EXTI-backed edge waits.
  The generator may lower `wait_for_high` / `wait_for_low` only from the same
  approved GPIO input-sample path already used for input reads, and shall fail
  explicitly rather than infer EXTI line routing, shared-vector attribution, or
  pending-clear semantics from vendor register names alone. When the same
  profile is lowered through `hair generate embassy-host`, the host-emulated
  crate shall preserve that same wait contract deterministically rather than
  widening it with host-only wake behavior.
- If an `i2c` driver instance selects
  `loweringPattern = "legacy-event-i2c-master"`, the same profile entry shall
  carry explicit `i2cMasterBindings` naming the control, status, data, and
  address-clear roles used by the generated master transaction path. The first
  such portable surface is limited to 7-bit controller-master transactions; the
  generator shall not infer START/STOP, address-phase, transmit-ready,
  receive-ready, byte-transfer-finished, ACK-control, or address-clear behavior
  from vendor register names alone.
- If the same `i2c` driver instance claims capability tag
  `embedded-hal-async-i2c-master`, it shall also carry the interrupt-route
  closure and any interrupt-enable handles required by the selected I2C family's
  async wake path. The generated async I2C surface remains limited to the same
  approved 7-bit controller-master contract and must fail explicitly for slave,
  SMBus, 10-bit addressing, or other unmodeled transaction families.
- A generated embedded Embassy HAL crate shall expose opt-in Cargo features
  for the emitted peripheral families rather than forcing every generated
  family and runtime hook into every consumer image. Those features shall be
  meaningful code-elimination boundaries: disabling a family feature must also
  suppress any module exports, helper tables, interrupt handlers, or runtime
  bring-up wiring that exist only to serve that disabled family or one of its
  optional async/IRQ-backed capabilities. The generated crate's default
  feature set shall remain empty so consumer firmware must select the families
  it actually uses explicitly.
- A generated embedded Embassy HAL crate shall also separate its
  lowering-essential runtime resources from any richer descriptive metadata
  surface. Constructors, runtime handles, and normal peripheral operations
  shall depend only on lean runtime resource records and shall not require
  transitive references to descriptive IDs, names, route inventories, semantic
  operation descriptions, or similar metadata that is not needed to execute
  the approved lowering.
- When the generator emits a richer metadata-inspection surface for downstream
  tooling, audit helpers, or reference smokes, that surface shall be exposed
  through distinct API entry points and constants rather than through the same
  runtime constructor inputs and handle fields used by production firmware. A
  consumer that uses only the lean runtime API shall be able to avoid retaining
  the richer metadata in the linked image without depending on an additional
  feature flag for correctness.
- When a generated bring-up or calibration helper depends on hardware status
  reaching a ready state before later writes or sampling can succeed, the
  approved semantic operation shall model that requirement explicitly with
  status-poll steps rather than relying on undocumented timing gaps or
  generator-specific hidden waits.
- If an `adc` driver instance claims a higher-level regular-group buffered
  sampling API backed by DMA, the same profile entry shall carry an explicit
  lowering-family selector and explicit binding refs naming the ADC and DMA
  control/data/status roles used by the generated sampling path. The generator
  shall not infer that path from vendor register names alone. The first such
  ADC family selector is `regular-sequence-adc-dma`, which is limited to
  software-started regular-group buffered sampling and may expose one-shot
  buffer fills and circular/continuous buffered capture only when the approved
  HAIR inputs justify those exact behaviors.
- A `regular-sequence-adc-dma` claim is allowed only when the same `adc` driver
  instance carries explicit `adcDmaBindings` naming the regular-sequence length
  and slot programming handles, per-channel sample-time handles, data register,
  either one direct software-start control or an explicit semantic start
  sequence, DMA transfer-count and memory/peripheral address handles, DMA
  channel enable handle, DMA half-transfer and transfer-complete status
  handles, the corresponding interrupt-enable handles when interrupt-driven
  circular sampling is claimed, and the semantic setup/clear operations needed
  for one-shot and circular operation on that family. The generator shall fail
  explicitly rather than recovering those roles from vendor-native names alone.
- When a `regular-sequence-adc-dma` lowering claims DMA-backed sampling, the
  generator shall also bring up the backing DMA controller path named by the
  same driver instance's `dmaRouteRefs` before touching DMA channel registers.
  That dependency shall be inferred from the referenced DMA route/controller
  topology plus the matching `dma` driver instance's clock/reset bindings, not
  from vendor-specific register-name guesses embedded in the ADC lowering.
- A `dma` driver instance may claim Embassy-aligned IRQ-driven completion
  futures only when it carries explicit `dmaAsyncBindings` plus the matching
  `interruptRouteRefs` for the bound DMA channels. Those bindings shall name the
  per-channel transfer-complete interrupt-enable, status, and clear handles, and
  may additionally name half-transfer handles when the generated API exposes
  half-buffer waits. The generator shall fail explicitly rather than inferring
  DMA interrupt servicing from controller names alone.
- A `regular-sequence-adc-dma` claim must remain scoped to the regular
  conversion group. Injected-group conversions, dual-ADC combined modes, and
  other ADC DMA families remain out of subset unless a later approved
  lowering-family contract names them explicitly.
- `usb-device` lowering is allowed only when the document carries the
  clock/reset, interrupt, pin-routing, and semantic/state-machine facts needed
  to lower the claimed USB behavior deterministically. If the generated surface
  claims endpoint-oriented behavior, the approved HAIR inputs shall model the
  relevant endpoint or FIFO control/data path explicitly. If the generated
  surface claims serial-style byte-stream helpers, those helpers shall remain
  traceable to an approved device-specific USB data path rather than to a
  repository-invented generic USB stack. If a USB lowering family has
  materially distinct bring-up behavior, the profile shall also carry an
  explicit lowering selector for that family and the referenced semantic
  operations shall justify the exact attach/reset-preservation pattern the
  generator emits.
- If a driver instance claims capability tag `embassy-time-driver`, the profile
  shall also carry an explicit selector distinguishing the existing
  SysTick-backed path from any hardware-timer-backed or rtc-backed path. A
  SysTick-backed time base is allowed only when the approved interrupt-path
  inputs justify it explicitly. A hardware-timer-backed time base is allowed
  only when the approved timer, interrupt, semantic, and structural inputs
  justify both the async wake behavior and any claimed blocking delay helpers
  deterministically. An rtc-backed time base is allowed only when the approved
  rtc, interrupt, semantic, and structural inputs justify the monotonic
  counter/alarm wake path deterministically. For lowering families whose
  generated code depends on directly named counter/alarm/interrupt roles, such
  as `counter-compare-timer`, the same non-SysTick profile entry shall also
  carry explicit
  `timeDriverBindings` naming the exact counter-read, alarm-programming,
  interrupt-enable, interrupt-pending, and interrupt-clear handles used by the
  generated lowering; the generator shall not infer those roles from vendor
  register names alone. When the timer family requires a distinct
  event/reload/latch step after alarm reprogramming, that binding map shall
  also carry the corresponding explicit semantic operation reference(s).
  If multiple hardware-timer lowering families have materially different timer
  semantics, the same profile entry shall also carry an explicit lowering-family
  selector rather than leaving that choice to generator heuristics. For a
  hardware-timer-backed or rtc-backed path, the generated core contract shall
  remain runtime-agnostic: it must expose the generated wake handler plus the
  unique approved interrupt-route metadata needed by a downstream runtime layer
  to bind and enable that interrupt without repository guesswork. The same
  controller path shall also carry explicit controller numbering semantics on
  the referenced `physical.interruptControllers[]` record whenever generated
  code or runtime helpers must place vectors or program controller
  enable/pending/active registers; repository heuristics are not an acceptable
  substitute for that controller contract. Controllers not yet used by such
  lowering may omit the metadata until a downstream flow needs it. The same
  profile entry shall also carry the explicit Embassy tick rate used by that
  non-SysTick source so generated async timing behavior and generated Cargo
  metadata agree on the duration unit. When the source peripheral exposes
  multiple interrupt sources or shares one device vector across multiple causes,
  the profile and interrupt topology shall still identify one explicit
  route/source/clear path for the generated time base rather than assuming an
  implicit default.
- `rtc` lowering is allowed only when the document carries the clock/reset,
  interrupt, semantic/state-machine, and structural facts needed to lower the
  claimed raw counter, prescaler, alarm, and flag/interrupt-handling surface
  deterministically. A generated rtc module may expose HAL-specific helpers for
  RTC behavior that does not fit the generic `embassy-time` contract, but those
  helpers must remain traceable to the approved RTC control and status path.
- `watchdog` lowering is allowed only when the document carries the clock/reset
  facts, when needed, plus the structural and semantic facts needed to lower
  watchdog feed, start, configuration, and any claimed status helpers
  deterministically. Because `embedded-hal` 1.0 does not ship watchdog traits,
  generated portable watchdog support may implement the aliased
  `embedded_hal_02::watchdog::Watchdog` and `WatchdogEnable` traits from
  `embedded-hal` 0.2, and may expose HAL-specific configuration/status helpers
  from that same approved path, but it shall not emit `WatchdogDisable` unless
  the approved HAIR inputs justify a real disable sequence.
- `flash` lowering is allowed only when the document carries a `flash`
  driver instance rooted in a `flash-controller` canonical block plus explicit
  `flashBindings` naming one managed storage region, the erase/write geometry,
  the busy/completion and optional error status handles, the
  program/page-erase/address/start controls, and the semantic unlock/lock and
  completion/error-clear operations required by the selected controller family.
  Portable lowering is limited to
  `embedded_storage::nor_flash::{ReadNorFlash, NorFlash}` on memory-mapped
  internal NOR arrays; option-byte, mass-erase, and vendor-specific fast-program
  helpers remain out of subset unless a later approved lowering family names
  them explicitly.
- When a `flash` driver instance selects
  `loweringPattern = "stm32f1-page-flash"`, that selector becomes part of the
  executable lowering contract. The generator shall fail explicitly rather than
  inferring unlock sequencing, erase/program geometry, or completion/error-flag
  clearing from vendor register names alone.
- Unsupported driver kinds, unresolved references, missing lowering inputs,
  and out-of-subset requests fail explicitly.

### RQ-015 Diff is structural and repository-aware

The repository shall compare HAIR documents structurally rather than as raw
text and shall support both filesystem and git-backed inputs.

**Acceptance criteria**
- `hair diff` accepts either filesystem paths or `git:<rev>:<repo-path>`
  operands.
- Diff output reports additions, removals, and changed values using stable
  HAIR paths.
- The command exits non-zero when differences are found.

### RQ-016 Exit behavior is stable

The repository shall preserve stable high-level CLI exit semantics.

**Acceptance criteria**
- Exit `0` indicates success; for `diff`, it indicates no differences.
- Exit `1` indicates a requested check failed for commands that report check
  outcomes directly, such as schema validation failure in `validate` or a diff
  finding in `diff`.
- Exit codes greater than `1` indicate operational or generation failures, such
  as unreadable input, invalid git selectors, schema rejection reached through a
  generator entry point, or generation failure.

## Non-goals in the current baseline

The following are explicitly outside the current repository guarantee:

1. Executing declarative `validation.rules` expressions from the CLI.
2. Treating `extract` or `normalize` as implemented CLI commands.
3. Defining complete cross-document import merge and override semantics.
4. Guaranteeing repository-managed CLI generation for every generator class
   named in long-term project goals.
5. Emitting placeholder Embassy drivers or silently widening generated driver
   behavior beyond explicit lowering inputs.

## Controlled gaps and open design areas

The current baseline intentionally leaves these areas open for later evolution:

- import resolution, merge, and override semantics across shared documents
- stronger semantic-expression constraints
- richer declarative validation execution
- additional generator contracts beyond the currently implemented CLI surface
- expansion of canonical peripheral/register/field vocabularies and
  profile-specific readiness
  rules
