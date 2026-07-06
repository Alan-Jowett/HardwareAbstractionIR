# HAIR Audit Report

## Device Summary

- **Target:** WCH CH32V203C8T6
- **Manifest target:** `CH32V203C8T6`
- **HAIR title:** `WCH CH32V203C8T6`
- **Architecture:** QingKe V4B / RV32IMACXW
- **Audit focus:** full-device coverage and downstream-generation readiness for metadata-rich outputs

## Input Inventory

| Artifact | Path | Role | Target identity | Notes |
| --- | --- | --- | --- | --- |
| HAIR JSON | `evidence\wch\ch32v203c8t6\hair.json` | Audit target | `WCH CH32V203C8T6` | Schema-valid HAIR document. Contains 8 provenance sources, 27 evidence records, 33 peripherals, 397 registers, 2373 fields, 47 interrupts, and a populated physical layer. |
| Evidence manifest | `evidence\wch\ch32v203c8t6\evidence-manifest.json` | Approved evidence scope | `WCH CH32V203C8T6` | Schema-valid manifest with 8 approved sources. Mixes official WCH family-level sources with explicitly approved community metadata gap-fillers. |
| Source-discovery report | `evidence\wch\ch32v203c8t6\source-discovery.md` | Source-approval context | `WCH CH32V203C8T6` | Documents why the community SVD/YAML were approved only as auditable gap-fillers rather than silent replacements. |
| Extraction report | `evidence\wch\ch32v203c8t6\extraction-report.md` | Claimed extraction status | `WCH CH32V203C8T6` | Useful as a claim set to challenge, but not treated as evidence by itself. |

## Approved Evidence Summary

| Source ID | Kind | Authority | Audit role |
| --- | --- | --- | --- |
| `wch-ch32v20x-30x-datasheet` | datasheet | official | Device identity, package, memories, clocks, voltage range, pins |
| `wch-ch32fv2x-v3x-rm` | reference-manual | official | Peripheral/register structure, timer/CAN adjudication, clocks/resets |
| `openwch-ch32v20x-sdk` | sdk | official | Linker/build cross-checks, core headers |
| `openwch-ch32v20x-header` | vendor-header | official | Register structs, offsets, bit definitions, D6 subgroup membership |
| `openwch-ch32v20x-startup-d6` | source-code | official | Interrupt vector ordering cross-check |
| `qingke-v4-processor-manual` | other | official | Core/exception architecture context |
| `ch32-rs-ch32v203xx-svd` | svd | approved community gap-filler | Exact-family metadata enrichment for descriptions, resets, and some field access |
| `ch32-rs-ch32v203c8t6-yaml` | generated | approved community gap-filler | Exact-variant fit cross-check for community metadata reuse |

## Evidence Validation Findings

1. **Provenance source IDs stay inside the approved manifest scope.** All 8 `provenance.sources[]` IDs appear in the approved manifest, and all `sourceRefs` / `evidenceRefs` resolve.
2. **The prior startup source-kind mismatch has been repaired.** The approved manifest and `hair.json` now both classify `openwch-ch32v20x-startup-d6` as `source-code`.
3. **Community metadata provenance stays within the approved policy boundary.** The imported description/reset/access metadata cites only the two approved community sources and does not introduce unapproved source IDs.
4. **The document survives the “wrong source” challenge for current imported metadata.** On exact-match overlap, there are no remaining cases where the community SVD provided a description or reset value for a matched register but the HAIR omitted it, and no remaining cases where the community SVD provided a description or access value for a matched field but the HAIR omitted it.

## Structural Completeness Findings

1. **No shell-only MMIO peripherals were found.** All 33 modeled peripherals carry `registers[]`; none are placeholder shells.
2. **Peripheral-level structure is broadly complete for the approved scope.** All 33 peripherals have descriptions and address blocks.
3. **Register-bearing depth is present across the modeled device.** The HAIR contains 397 registers and 2373 fields, with timer/CAN adjudication notes carried in provenance, explicit timer alternate-register overlays for `CHCTLR1` / `CHCTLR2`, and substantial USB prose/enum enrichment.
4. **Cluster-level prose is absent.** The document contains 4 register clusters (`CAN1.TxMailBox`, `CAN1.FIFOMailBox`, `CAN1.FilterRegister`, `DMA1.Channel`) and 0 cluster descriptions.
5. **Physical supporting structure is present.** The physical layer includes 6 clocks, 1 timing constraint, 1 power domain, 1 interrupt controller, 44 pins, and 1 package.

## Metadata Coverage and Gaps

### Metadata present in approved evidence and extracted

| Metadata class | Audit result |
| --- | --- |
| Peripheral descriptions | Present on 33 / 33 peripherals. |
| Peripheral address-block coverage | Present on 33 / 33 peripherals. |
| Register access modes | Present on 397 / 397 registers. |
| Register descriptions | Present on 393 / 397 registers. This now includes the timer/CAN/ADC/AFIO/DMA alias/template cases, explicit timer overlay views, and substantial USB prose imported conservatively from the approved transitive YAML fragments. |
| Register reset values | Present on 303 / 397 registers. For exact community-SVD register matches, no additional reset values remain unimported. |
| Field descriptions | Present on 1945 / 2373 fields. This now includes the safe alias/template import cases, explicit timer overlay fields, added CAN mailbox/filter and DMA channel template fields, and most USB device/host prose. |
| Field access modes | Present on 254 / 2373 fields. Field-level access coverage remains sparse because the approved community sources annotate only a subset of fields. |
| Field enumerated values | Present on 25 / 2373 fields, concentrated in the imported USB metadata. |

### Metadata present in approved evidence but not extracted

| Metadata class | Gap | Why it remains |
| --- | --- | --- |
| Register metadata on approved community sources | Remaining unmapped register metadata is now concentrated in `SPI1.I2SPR`, `FLASH.ACTLR`, `USBFSH.HOST_RX_CTRL`, and `USBFSH.HOST_TX_LEN`. | The first two still lack a clean approved-source import path; `HOST_RX_CTRL` still needs alias reconciliation from the transitive `RX_CTRL` fragment, and `HOST_TX_LEN` has no matching transitive prose record. |
| Field metadata on approved community sources | Remaining omissions are now concentrated in field-topology mismatches such as aggregate-vs-per-bit CAN modeling, AFIO/EXTI/RCC naming drift, timer/ADC families, and a small residual `USBFSH` host-side tail. | The approved metadata exists only partially, and importing the rest would require further field reshaping or richer source material rather than conservative enrichment. |
| USB community metadata | Mostly imported. | The approved `CH32V203C8T6.yaml -> FV2x_V3x_USBD.yaml + FV2x_V3x_USBFS.yaml -> otg_v2.yaml` path now fills all `USBFSD` register/field descriptions plus most `USBFSH` register/field descriptions and 25 USB enum-bearing fields without reshaping the dual-overlay topology. |

### Metadata not confidently recoverable from approved evidence

| Metadata class | Audit result |
| --- | --- |
| Cluster descriptions | Not confidently recoverable without further manual reconciliation. |
| Register reset masks | 0 / 397 present; the approved community SVD does not provide a reliable importable reset-mask layer here. |
| Remaining field access modes | 2119 / 2373 fields still lack field-level access semantics. Official sources examined so far do not provide a complete field-permission model, and the community SVD only annotates a subset. |
| Field enumerated values | 25 enumerated sets are now present, but almost entirely within the imported USB fragments; broad non-USB enum coverage is still absent. |

## Unsupported or Weakly Supported Claims

1. **Any claim that this HAIR is complete for metadata-rich full-device downstream generation is still too strong.** The document still omits approved-source metadata where the remaining work requires topology reshaping rather than conservative enrichment.
2. **USB overlay metadata remains only a partial gap now.** The current HAIR safely imports the approved transitive USB device/host prose and enum fragments for `USBFSD` and most of `USBFSH`, but a small host-side residual set still lacks a clean mapped source.
3. **Cluster-level completeness is weakly supported at best.** The document has cluster structure, but no cluster descriptions anywhere.

## Root-Cause Classification

| Major gap | Likely root cause |
| --- | --- |
| Remaining unmapped register metadata (`SPI1.I2SPR`, `FLASH.ACTLR`, and the `USBFSH` host-side tail) | ambiguous evidence, topology mismatch, source sparsity |
| Remaining unmapped community field metadata | ambiguous evidence, topology mismatch |
| Residual USB host-side metadata gap | ambiguous evidence, source sparsity |
| Missing cluster descriptions | ambiguous evidence, missing source evidence |
| Missing reset masks | missing source evidence |
| Broad missing field enumerated values outside USB | missing source evidence |
| Prior startup source kind mismatch | repaired |

## Rejected Candidate Findings

| Candidate finding | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| The HAIR document is invalid or references unapproved provenance sources. | Rejected. Both the HAIR document and manifest validate, and all provenance references resolve within the approved source/evidence inventory. | Successful HAIR validation, successful manifest validation, and reference-resolution check. |
| The document still contains shell-only MMIO peripherals. | Rejected. Every modeled peripheral carries a grounded register block. | Structural summary found 33 peripherals and none without `registers[]`. |
| Straightforward conservative alias/template community metadata is still being silently omitted. | Rejected. The current HAIR now imports the defensible timer/CAN/ADC/AFIO/DMA alias/template cases in addition to the earlier exact matches. | Post-fix inspection of the repaired `hair.json` plus successful validation/generation on the updated document. |
| The document is blocked by a schema/generator mismatch rather than extraction quality. | Rejected for the current revision. The recent schema alignment fixes allow both HAIR validation and SVD generation to succeed. | Successful HAIR validation and successful SVD generation from the current document. |

## Coverage

- **Examined**: `hair.json`, `evidence-manifest.json`, `source-discovery.md`, `extraction-report.md`, repo schema/docs grounding, cached `CH32V203xx.svd`, and current generator behavior.
- **Method**: schema validation, manifest validation, provenance-resolution checks, structural/metadata counting, exact-match overlap analysis against the approved community SVD, and generation-readiness spot checks.
- **Excluded**: full manual reread of every approved PDF page and non-approved external sources.
- **Limitations**: this audit challenged the current IR against approved evidence inventories and cached community artifacts, but did not repeat a full page-by-page manual extraction from the family manuals.

## Limitations

- The official WCH evidence remains family-level for several critical areas, so some metadata disputes collapse into “manual adjudication required” rather than a clean yes/no contradiction.
- The report now reflects the repaired document state, but it is still not a fresh from-scratch rerun of the audit workflow.
- USBFS device/host overlays now have grounded registers plus substantial imported prose and enum coverage, but `USBFSH` still retains a small host-side alias/sparsity gap because the current dual-overlay HAIR model does not map 1:1 onto every community `USBD` / `USBFS` record.

## Final Verdict

**INCOMPLETE**

The document is structurally credible, schema-valid, and generation-capable, and it closes the earlier conservative register-level reconciliation gaps. It still does **not** fully survive a metadata-rich downstream-readiness audit because the remaining omissions are now concentrated in field-topology reshaping cases, the residual `USBFSH` host-side prose gaps, `SPI1.I2SPR`, `FLASH.ACTLR`, missing cluster prose, reset masks, and sparse non-USB enum coverage. That makes the current IR a stronger and more complete auditable draft, but still not a fully metadata-complete end state.
