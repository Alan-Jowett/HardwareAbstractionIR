# HAIR Extraction Report
## Device Summary
- **Target:** ST STM32F405RGT6 for the Olimex STM32-H405 board context
- **Family:** STM32F405/415
- **Architecture:** ARM Cortex-M4F / ARMv7E-M
- **Package:** LQFP64 (`STM32F405RGTx` exact-chip topology source)
- **Requested profile scope:** core HAIR + `profiles.mcuSoc` + `profiles.embassyHal`
- **Requested Embassy driver scope:** as much of the supported first-cut subset as the evidence can defend
- **Status:** Full register-bearing structure was imported from the official STM32F405 SVD snapshot; Embassy profile coverage is limited to executable first-cut `uart`/`usart`, `spi`, `i2c`, and `interrupt` drivers, while `profiles.mcuSoc` still preserves the supporting RCC/DMA/pin/interrupt topology used to defend those bindings.

## Source Inventory
| Source ID | Kind | Location | Version | Notes | Likely HAIR layers |
| --- | --- | --- | --- | --- | --- |
| `olimex-stm32-h405-product-page` | other | https://www.olimex.com/Products/ARM/ST/STM32-H405/ | - | Board identity and board context. | metadata, physical |
| `st-stm32f405-datasheet` | datasheet | https://www.st.com/resource/en/datasheet/DM00037051.pdf | - | Official family datasheet. | structure, physical |
| `st-rm0090` | reference-manual | https://www.st.com/resource/en/reference_manual/DM00031020.pdf | - | Official peripheral reference manual. | structure, physical |
| `st-cmsis-device-header` | vendor-header | https://raw.githubusercontent.com/STMicroelectronics/cmsis-device-f4/3c77349ce04c8af401454cc51f85ea9a50e34fc1/Include/stm32f405xx.h | 3c77349ce04c8af401454cc51f85ea9a50e34fc1 | CPU flags, IRQ numbering, memory-base macros. | structure, physical |
| `st-cmsis-startup` | source-code | https://raw.githubusercontent.com/STMicroelectronics/cmsis-device-f4/3b64b7b0b546cefe84f13c65508126eea58b97d8/Source/Templates/gcc/startup_stm32f405xx.s | 3b64b7b0b546cefe84f13c65508126eea58b97d8 | Interrupt vector ordering cross-check. | structure |
| `st-stm32f4-svd-bundle` | svd | https://raw.githubusercontent.com/stm32-rs/stm32-rs/9887ffa9fb4f30fc0e928d9a2278d563eeb383c0/svd/vendor/en.stm32f4-svd.zip | 9887ffa9fb4f30fc0e928d9a2278d563eeb383c0 | Primary register-bearing structural model. | structure |
| `stm32-rs-stm32f405-patch` | generated | https://raw.githubusercontent.com/stm32-rs/stm32-rs/9887ffa9fb4f30fc0e928d9a2278d563eeb383c0/devices/stm32f405.yaml | 9887ffa9fb4f30fc0e928d9a2278d563eeb383c0 | Known SVD deltas and cleanup hints. | structure |
| `embassy-rs-stm32f405rg-json` | generated | https://raw.githubusercontent.com/embassy-rs/stm32-data-generated/8b94783cd40d22c6f3dd1e8105568f16ddb62fc7/data/chips/STM32F405RG.json | 8b94783cd40d22c6f3dd1e8105568f16ddb62fc7 | Exact-package pins, clocks, DMA, and interrupt topology. | physical, profiles.mcuSoc, profiles.embassyHal |
| `embassy-rs-embassy-stm32-cargo` | hal | https://raw.githubusercontent.com/embassy-rs/embassy/50ea4f72e54d9cdaa1f1108ba2a08fa2fa4dbdf9/embassy-stm32/Cargo.toml | 50ea4f72e54d9cdaa1f1108ba2a08fa2fa4dbdf9 | Embassy supported-chip feature list. | profiles.embassyHal |

## Major Hardware Areas Discovered
- Cortex-M4F CPU metadata, NVIC interrupt inventory, and memory-region inventory
- Full register-bearing MMIO model imported for 66 peripherals from the STM32F405 SVD snapshot
- Package-filtered pin inventory and alternate-function routing for the LQFP64 exact-chip variant
- RCC-based clock/reset bindings, interrupt routing, and DMA route candidates from exact-chip community topology
- Embassy HAL driver instances for the evidence-defended first-cut subset: uart/usart, spi, i2c, and interrupt

## Component Inventory
| Subsystem | Components discovered |
| --- | --- |
| CPU / core | Cortex-M4F CPU record, NVIC interrupt controller |
| Memory | BANK_1_REGION_1, BANK_1_REGION_2, BANK_1_REGION_3, CCMRAM, OTP, SRAM, SRAM2 |
| GPIO | GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, GPIOG, GPIOH, GPIOI |
| Serial / comms |  |
| DMA | DMA1, DMA2 |
| Timers / ADC present in structure |  |

## Metadata Coverage and Gaps
### Metadata present in approved evidence and extracted

| Metadata class | Extraction status | Supporting evidence |
| --- | --- | --- |
| Peripheral/register/field topology | Extracted for 66 peripherals, 986 registers, and 7311 fields. | Official ST SVD bundle snapshot. |
| CPU flags and IRQ numbers | Extracted. | Official CMSIS header plus startup vector cross-check. |
| Exact-package pin map and AF routes | Extracted. | `STM32F405RG.json`. |
| RCC clock/reset bindings | Extracted for topology-carrying peripherals. | `STM32F405RG.json` plus SVD-resolved RCC field references. |
| Interrupt routes | Extracted for community-topology-described peripheral signals and DMA streams. | `STM32F405RG.json`, CMSIS header, startup file. |
| DMA route candidates with RX/TX direction | Extracted conservatively. | `STM32F405RG.json`. |

### Metadata present in approved evidence but not extracted

| Metadata class | Status | Reason |
| --- | --- | --- |
| Timer/PWM lowering semantics | Omitted from `profiles.embassyHal`. | The examined evidence set did not include an equally conservative operation/state-machine extraction pass for timer mode transitions. |
| ADC lowering semantics | Omitted from `profiles.embassyHal`. | The examined evidence set did not include explicit audited init/calibration operations in this pass. |
| DMA request selector as a structured first-class field in `profiles.mcuSoc.dmaTopology` | Preserved only in route descriptions, not as its own structured property. | The current MCU profile schema has no dedicated request-selector slot for STM32F4 DMA stream/channel multiplexing. |

### Metadata not confidently recoverable from approved evidence

| Metadata class | Status | Why still absent |
| --- | --- | --- |
| Full reference-manual-derived semantic state machines for timers, ADCs, and serial protocol transactions | Not recovered in this pass. | The input set and time-bounded extraction favored conservative topology and register import over inventing behavior from generic expectations. |
| Per-field enumerated values | Largely absent. | The approved SVD snapshot does not carry enumerated value sets for STM32F405, and no richer structured enum source was approved in this pass. |

## Normalization and Completeness Matrix
| Approved source | Metadata class | Present in evidence | Extracted | Normalization required | Remaining unresolved inventory | Root cause |
| --- | --- | --- | --- | --- | --- | --- |
| `st-stm32f4-svd-bundle` | Register-bearing structural model | Yes | Yes | Minor SVD -> HAIR identifier normalization | Core peripheral views such as NVIC are not present in the vendor SVD. | Source scope limitation. |
| `embassy-rs-stm32f405rg-json` | Package pin topology | Yes | Yes | Yes | Alternate-function routes for unbonded package pads are already filtered out by the exact-chip JSON. | Exact-package community topology projection. |
| `embassy-rs-stm32f405rg-json` | DMA routes | Partially | Partially | Yes | DMA topology was preserved in `profiles.mcuSoc`, but DMA-backed Embassy driver lowering was excluded from the final executable subset because the current generator required additional executable inputs beyond the defended topology. | Generator limitation boundary. |
| `st-cmsis-device-header` + `st-cmsis-startup` | IRQ inventory | Yes | Yes | No | None identified. | None. |

## Epistemic Claims Ledger
| Claim or entity | Label | Supporting evidence or reason |
| --- | --- | --- |
| The Olimex STM32-H405 extraction target should be the STM32F405RGT6 path rather than the later GD32 substitute. | [KNOWN] | The user explicitly requested STM32-H405, and the Olimex page distinguishes the later GD32 alternative separately. |
| The package-filtered exact-chip topology should be taken from `STM32F405RG.json` while the register model comes from the official ST SVD. | [KNOWN] | The community JSON is exact-chip/package aware, while the ST SVD provides the full register structure. |
| RX/TX-labeled DMA routes are safe to classify by direction. | [KNOWN] | The community topology explicitly labels the signal as `RX` or `TX`. |
| Timer/ADC Embassy lowering is ready without an explicit semantics pass. | [ASSUMPTION] | Rejected; those driver kinds were intentionally omitted from `profiles.embassyHal`. |

## Conflicts and Clarification Questions
- **Resolved conflict:** the Olimex product page mentions later GD32 substitutions. This extraction stayed on the STM32F405RGT6 path because that is the user-requested target.
- **Resolved conflict:** the official ST SVD is family-level and includes peripherals whose pins are not bonded out on the LQFP64 package. The exact-chip community JSON was used only to package-filter pins and topology.
- **Clarification questions:** None identified.

## Rejected Candidate Claims
| Candidate claim | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| The board-context extraction should switch to GD32F405RGT6 because the current Olimex page mentions it. | Rejected: that would contradict the requested STM32 target. | User request plus the product-page distinction between STM32-H405 and GD32-H405. |
| Every STM32F405 timer/ADC block is Embassy-lowering-ready from the topology data alone. | Rejected: first-cut timer/pwm/adc drivers require semantic lowering inputs that were not extracted here. | Local `docs/embassy-hal-profile.md` contract. |
| Every DMA request in the exact-chip topology can be emitted as a lowering-ready route. | Rejected: non-RX/TX signals do not carry explicit transfer direction in the approved structured source. | `STM32F405RG.json` route inventory. |

## Unresolved Differences Inventory
- The physical/topology model is exact-package-filtered, but the core register-bearing structure remains family-level because the official ST SVD is family-level.
- DMA request selector values are preserved textually in `dmaTopology.routes[].description`, not as a dedicated structured property.
- `profiles.embassyHal` intentionally excludes `rcc`, `gpio-port`, `dma`, `timer`, `pwm`, and `adc` driver instances from the final executable subset; the supporting topology remains in `profiles.mcuSoc`.

## Coverage
- **Examined**: manifest, repository schemas/docs, official CMSIS header, official startup file, official STM32F405 SVD bundle snapshot, stm32-rs STM32F405 patch recipe, Embassy exact-chip JSON, Embassy chip feature manifest, and Olimex product page
- **Method**: manifest authoring, scoped source reads, SVD-to-HAIR structural import, package-filtered topology import, conservative Embassy profile synthesis, schema validation, and downstream generation
- **Excluded**: GD32 substitution path, non-Embassy-supported peripheral classes, and Embassy `rcc`/`gpio-port`/`dma`/`timer`/`pwm`/`adc` drivers that did not survive executable-lowering checks
- **Limitations**: ST PDF documents were referenced through approved source metadata, but this pass primarily operationalized the official header/startup/SVD and exact-chip structured topology because those were immediately machine-readable and auditable

## Limitations
- The top-level device is concrete to STM32F405RGT6, but several official ST inputs are family-level STM32F405/415 documents.
- The SVD import does not recover rich enumerated values for the STM32F405 field set.
- Embassy `rcc`, `gpio-port`, and `dma` driver instances were defensible at the topology level but did not survive the current executable-lowering contract; `timer`, `pwm`, and `adc` remain out of scope until a dedicated semantics pass grounds their operations and state machines.
