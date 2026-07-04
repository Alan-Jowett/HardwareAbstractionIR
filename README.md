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

At a high level, HAIR describes hardware as a graph of typed semantic objects. Common object categories include:

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

## Validation

Before generation, HAIR validates the semantic model against a set of invariants, including:

- Register alignment
- Field ranges
- Access modes
- Reset values
- Peripheral invariants
- Cross-vendor consistency
- Provenance completeness

The goal is to catch ambiguity and modeling errors early, before they appear in generated code or documentation.

## CLI

The HAIR toolchain is centered around a small set of commands operating on HAIR IR blocks:

```text
extract     Build IR from source materials
normalize   Apply canonical semantic rules
validate    Run invariants and consistency checks
generate    Produce SVDs, PACs, HALs, simulators, and docs
diff        Compare IR revisions or vendor updates
```

## Example Devices

Reference device models are expected to include:

- CH32V203
- STM32F4
- RP2040
- ESP32

Additional MCU families can be added as extraction pipelines mature.

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
- Maturing extraction pipelines for more vendors and device families
- Improving normalization across peripheral families
- Strengthening validation and cross-vendor consistency checks
- Growing the set of deterministic generators
- Building richer documentation and simulator outputs

## License

HAIR is released under the [MIT License](LICENSE).
