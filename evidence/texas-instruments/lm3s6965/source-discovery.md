# HAIR Evidence Discovery Report

## Target Device Summary

- **Requested board/model anchor**: `lm3s6965evb`
- **Installed emulator support checked**: WSL2 QEMU 11.0.50 reports `lm3s6965evb` as `Stellaris LM3S6965EVB (Cortex-M3)`
- **Chosen extraction target**: `LM3S6965`
- **Vendor**: Texas Instruments
- **Family**: Stellaris
- **Architecture**: ARM Cortex-M3
- **Local evidence provided**: None

The manifest targets the MCU, not the board, because HAIR documents one concrete device variant. Board- and QEMU-level sources are retained only to anchor the user's requested emulator target to that MCU and to constrain emulated coverage.

## Search Strategy

1. Grounded the repository schema and extraction workflow requirements locally.
2. Verified that the user's installed WSL2 QEMU actually supports `lm3s6965evb`.
3. Searched official TI sources first for exact-device documentation and software.
4. Searched official Arm sources for Cortex-M3/NVIC/SysTick core documentation.
5. Searched official QEMU documentation only to confirm the requested emulator target and modeled coverage.
6. Searched for structured or header-level register artifacts and treated non-official sources as gap-fillers only.

## Local Evidence Inventory

None identified.

## Discovered Source Inventory

| Source id | Kind | Location | Authority | Extraction value | Scope notes |
| --- | --- | --- | --- | --- | --- |
| `ti-lm3s6965-product-page` | other | https://www.ti.com/product/LM3S6965 | official | medium | Exact-device landing page and documentation hub |
| `ti-lm3s6965-datasheet` | datasheet | https://www.ti.com/lit/ds/symlink/lm3s6965.pdf | official | high | Primary MCU documentation for structure, memory, peripheral inventory, and device identity |
| `arm-cortex-m3-trm` | reference-manual | https://developer.arm.com/documentation/100165/0201/ | official | high | CPU core, NVIC, and SysTick details |
| `ti-sw-drl` | sdk | https://www.ti.com/tool/SW-DRL | official | high | Official TI statement that the legacy SDK includes per-device headers for direct register access |
| `ti-stellaris-driverlib-guide` | other | https://www.ti.com/lit/ug/spmu019p/spmu019p.pdf | official | medium | Driver-library semantics and usage patterns for targeted peripherals |
| `ti-lm3s6965evb-user-guide` | other | https://www.ti.com/lit/ug/spmu029a/spmu029a.pdf | official | medium | Board context tying the requested EVB/QEMU target to LM3S6965 |
| `ti-lm3s-class-errata` | errata | https://www.ti.com/lit/er/spmz850g/spmz850g.pdf | official | medium | Defensive cross-check for disputed or corrected silicon behavior |
| `cmsis-lm3s6965-header-mirror` | vendor-header | https://raw.githubusercontent.com/speters/CMSIS/c5da166f32162dd8ddccf9e9e2fdbd961b0f53c2/Device/TI/LM3S/Include/LM3S6965.h | community mirror | high | Directly accessible machine-readable IRQ/register definitions; gap-filler for legacy TI SDK packaging friction |
| `ti-driverlib-mirror-hw-gpio` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_gpio.h | community mirror | high | TI-derived GPIO register offsets and bit masks |
| `ti-driverlib-mirror-hw-sysctl` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_sysctl.h | community mirror | high | TI-derived clock/reset/system-control register addresses and field masks |
| `ti-driverlib-mirror-hw-uart` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_uart.h | community mirror | high | TI-derived UART register offsets and field masks |
| `ti-driverlib-mirror-hw-ssi` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_ssi.h | community mirror | high | TI-derived SSI register offsets and field masks |
| `ti-driverlib-mirror-hw-i2c` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_i2c.h | community mirror | high | TI-derived I2C register offsets and field masks |
| `ti-driverlib-mirror-hw-timer` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_timer.h | community mirror | high | TI-derived GPTM register offsets and field masks |
| `ti-driverlib-mirror-hw-watchdog` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_watchdog.h | community mirror | high | TI-derived watchdog register offsets and field masks |
| `ti-driverlib-mirror-hw-nvic` | vendor-header | https://raw.githubusercontent.com/kroesche/ti_driverlib/f3a614b90ff1817cbc55f4f1f01582ba039f144c/inc/hw_nvic.h | community mirror | high | TI-derived NVIC register offsets and field masks |
| `qemu-stellaris-docs` | other | https://www.qemu.org/docs/master/system/arm/stellaris.html | official QEMU | low | Confirms modeled board and emulator-visible coverage only |

## Recommended Manifest Sources

### Required

1. `ti-lm3s6965-product-page` — authoritative vendor landing page for exact-device identity and documentation hub.
2. `ti-lm3s6965-datasheet` — primary silicon source for memory, peripherals, addresses, and device-level structure.
3. `arm-cortex-m3-trm` — authoritative core source for Cortex-M3 CPU, NVIC, and SysTick facts.
4. `ti-sw-drl` — official TI evidence that per-device register headers exist in the legacy SDK.

### Recommended

1. `ti-stellaris-driverlib-guide` — useful for semantic and control-flow interpretation of clock gating, GPIO, UART, SSI, I2C, timers, flash, and watchdog operations.
2. `ti-lm3s6965evb-user-guide` — useful only for requested board/QEMU anchoring and related board context.
3. `ti-lm3s-class-errata` — needed for adversarial review of silicon claims.

### Optional / gap-filling

1. `cmsis-lm3s6965-header-mirror` — not an official TI-hosted copy, but directly consumable and high value for register definitions.
2. `ti-driverlib-mirror-hw-gpio`, `ti-driverlib-mirror-hw-sysctl`, `ti-driverlib-mirror-hw-uart`, `ti-driverlib-mirror-hw-ssi`, `ti-driverlib-mirror-hw-i2c`, `ti-driverlib-mirror-hw-timer`, `ti-driverlib-mirror-hw-watchdog`, and `ti-driverlib-mirror-hw-nvic` — community mirrors of TI legacy DriverLib headers, approved after discovery because they preserve field-level masks and offsets that are difficult to retrieve directly from the gated legacy TI package.
3. `qemu-stellaris-docs` — not a silicon authority; retain only to constrain the requested emulator target.

## Rejected Candidate Sources

| Candidate source | Reason rejected | Safer alternative |
| --- | --- | --- |
| TivaWare / TM4C SDK pages | Different MCU family; too easy to import neighboring-part assumptions | TI LM3S6965 datasheet and TI SW-DRL |
| Distributor product listings | Lower authority than TI and Arm originals | TI product page and official PDFs |
| Unofficial datasheet mirrors | Redundant while official TI PDFs remain available | TI datasheet and errata URLs |
| Raw QEMU board source file | Redundant with official QEMU docs for this workflow and not needed as primary extraction evidence | `qemu-stellaris-docs` |
| Keil/pack storefront pages | Useful hint, but not needed once stronger TI/Arm sources and directly browsable header mirrors were found | TI SW-DRL plus Arm core docs |

## Coverage Gaps

- No clear official TI-hosted SVD/IP-XACT/SystemRDL artifact was found for LM3S6965.
- The official TI SDK appears to be legacy-packaged rather than directly browsable as raw files, so the directly accessible field-level header evidence currently comes from approved community mirrors.
- Package-ordering-code granularity for the exact EVB-mounted MCU was not made a manifest requirement because the requested extraction target is the MCU model underlying the supported QEMU board, and the primary extraction scope is register/peripheral coverage rather than manufacturing-order-code provenance.

## Coverage

- **Examined**: repository schema/docs, local repo contents, WSL2 QEMU machine support, TI product/docs pages, Arm core docs, QEMU docs, community-accessible header mirror
- **Method**: local inventory first, then targeted official-source discovery, then gap-filler identification, then adversarial pruning
- **Excluded**: TivaWare-as-primary-source, unofficial PDF mirrors, distributor listings, redundant raw QEMU source
- **Limitations**: legacy packaging of official TI software may complicate direct header ingestion during extraction; structured machine-readable vendor metadata is weaker than for newer MCU families

## Follow-Up Recommendations

1. Begin extraction from the datasheet, Arm TRM, and TI SDK/guide set.
2. Use the header mirror only as a cross-check or gap-filler where direct TI sources are sparse or operationally inaccessible.
3. Keep QEMU material segregated from silicon authority: use it to confirm emulator-target alignment, not to invent missing hardware facts.
4. During extraction, challenge any claim that depends solely on community mirror headers or board/QEMU documents unless it is corroborated by TI or Arm primary sources or clearly traceable to the legacy TI DriverLib revision they mirror.
