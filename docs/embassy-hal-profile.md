# HAIR Embassy HAL profile

This document defines the intended first-cut contract for `profiles.embassyHal`.

It is a **governing specification** for Embassy-style HAL generation. The current Rust CLI is expected to converge on the first-cut contract described here, and later profile revisions may still extend beyond the current generator; generators are expected to follow this contract rather than inventing their own binding rules. The same profile may lower either to an embedded-target Embassy HAL crate or to a separate host-emulated companion crate.

## Design intent

`profiles.embassyHal` is an optional generation-oriented layer.

It does **not** duplicate hardware facts. Instead, it references:

- core HAIR structure and semantics
- physical clocks, pins, and electrical facts
- canonical MCU topology from `profiles.mcuSoc`

This keeps the hardware model auditable and reusable while still giving an Embassy generator an explicit lowering contract.

The generator mode chooses the artifact shape, not the HAIR document shape:

- `hair generate embassy` emits the embedded-target crate
- `hair generate embassy-host` emits a separate host-only `std` crate derived
  from the same approved `profiles.embassyHal` contract

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

## Generated metadata contract

Embassy generation emits more than executable driver methods. It also
emits a Rust metadata surface that downstream code can inspect and reuse
without reparsing HAIR JSON.

That generated metadata must preserve the **generator-relevant structured
subset** of the approved lowering inputs rather than collapsing them to
names or IDs when the extra structure affects real lowering or downstream
consumption.

Normative consequences:

1. emitted metadata for `profiles.mcuSoc` bindings and routes must retain
   the generator-relevant fields from the approved HAIR records, including
   control refs and other lowering-significant selectors such as binding
   kind, controller/reset-domain references, remap-control references, and
   reset-default route selection when present
2. emitted metadata for `semantics.operations` and
   `semantics.stateMachines` must preserve structured execution data such
   as steps, expressions, targets, transitions, and effects rather than
   degrading those records to ID-only lists
3. generators may still omit unrelated optional fields that are not part
   of the supported first-cut lowering contract, but they must not discard
   structure that a non-stub lowering pass or downstream consumer would
   need to reproduce the approved behavior deterministically
4. when a generated driver module already performs a lowering justified by
   those approved inputs, the richer metadata should remain available to
   that module in structured form instead of being re-derived by ad hoc
   name matching

## Host-emulated generation contract

`hair generate embassy-host` is a host-only lowering of the same approved
Embassy driver contract, not a separate profile with looser evidence rules.

Normative consequences:

1. the host-emulated crate must preserve a 1:1 relationship between each
   generated HAL-visible device surface and a companion emulator/state handle
   for that same device
2. the host-emulated crate must remain a separate generated `std` crate rather
   than changing the embedded crate into a dual-target feature split
3. host-only emulator/test-control APIs must be exposed through explicit
   companion emulator/state handles rather than by changing the generated HAL
   driver types into a different public shape
4. the generated host crate names are derived automatically from
   `profiles.embassyHal.crate` rather than through additional profile fields
5. simulated time, interrupt delivery, DMA completion, and similar emulated
   side effects must progress under explicit test control in the first cut so
   host execution stays deterministic and auditable
6. host mode must not invent emulator behavior that the embedded lowering could
   not justify from the approved HAIR topology, semantics, and reachable
   register/field data
7. if a generated HAL-visible device would lack a paired emulator/state handle,
   host generation must fail explicitly rather than silently emitting a
   partially emulated crate

The companion emulator/state handles are where host-only observation and control
surfaces live. Depending on the justified lowering inputs, those handles may
expose:

- register and field state inspection
- explicit interrupt triggering and acknowledgement
- deterministic DMA progress and completion injection
- transmit/receive queues for USART/UART- and SPI-like paths
- controller/target state for I2C-style transactions
- explicit time advancement for generated async timing support

## Async timing contract

First-cut Embassy async timing support is an **optional generated
contract**, not an ambient promise that every Embassy-capable crate can
already satisfy `embassy_time::Timer` primitives.

When a document intends to support generated async timing behavior such
as `Timer::after()`, `Ticker`, and executor-driven sleep progression, it
must identify exactly one time-base provider in
`profiles.embassyHal.driverInstances[]` using capability tag
`embassy-time-driver`.

Normative consequences:

1. the tagged driver instance must be a supported lowering target whose
   approved HAIR inputs can justify a real tick source and wakeup path;
   in the current first cut that means an `interrupt` driver instance
   whose single referenced route targets the SysTick exception
2. the current first cut emits a SysTick-backed generated time driver;
   alternative timer-backed sources remain future work until they have a
   dedicated lowering contract and implementation
3. the tagged driver instance may use generator-facing support-module
   placement such as `modulePath = "time"`; this is still subject to the
   same evidence-bounded lowering rules as any other emitted module
4. the generator must fail explicitly if zero or more than one driver
   instance claims `embassy-time-driver` in a crate that requests async
   timing support
5. workflows and generators must not treat generic timer inventory alone
   as evidence of async timing readiness; the document must carry the
   explicit SysTick interrupt inventory and startup facts needed to
   initialize and drive the generated tick source without guesswork

The first cut currently requires one **common generated async timing
contract** backed by SysTick across supported MCUs.

In host-emulated mode, that same timing contract must remain deterministic under
explicit test-controlled progression rather than background wall-clock advance.

## Executable-readiness extraction contract

When a workflow is asked to extract or audit `profiles.embassyHal` for an
in-scope executable driver set, it must gather the **lowering-critical
facts** for the behaviors that set may legitimately claim. This rule is
generic across supported driver kinds; it is not a per-peripheral special
case list.

At minimum, the extracted and approved HAIR must preserve, where the
evidence supports them:

1. topology selectors and controls needed to reproduce the behavior
   deterministically, such as route or binding `controlRefs`,
   controller/channel/CPU-target selectors, and reset-default route
   selection
2. semantic operations and state machines for any claimed enable,
   configure, transfer, acknowledge, conversion, or mode-transition path
3. structural register/field facts reachable from the chosen lowering path,
   either directly on the target block or through an explicit, resolvable
   structural relationship such as `derivedFromRef`

Resource discovery alone is not enough. A driver instance is not
executable-ready when the document names clocks, pins, interrupts, DMA
channels, or semantic labels but omits the control, operation, or
structural reachability data needed to lower the claimed behavior without
guesswork.

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
| `gpio-port` | supported | Generates a canonical per-pin GPIO API from a `gpio-port` driver instance. The first cut covers input/output mode, pull configuration, output writes, and level/state reads when those behaviors are backed by explicit lowering inputs; alternate-function setup and EXTI remain out of scope. |
| `uart` / `usart` | supported | Requires explicit pin-routing data. Interrupt and DMA routes are required for async DMA-backed transfers and may be omitted for pure polling-mode instances. |
| `spi` | supported | Requires explicit pin-routing data and any claimed DMA bindings. |
| `i2c` | supported | Requires explicit pin-routing data and any claimed interrupt/DMA bindings. |
| `timer` / `pwm` | supported | Requires state-machine and route data for the supported operating modes being generated. Timer/PWM blocks do not yet serve as the generated Embassy async time-base provider in the first cut. |
| `adc` | supported | Requires calibration/init operations and any claimed DMA bindings. |
| `dma` | supported | Generates DMA infrastructure from `dmaTopology`. |
| `interrupt` | supported | Generates IRQ enums/bindings from the device interrupt inventory plus `interruptTopology`. |
| `custom` | unsupported in the first cut | Must fail explicitly rather than generating placeholders. |

Any other generation request is out of subset for the first cut and must fail explicitly.

For `hair generate embassy-host`, the same supported driver subset applies, and
every generated driver in that subset must also produce its paired
emulator/state handle.

## Required document surfaces by supported driver kind

The table below defines the minimum expected evidence-backed HAIR surfaces
for first-cut generation of **real register-level code** rather than
metadata-only stubs.

For every row in the table, the required structural register/field data
must remain deterministically reachable from the emitted lowering path.
That reachability may be direct or may flow through explicit structural
relationships such as `derivedFromRef`, but the generator must not depend
on silent inheritance assumptions, unresolved shared-base topology, or ad
hoc name matching.

| `driverKind` | Minimum required supporting data |
| --- | --- |
| `rcc` | `profiles.mcuSoc.clockResetTopology`; referenced `clockBindingRefs` / `resetBindingRefs`; binding `controlRefs` plus field-level structure for emitted clock/reset helpers; and referenced semantic operations for any additional emitted RCC operation helpers |
| `gpio-port` | `profiles.mcuSoc.pinTopology.routes`; clock and/or reset bindings for emitted bring-up helpers in the first cut; and any referenced route `controlRefs` plus structural register/field data for emitted per-pin input/output/pull helpers and output/input readback. The reachable lowering path may be a classic single-block GPIO layout or a composite path through explicit routing/control fabrics such as ESP32-C3 GPIO + IO MUX + GPIO Matrix. For STM32-class lowering paths, the reachable structure typically includes mode, output-latch, output-write, input-sample, and pull-configuration registers/fields (for example `MODER`, `ODR`, `BSRR`, `IDR`, and `PUPDR` when the emitted API claims them). |
| `uart` / `usart` | `pinTopology.routes` always; clock/reset support for emitted bring-up helpers; explicit operations and/or control refs for any emitted enable/configure/read/write path; `interruptTopology.routes` and `dmaTopology.routes` only for emitted interrupt-driven or DMA-backed APIs |
| `spi` | `pinTopology.routes`; clock/reset support for emitted bring-up helpers; explicit operations and/or control refs for any emitted configuration or transfer path; interrupt/DMA routes only when the emitted API claims them |
| `i2c` | `pinTopology.routes`; clock/reset support for emitted bring-up helpers; explicit operations and/or control refs for any emitted bus transaction path; interrupt/DMA routes only when the emitted API claims them |
| `timer` / `pwm` | `pinTopology.routes` for exposed channels; target-local `semantics.stateMachines` and `semantics.operations` for mode transitions; state-machine transitions with exactly one supported effect targeting a field for first-cut lowering; and structural register/field data for emitted enable/disable/channel/duty behavior |
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
| `gpio-port` | Clock/reset bring-up helpers plus per-pin `Input` / `Output` / `Flex`-style configuration and state helpers only for the behaviors that can be lowered from explicit route controls plus register/field structure. In the first cut that means input/output mode selection, pull configuration, output set/clear, output-state reads, and input-level reads; alternate-function setup and EXTI helpers remain out of scope. The same API category may be justified either by a conventional GPIO block or by a composite routing/control path when the approved HAIR records make the effective writes and reads explicit. |
| `uart` / `usart` | Bring-up helpers and only those polling / interrupt / DMA TX/RX methods whose control/data paths are explicitly modeled |
| `spi` | Bring-up helpers and only those transfer/control methods whose clocking, enable, and data paths are explicitly modeled |
| `i2c` | Bring-up helpers and only those bus transaction methods whose start/address/data/stop behavior is explicitly modeled |
| `timer` / `pwm` | Enable/disable/mode/channel helpers derived from state machines, operations, route controls, and structural register data |
| `adc` | Calibration/enable helpers plus only those conversion/sample methods whose trigger/start/complete/data path is explicitly modeled |
| `dma` | Channel-oriented enable/configure/launch/status helpers derived from DMA topology and any referenced controls |
| `interrupt` | IRQ enums plus bind/clear/ack helpers justified by the interrupt inventory, routes, and source-level operations; and, when tagged `embassy-time-driver`, the generated SysTick-backed async time-base support module |

The generator may choose exact Rust names and signatures, but those names
and signatures must be traceable to the approved HAIR lowering inputs.
The schema profile does not define a universal fixed Rust API surface for
all documents of a given `driverKind`.

The same traceability requirement applies to the generated metadata
surface: helper structs and constants in the emitted crate must preserve
the lowering-relevant structure needed to explain and reuse the approved
driver contract.

For host-emulated generation, the same traceability rule applies to the
companion emulator/state handles and their observation/control methods.

## Authoring rules for `profiles.embassyHal`

1. `driverInstances[].targetRef` should resolve to a `profiles.mcuSoc.canonicalBlocks[]` entry in the first cut.
2. `driverInstances[].modulePath` is the generator-facing Rust module path for the emitted driver or support module.
3. `pinRoles[].signal` is required and must agree with the referenced `pinTopology.routes[].signal`.
4. A driver may omit `dmaRouteRefs` only when the generated first-cut implementation does not claim DMA-backed operation for that path.
5. Unsupported optional hardware must produce an explicit generator error, not a stub or silently degraded driver.
6. `initOperationRefs` and `stateMachineRefs` should be treated as executable lowering inputs, not as documentation-only labels.
7. A driver instance should not be interpreted as claiming every imaginable operation for its `driverKind`; it only claims the subset that the referenced topology and semantics can justify.
8. A `gpio-port` driver instance may still lower to a per-pin generated API surface; the HAIR contract stays rooted at the port block while each emitted pin helper must remain traceable to explicit `pinRoles`, routes, and reachable structural controls for that pin.
9. Capability tag `embassy-time-driver` reserves that driver instance as the crate's generated async timing provider in the first cut. At most one driver instance may claim it, and in the current first cut it must be an `interrupt` driver instance whose single route targets SysTick.
10. Host-emulated output derives its package/crate naming from `crate.packageName`
    and `crate.crateName`; the profile does not grow separate host naming
    fields for the first cut.
11. Async or DMA-backed UART/I2C/SPI/ADC surfaces are supported only when the
    driver instance names the full interrupt, DMA, pin-routing, and semantic
    lowering inputs needed for that behavior; otherwise the generator must emit
    only the supported polling subset or fail explicitly if the requested
    surface depends on the missing inputs.

## Failure contract

For Embassy generation, the following cases must fail explicitly:

1. a driver instance names a supported `driverKind` but the chosen emitted API would require unresolved or missing lowering inputs
2. referenced topology or semantic records exist but do not carry enough structural detail to emit register-level code safely
3. a data-path API such as transfer, transaction, sample, or DMA-backed I/O would require behavior that is not explicitly modeled
4. the input document carries only resource metadata for a claimed behavior, with no executable lowering path
5. referenced semantic operations or state machines target a different peripheral than the driver instance, or a lowered state transition effect cannot be represented as one supported field write in the first cut
6. async timing support is requested or claimed, but no unique `embassy-time-driver` instance carries the SysTick interrupt inventory and startup closure needed to drive the generated tick source
7. host-emulated generation would expose a HAL-visible device without a paired
   emulator/state handle
8. a requested host-only observation or control surface would require
   unsupported inference beyond the approved lowering inputs
9. host-emulated execution would depend on background wall-clock progression
   rather than explicit deterministic test control in the first cut

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

For any requested executable driver scope, workflows should also:

1. gather the lowering-critical selectors on those supporting records
   instead of collapsing them to resource IDs alone
2. verify that the structural register/field data required by the claimed
   lowering path remains reachable directly or through explicit resolvable
   structural relationships
3. stop and classify the driver as not executable-ready when approved
   evidence supports resource discovery but not executable closure
   (for example missing route controls, missing semantic operations, or
   unresolved inherited register structure)

If a requested driver instance does not meet that executable-readiness
contract, workflows should either stop and ask the user how to narrow
scope, or omit that driver instance from `profiles.embassyHal` and record
the blocker explicitly in the discovery report. They must not emit the
driver instance as if it were executable-ready.

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
- the host-emulated companion command is
  `hair generate embassy-host <input> --output-dir <path>`
- the host-emulated command emits a separate host-only crate, not a feature
  variation of the embedded crate
