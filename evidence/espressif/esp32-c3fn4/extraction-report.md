# ESP32-C3FN4 Extraction Report

## Target

- **Device:** Espressif ESP32-C3FN4
- **Context:** Seeed Studio XIAO ESP32C3
- **Scope:** core HAIR + `profiles.mcuSoc` + `profiles.embassyHal`
- **Embassy status:** executable first-cut lowering for `rcc`, `gpio-port`, `interrupt`, `uart`, `i2c`, `spi`, and `adc`, with GDMA closure modeled for SPI2 and APB_SARADC

## Approved evidence base

1. ESP32-C3 Series Datasheet v2.4
2. ESP32-C3 Technical Reference Manual v1.4
3. ESP32-C3 errata page
4. Commit-pinned ESP-IDF SDK snapshot (`e9da155a72624fce88b8ef2cf3cde9aee2e6067f`)
5. Commit-pinned official `esp32c3.svd` (`be20aa12560889d6125d144cdb48cf615ac17628`)

## Structural model

- Imported the official SVD as the primary structural scaffold for **37 peripherals**.
- Preserved peripheral inheritance links such as `UART1 <- UART0`, `TIMG1 <- TIMG0`, and `UHCI1 <- UHCI0` where the SVD exposes them.
- Added a device-level interrupt inventory from the official `interrupts.h` source enumeration rather than inferring it from partial peripheral links alone.
- Added memory-map regions for DRAM0, IRAM0, cached IROM/DROM flash views, RTC FAST SRAM, and the `0x6000_0000` peripheral MMIO window from official ESP-IDF headers.

## Physical model

- Added QFN32 package pin mappings, including the exposed center pad.
- Modeled GPIO0-GPIO21 plus dedicated analog, clock, reset, and power pins.
- Captured the direct IO MUX routes that are explicit in the datasheet, including:
  - UART0 on GPIO20/GPIO21
  - SPI2 fixed routes on GPIO2/GPIO4/GPIO5/GPIO6/GPIO7/GPIO10
  - ADC1/ADC2 channel pins on GPIO0-GPIO5
- Recorded power-domain groupings and high-confidence clock-domain nodes.

## `profiles.mcuSoc`

The MCU/SoC layer models the high-confidence topology already defended by the approved evidence set:

- canonical blocks for CPU, SYSTEM, interrupt matrix, GPIO, IO MUX, GPIO Matrix, GDMA, UART0/1, SPI2, I2C0, APB_SARADC, and SYSTIMER
- routing fabrics for IO MUX, GPIO Matrix, and Interrupt Matrix
- GDMA topology with three RX and three TX channels plus SVD-backed SPI2/APB_SARADC route closure
- clock/reset bindings from `SYSTEM.PERIP_CLK_EN*` and `SYSTEM.PERIP_RST_EN*`
- memory topology linking cached flash views, SRAM aliasing, RTC SRAM, and register space
- pin topology for the fixed, directly evidenced routes above

## `profiles.embassyHal`

The Embassy layer now carries executable first-cut lowering inputs for the requested ESP32-C3 driver set.

Included driver instances:

- `rcc`
- `gpio-port`
- `interrupt`
- `uart` (UART0)
- `i2c` (I2C0)
- `spi` (SPI2)
- `adc` (APB_SARADC)

The driver instances preserve enough clock/reset, interrupt, pin, semantic-operation, and DMA-route closure for first-cut Embassy lowering over the approved evidence set.

## Falsification results

### Surviving claims

- The datasheet explicitly supports the chosen **ESP32-C3FN4** variant identity and QFN32 package.
- The datasheet explicitly documents the presence of **IO MUX**, **GPIO Matrix**, and the **Interrupt Matrix**, so the document does not pretend this is a conventional fixed-routing MCU.
- The SVD and ESP-IDF headers agree on the presence of the main requested bring-up blocks (`GPIO`, `IO_MUX`, `SYSTEM`, `INTERRUPT_CORE0`, `UART0/1`, `SPI2`, `I2C0`, `APB_SARADC`, `DMA`, `SYSTIMER`).

### Rejected or deferred stronger claims

- No board-specific SDA/SCL or alternate UART1 routing was emitted, because the approved manifest does not include the Seeed board wiki as a provenance source.
- No board-specific GPIO Matrix defaults were emitted beyond the directly evidenced routes, because the approved manifest intentionally excludes non-authoritative board-routing summaries.
- No full ADC calibration or conversion-sequence state machine was emitted, because those lowering-critical semantic details still need a deeper TRM pass.
- No fixed interrupt-matrix output-line routes were emitted, because the approved evidence set supports programmable routing but not one canonical CPU-line assignment for each source.

## Remaining gaps

1. The document still needs a deeper TRM pass for richer semantic operations and state machines beyond the current first-cut bring-up methods.
2. Package-wide alternate-function coverage is still conservative: fixed IO MUX and analog routes are captured, but the full GPIO-Matrix signal universe is not exhaustively enumerated here.
3. GDMA route selection is modeled from the SVD channel-select registers, but descriptor formats and transfer-completion choreography are not yet encoded as HAIR state machines.
4. Errata review has not yet been used to annotate revision-specific deviations in the device JSON.
