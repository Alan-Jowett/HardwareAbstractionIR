# HAIR Embassy HAL profile

This document defines the intended first-cut contract for `profiles.embassyHal`.

It is a **governing specification** for Embassy-style HAL generation. The current Rust CLI implements the first-cut contract described here, but later profile revisions may still extend beyond the current generator; generators are expected to follow this contract rather than inventing their own binding rules.

## Design intent

`profiles.embassyHal` is an optional generation-oriented layer.

It does **not** duplicate hardware facts. Instead, it references:

- core HAIR structure and semantics
- physical clocks, pins, and electrical facts
- canonical MCU topology from `profiles.mcuSoc`

This keeps the hardware model auditable and reusable while still giving an Embassy generator an explicit lowering contract.

## Reference resolution contract

The Embassy profile uses `entityRef` strings heavily (`targetRef`, `clockBindingRefs`, `interruptRouteRefs`, `dmaRouteRefs`, `pinRoles[].routeRefs`, and semantic-operation refs).

These references are **not** resolved by JSON Schema alone.

The generator must:

1. resolve every referenced entity explicitly
2. verify that the referenced entity is of the expected kind
3. fail with a deterministic error when a reference is missing, ambiguous, or typed incompatibly

## First-cut supported subset

The first Embassy generator cut is expected to support this driver subset:

| `driverKind` | First-cut status | Notes |
| --- | --- | --- |
| `rcc` | supported | Generates clock/reset bring-up support from `clockResetTopology` and referenced operations. |
| `gpio-port` | supported | Generates GPIO port and pin-level alternate-function support from `pinTopology.routes`. |
| `uart` / `usart` | supported | Requires explicit pin-routing data. Interrupt and DMA routes are required for async DMA-backed transfers and may be omitted for pure polling-mode instances. |
| `spi` | supported | Requires explicit pin-routing data and any claimed DMA bindings. |
| `i2c` | supported | Requires explicit pin-routing data and any claimed interrupt/DMA bindings. |
| `timer` / `pwm` | supported | Requires state-machine and route data for the supported operating modes being generated. |
| `adc` | supported | Requires calibration/init operations and any claimed DMA bindings. |
| `dma` | supported | Generates DMA infrastructure from `dmaTopology`. |
| `interrupt` | supported | Generates IRQ enums/bindings from the device interrupt inventory plus `interruptTopology`. |
| `custom` | unsupported in the first cut | Must fail explicitly rather than generating placeholders. |

Any other generation request is out of subset for the first cut and must fail explicitly.

## Required document surfaces by supported driver kind

The table below defines the minimum expected evidence-backed HAIR surfaces for first-cut generation.

| `driverKind` | Minimum required supporting data |
| --- | --- |
| `rcc` | `profiles.mcuSoc.clockResetTopology`; referenced `clockBindingRefs` / `resetBindingRefs` where used; `semantics.operations` for startup, enable, disable, or release sequences when the hardware requires them |
| `gpio-port` | `profiles.mcuSoc.pinTopology.routes`; referenced electrical constraints when generation depends on them |
| `uart` / `usart` | `pinTopology.routes` always; `interruptTopology.routes` and `dmaTopology.routes` for any async DMA-backed RX/TX path being generated |
| `spi` | `pinTopology.routes`; `interruptTopology.routes` and `dmaTopology.routes` when the generated mode depends on them |
| `i2c` | `pinTopology.routes`; `interruptTopology.routes` and `dmaTopology.routes` when the generated mode depends on them |
| `timer` / `pwm` | `pinTopology.routes` for exposed channels; `semantics.stateMachines` and `semantics.operations` for mode transitions and safe enable/disable ordering |
| `adc` | `semantics.operations` for calibration/init; `pinTopology.routes` or pin/electrical data for exposed analog inputs; `dmaTopology.routes` when async buffered conversion is generated |
| `dma` | `profiles.mcuSoc.dmaTopology.routes` and the referenced `dmaTopology.channels` entries in the first cut; shared-channel-group metadata remains valuable supporting topology but is not yet a first-cut hard requirement for generation |
| `interrupt` | `structure.device.interrupts`, `profiles.mcuSoc.interruptTopology.routes`, and the referenced `interruptTopology.sources` entries in the first cut |

## Authoring rules for `profiles.embassyHal`

1. `driverInstances[].targetRef` should resolve to a `profiles.mcuSoc.canonicalBlocks[]` entry in the first cut.
2. `driverInstances[].modulePath` is the generator-facing Rust module path for the emitted driver or support module.
3. `pinRoles[].signal` is required and must agree with the referenced `pinTopology.routes[].signal`.
4. A driver may omit `dmaRouteRefs` only when the generated first-cut implementation does not claim DMA-backed operation for that path.
5. Unsupported optional hardware must produce an explicit generator error, not a stub or silently degraded driver.

## Relationship to the CLI contract

`docs/cli.md` defines the CLI surface.

For Embassy generation specifically:

- the command is `hair generate embassy <input> --output-dir <path>`
- generation is multi-file, so stdout is not the primary artifact surface
- input documents outside the supported subset must fail explicitly
