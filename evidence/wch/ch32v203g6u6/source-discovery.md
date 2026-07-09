# HAIR Evidence Discovery Report

## Target Device Summary

- **Vendor:** WCH
- **Family / series:** CH32V203
- **Concrete variant:** CH32V203G6U6
- **Package:** QFN28
- **Architecture:** QingKe V4B
- **Scope note:** The user asked for the MCU on the Adafruit QT Py board, not a board-level manifest. Adafruit board collateral is internally inconsistent about `G6` versus `G8`, so the approved target is the exact `CH32V203G6U6` variant because it matches Adafruit's `G6` / 32 KB wording and has an exact-variant community record.

## Search Strategy

1. Grounded the workflow against the local HAIR repository docs and manifest schema.
2. Checked the repository for an existing CH32V203G6 bundle and found none; only a neighboring `CH32V203C8T6` bundle exists.
3. Confirmed the intended target was the MCU itself, not the QT Py board as a board-level hardware description.
4. Confirmed there was no additional local evidence to prefer over web sources.
5. Prioritized official WCH sources first: datasheet, reference manual, and the official `openwch/ch32v20x` SDK/header/startup material.
6. Looked for exact-variant machine-readable cross-checks only after the official sources established family and subgroup fit.
7. Rejected board collateral and neighboring-part artifacts as target-defining evidence when they were conflicting, redundant, or lower-authority than WCH material.

## Local Evidence Inventory

| Candidate source id | Source kind | Local path | Apparent scope | Confidence it matches target |
| --- | --- | --- | --- | --- |
| repo-existing-ch32v203c8t6-bundle | other | `evidence\wch\ch32v203c8t6\...` | Neighboring CH32V203 variant bundle | Low |

## Discovered Source Inventory

| Source ID | Kind | Location | Authority | Scope | Extraction value | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `wch-ch32v203-datasheet` | `datasheet` | https://www.wch.cn/downloads/CH32V203DS0_PDF.html | official | family-level with exact-variant inclusion | high | Best authoritative source for device identity, package families, memory sizing, features, pins, and electrical coverage. Web search evidence indicates CH32V203G6U6 is listed in the covered model set. |
| `wch-ch32fv2x-v3x-rm` | `reference-manual` | https://www.wch.cn/downloads/CH32FV2x_V3xRM_PDF.html | official | family-level | high | Best authoritative register/peripheral source found for CH32V203-family devices. |
| `openwch-ch32v20x-sdk` | `sdk` | https://github.com/openwch/ch32v20x/tree/804daf39a21af99be64c5abe0ea4bdaf361eb2e4 | official | family-level | high | Official SDK repo tying the manuals to code-facing names, examples, startup files, and subgroup assets. |
| `openwch-ch32v20x-header` | `vendor-header` | https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h | official | family-level with exact D6 subgroup mapping | high | Header explicitly places `CH32V203G6` in `CH32V20x_D6` and provides register and interrupt definitions. |
| `openwch-ch32v20x-startup-d6` | `source-code` | https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Startup/startup_ch32v20x_D6.S | official | family-level with exact D6 subgroup mapping | medium | Independent cross-check for D6 subgroup membership and vector ordering. |
| `openwch-ch32v20x-adc-dma-example` | `source-code` | https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/ADC/ADC_DMA/User/main.c | official | family-level narrow functional example | medium | Useful to ground the ADC1 DMA path and direction without relying on community inference. |
| `openwch-ch32v20x-tim-dma-example` | `source-code` | https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/TIM/TIM_DMA/User/main.c | official | family-level narrow functional example | medium | Useful to ground at least one concrete timer DMA route and direction. |
| `qingke-v4-processor-manual` | `other` | https://www.wch.cn/downloads/QingKeV4_Processor_Manual_PDF.html | official | architecture-level | medium | Useful to backstop core/PFIC behavior if the family-level WCH collateral is underspecified. |
| `ch32-rs-ch32v203xx-svd` | `svd` | https://raw.githubusercontent.com/ch32-rs/ch32-rs/9b4ee66500b956bc87fbf83aa28ad245b39ebd15/svd/vendor/CH32V203xx.svd | community | exact-family fit | high | Useful structured metadata gap-filler for register descriptions, reset values, field descriptions, and access metadata when official sources are sparse. |
| `ch32-rs-ch32v203g6u6-yaml` | `generated` | https://raw.githubusercontent.com/ch32-rs/ch32-data/a515903589cfbc342dc6ad0d13c02b4382da5628/data/chips/CH32V203G6U6.yaml | community | exact-variant | medium | Exact-variant cross-check for package, device id, memory sizing, and included family fragments. |

## Recommended Manifest Sources

### Required

| Source ID | Why it belongs | Exact variant or family-level | Falsification result |
| --- | --- | --- | --- |
| `wch-ch32v203-datasheet` | Primary vendor source for identity, package family, memory, feature inventory, pins, and electrical facts. | Family-level with exact-variant inclusion | **Kept.** I tried to disqualify it for being family-level, but no higher-authority CH32V203G6U6-only datasheet was found and WCH appears to publish one family datasheet for the whole CH32V203 line. |
| `wch-ch32fv2x-v3x-rm` | Primary vendor source for registers, fields, clocks, resets, and peripheral semantics. | Family-level | **Kept.** I tried to disqualify it for broad scope, but it remains the best authoritative register/peripheral source available from WCH for this family. |
| `openwch-ch32v20x-sdk` | Official umbrella source for code-facing assets and subgroup-specific support material. | Family-level | **Kept.** Narrower files are useful, but the repo is still the authoritative parent source for header/startup/examples. |
| `openwch-ch32v20x-header` | Strongest machine-readable official source for D6 subgroup membership, IRQ names, peripheral types, and register declarations. | Family-level with exact D6 subgroup mapping | **Kept.** It survived the identity-fit challenge because it explicitly includes `CH32V203G6` in the `CH32V20x_D6` define block. |

### Recommended

| Source ID | Why it belongs | Exact variant or family-level | Falsification result |
| --- | --- | --- | --- |
| `openwch-ch32v20x-startup-d6` | Best official independent cross-check for vector ordering and D6 subgroup fit. | Family-level with exact D6 subgroup mapping | **Kept.** It is partly redundant with the header, but still adds independent evidence for the exact subgroup and startup vector ordering. |
| `openwch-ch32v20x-adc-dma-example` | Best official narrow-scope source found for a grounded ADC1 DMA route and direction. | Family-level narrow functional example | **Kept.** It is not a complete DMA map, but it provides audited direction evidence that the family collateral does not encode elsewhere. |
| `openwch-ch32v20x-tim-dma-example` | Best official narrow-scope source found for a grounded timer DMA route and direction. | Family-level narrow functional example | **Kept.** It only proves a subset, but that subset is valuable for executable Embassy claims. |
| `qingke-v4-processor-manual` | Best official architecture-level backstop for CPU and PFIC behavior. | Architecture-level | **Kept.** It remains the only explicit architecture-level manual in the approved set. |
| `ch32-rs-ch32v203xx-svd` | Best auditable metadata-rich gap-filler found for family-level register metadata. | Exact-family fit | **Kept.** I tried to reject it as unofficial, but it directly fits the CH32V203 family, is commit-pinned, and adds metadata classes not clearly recoverable from the official sources alone. |
| `ch32-rs-ch32v203g6u6-yaml` | Exact-variant cross-check for the user-approved target identity. | Exact-variant | **Kept.** I tried to reject it as redundant with the family SVD, but it adds the exact part number, QFN28 package, memory sizing, and device id for the approved target. |

## Rejected Candidate Sources

| Candidate source | Reason rejected | Safer or higher-authority alternative |
| --- | --- | --- |
| Existing repo bundle `evidence\wch\ch32v203c8t6\...` | Neighboring variant; using it as direct evidence would risk silently importing C8T6-specific package/peripheral assumptions into a G6 target. | Use the G6U6 manifest sources above. |
| Adafruit product / learn pages as target-defining evidence | Useful for explaining the user's request, but lower-authority than WCH for MCU facts and internally inconsistent about `G6` versus `G8`. | Use WCH sources for hardware facts; keep Adafruit only as background context. |
| Separate official errata document | No official CH32V203 errata document was found during discovery. | Use datasheet/manual revisions until a real errata source is found. |
| Official standalone SVD / ATDF / IP-XACT / SystemRDL download | No stable official machine-readable register artifact was found from WCH or `openwch`. | Use the WCH manuals and header, with commit-pinned community gap-fillers when needed. |
| `EVT/CH32V20x_List_EN.txt` from `openwch/ch32v20x` | Helpful for subgroup orientation, but redundant once the approved SDK repo, header, and startup file are already included. | Keep the SDK repo plus the exact header/startup files. |

## Coverage Gaps

- No official errata source identified.
- No stable official standalone SVD / ATDF / IP-XACT / SystemRDL source identified.
- The official datasheet and reference manual appear to be family-level rather than CH32V203G6U6-only.
- The official WCH download pages are wrapper pages; they do not expose much metadata directly through simple fetches.
- Board collateral from Adafruit is internally inconsistent about `G6` versus `G8`, so it cannot be treated as authoritative hardware evidence for the MCU manifest.

## Coverage

- **Examined:** HAIR repo grounding docs, local repo evidence inventory, official WCH download pages, official `openwch/ch32v20x` repository assets, exact-variant `ch32-data`, and exact-family `ch32-rs` SVD material.
- **Method:** repo grounding, local search, target clarification, vendor-first web discovery, commit-pinned GitHub artifact verification, and adversarial source falsification.
- **Excluded:** board-level collateral as authoritative MCU evidence, neighboring-part bundles, and non-pinned or lower-authority mirrors.
- **Limitations:** some official WCH pages are thin wrappers; official documents are family-level; community sources remain useful but non-vendor-authored.

## Follow-Up Recommendations

1. If you later obtain local PDFs or an installed MounRiver Studio tree, add them as local `path` sources so extraction does not depend on live web pages.
2. If exact board BOM identity matters, capture the Adafruit schematic/BOM separately as board collateral rather than folding it into the MCU evidence manifest.
3. During extraction, use the community SVD and YAML only as gap-fillers cross-checked against the official datasheet, reference manual, and header/startup material.
