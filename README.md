# HAIR — Hardware Abstraction Intermediate Representation

HAIR is a vendor-neutral intermediate representation for describing MCU and SoC hardware with enough structure, provenance, and reviewability to support deterministic downstream generation.

This repository is currently focused on three things:

1. defining the layered HAIR schema and optional MCU/Embassy profile layers
2. supporting evidence-first extraction and audit workflows
3. implementing a small Rust CLI for validation, structural diffing, SVD generation, and Embassy-oriented code generation

## Current goals

The current repository baseline is intentionally conservative.

- **One device per top-level HAIR document.** Shared hardware can be imported from separate HAIR sub-documents, but each top-level document still describes one concrete device variant.
- **Evidence-first modeling.** Approved source material is captured in an explicit evidence manifest, and extracted facts stay tied to provenance inside the HAIR document.
- **Deterministic lowering.** Generators may only emit behavior justified by the approved HAIR inputs, and they should fail explicitly when required data is missing or out of scope.
- **Workflow-gated extraction.** Source discovery, extraction, and audit are repository workflows, not implicit side effects of generation.

## Current repository state

Today this repository contains:

- a layered JSON Schema set under `schema/`
- optional `profiles.mcuSoc` and `profiles.embassyHal` specialization layers
- a Rust crate (`edition = "2024"`) that implements the current CLI
- governed repository requirements, design, and validation baselines under `specs/`
- workflow skills under `.github/skills/` for source discovery, extraction, audit, and repository maintenance
- reference evidence bundles under `evidence/` for real device examples and generated artifacts

## Repository layout

| Path | Purpose |
| --- | --- |
| `schema/` | Core HAIR schema layers, profile schemas, and the evidence-manifest schema |
| `docs/` | Human-oriented schema, CLI, MCU-profile, and Embassy-profile guides |
| `specs/` | Governing repository requirements, design, and validation baselines |
| `src/main.rs` | Current Rust CLI implementation |
| `evidence/` | Reference device bundles, manifests, reports, and generated artifacts |
| `.github/skills/` | Repository workflows for discovery, extraction, audit, bootstrap, evolve, and maintain |

## HAIR document model

The top-level schema is `schema/hair.json`. A HAIR document can include:

- `metadata`
- `imports`
- `provenance`
- `structure`
- `semantics`
- `physical`
- `normalization`
- `validation`
- `profiles`

The repository also defines `schema/evidence-manifest.json`, which is the input format for evidence-first extraction workflows.

## Schema layers

| File | Role |
| --- | --- |
| `schema/hair.json` | Top-level HAIR document schema |
| `schema/common.json` | Shared primitive types and reusable helpers |
| `schema/provenance.json` | Sources, evidence, reviews, and provenance records |
| `schema/structure.json` | Devices, peripherals, interrupts, registers, fields, and memory regions |
| `schema/semantics.json` | Behaviors, operations, state machines, and semantic relationships |
| `schema/physical.json` | Clocks, timing, pins, packages, power/reset domains, and electrical constraints |
| `schema/normalization.json` | Canonical mappings, naming rules, and vendor quirks |
| `schema/validation.json` | Declarative validation rules and validation profiles |
| `schema/profiles/mcu.json` | Canonical MCU/SoC topology and block classification layer |
| `schema/profiles/embassy-hal.json` | Embassy-generation lowering contract |
| `schema/evidence-manifest.json` | Approved-source manifest for extraction workflows |

## Current CLI surface

The current Rust CLI implements these commands:

```text
hair validate <input>
hair generate svd <input> [--output <path>]
hair generate embassy <input> --output-dir <path>
hair generate embassy-host <input> --output-dir <path>
hair diff <left> <right>
```

### What each command does today

| Command | Current behavior |
| --- | --- |
| `validate` | Checks one HAIR JSON document against the repository schema set rooted at `schema/hair.json` |
| `generate svd` | Lowers a validated HAIR document to CMSIS-SVD-style output |
| `generate embassy` | Lowers a validated HAIR document to an embedded-target Embassy-style HAL crate |
| `generate embassy-host` | Lowers the same Embassy contract to a separate host-only `std` companion crate |
| `diff` | Compares two HAIR document revisions structurally, including git-backed inputs |

## Quick start

Validate a reference HAIR document:

```powershell
cargo run -- validate evidence\st\stm32f405rgt6\hair.json
```

Generate SVD output:

```powershell
cargo run -- generate svd evidence\wch\ch32v203c8t6\hair.json --output ch32v203c8t6.svd
```

Generate an embedded-target Embassy crate:

```powershell
cargo run -- generate embassy evidence\texas-instruments\lm3s6965\hair.json --output-dir out\embassy
```

Generate a host-emulated Embassy companion crate:

```powershell
cargo run -- generate embassy-host evidence\texas-instruments\lm3s6965\hair.json --output-dir out\embassy-host
```

Compare a working-tree HAIR document against its committed version:

```powershell
cargo run -- diff git:HEAD:evidence/st/stm32f405rgt6/hair.json evidence\st\stm32f405rgt6\hair.json
```

Run the Rust regression suite:

```powershell
cargo test
```

## Workflow model

`extract` and `normalize` are **not** current CLI commands. The repository treats source discovery, extraction, and audit as explicit workflows.

The main workflow skills are:

- `find-mcu-sources`
- `extract`
- `audit`
- `bootstrap`
- `evolve`
- `maintain`

Use the workflow docs and generated reports under `evidence/` when you need an evidence-backed extraction or audit trail rather than just schema validation or generation.

## Reference evidence bundles

The current repository includes reference bundles for:

| Vendor | Device |
| --- | --- |
| ST | `stm32f405rgt6` |
| WCH | `ch32v203c8t6` |
| Espressif | `esp32-c3fn4` |
| Texas Instruments | `lm3s6965` |

These bundles are used to exercise different parts of the schema, generator surface, and workflow/audit process. Some bundles also include checked-in generated artifacts and smoke-test projects.

### Current MCU bundle matrix

The table below summarizes the checked-in device bundles that currently include a top-level `hair.json`, along with the peripheral driver kinds modeled in each bundle's current `profiles.embassyHal` surface and the QEMU smoke coverage currently wired in this repository.

| Vendor | Device | HAIR JSON | Supported peripherals | QEMU-tested today |
| --- | --- | --- | --- | --- |
| ST | `stm32f405rgt6` | `evidence/st/stm32f405rgt6/hair.json` | `gpio-port`, `i2c`, `spi`, `uart`, `usart`, `interrupt` | CI runs the checked-in Embassy smoke with `qemu-system-arm -M netduinoplus2 -nographic -semihosting -kernel "$SMOKE_BINARY"`; the smoke source exercises GPIOA APIs, USART1, and Embassy time, but does not assert GPIO state transitions because QEMU readback is unreliable |
| WCH | `ch32v203c8t6` | `evidence/wch/ch32v203c8t6/hair.json` | `rcc`, `gpio-port`, `uart`, `usart`, `spi`, `i2c`, `timer`, `pwm`, `adc`, `dma`, `interrupt` | None documented |
| Espressif | `esp32-c3fn4` | `evidence/espressif/esp32-c3fn4/hair.json` | `rcc`, `gpio-port`, `interrupt`, `uart`, `i2c`, `spi`, `adc` | Manual `run-qemu-smoke.ps1` uses the pinned containerized `esp32c3` QEMU path and checks boot/UART/interrupt smoke plus `PASS`; the current smoke firmware does not exercise GPIO |
| Texas Instruments | `lm3s6965` | `evidence/texas-instruments/lm3s6965/hair.json` | `rcc`, `gpio-port`, `uart`, `spi`, `i2c`, `timer`, `interrupt` | CI runs the checked-in Embassy smoke with `qemu-system-arm -M lm3s6965evb -display none -monitor none -serial stdio -semihosting-config enable=on,target=native -kernel "$SMOKE_BINARY"`; the harness prints to UART0 stdio and exercises RCC, GPIO, SSI, I2C, timers, watchdog, flash, NVIC/SysTick, and Embassy time |

## Current boundaries

These are important limits of the current baseline:

- `validate` is currently **schema-conformance-first**; it does not execute declarative `validation.rules`
- Embassy generation requires both `profiles.mcuSoc` and `profiles.embassyHal`
- generator behavior is intended to be explicit and evidence-bounded, not placeholder-driven
- broader long-term goals such as richer validation execution, more import semantics, and additional generators remain future work

## Documentation

Start with these documents:

- `docs/schema.md` — layered schema overview
- `docs/mcu-profile.md` — MCU/SoC profile contract
- `docs/embassy-hal-profile.md` — Embassy HAL generation contract
- `docs/cli.md` — current CLI contract
- `specs/requirements.md` — repository requirements baseline
- `specs/design.md` — repository design baseline
- `specs/validation.md` — validation baseline

## Contributing

Contributions should preserve the current repository guarantees:

1. keep HAIR explicit and semantically precise
2. preserve provenance and evidence boundaries
3. prefer deterministic lowerings over ad hoc generation
4. fail explicitly on unsupported or ambiguous inputs
5. update the governing specs and docs when repository guarantees change

## License

HAIR is released under the [MIT License](LICENSE).
