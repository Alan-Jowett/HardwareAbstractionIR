# HAIR — Hardware Abstraction Intermediate Representation

HAIR is a vendor-neutral intermediate representation for describing the structure and behavior of microcontrollers and SoCs. It provides a canonical semantic model that can drive deterministic generation of downstream artifacts such as SVDs, PACs, HALs, simulators, validation reports, and documentation.

The goal is to make silicon definitions **consistent**, **auditable**, and **reproducible** across vendors and device families. In that sense, HAIR aims to be the **LLVM IR of silicon**: a foundational substrate for hardware tooling.

## Why HAIR?

Hardware metadata published by vendors is often fragmented across datasheets, header files, errata, and SDKs. Even when machine-readable formats exist, they are frequently incomplete, inconsistent, or incorrect.

HAIR addresses that by providing:

- A **single semantic source of truth** for hardware definitions
- **Normalization rules** that smooth over vendor naming and modeling quirks
- **Provenance metadata** for every extracted fact
- **Deterministic generation** of downstream artifacts
- An **LLM-friendly structure** for extraction, review, and transformation
- A basis for **automated validation** before code generation

## What HAIR Models

HAIR captures both structural and behavioral properties of devices, including:

- **Devices**: name, vendor, family, architecture, memory layout
- **Peripherals**: type, grouping, base addresses, descriptions
- **Registers**: offsets, widths, access modes, reset values
- **Fields**: bit ranges, enums, semantic meaning
- **Interrupts**: mappings and priority-related metadata
- **Timing**: clocks, prescalers, and frequency constraints
- **Electrical characteristics**: pin modes, pulls, drive strength, alternate functions
- **Normalization metadata**: canonical names, grouping rules, vendor quirks
- **Provenance**: source excerpts, page references, annotations, confidence scores

The representation is designed to be **machine-readable**, **LLM-friendly**, and **transformable**.

## Core Architecture

HAIR is organized around a small number of layers:

| Layer | Responsibility |
| --- | --- |
| Extraction | Build structured IR from PDFs, headers, HALs, reverse engineering, and human annotations |
| Normalization | Apply canonical naming, grouping, and cross-vendor abstractions |
| Validation | Enforce invariants such as alignment, field ranges, reset values, and provenance completeness |
| Generation | Produce reproducible outputs including SVDs, PACs, HALs, simulators, docs, and reports |

This separation keeps the raw evidence, semantic model, normalization logic, and generated artifacts distinct and auditable.

## IR Schema

At a high level, HAIR describes hardware as a graph of typed semantic objects. A HAIR document represents **one concrete device variant**, with optional imports for shared family- or subsystem-level definitions.

Common object categories include:

- `Device`
- `Peripheral`
- `Register`
- `Field`
- `Interrupt`
- `Timing`
- `Electrical`
- `Provenance`
- `NormalizationMetadata`

Each object is expected to carry enough metadata to support:

1. Traceability back to original source material
2. Cross-vendor normalization
3. Deterministic downstream generation
4. Automated validation

### Schema layout

The repository now includes a layered JSON Schema set under `schema\`:

| File | Role |
| --- | --- |
| `schema/hair.json` | Top-level HAIR document schema |
| `schema/common.json` | Shared primitive types and reusable helpers |
| `schema/provenance.json` | Sources, evidence, reviews, and per-entity provenance |
| `schema/structure.json` | Devices, peripherals, registers, fields, interrupts, memory regions |
| `schema/semantics.json` | Behaviors, operations, state machines, semantic relationships |
| `schema/physical.json` | Clocks, timing, pins, packages, power/reset domains, electrical constraints |
| `schema/normalization.json` | Canonical mappings, naming rules, vendor quirks |
| `schema/validation.json` | Validation rules and profiles |
| `schema/profiles/mcu.json` | Optional MCU/SoC interpretation layer for canonical block classes and topology |
| `schema/profiles/embassy-hal.json` | Optional Embassy-generation profile that binds HAIR hardware facts to crate-generation-ready driver contracts |
| `schema/evidence-manifest.json` | Input manifest schema for extraction workflows |

The core HAIR document can optionally include `profiles.mcuSoc` to classify common embedded concepts such as GPIO ports, timer classes, interrupt controllers, GPIO matrices, IO muxes, flash controllers, and other SoC infrastructure blocks.

When a document is intended to drive Embassy-style HAL generation, it can also include `profiles.embassyHal` to bind supported driver instances to the audited clock/reset, interrupt, DMA, pin-routing, and semantic-operation records they require.

## Provenance and Auditability

Every meaningful element in HAIR carries provenance. That can include:

- Source text excerpts
- Datasheet page numbers
- Vendor header references
- Errata references
- Human annotations
- Confidence scores

This makes regeneration deterministic and reviewable: consumers can always trace a generated field, register, or constraint back to the source material that justified it.

## Extraction Pipeline

HAIR is intended to support spec-driven, LLM-mediated extraction from multiple sources:

- Vendor datasheets (PDF)
- Vendor header files
- HAL implementations
- Reverse-engineered behavior
- Human annotations

The extraction process produces structured HAIR IR blocks together with provenance, making it possible to iteratively refine device models without losing traceability.

### Evidence-first workflow

HAIR extraction is driven by an explicit evidence manifest rather than an implicit pile of source files.

An evidence manifest:

- targets exactly one MCU or SoC variant
- lists the approved source materials to use
- distinguishes local files from remote URIs
- provides an auditable starting point for extraction

This keeps extraction reproducible and makes it easier to challenge, review, and regenerate device descriptions later.

## Repository workflows

The repository includes agent skills under `.github\skills\` that formalize the current workflow:

| Skill | Purpose |
| --- | --- |
| `find-mcu-sources` | Discover and review authoritative source materials, then author an evidence manifest |
| `extract` | Build a HAIR document from an evidence manifest using phased extraction and adversarial review |
| `audit` | Adversarially audit an existing HAIR document against its approved evidence and completeness requirements before downstream generation |
| `maintain` / `evolve` | Support ongoing schema and repository maintenance workflows |

The extraction flow is intentionally conservative:

1. identify one exact target variant
2. assemble an explicit evidence manifest
3. extract a draft HAIR model with provenance
4. audit the extracted HAIR model against the approved evidence and full-device completeness expectations
5. adversarially challenge claims before treating the result as final

## Normalization Rules

HAIR includes a normalization layer so downstream tooling can work from a stable semantic model rather than vendor-specific quirks.

Normalization covers:

- Peripheral naming
- Register grouping
- Field canonicalization
- Cross-vendor peripheral families
- Common abstractions such as UART, SPI, I2C, timers, GPIO, and ADC

This enables tools to reason consistently about equivalent hardware blocks even when vendors describe them differently.

## Generators

HAIR supports deterministic generation of downstream artifacts, including:

- **SVD files**
- **Rust PACs** via `svd2rust`
- **C/C++ headers**
- **Rust, Go, and C HALs**
- **Peripheral simulators** and QEMU-style models
- **Human-readable documentation**
- **Validation reports**

Generators consume HAIR IR blocks and produce reproducible outputs from the same normalized source model.

For SVD generation, HAIR device documents are expected to carry explicit CPU metadata rather than relying on generator defaults. That includes CPU revision, endianness, interrupt priority width, and core feature flags such as MPU/FPU presence and vendor system-timer configuration.

For Embassy-style HAL generation, HAIR can additionally carry an explicit generation contract in `profiles.embassyHal` so generators fail deterministically when the input document falls outside the documented supported subset instead of silently inventing missing bindings.

## Validation

The HAIR model is intended to validate against a set of invariants before generation, including:

- Register alignment
- Field ranges
- Access modes
- Reset values
- Peripheral invariants
- Cross-vendor consistency
- Provenance completeness

The goal is to catch ambiguity and modeling errors early, before they appear in generated code or documentation.

For the first CLI cut, `validate` is narrower: it checks that a HAIR JSON document conforms to the repository schema set. Evaluation of document-defined validation rules remains future tooling work.

## CLI

The current repository CLI is centered around a small set of implemented commands operating on HAIR documents:

```text
validate    Check that a HAIR document matches the schema set
generate    Produce downstream artifacts, including SVD and Embassy-style HAL output
diff        Compare two HAIR document revisions, including git-backed inputs
```

`extract` and `normalize` remain workflow-driven operations rather than CLI promises in this repository. See `docs/cli.md` and `docs/embassy-hal-profile.md` for the current command contract and Embassy-generation profile.

## Example Devices

Reference device models are expected to include:

- CH32V203
- STM32F4
- RP2040
- ESP32 / ESP32-C3

Additional MCU families can be added as extraction pipelines mature.

## Documentation

The main human-oriented design references are:

- `docs/schema.md` — overview of the full layered schema
- `docs/mcu-profile.md` — explanation of the MCU/SoC profile layer
- `docs/embassy-hal-profile.md` — Embassy HAL generation profile and supported first-cut subset
- `docs/cli.md` — CLI scope and first-cut command contract

## Project Philosophy

HAIR is built around a few core principles:

- **Semantic clarity**
- **Deterministic generation**
- **Provenance and auditability**
- **Cross-vendor consistency**
- **LLM-assisted extraction with human verification**
- **Open, extensible schema design**
- **Minimal magic; explicit structure**

## Contributing

Contributions should preserve HAIR's core guarantees:

1. Keep the IR explicit and semantically precise.
2. Preserve provenance for extracted or inferred facts.
3. Prefer deterministic transformations over ad hoc generation logic.
4. Normalize vendor-specific quirks into reusable abstractions where possible.
5. Add or update validation rules when expanding the schema or generators.

Especially valuable contributions include:

- New device extractions
- Improved normalization rules
- Additional generators
- Validation rules and consistency checks
- Better provenance capture and review workflows

## Roadmap

Likely areas of growth for the project include:

- Expanding the core HAIR schema
- Refining the MCU/SoC profile layer for common hardware patterns
- Maturing extraction pipelines for more vendors and device families
- Improving normalization across peripheral families
- Strengthening validation and cross-vendor consistency checks
- Growing the set of deterministic generators
- Building richer documentation and simulator outputs

## License

HAIR is released under the [MIT License](LICENSE).
