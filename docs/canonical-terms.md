# HAIR canonical terminology seed set

This document defines the first repository-owned seed set of canonical
peripheral, register, and field terms for cross-vendor normalization.

The intent is **comparability**, not forced renaming:

1. vendor-facing names stay in `structure`
2. canonical terms live in `normalization`
3. mappings bind vendor entities to one or more canonical concepts when the
   evidence supports that equivalence

## First-cut mapping rules

1. Use canonical terms only for concepts that recur across vendors and meaningfully
   improve comparison.
2. Preserve vendor-native structure names such as `USART1`, `BRR`, `CR1`, or
   `TXE`.
3. A mapping may reference more than one canonical term when one vendor-facing
   entity bundles multiple concepts.
4. If the match is too vendor-specific or too ambiguous, omit the mapping and
   explain the gap in extraction or audit output.
5. The first cut is intentionally seeded from the concepts already exercised by
   the current reference bundles and workflows; it is not a complete MCU
   ontology.

## Seed peripheral terms

| Suggested id | Canonical term | Notes |
| --- | --- | --- |
| `term.peripheral.gpio-port` | GPIO port | Classic GPIO block exposing per-pin mode, state, and data control. |
| `term.peripheral.uart` | UART | Asynchronous serial block without synchronous clocking requirements. |
| `term.peripheral.usart` | USART | Serial block supporting synchronous/asynchronous modes. |
| `term.peripheral.spi` | SPI controller | Serial peripheral interface controller. |
| `term.peripheral.i2c` | I2C controller | Two-wire serial controller. |
| `term.peripheral.timer-basic` | Basic timer | Timer with counting/time-base behavior but limited channel features. |
| `term.peripheral.timer-general` | General-purpose timer | Timer with reusable counting/capture/compare behavior. |
| `term.peripheral.timer-advanced` | Advanced timer | Timer with richer capture/compare/PWM/control features. |
| `term.peripheral.pwm` | PWM controller | Block primarily exposed for pulse-width modulation outputs. |
| `term.peripheral.adc` | ADC | Analog-to-digital converter. |
| `term.peripheral.dma-controller` | DMA controller | Direct-memory-access controller or engine. |
| `term.peripheral.clock-controller` | Clock controller | Peripheral or system clock enable/configuration block. |
| `term.peripheral.reset-controller` | Reset controller | Peripheral or system reset control block. |
| `term.peripheral.interrupt-controller` | Interrupt controller | CPU-visible interrupt aggregation/control block. |

## Seed register terms

| Suggested id | Canonical term | Notes |
| --- | --- | --- |
| `term.register.control` | Control register | Primarily used to enable, disable, or configure a block. |
| `term.register.status` | Status register | Primarily exposes state or event flags. |
| `term.register.data` | Data register | Primarily carries transmit, receive, sample, or payload data. |
| `term.register.mode` | Mode register | Primarily selects operating modes. |
| `term.register.config` | Configuration register | Primarily carries persistent configuration settings. |
| `term.register.clock-enable` | Clock-enable register | Controls block clock gating or readiness. |
| `term.register.reset-control` | Reset-control register | Controls or reports reset assertion/deassertion. |
| `term.register.interrupt-enable` | Interrupt-enable register | Enables or masks interrupt sources. |
| `term.register.interrupt-status` | Interrupt-status register | Reports pending/raw/masked interrupt state. |
| `term.register.interrupt-clear` | Interrupt-clear register | Clears or acknowledges interrupt state. |
| `term.register.baud-rate` | Baud-rate register | Controls serial timing divisors or baud generation. |
| `term.register.prescaler` | Prescaler register | Divides an input clock or event source. |
| `term.register.counter` | Counter register | Holds a current count value. |
| `term.register.auto-reload` | Auto-reload register | Holds wrap/reload period data. |
| `term.register.compare` | Compare register | Holds compare or duty-cycle values. |
| `term.register.capture` | Capture register | Holds captured timer/input timestamps or sampled values. |

## Seed field terms

| Suggested id | Canonical term | Notes |
| --- | --- | --- |
| `term.field.enable` | Enable | Enables or disables a function or block. |
| `term.field.ready` | Ready | Reports that a block is ready for use. |
| `term.field.busy` | Busy | Reports that a block is active or not idle. |
| `term.field.mode-select` | Mode select | Selects an operating mode. |
| `term.field.clock-enable` | Clock enable | Enables clocking for a target block. |
| `term.field.reset-assert` | Reset assert | Asserts or deasserts reset. |
| `term.field.interrupt-enable` | Interrupt enable | Enables or masks an interrupt source. |
| `term.field.interrupt-pending` | Interrupt pending/status | Reports interrupt state. |
| `term.field.interrupt-clear` | Interrupt clear/acknowledge | Clears or acknowledges interrupt state. |
| `term.field.baud-divider` | Baud divider | Participates in baud-rate generation. |
| `term.field.prescaler` | Prescaler | Divides an input clock or event source. |
| `term.field.counter-value` | Counter value | Holds or reports a count value. |
| `term.field.compare-value` | Compare value | Holds a compare threshold or duty-cycle value. |
| `term.field.capture-value` | Capture value | Holds a captured sample or timestamp value. |
| `term.field.tx-data` | Transmit data | Carries transmit payload data. |
| `term.field.rx-data` | Receive data | Carries receive payload data. |

## Relationship to the schema

The first-cut schema contract expects:

- term definitions in `normalization.canonicalTerms[]`
- per-entity mappings in `normalization.mappings[]`
- one mapping target to reference one or more canonical term ids

This document is the repository-owned seed catalog that later workflows,
evidence bundles, and imported shared HAIR fragments can draw from.
