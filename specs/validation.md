# HAIR Repository Validation Baseline

## Purpose

This document defines how to verify the current HAIR repository baseline using
existing commands, reference artifacts, generated outputs, and review
workflows.

The validation baseline is intentionally split between:

1. **automated checks** provided by the Rust crate and its tests
2. **artifact-level checks** using reference device bundles under `evidence\`
3. **workflow reviews** for evidence approval, extraction claims, and audit
   verdicts

Validation promises only what the repository currently supports.

## 1. Validation principles

1. Validate the schema contract before trusting generation outputs.
2. Validate generators against real reference artifacts, not only synthetic
   fixtures.
3. Preserve a manual review gate where repository workflows are intentionally
   adversarial or evidence-dependent.
4. Treat unsupported or ambiguous inputs as failures or review blockers rather
   than as acceptable silent degradation.

## 2. Automated repository checks

### V-001 Rust regression suite

**Purpose:** verify CLI behavior, schema-loader behavior, generator
invariants, and profile-specific failure cases.

**Command**

```powershell
cargo test
```

**Expected result**
- The test suite passes.
- The suite covers schema validation, structural diffing, SVD lowering, and
  Embassy-generation contracts and failure paths.

### V-002 HAIR schema validation

**Purpose:** verify that a HAIR document conforms to the repository schema
set.

**Command**

```powershell
cargo run -- validate evidence\st\stm32f405rgt6\hair.json
```

**Expected result**
- The command succeeds.
- Failure output, when present, identifies the failing HAIR path.

### V-003 Structural diff behavior

**Purpose:** verify that diff is structural and stable.

**Command**

```powershell
cargo run -- diff evidence\st\stm32f405rgt6\hair.json evidence\st\stm32f405rgt6\hair.json
```

**Expected result**
- The command succeeds and reports no differences for identical inputs.

### V-004 SVD generation path

**Purpose:** verify that repository-managed SVD lowering still operates on a
reference HAIR document.

**Command**

```powershell
cargo run -- generate svd evidence\st\stm32f405rgt6\hair.json --output <svd-output-path>
```

**Expected result**
- The command writes one SVD document.
- Generation preserves the declared device interrupt inventory and required CPU
  block metadata when the input carries the necessary HAIR data.

### V-005 Embassy generation path

**Purpose:** verify that repository-managed Embassy lowering still operates on
an in-scope reference HAIR document.

**Command**

```powershell
cargo run -- generate embassy evidence\texas-instruments\lm3s6965\hair.json --output-dir <crate-output-dir>
```

**Expected result**
- The command writes a Rust crate rooted at `<crate-output-dir>`.
- The emitted crate contains `Cargo.toml`, `src\lib.rs`, `src\metadata.rs`,
  and generated driver modules justified by the reference document's profile
  scope.

## 3. Artifact-level validation

### V-006 Evidence-manifest validation

**Purpose:** verify that extraction inputs remain explicit and single-target.

**Artifacts**
- `evidence\st\stm32f405rgt6\evidence-manifest.json`
- `evidence\texas-instruments\lm3s6965\evidence-manifest.json`
- `evidence\wch\ch32v203c8t6\evidence-manifest.json`

**Check**
- Each manifest names one concrete target device.
- Each manifest carries at least one approved source.
- Source entries are classified with repository-supported source kinds.

### V-007 Reference HAIR document review

**Purpose:** verify that example HAIR documents reflect the repository model
in realistic device bundles.

**Artifacts**
- `evidence\st\stm32f405rgt6\hair.json`
- `evidence\texas-instruments\lm3s6965\hair.json`
- `evidence\wch\ch32v203c8t6\hair.json`

**Check**
- Each document represents one concrete device variant.
- Provenance, structure, and any included profiles remain internally
  consistent.
- Example documents exercise different portions of the schema and generator
  surface.

### V-008 Generated-output inspection

**Purpose:** verify that reference generated artifacts remain aligned with the
repository contracts.

**Artifacts**
- `evidence\st\stm32f405rgt6\generated\stm32f405rgt6.svd`
- `evidence\st\stm32f405rgt6\generated\embassy\`
- `evidence\st\stm32f405rgt6\generated\embassy-smoke\`
- `evidence\texas-instruments\lm3s6965\embassy-out\`
- `evidence\texas-instruments\lm3s6965\embassy-smoke\`

**Check**
- SVD output is present for a reference device that exercises the SVD path.
- Embassy output directories contain a complete generated crate layout.
- Smoke-project directories consume generated crates using normal Rust package
  boundaries rather than ad hoc post-processing.

## 4. Workflow validation

### V-009 Source-discovery review

**Purpose:** verify that source discovery remains evidence-first and
single-target.

**Artifacts**
- `evidence\...\source-discovery.md`
- `schema\evidence-manifest.json`
- `.github\skills\find-mcu-sources\SKILL.md`

**Check**
- Discovery reports justify the chosen evidence set.
- Approved sources are distinguished from rejected candidates and coverage
  gaps.
- Manifest content matches the documented discovery outcome.

### V-010 Extraction review

**Purpose:** verify that extraction remains provenance-first and scoped to
approved evidence.

**Artifacts**
- `evidence\...\extraction-report.md`
- `evidence\...\hair.json`
- `.github\skills\extract\SKILL.md`

**Check**
- Extraction reports identify target scope, source inventory, discovered
  hardware areas, metadata coverage, rejected claims, and limitations.
- HAIR documents align with the extraction scope claimed in the report.

### V-011 Audit review

**Purpose:** verify that full-device or generator-readiness claims survive
adversarial challenge.

**Artifacts**
- `evidence\wch\ch32v203c8t6\audit-report.md`
- `.github\skills\audit\SKILL.md`

**Check**
- Audit reports state approved evidence, completeness findings, unsupported
  claims, root-cause classification, and a final verdict.
- A document is not treated as downstream-ready solely because it is
  schema-valid.

### V-012 Governing-spec review

**Purpose:** verify that repository governance changes stay traceable.

**Artifacts**
- `specs\requirements.md`
- `specs\design.md`
- `specs\validation.md`
- `.github\skills\bootstrap\SKILL.md`
- `.github\skills\evolve\SKILL.md`
- `.github\skills\maintain\SKILL.md`

**Check**
- Requirements, design, and validation remain mutually traceable.
- Specification changes occur before implementation changes that would alter
  repository guarantees.
- Maintenance work treats missing or drifting governing specs as a first-class
  problem.

## 5. Optional smoke validation

### V-013 QEMU smoke execution for the LM3S6965 Embassy example

**Purpose:** provide an executable sanity check for the LM3S6965 Embassy smoke
project.

**Command**

```powershell
powershell -ExecutionPolicy Bypass -File evidence\texas-instruments\lm3s6965\embassy-smoke\run-qemu-smoke.ps1
```

**Expected result**
- The smoke firmware builds for `thumbv7m-none-eabi`.
- QEMU output includes the expected UART0 smoke confirmation text.

**Note**
- This is an environment-dependent smoke check, not a universal repository
  precondition.

## 6. Requirement traceability matrix

| Requirement | Validation activities |
| --- | --- |
| RQ-001 | V-002, V-006, V-007 |
| RQ-002 | V-007, V-012 |
| RQ-003 | V-001, V-002, V-007 |
| RQ-004 | V-001, V-002 |
| RQ-005 | V-007, V-010, V-011 |
| RQ-006 | V-006, V-009, V-010 |
| RQ-007 | V-009, V-010, V-011, V-012 |
| RQ-008 | V-001, V-007, V-010 |
| RQ-009 | V-001, V-004, V-005, V-008, V-011 |
| RQ-010 | V-001, V-002, V-003, V-004, V-005 |
| RQ-011 | V-001, V-002, V-011 |
| RQ-012 | V-001, V-004, V-008 |
| RQ-013 | V-001, V-005, V-008, V-010, V-011 |
| RQ-014 | V-001, V-003 |
| RQ-015 | V-001, V-002, V-003, V-004, V-005 |

## 7. Current validation limits

The current validation baseline intentionally does not claim:

1. execution of declarative `validation.rules` from `hair validate`
2. fully automated verification of cross-document import resolution or merge
   semantics
3. automatic proof that every schema-valid document is generation-ready
4. repository-managed automated checks for unimplemented generator families
5. replacement of evidence review and adversarial audit with schema checks
   alone
