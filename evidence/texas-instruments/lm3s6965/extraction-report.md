# HAIR Extraction Report

## Device Summary

- **Target device**: Texas Instruments LM3S6965
- **Board/model anchor**: `lm3s6965evb`
- **Architecture**: ARM Cortex-M3
- **Requested profile scope**: core HAIR + `profiles.mcuSoc` + `profiles.embassyHal`
- **Requested Embassy drivers**: `rcc`, `gpio-port`, `uart`, `spi`, `i2c`, `timer`, `interrupt`

## Source Inventory

Primary evidence came from the approved manifest sources:

- `ti-lm3s6965-datasheet`
- `arm-cortex-m3-trm`
- `ti-stellaris-driverlib-guide`
- `cmsis-lm3s6965-header-mirror`
- `ti-driverlib-mirror-hw-gpio`
- `ti-driverlib-mirror-hw-sysctl`
- `ti-driverlib-mirror-hw-uart`
- `ti-driverlib-mirror-hw-ssi`
- `ti-driverlib-mirror-hw-i2c`
- `ti-driverlib-mirror-hw-timer`
- `ti-driverlib-mirror-hw-watchdog`

## Major Hardware Areas Discovered

- Cortex-M3 core with NVIC, MPU, and SysTick
- 256 KiB flash and 64 KiB SRAM
- GPIO ports A-F
- UART0 and UART1
- SSI0
- I2C0
- GPTM TIMER0-TIMER3
- WATCHDOG0
- SYSCTL clock-gating and software-reset control
- FLASH_CTRL register block

## Component Inventory

| Area | Components extracted |
| --- | --- |
| Core | NVIC, SysTick |
| Memory | flash region, SRAM region, FLASH_CTRL |
| GPIO | GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF |
| Serial | UART0, UART1, SSI0, I2C0 |
| Timers | TIMER0, TIMER1, TIMER2, TIMER3, WATCHDOG0 |
| Clock/reset | SYSCTL clock/reset gates and RCC/RCC2 configuration |

## Metadata Coverage and Gaps

- **Present in approved evidence and extracted**: device identity, CPU metadata, memory sizes and base ranges, IRQ inventory, base addresses, requested MMIO register blocks, core clock/reset gate bindings, requested pin routes, basic timer timeout semantics, GPIO/UART/SSI/I2C/TIMER/WATCHDOG structural register models.
- **Present in approved evidence but only partially extracted**: many per-register narrative descriptions and some field-level enumerations in the vendor manuals; GPIO masked-address alias behavior was summarized but not expanded into a 256-entry register alias model; FLASH protection and some SYSCTL capability registers were kept structurally but not exhaustively semantically normalized.
- **Not confidently recoverable from approved evidence for this executable scope**: a uniform Embassy-ready capture/PWM/RTC timer lowering contract across TIMER0-TIMER3; DMA-backed UART/SPI/I2C lowering because uDMA topology was out of scope.

## Normalization and Completeness Matrix

| Source | Metadata class | Present in source | Extracted | Normalization needed | Remaining differences | Root cause |
| --- | --- | --- | --- | --- | --- | --- |
| `cmsis-lm3s6965-header-mirror` | peripheral/register topology | Yes | Yes | Yes | Shared CMSIS instance types are materialized into per-instance register ids where Embassy lowering requires local register or field refs | representational difference |
| `ti-driverlib-mirror-hw-gpio` | GPIO field masks | Yes | Yes (subset needed for scope) | No | GPIO masked-address alias window not expanded to a full arrayed register family | extraction omission recorded explicitly |
| `ti-driverlib-mirror-hw-sysctl` | clock/reset control fields | Yes | Yes | No | Sleep/deep-sleep gate families kept structurally thin outside the requested run-mode/peripheral-enable scope | scope narrowing |
| `ti-driverlib-mirror-hw-uart` | UART field masks | Yes | Yes | Yes | Same-offset `RSR`/`ECR` modeled as alternate views | overlay-preserving |
| `ti-driverlib-mirror-hw-ssi` | SSI field masks | Yes | Yes | No | DMA-related SSI fields omitted from Embassy claims | blocked by scope |
| `ti-driverlib-mirror-hw-i2c` | I2C field masks | Yes | Yes | No | No DMA topology extracted | blocked by scope |
| `ti-driverlib-mirror-hw-timer` + `ti-stellaris-driverlib-guide` | timer fields and mode semantics | Yes | Yes (basic timeout subset) | Yes | capture/PWM/RTC semantics not emitted as Embassy-ready drivers | executable-readiness blocker outside chosen subset |
| `ti-lm3s6965-datasheet` | pin/package routing | Yes | Yes (requested peripherals) | No | non-requested peripheral routes omitted | scope narrowing |

## Epistemic Claims Ledger

| Claim or entity | Label | Supporting evidence or reason |
| --- | --- | --- |
| LM3S6965 has 256 KiB flash and 64 KiB SRAM | [KNOWN] | TI datasheet memory summary and memory-map pages |
| GPIOB-F share the GPIOA register model | [KNOWN] | CMSIS header reuses `GPIOA_Type` and the datasheet describes duplicated GPIO blocks |
| UART1 shares the UART0 register model but not the same pin routes | [KNOWN] | CMSIS header reuse plus datasheet pin tables |
| TIMER1-3 share the TIMER0 structural register model | [KNOWN] | CMSIS header reuse plus common GPTM header offsets |
| Embassy timer support is safe only for basic timeout one-shot/periodic lowering | [KNOWN] | Timer chapter + DriverLib guide + approved user scope decision |
| Per-pin GPIO helpers can be lowered from aggregate GPIO port bitfields plus package pin indices | [INFERRED] | The datasheet and headers provide per-port bit semantics and fixed pin indices, but do not publish a separate per-pin field artifact |

## Conflicts and Clarification Questions

None identified after the timer-scope clarification. The only material scope choice was resolved by narrowing Embassy timer support to executable basic timeout timers.

## Rejected Candidate Claims

| Candidate claim | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| TIMER0-TIMER3 all support uniform Embassy-ready capture/PWM/RTC channel drivers | Rejected because CCP routing is not uniform across TIMER0-TIMER3 and the approved evidence did not close a uniform lowering contract | TI timer chapter + pin tables + timer semantics summary |
| UART/SPI/I2C DMA-backed Embassy drivers are in scope | Rejected because no approved uDMA topology was extracted in this run | Requested scope + missing DMA topology |
| GPIO analog-mode and port-control registers should be copied from newer LM3/TM4-style headers | Rejected because the approved LM3S6965 CMSIS register block does not expose them for this device and the generic mirrored header covers broader families | Device-specific CMSIS header is safer than family-generic extrapolation |

## Unresolved Differences Inventory

- GPIO masked-address alias region (`GPIODATA` address-mask window) is represented only by the canonical full-data register, not as a 256-alias array.
- SYSCTL sleep/deep-sleep gate registers are structurally present but not fully bound into Embassy driver instances because the requested scope centered on run-mode peripheral enable.
- FLASH protection policy registers are structurally present, but no full semantic operation model for protection programming was added because flash-controller Embassy generation was not requested.

## Overlay Reconciliation Table

| HAIR overlay view | Approved-source view | Match basis | Imported metadata classes | Residual gap |
| --- | --- | --- | --- | --- |
| `reg.uart0.rsr` + `reg.uart0.ecr` | Same-offset UART receive-status / error-clear view at offset `0x004` | overlay-preserving | register offset, access intent, field semantics needed for status vs clear | No additional gap for the requested scope |

## Embassy Readiness Summary

- **Requested scope**: `rcc`, `gpio-port`, `uart`, `spi`, `i2c`, `timer`, `interrupt`
- **Driver instances emitted**: SYSCTL/RCC, GPIOA-F, UART0/1, SSI0, I2C0, TIMER0-3, NVIC
- **Ready subset**:
  - `rcc`: peripheral gate/reset helpers
  - `gpio-port`: per-port GPIO API on package-visible pins
  - `uart`: UART0/UART1 polling/IRQ subset
  - `spi`: SSI0 polling/IRQ subset
  - `i2c`: I2C0 polling/IRQ subset
  - `interrupt`: IRQ inventory/NVIC binding subset
  - `timer`: executable basic timeout-only one-shot/periodic subset
- **Explicitly blocked from Embassy claims**:
  - timer capture/PWM/RTC channel lowering
  - DMA-backed UART/SPI/I2C APIs

## Coverage

- **Examined**: manifest, repo schemas/docs, TI datasheet, Arm Cortex-M3 TRM, TI DriverLib guide, LM3S6965 CMSIS header mirror, TI-derived mirrored `hw_*.h` headers
- **Method**: provenance-first extraction, structural synthesis from device header + TI field headers, targeted semantic extraction for clock/reset and timer timeout scope, normalization of same-offset alternate views, and generator-boundary lowering checks
- **Excluded**: non-requested peripherals, QEMU as silicon authority, DMA/uDMA lowering, timer capture/PWM/RTC executable claims
- **Limitations**: the extraction is maximum-coverage for the requested peripheral set, not a full-every-peripheral device dump; some datasheet narrative metadata remains summarized rather than exhaustively field-enumerated

## Limitations

- GPIO package availability varies by port; the Embassy GPIO instances are limited to package-visible A-F pins surfaced in the approved tables.
- TIMER0-TIMER3 Embassy support is intentionally restricted to basic timeout behavior.
- The document does not claim a full normalized system clock tree beyond the requested peripheral-enable and RCC/RCC2 configuration surfaces.
