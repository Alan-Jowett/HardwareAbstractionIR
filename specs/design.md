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

**Supports:** RQ-001, RQ-002, RQ-003, RQ-012

### 3.4 Semantic layer

`schema\semantics.json` captures behaviors, operations, state machines, and
semantic relationships that are not expressible in plain register topology.

This layer gives the repository a place to represent executable intent such as
initialization sequences, mode transitions, control semantics, and side
effects.

**Supports:** RQ-003, RQ-009, RQ-013

### 3.5 Physical layer

`schema\physical.json` captures clocks, timing constraints, power domains,
reset domains, pins, packages, electrical constraints, address spaces, and
interrupt-controller facts.

The physical layer exists because downstream tooling frequently depends on
non-register constraints that are absent from SVD-like formats.

**Supports:** RQ-003, RQ-009, RQ-013

### 3.6 Normalization layer

`schema\normalization.json` defines peripheral families, normalized mappings,
naming rules, and vendor quirks. This layer separates vendor-native structure
from the canonical model that downstream tooling can depend on across device
families.

**Supports:** RQ-003, RQ-009

### 3.7 Validation layer

`schema\validation.json` defines validation rules and validation profiles as
data inside the HAIR model. The layer is part of the document contract even
though the current CLI only enforces schema conformance.

This preserves an explicit place for richer validation logic without requiring
the current tooling to invent partial execution semantics.

**Supports:** RQ-011

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

**Supports:** RQ-008, RQ-013

### 4.2 Embassy HAL profile

`schema\profiles\embassy-hal.json` defines a generator-facing lowering
contract. Driver instances reference canonical blocks, clock/reset bindings,
interrupt routes, DMA routes, pin roles, semantic operations, and state
machines.

The design intent is explicit: generated APIs are derived from approved
lowering inputs, not from a fixed placeholder method list per `driverKind`.
The profile also preserves a structured metadata surface for downstream use.

**Supports:** RQ-008, RQ-009, RQ-013

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

**Supports:** RQ-010, RQ-015

### 6.2 Schema loading and validation

The CLI locates the repository schema root by walking ancestor directories
until it finds `schema\hair.json`. It then loads the complete schema set,
normalizes layered `allOf` composition, resolves schema references to the
repository document set, and compiles a Draft 2020-12 validator.

This design allows `hair validate` and generator entry points to operate on
the repository schema set as a coherent whole rather than relying on ad hoc
single-file validation.

**Supports:** RQ-003, RQ-011

### 6.3 `validate`

`validate` loads one JSON document, compiles the repository schema set, and
checks conformance to the HAIR document schema. It reports path-oriented
validation errors and does not execute declarative validation-rule logic.

**Supports:** RQ-011, RQ-015

### 6.4 `generate svd`

`generate svd` first validates the input document, then lowers the validated
HAIR model to SVD output. Output is written to stdout by default or to
`--output` when specified.

The SVD path depends on explicit CPU metadata and preserves the device-level
interrupt inventory even when peripheral linkage is incomplete.

**Supports:** RQ-009, RQ-012, RQ-015

### 6.5 `generate embassy`

`generate embassy` validates the input document, resolves the Embassy
generation model, and emits a multi-file Rust crate rooted at `--output-dir`.
It generates `Cargo.toml`, `src\lib.rs`, `src\metadata.rs`, and one or more
module files derived from the resolved driver set.

The generator enforces driver-kind support, scope checks, reference
resolution, and profile-specific failure contracts before writing output.

**Supports:** RQ-009, RQ-013, RQ-015

### 6.6 `diff`

`diff` accepts filesystem paths and `git:<rev>:<repo-path>` selectors, loads
both operands as JSON, compares parsed structures, and reports stable path
differences as additions, removals, or changed values.

**Supports:** RQ-014, RQ-015

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

## 9. Requirement traceability

| Design area | Primary requirements |
| --- | --- |
| Top-level document composition | RQ-001, RQ-002, RQ-003 |
| Common and provenance layers | RQ-004, RQ-005, RQ-006 |
| Structural, semantic, physical, normalization layers | RQ-003, RQ-009, RQ-012, RQ-013 |
| Optional profiles | RQ-008, RQ-013 |
| Workflow architecture | RQ-006, RQ-007 |
| CLI architecture | RQ-010, RQ-011, RQ-012, RQ-013, RQ-014, RQ-015 |
| Evidence and generated artifacts | RQ-006, RQ-007, RQ-009 |

## 10. Current design limits

The current design intentionally leaves these areas outside the stable
baseline:

1. complete import merge and override semantics across shared documents
2. execution of declarative validation rules from the CLI
3. repository-managed CLI commands for extraction or normalization
4. generator contracts beyond the currently implemented SVD and Embassy paths
5. blanket claims that every valid HAIR document is Embassy-generation-ready
