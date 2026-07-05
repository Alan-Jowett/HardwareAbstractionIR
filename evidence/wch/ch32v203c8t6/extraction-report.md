# HAIR Extraction Report

## Device Summary

- **Target:** WCH CH32V203C8T6
- **Family:** CH32V203 / CH32V20x_D6
- **Architecture:** QingKe V4B / RV32IMACXW
- **Package:** LQFP48 (7x7 mm, 0.5 mm pitch)
- **Status:** Rerun completed with register-bearing structural depth across the in-scope MMIO peripherals. Header-derived register recovery is now RM-corrected for timers and CAN, and USB is no longer modeled as a shell-only block.

## Source Inventory

| Source ID | Kind | Location | Version | Notes | Likely HAIR layers |
| --- | --- | --- | --- | --- | --- |
| `wch-ch32v20x-30x-datasheet` | datasheet | `https://www.wch-ic.com/downloads/CH32V20x_30xDS0_PDF.html` | 3.0 | Exact-variant scope, memories, package, clocks, and pinout. | structure, physical |
| `wch-ch32fv2x-v3x-rm` | reference-manual | `https://www.wch-ic.com/downloads/CH32FV2x_V3xRM_PDF.html` | 2.5 | Authoritative register tables and timer/CAN conflict resolution. | structure, semantics, physical |
| `openwch-ch32v20x-sdk` | sdk | `https://github.com/openwch/ch32v20x/tree/804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | `804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | Cross-check linker and build facts. | structure, physical |
| `openwch-ch32v20x-header` | vendor-header | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h` | V1.0.1 | Broad recovery source for register structs, offsets, and most field masks. | structure |
| `openwch-ch32v20x-startup-d6` | other | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Startup/startup_ch32v20x_D6.S` | V1.0.2 | D6 interrupt vector ordering cross-check. | structure |
| `qingke-v4-processor-manual` | other | `https://www.wch-ic.com/downloads/QingKeV4_Processor_Manual_PDF.html` | 1.5 | CPU ISA/core context. | structure |

## Major Hardware Areas Discovered

- Core/device identity, memory map, and PFIC interrupt model
- Full in-scope peripheral register inventory for timers, watchdogs, serial blocks, CAN, DMA, GPIO, analog, clock/reset, backup/power, FLASH/CRC, and USBFS host/device views
- RM-corrected timer class split between advanced TIM1 and general-purpose TIM2/3/4
- RM-corrected CAN filter-bank depth through bank 27
- Existing clocks/power/package/pin model from the prior pass, preserved

## Component Inventory

| Subsystem | Components discovered |
| --- | --- |
| CPU / core | QingKe V4B core, PFIC, debug interface |
| Memory | 64KB code flash, 64KB boot alias, 20KB SRAM |
| Timers | TIM1 advanced-control timer; TIM2, TIM3, TIM4 general-purpose timers |
| Serial / comms | USART1/2/3, UART4, SPI1/2, I2C1/2, bxCAN1 |
| Mixed-signal | ADC1, ADC2, OPA control block |
| System / infra | RCC, DMA1 with 8 channels, AFIO, EXTI, BKP, PWR, FLASH, CRC, RTC, IWDG, WWDG |
| GPIO | GPIOA, GPIOB, GPIOC, GPIOD |
| USB | USBFS device register view and USBFS host/device register view at the shared USBFS base |

## Epistemic Claims Ledger

| Claim or entity | Label | Supporting evidence or reason |
| --- | --- | --- |
| Peripheral register shells were replaced with concrete register-bearing blocks for all currently modeled MMIO peripherals. | [KNOWN] | Header register structs plus the updated HAIR `registers[]` content for every peripheral. |
| TIM1 includes RPTCR, BDTR, DMACFGR, and DMAADR on CH32V203C8T6. | [KNOWN] | RM TIM1 advanced-timer chapter pages 237-242. |
| TIM2/TIM3/TIM4 do not expose BDTR or RPTCR on this D6 device. | [KNOWN] | RM tables 15-3 through 15-5 pages 253-254. |
| TIMx_AUX should be omitted for CH32V203C8T6 despite appearing in generic timer material. | [KNOWN] | RM availability note on page 243 excludes CH32V20x_D6. |
| CAN1 implements 28 filter banks through F27R2. | [KNOWN] | RM CAN filter tables pages 430-432. |
| USBFS register presence is defensible, but USB bitfields remain intentionally omitted. | [KNOWN] | Header USB struct layouts provide offsets; no matching header bit-definition sections were found. |
| CAN filter-bank fields beyond bank 13 reuse the bank-0 bit pattern shape. | [INFERRED] | RM shows the repeated F14R1..F27R2 registers exist; header bank-0 field macros establish the 32-bit FB0..FB31 pattern. |
| Per-channel OPA1/OPA2 semantics are fully modeled. | [ASSUMPTION] | Not claimed; only the header-backed shared OPA control register block is modeled structurally. |

## Conflicts and Clarification Questions

- **Resolved conflict:** the generic `TIM_TypeDef` in the vendor header over-approximates general-purpose timers by including `RPTCR` and `BDTR`, and under-approximates by omitting `AUX`. The rerun follows the RM: TIM1 keeps advanced-only registers, TIM2/3/4 drop `RPTCR`/`BDTR`, and `TIMx_AUX` is omitted for CH32V20x_D6 because the RM availability note excludes this family.
- **Resolved conflict:** the header exposes 28 CAN filter-register slots but only ships bit-definition sections through bank 13. The rerun keeps all 28 banks from the RM table and only reuses the generic `FB0..FB31` field pattern where that reuse survives adversarial review.
- **Resolved ambiguity:** USBFS device and host/device views share the same base address. The rerun keeps both logical register views because the header defines two distinct typed overlays on `USBFS_BASE`.
- **Clarification questions:** None identified for this rerun.

## Rejected Candidate Claims

| Candidate claim | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| TIM2/TIM3/TIM4 have the same register set as generic `TIM_TypeDef`, including `RPTCR` and `BDTR`. | Rejected: the RM general-purpose timer tables for the D6 device omit both registers. | RM pages 253-254. |
| TIMx_AUX is present on CH32V203C8T6 because it appears in generic timer material. | Rejected: the RM availability note excludes CH32V20x_D6. | RM page 243. |
| CAN1 only has 14 filter banks because header bit definitions stop at `F13R2`. | Rejected: the RM filter table continues through `F27R2`. | RM pages 430-432. |
| USBFS should remain shell-only because field masks are missing. | Rejected: the header still provides concrete USB register structs, which are sufficient to ground register presence and offsets without inventing fields. | Header lines 587-680. |

## Coverage

- **Examined**: manifest, HAIR schemas/docs, current HAIR/report, vendor header struct and bit-definition sections, RM timer pages 237-254, RM CAN pages 430-432, prior datasheet/startup/core-manual evidence already embedded in the document.
- **Method**: manifest validation, targeted header parsing, selective RM falsification for ambiguous blocks, script-assisted `hair.json` rewrite, and schema validation with repository-aware `allOf` flattening.
- **Excluded**: unofficial SVDs/community mirrors, speculative reset values/semantics not efficiently grounded, and unverified per-lot optional timer AUX behavior.
- **Limitations**: USB fields remain omitted, CAN banks 14-27 reuse only generic bank-bit shape rather than bank-numbered header macros, and the OPA block is still a shared control-register view rather than separate per-channel semantic entities.

## Limitations

- USBFS device/host overlays now have grounded registers, but field-level masks were not recoverable from the official header or selectively-read RM pages, so those fields remain intentionally absent.
- OPA is structurally modeled as the shared header-backed control block at `OPA_BASE`; the current rerun does not split that block into separate OPA1/OPA2 semantic subdevices.
- Register reset values are still omitted where they were not efficiently and confidently recoverable during this rerun.
