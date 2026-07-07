# HAIR CLI

This document defines the first repository-managed CLI surface for HAIR.

## Scope

The current Rust CLI implements these commands today:

```text
hair validate <input>
hair generate svd <input> [--output <path>]
hair generate embassy <input> --output-dir <path>
hair generate embassy-host <input> --output-dir <path>
hair diff <left> <right>
```

`extract` and `normalize` remain workflow-driven operations implemented through repository skills and review processes. They are not part of the current CLI contract.

## Command contracts

### `hair validate`

`validate` accepts one HAIR JSON document and succeeds only when the document conforms to the repository schema set rooted at `schema/hair.json`.

First-cut behavior:

- resolve the layered schema set from the repository
- validate JSON structure, required fields, types, enums, and referenced subschemas
- report schema validation failures with enough location detail to identify the failing document path

Non-goals for the first cut:

- executing `validation.rules` expressions
- cross-document import resolution beyond what is needed for schema conformance
- semantic linting beyond schema validation

### `hair generate svd`

`generate` lowers one HAIR document to a CMSIS-SVD-style output.

First-cut behavior:

- require one HAIR JSON input document
- emit one SVD document to stdout by default or to `--output`
- map the HAIR device, peripherals, interrupts, registers, fields, enumerated values, reset values, and access metadata when those concepts are representable in SVD
- emit a complete CMSIS-SVD CPU block, including revision, endianness, MPU/FPU flags, interrupt priority bits, and the HAIR `vendorSystemTimerConfig` flag mapped to SVD `vendorSystickConfig`
- preserve declared `structure.device.interrupts[]` as the authoritative device interrupt inventory; peripheral `interruptRefs` may refine attribution, but missing linkage must not cause a real device interrupt to disappear from the generated SVD/PAC
- fail explicitly when SVD-required data is missing or when a structure that must appear in the SVD cannot be lowered safely

First-cut exclusions:

- PAC, HAL, simulator, or documentation generation
- silent omission of unsupported required data
- lossless export of HAIR-only layers such as provenance, normalization metadata, physical constraints, or semantic relationships that have no SVD representation

### `hair generate embassy`

`generate embassy` lowers one HAIR document to a multi-file Embassy-style HAL crate.

First-cut behavior:

- require one HAIR JSON input document plus `--output-dir`
- generate a compilable crate directory rather than a single stdout artifact
- consume the hardware facts from the core layers plus the canonical MCU topology in `profiles.mcuSoc`
- require an explicit `profiles.embassyHal` contract for the supported generated drivers
- derive the emitted Rust API surface from the approved topology and semantic lowering inputs in the HAIR document rather than from fixed placeholder signatures per driver kind
- allow a `gpio-port` driver instance to lower into a per-pin GPIO API surface when the approved HAIR routes and structural controls justify that shape
- preserve the generator-relevant structured subset of referenced topology and semantic inputs in the emitted Rust metadata so downstream code does not lose control refs, remap data, or executable semantic structure that the approved HAIR document already provides
- emit register-level code only for methods that can be justified by explicit HAIR lowering inputs, and fail explicitly when the requested or implied behavior is underspecified
- fail explicitly when the input document falls outside the documented supported subset or omits generator-required topology, semantics, or bindings documented in `docs/embassy-hal-profile.md`

First-cut exclusions:

- silent fallback from unsupported hardware to placeholder stubs
- fixed success-return driver methods that are disconnected from the input document's approved lowering data
- inference of driver contracts purely from vendor naming without the approved profile data
- silent widening of first-cut GPIO support into alternate-function or EXTI helpers when the approved profile did not request or justify them
- pretending that generic schema validity alone is enough for Embassy generation readiness

### `hair generate embassy-host`

`generate embassy-host` lowers one HAIR document to a separate host-only `std`
crate that pairs the generated Embassy-style HAL surface with explicit emulator
state and test-control APIs.

First-cut behavior:

- require one HAIR JSON input document plus `--output-dir`
- consume the same `profiles.mcuSoc` + `profiles.embassyHal` lowering contract
  used by `generate embassy`
- derive the generated host package/crate names automatically from
  `profiles.embassyHal.crate`
- emit a separate host-target crate rather than a dual-target feature split of
  the embedded crate
- preserve a 1:1 relationship between each generated HAL-visible device surface
  and a generated emulator/state handle for that same device
- keep the HAL-facing API evidence-bounded in the same way as
  `generate embassy`; host mode must not widen the executable HAL surface beyond
  what the approved HAIR lowering inputs justify
- expose host-only emulator/test-control APIs through companion emulator/state
  handles rather than by mutating the HAL driver types into a different shape
- support deterministic progress under explicit test control for simulated time,
  interrupts, DMA completion, and other emulated side effects needed by the
  generated HAL interactions
- fail explicitly if a generated HAL-visible device lacks a paired emulation
  surface or if an emulator/test-control API would require unsupported
  inference

First-cut exclusions:

- wall-clock-driven background progression as the default execution model
- a single crate that is both embedded-target and host-emulated through feature
  switches
- host-only placeholder emulations for behaviors that are not justified by the
  approved HAIR lowering inputs
- silently omitting emulator observability or control surfaces for generated
  HAL-visible devices

### `hair diff`

`diff` compares two HAIR document revisions and reports structural differences.

Each operand may be either:

- a filesystem path to a HAIR JSON document, or
- an explicit git-backed selector of the form `git:<rev>:<repo-path>`

First-cut behavior:

- load both HAIR JSON documents
- compare the parsed document structure rather than raw text
- report additions, removals, and changed values with stable paths into the HAIR document
- exit non-zero when differences are found

Non-goals for the first cut:

- semantic equivalence checking across reordered but equivalent arrays
- three-way merge support
- automatic comparison against extraction evidence or vendor PDFs

## Exit behavior

- `0`: command succeeded; for `diff`, no differences were found
- `1`: requested check failed; for example, schema validation failed or `diff` found differences
- `>1`: operational failure such as unreadable input, invalid git selector, or generation failure

## Relationship to the schema

The CLI is defined by the repository schema and documentation rather than by ad hoc tool behavior.

- `validate` is anchored to `schema/hair.json` and its referenced layered schemas.
- `generate svd` is a lowering step from HAIR, not a source of truth for the model.
- `generate embassy` is also a lowering step from HAIR, but one that depends on stronger generation-profile data and explicit unsupported-feature failures.
- `generate embassy-host` is a separate lowering step from the same
  `profiles.embassyHal` contract into a host-emulated companion crate.
- `diff` compares HAIR documents as repository artifacts; it does not redefine HAIR semantics.

The `validation` layer in HAIR remains important, but the first CLI cut only enforces schema conformance. Executing declarative validation rules is future work.

Cross-profile `entityRef` values used by `profiles.embassyHal` are generator-resolved rather than schema-resolved. `hair validate` may accept a document whose Embassy refs are syntactically valid but unresolved; `generate embassy` must reject that document explicitly.

Because `generate svd` now depends on CPU metadata that CMSIS-SVD requires, compliant HAIR device documents must include `structure.device.cpu.revision`, `structure.device.cpu.endianness`, `structure.device.cpu.interruptPriorityBits`, and `structure.device.cpu.featureFlags.{mpuPresent,fpuPresent,vendorSystemTimerConfig}`.
