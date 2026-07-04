# HAIR schema guide

This document explains the current HAIR JSON Schema layout, what each schema file is responsible for, and how the layers fit together.

The intended audience is primarily humans working on HAIR itself: schema authors, extractor authors, generator authors, and reviewers. It is also written to be easy for LLMs to consume as design context.

## Design overview

HAIR is modeled as a **layered semantic IR** rather than as a single flat register-description format.

The main design choices are:

1. **One HAIR document describes one concrete device variant.**
2. **Shared hardware definitions can live in separate HAIR sub-documents and be imported.**
3. **Structural hardware, provenance, semantics, physical facts, normalization, and validation are all first-class layers.**
4. **SVD-like register structure is only one slice of the model, not the whole model.**

## Top-level document model

The top-level schema is `schema/hair.json`.

It defines a HAIR document with these main sections:

- `documentKind`: fixed to `hair`
- `schemaVersion`: version of the HAIR schema itself
- `metadata`: document identity and human metadata
- `imports`: references to shared HAIR documents or fragments
- `provenance`: source materials, evidence, and review data
- `structure`: the concrete device definition
- `semantics`: higher-level behavior and intent
- `physical`: clocks, pins, timing, electrical, and domain data
- `normalization`: canonical names and vendor abstraction rules
- `validation`: machine-readable validation rules and profiles

### One device per document

Each HAIR document is intended to describe **exactly one device variant**.

That constraint is represented by:

- `metadata.role = "device-variant"`
- `structure.device` being a single object

The goal is to keep each top-level document easy to reason about, diff, validate, and generate from.

### Shared sub-documents

To avoid duplication across a family of MCUs or SoCs, a HAIR document may import other HAIR documents or fragments through `imports[]`.

Typical use cases:

- a family-level peripheral library
- a shared register block
- a common clock model
- a shared pinout or package description
- a reusable device-family baseline

The importing document remains the definition of one concrete device variant, but it can reference shared material by identifier.

## Schema files

## `schema/common.json`

Shared primitive and utility types used across the rest of the schema set.

This file includes:

- identifiers and qualified references
- timestamps and URIs
- confidence values
- bit ranges and numeric ranges
- generic expressions
- common named entity metadata
- array shape helpers

Use this file for reusable low-level schema vocabulary. It should stay small, stable, and dependency-light.

## `schema/provenance.json`

The provenance layer models **where facts came from** and **how trustworthy they are**.

It includes:

- `sources`: source artifacts such as datasheets, headers, errata, SVDs, or human notes
- `evidence`: extracted claims or excerpts tied back to a source
- `reviews`: human review or acceptance state
- `provenanceRecord`: per-entity links to source, evidence, review, derivation, and confidence

This layer exists because HAIR is intended to be auditable and regenerable, not just descriptive.

## `schema/structure.json`

The structural layer contains the main hardware object graph for the device.

It currently includes:

- architecture profiles
- device families
- peripheral templates
- the single concrete `device`
- memory regions
- interrupts
- peripherals
- registers
- register clusters
- fields
- enumerated values

This is the layer that most directly overlaps with CMSIS-SVD and similar register-description formats.

### Important structural rule

`structure.device` is the concrete hardware variant described by the current document.

Optional arrays such as `architectures`, `deviceFamilies`, and `peripheralTemplates` exist to hold supporting definitions that belong with the current device document. Shared definitions that should be reused across documents can instead live in imported documents.

### Cross-document structural reuse

Two places are intended for composition:

- `structure.sharedRefs[]`: references to imported shared entities
- `structure.device.extendsRefs[]`: references to family-level or baseline definitions that the current device builds on

These references are intentionally lightweight at the schema level. Resolution, merge behavior, and conflict rules can be defined in HAIR tooling later.

## `schema/semantics.json`

The semantic layer captures meaning that is usually missing from plain register schemas.

It includes:

- `behaviors`: what a register, field, or block means operationally
- `operations`: ordered procedures such as init, unlock, reset, or mode-switch sequences
- `stateMachines`: peripheral or subsystem state transitions
- `relationships`: semantic dependencies such as gating, triggering, or reset relationships

This layer is where HAIR goes beyond SVD-like formats and starts to express hardware intent rather than only hardware layout.

## `schema/physical.json`

The physical layer models non-register facts needed to understand real hardware.

It includes:

- address spaces
- clocks and frequency constraints
- timing constraints
- power domains
- reset domains
- pins and alternate functions
- packages and pin mappings
- electrical constraints
- interrupt controllers

This layer is important because many MCU and SoC facts needed by HALs, simulators, or validation do not fit into register-only formats.

## `schema/normalization.json`

The normalization layer captures how vendor-specific descriptions map into a canonical cross-vendor model.

It includes:

- peripheral families
- canonical mappings
- naming rules
- vendor quirks

This is how HAIR can represent both:

1. the original vendor-facing shape of the hardware, and
2. a normalized semantic model that downstream tools can rely on

## `schema/validation.json`

The validation layer defines explicit rules that can be applied to HAIR documents.

It includes:

- validation rules
- validation profiles

This is intended for checks such as:

- required provenance completeness
- legal field ranges
- register alignment
- consistent reset values
- generation readiness for specific targets such as SVD, HALs, or docs

## How the layers fit together

The layers are meant to answer different questions:

| Layer | Main question |
| --- | --- |
| `common` | What shared primitive types and conventions do we use? |
| `provenance` | Where did this fact come from, and how confident are we? |
| `structure` | What hardware objects exist? |
| `semantics` | What do those hardware objects mean and do? |
| `physical` | What timing, clocking, pin, and electrical facts constrain them? |
| `normalization` | How do vendor-specific names map into canonical concepts? |
| `validation` | What must be true before we trust or generate from the IR? |

Together, these layers make HAIR a semantic IR rather than just a register dump.

## Relationship to SVD and similar formats

HAIR is intended to be a **superset** of SVD-like register specifications, but not a clone of them.

In rough terms:

- `structure` covers most of what SVD expresses
- `provenance` adds source traceability
- `semantics` adds intent and behavior
- `physical` adds clocks, timing, pins, and electrical detail
- `normalization` adds cross-vendor abstraction
- `validation` adds explicit correctness rules

That means SVD export should be treated as a **lowering step** from HAIR, not as the shape that defines HAIR itself.

## Current limitations

The schema set is intentionally broad, but it does not yet define everything about HAIR tooling behavior.

Open design areas include:

- import resolution rules
- merge and override semantics across imported documents
- canonical path and naming conventions for cross-document references
- stronger constraints for semantic expressions
- profile-specific requirements for different generators

Those can evolve without abandoning the current layered structure.

## Practical authoring guidance

When adding to the schema:

1. Put shared low-level vocabulary in `common.json`.
2. Put hardware object shape in `structure.json`.
3. Put meaning and behavior in `semantics.json`.
4. Put clocks, pins, timing, and electrical facts in `physical.json`.
5. Put canonicalization logic in `normalization.json`.
6. Put source traceability in `provenance.json`.
7. Put machine-checkable requirements in `validation.json`.
8. Keep `hair.json` focused on composition and the one-device-per-document model.

## Summary

The current HAIR schema is organized around a simple top-level rule:

**one document describes one concrete device variant**

Everything else exists to support that one device description:

- imports for shared family data
- provenance for auditability
- structure for layout
- semantics for intent
- physical data for real-world constraints
- normalization for cross-vendor consistency
- validation for trust and generator readiness
