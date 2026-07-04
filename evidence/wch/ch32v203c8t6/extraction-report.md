# HAIR Extraction Report

## Device Summary

- **Target:** WCH CH32V203C8T6
- **Family:** CH32V203
- **Architecture:** QingKe V4B / RV32IMACXW
- **Package:** LQFP48 (7x7 mm, 0.5 mm pitch)
- **Status:** Draft extraction now includes grounded device identity, memory, interrupts, major peripherals, clocks/power, and a normalized LQFP48 pin/package model.

## Source Inventory

| Source ID | Kind | Authoritative reference | Notes |
| --- | --- | --- | --- |
| `wch-ch32v20x-30x-datasheet` | datasheet | `https://www.wch-ic.com/download/file?id=354` | Official CH32V203 datasheet v3.0; exact scope includes CH32V203C8T6. |
| `wch-ch32fv2x-v3x-rm` | reference-manual | `https://www.wch-ic.com/download/file?id=324` | Official reference manual v2.5 for CH32V203C8T6-class devices. |
| `openwch-ch32v20x-sdk` | sdk | `https://github.com/openwch/ch32v20x/tree/804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | Official SDK snapshot used for linker, toolchain, and cross-check evidence. |
| `openwch-ch32v20x-header` | vendor-header | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h` | Machine-readable peripheral and base-address source. |
| `openwch-ch32v20x-startup-d6` | other | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Startup/startup_ch32v20x_D6.S` | Confirms D6 vector ordering for CH32V203C8-class devices. |
| `qingke-v4-processor-manual` | other | `https://www.wch-ic.com/download/file?id=367` | Core-level ISA/context source. |

## Major Hardware Areas Discovered

- CPU/core identity and PFIC interrupt model
- C8T6-exact flash and SRAM capacities
- D6 interrupt list and vector ordering
- Major peripheral inventory with grounded MMIO base addresses
- Clock sources, PLL/SYSCLK ceiling, and operating-voltage range
- Normalized LQFP48 package map including the C8T6-specific OSC_IN/OSC_OUT to PD0/PD1 remap note

## Component Inventory

| Subsystem | Components discovered |
| --- | --- |
| CPU / core | QingKe V4B core, PFIC, SDI debug interface |
| Memory | 64KB code flash, 64KB flash boot alias at `0x00000000`, 20KB SRAM |
| Timers | TIM1, TIM2, TIM3, TIM4 |
| Serial | USART1, USART2, USART3, UART4, SPI1, SPI2, I2C1, I2C2 |
| Mixed-signal | ADC1, ADC2, OPA/CMP block |
| Connectivity | CAN1, USBFS device interface, USBFS host/device interface |
| System | RCC, PWR, EXTI, AFIO, DMA1, RTC, BKP, FLASH, CRC, IWDG, WWDG |
| GPIO / package | 37 GPIO-capable pads plus LQFP48 package supply, boot, reset, and debug pads |

## Epistemic Claims Ledger

| Claim or entity | Label | Supporting evidence or reason |
| --- | --- | --- |
| CH32V203C8T6 is in scope of CH32V203DS0 v3.0 | [KNOWN] | WCH file metadata id 354 and datasheet scope coverage explicitly include CH32V203C8T6. |
| The exact variant has 64KB flash and 20KB SRAM | [KNOWN] | Datasheet model table page 7 and official C8T6 linker script agree. |
| The D6 interrupt numbering in the draft is correct for C8T6 | [KNOWN] | D6 startup vector file provides the exact interrupt ordering used in the draft. |
| The extracted ISA should include the vendor `XW` extension | [KNOWN] | The official CH32V203C8T6 SDK build uses `-march=rv32imacxw`, so the architecture record models `RV32IMACXW`. |
| OPA/CMP hardware exists on CH32V203C8T6 | [KNOWN] | Datasheet model table lists 2 OPA/CMP units for C8x6, and the header provides the OPA MMIO base. |
| The LQFP48 pad map in the draft matches the vendor pin-definition table | [KNOWN] | Datasheet table 3-1-1 pages 23-26 and package table page 59. |
| Pads 5 and 6 should be modeled as PD0/PD1 with reset-default oscillator roles on C8T6 | [KNOWN] | Datasheet note 4 on page 35 explicitly states the C8T6-specific reset behavior and remap. |
| FPU support is intentionally omitted from the draft | [KNOWN] | The datasheet feature list conflicts with QingKe V4B naming and the official example build flags, so the draft keeps no positive FPU claim. |
| Exhaustive register-field coverage is complete | [ASSUMPTION] | Not claimed; machine-readable SVD data is absent and field-level extraction has not yet been normalized. |

## Conflicts and Clarification Questions

- **Resolved conflict:** the datasheet overview text mentions "hardware FPU," but the same document identifies the core as QingKe V4B and the official CH32V203C8T6 example build uses `-march=rv32imacxw` without any floating-point extension. The draft resolves this by omitting FPU support rather than asserting it.
- **Resolved ambiguity:** the generic family block diagram and header mention TIM5 and Ethernet resources, but the model table and SDK D6/D8 split constrain those blocks to CH32V203RBx / D8-class devices. The draft excludes both for CH32V203C8T6.

## Rejected Candidate Claims

| Candidate claim | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| CH32V203C8T6 has 224KB of directly usable application flash | Rejected: the family overview mentions 224KB max code-flash organization for non-RBx parts, but the exact C8T6 model table and official linker script both constrain the variant to 64KB flash. | Datasheet page 7 and the official C8T6 linker script. |
| CH32V203C8T6 has a hardware FPU | Rejected: the family feature list is contradicted by the QingKe V4B core naming and the official example build flags, which compile without a floating-point ISA extension. | Datasheet page 1, QingKe V4 manual, and `subdir.mk` `-march=rv32imacxw`. |
| TIM5 is present on CH32V203C8T6 | Rejected: TIM5 appears only for CH32V203RBx / D8-class devices in the family block diagram and SDK split, not for the D6 C8T6 subset. | Datasheet page 8, `CH32V20x_List_EN.txt`, and the D6 startup vector file. |
| Ethernet is present on CH32V203C8T6 | Rejected: Ethernet is explicitly a D8 / D8W-class resource, while C8T6 is in the D6 group. | Datasheet page 7, `CH32V20x_List_EN.txt`, and header D6 grouping. |

## Coverage

- **Examined**: manifest, HAIR schemas, CH32V203 datasheet, CH32FV2x_V3x reference manual, QingKe V4 core manual, official header, D6 startup file, linker script, SDK build artifacts.
- **Method**: manifest validation, local PDF extraction, header/startup parsing, pin-table normalization, and adversarial cross-checking against D6-vs-D8 family splits.
- **Excluded**: community mirrors, unofficial SVDs, and unsupported field-level guesses.
- **Limitations**: register-field extraction is still omitted, and alternate functions are currently normalized as pin-level names rather than fully linked signal entities.

## Limitations

- Register structures are not yet normalized into full register/field schema objects; the current draft focuses on device identity, memory, interrupts, peripherals, and the physical pin/package model.
- Repeated supply/ground pads are normalized to shared logical power pins because the physical schema models pin entities rather than per-pad electrical nodes.
- The repository's current HAIR JSON schemas use `allOf` with `additionalProperties: false`, which causes stock JSON Schema validators to reject composed entities during local machine validation; this is treated as a schema/tooling issue separate from the evidence-grounded draft content.
