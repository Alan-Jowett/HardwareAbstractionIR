# HAIR MCU and SoC profile

This document describes the **MCU/SoC profile layer** that sits on top of the core HAIR schema.

The core HAIR schema is intentionally general. It can describe devices, peripherals, registers, semantics, physical facts, provenance, normalization, and validation. The MCU/SoC profile adds a more opinionated layer for the hardware patterns that show up repeatedly in real MCU and SoC families.

The profile is defined by `schema/profiles/mcu.json` and is mounted into the top-level document at:

```json
{
  "profiles": {
    "mcuSoc": { }
  }
}
```

## Why this layer exists

The generic HAIR layers are expressive, but they leave many MCU-specific conventions unconstrained.

For example:

- a CH32V203 mostly looks like a classic fixed-function MCU
- an ESP32-C3 includes routing fabrics, GPIO matrix behavior, interrupt matrix behavior, low-power domains, and more complex memory views

Both devices can be described using the core schema, but generators and validators benefit from a stronger, more canonical vocabulary for common MCU and SoC shapes.

The MCU/SoC profile provides that vocabulary without replacing the core schema.

## Design principle

The profile should be understood as:

**core HAIR facts + canonical MCU/SoC classification and topology**

The core layers still carry the raw hardware description. The profile layer adds:

- canonical block classes
- topology-level relationships
- reusable MCU/SoC concepts
- stronger expectations for validation and generation

## Main sections

The profile currently defines these major sections:

- `profileKind`
- `deviceClass`
- `executionModel`
- `canonicalBlocks`
- `routingFabrics`
- `interruptTopology`
- `clockResetTopology`
- `memoryTopology`
- `pinTopology`
- `lowPowerTopology`
- `securityTopology`
- `busTopology`

## `profileKind`

Fixed to:

```json
"mcu-soc"
```

This identifies the profile as the canonical HAIR layer for MCU- and SoC-style devices.

## `deviceClass`

This gives the broad device shape, such as:

- `mcu`
- `wireless-mcu`
- `mixed-signal-mcu`
- `soc`
- `wireless-soc`
- `application-soc`

This helps distinguish simpler STM32-like or CH32-like devices from richer SoCs such as ESP32-class parts.

## `executionModel`

This describes the top-level execution architecture by reference.

It can point to:

- CPU cores
- primary interrupt controllers
- debug blocks
- timer blocks

This is useful for describing what software execution fundamentally targets.

## `canonicalBlocks`

This is the most important part of the profile.

A canonical block binds a **core HAIR entity** to a **standardized MCU/SoC block class**.

Examples of block classes include:

- `cpu-core`
- `gpio-port`
- `gpio-matrix`
- `io-mux`
- `interrupt-controller`
- `interrupt-matrix`
- `clock-controller`
- `reset-controller`
- `rtc-controller`
- `efuse-controller`
- `flash-controller`
- `cache-controller`
- `mmu`
- `timer-basic`
- `timer-general`
- `timer-advanced`
- `watchdog`
- `uart`
- `spi`
- `i2c`
- `adc`
- `dma-controller`

Each canonical block references a target entity from the core layers, usually a peripheral or controller in `structure`, and classifies it in a reusable way.

This is what lets downstream tooling say things like:

- “find all timers”
- “find the GPIO matrix block”
- “find the flash controller”
- “find the interrupt controller for this CPU”

### Example

```json
{
  "id": "sys-gpioa",
  "name": "GPIOA canonical block",
  "targetRef": "gpioa",
  "blockClass": "gpio-port",
  "functionalDomain": "peripheral-io",
  "gpioCount": 16,
  "supportsAlternateFunctions": true
}
```

## `routingFabrics`

This section models configurable routing planes that are common in more complex SoCs.

Examples:

- GPIO matrix
- IO mux
- interrupt matrix
- DMA routing
- clock routing
- signal crossbars

This is essential for devices like the ESP32-C3, where internal signals are not always hardwired to pins or CPU interrupt lines.

Each routing fabric can describe:

- the kind of routing block
- the source entities
- the sink entities
- whether routing is hardwired, muxed, or matrix-based
- which registers or fields control the routing

## `interruptTopology`

This section models interrupt behavior beyond a flat list of interrupt definitions.

It distinguishes:

- interrupt sources
- interrupt controllers
- routing to CPU-visible lines
- configurable versus fixed routes

This matters for:

- classic NVIC-style MCU interrupt models
- PLIC-style controllers
- interrupt matrices and crossbars

## `clockResetTopology`

The core physical layer already has clocks and reset domains, but the MCU/SoC profile adds a more canonical view of how blocks consume clocks and resets.

It can describe:

- which controller manages clocks
- which controller manages resets
- how clocks bind to consumers
- whether the binding is direct, gated, divided, or selectable
- how reset delivery works for a given block

This helps generators and validators understand system bring-up dependencies.

## `memoryTopology`

This section is designed for devices where the memory system is more complex than a simple list of flash and RAM regions.

It distinguishes:

- **backing stores** such as flash, ROM, SRAM, eFuse, or cache RAM
- **memory views** that software actually sees

This is especially important for SoCs like ESP32-C3 where executable flash views may be exposed through cache or MMU translation rather than as a simple direct physical region.

## `pinTopology`

The core physical layer already models pins and alternate functions.

The MCU/SoC profile adds a more canonical topological summary of:

- pad references
- GPIO port references
- IO mux references
- GPIO matrix references

This makes it easier to reason about classic fixed-pin MCUs and matrix-routed SoCs using the same conceptual layer.

## `lowPowerTopology`

This section captures low-power structure that often spans multiple blocks.

It can describe:

- low-power controllers
- low-power or always-on domains
- sleep states
- wake sources
- retained blocks

This is particularly useful for RTC-domain or deep-sleep-heavy SoCs.

## `securityTopology`

This section gives first-class places for security-related hardware infrastructure, including:

- eFuse
- OTP
- key stores
- secure boot blocks
- flash encryption control

This matters because these blocks are often central to modern SoCs but do not fit neatly into “just another peripheral”.

## `busTopology`

This section can model internal buses such as:

- APB
- AHB
- AXI
- RTC buses
- vendor-specific internal buses

The goal is not to create a full EDA interconnect description, but to capture enough structure to say:

- what bus exists
- what masters and slaves attach to it
- which blocks conceptually belong to which interconnect domain

## Relationship to the core schema

The MCU/SoC profile does **not** replace the core layers.

Instead:

- `structure` still describes registers, fields, peripherals, devices, and memory regions
- `semantics` still describes behavior and operations
- `physical` still describes clocks, timing, pins, packages, and electrical facts
- `normalization` still describes canonical naming and vendor abstraction
- `provenance` still explains where the information came from

The profile layer only adds a more constrained, more canonical **architectural interpretation** of those facts.

## How to use it

Use this profile when:

- the target is an MCU or embedded SoC
- you want stronger validation than the generic HAIR layer provides
- you want generators to reason about canonical block types
- you want to normalize both simple MCUs and richer SoCs into a shared model

Do not use it as a substitute for the core structural description. It is a specialization layer, not a replacement layer.

## Mental model

Think of the profile as answering questions like:

- Which block is the real GPIO controller?
- Is pin routing fixed, muxed, or matrix-based?
- Which controller owns interrupt routing?
- Which blocks are part of the low-power domain?
- Is flash directly mapped or exposed through cache/MMU views?
- What kind of timer is this: basic, general-purpose, or advanced?

Those are the kinds of questions that tend to matter for HALs, PACs, documentation generators, simulators, and validation rules.
