# HAIR CLI

This document defines the first repository-managed CLI surface for HAIR.

## Scope

The initial CLI covers three commands:

```text
hair validate <input>
hair generate svd <input> [--output <path>]
hair diff <left> <right>
```

`extract` and `normalize` remain workflow-driven operations implemented through repository skills and review processes. They are not part of the initial CLI contract.

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
- `diff` compares HAIR documents as repository artifacts; it does not redefine HAIR semantics.

The `validation` layer in HAIR remains important, but the first CLI cut only enforces schema conformance. Executing declarative validation rules is future work.

Because `generate svd` now depends on CPU metadata that CMSIS-SVD requires, compliant HAIR device documents must include `structure.device.cpu.revision`, `structure.device.cpu.endianness`, `structure.device.cpu.interruptPriorityBits`, and `structure.device.cpu.featureFlags.{mpuPresent,fpuPresent,vendorSystemTimerConfig}`.
