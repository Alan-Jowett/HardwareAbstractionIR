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
- `profiles`: optional specialization layers built on top of the core schema

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

## `schema/evidence-manifest.json`

This schema defines an **input manifest for extraction workflows**. It
is not itself a HAIR document; it is the structured source list that
drives creation of a HAIR document.

It includes:

- `documentKind`: fixed to `hair-evidence-manifest`
- `schemaVersion`: version of the manifest schema
- `targetDevice`: identity for the single MCU or SoC variant being extracted
- `sources`: the evidence set, with source kind plus either a local path or URI

The goal is to give extractors a deterministic, auditable starting point:
one target device and an explicit list of allowed source materials.

Profile-selection intent for optional generator-facing layers is a
**workflow concern**, not a manifest-schema concern. The manifest still
describes approved evidence and one target device; extraction workflows
should ask the user which optional profiles, if any, are being requested
for the current run.

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

### Interrupt inventory versus peripheral linkage

`structure.device.interrupts[]` is the authoritative interrupt inventory for the concrete device variant.

`peripheral.interruptRefs[]` serves a different purpose: it links a peripheral back to one or more entries in that device-level interrupt inventory so downstream tooling can attribute interrupts to blocks such as `USART1`, `DMA1`, or `WWDG`.

Important consequences:

1. a declared device interrupt is still part of the device even if no peripheral currently links to it
2. generators must not silently drop device interrupts just because `interruptRefs` is absent on a peripheral
3. missing `interruptRefs` is an attribution/completeness problem, not evidence that the interrupt does not exist

This distinction matters for generation readiness. A PAC or SVD that loses a real device interrupt because of missing peripheral linkage is incomplete even when the device-level interrupt entry is present.

The structural `device.cpu` model is also where HAIR carries generator-critical CPU metadata. Compliant device documents now require CPU revision, endianness, interrupt priority width, and core feature flags including `vendorSystemTimerConfig`, so downstream SVD generation does not have to invent missing CPU facts.

### Register overlays and alternate views

HAIR supports **same-offset alternate register views** through `register.alternateOfRef`.

Use this when one logical register location can be interpreted in more than one mode, for example:

- timer capture/compare mode registers with distinct input and output field layouts
- other register overlays where the address, width, and access mode stay the same but the exposed field meaning changes

The intended contract is:

1. represent each view as its own `register` object
2. keep the overlaid registers in the same peripheral or cluster scope
3. keep `offsetBytes`, `widthBits`, access mode, reset metadata, and any array shape aligned across the overlaid views
4. point each alternate view back to the canonical/base view with `alternateOfRef`
5. give each view a distinct `name` for machine-facing generation, and use `displayName` when you need to preserve the shared vendor-facing label

This distinction matters for SVD lowering. A downstream generator may need distinct register identities such as `CHCTLR1_Output` and `CHCTLR1_Input` while still preserving the fact that both describe the same underlying vendor register location `CHCTLR1`.

This contract is intentionally narrower than every possible same-address modeling case. In particular, **peripheral-level shared-base overlays** such as two typed peripheral views at one base address are not implicitly modeled by `alternateOfRef`; they remain a separate topology question that tooling must handle explicitly rather than by reusing the register-overlay rule silently.

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
- generation readiness for specific targets such as SVD, HALs, Embassy HALs, or docs

## `schema/profiles/mcu.json`

The MCU/SoC profile layer specializes the core HAIR model for common embedded hardware patterns.

It adds canonical concepts for things like:

- classic MCU peripheral classes
- SoC infrastructure blocks
- interrupt routing
- GPIO matrix and IO mux behavior
- clock/reset topology
- memory views versus backing stores
- low-power and security-control blocks

This layer is especially useful when modeling both:

1. simpler fixed-function MCUs such as CH32V203-class devices, and
2. richer SoCs such as ESP32-C3-class devices

The profile is optional and appears under `profiles.mcuSoc` in the top-level document.

For generator-facing MCU work, `profiles.mcuSoc` now also carries named topology records that other profiles can reference directly, including:

- interrupt sources and routes
- clock and reset bindings
- DMA channels and DMA routes
- pin-routing records that tie pins, signals, remap controls, and electrical constraints together

These remain hardware-topology facts, not Embassy-specific policy.

## `schema/profiles/embassy-hal.json`

The Embassy HAL profile is an optional generation-oriented layer mounted at `profiles.embassyHal`.

It exists to describe how a concrete HAIR document lowers into an Embassy-style HAL crate **without** pushing Embassy-specific structure into the core hardware layers.

It currently carries:

- crate/package metadata for generated output
- driver-instance declarations for supported peripheral blocks
- explicit references to the clock/reset, interrupt, DMA, pin-routing, operation, and state-machine records each generated driver depends on
- capability tags that are generator-facing rather than raw hardware facts

The profile intentionally does **not** define one universal fixed Rust
method list per `driverKind`. Instead, the emitted API surface is expected
to be derived from the referenced topology and semantic records that the
generator can lower into real register-level code for the concrete
document under generation.

The intended division of responsibility is:

1. core layers + `profiles.mcuSoc` describe the hardware
2. `profiles.embassyHal` describes a deterministic generator contract over that hardware

When an extraction or audit workflow is asked to target Embassy-ready
output, it should explicitly ask the user whether `profiles.embassyHal`
is in scope. If the answer is yes, the workflow must also ask which
supported driver set is in scope and treat the
supporting `profiles.mcuSoc` topology, relevant semantic operations/state
machines, and referenced physical/topology records as required for the
requested profile scope rather than as optional nice-to-have enrichment.

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
| `profiles.mcuSoc` | How should a device be interpreted as a canonical MCU/SoC architecture? |
| `profiles.embassyHal` | How should this specific HAIR document lower into an Embassy-style HAL crate? |

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
- `profiles.mcuSoc` adds an opinionated MCU/SoC interpretation layer
- `profiles.embassyHal` adds an optional generator contract for Embassy-style HAL output

That means SVD export should be treated as a **lowering step** from HAIR, not as the shape that defines HAIR itself.

## Current limitations

The schema set is intentionally broad, but it does not yet define everything about HAIR tooling behavior.

Open design areas include:

- import resolution rules
- merge and override semantics across imported documents
- canonical path and naming conventions for cross-document references
- stronger constraints for semantic expressions
- profile-specific requirements for different generators
- richer canonical vocabularies for MCU/SoC block classes

Those can evolve without abandoning the current layered structure.

For the initial CLI, `hair validate` is limited to schema conformance against `schema/hair.json` and the layered subschemas. The declarative `validation` layer remains the place to model richer invariants and generator preconditions, including profile- and target-specific rules such as `generatorTargets: ["embassy-hal"]`, but executing those rules is future tooling work until implemented by repository tooling.

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
9. Put canonical MCU/SoC interpretation rules in `schema/profiles/mcu.json`.
10. Put Embassy-specific generation bindings in `schema/profiles/embassy-hal.json`.

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
- profiles for domain-specific interpretation layers
