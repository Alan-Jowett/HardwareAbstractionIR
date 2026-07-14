# HAIR Extraction Report

## Device Summary

- **Target:** WCH CH32V203G6U6
- **Family:** CH32V203 / CH32V20x_D6 subgroup
- **Architecture:** QingKe V4B / RV32IMACXW
- **Package:** QFN28
- **Exact-variant identity facts:** device_id `0x20360500`, 32KB flash, 10KB SRAM, 24 GPIO, 2 USART/UART, 1 SPI, 1 I2C, 1 CAN, USBD present, USBFS/USBHD absent.
- **Requested profile scope:** core HAIR + `profiles.mcuSoc` + `profiles.embassyHal`
- **Requested Embassy scope:** the full driver inventory the evidence can justify
- **Status:** Final HAIR JSON written. `cargo run -- validate evidence\wch\ch32v203g6u6\hair.json` passed. `cargo run -- generate embassy evidence\wch\ch32v203g6u6\hair.json --output-dir evidence\wch\ch32v203g6u6\generated-check` also passed after omitting Embassy driver kinds/instances that the approved evidence or current generator could not justify.

## Source Inventory

| Source ID | Kind | Location | Version | Notes | Likely HAIR layers |
| --- | --- | --- | --- | --- | --- |
| `wch-ch32v203-datasheet` | datasheet | `https://www.wch.cn/downloads/CH32V203DS0_PDF.html` | 3.0 | Exact-variant model table, package/pin tables, clocks, supplies, and memory-map facts. | structure, physical |
| `wch-ch32fv2x-v3x-rm` | reference-manual | `https://www.wch.cn/downloads/CH32FV2x_V3xRM_PDF.html` | 2.5 | Family register semantics and timer classification. | structure, semantics |
| `openwch-ch32v20x-sdk` | sdk | `https://github.com/openwch/ch32v20x/tree/804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | `804daf39...` | Official umbrella repo; generic D6 linker layout cross-check. | structure |
| `openwch-ch32v20x-header` | vendor-header | `.../ch32v20x.h` | V1.0.1 | Official D6 subgroup identity, base addresses, register typedefs, bit definitions, timer/ADC control bits. | structure, semantics |
| `openwch-ch32v20x-startup-d6` | source-code | `.../startup_ch32v20x_D6.S` | V1.0.2 | D6 vector ordering cross-check; subgroup-level superset to be pruned against exact-variant evidence. | structure, profiles.mcuSoc |
| `openwch-ch32v20x-adc-dma-example` | source-code | `.../ADC_DMA/User/main.c` | V1.0.1 | Grounds ADC1 DMA direction. | semantics, profiles.mcuSoc |
| `openwch-ch32v20x-tim-dma-example` | source-code | `.../TIM_DMA/User/main.c` | V1.0.1 | Grounds TIM1 update DMA direction. | semantics, profiles.mcuSoc |
| `qingke-v4-processor-manual` | other | `https://www.wch.cn/downloads/QingKeV4_Processor_Manual_PDF.html` | 1.5 | Core/PFIC context. | structure, physical |
| `ch32-rs-ch32v203xx-svd` | svd | `.../CH32V203xx.svd` | `9b4ee665...` | Exact-family metadata-rich structural gap filler. | structure |
| `ch32-rs-ch32v203g6u6-yaml` | generated | `.../CH32V203G6U6.yaml` | `a5159035...` | Exact-variant package/device-id/topology source; transitive family/peripheral fragments used conservatively. | structure, physical, profiles.mcuSoc, profiles.embassyHal |
| `adafruit-qt-py-ch32v203-schematic` | source-code | `https://raw.githubusercontent.com/adafruit/Adafruit-QT-Py-CH32V203-PCB/c5c1580d9823d2f8923778530339179531d024f6/Adafruit%20QT%20Py%20CH32V203.sch` | `c5c1580d...` | Commit-pinned Adafruit Eagle schematic used only for board-level connector, UART-header, and NeoPixel wiring facts. | provenance, physical extensions |
| `adafruit-qt-py-ch32v203-overview` | other | `https://learn.adafruit.com/adafruit-qt-py-ch32v203/overview.md` | current web page | Adafruit board overview for USB-C, native USB/TinyUSB CDC, NeoPixel, and bootloader workflow claims. | provenance, physical extensions |
| `adafruit-qt-py-ch32v203-pinouts` | other | `https://learn.adafruit.com/adafruit-qt-py-ch32v203/pinouts.md` | current web page | Adafruit board pinout notes for PA2/PA3 UART and PA4 NeoPixel. | provenance, physical extensions |

## Major Hardware Areas Discovered

- QingKe V4B core, PFIC interrupt controller, vendor SysTick, and exact-variant flash/SRAM map
- Full register-bearing MMIO model for family-common support blocks plus exact-variant peripheral inventory: RCC, AFIO, EXTI, DMA1, GPIOA-D, USART1/2, SPI1, I2C1, TIM1/2/3/4, ADC1/2, CAN1, USBD, FLASH, RTC, watchdogs, CRC, OPA/CMP, BKP, and PWR
- Exact QFN28 package and package-filtered pad inventory, including PD0/PD1 oscillator-pad remap behavior
- MCU/SoC topology for clocks/resets, interrupts, DMA, pin routes, CAN1, and USBD
- Minimal but evidence-backed timer enable state machines and ADC enable/calibration operations
- Board-collateral annotations for the Adafruit QT Py CH32V203 USB-C connector, separate UART header nets, and onboard NeoPixel wiring

## Component Inventory

| Subsystem | Components discovered |
| --- | --- |
| CPU / interrupt | QingKe V4B core, PFIC, vendor SysTick |
| Memory | 32KB flash, 32KB flash boot alias, 10KB SRAM |
| Clock / reset | RCC, HCLK, PCLK1, PCLK2, PCLK1_TIM, PCLK2_TIM, ADC, AHB/APB1/APB2 reset domains |
| GPIO / mux | GPIOA, GPIOB, GPIOC register block, GPIOD, AFIO; package-visible pads only on GPIOA/GPIOB/GPIOD |
| Serial / comms | USART1, USART2, SPI1, I2C1, CAN1, USBD |
| Timers / PWM | TIM1 advanced timer, TIM2/TIM3/TIM4 general timers, PWM views over TIM1-TIM4 |
| Mixed-signal | ADC1, ADC2, OPA/CMP |
| Support / infrastructure | DMA1 (8 channels), EXTI, FLASH, RTC, BKP, PWR, CRC, WWDG, IWDG |
| Board annotations | USB-C native-device connector, PA2/PA3 UART header roles, and PA4 NeoPixel data |

## Embassy Readiness Summary

- **Requested scope:** full Embassy driver inventory justified by the approved evidence
- **Driver instances emitted:** 17 total — RCC; USART1/2; SPI1; I2C1; TIM1/2/3/4 timers; TIM1/2/3/4 PWM; ADC1/2; DMA1; PFIC interrupt
- **Supporting records in the extracted HAIR profiles:** 24 canonical blocks, 17 clock bindings, 16 reset bindings, 36 interrupt sources, 36 interrupt routes, 8 DMA channels, 10 DMA routes, 118 pin routes, 6 semantic operations, and 4 state machines
- **Checked-in Embassy crate materialization:** the generated crate currently exposes 7 DMA channels and 9 DMA routes in `generated/embassy/src/dma.rs`; the extracted HAIR profile also retains the TIM1 update DMA route on DMA1 channel 8 even though that extra route/channel pair is not emitted in the checked-in Embassy artifact surface
- **Direct vs inherited lowering data:** retained Embassy drivers resolve directly to extracted core per-peripheral registers/fields; topology metadata comes from explicit transitive fragments reachable from the exact-variant YAML (`family/`, `peripherals/`, `interrupts/`, `dma/`) and remains reference-resolvable without guessing
- **Omitted candidate driver instances:**
  - `gpio-port` for GPIOA/GPIOB/GPIOD — omitted because `hair generate embassy` rejects the CH32 `CFGLR/CFGHR/INDR/OUTDR/BSHR/BCR` GPIO contract as an unsupported lowering family
  - `gpio-port` for GPIOC — omitted because the exact QFN28 package exposes no GPIOC pads
  - `usb-device` for USBD — omitted because the approved evidence supports clocks/resets/interrupts/pins and core registers, but does not yet justify the endpoint/FIFO/attach/reset semantic closure needed for executable lowering
  - `can` — not emitted because `can` is outside the current first-cut supported Embassy driver subset
- **Generator result:** Embassy generation succeeds from the final HAIR document

## Metadata Coverage and Gaps

### Metadata present in approved evidence and extracted

| Metadata class | Extraction status | Supporting evidence |
| --- | --- | --- |
| Exact-variant identity, package, memory sizing, peripheral inventory | Extracted | Datasheet model table + exact-variant YAML |
| Core register-bearing MMIO model for present peripherals | Extracted | Official header typedefs + exact-family SVD + RM timer/CAN context |
| Exact QFN28 pad inventory and package filtering | Extracted | Datasheet table 3-1-3 + oscillator-pad remap note |
| Clock/reset topology for MCU/SoC profile | Extracted | Exact-variant YAML + transitive family/peripheral RCC fragments |
| Interrupt topology for retained profile blocks | Extracted | Exact-variant YAML + `CH32V2_D6.yaml` + startup vector order |
| DMA topology for ADC1, SPI1, USART1/2, I2C1, TIM1 update | Extracted | Exact-variant YAML + official ADC/TIM DMA examples |
| Minimal timer enable and ADC calibration semantics | Extracted | Official header bit definitions |
| Adafruit board-level USB-C, UART-header, and NeoPixel annotations | Extracted conservatively | Commit-pinned Adafruit schematic + Learn overview/pinouts |

### Metadata present in approved evidence but not extracted

| Metadata class | Status | Reason |
| --- | --- | --- |
| Embassy `gpio-port` driver instances | Omitted from `profiles.embassyHal.driverInstances` | Current generator limitation; structural/package evidence exists but lowering fails on CH32 GPIO layout |
| Embassy `usb-device` driver instance | Omitted from `profiles.embassyHal.driverInstances` | Pins/clocks/interrupts/core registers are present, but approved evidence examined here does not justify executable attach/reset/endpoint/FIFO semantics |

### Metadata not confidently recoverable from approved evidence examined in this pass

| Metadata class | Status | Why still absent |
| --- | --- | --- |
| Exact reservedness of family-shared RCC/AFIO control bits for absent peripherals (`USART3/UART4/SPI2/I2C2/USBFS host`) | Not recovered | Official header/SVD are D6/family-shared and the exact-variant approved sources do not provide a narrower per-bit reservedness map |
| Richer peripheral state machines beyond timer enable and ADC calibration | Not recovered | Approved sources examined in this pass do not provide audited executable semantic sequences for every supported peripheral mode |
| Executable USB device lowering semantics | Not recovered | No approved source in this run closes the attach/reset/endpoint/FIFO behavior surface |
| Exact USB D+/D- pin identity from Adafruit board collateral | Not recovered into board annotations | The Adafruit schematic's USBD pin labels conflict with the WCH exact-variant package/pin evidence, so WCH remains authoritative for MCU USB pin identity |

## Normalization and Completeness Matrix

| Approved source | Metadata class | Present in evidence | Extracted | Normalization required | Canonical mapping assigned | Remaining unresolved inventory | Root cause |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `wch-ch32v203-datasheet` | Model table + package/pin tables | Yes | Yes | Yes — family table + exact QFN28 pad filtering | N/A | GPIOC has no bonded pads; PB8/PB9/PB10-15/PC13-15/PA8 routes were removed from exact-package topology | Family-vs-package representational difference |
| `ch32-rs-ch32v203g6u6-yaml` + transitive family/peripheral fragments | Exact-variant profile topology | Yes | Yes | Yes — shared D6/family fragments pruned to exact variant | Yes | CAN1/USBD kept in MCU/SoC profile; Embassy-only omissions are explicit | Exact-variant pruning + generator boundary |
| `openwch-ch32v20x-header` + `ch32-rs-ch32v203xx-svd` + RM | Core register model and metadata | Yes | Yes | Yes — shared family register layout retained while exact peripheral inventory is pruned | Partially | Shared RCC/AFIO fields still name absent-peripheral controls such as `USART4EN`, `USART4RST`, `USART4_RM`, `SPI2EN`, `I2C2EN`, `TIM5CH4RM`, and `ETHRM` | Missing exact-variant per-bit reservedness evidence |
| `openwch-ch32v20x-adc-dma-example` + `openwch-ch32v20x-tim-dma-example` | DMA direction evidence | Yes | Yes | No | N/A | None identified for extracted routes | None |
| `adafruit-qt-py-ch32v203-schematic` + Learn pages | Board-level USB/UART/NeoPixel facts | Yes | Yes, conservatively | Yes — board collateral was attached as extensions instead of overriding MCU-native pin identity | N/A | USB-C exact D+/D- MCU pin labels were not imported from the board schematic | Lower-authority board source conflicts with WCH package map |

## Overlay Reconciliation Table

None identified. The exact variant keeps only the device-side USB block; the USBFS host overlay from the neighboring C8T6 prior-art bundle was intentionally not carried into the final G6U6 extraction.

## Epistemic Claims Ledger

| Claim or entity | Label | Supporting evidence or reason |
| --- | --- | --- |
| CH32V203G6U6 is a QFN28, 32KB flash / 10KB SRAM exact variant with 24 GPIO, USART1/2, SPI1, I2C1, CAN1, and USBD. | [KNOWN] | Datasheet model table + exact-variant YAML |
| USBFS/USBHD host-side functionality is absent on the exact G6U6 variant. | [KNOWN] | Datasheet model table + exact-variant YAML include list |
| PD0 and PD1 are only package-valid via oscillator-pad remap. | [KNOWN] | Datasheet QFN28 table + pin-remap note |
| The Adafruit QT Py CH32V203 USB-C connector is wired to the MCU's native USB device path rather than to a discrete onboard USB-UART bridge. | [KNOWN] | Adafruit overview/pinouts + schematic D+/D- vs separate TX/RX nets |
| The Adafruit QT Py CH32V203 header UART uses PA2 for TX and PA3 for RX. | [KNOWN] | Adafruit pinouts + schematic TX/RX nets |
| The Adafruit QT Py CH32V203 onboard NeoPixel data line is connected to PA4. | [KNOWN] | Adafruit pinouts + schematic NEOPIX net |
| GPIOC should remain as a core register block even though the exact package exposes no GPIOC pads. | [INFERRED] | Family header / family YAML preserve GPIOC; exact package filtering removes all GPIOC pads |
| Shared D6 startup/header interrupt data must be pruned against exact-variant peripheral inventory rather than copied wholesale. | [INFERRED] | Startup/header enumerate subgroup superset, while datasheet model table and exact YAML narrow the exact part |
| CH32 GPIO ports are executable-ready for the current Embassy generator. | [ASSUMPTION] | Rejected by generator failure; omitted from final Embassy profile |

## Conflicts and Clarification Questions

- **Resolved conflict:** the D6 startup/header enumerate a subgroup superset including USART3/UART4/SPI2/I2C2/USBFS-host signals, while the exact G6U6 model table and YAML narrow the exact part to USART1/2, SPI1, I2C1, CAN1, and USBD. The final extraction prunes the absent peripherals and their exact-variant profile routes/interrupts.
- **Resolved conflict:** family evidence preserves a GPIOC register block, but the QFN28 package exposes no GPIOC pads. The core register block remains, but no GPIOC pin routes or Embassy driver instance are emitted.
- **Resolved conflict:** the family memory-map evidence names a USBFS window at `0x5000_0000`, but the exact G6U6 variant lacks USBFS/USBHD. The final exact-variant `memoryMap` removes that region and keeps only the device-side USBD block at `0x40005C00`.
- **Resolved conflict:** the Adafruit schematic labels the USB-C data pair on IC2 `PA10/USBD-` and `PA12/USBD+`, but the WCH exact-variant QFN28 package map and existing WCH-backed USB routes remain authoritative for MCU pin identity. The final HAIR records the board's USB-C attachment at the `periph.usbfsd` level and does not replace the WCH-backed PA11/PA12 USB routes.
- **Clarification questions:** None identified.

## Rejected Candidate Claims

| Candidate claim | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| CH32V203G6U6 should inherit USART3/UART4/SPI2/I2C2/USBFS host from the neighboring C8T6 bundle. | Rejected: exact-variant model table and YAML exclude those peripherals. | Datasheet model table + `CH32V203G6U6.yaml` include list |
| PB8/PB9/PB10-15/PC13-15/PA8 routes are valid on the exact QFN28 package. | Rejected: the approved QFN28 pin-definition table does not expose those pads. | Datasheet table 3-1-3 |
| Embassy `gpio-port` drivers can be claimed for CH32 GPIOA/B/D from topology evidence alone. | Rejected: the generator explicitly reports the CH32 GPIO register contract as unsupported. | `hair generate embassy` failure before gpio-driver omission |
| USB device Embassy readiness can be claimed from clocks, interrupts, and D+/D- pins alone. | Rejected: lowering-critical attach/reset/endpoint/FIFO semantics remain unsupported by the approved evidence examined here. | Embassy contract + absence of auditable USB semantic closure |
| The clean G6U6 extraction should mechanically copy the C8T6 USB host overlay. | Rejected: the user selected a clean exact-variant extraction and the approved evidence says USBFS/USBHD are absent. | User-approved scope + datasheet model table + exact-variant YAML |
| The Adafruit QT Py CH32V203 onboard RGB LED data line is on PA7. | Rejected: both the Adafruit pinouts page and the Adafruit schematic route the NeoPixel data net to PA4 instead. | Adafruit pinouts + schematic NEOPIX net |
| The Adafruit QT Py CH32V203 USB-C connector is attached to an onboard USB-UART bridge. | Rejected: the schematic shows D+/D- routed directly to IC2 USBD-labeled pins and separate TX/RX nets routed to PA2/PA3. | Adafruit overview + schematic USB/UART nets |
| The Adafruit schematic's exact USB D+/D- pin labels should override the WCH exact-variant package map. | Rejected: board collateral is lower authority than the WCH package/pin table, and the labels conflict. | WCH QFN28 package map + existing WCH-backed USB routes |

## Unresolved Differences Inventory

- **Family-shared RCC/AFIO control fields:** the retained family header/SVD register model still carries fields such as `USART4EN`, `USART4RST`, `USART4_RM`, `SPI2EN`, `SPI2RST`, `I2C2EN`, `I2C2RST`, `TIM5CH4RM`, and `ETHRM`. The exact approved evidence narrows the peripheral inventory, but does not provide an exact-variant per-bit reservedness map.
- **GPIOC core/profile split:** `periph.gpioc`, `block.gpioc`, `clk.gpioc`, and `rst.gpioc` remain because family-approved evidence preserves the block, but the QFN28 package exposes no GPIOC pads and the Embassy profile emits no GPIOC driver instance.
- **GPIO Embassy readiness gap:** GPIOA/GPIOB/GPIOD routes remain in `profiles.mcuSoc.pinTopology`, but `profiles.embassyHal.driverInstances` omit `gpio-port` because the current generator cannot lower the CH32 GPIO layout.
- **USB device Embassy readiness gap:** the core USBD block and MCU/SoC topology remain, but no `usb-device` Embassy driver instance is emitted because the approved evidence examined here does not yet justify lowerable attach/reset/endpoint/FIFO semantics.
- **Semantic depth gap:** only timer enable/disable and ADC calibration/init operations/state machines were confidently grounded in this pass.
- **Board-collateral USB pin-label conflict:** the Adafruit schematic directly supports native USB-C attachment, but its exact USBD pin labels disagree with the WCH exact-variant package map. The final HAIR keeps WCH-backed PA11/PA12 USB pin routes and limits the board annotation to the `periph.usbfsd` / connector level.

## Coverage

- **Examined:** manifest, repository schema/profile docs, exact-variant manifest, official datasheet pages 7 and 29-31, official header/startup/linker assets, official ADC/TIM DMA examples, exact-family SVD, exact-variant YAML, transitive family/peripheral/interrupt/DMA YAML fragments reachable from that approved source, and Adafruit board collateral (commit-pinned schematic plus Learn overview/pinouts)
- **Method:** manifest-grounded clean extraction; exact-variant pruning of family-level structural/profile data; package-aware pin-route filtering; direct schematic net tracing for board facts; adversarial comparison against both the neighboring C8T6 bundle and the Adafruit board collateral; schema validation; Embassy generator translation-boundary check
- **Excluded:** the C8T6 bundle as direct provenance, unsupported Embassy driver kinds, and any USB/GPIO lowering behavior not justified by approved evidence or current generator support
- **Limitations:** official vendor documents are family-level; some family-shared register fields remain structurally present even when the exact variant omits the corresponding peripheral; GPIO and USB Embassy omissions are explicit rather than guessed around; Adafruit board collateral is lower authority than WCH for MCU pin identity and contains USB pin labels that conflict with the WCH package map

## Limitations

- The final Embassy profile is intentionally narrower than the core/MMIO extraction: no `gpio-port` drivers, no `usb-device` driver, and no unsupported `can` driver instance are emitted.
- The final HAIR keeps family-shared RCC/AFIO register fields that may be reserved on the exact G6U6 variant because the approved evidence does not provide a narrower exact-variant bit-reservedness map.
- GPIOC remains structurally modeled but package-unbonded.
- The extraction relies on family-level official datasheet/reference-manual/header material plus exact-variant community topology sources; that combination is auditable, but not as strong as a vendor-authored G6U6-only machine-readable register artifact.
- The Adafruit board collateral is useful for connector/header/NeoPixel facts, but it is not authoritative enough to replace the WCH MCU pin map where the two disagree.
