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
