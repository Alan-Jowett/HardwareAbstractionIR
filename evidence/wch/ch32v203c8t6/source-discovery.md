# HAIR Evidence Discovery Report

## Target Device Summary

- **Vendor:** WCH
- **Family / series:** CH32V203
- **Concrete variant:** CH32V203C8T6
- **Package:** LQFP48
- **Architecture:** QingKe V4B
- **Scope note:** WCH appears to publish family-level CH32V20x/30x documents rather than a CH32V203C8T6-only datasheet or reference manual.

## Search Strategy

1. Grounded the workflow against the local HAIR schema and extraction skill documents.
2. Confirmed the exact target variant with the user before broad source discovery.
3. Checked the local repository for pre-existing CH32V203 evidence artifacts and found the existing manifest, source-discovery report, HAIR extraction outputs, and a local `CH32F203xx.svd` community comparison file.
4. Prioritized official WCH sources first: product/download pages, the official `openwch` GitHub repository, and raw vendor-published headers/startup files.
5. Looked specifically for datasheet, reference manual, errata, SDK/header material, and any official machine-readable register descriptions.
6. After the stricter extract-skill rerun exposed missing register prose/reset/enum metadata, searched for auditable community gap-fillers that could provide richer machine-readable metadata without pretending to be official WCH artifacts.
7. Rejected sources that were missing, indirect, board-specific, or lower-authority than a vendor-published alternative.

## Local Evidence Inventory

None identified.

## Discovered Source Inventory

| Source ID | Kind | Location | Authority | Scope | Extraction value | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `wch-ch32v20x-30x-datasheet` | `datasheet` | https://www.wch-ic.com/downloads/CH32V20x_30xDS0_PDF.html | official | family-level | high | Best authoritative source found for exact variant identity, feature set, memories, packages, pins, and electrical/timing facts. |
| `wch-ch32fv2x-v3x-rm` | `reference-manual` | https://www.wch-ic.com/downloads/CH32FV2x_V3xRM_PDF.html | official | family-level | high | Best authoritative source found for registers, fields, peripheral blocks, interrupts, clocks, and resets. |
| `openwch-ch32v20x-sdk` | `sdk` | https://github.com/openwch/ch32v20x/tree/804daf39a21af99be64c5abe0ea4bdaf361eb2e4 | official | family-level | high | Official WCH repository with SDK/examples and package-specific project assets. Useful for cross-checking names and implementation-facing details. |
| `openwch-ch32v20x-header` | `vendor-header` | https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h | official | family-level with exact D6 subgroup mapping | high | Header explicitly maps `CH32V20x_D6` to `CH32V203C8`, which is the strongest variant-near machine-readable source found. |
| `openwch-ch32v20x-startup-d6` | `source-code` | https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Startup/startup_ch32v20x_D6.S | official | family-level with exact D6 subgroup mapping | medium | Useful cross-check for vector table ordering and handler names; narrower value than the header or manuals. |
| `qingke-v4-processor-manual` | `other` | https://www.wch-ic.com/downloads/QingKeV4_Processor_Manual_PDF.html | official | architecture-level | medium | Useful when the family docs leave core/exception details underspecified. |
| `ch32-rs-ch32v203xx-svd` | `svd` | `https://raw.githubusercontent.com/ch32-rs/ch32-rs/9b4ee66500b956bc87fbf83aa28ad245b39ebd15/svd/vendor/CH32V203xx.svd` | community | family-level exact-family fit | high | Community-maintained CH32V203 family SVD with rich register/field descriptions, many reset values, and field access annotations; useful as a metadata gap-filler, not as an official replacement. |
| `ch32-rs-ch32v203c8t6-yaml` | `generated` | `https://raw.githubusercontent.com/ch32-rs/ch32-data/a515903589cfbc342dc6ad0d13c02b4382da5628/data/chips/CH32V203C8T6.yaml` | community | exact-variant | medium | Community-curated exact-variant YAML that names CH32V203C8T6 directly and records family/peripheral composition; useful as a structured cross-check and provenance trail for community-derived metadata. |

## Recommended Manifest Sources

### Required

| Source ID | Why it belongs | Exact variant or family-level | Falsification result |
| --- | --- | --- | --- |
| `wch-ch32v20x-30x-datasheet` | Primary vendor source for device identity, package/pinout, electrical limits, clocks, and feature inventory. | Family-level | **Kept.** I tried to disqualify it for being family-level, but WCH does not appear to publish a higher-authority C8T6-only datasheet. |
| `wch-ch32fv2x-v3x-rm` | Primary vendor source for registers, fields, peripherals, and reset/clock behavior. | Family-level | **Kept.** I tried to disqualify it for broad scope, but it is still the best authoritative register/peripheral source available from WCH. |
| `openwch-ch32v20x-sdk` | Official WCH SDK corpus that links the manuals to code-facing names, examples, and package-specific assets. | Family-level | **Kept.** I tried to replace it with narrower files only, but the repo remains the authoritative umbrella source for the header/startup files and examples. |
| `openwch-ch32v20x-header` | Strongest machine-readable vendor source found for IRQ names, register structs, and D6 subgroup membership including `CH32V203C8`. | Family-level with exact D6 subgroup mapping | **Kept.** It survived the identity-fit challenge because the header comment explicitly names `CH32V203C8`. |

### Recommended

| Source ID | Why it belongs | Exact variant or family-level | Falsification result |
| --- | --- | --- | --- |
| `openwch-ch32v20x-startup-d6` | Best official source found for interrupt vector ordering and handler naming. | Family-level with exact D6 subgroup mapping | **Kept.** It is somewhat redundant with the header, but still adds independent evidence for vector order. |
| `qingke-v4-processor-manual` | Useful architecture-level backstop for CPU/exceptions if the family docs are thin or ambiguous. | Architecture-level | **Kept.** I tried to remove it as redundant, but it remains the only explicit architecture manual referenced by the official openwch datasheet README. |
| `ch32-rs-ch32v203xx-svd` | Best auditable metadata-rich gap-filler found for register descriptions, many reset values, field descriptions, and some access annotations. | Family-level exact-family fit | **Kept.** I tried to reject it as unofficial, but it directly fits the CH32V203 family, is commit-pinned, and clearly adds the exact metadata classes missing from the official-source-only pass. |
| `ch32-rs-ch32v203c8t6-yaml` | Structured exact-variant community cross-check that records the concrete C8T6 package/device identity and composition. | Exact-variant | **Kept.** I tried to reject it as redundant with the SVD, but it adds exact-variant scoping and an explicit statement of its upstream source mix (official datasheets, MRS SVDs, and openwch material). |

## Rejected Candidate Sources

| Candidate source | Reason rejected | Safer or higher-authority alternative |
| --- | --- | --- |
| Separate official errata document | No official CH32V203 errata document was found during discovery. | Use datasheet/manual revisions until a real errata source is found. |
| Official standalone SVD / ATDF / IP-XACT / SystemRDL download | No stable official standalone machine-readable register artifact was found from WCH or `openwch`, even though community projects report that MounRiver Studio ships internal SVDs. | Use the WCH manuals plus official header/startup code, and optionally the approved community gap-fillers below. |
| Claimed GitHub datasheet PDF mirror under `openwch/ch32v20x/Datasheet/...` | The repository tree does not actually contain the claimed PDF, so it is not auditable as a real source. | Use the official WCH download pages instead. |
| `SCHPCB/CH32V203C8T6-R0.pdf` board/example schematic | Official, but board-specific and lower-value for core MCU extraction than the manuals and headers. | Keep it out of the initial manifest unless a later extraction gap specifically needs board-context clues. |
| `ch32-rs-ch32f203xx-svd` | Rich metadata, but it targets the Cortex-M CH32F203 family rather than the QingKe V4B CH32V203 family. | Prefer the exact-family `ch32-rs-ch32v203xx-svd` instead. |

## Coverage Gaps

- No official errata source identified yet.
- No stable official standalone SVD / ATDF / IP-XACT / SystemRDL source identified yet.
- Family-level manuals may still leave some package-specific nuances implicit rather than explicit.
- Even with the new community gap-fillers, provenance for richer metadata such as reset values and field descriptions still traces back through community curation rather than directly to a vendor-published machine-readable artifact.

## Coverage

- **Examined:** HAIR repo grounding docs, WCH download references, official `openwch/ch32v20x` repository, official vendor header/startup files, `ch32-rs/ch32-rs`, and `ch32-rs/ch32-data`.
- **Method:** schema grounding, local repo search, targeted vendor-first web discovery, metadata-gap-driven community discovery, authority filtering, and adversarial source falsification.
- **Excluded:** low-authority mirrors without a stronger upstream story, board-only collateral, and any source that could not be verified as official or actually present.
- **Limitations:** WCH download pages are JavaScript-heavy, so the page wrapper is easier to verify than the PDF metadata itself; the official docs appear to be family-level rather than exact-part-only; the new community sources are useful but not vendor-authored.

## Follow-Up Recommendations

1. If you later obtain local PDFs or an installed MounRiver Studio tree, add them as local `path` sources so extraction does not depend on live vendor URLs.
2. During extraction, use the community SVD and YAML only as gap-fillers cross-checked against the official datasheet, RM, and openwch header/startup material.
3. Re-run discovery if WCH publishes an official errata document or a standalone machine-readable register description for CH32V203.
