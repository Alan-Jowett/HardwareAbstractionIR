# HAIR Embassy HAL profile

This document defines the intended first-cut contract for `profiles.embassyHal`.

It is a **governing specification** for Embassy-style HAL generation. The current Rust CLI is expected to converge on the first-cut contract described here, and later profile revisions may still extend beyond the current generator; generators are expected to follow this contract rather than inventing their own binding rules.

## Design intent

`profiles.embassyHal` is an optional generation-oriented layer.

It does **not** duplicate hardware facts. Instead, it references:

- core HAIR structure and semantics
- physical clocks, pins, and electrical facts
- canonical MCU topology from `profiles.mcuSoc`

This keeps the hardware model auditable and reusable while still giving an Embassy generator an explicit lowering contract.

## Executable lowering contract

The Embassy profile does **not** promise one fixed Rust method set per
`driverKind`.

Instead, the generated API surface must be derived from the approved HAIR
inputs that the generator can actually lower into register-level code:

- referenced MCU topology records such as clock/reset, interrupt, DMA, and pin routes
- semantic operations and state machines
- structural register and field records reached through those references

Normative consequences:

1. A generator must only emit methods that it can trace back to explicit HAIR lowering inputs.
2. A generator must not emit success-shaped placeholder bodies such as `Ok(())` or `Ok(0)` when no real lowering exists.
3. If the approved HAIR inputs justify only part of a driver's possible behavior, the generated API may expose only that supported subset.
4. If the profile or chosen emitted API would require unsupported inference, generation must fail explicitly rather than inventing behavior.

This means Embassy generation is **profile-derived and evidence-bounded**:
the emitted Rust API may vary across documents of the same `driverKind`
depending on what was actually extracted, reviewed, and approved.

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

The table below defines the minimum expected evidence-backed HAIR surfaces
for first-cut generation of **real register-level code** rather than
metadata-only stubs.

| `driverKind` | Minimum required supporting data |
| --- | --- |
| `rcc` | `profiles.mcuSoc.clockResetTopology`; referenced `clockBindingRefs` / `resetBindingRefs`; and either binding `controlRefs` and field-level structure or referenced semantic operations for any emitted clock/reset helper |
| `gpio-port` | `profiles.mcuSoc.pinTopology.routes`; clock and/or reset bindings for emitted bring-up helpers in the first cut; and any referenced route `controlRefs` plus structural register/field data for emitted mode/remap/output helpers |
| `uart` / `usart` | `pinTopology.routes` always; clock/reset support for emitted bring-up helpers; explicit operations and/or control refs for any emitted enable/configure/read/write path; `interruptTopology.routes` and `dmaTopology.routes` only for emitted interrupt-driven or DMA-backed APIs |
| `spi` | `pinTopology.routes`; clock/reset support for emitted bring-up helpers; explicit operations and/or control refs for any emitted configuration or transfer path; interrupt/DMA routes only when the emitted API claims them |
| `i2c` | `pinTopology.routes`; clock/reset support for emitted bring-up helpers; explicit operations and/or control refs for any emitted bus transaction path; interrupt/DMA routes only when the emitted API claims them |
| `timer` / `pwm` | `pinTopology.routes` for exposed channels; `semantics.stateMachines` and `semantics.operations` for mode transitions; structural register/field data for emitted enable/disable/channel/duty behavior |
| `adc` | `semantics.operations` for calibration/init/enable; structural register/field data for any emitted conversion or sample path; pin/electrical data for exposed analog inputs; `dmaTopology.routes` only when the emitted API claims DMA-backed sampling |
| `dma` | `profiles.mcuSoc.dmaTopology.routes` and the referenced `dmaTopology.channels`; any referenced route `controlRefs`; and structural register/field data for emitted channel enable/launch/status helpers |
| `interrupt` | `structure.device.interrupts`, `profiles.mcuSoc.interruptTopology.routes`, and the referenced `interruptTopology.sources`; plus any clear/ack operations or control refs required by emitted helper methods |

## Generated API contract by driver kind

The first-cut generator should treat the following as the intended
**category** of generated APIs, not as a fixed universal method list or
exact naming contract:

| `driverKind` | Intended emitted API categories |
| --- | --- |
| `rcc` | Per-binding clock-enable / clock-disable / reset-assert / reset-release helpers when justified by the resolved topology and lowering inputs |
| `gpio-port` | Clock/reset bring-up helpers in the first cut, plus pin- and route-oriented configuration helpers only for modes/remaps that can be lowered from explicit route controls plus register/field structure |
| `uart` / `usart` | Bring-up helpers and only those polling / interrupt / DMA TX/RX methods whose control/data paths are explicitly modeled |
| `spi` | Bring-up helpers and only those transfer/control methods whose clocking, enable, and data paths are explicitly modeled |
| `i2c` | Bring-up helpers and only those bus transaction methods whose start/address/data/stop behavior is explicitly modeled |
| `timer` / `pwm` | Enable/disable/mode/channel helpers derived from state machines, operations, route controls, and structural register data |
| `adc` | Calibration/enable helpers plus only those conversion/sample methods whose trigger/start/complete/data path is explicitly modeled |
| `dma` | Channel-oriented enable/configure/launch/status helpers derived from DMA topology and any referenced controls |
| `interrupt` | IRQ enums plus bind/clear/ack helpers justified by the interrupt inventory, routes, and source-level operations |

The generator may choose exact Rust names and signatures, but those names
and signatures must be traceable to the approved HAIR lowering inputs.
The schema profile does not define a universal fixed Rust API surface for
all documents of a given `driverKind`.

## Authoring rules for `profiles.embassyHal`

1. `driverInstances[].targetRef` should resolve to a `profiles.mcuSoc.canonicalBlocks[]` entry in the first cut.
2. `driverInstances[].modulePath` is the generator-facing Rust module path for the emitted driver or support module.
3. `pinRoles[].signal` is required and must agree with the referenced `pinTopology.routes[].signal`.
4. A driver may omit `dmaRouteRefs` only when the generated first-cut implementation does not claim DMA-backed operation for that path.
5. Unsupported optional hardware must produce an explicit generator error, not a stub or silently degraded driver.
6. `initOperationRefs` and `stateMachineRefs` should be treated as executable lowering inputs, not as documentation-only labels.
7. A driver instance should not be interpreted as claiming every imaginable operation for its `driverKind`; it only claims the subset that the referenced topology and semantics can justify.

## Failure contract

For Embassy generation, the following cases must fail explicitly:

1. a driver instance names a supported `driverKind` but the chosen emitted API would require unresolved or missing lowering inputs
2. referenced topology or semantic records exist but do not carry enough structural detail to emit register-level code safely
3. a data-path API such as transfer, transaction, sample, or DMA-backed I/O would require behavior that is not explicitly modeled
4. the input document carries only resource metadata for a claimed behavior, with no executable lowering path

Generators may emit a smaller API surface than another document of the
same `driverKind`, but they must never silently widen that surface beyond
the approved HAIR contract.

## Workflow targeting contract

Extraction and audit workflows should not assume that Embassy-generation
data is always desired. They should ask the user which optional profiles
are in scope for the current run.

If `profiles.embassyHal` is requested, workflows should also ask which
supported driver kinds or concrete driver instances are intended. A
request for the Embassy profile alone is not enough to justify inventing
an all-peripheral Embassy-ready claim.

If the user requests `profiles.embassyHal`, the workflow should also
treat these supporting surfaces as required for the requested scope:

- `profiles.mcuSoc.canonicalBlocks`
- `profiles.mcuSoc.clockResetTopology` bindings needed by the selected drivers
- `profiles.mcuSoc.interruptTopology` sources and routes needed by the selected drivers
- `profiles.mcuSoc.dmaTopology` channels and routes needed by the selected drivers
- `profiles.mcuSoc.pinTopology.routes` needed by the selected drivers
- `semantics.operations` and `semantics.stateMachines` for driver kinds whose contract requires them

Workflows should not invent a blanket Embassy-ready claim for every
peripheral on the device. They should either:

1. extract/audit the specific supported driver set requested by the user, or
2. stop and ask the user to narrow the requested driver/profile scope.

## Relationship to the CLI contract

`docs/cli.md` defines the CLI surface.

For Embassy generation specifically:

- the command is `hair generate embassy <input> --output-dir <path>`
- generation is multi-file, so stdout is not the primary artifact surface
- input documents outside the supported subset must fail explicitly
