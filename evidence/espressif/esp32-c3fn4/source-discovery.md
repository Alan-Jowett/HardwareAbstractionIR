# HAIR Evidence Discovery Report

## Target Device Summary

- **Vendor:** Espressif
- **Family / series:** ESP32-C3
- **Concrete variant:** ESP32-C3FN4
- **Package:** QFN32 (5x5 mm)
- **Architecture:** RISC-V single-core 32-bit MCU/SoC
- **Board context:** Seeed Studio XIAO ESP32C3
- **Scope note:** The manifest targets the MCU/SoC variant `ESP32-C3FN4`, not the board as a whole. Public Seeed pages fetched during discovery confirm `ESP32-C3` plus 4 MB flash board context, while the exact `FN4` package choice was user-selected and is consistent with the Espressif family datasheet's variant table.

## Search Strategy

1. Grounded discovery against the local HAIR repo docs and `schema/evidence-manifest.json`.
2. Confirmed that the request referred to the Seeed Studio XIAO ESP32C3 board context, then resolved the actual extraction target to the concrete MCU variant `ESP32-C3FN4`.
3. Checked the local repository for pre-existing ESP32-C3 / XIAO evidence and found none.
4. Prioritized official Espressif sources first: datasheet, technical reference manual, errata, official SDK, and official machine-readable artifacts.
5. Looked at official Seeed board documentation only to assess whether it materially improved target identity confidence or added extraction-critical evidence.
6. Rejected board-level or community sources that would blur the MCU-vs-board scope or substitute for higher-authority Espressif sources.

## Local Evidence Inventory

None identified.

## Discovered Source Inventory

| Source ID | Kind | Location | Authority | Scope | Extraction value | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `espressif-esp32-c3-datasheet` | `datasheet` | https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf | official | family-level including `ESP32-C3FN4` | high | Best authoritative source for variant table, package/flash options, pinout, IO MUX, GPIO matrix, interrupt matrix overview, memory summary, clocks/resets, electrical, RF, and package facts. |
| `espressif-esp32-c3-trm` | `reference-manual` | https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf | official | family-level including `ESP32-C3FN4` | high | Best authoritative source for register-level peripheral structure, memory map details, interrupt system, clock/reset blocks, low-power/system infrastructure, and routing/topology details. |
| `espressif-esp32-c3-errata` | `errata` | https://docs.espressif.com/projects/esp-chip-errata/en/latest/esp32c3/index.html | official | family-level / revision-sensitive | medium-high | Needed to challenge silicon-specific assumptions during later extraction and falsification. |
| `espressif-esp-idf-esp32c3` | `sdk` | https://github.com/espressif/esp-idf/tree/e9da155a72624fce88b8ef2cf3cde9aee2e6067f | official | family-level | high | Commit-pinned official SDK snapshot. The `components/soc/esp32c3/include/soc` subtree is the main header-level cross-check target. |
| `espressif-esp32c3-svd` | `svd` | https://raw.githubusercontent.com/espressif/svd/be20aa12560889d6125d144cdb48cf615ac17628/svd/esp32c3.svd | official | family-level | medium-high | Official machine-readable artifact for structural scaffolding. Must not be trusted alone because Espressif marks these SVDs as in-progress and potentially incomplete. |
| `seeed-xiao-esp32c3-wiki` | `other` | https://wiki.seeedstudio.com/XIAO_ESP32C3_Getting_Started/ | official | board-level | low-medium | Confirms the board uses `ESP32-C3` with 4 MB flash and exposes board pin mapping, but does not improve full-chip extraction enough to include in the initial MCU manifest. |
| `seeed-xiao-oshw` | `source-code` | https://github.com/Seeed-Studio/OSHW-XIAO-Series/tree/8648ce7eafa33cc5c2707698808b5bafe30737c5 | official | board-level | low-medium | Potential board-level BOM/schematic context, but not required for full-chip ESP32-C3FN4 extraction and not directly needed once the user approved the target identity. |

## Recommended Manifest Sources

### Required

| Source ID | Why it belongs | Exact variant or family-level | Falsification result |
| --- | --- | --- | --- |
| `espressif-esp32-c3-datasheet` | Primary authoritative source for variant identity, packaging, flash-in-package options, pins, clocks/resets, electrical limits, and physical characteristics. | Family-level including exact variant | **Kept.** I tried to reject it for being family-level, but it explicitly lists `ESP32-C3FN4` and is the best authoritative variant/physical source. |
| `espressif-esp32-c3-trm` | Primary authoritative source for peripherals, registers, interrupts, low-power/system blocks, and routing/control infrastructure. | Family-level including exact variant | **Kept.** I tried to reject it as broad family documentation, but no more authoritative register-level source exists for the target. |
| `espressif-esp32-c3-errata` | Required adversarial backstop for revision-specific faults, limitations, and claim challenges. | Family-level / revision-sensitive | **Kept.** I tried to remove it as non-structural, but it is still required for safe falsification of later extraction claims. |
| `espressif-esp-idf-esp32c3` | Official code-facing source for header-level names, memory/base-address macros, and implementation-facing definitions that can cross-check datasheet/TRM claims. | Family-level | **Kept.** I tried to replace it with documentation-only sources, but the SDK remains the strongest official code-facing evidence set. |
| `espressif-esp32c3-svd` | Official machine-readable structural scaffold for peripherals/registers/fields, useful as a cross-check and extraction accelerator. | Family-level | **Kept.** I tried to reject it because Espressif marks it in-progress, but it still adds value when treated as supplemental evidence rather than as the sole source of truth. |

## Rejected Candidate Sources

| Candidate source | Reason rejected | Safer or higher-authority alternative |
| --- | --- | --- |
| `seeed-xiao-esp32c3-wiki` | Official board page, but board-level and incomplete for full-chip extraction. It does not materially improve MCU register/topology coverage and does not explicitly name `ESP32-C3FN4`. | Use the Espressif datasheet/TRM for chip facts. Keep the Seeed page only as background board context outside the manifest. |
| `seeed-xiao-oshw` | Board-level OSHW repo could help only if board-BOM proof becomes necessary, but it is not needed for an MCU-targeted initial manifest and would expand scope toward board extraction. | Use the user-approved target identity plus the Espressif variant table unless a later audit specifically requires board-BOM proof. |
| Community tutorials, teardowns, distributor listings, and mirrors | Lower authority than official Espressif/Seeed sources and not needed for the initial evidence set. | Prefer the official Espressif datasheet, TRM, errata, SDK, and SVD. |

## Coverage Gaps

- Public official Seeed pages fetched during discovery confirm `ESP32-C3` plus 4 MB flash board context, but do not explicitly spell out the `ESP32-C3FN4` package string.
- The manifest therefore relies on the **user-approved exact target identity** plus the Espressif family datasheet's variant table rather than on a directly fetched official board BOM line.
- The official SVD is valuable but explicitly incomplete, so later extraction must cross-check it carefully against the TRM and SDK headers.
- No local PDFs, SDK checkout, or board design files were available in this repo, so the initial manifest depends on live official URLs.

## Coverage

- **Examined:** HAIR repo grounding docs, local repository tree, official Espressif documentation and repositories, official Seeed board documentation.
- **Method:** target clarification, local evidence inventory, official-source-first web discovery, direct source verification, authority filtering, and adversarial source falsification.
- **Excluded:** community tutorials, distributor mirrors, and board-level collateral that did not improve the initial MCU-targeted evidence set enough to justify inclusion.
- **Limitations:** some document version details were easier to confirm from vendor docs/search metadata than from directly rendered PDF text; exact board-to-package confirmation remains weaker than the chip-level Espressif evidence.

## Follow-Up Recommendations

1. If you later obtain a local ESP-IDF checkout, local PDFs, or a directly inspectable XIAO ESP32C3 BOM/schematic file, add them as local `path` sources so extraction does not depend entirely on live URLs.
2. During extraction, treat the SVD as a convenience scaffold only; resolve any discrepancy in favor of the datasheet, TRM, errata, and official SDK headers unless strong evidence says otherwise.
3. If a later audit requires stronger board-to-package grounding, add a directly inspectable official Seeed BOM or schematic artifact as a supplemental source rather than broadening the current manifest silently.
