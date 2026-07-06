# HAIR Extraction Report

## Device Summary

- **Target:** WCH CH32V203C8T6
- **Family:** CH32V203 / CH32V20x_D6
- **Architecture:** QingKe V4B / RV32IMACXW
- **Package:** LQFP48
- **Requested profile scope:** core HAIR + `profiles.mcuSoc` + `profiles.embassyHal`
- **Requested Embassy driver scope:** all first-cut supported driver kinds for the exact variant
- **Status:** The document now preserves the full-device structural model and adds evidence-backed MCU/SoC topology, Embassy HAL profile data, timer enable state machines, and ADC enable/calibration operations. `hair generate svd` and `hair generate embassy` both complete successfully from the extracted HAIR document.

## Source Inventory

| Source ID | Kind | Location | Version | Notes | Likely HAIR layers |
| --- | --- | --- | --- | --- | --- |
| `wch-ch32v20x-30x-datasheet` | datasheet | `https://www.wch-ic.com/download/file?id=354` | 3.0 | Exact-variant package, clocks, memories, and pad inventory. | structure, physical, profiles.mcuSoc |
| `wch-ch32fv2x-v3x-rm` | reference-manual | `https://www.wch-ic.com/download/file?id=324` | 2.5 | Timer-class adjudication and timer register semantics. | structure, semantics |
| `openwch-ch32v20x-sdk` | sdk | `https://github.com/openwch/ch32v20x/tree/804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | `804daf39a21af99be64c5abe0ea4bdaf361eb2e4` | Cross-checks linker, startup, and core-support behavior. | structure, physical |
| `openwch-ch32v20x-header` | vendor-header | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Peripheral/inc/ch32v20x.h` | V1.0.1 | Official register structs, many field descriptions, timer enable bit, and ADC enable/calibration bits. | structure, semantics |
| `openwch-ch32v20x-startup-d6` | source-code | `https://raw.githubusercontent.com/openwch/ch32v20x/804daf39a21af99be64c5abe0ea4bdaf361eb2e4/EVT/EXAM/SRC/Startup/startup_ch32v20x_D6.S` | V1.0.2 | D6 interrupt-vector ordering cross-check. | structure, profiles.mcuSoc |
| `qingke-v4-processor-manual` | other | `https://www.wch-ic.com/download/file?id=367` | 1.5 | Core/PFIC context. | structure, physical |
| `ch32-rs-ch32v203xx-svd` | svd | `https://raw.githubusercontent.com/ch32-rs/ch32-rs/9b4ee66500b956bc87fbf83aa28ad245b39ebd15/svd/vendor/CH32V203xx.svd` | `9b4ee66500b956bc87fbf83aa28ad245b39ebd15` | Conservative structural metadata gap-filler retained from the prior pass. | structure |
| `ch32-rs-ch32v203c8t6-yaml` | generated | `https://raw.githubusercontent.com/ch32-rs/ch32-data/a515903589cfbc342dc6ad0d13c02b4382da5628/data/chips/CH32V203C8T6.yaml` | `a515903589cfbc342dc6ad0d13c02b4382da5628` | Exact-variant topology source for RCC bindings, interrupt routes, DMA channels, package-filtered pin routes, and remap controls. | structure, physical, semantics, profiles.mcuSoc, profiles.embassyHal |

## Major Hardware Areas Discovered

- QingKe V4B core, PFIC interrupt controller, vendor system timer, and exact-variant memory map
- Full register-bearing MMIO model for RCC, AFIO, DMA1, GPIOA-D, USART1/2/3, UART4, SPI1/2, I2C1/2, TIM1/2/3/4, ADC1/2, CAN, USBFS, FLASH, RTC, watchdogs, and support blocks
- MCU/SoC canonical blocks for clock/reset, DMA, GPIO ports, serial blocks, timers, PWM views, ADCs, and the interrupt controller
- Clock/reset topology linking exact-variant peripherals to HCLK, PCLK1, PCLK2, PCLK1_TIM, PCLK2_TIM, and ADC domains
- Interrupt topology for supported first-cut Embassy drivers plus DMA1 channel interrupts
- DMA topology for ADC1, USART1/2/3/4, SPI1/2, and I2C1/2
- Package-filtered pin and remap topology for GPIO, USART, SPI, I2C, TIM, PWM, and ADC signals
- Minimal but evidence-backed semantics for timer counter enable/disable and ADC enable/calibration

## Component Inventory

| Subsystem | Components discovered |
| --- | --- |
| CPU / core | QingKe V4B core, PFIC, vendor system timer |
| Memory | 64KB code flash, 64KB boot alias, 20KB SRAM |
| Clock / reset | RCC, HCLK, PCLK1, PCLK2, PCLK1_TIM, PCLK2_TIM, ADC, APB1/APB2/AHB reset domains |
| GPIO / mux | GPIOA, GPIOB, GPIOC, GPIOD, AFIO, exact LQFP48 pad map |
| Serial / comms | USART1, USART2, USART3, UART4, SPI1, SPI2, I2C1, I2C2 |
| Timers / PWM | TIM1 advanced timer, TIM2/TIM3/TIM4 general timers, PWM views over TIM1-TIM4 |
| Mixed-signal | ADC1, ADC2 |
| Data movement / interrupt | DMA1 with 8 channels, PFIC interrupt routes for supported drivers and DMA channels |
| Existing out-of-profile structure | CAN1, USBFS device/host overlays, RTC, FLASH, watchdogs, BKP, PWR, CRC, OPA/CMP |

## Embassy Readiness Summary

- **Requested scope:** all first-cut supported driver kinds on the exact CH32V203C8T6 variant
- **Driver instances extracted:** 25 total — RCC; GPIOA-D; USART1/2/3; UART4; SPI1/2; I2C1/2; TIM1/2/3/4; TIM1/2/3/4 PWM; ADC1/2; DMA1; PFIC interrupt
- **Supporting records added:** 26 canonical blocks, 19 clock bindings, 18 reset bindings, 34 interrupt sources/routes, 8 DMA channels, 17 DMA routes, 159 pin routes, 6 semantic operations, and 4 state machines
- **Generation result:** both downstream lowerings succeed from this HAIR document
- **Known readiness limits:** timer DMA request names remain outside the Embassy profile because the approved DMA map does not encode transfer direction; ADC2 has no DMA route because the approved DMA map lists only ADC1; UART4 is exposed only through the PB0/PB1 remapped route because the default PC10/PC11 pins are not present on the LQFP48 package

## Metadata Coverage and Gaps

### Metadata present in approved evidence and extracted

| Metadata class | Extraction status | Supporting evidence |
| --- | --- | --- |
| Canonical MCU block inventory | Extracted for RCC, AFIO, PFIC, DMA1, GPIOA-D, USART/UART, SPI, I2C, TIM, PWM, and ADC. | Official structure plus exact-variant YAML topology records. |
| Clock/reset topology | Extracted for all first-cut Embassy peripherals in scope. | `CH32V203C8T6.yaml` and included family/peripheral RCC fragments. |
| Interrupt topology | Extracted for supported drivers plus DMA1 channel interrupts. | `CH32V203C8T6.yaml`, `CH32V2_D6.yaml`, and D6 startup vector ordering. |
| DMA topology | Extracted for ADC1, USART1/2/3/4, SPI1/2, and I2C1/2. | `CH32V_V4B.yaml`. |
| Pin/remap topology | Extracted and filtered to the exact LQFP48 package. | Datasheet table 3-1-1 plus peripheral YAML pin/remap sections. |
| Timer enable semantics | Extracted as one operation plus one state machine per TIM1/TIM2/TIM3/TIM4. | `TIM_CEN` in the official header plus the timer chapters already used to classify TIM1 vs TIM2/3/4. |
| ADC enable/calibration semantics | Extracted for ADC1 and ADC2. | `ADC_ADON`, `ADC_CAL`, and `ADC_RSTCAL` in the official header. |

### Metadata present in approved evidence but not extracted

| Metadata class | Status | Reason |
| --- | --- | --- |
| Timer DMA request lines in `CH32V_V4B.yaml` | Intentionally omitted from `profiles.mcuSoc.dmaTopology.routes`. | The approved DMA map names timer request sources but does not encode transfer direction, so emitting Embassy DMA routes would require invention. |
| RCC/AFIO field-level structural records for some Embassy-only topology facts | Still absent from the core structural model. | The profile layer captures the needed clock/reset/remap relationships from approved community YAML even where the official header and current structural extraction do not expose a complete matching field record, notably for UART4 remap and several late APB1 enable bits. |

### Metadata not confidently recoverable from approved evidence examined in this pass

| Metadata class | Status | Why still absent |
| --- | --- | --- |
| Timer DMA transfer direction | Not recovered. | The approved DMA route map gives channel assignments but not a direction contract for timer request lines. |
| Broader peripheral state-machine semantics beyond timer enable and ADC calibration | Not recovered. | The approved sources examined in this pass support the minimal enable/calibration controls needed for the requested generator contract, but not a richer auditable behavior model for every supported peripheral mode. |
| Unsupported first-cut Embassy driver kinds | Not applicable. | CAN and USB remain structurally modeled in HAIR but are outside the current Embassy generator's supported driver subset. |

## Normalization and Completeness Matrix

| Approved source | Metadata class | Present in evidence | Extracted | Normalization required | Remaining unresolved inventory | Root cause |
| --- | --- | --- | --- | --- | --- | --- |
| `openwch-ch32v20x-header` + `wch-ch32fv2x-v3x-rm` | Timer enable semantics | Yes | Yes | No | None identified. | None. |
| `openwch-ch32v20x-header` | ADC enable/calibration semantics | Yes | Yes | No | None identified. | None. |
| `ch32-rs-ch32v203c8t6-yaml` + datasheet package map | Pin/remap routes | Yes | Yes | Yes | Default-package routes for absent pins were intentionally excluded, notably UART4 PC10/PC11 and ADC1/2 PC4/PC5. | Exact-variant package filtering. |
| `ch32-rs-ch32v203c8t6-yaml` + `CH32V2_D6.yaml` | Interrupt routes | Yes | Yes | Yes | Shared CAN/USB vector names remain modeled only as shared interrupt routes, not Embassy drivers. | Supported-driver boundary. |
| `ch32-rs-ch32v203c8t6-yaml` + `CH32V_V4B.yaml` | DMA routes | Partially | Partially | Yes | Timer DMA request lines omitted; ADC2 has no listed DMA route. | Direction ambiguity for timers; source sparsity for ADC2. |
| Existing official/community structural extraction | Core register-bearing MMIO model | Yes | Preserved | Previously completed | Structural gaps such as cluster prose, reset masks, and some field-level metadata remain from the prior full-device pass. | Outside this Embassy-profile enablement pass. |

## Overlay Reconciliation Table

| HAIR overlay view | Approved-source view | Comparison basis | Imported metadata classes | Residual unmapped items | Reason |
| --- | --- | --- | --- | --- | --- |
| `USBFSD` / `USBFSH` shared-base overlays | `CH32V203C8T6.yaml -> FV2x_V3x_USBD.yaml + FV2x_V3x_USBFS.yaml -> otg_v2.yaml` | Overlay-preserving | Existing USB register/field prose and enums from the prior structural pass | Small host-side prose tail retained from the prior pass | Dual-overlay topology intentionally preserved. |

## Epistemic Claims Ledger

| Claim or entity | Label | Supporting evidence or reason |
| --- | --- | --- |
| The CH32V203C8T6 Embassy profile should be package-filtered to the LQFP48 pinout. | [KNOWN] | Datasheet table 3-1-1 and the exact-variant YAML both constrain the package. |
| UART4 is available only on PB0/PB1 for this exact package. | [KNOWN] | The peripheral YAML offers default PC10/PC11 and remapped PB0/PB1 routes, but only PB0/PB1 exist on the LQFP48 package. |
| TIM1/TIM2/TIM3/TIM4 can be represented with a minimal enabled/disabled counter state machine. | [KNOWN] | `TIM_CEN` is explicitly the counter-enable bit in the official header. |
| ADC1 and ADC2 can be represented with a minimal enable/calibration operation. | [KNOWN] | `ADC_ADON`, `ADC_CAL`, and `ADC_RSTCAL` are explicitly defined in the official header. |
| PCLK1_TIM and PCLK2_TIM should be modeled as derived timer clock domains distinct from plain PCLK1/PCLK2 in the profile layer. | [INFERRED] | The approved YAML distinguishes `bus_clock: PCLKx_TIM` from `kernel_clock: PCLKx`, so the profile keeps the timer-facing derived domains explicit. |
| Timer DMA direction is safe to infer from channel names alone. | [ASSUMPTION] | Rejected; the profile omits those routes rather than guessing direction. |

## Conflicts and Clarification Questions

- **Resolved conflict:** the official header is sufficient for the core register model, but it is not sufficient by itself for the full Embassy topology because some late APB1 enable bits and UART4 remap details are not recoverable there. The profile therefore uses the approved exact-variant YAML to carry those topology facts explicitly.
- **Resolved conflict:** several community routes land on pins not present on the C8T6 package. Those routes were dropped rather than carried into the exact-variant profile.
- **Resolved conflict:** the approved DMA map includes timer request names but not a transfer-direction contract. Timer DMA routes were omitted instead of being guessed.
- **Clarification questions:** None identified.

## Rejected Candidate Claims

| Candidate claim | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| UART4 default pins PC10/PC11 are valid Embassy routes for CH32V203C8T6. | Rejected: the exact LQFP48 package does not expose PC10 or PC11. | Datasheet package pin map plus the package-filtered HAIR pin inventory. |
| ADC2 should share ADC1's DMA route because both use the combined ADC1_2 interrupt. | Rejected: the approved DMA map lists ADC1 only. | `CH32V_V4B.yaml`. |
| Timer DMA routes can be emitted with a specific direction from the approved channel map alone. | Rejected: the approved route map names timer request sources but not direction. | `CH32V_V4B.yaml`. |
| The official header alone is enough to recover every Embassy clock/reset/remap binding needed for this variant. | Rejected: the exact-variant topology still depends on approved community YAML for several clock/reset/remap facts. | `CH32V203C8T6.yaml` and included family/peripheral fragments. |

## Unresolved Differences Inventory

- **Profile-vs-core structural gap:** the Embassy profile carries some clock/reset/remap facts at topology level that are still not represented as precise field-level structure records in the core HAIR model, especially around UART4 remap and incomplete late APB1 enable coverage.
- **Timer DMA omission:** the approved DMA map's timer request lines remain unmodeled in the Embassy profile because emitting a route direction would require unsupported inference.
- **Existing structural residuals from the prior full-device pass:** cluster descriptions, reset masks, and portions of field-level prose/access/enums remain incomplete outside the Embassy-focused additions.

## Coverage

- **Examined:** manifest, repository schemas/docs, current CH32V203 HAIR/report, official header, official startup vector, RM timer chapters already used in the structural pass, datasheet package pin map, `CH32V203C8T6.yaml`, `CH32V2.yaml`, `CH32V2_D6.yaml`, and `CH32V_V4B.yaml`
- **Method:** manifest validation, package-aware topology extraction, conservative community-topology reuse, minimal semantics extraction from explicit control-bit definitions, JSON rewrite, schema validation, and downstream `svd` / `embassy` generation
- **Excluded:** timer DMA routes without explicit direction, unsupported Embassy driver kinds (for example CAN and USB), and any richer semantic behavior not directly evidenced by the approved sources examined here
- **Limitations:** the Embassy-ready profile is complete for the requested first-cut supported drivers, but it still sits on top of a core HAIR model that retains some pre-existing structural metadata gaps outside this pass

## Limitations

- The Embassy profile intentionally omits timer DMA routes because the approved DMA map does not encode direction.
- ADC2 has no DMA route in the approved evidence examined here.
- UART4 is Embassy-ready only through the PB0/PB1 remapped pins on the exact LQFP48 package.
- Some field-level RCC/AFIO structural records needed to mirror the profile topology exactly are still absent from the core HAIR register model.
- Existing non-Embassy structural residuals from the prior full-device extraction remain, including cluster prose, reset masks, and a tail of field-level metadata gaps.
