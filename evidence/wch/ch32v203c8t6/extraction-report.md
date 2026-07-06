# HAIR Extraction Report

## Device Summary

- **Target:** WCH CH32V203C8T6
- **Family:** CH32V203 / CH32V20x_D6
- **Architecture:** QingKe V4B / RV32IMACXW
- **Package:** LQFP48 (7x7 mm, 0.5 mm pitch)
- **Status:** Rerun completed with full register-bearing structural depth across the in-scope MMIO peripherals, official-source-backed peripheral address-block coverage and register-level access metadata, conservative community metadata enrichment, explicit HAIR alternate-register modeling for the timer `CHCTLR1` / `CHCTLR2` input/output overlays, conservative transitive USB metadata import from the approved `CH32V203C8T6.yaml` fragments, and a follow-up cleanup pass that removed overlapping USB field definitions unsafe for SVD lowering. Remaining incompleteness is now concentrated in field-level topology/name drift, a small residual USB host-side prose gap, reset masks, broader non-USB enum coverage, and missing cluster prose.

## Source Inventory

| Source ID | Kind | Location | Version | Notes | Likely HAIR layers |
| --- | --- | --- | --- | --- | --- |
| `wch-ch32v20x-30x-datasheet` | datasheet | `https://www.wch-ic.com/downloads/CH32V20x_30xDS0_PDF.html` | 3.0 | Exact-variant scope, memories, package, clocks, and pinout. | structure, physical |
| `wch-ch32fv2x-v3x-rm` | reference-manual | `https://www.wch-ic.com/downloads/CH32FV2x_V3xRM_PDF.html` | 2.5 | Authoritative register tables and timer/CAN conflict resolution. | structure, semantics, physical |
| `openwch-ch32v20x-sdk` | sdk | `https://github.com/openwch/ch32v20x/tree/804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | `804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | Cross-check linker and build facts. | structure, physical |
| `openwch-ch32v20x-header` | vendor-header | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h` | V1.0.1 | Broad recovery source for register structs, offsets, and most field masks. | structure |
| `openwch-ch32v20x-startup-d6` | source-code | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Startup/startup_ch32v20x_D6.S` | V1.0.2 | D6 interrupt vector ordering cross-check. | structure |
| `qingke-v4-processor-manual` | other | `https://www.wch-ic.com/downloads/QingKeV4_Processor_Manual_PDF.html` | 1.5 | CPU ISA/core context. | structure |
| `ch32-rs-ch32v203xx-svd` | svd | `https://raw.githubusercontent.com/ch32-rs/ch32-rs/9b4ee66500b956bc87fbf83aa28ad245b39ebd15/svd/vendor/CH32V203xx.svd` | `9b4ee66500b956bc87fbf83aa28ad245b39ebd15` | Community-maintained family SVD used only as an auditable metadata gap-filler on exact structural matches. | structure |
| `ch32-rs-ch32v203c8t6-yaml` | generated | `https://raw.githubusercontent.com/ch32-rs/ch32-data/a515903589cfbc342dc6ad0d13c02b4382da5628/data/chips/CH32V203C8T6.yaml` | `a515903589cfbc342dc6ad0d13c02b4382da5628` | Community exact-variant YAML used to defend exact-variant fit for the family-SVD metadata reuse. | structure |

## Major Hardware Areas Discovered

- Core/device identity, memory map, and PFIC interrupt model
- Full in-scope peripheral register inventory for timers, watchdogs, serial blocks, CAN, DMA, GPIO, analog, clock/reset, backup/power, FLASH/CRC, and USBFS host/device views
- RM-corrected timer class split between advanced TIM1 and general-purpose TIM2/3/4
- RM-corrected CAN filter-bank depth through bank 27
- Header-backed peripheral address-block coverage and register-level read/write access metadata across the modeled MMIO set
- Exact-match plus conservative alias/template community enrichment for register prose, many reset values, and many field descriptions/access annotations without changing the official-source-derived topology
- Existing clocks/power/package/pin model from the prior pass, preserved

## Component Inventory

| Subsystem | Components discovered |
| --- | --- |
| CPU / core | QingKe V4B core, PFIC, vendor system timer, debug interface (SWDIO/SWCLK) |
| Memory | 64KB code flash, 64KB flash boot alias at `0x00000000`, 20KB SRAM |
| Timers | TIM1 advanced-control timer; TIM2, TIM3, TIM4 general-purpose timers |
| Serial / comms | USART1/2/3, UART4, SPI1/2, I2C1/2, bxCAN1 |
| Mixed-signal | ADC1, ADC2, OPA/CMP control block |
| System / infra | RCC, DMA1 with 8 channels, AFIO, EXTI, BKP, PWR, FLASH, CRC, RTC, IWDG, WWDG |
| GPIO / package | GPIOA, GPIOB, GPIOC, GPIOD, plus LQFP48 package supply, boot, reset, and debug pads |
| USB | USBFS device register view and USBFS host/device register view at the shared USBFS base |

## Metadata Coverage and Gaps

### Metadata present in approved evidence and extracted

| Metadata class | Extraction status | Supporting evidence |
| --- | --- | --- |
| Peripheral descriptions | Extracted for all modeled peripherals. | Existing HAIR peripheral descriptions grounded by the approved header/datasheet-backed extraction pass. |
| Peripheral address-block coverage | Extracted for all modeled peripherals as a single `registers` block spanning the recovered register window. | Vendor header struct/member order, reserved padding, nested mailbox/filter structs, and array strides (`e_header_register_window_extents`). |
| Register access modes | Extracted for all recovered peripheral registers as header-level `read-write`. | `core_riscv.h` defines `__IO` as read/write, and the modeled peripheral members are `__IO`-qualified (`e_core_io_qualifiers` plus `e_header_register_typedefs`). |
| Register descriptions | Extracted on 393 / 397 modeled registers, including the explicit timer alternate-register views and the transitive USB prose import. | Community `CH32V203xx.svd`, plus conservative transitive reuse from `CH32V203C8T6.yaml -> FV2x_V3x_USBD.yaml + FV2x_V3x_USBFS.yaml -> otg_v2.yaml`, gated by exact or alias-safe overlap that preserves the current topology (`e_ch32rs_ch32v203xx_svd_metadata`, `e_ch32rs_ch32v203c8t6_yaml_variant`, `e_ch32rs_ch32v203c8t6_yaml_usb_fragments`). |
| Register reset values | Extracted on 303 / 397 modeled registers. | Community `CH32V203xx.svd`, reused only on exact structural matches or defensible clustered-template reuse and now accepted by the corrected `common.valueLiteral` schema (`e_ch32rs_ch32v203xx_svd_metadata`). |
| Field descriptions | Expanded to 1938 / 2366 modeled fields, including the safe timer alias cases, explicit timer input/output overlay views, BKP backup-data fields, selected CAN status bits, CAN mailbox/filter and DMA channel template members, and the approved USB transitive fragments. | Community `CH32V203xx.svd`, plus conservative transitive reuse from the approved USB fragments, imported only where register/template identity and bit ranges could be defended without reshaping the official-source-derived model (`e_ch32rs_ch32v203xx_svd_metadata`, `e_ch32rs_ch32v203c8t6_yaml_usb_fragments`). |
| Field access modes | Expanded to 254 / 2366 modeled fields. | Community `CH32V203xx.svd` field annotations, imported only where the matched or normalized field record carried explicit access metadata (`e_ch32rs_ch32v203xx_svd_metadata`). |
| Field enumerated values | Expanded to 25 / 2366 modeled fields, concentrated in the imported USB metadata. | Conservative transitive reuse from `otg_v2.yaml` through the approved `CH32V203C8T6.yaml` USB fragments (`e_ch32rs_ch32v203c8t6_yaml_usb_fragments`). |

### Metadata present in approved evidence but not extracted

| Metadata class | Status | Reason |
| --- | --- | --- |
| Additional community register metadata on unmapped or sparsely mapped blocks | Still partially omitted for `SPI1.I2SPR`, `FLASH.ACTLR`, `USBFSH.HOST_RX_CTRL`, and `USBFSH.HOST_TX_LEN`. | `SPI1.I2SPR` and `FLASH.ACTLR` still lack a clean approved-source import path, `USBFSH.HOST_RX_CTRL` still needs alias reconciliation from the transitive `RX_CTRL` fragment, and `USBFSH.HOST_TX_LEN` has no matching transitive prose record. |
| Additional community field metadata on structurally divergent or aggregate-modeled fields | Still partially omitted. | The remaining gaps are now concentrated in field-level topology/name drift such as CAN aggregate-vs-bit-per-filter modeling, AFIO/EXTI/RCC naming drift, timer/ADC families, and a small residual `USBFSH` host-side tail that would require further reshaping or source augmentation rather than conservative enrichment. |
| USB community metadata | Mostly imported. | Conservative transitive reuse from `CH32V203C8T6.yaml -> FV2x_V3x_USBD.yaml + FV2x_V3x_USBFS.yaml -> otg_v2.yaml` now fills all `USBFSD` register/field descriptions plus most `USBFSH` register/field descriptions and 25 USB enum-bearing fields without reshaping the dual-overlay topology. |

### Metadata not confidently recoverable from approved evidence examined in this rerun

| Metadata class | Status | Why still absent |
| --- | --- | --- |
| Cluster descriptions | Not recovered. | Neither the official sources examined nor the exact-match community import provided a stable cluster-description layer that could be attached without broader manual reconciliation. |
| Register reset masks | Not recovered. | The community SVD overlap supplied no reliable per-register reset-mask layer suitable for import. |
| Remaining field access modes | Partially recovered only. | The community SVD carries explicit field-level access for only a subset of fields, so even after the alias/template import pass the document still lacks a complete field-level permission model. |
| Field enumerated values | Partially recovered only. | 25 enum-bearing fields now come from the approved USB transitive fragments, but most modeled fields still lack approved-source enum tables. |
| USB field metadata | Partially recovered only. | Device-side USB prose is complete and host-side prose is mostly complete, but the approved transitive fragments still leave `USBFSH.HOST_EP_MOD.{RBUF_MOD,RX_EN,TBUF_MOD,TX_EN}` and `USBFSH.HOST_TX_CTRL.{T_RES,T_TOG,T_AUTO_TOG}` without descriptions, and do not provide a mapped prose source for `USBFSH.HOST_TX_LEN`. |

## Normalization and Completeness Matrix

| Approved source | Metadata class | Present in evidence | Extracted | Normalization required | Remaining unresolved inventory | Root cause |
| --- | --- | --- | --- | --- | --- | --- |
| `openwch-ch32v20x-header` + `core_riscv.h` | Peripheral windows and register access | Yes | Yes | No | None identified. | None. |
| `wch-ch32fv2x-v3x-rm` | Timer/CAN topology corrections | Yes | Yes | Yes | None identified for the challenged TIM1/TIM2-4 and CAN filter-bank depth cases. | None after RM adjudication. |
| `ch32-rs-ch32v203xx-svd` + `ch32-rs-ch32v203c8t6-yaml` | Register descriptions | Yes | Mostly | Yes | 4 remaining gaps: `SPI1.I2SPR`, `FLASH.ACTLR`, `USBFSH.HOST_RX_CTRL`, and `USBFSH.HOST_TX_LEN`. | Normalization mismatch / representational difference plus source sparsity. |
| `ch32-rs-ch32v203xx-svd` + `ch32-rs-ch32v203c8t6-yaml` | Register reset values | Yes | Mostly | Yes | 94 registers still lack reset values, concentrated outside the exact/template-matched families. | Normalization mismatch / representational difference. |
| `ch32-rs-ch32v203xx-svd` + `ch32-rs-ch32v203c8t6-yaml` | Field descriptions | Yes | Partially | Yes | 428 remaining gaps, led by AFIO, EXTI, RCC, TIM2/3/4, ADC1/2, DMA1, and a 7-field `USBFSH` host-side tail. | Normalization mismatch / representational difference. |
| `ch32-rs-ch32v203xx-svd` + `ch32-rs-ch32v203c8t6-yaml` | Field access modes | Yes | Partially | Yes | 2112 fields still lack explicit access semantics, including most AFIO/TIM/GPIO/ADC families and all imported USB fields. | Partial source sparsity plus normalization mismatch / representational difference. |
| `ch32-rs-ch32v203xx-svd` | Register reset masks | No reliable overlap layer | No | N/A | Whole metadata class remains absent. | Unsupported by approved evidence. |
| `ch32-rs-ch32v203c8t6-yaml` transitive USB fragments | Field enumerated values | Yes | Partially | Yes | 25 USB fields now carry enums; 2341 fields still lack approved-source enum tables. | Partial source sparsity. |
| `ch32-rs-ch32v203c8t6-yaml` + official USB overlays | USB register/field prose and enums | Yes | Mostly | Yes | `USBFSD` is fully described; `USBFSH` retains 2 register-description gaps, 7 field-description gaps, and only `INT_ST.MASK_TOKEN` currently carries an enum. | Representational difference preserved by the overlay boundary plus transitive-source sparsity. |

## Epistemic Claims Ledger

| Claim or entity | Label | Supporting evidence or reason |
| --- | --- | --- |
| CH32V203C8T6 is in scope of CH32V203DS0 v3.0 | [KNOWN] | WCH file metadata id 354 and datasheet scope coverage explicitly include CH32V203C8T6. |
| The exact variant has 64KB flash and 20KB SRAM | [KNOWN] | Datasheet model table page 7 and official C8T6 linker script agree. |
| The D6 interrupt numbering in the draft is correct for C8T6 | [KNOWN] | D6 startup vector file provides the exact interrupt ordering used in the draft. |
| The extracted ISA should include the vendor `XW` extension | [KNOWN] | The official CH32V203C8T6 SDK build uses `-march=rv32imacxw`, so the architecture record models `RV32IMACXW`. |
| The device CPU revision identifier should be modeled as `V4B` | [KNOWN] | The datasheet and QingKe core manual both identify the CPU as the QingKe V4B core, which is the best grounded revision/variant identifier currently exposed by the authoritative sources. |
| The PFIC implements four interrupt priority bits | [KNOWN] | The official `core_riscv.h` priority API documents bit7 plus bits6-bit4 as active priority bits. |
| The device uses vendor system-timer configuration | [KNOWN] | The official core support header defines a vendor SysTick block at `0xE000F000`, and the official CH32V203C8T6 debug support code configures it directly for delays. |
| Peripheral register shells were replaced with concrete register-bearing blocks for all currently modeled MMIO peripherals. | [KNOWN] | Header register structs plus the updated HAIR `registers[]` content for every peripheral. |
| Header struct extent is sufficient to model one register-bearing address block per in-scope peripheral. | [KNOWN] | The vendor header typedefs include ordered members, reserved padding, nested mailbox/filter structs, and array stride information that bound each recovered peripheral window. |
| Register access metadata can be modeled as header-level `read-write` for the recovered peripheral members. | [KNOWN] | `core_riscv.h` defines `__IO` as read/write, and the recovered peripheral register structs use `__IO`-qualified members. |
| Conservative community metadata reuse is defensible on exact peripheral/register/field matches without reshaping the official-source-derived topology. | [KNOWN] | The imported metadata were gated by exact peripheral-name, register-name-plus-offset, and field-name-plus-bit-range overlap, with the exact variant fit cross-checked against `CH32V203C8T6.yaml`. |
| TIM1 includes RPTCR, BDTR, DMACFGR, and DMAADR on CH32V203C8T6. | [KNOWN] | RM TIM1 advanced-timer chapter pages 237-242. |
| TIM2/TIM3/TIM4 do not expose BDTR or RPTCR on this D6 device. | [KNOWN] | RM tables 15-3 through 15-5 pages 253-254. |
| TIMx_AUX should be omitted for CH32V203C8T6 despite appearing in generic timer material. | [KNOWN] | RM availability note on page 243 excludes CH32V20x_D6. |
| CAN1 implements 28 filter banks through F27R2. | [KNOWN] | RM CAN filter tables pages 430-432. |
| USBFS register presence and most USB prose/enum metadata are defensible, though a small host-side tail remains unresolved. | [KNOWN] | Header USB struct layouts provide offsets, and the approved `CH32V203C8T6.yaml -> ... -> otg_v2.yaml` transitive fragments safely populate `USBFSD` fully and `USBFSH` mostly without reshaping the dual-overlay topology. |
| CAN filter-bank fields beyond bank 13 reuse the bank-0 bit pattern shape. | [INFERRED] | RM shows the repeated F14R1..F27R2 registers exist; header bank-0 field macros establish the 32-bit FB0..FB31 pattern. |
| OPA/CMP hardware exists on CH32V203C8T6 | [KNOWN] | Datasheet model table lists 2 OPA/CMP units for C8x6, and the header provides the OPA MMIO base. |
| The LQFP48 pad map in the draft matches the vendor pin-definition table | [KNOWN] | Datasheet table 3-1-1 pages 23-26 and package table page 59. |
| Pads 5 and 6 should be modeled as PD0/PD1 with reset-default oscillator roles on C8T6 | [KNOWN] | Datasheet note 4 on page 35 explicitly states the C8T6-specific reset behavior and remap. |
| FPU support is intentionally omitted from the draft | [KNOWN] | The datasheet feature list conflicts with QingKe V4B naming and the official example build flags, so the draft keeps no positive FPU claim. |
| Per-channel OPA1/OPA2 semantics are fully modeled. | [ASSUMPTION] | Not claimed; only the header-backed shared OPA/CMP control register block is modeled structurally. |
| Exhaustive register-field coverage is complete | [ASSUMPTION] | Not claimed; the normalization pass still leaves explicit representational-difference and unsupported-evidence gaps. |

## Conflicts and Clarification Questions

- **Resolved conflict:** the generic `TIM_TypeDef` in the vendor header over-approximates general-purpose timers by including `RPTCR` and `BDTR`, and under-approximates by omitting `AUX`. The rerun follows the RM: TIM1 keeps advanced-only registers, TIM2/3/4 drop `RPTCR`/`BDTR`, and `TIMx_AUX` is omitted for CH32V20x_D6 because the RM availability note excludes this family.
- **Resolved conflict:** timer `CHCTLR1` / `CHCTLR2` registers carry distinct input-capture and output-compare field layouts at one offset. The rerun now models them as explicit alternate register views rather than flattening both interpretations into one HAIR register.
- **Resolved conflict:** the header exposes 28 CAN filter-register slots but only ships bit-definition sections through bank 13. The rerun keeps all 28 banks from the RM table and only reuses the generic `FB0..FB31` field pattern where that reuse survives adversarial review.
- **Resolved ambiguity:** USBFS device and host/device views share the same base address. The rerun keeps both logical register views because the header defines two distinct typed overlays on `USBFS_BASE`.
- **Resolved metadata gap:** the header/core access-layer sources are sufficient to recover one register-bearing address block per modeled peripheral and header-level register access as `read-write`, even though they do not expose richer field-level or side-effect-specific access semantics.
- **Resolved community-source policy question:** the community SVD and YAML were used only as metadata enrichers on exact structural matches. Unmatched or structurally divergent community records were left unimported rather than silently remapped into the official-source-derived model.
- **Resolved scope boundary:** preserve the current official-source-derived topology rather than reshaping CAN or USB blocks purely to chase closer community-SVD completeness, so the remaining topology-dependent deltas stay explicitly documented instead of being force-imported.

## Rejected Candidate Claims

| Candidate claim | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| TIM2/TIM3/TIM4 have the same register set as generic `TIM_TypeDef`, including `RPTCR` and `BDTR`. | Rejected: the RM general-purpose timer tables for the D6 device omit both registers. | RM pages 253-254. |
| TIMx_AUX is present on CH32V203C8T6 because it appears in generic timer material. | Rejected: the RM availability note excludes CH32V20x_D6. | RM page 243. |
| CAN1 only has 14 filter banks because header bit definitions stop at `F13R2`. | Rejected: the RM filter table continues through `F27R2`. | RM pages 430-432. |
| USBFS should remain shell-only because field masks are missing. | Rejected: the header still provides concrete USB register structs, which are sufficient to ground register presence and offsets without inventing fields. | Header lines 587-680. |
| Community SVD metadata can be used to rename or reshape the official-source-derived register topology. | Rejected: the community sources were approved only as metadata gap-fillers, so this pass reused them only on exact structural matches and left divergent names or layouts untouched. | `e_ch32rs_ch32v203xx_svd_metadata` plus the unchanged official-source-derived register layout. |
| All remaining metadata gaps can be filled automatically from the community sources. | Rejected: several remaining gaps are caused by topology/name drift, the residual `USBFSH` host-side alias/sparsity tail, or the absence of reset masks and broad enum coverage in the approved community sources themselves. | Exact-match overlap results and the remaining unmatched register/field sets. |

## Unresolved Differences Inventory

- **CAN filter-control bit families:** `CAN1.FMCFGR.FBM0..FBM13`, `CAN1.FSCFGR.FSC0..FSC13`, `CAN1.FAFIFOR.FFA0..FFA13`, and related per-filter control bits still differ because the community SVD models individual bits while the current HAIR keeps aggregate field groupings from the official-source-derived topology; this difference is intentionally preserved.
- **USB host-side residuals:** the conservative transitive USB import leaves `USBFSH.HOST_RX_CTRL`, `USBFSH.HOST_TX_LEN`, `USBFSH.HOST_EP_MOD.{RBUF_MOD,RX_EN,TBUF_MOD,TX_EN}`, and `USBFSH.HOST_TX_CTRL.{T_RES,T_TOG,T_AUTO_TOG}` without prose, because the current overlay mapping still has one alias case and several sparse host-side source records.
- **FLASH control extras:** `FLASH.ACTLR` still exists in the community family SVD without a cleanly justified import path into the current official-source-derived CH32V203C8T6 model.
- **Residual naming-drift field prose/access cases:** a smaller tail of RTC/I2C/BKP field metadata remains unmatched after conservative normalization because the family/community names do not cleanly align with the current official-source-derived field identities.
- **Whole-class gaps:** cluster descriptions and register reset masks remain absent, and field enumerated values remain sparse outside the imported USB fragments.

## Coverage

- **Examined**: manifest, HAIR schemas/docs, current HAIR/report, vendor header struct and bit-definition sections, `core_riscv.h` access-qualifier definitions, RM timer pages 237-254, RM CAN pages 430-432, community `CH32V203xx.svd`, community `CH32V203C8T6.yaml`, and prior datasheet/startup/core-manual evidence already embedded in the document.
- **Method**: manifest validation, targeted header parsing, access-qualifier review, selective RM falsification for ambiguous blocks, derivedFrom-aware exact-match overlap analysis against the community SVD, script-assisted `hair.json` rewrite, and metadata-gap classification against the updated extract skill requirements.
- **Excluded**: community metadata that still requires field-topology reshaping, speculative USB view remapping, uninspected RM prose/table sections outside the timer/CAN challenge set, speculative reset masks/semantics not efficiently grounded, and unverified per-lot optional timer AUX behavior.
- **Limitations**: the rerun now recovers the defensible register-level alias/template cases plus several additional field-level prose, enum, and reset-value fixes from the approved community sources, but still leaves aggregate/split field mismatches and a small residual USB host-side gap where import would require further topology work or unsupported assumptions.

## Limitations

- USBFS device/host overlays now have grounded registers plus substantial imported prose and enum coverage, but `USBFSH` still retains a small host-side alias/sparsity gap because the current dual-overlay HAIR model does not map 1:1 onto every community `USBD` / `USBFS` record.
- OPA is structurally modeled as the shared header-backed control block at `OPA_BASE`; the current rerun does not split that block into separate OPA1/OPA2 semantic subdevices.
- Register access metadata is still modeled primarily at the vendor-header qualifier level (`__IO` => `read-write`); the community SVD adds explicit field-access annotations only for a subset of fields, so richer side-effect-specific access remains incomplete.
- Reset masks remain absent, and enumerated values remain sparse outside the imported USB fragment coverage.
- The remaining gaps are now mostly field-topology reconciliation problems (especially CAN aggregate-vs-per-bit modeling), the residual `USBFSH` host-side gap, and `FLASH.ACTLR`.
