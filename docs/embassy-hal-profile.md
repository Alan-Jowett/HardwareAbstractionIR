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

## Canonical-term-assisted resolution

When a document carries explicit `normalization.canonicalTerms[]` and
`normalization.mappings[]`, Embassy lowering may use those mappings as
**secondary** lookup hints for supported cross-vendor concepts such as
equivalent peripheral, register, or field roles.

Normative consequences:

1. canonical mappings may help the generator reduce internal lowering variance
   across vendors that express the same supported concept under different native
   names
2. canonical mappings do not replace `profiles.mcuSoc`,
   `profiles.embassyHal`, explicit semantic operations/state machines, pin
   routes, clock/reset bindings, interrupt routes, DMA routes, or reachable
   structural register/field data
3. canonical mappings do not silently rename the source-derived structural model
   or force a document into a more uniform public API shape than the approved
   lowering inputs justify
4. if a canonical-mapping-assisted lowering path encounters ambiguous mappings
   or conflicts with the explicit lowering path, the generator must still fail
   explicitly rather than guessing; missing canonical mappings alone are not an
   error when another supported explicit lowering path already resolves the
   concept safely
5. canonical mappings may let one supported family-aware lowering path bind
   vendor register aliases that preserve the same executable contract, such as
   CH32-style `STATR` / `DATAR` / `CTLR1-3` names on an otherwise STM32-like
   USART block, but only when the approved HAIR document still carries the
   explicit register-field closure needed for each emitted method

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
   approved HAIR inputs can justify a real tick source and wakeup path
2. `timeDriverSource = "systick"` remains the existing `interrupt`
   driver path whose single referenced route targets the SysTick
   exception
3. `timeDriverSource = "hardware-timer"` is allowed only for an approved
   `timer` driver instance whose referenced timer path justifies start,
   running state, counter reads, compare/alarm programming,
   interrupt-enable control, pending checks, and interrupt
   acknowledgement explicitly
4. `timeDriverSource = "rtc"` is allowed only for an approved `rtc`
   driver instance whose referenced rtc path justifies counter reads,
   alarm programming, interrupt-enable control, pending checks, and
   interrupt acknowledgement explicitly
5. a non-SysTick path must also declare the Embassy tick rate explicitly
   so generated async durations and the modeled hardware time base use
   the same unit
6. when one supported hardware-timer architecture still has materially
   different lowering families, the driver instance must also name an
   explicit `loweringPattern` for that family rather than relying on
   generator heuristics
7. for timer or rtc blocks with multiple interrupt causes or shared
   device vectors, the driver instance must still identify one explicit
   interrupt route/source pair plus one explicit clear operation for the
   generated time base; generic inventory is not enough
8. a first-cut counter/compare timer time base uses
   `loweringPattern = "counter-compare-timer"` and requires the approved
   path to justify prescaler/reload setup, free-running counter reads,
   compare/alarm programming, event/reload application, interrupt enable,
   and interrupt acknowledge/clear explicitly
9. a non-SysTick time base whose generated code depends on directly named
   counter/alarm/interrupt roles must carry explicit `timeDriverBindings`
   naming the exact counter, alarm/compare, interrupt-enable,
   interrupt-pending, and interrupt-clear handles the generator may
   lower; generators must not infer those roles from vendor register
   names alone. When the selected family requires an explicit
   event/reload/latch step after reprogramming the alarm, that same
   binding map must also name the semantic apply operation(s)
10. a higher-level ADC DMA sampling path whose generated code depends on
   directly named ADC and DMA roles must carry an explicit lowering family
   selector plus explicit `adcDmaBindings`; generators must not infer that
   path from vendor register names alone. The first such family is
   `regular-sequence-adc-dma`, which is limited to software-started
   regular-group buffered sampling and may expose one-shot and/or circular
   buffered helpers only when the approved HAIR inputs justify those exact
   behaviors. When this family also names `dmaRouteRefs`, generators may infer
   the backing DMA controller bring-up from the referenced DMA controller's own
   clock/reset bindings rather than duplicating those gates inside
   `adcDmaBindings`
11. a `dma` driver instance may expose Embassy-aligned IRQ-driven completion
   futures only when the same instance carries explicit `dmaAsyncBindings` plus
   the interrupt routes for the bound DMA channels. Those bindings name the
   exact transfer-complete and optional half-transfer interrupt/flag/clear
   handles the generator may use; they do not authorize unsupported channels or
   inferred interrupt behavior
12. the generated core contract for a non-SysTick path must stay
   runtime-agnostic: the generated crate may emit source-specific init
   helpers, blocking delay helpers when justified, the wake-handler
   entry point, the unique interrupt-route metadata, and
   interrupt-controller helper methods that operate on approved
   interrupt identities plus any explicit controller numbering
   semantics, but a downstream runtime layer remains
   responsible for binding the concrete trap symbol and deciding how to
   dispatch external interrupts on the board/runtime stack
12. the tagged driver instance may use generator-facing support-module
   placement such as `modulePath = "time"`; this is still subject to the
   same evidence-bounded lowering rules as any other emitted module
12. the generator must fail explicitly if zero or more than one driver
   instance claims `embassy-time-driver` in a crate that requests async
   timing support
13. workflows and generators must not treat generic timer or rtc
   inventory alone as evidence of async timing readiness; the document
   must carry the explicit interrupt inventory plus startup, clear/ack,
   and counter facts needed to initialize and drive the generated tick
   source without guesswork
14. when a checked-in or derived Embassy runtime harness for the same
   profile must choose between materially different thread-executor idle
   strategies, `profiles.embassyHal.crate.executorIdleStrategy` must
   declare that choice explicitly rather than relying on target-name
   heuristics. `wfi` selects the default wait-for-interrupt idle path;
   `spin` selects a non-sleeping idle loop for targets whose approved
   runtime/interrupt behavior cannot safely rely on WFI under the active
   critical-section strategy

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
4. for any hardware-timer lowering family whose generated code depends on
   directly named counter/alarm/interrupt roles, such as
   `counter-compare-timer`, an explicit
   `timeDriverBindings` map naming the exact counter, alarm/compare,
   interrupt-enable, interrupt-pending, and clear/ack handles used by
   the generated time-driver lowering, plus any required
   alarm-apply/reload semantic operations

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
| `flash` | supported | Requires one explicit managed flash region plus a controller-local erase/program contract. Portable lowering is limited to memory-mapped internal NOR arrays that can justify `embedded_storage::nor_flash::{ReadNorFlash, NorFlash}` from explicit geometry, unlock, busy/error, erase, and completion-flag bindings. Families with materially different sequencers must use an explicit `loweringPattern`; the first such family is `stm32f1-page-flash`. |
| `watchdog` | supported | Requires explicit control/status structure for feed/start/configure semantics. Because `embedded-hal` 1.0 does not ship watchdog traits, portable lowering may implement aliased `embedded-hal` 0.2 watchdog traits such as `embedded_hal_02::watchdog::Watchdog` and `WatchdogEnable`; `WatchdogDisable` is allowed only when the approved path justifies a real disable sequence. HAL-specific helpers may expose only modeled configuration/status controls such as prescaler/reload programming and update-state queries. |
| `rtc` | supported | Requires explicit RTC control/status structure plus interrupt and semantic evidence for any claimed raw counter/prescaler/alarm helpers. An rtc driver instance may also serve as the generated Embassy async time-base provider when `timeDriverSource = "rtc"` and the approved path includes the full counter/alarm/interrupt closure plus explicit `timeDriverBindings` when the lowering depends on direct role bindings. |
| `timer` / `pwm` | supported | Requires state-machine and route data for the supported operating modes being generated. A timer block may also serve as the generated Embassy async time-base provider when `timeDriverSource = "hardware-timer"` and the approved path includes the full counter/alarm/interrupt closure plus explicit `timeDriverBindings` for the generated counter/alarm/interrupt accesses. If the selected timer architecture still has materially different supported lowering families, the driver instance must also carry an explicit `loweringPattern` such as `counter-compare-timer`. |
| `adc` | supported | Requires calibration/init operations, any claimed DMA bindings, and explicit `adcDmaBindings` when the profile claims higher-level `regular-sequence-adc-dma` buffered sampling helpers. |
| `dma` | supported | Generates DMA infrastructure from `dmaTopology`. |
| `interrupt` | supported | Generates IRQ enums/bindings from the device interrupt inventory plus `interruptTopology`. |
| `usb-device` | supported | Requires explicit D+/D- pin-routing data, clock/reset support, interrupt routes, and semantic/state-machine plus structural register/field closure for any claimed controller-level USB behavior. Some USB lowering families may additionally require an explicit `loweringPattern` selector when bring-up behavior is materially family-specific. Standard USB functions such as CDC ACM should be layered from Embassy USB libraries on top of the generated controller module rather than encoded as separate HAIR driver kinds. |
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
| `flash` | one explicit managed flash storage region; portable erase/program geometry in `flashBindings`; explicit `semantics.operations` for unlock/lock and any required completion/error-flag clearing; structural register/field data for the reachable busy, completion, error, program-enable, page-erase-enable, erase-address, and erase-start path; and `driverInstances[].loweringPattern` whenever the supported flash sequencer family is materially distinct. For `loweringPattern = "stm32f1-page-flash"`, the first cut is limited to a memory-mapped page-erase/program path over that named storage region rather than inferred option-byte, mass-erase, or vendor-specific fast-program helpers |
| `watchdog` | clock/reset support for emitted bring-up helpers when claimed; explicit `semantics.operations` and/or `semantics.stateMachines` for any composite unlock, start, feed, or status-poll path that cannot be lowered as one direct structural write or read; and structural register/field data for the reachable watchdog feed, start, configuration, and update-status path |
| `rtc` | clock/reset support for emitted bring-up helpers when claimed; explicit `interruptTopology.routes` for any emitted interrupt-driven or wakeup behavior; explicit `semantics.operations` and/or `semantics.stateMachines` for claimed raw counter/prescaler/alarm setup or flag/interrupt handling; structural register/field data for the reachable RTC counter, prescaler, alarm, enable, pending, and clear/ack path; and, when the same driver instance also claims `embassy-time-driver`, the explicit tick-rate and binding facts needed for the selected rtc-backed time-base path |
| `timer` / `pwm` | `pinTopology.routes` for exposed channels; target-local `semantics.stateMachines` and `semantics.operations` for mode transitions; state-machine transitions with exactly one supported effect targeting a field for first-cut lowering; structural register/field data for emitted enable/disable/channel/duty behavior; and, if the driver instance also claims blocking delay helpers or `embassy-time-driver`, the explicit interrupt routes plus semantic/structural counter, alarm, clear/ack, reload, and tick-rate facts needed for that timing path. Shared-vector timers must still narrow the generated time base to one explicit route/source/clear path. For `loweringPattern = "counter-compare-timer"`, that reachable structure includes the timer-enable path plus prescaler, reload, counter, alarm/compare, event/update, interrupt-enable, and interrupt-status/clear registers or canonical equivalents. |
| `adc` | `semantics.operations` for calibration/init/enable, including any explicit status-poll steps needed before conversion can begin; structural register/field data for any emitted conversion or sample path; pin/electrical data for exposed analog inputs; `dmaTopology.routes` only when the emitted API claims DMA-backed sampling; and explicit `adcDmaBindings` when `loweringPattern = "regular-sequence-adc-dma"` claims higher-level regular-group buffered DMA helpers. The generated family may also reuse the referenced DMA controller driver's clock/reset bindings to bring that DMA path up before channel programming |
| `dma` | `profiles.mcuSoc.dmaTopology.routes` and the referenced `dmaTopology.channels`; any referenced route `controlRefs`; structural register/field data for emitted channel enable/launch/status helpers; and explicit `dmaAsyncBindings` plus matching interrupt routes when the generated API claims IRQ-driven completion futures |
| `interrupt` | `structure.device.interrupts`, `profiles.mcuSoc.interruptTopology.routes`, and the referenced `interruptTopology.sources`; plus any clear/ack operations or control refs required by emitted helper methods |
| `usb-device` | `pinTopology.routes` for D+ and D-; clock/reset support for emitted bring-up helpers; `interruptTopology.routes` for any emitted interrupt-driven behavior; explicit `semantics.operations` and/or `semantics.stateMachines` for claimed bus-reset, attach, endpoint/FIFO, or other controller-level helpers; structural register/field data for the reachable USB control/data path being lowered; and `driverInstances[].loweringPattern` whenever the chosen USB lowering family has materially distinct bring-up behavior that the generator must not infer |

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
| `flash` | `embedded_storage::nor_flash::{ReadNorFlash, NorFlash}` on the named managed flash region only when the approved controller path explicitly models unlock, page erase, programming, busy/completion/error handling, and the required erase/write geometry. HAL-specific helpers may expose only the minimum modeled unlock/lock/status path needed to realize that portable surface. Option-byte, mass-erase, and vendor-specific fast-program helpers remain out of subset unless a later approved contract names them explicitly |
| `watchdog` | Bring-up helpers plus portable watchdog feed/start support only when the approved control/status path is explicitly modeled. Because `embedded-hal` 1.0 has no watchdog module, that portable surface is the aliased `embedded_hal_02::watchdog::{Watchdog, WatchdogEnable}` contract unless the governing spec says otherwise. HAL-specific raw configuration/status helpers such as prescaler/reload programming and update-state queries may be emitted from that same approved path. `WatchdogDisable` is allowed only when the approved HAIR inputs justify a real disable sequence; otherwise the generator must omit it rather than invent one. |
| `rtc` | Bring-up helpers plus HAL-specific raw counter/prescaler/alarm/flag methods whose control and status paths are explicitly modeled; and, when selected as `timeDriverSource = "rtc"`, generated Embassy async time-base support only when the approved RTC counter/alarm/interrupt path is explicitly modeled and the emitted raw RTC helpers remain traceable to that same approved path |
| `timer` / `pwm` | Enable/disable/mode/channel helpers derived from state machines, operations, route controls, and structural register data; plus blocking delay/timebase helpers and, when selected as `timeDriverSource = "hardware-timer"`, generated Embassy async time-base support only when the approved timer counter/alarm path is explicitly modeled. That async support must preserve a runtime-agnostic wake-handler hook, the unique interrupt-route metadata needed by a downstream runtime layer, and the explicit Embassy tick rate used by the timer source. When `loweringPattern = "counter-compare-timer"`, the generated time-base path is specifically the approved counter/compare/event/interrupt closure rather than an inferred generic timer API. |
| `adc` | Calibration/enable helpers plus only those conversion/sample methods whose trigger/start/complete/data path is explicitly modeled. When `loweringPattern = "regular-sequence-adc-dma"`, the generated API may additionally expose one-shot and circular regular-group buffered DMA sampling helpers only for the exact path named by `adcDmaBindings`, with DMA controller bring-up inferred from the same driver's referenced `dmaRouteRefs`. When the matching DMA driver also carries `dmaAsyncBindings`, the ADC surface may compose on top of that DMA future for awaited one-shot capture |
| `dma` | Channel-oriented enable/configure/launch/status helpers derived from DMA topology and any referenced controls, plus IRQ-driven completion futures only for the explicitly bound channels in `dmaAsyncBindings` |
| `interrupt` | IRQ enums plus bind/clear/ack helpers justified by the interrupt inventory, routes, and source-level operations; and, when tagged `embassy-time-driver` with `timeDriverSource = "systick"`, the generated SysTick-backed async time-base support module |
| `usb-device` | Bring-up helpers plus only those controller-level USB methods whose attach, reset, interrupt, and data paths are explicitly modeled; device-specific helpers are allowed when they remain traceable to the approved USB lowering path rather than to inferred generic USB behavior. Standard USB classes such as CDC ACM are expected to come from Embassy USB libraries layered on top of the generated controller module. When `loweringPattern = "serial-jtag-preserve-link"`, the emitted bring-up sequence must preserve the boot-established USB Serial/JTAG link instead of synthesizing an unconditional reset-and-reattach sequence |

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
7. First-cut generated bring-up helpers may lower explicit semantic `poll` steps as blocking MMIO status waits when the approved operation names the exact register field and expected ready value; generators must not invent hidden waits that are absent from the approved operation.
8. A driver instance should not be interpreted as claiming every imaginable operation for its `driverKind`; it only claims the subset that the referenced topology and semantics can justify.
9. A `gpio-port` driver instance may still lower to a per-pin generated API surface; the HAIR contract stays rooted at the port block while each emitted pin helper must remain traceable to explicit `pinRoles`, routes, and reachable structural controls for that pin.
10. Capability tag `embassy-time-driver` reserves that driver instance as the crate's generated async timing provider in the first cut. At most one driver instance may claim it, and it must also declare `timeDriverSource`.
11. Host-emulated output derives its package/crate naming from `crate.packageName`
    and `crate.crateName`; the profile does not grow separate host naming
    fields for the first cut.
12. Async or DMA-backed UART/I2C/SPI/ADC surfaces are supported only when the
    driver instance names the full interrupt, DMA, pin-routing, and semantic
    lowering inputs needed for that behavior; otherwise the generator must emit
    only the supported polling subset or fail explicitly if the requested
    surface depends on the missing inputs.
12. A `usb-device` driver instance may claim only the controller-level helper
    subset justified by its approved USB control/data path. Generic USB
    endpoint inventory alone does not justify CDC- or UART-like byte-stream
    helpers; standard USB functions such as CDC ACM should come from layered
    Embassy USB libraries. A device-specific byte-stream path still does not
    justify unrelated generic USB APIs.
13. If a `usb-device` driver instance names a `loweringPattern`, that selector
    becomes part of the executable lowering contract. In particular,
    `serial-jtag-preserve-link` requires the referenced semantic operations to
    justify a bring-up path that preserves an already boot-established USB
    Serial/JTAG link; the generator must not silently substitute a reset or
    attach pattern that is not explicitly modeled. `fsdev-pma-btable` selects
    a controller-level PMA/BTABLE-style full-speed device family and does not
    by itself authorize CDC- or other class-specific helpers.
14. If an `adc` driver instance names `loweringPattern = "regular-sequence-adc-dma"`,
    that selector becomes part of the executable lowering contract. The same
    driver instance must also carry explicit `adcDmaBindings` naming the
    regular-sequence length and slot fields, per-channel sample-time fields,
    data register, either one direct software-start control or an explicit
    semantic start sequence, DMA transfer-count and address programming
    handles, DMA channel-enable handle, DMA half-transfer and
    transfer-complete status/interrupt handles, and the semantic setup/clear
    operations the generator may use for one-shot and circular buffered
    sampling. The family still relies on `dmaRouteRefs` for the backing DMA
    controller path, and generators may reuse the matching `dma` driver
    instance's clock/reset bindings to ensure that controller is live before DMA
    register access. This family does not authorize injected-group or dual-ADC
    DMA helpers.
15. If a `dma` driver instance carries `dmaAsyncBindings`, the same driver
    instance must also carry the interrupt routes for those channels. The
    binding map names the transfer-complete interrupt-enable, status, and clear
    handles, and may additionally name half-transfer handles. Generated DMA
    futures are limited to the explicitly bound channels.
16. If a `flash` driver instance names `loweringPattern = "stm32f1-page-flash"`,
    the same driver instance must also carry explicit `flashBindings` naming
    the managed storage region, portable erase/write geometry, busy/completion
    and optional error status handles, the program/page-erase/address/start
    controls, and the semantic unlock/lock/flag-clear operations required by
    that sequencer. The resulting portable surface is limited to
    memory-mapped `embedded_storage::nor_flash::{ReadNorFlash, NorFlash}` over
    that named region; option-byte, mass-erase, and unrelated fast-program
    helpers remain out of subset unless a later approved contract names them
    explicitly.
17. If a driver instance claims `embassy-time-driver`, `timeDriverSource`
    becomes part of the executable lowering contract. `systick` preserves the
    existing SysTick-backed interrupt path. `hardware-timer` selects an
    explicitly modeled timer-backed time base. `rtc` selects an explicitly
    modeled rtc-backed time base. None of those choices may be inferred from
    `driverKind` or capability tags alone.
18. A `hardware-timer` time-driver claim must remain rooted in a `timer`
    driver instance whose approved HAIR inputs justify timer start, running
    state, compare/alarm or wrap behavior, interrupt acknowledgement, and the
    counter facts needed for any emitted blocking delay helpers. An `rtc`
    time-driver claim must remain rooted in an `rtc` driver instance whose
    approved HAIR inputs justify rtc counter reads, alarm behavior, interrupt
    acknowledgement, and any emitted raw RTC helpers. Both non-SysTick paths
    must declare `timeDriverTickHz`.
19. If a hardware-timer time-driver claim uses a timer family whose supported
    lowering shape is materially distinct from other supported timer families,
    that driver instance must also declare `loweringPattern`. The first such
    timer-family selector is `counter-compare-timer`.
20. A non-SysTick time-driver claim must narrow the generated time base to one
    explicit interrupt route/source pair and one explicit clear operation, even
    when the underlying peripheral exposes multiple causes or shares one device
    vector across several events.
21. A `systick` time-driver claim must remain rooted in an `interrupt` driver
    instance whose approved route inventory explicitly targets SysTick.
22. A `systick` time-driver claim must not declare `timeDriverTickHz`; that
    rate is defined by the SysTick lowering path itself.
23. A generated non-SysTick time-driver path must preserve the approved
    interrupt-route identity in generated metadata and expose a
    runtime-agnostic wake-handler hook that a board/runtime layer can call from
    the concrete interrupt binding. The generated crate metadata must also
    preserve any explicit interrupt-controller numbering semantics needed to
    place that vector or program controller register indices correctly, and
    select the matching `embassy-time-driver` tick-rate feature from
    `timeDriverTickHz`.

## Building a bootable ESP32-C3 image around a generated Embassy crate

`hair generate embassy` emits a generated HAL crate, not a complete flash-ready
application image. For Espressif parts such as the ESP32-C3, a standalone image
that boots through the normal ROM + second-stage flash boot flow must still
conform to Espressif's app-image contract.

The checked-in reference for that packaging is:

- `evidence\espressif\esp32-c3fn4\generated\embassy-usb-smoke\Cargo.toml`
- `evidence\espressif\esp32-c3fn4\generated\embassy-usb-smoke\.cargo\config.toml`
- `evidence\espressif\esp32-c3fn4\generated\embassy-usb-smoke\src\main.rs`

For the current ESP32-C3 first cut, that means the application crate should:

1. depend on the generated Embassy crate plus `esp-hal`
2. add the normal Embassy executor/time crates when the application wants a
   canonical async task model on top of the generated time-driver contract
3. use `esp-bootloader-esp-idf` so the resulting ELF carries the boot metadata
   and layout expected by Espressif's flash boot chain
4. emit an app descriptor with `esp_bootloader_esp_idf::esp_app_desc!();`
5. link with `-Tlinkall.x`
6. package the release ELF into an ESP32-C3 flash image before writing it to
   the factory app slot

Minimal project shape:

```toml
[dependencies]
embassy-executor = { version = "0.7.0", features = ["arch-riscv32", "executor-thread"] }
embassy-time = "0.5.1"
esp-bootloader-esp-idf = { version = "0.5.0", features = ["esp32c3"] }
esp-hal = { version = "1.1.1", features = ["esp32c3", "unstable"] }
esp32c3fn4_generated = { package = "esp32c3fn4-generated", path = "../embassy" }
panic-halt = "1.0"
static_cell = "2.1.0"
```

```toml
[build]
target = "riscv32imc-unknown-none-elf"

[target.riscv32imc-unknown-none-elf]
rustflags = ["-C", "link-arg=-Tlinkall.x"]
```

```rust
#![no_std]
#![no_main]

use esp_hal as _;
use panic_halt as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    loop {}
}
```

Build, package, and flash flow:

```powershell
Set-Location evidence\espressif\esp32-c3fn4\generated\embassy-usb-smoke
cargo build --release
python -m esptool --chip esp32c3 elf2image --flash-mode dio --flash-freq 40m --flash-size 4MB `
  -o target\riscv32imc-unknown-none-elf\release\embassy-usb-smoke.bin `
  target\riscv32imc-unknown-none-elf\release\embassy-usb-smoke
python -m esptool --chip esp32c3 --port COM6 write-flash 0x10000 `
  target\riscv32imc-unknown-none-elf\release\embassy-usb-smoke.bin
espflash monitor -p COM6
```

Notes:

- The generated Embassy HAL crate itself is reusable; the boot-image packaging
  lives in the board/application crate wrapped around it.
- For a generated non-SysTick Embassy time base, that board/application crate
  also owns the concrete interrupt binding needed to connect the generated
  wake-handler hook to the runtime/startup stack.
- For the current tool versions in this repository, `esptool elf2image` is the
  reliable path for packaging the final flash image. Direct `espflash flash
  <elf>` packaging may not work for this crate shape.
- This boot-packaging guidance is ESP32-C3-specific. Other families may require
  different vendor boot metadata, different linker arguments, or no extra
  packaging step at all.

## Failure contract

For Embassy generation, the following cases must fail explicitly:

1. a driver instance names a supported `driverKind` but the chosen emitted API would require unresolved or missing lowering inputs
2. referenced topology or semantic records exist but do not carry enough structural detail to emit register-level code safely
3. a data-path API such as transfer, transaction, sample, or DMA-backed I/O would require behavior that is not explicitly modeled
4. the input document carries only resource metadata for a claimed behavior, with no executable lowering path
5. referenced semantic operations or state machines target a different peripheral than the driver instance, or a lowered state transition effect cannot be represented as one supported field write in the first cut
6. async timing support is requested or claimed, but no unique `embassy-time-driver` instance carries the explicit source selection plus interrupt/semantic/startup closure needed to drive the generated tick source
7. a claimed `embassy-time-driver` instance omits `timeDriverSource`, uses a source that contradicts the driver kind, omits the required `timeDriverTickHz` for a non-SysTick path, sets `timeDriverTickHz` on a SysTick path, or lacks the interrupt/semantic/structural closure required for that source
8. a timer-backed or rtc-backed delay/time-base/API path would require inferred counter, alarm, wrap, pending, or acknowledgement behavior beyond the approved HAIR records
9. a `usb-device` API would require inferred USB protocol state, endpoint
   semantics, or serial-style behavior that is not explicitly modeled by the
   approved clock/reset, interrupt, pin, semantic, and structural inputs
10. a `watchdog` API would require inferred disable semantics or unmodeled
   unlock, feed, configuration, or update-status behavior beyond the approved
   control/status path
11. a `flash` API would require inferred erase/program geometry, implicit
   unlock/lock or completion/error-flag clearing, or controller-local
   option-byte / mass-erase / fast-program behavior beyond the explicit
   `flashBindings`, referenced operations, and reachable structural data
12. a selected `loweringPattern` requires semantic or structural bring-up
   inputs that are absent, contradictory, or would force the generator to
   invent a different attach/reset sequence
13. an `adc` buffered-sampling API would require inferred regular-sequence,
   sample-time, start sequencing, DMA programming, or DMA event-clear behavior
   beyond the explicit `adcDmaBindings`, referenced operations, and reachable
   structural data
14. a non-SysTick time-driver claim relies on an implicit choice among multiple
   interrupt causes, shared vectors, or clear paths instead of naming one
   approved route/source/clear sequence explicitly
15. host-emulated generation would expose a HAL-visible device without a paired
   emulator/state handle
16. a requested host-only observation or control surface would require
   unsupported inference beyond the approved lowering inputs
17. host-emulated execution would depend on background wall-clock progression
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
