# HAIR Audit Report

## Device Summary

- **Target device**: CH32V203G6U6 (`CH32V203`, variant `G6U6`, package `QFN28`)
- **HAIR artifact**: `evidence\wch\ch32v203g6u6\hair.json`
- **Manifest artifact**: `evidence\wch\ch32v203g6u6\evidence-manifest.json`
- **Profiles present**: `profiles.mcuSoc`, `profiles.embassyHal`
- **Requested emphasis**: full-device audit with focused adversarial review of interrupt/timer behavior and the claimed Embassy async time-driver path
- **Claim under strongest challenge**: `profiles.embassyHal.driverInstances[].id = "drv.tim4-time"` (`hardware-timer`, `counter-compare-timer`, `embassy-time-driver`) (`evidence\wch\ch32v203g6u6\hair.json:73812-73854`)

## Input Inventory

| Artifact | Path | Role | Target identity | Notes |
| --- | --- | --- | --- | --- |
| HAIR document | `evidence\wch\ch32v203g6u6\hair.json` | Audit target | `device.ch32v203g6u6` / `CH32V203G6U6` | `hair validate` passed during this audit. |
| Evidence manifest | `evidence\wch\ch32v203g6u6\evidence-manifest.json` | Approved source inventory | `CH32V203G6U6` | JSON-schema validation passed during this audit. |
| Repository contract | `README.md`, `docs\schema.md`, `docs\mcu-profile.md`, `docs\embassy-hal-profile.md`, `schema\*.json` | Governing audit baseline | Repository-wide | Used to challenge claimed scope and lowering requirements. |
| Generated Embassy output | `evidence\wch\ch32v203g6u6\generated\embassy\src\time.rs` | Translation-gap check | Derived from same HAIR | `hair generate embassy` succeeded during this audit. |
| Official startup source | `openwch-ch32v20x-startup-d6` from manifest | Interrupt vector evidence | CH32V20x D6 subgroup including CH32V203G6 | Confirms one `TIM4_IRQHandler` vector entry, not separate TIM4 UP/TRG/CC vectors. |
| Official header | `openwch-ch32v20x-header` from manifest | IRQ numbers, register/bit identity | CH32V20x D6 subgroup including CH32V203G6 | Confirms `TIM4_IRQn = 46`, timer register/bit inventory, and `__Vendor_SysTickConfig = 0`. |
| Official timer example | `openwch-ch32v20x-tim-dma-example` from manifest | Concrete timer behavior sample | CH32V20x family | Demonstrates TIM1 DMA/PWM, not TIM4 compare-interrupt time-base behavior. |

## Approved Evidence Summary

- The manifest is for one concrete device variant, `CH32V203G6U6`, and includes the datasheet, reference manual, official SDK/header/startup/example sources, QingKe core manual, and community `ch32-rs` SVD/YAML inputs (`evidence\wch\ch32v203g6u6\evidence-manifest.json:1-83`).
- The HAIR provenance records the same official and community source families, including:
  - timer register model evidence `e_rm_tim234_register_model` (`evidence\wch\ch32v203g6u6\hair.json:343-365`)
  - community interrupt topology evidence `e_ch32rs_ch32v203g6u6_yaml_interrupt_topology` (`evidence\wch\ch32v203g6u6\hair.json:475-485`)
  - timer counter-enable bit evidence `e_header_tim_counter_enable` (`evidence\wch\ch32v203g6u6\hair.json:508-520`)

## Official WCH C Source Cross-Check

The whole-document pass against official WCH C sources found that the **majority of the C-code-backed HAIR material agrees** with the SDK:

- **Device/core identification** agrees with the D6 grouping, QingKe V4B core support, PFIC layout, and vendor SysTick block (`ch32v20x.h`, `core_riscv.h`, `debug.c`; see `evidence\wch\ch32v203g6u6\hair.json:190-205`, `279-303`, `613-699`).
- **Interrupt numbering and vector order** agree with the official header and startup file for the D6 subgroup, including TIM2/TIM3/TIM4 global vectors (`evidence\wch\ch32v203g6u6\hair.json:702-980`).
- **Peripheral base addresses and register topology** are consistent with the official header-derived macro and typedef model for the C-code-backed MMIO blocks reviewed during this pass, including TIM, RCC, ADC, GPIO, USART, I2C, SPI, CAN, and USB family entries (`evidence\wch\ch32v203g6u6\hair.json:7828-8060`, `55280-56020`).
- **Clock/reset bindings** align with the official RCC peripheral-enable and reset macros for APB1/APB2/AHB-gated peripherals (`evidence\wch\ch32v203g6u6\hair.json:68282-68710`; `ch32v20x_rcc.h`).
- **Timer and ADC semantics** mostly align with official driver code: `op.tim4.enable` matches `TIM_Cmd`, the timer interrupt source bits match `TIM_IT_*`, and `op.adc1.calibrate` / `op.adc2.calibrate` match the official ADON/RSTCAL/CAL sequencing exposed by `ADC_Cmd`, `ADC_ResetCalibration`, and `ADC_StartCalibration`.

The remaining problems are therefore narrower than a whole-document structural failure: they are concentrated around **how strongly the HAIR document claims async time-driver readiness from the timer evidence**, not around the basic header/SDK-derived device structure.

## Evidence Validation Findings

1. **The TIM4 Embassy async time-driver claim is not fully supported by the approved evidence set.**  
   `drv.tim4-time` is explicitly marked `status: "inferred"` and depends on a repository-authored TIM4 time-base configuration plus a community interrupt-topology split, not on a vendor source that demonstrates a TIM4 compare-driven Embassy-style wake path (`evidence\wch\ch32v203g6u6\hair.json:73812-73854`). The official timer example in the approved manifest is a TIM1 DMA/PWM example, so it does not defend the claimed TIM4 async route (`evidence\wch\ch32v203g6u6\evidence-manifest.json:55-60`).

2. **The TIM4 shared-vector sub-cause split is only weakly supported by official material.**  
   HAIR models `isrc.tim4.up`, `isrc.tim4.trg`, and `isrc.tim4.cc`, all routed to the single external interrupt `int.tim4` (`evidence\wch\ch32v203g6u6\hair.json:69050-69100`, `69695-69754`). The official startup file supports a single `TIM4_IRQHandler` vector for the D6 subgroup, while the finer UP/TRG/CC partition comes from the approved community YAML evidence record `e_ch32rs_ch32v203g6u6_yaml_interrupt_topology` (`evidence\wch\ch32v203g6u6\hair.json:476-485`), not from a vendor TIM4-specific interrupt example.

3. **`op.tim4.clear_cc1` is now better supported at the timer-family level than this HAIR record states, but the HAIR operation wording still does not capture the exact official helper behavior.**  
   The official WCH timer driver clears timer pending bits with `TIMx->INTFR = (uint16_t)~TIM_IT`, which directly supports WCH-family timer interrupt acknowledgement semantics for `UIF`, `CC1IF`, and related causes. That narrows the earlier uncertainty substantially, but the HAIR operation is still marked `status: "inferred"` and its plain-language `"Clear CC1IF"` step does not yet encode the exact write-mask behavior used by the vendor helper (`evidence\wch\ch32v203g6u6\hair.json:63942-63971`).

4. **`op.tim4.configure_counter_compare_timebase` is repository-authored and depends on an unproven clock-rate assumption.**  
   The operation writes `PSC = 7999` and describes that as dividing a “reset-default 8 MHz timer clock” down to 1 kHz, but the operation provenance itself admits this is inferred and tied to the current smoke-flow assumption (`evidence\wch\ch32v203g6u6\hair.json:63869-63939`). That is not the same as approved-source proof that the device-level Embassy time-driver contract is ready.

## Structural Completeness Findings

1. **The core TIM4/PFIC interrupt inventory survives the basic structural challenge.**  
   The HAIR document has a single `int.tim4` external interrupt at vector number 46 and ties it to `ic.pfic` (`evidence\wch\ch32v203g6u6\hair.json:914-919`, `62565-62585`). This matches the official header/startup identity used in the manifest.

2. **The device-level CPU/interrupt metadata required for downstream consumers is present.**  
   The device CPU model includes revision, endianness, `interruptPriorityBits = 4`, and `vendorSystemTimerConfig` (`evidence\wch\ch32v203g6u6\hair.json:619-628`).

3. **The timer-backed Embassy readiness claim fails the stronger structure-for-execution test.**  
   The profile contract says a `hardware-timer` Embassy time driver must justify start, running state, counter reads, compare/alarm programming, interrupt enable, and interrupt acknowledge/clear explicitly (`docs\embassy-hal-profile.md:166-185`). `drv.tim4-time` references only:
   - one interrupt route: `iroute.tim4.cc`
   - two init operations: `op.tim4.configure_counter_compare_timebase`, `op.tim4.enable`
   - one state machine: `sm.tim4`  
   (`evidence\wch\ch32v203g6u6\hair.json:73812-73854`)

4. **The generated lowering uses behavior that is thinner in HAIR than the contract expects.**  
   The generated `time.rs` writes the compare register, toggles the compare-interrupt enable bit, checks pending status, and performs interrupt acknowledgement directly from structural addresses/masks (`evidence\wch\ch32v203g6u6\generated\embassy\src\time.rs:236-317`). Those behaviors are real emitted code, but they are not all represented as explicit timer semantic operations on the HAIR side.

## Metadata Coverage and Gaps

### Metadata present in approved evidence and extracted

- TIM4 interrupt identity, description, vector number, and controller binding (`evidence\wch\ch32v203g6u6\hair.json:914-919`)
- PFIC controller identity (`evidence\wch\ch32v203g6u6\hair.json:62565-62585`)
- TIM4 clock/reset bindings (`evidence\wch\ch32v203g6u6\hair.json:68282-68304`, `68675-68697`)
- TIM4 register and field inventory for `CTLR1.CEN`, `DMAINTENR.UIE`, `INTFR.UIF`, and `INTFR.CC1IF` (`evidence\wch\ch32v203g6u6\hair.json:7853-7868`, `8583-8604`, `8978-9029`)

### Metadata present in approved evidence but not extracted as explicit lowering inputs

- None identified conclusively outside the timer/interrupt lowering contract; the main problem is not missing raw register identity, but missing explicit **behavioral** lowering records for compare programming and interrupt-enable/ack sequencing.

### Metadata not confidently recoverable from the approved evidence used here

- A vendor-backed TIM4-specific async compare/alarm bring-up flow suitable for `embassy-time-driver`
- A vendor-backed proof that the `PSC = 7999` / 1 kHz time base is the correct board-independent default for the claimed device-level Embassy profile

## Unsupported or Weakly Supported Claims

1. **Unsupported claim: `drv.tim4-time` is downstream-ready as a hardware-timer Embassy async time base.**  
   The profile claim exists (`evidence\wch\ch32v203g6u6\hair.json:73812-73854`), generation succeeds, and the generated crate emits a working-looking time module (`evidence\wch\ch32v203g6u6\generated\embassy\src\time.rs:99-220`, `236-317`). But the approved evidence set audited here still does not fully justify the TIM4-specific compare/alarm wake path, the device-level 1 kHz time-base assumption, or the stronger “verified” interpretation of the shared TIM4 sub-cause routing that the generated module relies on.

2. **Weakly supported claim: the TIM4 UP/TRG/CC split is verified rather than community-backed/inferred.**  
   The HAIR topology marks the split sources/routes as `status: "verified"` (`evidence\wch\ch32v203g6u6\hair.json:69050-69100`, `69695-69754`), but the official startup material only proves a single shared TIM4 vector. The sub-cause partition is therefore materially weaker than the status wording suggests.

3. **Weakly supported claim: `op.tim4.clear_cc1` is fully and precisely modeled as an executable acknowledgement path.**  
   Official WCH timer-driver code now supports the timer-family clear semantics, but the HAIR operation still does not capture the exact vendor helper form (`TIMx->INTFR = (uint16_t)~TIM_IT`) that would make this lowering claim precise (`evidence\wch\ch32v203g6u6\hair.json:63942-63971`).

4. **Weakly supported claim: `op.tim4.configure_counter_compare_timebase` is a source-backed device contract rather than a smoke-flow assumption.**  
   The operation notes already acknowledge this is repository-authored and assumes an 8 MHz timer clock (`evidence\wch\ch32v203g6u6\hair.json:63927-63939`).

## Root-Cause Classification

| Finding | Classification | Notes |
| --- | --- | --- |
| `drv.tim4-time` overclaims Embassy async readiness | missing source evidence; extraction omission | Explicit compare/alarm and interrupt-enable semantics are not carried as approved lowering inputs. |
| TIM4 UP/TRG/CC split marked fully verified | ambiguous evidence; normalization mismatch / representational difference | One shared vendor vector is being refined using community YAML topology. |
| `op.tim4.clear_cc1` operation wording vs official helper implementation | extraction omission | Official WCH timer-driver code now supports the family-level clear semantics, but the HAIR operation still abstracts that behavior too loosely. |
| `op.tim4.configure_counter_compare_timebase` 1 kHz assumption | missing source evidence; extraction omission | The operation is intentionally repository-authored and depends on a chosen clock assumption. |
| Generated `time.rs` performs more behavior than the profile declares explicitly | generator limitation; schema/profile mismatch | Generation succeeds by reading structural fields directly, but the HAIR contract is thinner than the documented async-timing burden of proof. |

## Normalization and Translation Attribution Matrix

| Difference | Approved source / reference artifact | Normalization needed? | Where the difference appears | Classification |
| --- | --- | --- | --- | --- |
| Single vendor `TIM4_IRQHandler` vector vs HAIR `tim4.up` / `tim4.trg` / `tim4.cc` split | Official startup file + community YAML interrupt topology | No | Representation/profile layer in HAIR | ambiguous evidence; representational difference |
| HAIR claims `timeDriverTickHz = 1000` via `PSC = 7999` | RM register tables + datasheet clock evidence | No | HAIR | missing source evidence |
| Generated module writes CCR1 and CC1IE directly though HAIR exposes only init/enable/clear ops | Generated `time.rs` vs `drv.tim4-time` refs | No | Generation relative to HAIR contract | generator limitation / profile mismatch |
| Generated module acknowledges pending compare interrupts with direct INTFR bit clearing | Official WCH timer driver vs `op.tim4.clear_cc1` | No | Mostly representation / operation-detail gap | extraction omission |

## Rejected Candidate Findings

| Candidate finding | Reason rejected | Safe/correcting evidence |
| --- | --- | --- |
| `int.tim4` vector number is wrong | Rejected: HAIR, official header, and official startup agree on TIM4 vector identity. | `evidence\wch\ch32v203g6u6\hair.json:914-919`; official header/startup sources from the approved manifest (`evidence\wch\ch32v203g6u6\evidence-manifest.json:34-46`) |
| PFIC controller is missing from the device model | Rejected: HAIR includes `ic.pfic` and wires `int.tim4` to it. | `evidence\wch\ch32v203g6u6\hair.json:62565-62585`, `914-919` |
| The HAIR document lacks any TIM4 structural model | Rejected: TIM4 register/field inventory, clock/reset bindings, interrupt routes, and state machine are all present. | `evidence\wch\ch32v203g6u6\hair.json:7853-7868`, `8583-8604`, `8978-9029`, `68282-68304`, `68675-68697`, `64245-64296` |

## Unresolved Differences Inventory

1. Exact vendor-backed semantics for clearing TIM4 compare and update interrupt flags on this family
2. Exact vendor-backed semantics for arming a TIM4 compare interrupt as a reusable async alarm source
3. Whether the UP/TRG/CC split on the shared TIM4 vector should remain `verified`, or should be downgraded to community-backed/inferred topology
4. Whether the repository should model the current TIM4 Embassy path as not-ready until stronger evidence is added, or should narrow the profile claim to a weaker non-async timer surface

## Coverage

- **Examined**: `hair.json`, `evidence-manifest.json`, repository schema/docs, generated Embassy output, official startup/header/example sources named in the manifest
- **Method**: input validation, provenance cross-checks, scoped reads around TIM4/PFIC/Embassy records, `hair validate`, `hair generate embassy`, and translation-gap inspection of generated `time.rs`
- **Excluded**: full register-by-register metadata reconciliation for every non-timer peripheral; a complete family-wide audit of all community `ch32-rs` overlays
- **Limitations**: the approved official source set used here does not include a TIM4-specific interrupt example; some reference-manual support is only present indirectly through extracted HAIR evidence records rather than a local text copy; local hardware-smoke failures informed suspicion but were not treated as approved-source evidence

## Limitations

- None beyond the coverage limitations above.

## Final Verdict

**UNSUPPORTED-CLAIMS**

The broader official-WCH-C cross-check shows that the **core CH32V203G6U6 structure in `hair.json` mostly agrees with the vendor SDK**. The document’s remaining failure is narrower: it still does **not** currently earn a downstream-ready Embassy async-timer claim for `drv.tim4-time`. The approved evidence audited here does not fully support:

1. the repository-authored 1 kHz TIM4 time-base configuration,
2. a TIM4-specific compare/alarm time-driver pattern for Embassy async wake scheduling, or
3. the stronger interpretation that the shared TIM4 vector is ready to serve as a proven compare-driven Embassy wake path.

`hair generate embassy` succeeding is not enough to rescue that claim. The generated `time.rs` proves the current generator can emit a timer-backed module, but the audit attributes the gap to unsupported HAIR claims and profile/generator mismatch rather than to evidence-backed readiness.
