<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

<p align="center">

# ATIPICIAL DECOMPILER — The Lens

### See what the bytecode actually does

**AEF → disassembly → high-level pseudo-code: pattern analysis, overflow detection, control-flow lifting**

</p>

<p align="center">
  <img alt="Founder" src="https://img.shields.io/badge/%F0%9F%91%91_Founder-xmoohad-ff006e?style=for-the-badge">
  <img alt="Chain" src="https://img.shields.io/badge/Chain-Atipicial_L1-9d4edd?style=for-the-badge">
  <img alt="ATC" src="https://img.shields.io/badge/%F0%9F%AA%99_ATC-Atipicial_Coin-ffd60a?style=for-the-badge">
  <img alt="ATD" src="https://img.shields.io/badge/%F0%9F%92%B5_ATD-AtipicialDollar-06d6a0?style=for-the-badge">
  <img alt="License" src="https://img.shields.io/badge/License-MIT-3a86ff?style=for-the-badge">
</p>

---


Atipicial is a sovereign Layer-1 blockchain for smart contracts, digital assets,
and decentralized applications. **The
Decompiler is its lens**: parse AEF containers, disassemble AtipicialVM
bytecode, recover control flow, and lift it to high-level pseudo-code — so
developers and auditors can inspect what a
contract *actually* does, not what its manifest claims.

Smart-contract behavior must be inspectable.
This tool turns bytecode back into readable structure: pattern analysis,
overflow collapse, royalty/standards detection, and schema-annotated output
for programmatic consumption.


---

## ⚖️ The Design Laws of ATIPICIAL DECOMPILER — The Lens

1. **Real, not fake.** Every function that claims to do something, does it.
   No stubs, no mocks wearing production clothes, no vaporware APIs.
2. **Typed or it doesn't exist.** Strings lying about being integers are a bug.
   Domain types own their invariants at construction.
3. **Determinism above all.** No nondeterministic iteration, floating point,
   or wall-clock time anywhere near consensus or state.
4. **Boundaries are law.** Layers depend downward. A service never reopens a
   database another service owns. Capabilities cross boundaries as narrow traits.
5. **Fail loud, fail early.** Invalid configuration is a startup error, not a
   runtime surprise three weeks later.
6. **Performance is earned.** Measured, benchmarked, and never traded against
   correctness.


---

## ⛓️ Chain Identity — What Every Atipicial Component Shares

| Attribute | Value |
|---|---|
| **Chain** | Atipicial Chain — sovereign Layer-1 for smart contracts and digital assets |
| **ATC** | Atipicial Coin — governance & staking, 1,000,000,000 total |
| **ATD** | AtipicialDollar — settlement & fees, 500,000,000 genesis |
| **Addresses** | Begin with capital **`A`** (version byte `0x09`) |
| **Genesis** | 2026-07-20 00:00:00 UTC |
| **Standards** | AEP-17 (fungible) · AEP-11 (NFT) · AEP-6 (wallets) · AEP-2 (keys) |
| **Format** | AEF — Atipicial Executable Format |
| **Consensus** | dBFT 2.0 — single-block finality |
| **Seeds** | `seed1-5.atipicial.com:10333` (P2P) · `seed1-5.atipicial.com:10332` (RPC) |


---

## ⚡ Quick Start — atipicial-decompiler

```bash
```
# 1 — get it
cargo build --release

# 2 — run it
./target/release/atipicial-decompiler path/to/contract.aef

# 3 — prove it works
review the lifted pseudo-code + analysis report
```


```
## 🌳 Repository Tree

```text
atipicial-decompiler/
├── .github/
│   └── workflows/
├── .gitignore
├── .planning/
│   └── debug/
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── Justfile
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── RELEASING.md
├── SECURITY.md
├── SUPPORT.md
├── TestingArtifacts/
│   ├── README.md
│   ├── edgecases/
│   ├── embedded/
│   ├── expected_invalid.txt
│   └── known_unsupported.txt
├── atipicial_csharp/
│   ├── core/
│   └── vm/
├── deny.toml
├── docs/
│   ├── logo.svg
│   ├── plans/
│   ├── schema/
│   ├── superpowers/
│   └── testing-artifacts.md
├── examples/
│   ├── README.md
│   ├── hello_world/
│   └── test_ssa.rs
├── fuzz/
│   ├── Cargo.toml
│   └── fuzz_targets/
├── js/
│   ├── README.md
│   ├── package.json
│   ├── scripts/
│   ├── src/
│   └── test/
├── rustfmt.toml
├── spec/
│   └── atipicial-decompiler-spec.tex
├── src/
│   ├── aef/
│   ├── aef.rs
│   ├── cli/
│   ├── cli.rs
│   ├── decompiler/
│   ├── decompiler.rs
│   ├── disassembler/
│   ├── disassembler.rs
│   ├── error/
│   ├── error.rs
│   ├── instruction/
│   ├── instruction.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── manifest/
│   ├── manifest.rs
│   ├── native_contracts/
│   ├── native_contracts.rs
│   ├── native_contracts_generated.rs
│   ├── opcodes_generated.rs
│   ├── syscalls.rs
│   ├── syscalls_generated.rs
│   ├── util/
│   ├── util.rs
│   ├── web/
│   └── web.rs
├── tests/
│   ├── cli_smoke/
│   ├── cli_smoke.rs
│   ├── corpus_replay.rs
│   ├── csharp_compile.rs
│   ├── decompile_artifacts/
│   ├── decompile_artifacts.rs
│   ├── ir_pipeline.rs
│   ├── ssa_e2e.rs
│   ├── typed_declarations.rs
│   └── web_api.rs
├── tools/
│   ├── batch_decompile.py
│   ├── ci/
│   ├── data/
│   ├── extract_devpack_artifacts.py
│   ├── generate_opcodes.py
│   ├── scrape_native_contracts.py
│   ├── scrape_syscalls.py
│   └── tests/
└── web/
│   ├── .npmignore
│   ├── README.md
│   ├── index.html
│   ├── main.js
│   ├── package-lock.json
│   ├── package.json
│   ├── scripts/
│   ├── src/
│   ├── test/
│   └── tsconfig.json


```
---

## 🗂️ Complete Source Registry — 467 files · 89,816 lines

Every source file in this repository, inventoried. No file hidden,
no module forgotten. This is the real map of ATIPICIAL DECOMPILER — The Lens.


### `examples/` — 1 files · 51 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `examples/test_ssa.rs` | Rust | 51 | — |

### `fuzz/` — 6 files · 592 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `fuzz/fuzz_targets/fuzz_aef_parse.rs` | Rust | 10 | ![no_main] |
| `fuzz/fuzz_targets/fuzz_decompile.rs` | Rust | 9 | ![no_main] |
| `fuzz/fuzz_targets/fuzz_decompile_raw.rs` | Rust | 48 | ![no_main] |
| `fuzz/fuzz_targets/fuzz_disassemble.rs` | Rust | 10 | ![no_main] |
| `fuzz/fuzz_targets/fuzz_grammar.rs` | Rust | 506 | ![no_main] |
| `fuzz/fuzz_targets/fuzz_manifest.rs` | Rust | 9 | ![no_main] |

### `js/` — 1 files · 558 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `js/src/index.d.ts` | TypeScript | 558 | Hash160 script hash, as uppercase hex without the `0x` prefix. */ |

### `src/` — 422 files · 80,147 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `src/aef.rs` | Rust | 36 | Atipicial AEF container parsing and helpers. |
| `src/aef/encoding.rs` | Rust | 136 | — |
| `src/aef/flags.rs` | Rust | 27 | Return the individual call flag labels set on the provided mask. |
| `src/aef/parser.rs` | Rust | 17 | Parser for Atipicial AEF containers. |
| `src/aef/parser/checksum.rs` | Rust | 23 | [must_use] |
| `src/aef/parser/method_tokens.rs` | Rust | 103 | — |
| `src/aef/parser/parse.rs` | Rust | 60 | — |
| `src/aef/parser/parse/header.rs` | Rust | 47 | offset += 4; |
| `src/aef/parser/parse/reserved.rs` | Rust | 38 | offset += 1; |
| `src/aef/parser/parse/script.rs` | Rust | 10 | offset += script_len; |
| `src/aef/parser/parse/trailer.rs` | Rust | 44 | — |
| `src/aef/tests/flags.rs` | Rust | 21 | [test] |
| `src/aef/tests/limits.rs` | Rust | 90 | [test] |
| `src/aef/tests/method_tokens.rs` | Rust | 2 | — |
| `src/aef/tests/method_tokens/errors.rs` | Rust | 260 | [test] |
| `src/aef/tests/method_tokens/parse.rs` | Rust | 44 | [test] |
| `src/aef/tests/mod.rs` | Rust | 45 | — |
| `src/aef/tests/parse.rs` | Rust | 160 | [test] |
| `src/aef/types.rs` | Rust | 83 | Parsed AEF header information. |
| `src/cli.rs` | Rust | 9 | Command line interface for inspecting and decompiling Atipicial AEF files. |
| `src/cli/args.rs` | Rust | 49 | Command line interface for the minimal Atipicial decompiler. |
| `src/cli/args/catalog.rs` | Rust | 36 | [derive(Debug, Args)] |
| `src/cli/args/commands.rs` | Rust | 122 | [derive(Debug, Subcommand)] |
| `src/cli/args/formats.rs` | Rust | 36 | [derive(Debug, Clone, Copy, ValueEnum, Default)] |
| `src/cli/args/schema.rs` | Rust | 32 | [derive(Debug, Args)] |
| `src/cli/catalog.rs` | Rust | 81 | [derive(Serialize)] |
| `src/cli/reports/instructions.rs` | Rust | 95 | [derive(Serialize)] |
| `src/cli/reports/manifest.rs` | Rust | 8 | Manifest report structures for CLI JSON output. |
| `src/cli/reports/manifest/build.rs` | Rust | 71 | — |
| `src/cli/reports/manifest/convert.rs` | Rust | 45 | — |
| `src/cli/reports/manifest/model.rs` | Rust | 86 | [derive(Serialize)] |
| `src/cli/reports/method_tokens.rs` | Rust | 90 | [derive(Serialize)] |
| `src/cli/reports/mod.rs` | Rust | 13 | JSON report structures and helpers for CLI output. |
| `src/cli/reports/types.rs` | Rust | 69 | [derive(Serialize)] |
| `src/cli/runner/catalog.rs` | Rust | 90 | — |
| `src/cli/runner/cfg.rs` | Rust | 35 | — |
| `src/cli/runner/common.rs` | Rust | 119 | — |
| `src/cli/runner/decompile.rs` | Rust | 153 | — |
| `src/cli/runner/disasm.rs` | Rust | 76 | — |
| `src/cli/runner/info.rs` | Rust | 27 | — |
| `src/cli/runner/info/json.rs` | Rust | 49 | — |
| `src/cli/runner/info/text.rs` | Rust | 123 | — |
| `src/cli/runner/mod.rs` | Rust | 60 | CLI command execution. |
| `src/cli/runner/schema.rs` | Rust | 99 | — |
| `src/cli/runner/tokens.rs` | Rust | 66 | — |
| `src/cli/schema/embedded.rs` | Rust | 23 | — |
| `src/cli/schema/kind.rs` | Rust | 66 | [derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)] |
| `src/cli/schema/metadata.rs` | Rust | 47 | [derive(Clone, Copy)] |
| `src/cli/schema/mod.rs` | Rust | 6 | — |
| `src/decompiler.rs` | Rust | 37 | High-level decompilation pipeline shared by the library and CLI. |
| `src/decompiler/analysis/call_graph.rs` | Rust | 248 | Call graph construction for Atipicial scripts. |
| `src/decompiler/analysis/call_graph/pointers.rs` | Rust | 468 | [derive(Clone, Copy, Debug, PartialEq, Eq)] |
| `src/decompiler/analysis/call_graph/pointers/arguments.rs` | Rust | 135 | Second pass over call edges: resolve CALLA targets that load their function |
| `src/decompiler/analysis/call_graph/pointers/static_values.rs` | Rust | 44 | Resolve a static delegate slot whose complete write history is one |
| `src/decompiler/analysis/method_contracts.rs` | Rust | 313 | Shared stack-call contracts for manifest and inferred methods. |
| `src/decompiler/analysis/method_contracts/calls.rs` | Rust | 204 | Call-contract construction and return-shape analysis. |
| `src/decompiler/analysis/method_contracts/collection.rs` | Rust | 455 | [derive(Debug)] |
| `src/decompiler/analysis/method_contracts/collection/effects.rs` | Rust | 278 | — |
| `src/decompiler/analysis/method_contracts/tests.rs` | Rust | 751 | [test] |
| `src/decompiler/analysis/methods.rs` | Rust | 346 | Reference to a (possibly inferred) method within a script. |
| `src/decompiler/analysis/mod.rs` | Rust | 50 | Program analysis helpers for lifted Atipicial bytecode. |
| `src/decompiler/analysis/patterns.rs` | Rust | 284 | Conservative contract and C# target metadata identification. |
| `src/decompiler/analysis/patterns/abi.rs` | Rust | 86 | — |
| `src/decompiler/analysis/patterns/language.rs` | Rust | 34 | Infer the source target supported by this decompiler. |
| `src/decompiler/analysis/patterns/native_patterns.rs` | Rust | 84 | — |
| `src/decompiler/analysis/patterns/syscall_patterns.rs` | Rust | 94 | — |
| `src/decompiler/analysis/patterns/tests.rs` | Rust | 535 | Pattern-identification tests. |
| `src/decompiler/analysis/types.rs` | Rust | 208 | Lightweight type inference for lifted Atipicial bytecode. |
| `src/decompiler/analysis/types/infer.rs` | Rust | 534 | — |
| `src/decompiler/analysis/xrefs.rs` | Rust | 233 | Cross-reference analysis for local/argument/static slots. |
| `src/decompiler/cfg/basic_block/block.rs` | Rust | 57 | A basic block: a sequence of instructions with single entry and exit. |
| `src/decompiler/cfg/basic_block/block_id.rs` | Rust | 32 | Unique identifier for a basic block within a CFG. |
| `src/decompiler/cfg/basic_block/mod.rs` | Rust | 9 | Basic block representation for CFG. |
| `src/decompiler/cfg/basic_block/terminator.rs` | Rust | 115 | How a basic block terminates. |
| `src/decompiler/cfg/builder.rs` | Rust | 69 | CFG construction from instruction stream. |
| `src/decompiler/cfg/builder/blocks.rs` | Rust | 39 | — |
| `src/decompiler/cfg/builder/edges.rs` | Rust | 64 | — |
| `src/decompiler/cfg/builder/finally.rs` | Rust | 373 | [derive(Debug)] |
| `src/decompiler/cfg/builder/leaders.rs` | Rust | 87 | — |
| `src/decompiler/cfg/builder/offsets.rs` | Rust | 57 | — |
| `src/decompiler/cfg/builder/targets.rs` | Rust | 91 | ![allow( |
| `src/decompiler/cfg/builder/terminator.rs` | Rust | 142 | — |
| `src/decompiler/cfg/graph.rs` | Rust | 11 | Control Flow Graph structure and operations. |
| `src/decompiler/cfg/graph/core.rs` | Rust | 161 | A Control Flow Graph representing the structure of a function/contract. |
| `src/decompiler/cfg/graph/dot.rs` | Rust | 49 | [must_use] |
| `src/decompiler/cfg/graph/edge.rs` | Rust | 31 | An edge in the CFG. |
| `src/decompiler/cfg/graph/reachability.rs` | Rust | 52 | [must_use] |
| `src/decompiler/cfg/graph/traversal.rs` | Rust | 36 | [must_use] |
| `src/decompiler/cfg/method_body.rs` | Rust | 296 | [path = "method_body_names.rs"] |
| `src/decompiler/cfg/method_body/cfg.rs` | Rust | 86 | Build a self-contained CFG for one method slice. |
| `src/decompiler/cfg/method_body/opcode.rs` | Rust | 56 | [derive(Debug, Clone, Copy, PartialEq, Eq)] |
| `src/decompiler/cfg/method_body/source_map.rs` | Rust | 141 | next_id += 1; |
| `src/decompiler/cfg/method_body/validation.rs` | Rust | 210 | [derive(Default)] |
| `src/decompiler/cfg/method_body_names.rs` | Rust | 158 | — |
| `src/decompiler/cfg/method_body_symbols.rs` | Rust | 169 | — |
| `src/decompiler/cfg/method_body_tests.rs` | Rust | 512 | [test] |
| `src/decompiler/cfg/method_body_types.rs` | Rust | 500 | [cfg(test)] |
| `src/decompiler/cfg/method_view.rs` | Rust | 479 | Per-method view of a contract for the structured-IR renderer. |
| `src/decompiler/cfg/mod.rs` | Rust | 23 | Control Flow Graph (CFG) construction and analysis. |
| `src/decompiler/cfg/phi_lowering.rs` | Rust | 508 | [derive(Clone, Debug, PartialEq, Eq)] |
| `src/decompiler/cfg/ssa/builder.rs` | Rust | 279 | Stack-effect SSA construction from a CFG and instruction stream. |
| `src/decompiler/cfg/ssa/builder/collection.rs` | Rust | 623 | — |
| `src/decompiler/cfg/ssa/builder/diagnostics.rs` | Rust | 125 | Instruction fidelity and stack-loss diagnostics for SSA lowering. |
| `src/decompiler/cfg/ssa/builder/expr.rs` | Rust | 95 | Opcode-to-expression lowering for the SSA builder. |
| `src/decompiler/cfg/ssa/builder/helpers.rs` | Rust | 250 | Map a binary compute opcode to its IR operator, if applicable. |
| `src/decompiler/cfg/ssa/builder/instructions.rs` | Rust | 396 | — |
| `src/decompiler/cfg/ssa/builder/instructions/calls.rs` | Rust | 301 | — |
| `src/decompiler/cfg/ssa/builder/instructions/indexed.rs` | Rust | 114 | — |
| `src/decompiler/cfg/ssa/builder/instructions/reorder.rs` | Rust | 96 | — |
| `src/decompiler/cfg/ssa/builder/instructions/special.rs` | Rust | 261 | Operand-dependent collection and stack-special instruction handling. |
| `src/decompiler/cfg/ssa/builder/instructions/syscall.rs` | Rust | 101 | — |
| `src/decompiler/cfg/ssa/builder/joins.rs` | Rust | 481 | bid, |
| `src/decompiler/cfg/ssa/builder/pipeline.rs` | Rust | 370 | Fixpoint orchestration and straight-line execution for SSA construction. |
| `src/decompiler/cfg/ssa/builder/slots.rs` | Rust | 141 | Slot naming and static/local reaching-definition helpers. |
| `src/decompiler/cfg/ssa/builder/tests.rs` | Rust | 146 | Build instructions + a matching CFG for a straight-line program. |
| `src/decompiler/cfg/ssa/builder/tests/calls.rs` | Rust | 665 | [test] |
| `src/decompiler/cfg/ssa/builder/tests/collection_facts.rs` | Rust | 737 | [test] |
| `src/decompiler/cfg/ssa/builder/tests/control_flow.rs` | Rust | 755 | [test] |
| `src/decompiler/cfg/ssa/builder/tests/dynamic_stack.rs` | Rust | 529 | [test] |
| `src/decompiler/cfg/ssa/builder/tests/fidelity.rs` | Rust | 279 | [test] |
| `src/decompiler/cfg/ssa/builder/tests/stack_ops.rs` | Rust | 738 | [test] |
| `src/decompiler/cfg/ssa/context.rs` | Rust | 153 | Exact fixed-length collection shape proven by SSA. |
| `src/decompiler/cfg/ssa/dominance.rs` | Rust | 764 | Dominance analysis for SSA construction. |
| `src/decompiler/cfg/ssa/effects.rs` | Rust | 246 | Stack-effect model for every Atipicial VM opcode. |
| `src/decompiler/cfg/ssa/form.rs` | Rust | 563 | SSA form types for representing code in static single assignment form. |
| `src/decompiler/cfg/ssa/form/expr.rs` | Rust | 232 | SSA expression vocabulary and constructors. |
| `src/decompiler/cfg/ssa/form/stmt.rs` | Rust | 122 | SSA statement vocabulary and constructors. |
| `src/decompiler/cfg/ssa/mod.rs` | Rust | 39 | Static Single Assignment (SSA) construction and analysis. |
| `src/decompiler/cfg/ssa/optimize.rs` | Rust | 654 | SSA optimization passes. |
| `src/decompiler/cfg/ssa/optimize/indexes.rs` | Rust | 75 | Recompute the `definitions` and `uses` indexes from the current blocks. |
| `src/decompiler/cfg/ssa/optimize/tests.rs` | Rust | 728 | A named slot's constant value must NOT be substituted into its uses: a |
| `src/decompiler/cfg/ssa/to_ir.rs` | Rust | 324 | Lower [`SsaForm`] back to the typed [`crate::decompiler::ir`] for rendering. |
| `src/decompiler/cfg/ssa/variable.rs` | Rust | 303 | SSA variable and φ node types. |
| `src/decompiler/cfg/structure.rs` | Rust | 543 | Structural control-flow recovery: CFG → typed `ir::Block`. |
| `src/decompiler/cfg/structure/analysis.rs` | Rust | 233 | — |
| `src/decompiler/cfg/structure/branches.rs` | Rust | 193 | then_distances.get(&block)?.max(else_distances.get(&block)?), |
| `src/decompiler/cfg/structure/cleanup.rs` | Rust | 255 | Readability-oriented temporary reduction (copy propagation, redundant cast |
| `src/decompiler/cfg/structure/cleanup/int_normalization.rs` | Rust | 521 | Collapse compiler-generated unchecked `int32` normalization wrappers. |
| `src/decompiler/cfg/structure/cleanup/int_normalization_uses.rs` | Rust | 144 | Expression/statement liveness checks used by int32 normalization cleanup. |
| `src/decompiler/cfg/structure/cleanup/loops.rs` | Rust | 453 | Recovery of compiler-generated counting loops. |
| `src/decompiler/cfg/structure/cleanup/size_normalization.rs` | Rust | 554 | Collapse compiler-generated `SIZE`-guarded signed integer normalization. |
| `src/decompiler/cfg/structure/cleanup/temps.rs` | Rust | 307 | Readability-oriented temporary reduction over structured IR. |
| `src/decompiler/cfg/structure/cleanup/temps/arrays.rs` | Rust | 146 | Array-constructor and SETITEM folding for structured IR. |
| `src/decompiler/cfg/structure/cleanup/temps/casts.rs` | Rust | 65 | Redundant cast collapsing for structured IR. |
| `src/decompiler/cfg/structure/cleanup/temps/copies.rs` | Rust | 127 | Single-use copy propagation for structured IR. |
| `src/decompiler/cfg/structure/cleanup/temps/dead_stores.rs` | Rust | 44 | Dead-store elimination for structured IR. |
| `src/decompiler/cfg/structure/cleanup/temps/merges.rs` | Rust | 130 | Phi-style branch value-merge folding for structured IR. |
| `src/decompiler/cfg/structure/cleanup/temps/queries.rs` | Rust | 277 | Safety and variable queries shared by temporary-reduction passes. |
| `src/decompiler/cfg/structure/cleanup/temps/support.rs` | Rust | 361 | Shared IR traversal helpers for temporary-reduction passes. |
| `src/decompiler/cfg/structure/for_loops.rs` | Rust | 502 | [cfg(test)] |
| `src/decompiler/cfg/structure/for_loops/terminal_update.rs` | Rust | 279 | Recover a compiler-shaped scan loop whose induction update is the |
| `src/decompiler/cfg/structure/graph.rs` | Rust | 235 | CFG graph queries used by structural control-flow recovery. |
| `src/decompiler/cfg/structure/loops.rs` | Rust | 158 | — |
| `src/decompiler/cfg/structure/regions.rs` | Rust | 328 | — |
| `src/decompiler/cfg/structure/switches.rs` | Rust | 181 | Switch-cascade recovery for the CFG structurer. |
| `src/decompiler/cfg/structure/tests.rs` | Rust | 325 | Build a diamond: BB0 branches to BB1/BB2, both jump to BB3 (merge/ret). |
| `src/decompiler/cfg/structure/tests_branches_loops.rs` | Rust | 1710 | [test] |
| `src/decompiler/cfg/structure/tests_do_while_switch.rs` | Rust | 376 | A do-while: BB0 (body entry, falls through to the latch) is the loop |
| `src/decompiler/cfg/structure/tests_entry_phi.rs` | Rust | 305 | [test] |
| `src/decompiler/cfg/structure/tests_irreducible_phi.rs` | Rust | 406 | [test] |
| `src/decompiler/cfg/structure/tests_terminal_loops.rs` | Rust | 175 | [test] |
| `src/decompiler/cfg/structure/tests_try_regions.rs` | Rust | 467 | A try/catch: TryEntry{body, catch, finally=None}; body and catch both |
| `src/decompiler/cfg/structure/try_regions.rs` | Rust | 199 | Exception-region recovery for the CFG structurer. |
| `src/decompiler/cfg/tests.rs` | Rust | 18 | Tests for CFG construction. |
| `src/decompiler/cfg/tests/basic.rs` | Rust | 65 | [test] |
| `src/decompiler/cfg/tests/dot.rs` | Rust | 15 | [test] |
| `src/decompiler/cfg/tests/jumps.rs` | Rust | 104 | [test] |
| `src/decompiler/cfg/tests/reachability.rs` | Rust | 17 | [test] |
| `src/decompiler/cfg/tests/rpo.rs` | Rust | 19 | [test] |
| `src/decompiler/cfg/tests/terminators.rs` | Rust | 102 | [test] |
| `src/decompiler/cfg/tests/try_blocks.rs` | Rust | 441 | [test] |
| `src/decompiler/csharp.rs` | Rust | 14 | C# output renderer for the decompiler. |
| `src/decompiler/csharp/helpers.rs` | Rust | 273 | [path = "helpers/signatures.rs"] |
| `src/decompiler/csharp/helpers/legacy_expression.rs` | Rust | 31 | Compatibility rewrites for the legacy high-level C# test renderer. |
| `src/decompiler/csharp/helpers/legacy_expression/collections.rs` | Rust | 184 | Collection and conversion rewrites for legacy lifted C# expressions. |
| `src/decompiler/csharp/helpers/legacy_expression/literals.rs` | Rust | 77 | Literal normalization for legacy lifted C# expressions. |
| `src/decompiler/csharp/helpers/legacy_expression/numeric.rs` | Rust | 163 | Numeric helper lowering for legacy lifted C# expressions. |
| `src/decompiler/csharp/helpers/legacy_expression_scanner.rs` | Rust | 219 | String- and nesting-aware scanners used by the legacy C# expression pass. |
| `src/decompiler/csharp/helpers/signatures.rs` | Rust | 223 | [derive(Clone)] |
| `src/decompiler/csharp/render.rs` | Rust | 384 | C# skeleton renderer. |
| `src/decompiler/csharp/render/body.rs` | Rust | 229 | [derive(Debug, Clone, Copy, PartialEq, Eq)] |
| `src/decompiler/csharp/render/body/fidelity.rs` | Rust | 132 | — |
| `src/decompiler/csharp/render/body/recovery.rs` | Rust | 257 | [path = "recovery_underflow.rs"] |
| `src/decompiler/csharp/render/body/recovery_underflow.rs` | Rust | 67 | C# rendering policy for calls whose VM argument stack underflows. |
| `src/decompiler/csharp/render/events.rs` | Rust | 69 | — |
| `src/decompiler/csharp/render/header/helpers.rs` | Rust | 122 | Generated C# helper declarations used by lifted VM bodies. |
| `src/decompiler/csharp/render/header/metadata.rs` | Rust | 64 | AEF token and manifest-trust metadata comments for generated C#. |
| `src/decompiler/csharp/render/header/mod.rs` | Rust | 241 | Render the inferred contract standards and C# target metadata as comments |
| `src/decompiler/csharp/render/methods.rs` | Rust | 354 | — |
| `src/decompiler/csharp/render/structured/declaration_type_catalog.rs` | Rust | 330 | Convert the VM-level value category to the concrete C# type used by the |
| `src/decompiler/csharp/render/structured/declaration_types.rs` | Rust | 337 | [cfg_attr(not(test), allow(dead_code))] |
| `src/decompiler/csharp/render/structured/declarations.rs` | Rust | 424 | [derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] |
| `src/decompiler/csharp/render/structured/expr.rs` | Rust | 662 | [derive(Debug)] |
| `src/decompiler/csharp/render/structured/expr_calls.rs` | Rust | 72 | Semantic call-target rendering for structured C# expressions. |
| `src/decompiler/csharp/render/structured/expr_context.rs` | Rust | 448 | [path = "expr_context_index.rs"] |
| `src/decompiler/csharp/render/structured/expr_context_index.rs` | Rust | 71 | Provenance-aware typing for constant indexes in structured C# expressions. |
| `src/decompiler/csharp/render/structured/expr_context_intrinsics.rs` | Rust | 62 | VM intrinsic result typing for [`ExprContext`]. |
| `src/decompiler/csharp/render/structured/expr_context_patterns.rs` | Rust | 161 | Find compiler-generated state temporaries used by the `Runtime.Debug` |
| `src/decompiler/csharp/render/structured/expr_context_types.rs` | Rust | 73 | Type and collection-provenance helpers for structured expression contexts. |
| `src/decompiler/csharp/render/structured/expr_inline.rs` | Rust | 316 | [derive(Debug)] |
| `src/decompiler/csharp/render/structured/expr_intrinsics.rs` | Rust | 412 | — |
| `src/decompiler/csharp/render/structured/expr_intrinsics/bytes.rs` | Rust | 131 | — |
| `src/decompiler/csharp/render/structured/expr_intrinsics/nullability.rs` | Rust | 68 | VM value categories are broader than C# nullability. In particular, a |
| `src/decompiler/csharp/render/structured/expr_low_level.rs` | Rust | 97 | Low-level opcode and explicit type-tag rendering for structured C#. |
| `src/decompiler/csharp/render/structured/expr_native.rs` | Rust | 156 | C# rendering for resolved native-contract method tokens. |
| `src/decompiler/csharp/render/structured/expr_syscalls.rs` | Rust | 277 | [path = "expr_syscalls_catalog.rs"] |
| `src/decompiler/csharp/render/structured/expr_syscalls_catalog.rs` | Rust | 210 | C# API bindings for syscall hashes recognized by the structured renderer. |
| `src/decompiler/csharp/render/structured/expr_values.rs` | Rust | 107 | — |
| `src/decompiler/csharp/render/structured/mod.rs` | Rust | 21 | [cfg(test)] |
| `src/decompiler/csharp/render/structured/native_framework.rs` | Rust | 182 | C# framework bindings for catalogued native-contract method tokens. |
| `src/decompiler/csharp/render/structured/nullability.rs` | Rust | 112 | Resolve only direct slot aliases feeding ISNULL. This intentionally stops |
| `src/decompiler/csharp/render/structured/plan.rs` | Rust | 172 | [path = "declaration_type_catalog.rs"] |
| `src/decompiler/csharp/render/structured/plan_activity.rs` | Rust | 366 | [path = "plan_activity/visitor.rs"] |
| `src/decompiler/csharp/render/structured/plan_activity/visitor.rs` | Rust | 220 | — |
| `src/decompiler/csharp/render/structured/plan_helpers.rs` | Rust | 188 | occurrence += 1; |
| `src/decompiler/csharp/render/structured/plan_methods.rs` | Rust | 398 | [path = "plan_methods/calls.rs"] |
| `src/decompiler/csharp/render/structured/plan_methods/calls.rs` | Rust | 197 | Build the resolved call contracts used by each C# method plan. |
| `src/decompiler/csharp/render/structured/plan_methods/parameter_calls.rs` | Rust | 219 | Internal-call argument extraction for private-helper inference. |
| `src/decompiler/csharp/render/structured/plan_methods/parameter_names.rs` | Rust | 362 | Source-meaningful parameter names for private helper methods. |
| `src/decompiler/csharp/render/structured/plan_methods/parameter_names/tests.rs` | Rust | 67 | [test] |
| `src/decompiler/csharp/render/structured/plan_methods/parameter_names/traversal.rs` | Rust | 136 | Recursive expression traversal for helper-parameter inference. |
| `src/decompiler/csharp/render/structured/plan_methods/parameter_types.rs` | Rust | 311 | Conservative C# parameter inference for private helper methods. |
| `src/decompiler/csharp/render/structured/plan_methods/return_types.rs` | Rust | 243 | Resolve concrete C# return types for inferred private methods only. |
| `src/decompiler/csharp/render/structured/stmt.rs` | Rust | 464 | [path = "stmt_control_flow.rs"] |
| `src/decompiler/csharp/render/structured/stmt_control_flow.rs` | Rust | 326 | Structured control-flow statement rendering. |
| `src/decompiler/csharp/render/structured/stmt_facts.rs` | Rust | 94 | Prefix definition facts used to validate indexed-loop recovery. |
| `src/decompiler/csharp/render/structured/stmt_foreach.rs` | Rust | 325 | Conservative recovery of compiler-generated indexed `foreach` loops. |
| `src/decompiler/csharp/render/structured/stmt_foreach_guards.rs` | Rust | 360 | Recursive safety guards for indexed-loop recovery. |
| `src/decompiler/csharp/render/structured/stmt_values.rs` | Rust | 263 | — |
| `src/decompiler/csharp/render/structured/tests.rs` | Rust | 83 | [path = "tests_events.rs"] |
| `src/decompiler/csharp/render/structured/tests_events.rs` | Rust | 126 | [test] |
| `src/decompiler/csharp/render/structured/tests_expr.rs` | Rust | 10 | [path = "tests_expr_collections.rs"] |
| `src/decompiler/csharp/render/structured/tests_expr_collections.rs` | Rust | 270 | [test] |
| `src/decompiler/csharp/render/structured/tests_expr_core.rs` | Rust | 559 | [test] |
| `src/decompiler/csharp/render/structured/tests_expr_formatting.rs` | Rust | 202 | [test] |
| `src/decompiler/csharp/render/structured/tests_expr_operators.rs` | Rust | 217 | [test] |
| `src/decompiler/csharp/render/structured/tests_expr_syscalls.rs` | Rust | 475 | [test] |
| `src/decompiler/csharp/render/structured/tests_expr_types.rs` | Rust | 663 | [test] |
| `src/decompiler/csharp/render/structured/tests_plan_declarations.rs` | Rust | 324 | [test] |
| `src/decompiler/csharp/render/structured/tests_plan_methods.rs` | Rust | 1052 | [test] |
| `src/decompiler/csharp/render/structured/tests_plan_types.rs` | Rust | 535 | [test] |
| `src/decompiler/csharp/render/structured/tests_stmt.rs` | Rust | 6 | [path = "tests_stmt_control.rs"] |
| `src/decompiler/csharp/render/structured/tests_stmt_control.rs` | Rust | 246 | [test] |
| `src/decompiler/csharp/render/structured/tests_stmt_foreach.rs` | Rust | 499 | [test] |
| `src/decompiler/csharp/render/structured/tests_stmt_inlining.rs` | Rust | 434 | [test] |
| `src/decompiler/csharp/render/structured/tests_stmt_types.rs` | Rust | 535 | [test] |
| `src/decompiler/decompilation.rs` | Rust | 222 | Result of a successful decompilation run. |
| `src/decompiler/helpers.rs` | Rust | 26 | Small helper utilities shared across decompiler renderers. |
| `src/decompiler/helpers/identifiers.rs` | Rust | 49 | Sanitize an arbitrary manifest or user-provided identifier into a stable |
| `src/decompiler/helpers/lifted.rs` | Rust | 347 | Build method argument counts keyed by method start offset. |
| `src/decompiler/helpers/manifest.rs` | Rust | 48 | Extract and sanitize the contract name from a manifest, falling back to |
| `src/decompiler/helpers/methods.rs` | Rust | 402 | Return the ABI method that matches the script entry offset, falling back to |
| `src/decompiler/helpers/parameters.rs` | Rust | 27 | Format ABI parameters into `name: type` pseudo-signature entries. |
| `src/decompiler/helpers/types.rs` | Rust | 88 | [cfg(test)] |
| `src/decompiler/helpers/vm_values.rs` | Rust | 139 | Decode a Atipicial VM StackItemType operand into the language-neutral type model. |
| `src/decompiler/high_level.rs` | Rust | 8 | [cfg(test)] |
| `src/decompiler/high_level/emitter/control_flow.rs` | Rust | 11 | Control flow lifting helpers for the high-level emitter. |
| `src/decompiler/high_level/emitter/control_flow/branches.rs` | Rust | 454 | ![allow( |
| `src/decompiler/high_level/emitter/control_flow/jumps.rs` | Rust | 306 | — |
| `src/decompiler/high_level/emitter/control_flow/loops.rs` | Rust | 212 | ![allow( |
| `src/decompiler/high_level/emitter/control_flow/targets.rs` | Rust | 54 | ![allow( |
| `src/decompiler/high_level/emitter/control_flow/try_blocks.rs` | Rust | 5 | TRY/CATCH/FINALLY lifting helpers. |
| `src/decompiler/high_level/emitter/control_flow/try_blocks/emit.rs` | Rust | 143 | closer_entry += 1; |
| `src/decompiler/high_level/emitter/control_flow/try_blocks/search.rs` | Rust | 87 | — |
| `src/decompiler/high_level/emitter/control_flow/try_blocks/targets.rs` | Rust | 51 | ![allow( |
| `src/decompiler/high_level/emitter/core.rs` | Rust | 314 | — |
| `src/decompiler/high_level/emitter/dispatch.rs` | Rust | 34 | Instruction dispatch table for the high-level emitter. |
| `src/decompiler/high_level/emitter/dispatch/collections.rs` | Rust | 64 | — |
| `src/decompiler/high_level/emitter/dispatch/control_flow.rs` | Rust | 110 | — |
| `src/decompiler/high_level/emitter/dispatch/literals.rs` | Rust | 82 | — |
| `src/decompiler/high_level/emitter/dispatch/math.rs` | Rust | 61 | — |
| `src/decompiler/high_level/emitter/dispatch/slots.rs` | Rust | 65 | — |
| `src/decompiler/high_level/emitter/dispatch/stack_ops.rs` | Rust | 34 | — |
| `src/decompiler/high_level/emitter/helpers.rs` | Rust | 109 | ![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)] |
| `src/decompiler/high_level/emitter/mod.rs` | Rust | 119 | Maximum number of instructions a single method body may have before |
| `src/decompiler/high_level/emitter/postprocess.rs` | Rust | 17 | Post-processing passes applied to lifted high-level statements. |
| `src/decompiler/high_level/emitter/postprocess/compound_assign.rs` | Rust | 121 | statement = format!("for ({init}; {condition}; {rewritten}) {{"); |
| `src/decompiler/high_level/emitter/postprocess/else_if.rs` | Rust | 91 | Collapse `} else { if condition {` into `} else if condition {`. |
| `src/decompiler/high_level/emitter/postprocess/else_if/tests.rs` | Rust | 41 | [test] |
| `src/decompiler/high_level/emitter/postprocess/for_loops.rs` | Rust | 162 | [cfg(test)] |
| `src/decompiler/high_level/emitter/postprocess/indexing.rs` | Rust | 193 | Rewrite collection ops into more idiomatic indexing syntax. |
| `src/decompiler/high_level/emitter/postprocess/inline.rs` | Rust | 5 | Inline expansion passes used by the high-level emitter. |
| `src/decompiler/high_level/emitter/postprocess/inline/condition.rs` | Rust | 112 | [cfg(test)] |
| `src/decompiler/high_level/emitter/postprocess/inline/for_increment.rs` | Rust | 103 | [cfg(test)] |
| `src/decompiler/high_level/emitter/postprocess/inline/single_use.rs` | Rust | 33 | [derive(Debug, Clone)] |
| `src/decompiler/high_level/emitter/postprocess/inline/single_use/analysis.rs` | Rust | 69 | use_counts.entry(var.clone()).or_insert(0) += 1; |
| `src/decompiler/high_level/emitter/postprocess/inline/single_use/rewrite.rs` | Rust | 42 | statement = updated; |
| `src/decompiler/high_level/emitter/postprocess/inline/single_use/tests.rs` | Rust | 65 | [test] |
| `src/decompiler/high_level/emitter/postprocess/inline/single_use/util.rs` | Rust | 168 | Walk `expr` and return `true` only if every `name(` it contains |
| `src/decompiler/high_level/emitter/postprocess/join_close_chain.rs` | Rust | 60 | Final-pass formatting cleanup: join a standalone close-brace line |
| `src/decompiler/high_level/emitter/postprocess/labels.rs` | Rust | 59 | — |
| `src/decompiler/high_level/emitter/postprocess/overflow_collapse.rs` | Rust | 576 | Collapse verbose Atipicial C# compiler overflow-check patterns into clean expressions. |
| `src/decompiler/high_level/emitter/postprocess/simplify.rs` | Rust | 506 | Return `true` when every `name(` call in `expr` starts with a |
| `src/decompiler/high_level/emitter/postprocess/simplify/temps.rs` | Rust | 143 | Temporary-value cleanup passes. |
| `src/decompiler/high_level/emitter/postprocess/switches.rs` | Rust | 549 | Rewrite `if` / `else if` equality chains into `switch` statements. |
| `src/decompiler/high_level/emitter/postprocess/switches/chain.rs` | Rust | 139 | Equality-chain to switch rewriting. |
| `src/decompiler/high_level/emitter/postprocess/switches/guarded.rs` | Rust | 161 | Guarded-goto switch rewriting. |
| `src/decompiler/high_level/emitter/postprocess/util.rs` | Rust | 15 | Shared helper routines for postprocessing lifted statements. |
| `src/decompiler/high_level/emitter/postprocess/util/analysis.rs` | Rust | 97 | — |
| `src/decompiler/high_level/emitter/postprocess/util/blocks.rs` | Rust | 22 | — |
| `src/decompiler/high_level/emitter/postprocess/util/ident.rs` | Rust | 62 | — |
| `src/decompiler/high_level/emitter/postprocess/util/model.rs` | Rust | 6 | [derive(Debug, Clone)] |
| `src/decompiler/high_level/emitter/postprocess/util/parsing.rs` | Rust | 101 | A valid LHS is a simple identifier: starts with a letter or underscore, |
| `src/decompiler/high_level/emitter/postprocess/util/patterns.rs` | Rust | 47 | Shared pattern-matching helpers for recognising `if`/`else`/`else if` |
| `src/decompiler/high_level/emitter/postprocess/util/scan.rs` | Rust | 46 | — |
| `src/decompiler/high_level/emitter/postprocess/while_loops.rs` | Rust | 521 | stmt = format!("{indent}break;"); |
| `src/decompiler/high_level/emitter/slots.rs` | Rust | 160 | — |
| `src/decompiler/high_level/emitter/stack.rs` | Rust | 7 | Stack lifting helpers for the high-level emitter. |
| `src/decompiler/high_level/emitter/stack/expressions.rs` | Rust | 7 | Expression-building helpers for the high-level emitter. |
| `src/decompiler/high_level/emitter/stack/expressions/collections.rs` | Rust | 283 | — |
| `src/decompiler/high_level/emitter/stack/expressions/core.rs` | Rust | 133 | — |
| `src/decompiler/high_level/emitter/stack/expressions/flow.rs` | Rust | 192 | — |
| `src/decompiler/high_level/emitter/stack/manipulation.rs` | Rust | 74 | Stack manipulation opcode handlers. |
| `src/decompiler/high_level/emitter/stack/manipulation/basic.rs` | Rust | 99 | — |
| `src/decompiler/high_level/emitter/stack/manipulation/indexed.rs` | Rust | 109 | — |
| `src/decompiler/high_level/emitter/stack/manipulation/reorder.rs` | Rust | 68 | — |
| `src/decompiler/high_level/emitter/stack/manipulation/reverse.rs` | Rust | 44 | — |
| `src/decompiler/high_level/emitter/types.rs` | Rust | 32 | [derive(Clone, Copy, Debug, PartialEq, Eq)] |
| `src/decompiler/high_level/emitter/util.rs` | Rust | 70 | — |
| `src/decompiler/high_level/render.rs` | Rust | 183 | Render the high-level pseudo-contract view. |
| `src/decompiler/high_level/render/body.rs` | Rust | 95 | — |
| `src/decompiler/high_level/render/entry.rs` | Rust | 139 | — |
| `src/decompiler/high_level/render/header.rs` | Rust | 47 | — |
| `src/decompiler/high_level/render/manifest_summary.rs` | Rust | 139 | — |
| `src/decompiler/high_level/render/method_tokens.rs` | Rust | 46 | — |
| `src/decompiler/high_level/render/methods.rs` | Rust | 184 | — |
| `src/decompiler/ir/control_flow.rs` | Rust | 104 | Control flow IR nodes for decompiled code. |
| `src/decompiler/ir/expression.rs` | Rust | 9 | Expression IR nodes for decompiled code. |
| `src/decompiler/ir/expression/expr.rs` | Rust | 119 | Expression nodes in the IR. |
| `src/decompiler/ir/expression/literal.rs` | Rust | 31 | Literal values that can appear in expressions. |
| `src/decompiler/ir/expression/operators.rs` | Rust | 90 | Binary operators. |
| `src/decompiler/ir/mod.rs` | Rust | 24 | Typed intermediate representation for decompiled code. |
| `src/decompiler/ir/render.rs` | Rust | 7 | IR to text rendering utilities. |
| `src/decompiler/ir/render/expr.rs` | Rust | 78 | — |
| `src/decompiler/ir/render/stmt/control_flow.rs` | Rust | 112 | — |
| `src/decompiler/ir/render/stmt/mod.rs` | Rust | 51 | [must_use] |
| `src/decompiler/ir/semantic.rs` | Rust | 98 | A VM operation represented as a call-shaped expression in shared IR. |
| `src/decompiler/ir/statement.rs` | Rust | 136 | Statement IR nodes for decompiled code. |
| `src/decompiler/ir/tests.rs` | Rust | 6 | Tests for the IR module. |
| `src/decompiler/ir/tests/block.rs` | Rust | 29 | [test] |
| `src/decompiler/ir/tests/control_flow_rendering.rs` | Rust | 54 | [test] |
| `src/decompiler/ir/tests/expression_rendering.rs` | Rust | 77 | [test] |
| `src/decompiler/ir/tests/statement_rendering.rs` | Rust | 23 | [test] |
| `src/decompiler/native_method_types.rs` | Rust | 325 | Return types for native method tokens with stable Atipicial C# framework APIs. |
| `src/decompiler/output_format.rs` | Rust | 63 | [cfg(feature = "cli")] |
| `src/decompiler/pipeline.rs` | Rust | 261 | Main entry point used by the CLI and tests. |
| `src/decompiler/pipeline/io.rs` | Rust | 52 | — |
| `src/decompiler/pseudocode.rs` | Rust | 16 | Render a simple offset + mnemonic listing mirroring the disassembly stream. |
| `src/decompiler/syscall_types.rs` | Rust | 128 | Return types for syscall-backed Atipicial C# framework APIs. |
| `src/decompiler/tests/core.rs` | Rust | 8 | — |
| `src/decompiler/tests/core/analysis.rs` | Rust | 1344 | [test] |
| `src/decompiler/tests/core/decompile.rs` | Rust | 977 | [test] |
| `src/decompiler/tests/core/entry_point.rs` | Rust | 148 | [test] |
| `src/decompiler/tests/core/identifiers.rs` | Rust | 117 | [test] |
| `src/decompiler/tests/core/syscalls.rs` | Rust | 332 | [test] |
| `src/decompiler/tests/core/unknowns.rs` | Rust | 65 | [test] |
| `src/decompiler/tests/csharp.rs` | Rust | 48 | [path = "csharp_body_core.rs"] |
| `src/decompiler/tests/csharp_body_core.rs` | Rust | 707 | [test] |
| `src/decompiler/tests/csharp_control_flow.rs` | Rust | 286 | [test] |
| `src/decompiler/tests/csharp_coverage.rs` | Rust | 294 | [derive(Debug)] |
| `src/decompiler/tests/csharp_fidelity.rs` | Rust | 388 | [test] |
| `src/decompiler/tests/csharp_legacy.rs` | Rust | 401 | [test] |
| `src/decompiler/tests/csharp_literals.rs` | Rust | 234 | [test] |
| `src/decompiler/tests/csharp_metadata.rs` | Rust | 279 | [test] |
| `src/decompiler/tests/csharp_methods.rs` | Rust | 858 | [test] |
| `src/decompiler/tests/csharp_packing.rs` | Rust | 470 | [test] |
| `src/decompiler/tests/high_level.rs` | Rust | 9 | — |
| `src/decompiler/tests/high_level/branches.rs` | Rust | 229 | [test] |
| `src/decompiler/tests/high_level/entry_range.rs` | Rust | 131 | [test] |
| `src/decompiler/tests/high_level/loops.rs` | Rust | 8 | — |
| `src/decompiler/tests/high_level/loops/break_continue.rs` | Rust | 28 | [test] |
| `src/decompiler/tests/high_level/loops/do_while.rs` | Rust | 24 | [test] |
| `src/decompiler/tests/high_level/loops/for_loop.rs` | Rust | 27 | [test] |
| `src/decompiler/tests/high_level/loops/inlining.rs` | Rust | 44 | [test] |
| `src/decompiler/tests/high_level/loops/loopif_recovery.rs` | Rust | 38 | [test] |
| `src/decompiler/tests/high_level/loops/while_loop.rs` | Rust | 24 | [test] |
| `src/decompiler/tests/high_level/postprocess.rs` | Rust | 725 | [test] |
| `src/decompiler/tests/high_level/stack_ops.rs` | Rust | 4 | — |
| `src/decompiler/tests/high_level/stack_ops/collections.rs` | Rust | 184 | [test] |
| `src/decompiler/tests/high_level/stack_ops/control.rs` | Rust | 73 | [test] |
| `src/decompiler/tests/high_level/stack_ops/manipulation.rs` | Rust | 4 | — |
| `src/decompiler/tests/high_level/stack_ops/manipulation/basic.rs` | Rust | 41 | [test] |
| `src/decompiler/tests/high_level/stack_ops/manipulation/indexed.rs` | Rust | 119 | [test] |
| `src/decompiler/tests/high_level/stack_ops/manipulation/reorder.rs` | Rust | 50 | [test] |
| `src/decompiler/tests/high_level/stack_ops/manipulation/reverse.rs` | Rust | 95 | [test] |
| `src/decompiler/tests/high_level/stack_ops/slots.rs` | Rust | 156 | [test] |
| `src/decompiler/tests/high_level/switches.rs` | Rust | 55 | [test] |
| `src/decompiler/tests/high_level/try_blocks.rs` | Rust | 270 | [test] |
| `src/decompiler/tests/mod.rs` | Rust | 150 | [path = "csharp_coverage.rs"] |
| `src/disassembler.rs` | Rust | 158 | Stateless Atipicial VM bytecode decoder used by the decompiler and CLI. |
| `src/disassembler/operand.rs` | Rust | 159 | — |
| `src/disassembler/operand/immediates.rs` | Rust | 28 | — |
| `src/disassembler/tests.rs` | Rust | 198 | [test] |
| `src/error.rs` | Rust | 41 | Error types returned by the library. |
| `src/error/aef.rs` | Rust | 141 | Errors returned while parsing AEF containers. |
| `src/error/disassembly.rs` | Rust | 31 | Errors returned during bytecode disassembly. |
| `src/error/manifest.rs` | Rust | 41 | Errors returned while parsing Atipicial manifest files. |
| `src/instruction.rs` | Rust | 12 | Atipicial VM instruction and operand types. |
| `src/instruction/model.rs` | Rust | 38 | A decoded Atipicial VM instruction with its bytecode offset. |
| `src/instruction/opcode.rs` | Rust | 72 | Metadata describing how to decode operands for a specific opcode. |
| `src/instruction/operand.rs` | Rust | 65 | Instruction operands supported by the disassembler. |
| `src/lib.rs` | Rust | 47 | Atipicial AEF inspection, disassembly, and decompilation tooling. |
| `src/main.rs` | Rust | 12 | ![forbid(unsafe_code)] |
| `src/manifest.rs` | Rust | 16 | Atipicial contract manifest parsing and helpers. |
| `src/manifest/describe.rs` | Rust | 216 | [must_use] |
| `src/manifest/model.rs` | Rust | 9 | — |
| `src/manifest/model/abi.rs` | Rust | 58 | ABI section describing contract methods and events. |
| `src/manifest/model/contract.rs` | Rust | 48 | Representation of a Atipicial contract manifest (`.manifest.json`). |
| `src/manifest/model/permissions.rs` | Rust | 99 | Manifest permission entry restricting contract/method calls. |
| `src/manifest/model/trusts.rs` | Rust | 14 | Manifest trust configuration controlling which contracts are trusted. |
| `src/manifest/parse.rs` | Rust | 160 | — |
| `src/manifest/tests.rs` | Rust | 275 | [test] |
| `src/native_contracts.rs` | Rust | 126 | Lookup information for Atipicial native contracts. |
| `src/native_contracts/tests.rs` | Rust | 101 | [test] |
| `src/native_contracts_generated.rs` | Rust | 75 | — |
| `src/opcodes_generated.rs` | Rust | 1012 | [allow(non_camel_case_types)] |
| `src/syscalls.rs` | Rust | 108 | Lookup table for Atipicial syscall metadata. |
| `src/syscalls_generated.rs` | Rust | 56 | — |
| `src/util.rs` | Rust | 46 | Write the provided bytes as uppercase hexadecimal into the supplied formatter. |
| `src/util/tests.rs` | Rust | 32 | [test] |
| `src/web.rs` | Rust | 297 | Browser-friendly reports and optional WebAssembly bindings. |
| `src/web/report.rs` | Rust | 500 | Browser-friendly summary of `info` output. |

### `tests/` — 30 files · 6,525 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `tests/cli_smoke.rs` | Rust | 25 | ![cfg(feature = "cli")] |
| `tests/cli_smoke/catalog.rs` | Rust | 90 | [test] |
| `tests/cli_smoke/cfg.rs` | Rust | 44 | [test] |
| `tests/cli_smoke/common.rs` | Rust | 187 | [derive(Debug, Clone, Copy)] |
| `tests/cli_smoke/decompile.rs` | Rust | 373 | [test] |
| `tests/cli_smoke/disasm.rs` | Rust | 79 | [test] |
| `tests/cli_smoke/info.rs` | Rust | 187 | [test] |
| `tests/cli_smoke/schema.rs` | Rust | 142 | [test] |
| `tests/cli_smoke/tokens.rs` | Rust | 87 | [test] |
| `tests/corpus_replay.rs` | Rust | 252 | Corpus replay / regression test. |
| `tests/csharp_compile.rs` | Rust | 354 | ![allow(clippy::unwrap_used)] |
| `tests/decompile_artifacts.rs` | Rust | 23 | [path = "decompile_artifacts/common.rs"] |
| `tests/decompile_artifacts/artifact.rs` | Rust | 85 | [derive(PartialEq, Eq)] |
| `tests/decompile_artifacts/common.rs` | Rust | 30 | — |
| `tests/decompile_artifacts/csharp_embed.rs` | Rust | 15 | — |
| `tests/decompile_artifacts/expected_failures.rs` | Rust | 92 | [derive(Debug, Clone, PartialEq, Eq)] |
| `tests/decompile_artifacts/loop_parity.rs` | Rust | 106 | [test] |
| `tests/decompile_artifacts/parity.rs` | Rust | 28 | [path = "parity/calls_helpers.rs"] |
| `tests/decompile_artifacts/parity/calls_helpers.rs` | Rust | 301 | [test] |
| `tests/decompile_artifacts/parity/contracts.rs` | Rust | 217 | [test] |
| `tests/decompile_artifacts/parity/properties.rs` | Rust | 318 | [test] |
| `tests/decompile_artifacts/parity/recursion.rs` | Rust | 199 | [test] |
| `tests/decompile_artifacts/parity/stack_shapes.rs` | Rust | 443 | [test] |
| `tests/decompile_artifacts/parity/switches.rs` | Rust | 279 | [test] |
| `tests/decompile_artifacts/process.rs` | Rust | 179 | — |
| `tests/decompile_artifacts/test.rs` | Rust | 176 | [test] |
| `tests/ir_pipeline.rs` | Rust | 1592 | End-to-end validation of the IR-spine pipeline (`--format ir`) on real bytecode. |
| `tests/ssa_e2e.rs` | Rust | 159 | End-to-end coverage for real stack-effect SSA via `Decompilation::compute_ssa`. |
| `tests/typed_declarations.rs` | Rust | 157 | End-to-end coverage for `Decompiler::with_typed_declarations`. |
| `tests/web_api.rs` | Rust | 306 | ![cfg(feature = "web")] |

### `tools/` — 6 files · 1,539 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `tools/batch_decompile.py` | Python | 217 | !/usr/bin/env python3 |
| `tools/extract_devpack_artifacts.py` | Python | 329 | !/usr/bin/env python3 |
| `tools/generate_opcodes.py` | Python | 207 | !/usr/bin/env python3 |
| `tools/scrape_native_contracts.py` | Python | 362 | !/usr/bin/env python3 |
| `tools/scrape_syscalls.py` | Python | 252 | !/usr/bin/env python3 |
| `tools/tests/extract_devpack_artifacts_tests.py` | Python | 172 | — |

### `web/` — 1 files · 404 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `web/src/index.ts` | TypeScript | 404 | — |

---

## 🧬 Public API Surface — Every Exported Symbol

The complete inventory of public symbols this project exposes. 
**420 exported symbols across 72 files.**

This is the contract surface other Atipicial components build against.

**`js/src/index.d.ts`** (77 symbols): `AtipicialDecompilerError`, `AefParseError`, `DisassemblyError`, `ManifestParseError`, `OperandEncoding`, `OpCode`, `Operand`, `Instruction`, `MethodToken`, `AefHeader`, `AefFile`, `ManifestParameter`, `ManifestMethod`, `ManifestEvent`, `ManifestAbi`, `ManifestFeatures`, `ManifestGroup`, `ManifestPermission`, `PermissionContractClassification`, `classifyPermissionContract`, `ManifestTrusts`, `ContractManifest`, `MethodGroup`, `MethodRef`, `InternalCallTarget`, `SyscallCallTarget`, `MethodTokenCallTarget`, `IndirectCallTarget`, `UnresolvedInternalCallTarget`, `CallTarget`, `CallEdge`, `CallGraph`, `WireInternalCallTarget`, `WireSyscallCallTarget`, `WireMethodTokenCallTarget`, `WireIndirectCallTarget`, `WireUnresolvedInternalCallTarget`, `WireCallTarget`, `WireCallEdge`, `WireCallGraph`, `toWireCallGraph`, `ReturnBehavior`, `MethodContract`, `MethodContracts`, `SlotXref`, `MethodXrefs`, `Xrefs`, `MethodTypes`, `TypeInfo`, `PatternConfidence`, `PatternEvidence`, `PatternInfo`, `DisassemblyOptions`, `DecompileOptions`, `DisassemblyResult`, `DecompileResult`, `DecompileWithManifestResult`, `HighLevelResult`, `HighLevelWithManifestResult`, `AnalyzeResult`, `parseAef`, `disassembleScript`, `parseManifest`, `renderCSharpContract`, `decompileBytes`, `decompileBytesWithManifest`, `decompileHighLevelBytes`, `decompileHighLevelBytesWithManifest`, `analyzeBytes`, `buildCallGraph`, `buildMethodGroups`, `buildXrefs`, `inferTypes`, `identifyPatterns`, `renderPseudocode`, `renderGroupedPseudocode`, `renderHighLevelMethodGroups`

**`src/aef.rs`** (1 symbols): `MAX_AEF_FILE_SIZE`

**`src/decompiler.rs`** (4 symbols): `MAX_AEF_FILE_SIZE`, `analysis`, `cfg`, `ir`

**`src/disassembler.rs`** (8 symbols): `UnknownHandling`, `Disassembler`, `DisassemblyOutput`, `DisassemblyWarning`, `new`, `with_unknown_handling`, `disassemble`, `disassemble_with_warnings`

**`src/error.rs`** (2 symbols): `Result`, `Error`

**`src/lib.rs`** (10 symbols): `cli`, `decompiler`, `disassembler`, `error`, `instruction`, `manifest`, `native_contracts`, `aef`, `syscalls`, `web`

**`src/native_contracts.rs`** (6 symbols): `lookup`, `all`, `NativeMethodHint`, `formatted_label`, `has_exact_method`, `describe_method_token`

**`src/native_contracts_generated.rs`** (2 symbols): `NativeContractInfo`, `NATIVE_CONTRACTS`

**`src/opcodes_generated.rs`** (5 symbols): `OpCode`, `from_byte`, `byte`, `mnemonic`, `operand_encoding`

**`src/syscalls.rs`** (4 symbols): `lookup`, `returns_value`, `all`, `summarize`

**`src/syscalls_generated.rs`** (2 symbols): `SyscallInfo`, `SYSCALLS`

**`src/web.rs`** (9 symbols): `WebDisasmOptions`, `WebDecompileOptions`, `info_report`, `disasm_report`, `decompile_report`, `init_panic_hook`, `info_report_wasm`, `disasm_report_wasm`, `decompile_report_wasm`

**`src/aef/flags.rs`** (2 symbols): `call_flag_labels`, `describe_call_flags`

**`src/aef/parser.rs`** (2 symbols): `AefParser`, `new`

**`src/aef/types.rs`** (7 symbols): `AefHeader`, `MethodToken`, `AefFile`, `payload_len`, `script_hash`, `script_hash_le`, `script_hash_be`

**`src/aef/parser/checksum.rs`** (1 symbols): `calculate_checksum`

**`src/aef/parser/parse.rs`** (1 symbols): `parse`

**`src/cli/args.rs`** (1 symbols): `Cli`

**`src/cli/runner/mod.rs`** (1 symbols): `run`

**`src/decompiler/decompilation.rs`** (9 symbols): `Decompilation`, `cfg_to_dot`, `ssa`, `compute_ssa`, `optimize_ssa`, `render_optimized_ssa`, `render_structured_ir`, `ssa_stats`, `render_ssa`

**`src/decompiler/output_format.rs`** (1 symbols): `OutputFormat`

**`src/decompiler/pipeline.rs`** (12 symbols): `Decompiler`, `new`, `with_unknown_handling`, `with_inline_single_use_temps`, `with_trace_comments`, `with_typed_declarations`, `decompile_bytes`, `disassemble_bytes`, `decompile_bytes_with_manifest`, `decompile_file`, `disassemble_file`, `decompile_file_with_manifest`

**`src/decompiler/analysis/call_graph.rs`** (4 symbols): `CallTarget`, `CallEdge`, `CallGraph`, `build_call_graph`

**`src/decompiler/analysis/methods.rs`** (7 symbols): `MethodRef`, `MethodTable`, `new`, `methods`, `method_for_offset`, `resolve_internal_target`, `manifest_index_for_start`

**`src/decompiler/analysis/method_contracts.rs`** (5 symbols): `ReturnBehavior`, `MethodContract`, `MethodContracts`, `get`, `infer_method_contracts`

**`src/decompiler/analysis/mod.rs`** (5 symbols): `call_graph`, `method_contracts`, `patterns`, `types`, `xrefs`

**`src/decompiler/analysis/patterns.rs`** (4 symbols): `PatternConfidence`, `PatternEvidence`, `PatternInfo`, `identify_patterns`

**`src/decompiler/analysis/types.rs`** (4 symbols): `ValueType`, `MethodTypes`, `TypeInfo`, `infer_types`

**`src/decompiler/analysis/xrefs.rs`** (5 symbols): `SlotKind`, `SlotXref`, `MethodXrefs`, `Xrefs`, `build_xrefs`

**`src/decompiler/cfg/builder.rs`** (4 symbols): `CfgBuilder`, `new`, `with_non_returning_calls`, `build`

**`src/decompiler/cfg/mod.rs`** (2 symbols): `method_view`, `ssa`

**`src/decompiler/cfg/structure.rs`** (1 symbols): `structure`

**`src/decompiler/cfg/basic_block/block.rs`** (5 symbols): `BasicBlock`, `new`, `contains_offset`, `instruction_count`, `is_empty`

**`src/decompiler/cfg/basic_block/block_id.rs`** (4 symbols): `BlockId`, `ENTRY`, `new`, `index`

**`src/decompiler/cfg/basic_block/terminator.rs`** (4 symbols): `Terminator`, `successors`, `can_fallthrough`, `is_conditional`

**`src/decompiler/cfg/graph/core.rs`** (15 symbols): `Cfg`, `new`, `add_block`, `add_edge`, `block`, `block_mut`, `entry_block`, `blocks`, `block_count`, `edges`, `successors`, `predecessors`, `edge_kind`, `exit_blocks`, `block_at_offset`

**`src/decompiler/cfg/graph/dot.rs`** (1 symbols): `to_dot`

**`src/decompiler/cfg/graph/edge.rs`** (2 symbols): `Edge`, `EdgeKind`

**`src/decompiler/cfg/graph/reachability.rs`** (3 symbols): `reachable_blocks`, `unreachable_blocks`, `is_reachable`

**`src/decompiler/cfg/graph/traversal.rs`** (1 symbols): `reverse_postorder`

**`src/decompiler/cfg/ssa/builder.rs`** (3 symbols): `SsaBuilder`, `new`, `build`

**`src/decompiler/cfg/ssa/context.rs`** (3 symbols): `CollectionShape`, `CollectionShapeFacts`, `CollectionArgumentEffect`

**`src/decompiler/cfg/ssa/dominance.rs`** (7 symbols): `DominanceInfo`, `new`, `idom`, `children`, `dominance_frontier_vec`, `strictly_dominates`, `compute`

**`src/decompiler/cfg/ssa/form.rs`** (21 symbols): `SsaForm`, `new`, `add_block`, `block`, `blocks_iter`, `block_count`, `add_definition`, `add_use`, `uses_of`, `render`, `stats`, `SsaStats`, `SsaBlock`, `new`, `add_phi`, `add_stmt`, `is_empty`, `phi_count`, `stmt_count`, `UseSite`, `fn`

**`src/decompiler/cfg/ssa/mod.rs`** (1 symbols): `builder`

**`src/decompiler/cfg/ssa/optimize.rs`** (1 symbols): `optimize`

**`src/decompiler/cfg/ssa/to_ir.rs`** (2 symbols): `ssa_expr_to_ir`, `render_ssa_form`

**`src/decompiler/cfg/ssa/variable.rs`** (9 symbols): `SsaVariable`, `fn`, `initial`, `next`, `fn`, `PhiNode`, `fn`, `add_operand`, `operand_count`

**`src/decompiler/cfg/ssa/form/expr.rs`** (7 symbols): `SsaExpr`, `var`, `fn`, `binary`, `unary`, `call`, `unresolved_call`

**`src/decompiler/cfg/ssa/form/stmt.rs`** (9 symbols): `SsaStmt`, `assign`, `expr`, `ret`, `throw`, `abort`, `assert`, `fn`, `fn`

**`src/decompiler/ir/control_flow.rs`** (7 symbols): `ControlFlow`, `if_then`, `if_else`, `while_loop`, `do_while`, `for_loop`, `try_catch`

**`src/decompiler/ir/semantic.rs`** (4 symbols): `Intrinsic`, `display_name`, `SemanticCallTarget`, `display_name`

**`src/decompiler/ir/statement.rs`** (16 symbols): `BlockLabel`, `Stmt`, `assign`, `ret`, `ret_void`, `throw`, `abort`, `assert`, `expr`, `comment`, `Block`, `new`, `with_stmts`, `push`, `is_empty`, `len`

**`src/decompiler/ir/expression/expr.rs`** (8 symbols): `Expr`, `int`, `var`, `binary`, `unary`, `call`, `unresolved_call`, `index`

**`src/decompiler/ir/expression/literal.rs`** (1 symbols): `Literal`

**`src/decompiler/ir/expression/operators.rs`** (2 symbols): `BinOp`, `UnaryOp`

**`src/decompiler/ir/render/expr.rs`** (1 symbols): `render_expr`

**`src/decompiler/ir/render/stmt/mod.rs`** (2 symbols): `render_stmt`, `render_block`

**`src/error/aef.rs`** (1 symbols): `AefError`

**`src/error/disassembly.rs`** (1 symbols): `DisassemblyError`

**`src/error/manifest.rs`** (1 symbols): `ManifestError`

**`src/instruction/model.rs`** (2 symbols): `Instruction`, `new`

**`src/instruction/opcode.rs`** (2 symbols): `OperandEncoding`, `all_known`

**`src/instruction/operand.rs`** (1 symbols): `Operand`

**`src/manifest/describe.rs`** (3 symbols): `describe`, `describe`, `describe`

**`src/manifest/parse.rs`** (6 symbols): `from_reader`, `from_json_str`, `from_json_str_strict`, `from_bytes`, `from_file`, `from_file_strict`

**`src/manifest/model/abi.rs`** (4 symbols): `ManifestAbi`, `ManifestMethod`, `ManifestParameter`, `ManifestEvent`

**`src/manifest/model/contract.rs`** (2 symbols): `ContractManifest`, `ManifestGroup`

**`src/manifest/model/permissions.rs`** (3 symbols): `ManifestPermission`, `ManifestPermissionContract`, `ManifestPermissionMethods`

**`src/manifest/model/trusts.rs`** (1 symbols): `ManifestTrusts`

**`src/web/report.rs`** (3 symbols): `WebInfoReport`, `WebDisasmReport`, `WebDecompileReport`

**`web/src/index.ts`** (43 symbols): `WasmInitInput`, `OutputFormat`, `InfoOptions`, `DisasmOptions`, `DecompileOptions`, `NativeContractReport`, `MethodTokenReport`, `OperandValueReport`, `InstructionReport`, `GroupSummary`, `PermissionContractSummary`, `PermissionMethodsSummary`, `TrustSummary`, `ParameterSummary`, `MethodSummary`, `EventSummary`, `AbiSummary`, `PermissionSummary`, `ManifestSummary`, `MethodRef`, `CallTarget`, `CallEdge`, `CallGraph`, `ReturnBehavior`, `MethodContract`, `MethodContracts`, `SlotXref`, `MethodXrefs`, `Xrefs`, `ValueType`, `MethodTypes`, `TypeInfo`, `AnalysisReport`, `WebInfoReport`, `WebDisasmReport`, `WebDecompileReport`, `WasmBindings`, `AtipicialDecompilerClient`, `createAtipicialDecompilerClient`, `infoReport`, `disasmReport`, `decompileReport`, `initPanicHook`


---

## 🔢 Protocol Constants — The Numbers That Govern

**3 public constants define this project's behavior.**

| Constant | Value | Defined in |
|---|---|---|
| `MAX_AEF_FILE_SIZE` | `0x10_0000` | `src/aef.rs` |
| `MAX_AEF_FILE_SIZE` | `crate::aef::MAX_AEF_FILE_SIZE` | `src/decompiler.rs` |
| `ENTRY` | `BlockId(0)` | `src/decompiler/cfg/basic_block/block_id.rs` |

---

## 🧪 Test Inventory — Proof, Not Promises

**1,045 test functions across 153 files.**

**`js/src/csharp-catches.js`**: `it ,`

**`js/src/csharp-scopes.js`**: `it ,`

**`js/src/csharp-types.js`**: `it ,`

**`js/src/grouped-pseudocode.js`**: `it \n`

**`js/src/method-contracts.js`**: `it \n`

**`js/src/native-contracts.js`**: `it :`

**`js/src/postprocess.js`**: `it ;`

**`js/src/postprocess/helpers.js`**: `it ;`

**`js/src/postprocess/loop-normalization.js`**: `it ;`

**`src/syscalls.rs`**: `lookup_finds_every_syscall`, `lookup_unknown_hash_returns_none`, `returns_value_matches_table_for_known_syscalls`, `returns_value_defaults_to_true_for_unknown_syscalls`, `syscall_table_is_sorted_by_hash`, `summarize_matches_syscall_fields`

**`src/aef/tests/flags.rs`**: `describes_call_flags`, `call_flag_labels_report_individual_bits`

**`src/aef/tests/limits.rs`**: `rejects_source_too_long`, `rejects_source_length_before_allocation`, `rejects_script_too_large`, `rejects_script_length_before_allocation`, `rejects_files_larger_than_limit`

**`src/aef/tests/parse.rs`**: `parses_valid_aef`, `rejects_bad_magic`, `rejects_bad_checksum`, `rejects_truncated_checksum_instead_of_panicking`, `rejects_trailing_bytes`, `rejects_nonzero_reserved_byte`, `rejects_nonzero_reserved_word`, `rejects_oversized_u64_varint_for_source_length`, `accepts_non_canonical_varint_for_source_length`, `rejects_leading_0xff_magic`

**`src/aef/tests/method_tokens/errors.rs`**: `rejects_overlong_method_token_name`, `rejects_method_name_with_leading_underscore`, `rejects_call_flags_with_unsupported_bits`, `rejects_too_many_method_tokens`, `rejects_oversized_u64_varint_for_method_token_count`, `accepts_non_canonical_varint_for_method_token_count`, `rejects_method_name_longer_than_32_bytes`, `rejects_more_than_128_method_tokens_and_accepts_exactly_128`, `accepts_method_name_of_exactly_32_bytes`

**`src/aef/tests/method_tokens/parse.rs`**: `parses_method_tokens`

**`src/decompiler/native_method_types.rs`**: `resolves_only_hash_bound_native_signatures`, `maps_framework_string_and_collection_returns`, `maps_framework_native_contract_returns`, `maps_additional_framework_native_returns`, `distinguishes_known_native_void_methods_from_values`

**`src/decompiler/output_format.rs`**: `defaults_to_the_csharp_contract_view`

**`src/decompiler/syscall_types.rs`**: `resolves_catalog_bound_framework_returns`, `preserves_framework_scalar_and_collection_return_types`, `unknown_hashes_remain_untyped`

**`src/decompiler/analysis/methods.rs`**: `methods_iterates_spans_in_order`

**`src/decompiler/analysis/method_contracts/tests.rs`**: `infers_private_void_leaf_with_entry_arity`, `infers_fixed_struct_shape_from_all_reachable_returns`, `infers_nested_private_entry_facts_through_static_constructor_chain`, `distinguishes_shape_preserving_and_resizing_argument_effects`, `returned_argument_alias_does_not_preserve_collection_shape`, `known_zero_argument_syscall_does_not_hide_shape_preserving_receiver`, `static_and_nested_argument_aliases_remain_unknown`, `converted_argument_aliases_escape_shape_preservation`, `converted_argument_alias_passed_to_method_token_is_unknown`, `static_fact_intersection_rejects_unknown_and_conflicting_writes`, `private_entry_facts_require_every_direct_incoming_call_and_exclude_public_entries`, `infers_five_entry_arguments_for_private_memcpy_helper`, `converges_private_void_wrapper_chain_from_leaf_to_caller`, `keeps_recursive_private_method_unknown`, `keeps_mixed_return_private_method_unknown`, `keeps_private_method_without_return_unknown`, `method_token_contract_drives_private_void_inference`, `manifest_declaration_overrides_private_return_inference_and_arity`, `manifest_void_declaration_overrides_value_left_on_stack`, `offsetless_manifest_entry_uses_declared_contract`, `sorts_and_deduplicates_call_graph_methods_by_offset`, `serializes_return_behaviors_as_lowercase_strings`, `infers_non_returning_effect_through_manifest_wrapper`, `keeps_may_return_when_any_reachable_path_returns`, `get_returns_contract_at_requested_offset`, `map_projections_include_all_contracts_and_treat_unknown_as_value`

**`src/decompiler/analysis/patterns/tests.rs`**: `manifest_standard_is_high_confidence`, `weak_metadata_does_not_claim_a_standard`, `csharp_source_paths_infer_only_the_supported_target`, `unsupported_source_metadata_is_not_claimed_as_a_renderer`, `short_csharp_compiler_tags_infer_language`, `compiler_tags_require_explicit_csharp_tokens`, `backward_jump_reports_loops_pattern`, `events_manifest_reports_events_pattern_with_evidence`, `crypto_syscalls_report_signature_and_multisig_patterns`, `check_witness_reports_authorization_pattern`, `caller_and_signer_syscalls_report_context_patterns`, `storage_runtime_and_account_syscalls_report_behavior_patterns`, `wildcard_permissions_are_reported_as_behavior_evidence`, `abi_events_are_reported_as_a_contract_pattern`, `transfer_event_and_method_report_token_transfer_behavior`, `owner_and_transfer_methods_report_ownership_pattern`, `token_lifecycle_methods_report_conservative_behavior_patterns`, `royalty_info_reports_aep24_and_royalties_patterns`, `token_payment_callbacks_report_receiver_behavior_without_standard_guess`, `method_tokens_and_calls_are_reported_without_standard_guesses`, `native_oracle_method_tokens_report_oracle_behavior`, `native_contract_management_update_reports_upgradeability`, `native_role_management_method_tokens_report_role_management`, `native_policy_method_tokens_report_policy_management`, `native_method_tokens_report_fine_grained_behavior_patterns`

**`src/decompiler/cfg/method_body_tests.rs`**: `all_known_opcodes_have_an_explicit_classification`, `type_operand_opcodes_are_exact_once_tags_are_preserved`, `cat_temporaries_preserve_known_byte_container_types`, `typed_array_index_temporaries_keep_their_element_type`, `dynamic_stack_opcodes_defer_fidelity_to_literal_resolution`, `report_finish_sorts_and_deduplicates_by_diagnostic_identity`, `lowers_only_the_exact_slice_with_neutral_source_symbols`, `catch_exception_symbol_is_a_dynamic_vm_payload`, `phi_assignments_refine_common_value_types`, `phi_assignments_keep_conflicting_value_types_dynamic`, `local_and_static_assignments_refine_only_unanimous_types`, `local_assignments_with_conflicting_or_unknown_paths_stay_dynamic`, `pusha_literal_values_remain_pointer_typed_for_csharp_refinement`, `source_map_unions_offsets_for_folded_return`, `rejects_an_oversized_slice_before_cfg_construction`, `unknown_merge_value_keeps_the_method_incomplete`, `preserves_unknown_return_behavior`

**`src/decompiler/cfg/method_view.rs`**: `extract_builds_local_cfgs_and_rewrites_cross_range_jump`, `render_method_body_emits_fn_with_return_type`, `render_void_method_does_not_return_ambient_value_across_call`, `render_method_body_does_not_associate_manifest_by_name_only`

**`src/decompiler/cfg/phi_lowering.rs`**: `groups_live_phi_operands_by_incoming_edge`, `de_versioned_slot_phi_does_not_emit_identity_copies`, `fills_missing_real_predecessor_with_unknown`, `separates_virtual_entry_from_real_backedge`, `lowers_vm_null_phi_operands_to_literal_assignments`, `schedules_acyclic_parallel_copies_without_clobbering_sources`, `breaks_parallel_copy_cycle_with_one_unique_temporary`, `fresh_helper_name_avoids_lowered_source_names`

**`src/decompiler/cfg/ssa/dominance.rs`**: `test_dominance_empty_cfg`, `test_dominance_single_block`, `test_dominance_linear_chain`, `test_dominance_diamond`, `test_dominator_tree_structure`, `diamond_cfg_dominance_frontier`, `loop_cfg_dominance_frontier`, `loop_latch_is_dominated_by_header_and_header_is_a_loop_header`

**`src/decompiler/cfg/ssa/effects.rs`**: `push_opcodes_produce_one_value`, `slot_loads_push_one`, `slot_stores_pop_one`, `collection_ops_have_correct_effects`, `conditional_jumps_pop_conditions`, `reorders_and_specials_are_neutral_in_the_table`, `control_flow_is_neutral_and_calls_produce_values`

**`src/decompiler/cfg/ssa/form.rs`**: `test_ssa_form_creation`, `test_ssa_block_additions`, `test_dominance_info_empty`, `test_ssa_expr_constructors`, `test_use_site`, `test_ssa_expr_display`, `test_ssa_stmt_display`, `test_ssa_block_display`, `test_ssa_form_render`, `test_ssa_stats`, `test_ssa_expr_complex`, `phi_placement_diamond_cfg`

**`src/decompiler/cfg/ssa/mod.rs`**: `test_dominance_via_cfg`

**`src/decompiler/cfg/ssa/to_ir.rs`**: `lowers_binary_and_literal_to_ir`, `lowers_vm_null_sentinel_to_null_literal`, `semantic_call_identity_survives_ssa_to_ir`, `render_form_shows_block_header_and_assignments`, `render_phi_lists_one_operand_per_predecessor`

**`src/decompiler/cfg/ssa/variable.rs`**: `test_ssa_variable_versioning`, `test_ssa_variable_display_hides_version`, `exception_payloads_are_handler_scoped`, `test_ssa_variable_ord`, `test_phi_node_creation`, `test_phi_node_add_operands`, `test_phi_node_display`

**`src/decompiler/cfg/ssa/builder/tests/calls.rs`**: `linear_compute_produces_real_binary_expr`, `dup_creates_a_copy_definition`, `call_results_replace_pre_call_stack_values_at_ret`, `dropping_opaque_call_result_does_not_expose_pre_call_values`, `known_call_contract_preserves_stack_and_uses_source_argument_order`, `known_tail_jump_returns_resolved_call_with_source_argument_order`, `known_call_contract_emits_void_call_without_phantom_result`, `known_calla_contract_consumes_pointer_without_rendering_it_as_an_argument`, `collection_mutations_emit_ordered_effect_calls`, `collection_mutation_underflow_preserves_declared_arity`, `structured_known_syscall_value_uses_catalog_contract`, `structured_known_syscall_void_preserves_ambient_stack_value`, `structured_known_syscall_preserves_declaration_order`, `structured_syscall_fallback_keeps_missing_known_argument_visible`, `structured_syscall_fallback_unknown_hash_uses_opaque_barrier`

**`src/decompiler/cfg/ssa/builder/tests/collection_facts.rs`**: `aliased_unpack_preserves_unmodified_collection_provenance`, `slot_round_trip_preserves_unmodified_collection_provenance`, `collection_mutation_invalidates_all_alias_provenance`, `setitem_invalidates_contents_but_preserves_collection_shape`, `dynamic_pickitem_uses_a_uniform_nested_collection_shape`, `internal_call_return_facts_reach_dynamic_pickitem_unpack`, `argument_field_writes_reject_dynamic_conflicting_partial_and_overwritten_facts`, `static_alias_resize_and_unknown_call_prevent_reusing_seeded_shape`, `value_returning_collection_mutation_invalidates_all_alias_provenance`, `known_call_invalidates_collection_argument_provenance`, `collection_returning_call_does_not_preserve_argument_shape_across_alias_mutation`, `shape_preserving_internal_call_discards_contents_but_retains_arity`, `syscall_invalidates_collection_argument_provenance`, `opaque_call_invalidates_all_collection_provenance`, `internal_call_invalidates_static_collection_provenance`, `later_internal_call_does_not_retroactively_invalidate_collection_provenance`, `loop_backedge_mutation_invalidates_header_collection_provenance`

**`src/decompiler/cfg/ssa/builder/tests/control_flow.rs`**: `entry_loop_keeps_manifest_arguments_as_incoming_slots`, `inferred_entry_stack_arguments_follow_vm_order`, `entry_loop_keeps_inferred_arguments_as_incoming_stack_values`, `store_local_emits_a_slot_assignment`, `store_then_load_connects_within_a_block`, `diamond_places_a_phi_at_the_merge`, `diamond_places_a_phi_for_a_slot`, `partially_initialized_slot_merge_is_incomplete`, `initslot_seeds_locals_with_null_before_partial_assignment`, `first_static_load_establishes_snapshot_for_non_writing_branch`, `loop_phi_uses_ambient_static_value_on_preheader`, `exception_edges_supply_their_payload_at_mixed_joins`, `exceptional_finally_entry_does_not_taint_normal_return_stack`, `known_non_returning_call_does_not_produce_a_stack_value`, `dup_conditional_join_reuses_a_shorter_prefix_top_value`, `entry_loop_slot_without_virtual_initial_value_is_incomplete`

**`src/decompiler/cfg/ssa/builder/tests/dynamic_stack.rs`**: `reported_build_marks_clean_method_exact`, `reported_build_marks_literal_pack_exact`, `literal_dynamic_stack_operations_apply_exact_vm_order`, `literal_pick_creates_a_fresh_ssa_copy`, `literal_dynamic_stack_operand_resolves_through_an_ssa_copy`, `literal_dynamic_stack_operations_accept_zero_and_depth_boundary`, `literal_dynamic_stack_operations_reject_positions_beyond_depth`, `dynamic_stack_literals_must_be_nonnegative_i32_integers`, `dynamic_stack_i32_max_resolves_before_stack_bounds_check`, `runtime_variable_dynamic_stack_operands_remain_incomplete`, `literal_dynamic_stack_operations_report_unknown_selected_values`, `reported_build_keeps_dynamic_pack_incomplete`

**`src/decompiler/cfg/ssa/builder/tests/fidelity.rs`**: `reported_build_records_unresolved_call_at_the_call_site`, `reported_build_records_explicitly_unresolved_call_target`, `reported_build_records_missing_operand_metadata_at_the_instruction`, `unreachable_underflow_is_covered_without_reducing_semantic_fidelity`, `reported_build_records_unknown_value_reaching_return`, `reported_build_records_unknown_phi_reaching_return`, `reported_build_records_unknown_phi_consumed_by_resolved_call`, `reported_build_records_unknown_phi_consumed_by_drop`, `reported_build_records_fixed_reorder_underflow_at_the_instruction`, `reported_build_keeps_uncertain_syscall_overloads_conservative`, `reported_build_does_not_warn_for_exact_csharp_syscall_bindings`, `reported_build_records_unsupported_control_at_the_instruction`, `reported_build_records_stack_underflow_at_the_instruction`, `reported_build_records_slot_load_without_reaching_definition`

**`src/decompiler/cfg/ssa/builder/tests/stack_ops.rs`**: `convert_consumes_one_value`, `istype_preserves_target_tag`, `convert_and_istype_reject_any_target_tag`, `newarray_t_accepts_any_target_tag`, `newarray_t_preserves_element_type`, `pack_preserves_elements`, `pack_accepts_nonnegative_wide_literal_count`, `packstruct_preserves_elements`, `reports_only_unanimous_unmodified_collection_return_shapes`, `packmap_preserves_pairs_in_source_order`, `unpack_constant_pack_pushes_literal_count`, `unpack_constant_pack_replays_vm_element_order`, `unpack_shaped_call_result_uses_runtime_indexes_once_in_vm_order`, `mutation_invalidates_shaped_call_result_before_unpack`, `adjacent_drop_bare_throw_preserves_the_empty_stack_fault_exactly`, `drop_throw_with_an_ambient_value_keeps_the_throw_payload`, `drop_bare_throw_is_not_fused_across_a_basic_block_boundary`, `adjacent_unpack_packstruct_becomes_exact_clone_intrinsic`, `unpack_packstruct_fusion_preserves_ambient_stack_values`, `non_adjacent_unpack_packstruct_is_not_fused`, `unpack_packstruct_is_not_fused_across_basic_block_boundary`, `unpack_packstruct_fusion_preserves_source_underflow_diagnostic`, `unpack_packstruct_fusion_preserves_unknown_source_diagnostic`, `signed_wide_pushes_decode_to_decimal`, `printable_pushdata_becomes_string_literal`, `nonprintable_pushdata_remains_bytes`, `user_append_call_remains_internal_while_vm_append_is_intrinsic`, `context_free_calls_preserve_encoded_identity`

**`src/decompiler/cfg/ssa/optimize/tests.rs`**: `does_not_propagate_constant_through_a_slot_variable`, `propagates_vm_null_through_a_slot_load_alias`, `folds_constant_binary_and_propagates`, `does_not_fold_negative_integer_exponents`, `does_not_fold_i64_overflow_as_wrapping_vm_arithmetic`, `propagates_copy_chains`, `eliminates_trivial_phi`, `retargets_terminator_use_when_removing_variable_trivial_phi`, `retargets_terminator_through_variable_trivial_phi_chain`, `rewrites_expression_through_variable_trivial_phi_chain`, `rewrites_expression_before_pruning_surviving_phi_operands`, `leaves_rooted_cyclic_phis_stable`, `preserves_literal_trivial_phi_used_by_terminator`, `preserves_literal_trivial_phi_used_by_nontrivial_phi`, `removes_dead_phi_and_releases_operand_definition`, `converges_long_reverse_copy_chain_in_one_call`, `removes_dead_mutually_dependent_phi_component`, `eliminates_dead_constant_def`, `effect_statement_keeps_input_definition_live`

**`src/decompiler/cfg/structure/for_loops.rs`**: `arithmetic_update_recovers_increment_and_decrement`, `normalized_update_rejects_source_state_after_increment`

**`src/decompiler/cfg/structure/tests.rs`**: `pathological_dense_jmpif_cfg_structures_promptly`

**`src/decompiler/cfg/structure/tests_branches_loops.rs`**: `bypassable_loop_node_is_not_a_shared_merge`, `bypassable_acyclic_join_is_not_selected_as_branch_merge`, `branch_headed_loop_with_terminal_exit_is_unconditional`, `removes_unreferenced_leave_label_after_terminal_try`, `keeps_referenced_label_after_terminal_transfer`, `unreachable_goto_does_not_keep_its_label_alive`, `constant_false_continue_self_loop_recovers_do_while`, `structures_a_diamond_into_an_if_else`, `direct_branch_to_merge_copy_stays_inside_selected_arm`, `degenerate_same_target_branch_emits_one_edge_copy`, `analysis_ssa_retains_phi_while_structured_ir_lowers_it`, `inlines_branch_comparison_condition_and_does_not_duplicate_it`, `straight_line_cfg_emits_flat_block`, `structures_a_back_edge_into_a_while_loop`, `nearest_loop_diamond_merge_stays_after_both_branch_arms`, `unconditional_backedge_to_try_entry_becomes_while_true`, `nonlocal_plain_endtry_returns_from_try_entry_loop`, `structures_early_break_and_continue`, `structures_false_edge_loop_body_with_nested_break`, `promotes_explicit_induction_loop_to_for`, `promotes_compiler_copy_chain_induction_loop_to_for`, `promotes_scalar_normalized_induction_loop_to_for`

**`src/decompiler/cfg/structure/tests_do_while_switch.rs`**: `structures_a_bottom_tested_loop_into_do_while`, `do_while_phi_backedge_copy_stays_in_body`, `structures_an_equality_cascade_into_a_switch`

**`src/decompiler/cfg/structure/tests_entry_phi.rs`**: `structure_initializes_virtual_entry_phi_once`, `entry_self_loop_keeps_virtual_initialization_separate`, `structure_emits_jump_edge_copy_before_merge_body`, `adjacent_single_use_call_temp_is_returned_directly`, `unused_call_temp_is_an_expression_statement`, `missing_use_index_keeps_referenced_call_temp_assigned`, `missing_cross_block_use_index_keeps_call_temp_assigned`, `multi_use_call_temp_remains_assigned`, `named_slot_call_remains_assigned_when_unused`, `unused_non_call_temp_remains_assigned`, `call_temp_used_as_call_argument_remains_assigned`

**`src/decompiler/cfg/structure/tests_irreducible_phi.rs`**: `irreducible_region_uses_typed_labels`, `infinite_loop_phi_copies_cover_both_arms_and_backedge`, `while_phi_copies_run_in_preheader_and_latch`

**`src/decompiler/cfg/structure/tests_terminal_loops.rs`**: `promotes_terminal_return_scan_loop_to_for`, `terminal_return_scan_loop_keeps_while_with_extra_update_effect`

**`src/decompiler/cfg/structure/tests_try_regions.rs`**: `structures_a_try_entry_into_try_catch`, `direct_leave_successor_is_hoisted_as_the_branch_merge`, `try_phi_copies_stay_in_their_selected_region`, `endtry_continuation_copy_is_shared_after_all_regions`

**`src/decompiler/cfg/structure/cleanup/int_normalization.rs`**: `collapses_exact_wrapper_to_signed_mask_expression`, `leaves_partial_wrapper_untouched`, `collapses_wrappers_inside_loop_bodies`, `collapses_wrapper_when_the_normalized_value_is_returned`, `rewrites_setitem_value_after_normalization`, `preserves_normalized_value_after_slot_assignment`

**`src/decompiler/cfg/structure/cleanup/size_normalization.rs`**: `collapses_exact_size_guarded_wrapper`, `collapses_i64_size_guarded_wrapper`, `leaves_unrecognized_size_bound_untouched`

**`src/decompiler/cfg/structure/cleanup/temps.rs`**: `propagates_single_use_copy_into_next_statement`, `propagates_copy_chain_through_multiple_steps`, `keeps_multi_use_temporaries`, `does_not_propagate_across_interfering_assignment`, `propagates_into_nested_branch_of_following_statement`, `skips_propagation_into_loop_that_reassigns_free_variable`, `collapses_nested_dynamic_and_identity_casts`, `drops_casts_of_literals_to_natural_types`, `removes_dead_store_and_its_now_unused_source`, `keeps_side_effecting_rhs_as_expression_statement`, `keeps_dead_division_that_can_fault`, `keeps_dead_index_read_that_can_fault`, `static_stores_are_never_dead`, `phi_branch_merge_folds_into_conditional_expression`, `bool_false_arm_folds_into_logical_and`, `call_results_are_not_propagated`

**`src/decompiler/cfg/structure/for_loops/terminal_update.rs`**: `terminal_update_requires_a_single_induction_assignment`, `terminal_update_rejects_extra_effects_and_continue`

**`src/decompiler/cfg/tests/basic.rs`**: `empty_instructions_produces_empty_cfg`, `single_block_linear_code`, `block_contains_offset`, `block_instruction_count`, `block_id_display`

**`src/decompiler/cfg/tests/dot.rs`**: `cfg_to_dot_produces_valid_output`

**`src/decompiler/cfg/tests/jumps.rs`**: `unconditional_jump_creates_two_blocks`, `conditional_branch_creates_multiple_blocks`, `multiple_exit_blocks`, `successors_and_predecessors`, `edge_count_matches_terminators`, `long_jump_creates_blocks`

**`src/decompiler/cfg/tests/reachability.rs`**: `unreachable_blocks_detect_dead_code_after_jump`

**`src/decompiler/cfg/tests/rpo.rs`**: `reverse_postorder_visits_all_blocks`

**`src/decompiler/cfg/tests/terminators.rs`**: `throw_creates_exit_block`, `abort_creates_exit_block`, `resolved_non_returning_call_terminates_its_block`, `terminator_successors`, `terminator_properties`, `endtry_is_modeled_as_endtry_terminator`

**`src/decompiler/cfg/tests/try_blocks.rs`**: `try_entry_adds_exception_and_finally_edges`, `endtry_routes_through_finally_to_the_natural_continuation`, `nonlocal_finally_continuation_routes_to_its_explicit_target`, `nested_endtry_at_inner_resume_boundary_routes_through_outer_finally`, `nested_catch_resume_boundary_belongs_to_the_enclosing_try`, `nested_body_resume_boundary_belongs_to_the_enclosing_catch`, `triple_nested_endtry_chain_unwinds_one_parent_at_a_time`, `shared_finally_dispatches_to_each_saved_normal_continuation`, `catch_region_distinguishes_natural_and_nonlocal_endtry_targets`, `catch_endtry_defines_natural_continuation_when_try_body_throws`

**`src/decompiler/csharp/render/body.rs`**: `long_relative_call_underflow_uses_compatibility_recovery`, `non_void_partial_body_gets_fail_closed_fallthrough`

**`src/decompiler/csharp/render/body/recovery.rs`**: `compatibility_recovery_does_not_invent_zero_argument_placeholders`, `compatibility_recovery_bounds_large_placeholder_argument_lists`

**`src/decompiler/csharp/render/header/metadata.rs`**: `it , `

**`src/decompiler/csharp/render/structured/native_framework.rs`**: `maps_vm_method_casing_to_framework_spelling`, `rejects_catalog_methods_without_framework_bindings`

**`src/decompiler/csharp/render/structured/tests_events.rs`**: `manifest_event_notify_omits_only_an_exact_packed_state_temp`

**`src/decompiler/csharp/render/structured/tests_expr_collections.rs`**: `collection_intrinsics_use_the_receiver_container_type`, `ambiguous_collection_intrinsics_use_low_level_wrappers`, `indexing_intrinsics_guard_unsupported_receivers`, `byte_intrinsics_use_framework_compatible_conversions`

**`src/decompiler/csharp/render/structured/tests_expr_core.rs`**: `renders_all_expression_variants`

**`src/decompiler/csharp/render/structured/tests_expr_formatting.rs`**: `map_intrinsics_guard_known_non_map_receivers`, `unmodeled_intrinsic_uses_a_low_level_wrapper`, `renders_expression_precedence_from_structure`, `nested_predicate_calls_parenthesize_ternary_operands`, `negative_integer_literal_does_not_form_a_decrement_token`, `csharp_strings_escape_unicode_line_separators`, `typed_shift_counts_render_as_int`

**`src/decompiler/csharp/render/structured/tests_expr_operators.rs`**: `value_equality_uses_csharp_operators_only_for_known_value_types`, `logical_not_uses_vm_truthiness_for_integer_operands`, `isnull_is_false_for_non_nullable_value_types`, `isnull_preserves_ambiguous_integer_aliases`, `numeric_operators_use_vm_wrappers_for_static_any_values`, `vm_boolean_binary_operators_are_eager_only_for_known_booleans`

**`src/decompiler/csharp/render/structured/tests_expr_syscalls.rs`**: `syscall_rendering_uses_hash_identity_and_drops_display_metadata`, `compiler_debug_notify_lowers_only_proven_singleton_string_states`, `manifest_event_notify_lifts_only_an_exact_packed_state`, `typed_syscall_fallbacks_preserve_catalog_return_types`, `check_witness_requires_explicit_framework_overload_evidence`, `check_witness_uses_proven_address_types_without_redundant_casts`, `syscall_arguments_match_framework_signatures`, `syscall_metadata_is_removed_only_from_the_extra_selector_slot`, `storage_syscalls_select_overloads_from_neutral_types`, `storage_syscalls_use_validated_csharp_types_when_vm_types_are_unknown`, `every_known_syscall_has_an_explicit_csharp_policy`

**`src/decompiler/csharp/render/structured/tests_expr_types.rs`**: `resolved_internal_call_return_types_drive_expression_typing`, `resolved_boolean_internal_call_avoids_dynamic_truthiness_cast`, `proven_literal_array_indexes_preserve_selected_element_types`, `literal_array_index_provenance_stays_conservative_at_invalid_indexes`, `object_array_literal_indexes_are_runtime_typed_but_not_static_exact`, `known_native_method_tokens_drive_exact_csharp_expression_types`, `framework_native_alias_types_remain_concrete_in_expression_context`, `additional_framework_returns_remain_concrete_in_expression_context`, `native_array_returns_preserve_index_element_types`, `framework_object_members_preserve_concrete_types`, `validated_csharp_variable_types_refine_unknown_value_types`, `planned_alias_types_flow_into_member_and_index_inference`, `planned_parameter_types_flow_into_member_inference`, `known_syscalls_drive_exact_csharp_expression_types`, `proven_expression_shapes_keep_concrete_value_types`, `intrinsic_and_member_shapes_keep_concrete_value_types`

**`src/decompiler/csharp/render/structured/tests_plan_declarations.rs`**: `plans_declarations`, `for_body_definition_used_by_update_is_hoisted_to_the_loop_scope`, `missing_uninitialized_symbol_is_a_lost_stack_value`, `stack_placeholder_is_a_lost_stack_value`, `unused_local_copy_is_removed_without_dropping_its_source`, `csharp_emits_static_referenced_beyond_type_info`, `referenced_static_beyond_type_info_reserves_the_method_name`, `static_fields_reserve_contract_member_names`

**`src/decompiler/csharp/render/structured/tests_plan_methods.rs`**: `plans_overloads_and_calls_together`, `infers_concrete_return_type_for_private_literal_helper`, `infers_private_parameter_type_from_unanimous_internal_calls`, `conflicting_private_parameter_calls_remain_dynamic`, `null_checked_private_parameter_stays_dynamic`, `null_checked_private_array_parameter_uses_the_proven_reference_type`, `indexed_private_parameter_stays_dynamic`, `indexed_private_array_parameter_uses_the_proven_array_type`, `infers_exact_string_return_type_for_private_native_helper`, `unresolved_private_call_keeps_helper_return_dynamic`, `propagates_private_return_type_through_helper_chain`, `mixed_private_returns_remain_dynamic`, `plans_cross_range_tail_jump_with_detached_helper_arity`, `null_checked_value_parameters_use_dynamic_csharp_signatures`, `null_checked_local_aliases_use_dynamic_csharp_signatures`, `infers_private_helper_parameter_name_from_manifest_argument`, `conflicting_helper_parameter_name_votes_keep_placeholder`

**`src/decompiler/csharp/render/structured/tests_plan_types.rs`**: `infers_concrete_types_for_common_structured_expressions`, `symbol_aware_expression_types_cover_index_and_numeric_copies`, `framework_native_aliases_are_accepted_for_typed_declarations`, `internal_call_return_types_drive_typed_local_aliases`, `typed_array_element_types_survive_aliases_but_unknown_arrays_stay_dynamic`, `unknown_pickitem_provenance_remains_dynamic`, `repeated_concrete_definitions_keep_their_shared_csharp_type`, `repeated_conflicting_definitions_remain_dynamic`, `repeated_nullable_reference_definitions_keep_the_reference_type`, `nullable_reference_aliases_keep_the_proven_reference_type`, `nullable_reference_aliases_can_start_with_a_null_slot_type`, `nullable_typed_array_aliases_stay_dynamic_at_unknown_mutation_boundaries`, `nullable_reference_provenance_does_not_cross_boolean_type_checks`, `nullable_value_definitions_remain_dynamic`

**`src/decompiler/csharp/render/structured/tests_stmt_control.rs`**: `renders_all_control_flow_variants`, `typed_statement_termination_is_recursive`, `typed_statement_returns_follow_the_method_contract`, `typed_statement_rendering_removes_inlined_temporary_definitions`, `typed_for_rendering_preserves_the_planned_loop_scope`, `typed_expression_statements_are_compile_valid_and_effect_preserving`

**`src/decompiler/csharp/render/structured/tests_stmt_foreach.rs`**: `typed_literal_array_foreach_uses_uniform_element_type_through_aliases`, `typed_array_index_loops_render_as_foreach_when_the_index_is_private`, `typed_array_index_loops_follow_size_aliases`, `typed_array_index_loops_keep_for_when_bound_is_not_collection_size`, `typed_array_index_loops_keep_for_when_counter_escapes_after_loop`, `typed_array_index_loops_keep_for_when_extraction_temporary_escapes`, `typed_array_index_loops_keep_for_when_the_counter_escapes`, `typed_array_index_loops_keep_for_across_opaque_body_calls`

**`src/decompiler/csharp/render/structured/tests_stmt_inlining.rs`**: `inlines_only_pure_single_use_temporaries`, `observable_state_and_allocations_are_not_inlineable`, `temporary_inlining_does_not_move_throwing_expressions`, `temporary_inlining_does_not_move_wrapper_backed_predicates`, `temporary_inlining_does_not_move_casts`, `temporary_inlining_does_not_move_values_into_while_conditions`, `temporary_inlining_does_not_move_for_initializers_into_conditions`, `temporary_inlining_does_not_move_for_initializers_into_updates`, `temporary_inlining_does_not_move_values_into_do_while_conditions`, `temporary_inlining_rejects_reassigned_dependencies`, `temporary_inlining_requires_a_concrete_value_type`, `temporary_inlining_requires_definition_before_use`

**`src/decompiler/csharp/render/structured/tests_stmt_types.rs`**: `typed_statement_references_use_planned_csharp_identifiers`, `typed_boundaries_render_valid_explicit_conversions`, `runtime_typed_literal_indexes_keep_object_array_cast_boundaries`, `typed_internal_call_boundaries_use_exact_resolved_return_types`, `typed_ambient_assignments_render_boundary_conversions`, `typed_static_field_boundaries_use_contract_field_types`, `concrete_native_reference_arrays_flow_into_object_array_storage`, `known_native_void_calls_render_as_statements`, `compiler_debug_notify_omits_the_packed_state_temp`, `typed_index_definitions_ignore_stale_slot_collection_types`, `typed_index_copy_provenance_converges_independently_of_statement_order`, `typed_index_assignments_dynamicize_parameter_and_static_storage`, `hoisted_phi_declarations_are_default_initialized`, `missing_phi_definition_gets_a_conservative_default`, `typed_boundaries_bridge_incompatible_known_types`, `typed_boundaries_box_value_array_literals_into_object_arrays`

**`src/decompiler/csharp/render/structured/plan_methods/parameter_names/tests.rs`**: `hints_through_wrappers`, `rejects_placeholders_and_expressions`, `votes_conflict_on_disagreement`, `call_arity_disagreement_blocks_name_inference`

**`src/decompiler/helpers/lifted.rs`**: `memcpy_requires_five_entry_stack_arguments`

**`src/decompiler/helpers/types.rs`**: `known_kinds_normalise_regardless_of_case`, `unknown_kinds_preserve_original_case`, `inferred_csharp_types_map_correctly`

**`src/decompiler/helpers/vm_values.rs`**: `stack_item_type_tags_round_trip`

**`src/decompiler/high_level/emitter/postprocess/compound_assign.rs`**: `rewrites_simple_assignment_into_compound_form`, `does_not_rewrite_let_bindings`, `rewrites_for_header_increment_expression`

**`src/decompiler/high_level/emitter/postprocess/for_loops.rs`**: `precompute_block_ends_matches_find_block_end_simple`, `precompute_block_ends_matches_find_block_end_nested`, `precompute_block_ends_matches_find_block_end_sequential_and_else`, `precompute_block_ends_matches_find_block_end_unclosed_headers`, `precompute_block_ends_matches_find_block_end_balanced_inline_braces`

**`src/decompiler/high_level/emitter/postprocess/indexing.rs`**: `rewrite_indexing_preserves_get_token_inside_string_literal`, `rewrite_indexing_still_rewrites_real_get_operator`

**`src/decompiler/high_level/emitter/postprocess/overflow_collapse.rs`**: `collapses_unchecked_int32_add`, `collapses_checked_int32_add`, `collapses_unsigned_range_check`, `collapses_int64_range_check`, `does_not_match_unrelated_if`, `handles_negate_equality_check`, `skips_interleaved_comments`, `handles_if_else_without_comments`, `preserves_indentation`, `find_matching_brace_closes_at_combined_else_line`

**`src/decompiler/high_level/emitter/postprocess/simplify.rs`**: `invert_empty_if_else_keeps_braces_balanced`, `eliminate_identity_temps_substitutes_forward`, `eliminate_identity_temps_skips_lhs_used_before_definition`, `invert_empty_if_else_wraps_compound_condition`

**`src/decompiler/high_level/emitter/postprocess/switches.rs`**: `switch_fold_preserves_non_temp_inter_case_statement`, `switch_fold_preserves_side_effecting_temp_between_cases`, `switch_fold_still_applies_to_consecutive_cases`, `switch_fold_accepts_negated_final_equality_case`

**`src/decompiler/high_level/emitter/postprocess/else_if/tests.rs`**: `collapses_else_if_chain`, `preserves_simple_else`, `extracts_if_condition`

**`src/decompiler/high_level/emitter/postprocess/inline/condition.rs`**: `inlines_negated_loop_condition_temp`, `inlines_negated_for_condition_temp`, `still_inlines_bare_condition_temp`

**`src/decompiler/high_level/emitter/postprocess/inline/for_increment.rs`**: `does_not_inline_for_increment_temp_used_elsewhere`, `inlines_pure_single_use_for_increment_temp`

**`src/decompiler/high_level/emitter/postprocess/inline/single_use/tests.rs`**: `single_use_temp_is_inlined_into_first_use_site`, `single_use_temp_is_not_inlined_into_control_flow_conditions`, `single_use_literal_temp_is_inlined_into_control_flow_conditions`, `non_temp_identifiers_are_not_inlined`, `temp_replacement_respects_identifier_boundaries`, `chained_inline_preserves_equality_operator`

**`src/decompiler/ir/semantic.rs`**: `method_token_display_name_is_a_safe_identifier_without_losing_metadata`, `known_syscall_keeps_name_metadata_and_generic_spelling`

**`src/decompiler/ir/tests/block.rs`**: `test_block_rendering`, `test_block_empty`, `test_block_push`

**`src/decompiler/ir/tests/control_flow_rendering.rs`**: `test_if_statement_rendering`, `test_if_else_rendering`, `test_while_loop_rendering`, `test_try_catch_rendering`

**`src/decompiler/ir/tests/expression_rendering.rs`**: `test_literal_rendering`, `test_variable_rendering`, `test_binary_expression_rendering`, `test_unary_expression_rendering`, `test_call_expression_rendering`, `test_index_expression_rendering`, `test_cast_expression_rendering`, `test_array_rendering`

**`src/decompiler/ir/tests/statement_rendering.rs`**: `test_assignment_statement_rendering`, `test_return_statement_rendering`, `test_comment_rendering`

**`src/decompiler/tests/csharp_body_core.rs`**: `csharp_multimethod_uses_structured_constant_fold`, `csharp_typed_declarations_preserve_loop_counter_type`, `csharp_loopif_deversions_local_slot`, `csharp_loopif_recovers_counting_loop_without_defeated_condition`, `csharp_assert_uses_structured_body`, `csharp_separates_clr_exception_transport_from_vm_payload`, `csharp_assert_preserves_non_scalar_vm_truthiness`, `csharp_assert_message_preserves_eager_vm_message_validation`, `csharp_assert_message_helper_call_ignores_parameter_shadowing`, `csharp_assert_uses_globally_qualified_framework_intrinsic`, `csharp_assert_message_uses_globally_qualified_opcode_attribute`, `csharp_failure_statements_preserve_semantics_and_abort_warning`, `csharp_trace_failure_statements_preserve_semantics_and_abort_warning`, `csharp_trace_assert_converts_numeric_local_to_boolean`, `csharp_trace_assert_converts_named_numeric_parameter_to_boolean`, `csharp_trace_assert_prefers_emitted_parameter_name_over_raw_slot_syntax`

**`src/decompiler/tests/csharp_control_flow.rs`**: `csharp_recovers_counting_loop_from_header_init_back_edge`, `csharp_translates_switch_to_idiomatic_c_sharp`, `csharp_else_if_chain_uses_parenthesised_conditions`, `csharp_view_respects_manifest_metadata_and_parameters`, `csharp_view_escapes_all_manifest_attribute_controls`, `high_level_view_renders_manifest_groups_block`, `csharp_view_renders_non_string_scalar_extra_metadata`

**`src/decompiler/tests/csharp_coverage.rs`**: `csharp_renderer_has_no_legacy_body_dependencies`, `csharp_corpus_has_zero_structured_fallback`, `it #`, `it /`

**`src/decompiler/tests/csharp_fidelity.rs`**: `csharp_invalid_any_convert_and_istype_use_whole_method_fallback`, `csharp_unknown_source_and_unresolved_call_use_whole_method_fallback`, `csharp_detached_packstruct_helper_keeps_vm_underflow_explicit`, `csharp_typed_map_temporary_keeps_its_receiver_type`, `csharp_fallback_primary_issue_is_the_first_incomplete_issue`, `csharp_coverage_retains_same_name_overloads_at_one_offset`, `csharp_synthetic_script_entry_exposes_initslot_args_and_preserves_return`, `csharp_omits_trailing_return_in_void_methods`, `csharp_private_void_call_preserves_ambient_return_value`, `csharp_keeps_explicit_return_value_in_non_void_methods`

**`src/decompiler/tests/csharp_legacy.rs`**: `legacy_statement_to_csharp_converts_known_forms`

**`src/decompiler/tests/csharp_literals.rs`**: `legacy_statement_to_csharp_does_not_panic_on_degenerate_headers`, `csharpize_nested_helper_calls_in_cast_path_helpers`, `legacy_expression_to_csharp_preserves_multibyte_in_cat_path`, `csharp_escapes_control_chars_in_pushdata_string_literal`, `csharp_non_void_method_with_empty_body_throws_not_implemented`, `csharp_void_event_parameter_renders_as_object_not_void`, `csharp_wraps_only_oversized_integer_literals`, `csharp_renders_oversized_hex_blob_as_byte_array`, `csharp_renders_map_literal_as_collection_initializer`

**`src/decompiler/tests/csharp_metadata.rs`**: `csharp_trims_initslot_boundaries`, `csharp_multi_entry_typed_trusts_render_as_block`, `header_surfaces_aef_compiler_and_source_fields`, `csharp_header_surfaces_inferred_patterns_and_language`, `csharp_header_renders_method_tokens_block`, `csharp_header_omits_method_tokens_block_when_none`, `csharp_single_entry_typed_trusts_stay_on_one_line`, `it public static BigInteger sumFunc`, `it private static dynamic sub_0x000C`

**`src/decompiler/tests/csharp_methods.rs`**: `csharp_resolves_internal_calls_to_method_names`, `csharp_internal_call_uses_duplicate_signature_suffix`, `csharp_ambiguous_internal_call_emits_unresolved_call_helper`, `csharp_manifest_void_internal_call_is_a_statement`, `csharp_manifest_void_resolved_calla_is_a_statement`, `csharp_offsetless_manifest_void_internal_call_is_a_statement`, `csharp_offsetless_manifest_void_resolved_calla_is_a_statement`, `csharp_manifest_void_tail_call_does_not_return_ambient_stack_value`, `csharp_manifest_void_internal_call_underflow_keeps_call_visible`, `csharp_manifest_long_internal_call_underflow_keeps_call_visible`, `csharp_manifest_value_tail_call_underflow_still_returns_call`, `csharp_manifest_value_internal_call_still_produces_a_value`, `csharp_unknown_resolved_calla_still_produces_a_value`, `csharp_emits_inferred_helper_methods`, `csharp_inferred_nonvoid_helpers_do_not_emit_bare_return`, `csharp_includes_offsetless_manifest_methods_as_stubs`, `csharp_includes_manifest_events`, `csharp_disambiguates_events_against_contract_members`, `csharp_escapes_reserved_keywords`, `csharp_uses_label_style_for_transfer_placeholders`, `csharp_mismatch_offset_emits_script_entry_and_manifest_method`, `csharp_missing_manifest_offset_uses_first_method_as_entry_signature`, `it public static BigInteger helper`

**`src/decompiler/tests/csharp_packing.rs`**: `csharp_constant_pack_is_structured_while_trace_mode_selects_legacy`, `csharp_structures_pack_families_and_constant_unpack`, `csharp_structures_printable_raw_and_wide_literals`, `csharp_type_tag_operands_use_structured_renderer`, `csharp_type_tag_helper_avoids_contract_member_collisions`, `csharp_unpack_packstruct_helper_preserves_opcode_order_and_avoids_collisions`, `csharp_bare_throw_helper_preserves_the_opcode_and_avoids_collisions`, `csharp_static_initializer_keeps_adjacent_static_and_local_slot_prologues_together`, `csharp_type_tags_preserve_bytestring_and_struct_identity`

**`src/decompiler/tests/core/analysis.rs`**: `decompilation_includes_call_graph_syscalls`, `decompilation_includes_call_graph_internal_calls`, `call_graph_resolves_relative_call_from_opcode_offset`, `call_graph_out_of_range_call_target_is_unresolved`, `call_graph_out_of_range_calla_target_is_indirect`, `decompilation_includes_call_graph_method_tokens`, `decompilation_includes_indirect_calls`, `call_graph_resolves_static_pointer_initialized_after_caller`, `decompilation_resolves_pusha_calla_to_internal_call_edge`, `decompilation_resolves_local_pointer_flow_into_calla_edge`, `decompilation_resolves_local_pointer_flow_with_nop_before_calla`, `decompilation_resolves_multi_hop_local_pointer_flow_into_calla_edge`, `decompilation_does_not_resolve_local_pointer_across_method_boundary`, `type_inference_uses_manifest_parameter_types_for_offsetless_entry_method`, `type_inference_tracks_read_only_fixed_argument_slots_without_initslot`, `call_graph_attributes_helper_syscall_to_inferred_helper_method`, `call_graph_attributes_pusha_calla_helper_syscall_to_inferred_helper_method`, `call_graph_attributes_ldarg_calla_helper_syscall_to_inferred_helper_method`, `call_graph_attributes_ldloc_from_argument_calla_helper_syscall_to_inferred_helper_method`, `call_graph_resolves_nested_pusha_argument_through_calla_helper`, `call_graph_resolves_nested_pusha_argument_through_calla_helper_without_initslot`, `call_graph_resolves_two_level_nested_calla_argument_chain`, `inferred_method_starts_tolerate_malformed_tryl_operand`, `decompilation_resolves_pickitem_delegate_array_into_calla_edge`, `decompilation_resolves_pickitem_delegate_array_through_local_alias`, `decompilation_resolves_duplicated_pointer_into_calla_edge`, `decompilation_includes_slot_xrefs`, `decompilation_includes_argument_slot_xrefs`, `decompilation_includes_indexed_slot_xrefs`, `decompilation_includes_static_slot_xrefs`, `decompilation_includes_indexed_static_slot_xrefs`, `decompilation_infers_collection_types_for_locals`, `type_inference_consumes_reverseitems_operand`, `type_inference_consumes_all_memcpy_operands`, `type_inference_consumes_known_syscall_arguments`, `type_inference_does_not_reuse_values_below_a_syscall_result`, `decompilation_propagates_manifest_argument_types`, `decompilation_infers_static_slot_types`, `decompilation_infers_packmap_types`, `decompilation_infers_convert_target_types`

**`src/decompiler/tests/core/decompile.rs`**: `disassemble_bytes_returns_instruction_stream_without_rendering`, `decompile_end_to_end`, `decompile_with_manifest_produces_contract_name`, `cfg_to_dot_includes_contract_name_and_script_hash_in_label`, `decompile_lifts_indirect_calls_without_not_yet_translated_warning`, `decompile_uses_method_token_signature_for_callt_arguments_and_returns`, `restricted_native_callt_does_not_emit_a_qualified_label`, `decompile_lifts_relative_calls_without_control_flow_warning`, `decompile_resolves_relative_call_target_to_inferred_method_name`, `decompile_relative_call_passes_known_method_arguments`, `decompile_infers_entry_stack_argument_for_syscall_only_helper`, `decompile_lifts_unconditional_jumps_without_control_flow_warning`, `decompile_manifestless_entry_surfaces_initslot_args`, `decompile_known_syscall_drops_redundant_hash_comment_in_clean_mode`, `decompile_unknown_syscall_keeps_unknown_annotation`, `decompile_lifts_endtry_transfers_without_control_flow_warning`, `decompile_uses_label_style_for_unresolved_jump_targets`, `decompile_uses_label_style_for_unresolved_endtry_targets`, `decompile_calla_with_stack_setup`, `decompile_resolves_pusha_calla_to_internal_call_placeholder`, `decompile_resolves_local_pointer_flow_into_calla`, `decompile_resolves_static_pointer_flow_into_calla`, `decompile_multiple_sequential_calls`, `decompile_nested_loop_in_if`, `decompile_try_in_loop`, `decompile_nested_if_else`, `decompile_all_comparison_jumps`, `packmap_pops_key_value_pairs_and_renders_entries`, `huge_pack_count_terminates_quickly`, `huge_packmap_count_terminates_quickly`, `invalid_type_bytes_render_as_raw_hex`, `oversized_method_hits_high_level_lifting_cap`

**`src/decompiler/tests/core/entry_point.rs`**: `renames_script_entry_using_manifest_signature`, `mismatch_offset_emits_synthetic_entry_and_keeps_manifest_method`, `missing_manifest_offset_uses_first_method_as_entry_signature`, `it fn helper() -> int {`

**`src/decompiler/tests/core/identifiers.rs`**: `contract_name_is_sanitized_with_manifest`, `high_level_sanitizes_manifest_method_and_parameter_names`, `sanitize_identifier_handles_edge_cases`, `high_level_disambiguates_colliding_method_names`

**`src/decompiler/tests/core/syscalls.rs`**: `decompile_syscall_includes_human_name`, `void_syscall_does_not_push_stack_value`, `unknown_syscall_is_assumed_to_return_value`, `syscall_arguments_render_in_declaration_order`, `storage_put_arguments_render_in_pop_order`, `void_storage_syscall_is_emitted_as_statement`, `void_storage_local_syscall_is_emitted_as_statement`, `syscall_contract_call_returns_value`, `syscall_runtime_log_is_void`, `syscall_runtime_log_missing_argument_emits_warning`, `syscall_runtime_log_after_packed_store_reports_consumed_slot_context`, `syscall_check_witness_returns_value`

**`src/decompiler/tests/core/unknowns.rs`**: `untranslated_opcode_inline_comment_survives_clean_mode`, `tolerant_mode_emits_unknown_opcode`

**`src/decompiler/tests/high_level/branches.rs`**: `high_level_lifts_simple_if_block`, `high_level_closes_if_at_end`, `high_level_lifts_if_else_block`, `high_level_lifts_jmpeq_forward_branch`, `high_level_lifts_jmpif_forward_branch`, `high_level_lifts_jmpif_l_forward_branch`, `high_level_else_branch_restores_pre_branch_stack_snapshot`, `crossing_comparison_branch_does_not_emit_malformed_double_else`, `crossing_unary_branch_does_not_emit_malformed_double_else`

**`src/decompiler/tests/high_level/entry_range.rs`**: `high_level_limits_instructions_to_entry_range`, `high_level_trims_initslot_boundaries`, `high_level_private_void_call_preserves_ambient_return_value`, `it fn other`, `it \n    fn testDelegate(`

**`src/decompiler/tests/high_level/postprocess.rs`**: `strip_stack_comments_removes_swap_annotations`, `reduce_double_parens_collapses_nested_pairs`, `eliminate_dead_temps_strips_unused_arithmetic_expression`, `eliminate_dead_temps_keeps_calls_for_their_side_effects`, `eliminate_dead_temps_keeps_used_temps`, `eliminate_dead_temps_keeps_potentially_throwing_division_or_indexing`, `eliminate_fallthrough_gotos_strips_goto_followed_by_label`, `eliminate_fallthrough_gotos_strips_leave_followed_by_label`, `eliminate_fallthrough_gotos_strips_leave_through_close_braces`, `eliminate_fallthrough_gotos_keeps_leave_when_intervening_code_present`, `eliminate_fallthrough_gotos_keeps_leave_when_target_is_distant`, `rewrite_for_loops_handles_temp_increment_chain`, `rewrite_for_loops_handles_direct_increment`, `rewrite_indexing_syntax_rewrites_conditions_and_assignments`, `rewrite_switch_statements_supports_temp_case_values`, `rewrite_switch_statements_supports_string_literal_case_values`, `rewrite_switch_statements_rewrites_long_guarded_goto_chains`, `rewrite_switch_statements_rewrites_guarded_chain_with_else_embedded_default_label`, `rewrite_switch_statements_rewrites_guarded_chain_with_else_case_and_external_default_label`, `rewrite_switch_statements_flattens_else_blocks_with_nested_chains`, `rewrite_switch_statements_skips_duplicate_cases`, `rewrite_switch_statements_skips_non_literal_cases`, `rewrite_switch_statements_collapses_consecutive_standalone_ifs`, `rewrite_switch_statements_skips_two_consecutive_standalone_ifs`, `rewrite_switch_statements_skips_consecutive_ifs_with_different_scrutinee`, `rewrite_switch_keeps_if_chain_when_case_body_reassigns_scrutinee`, `rewrite_switch_folds_standalone_ifs_when_bodies_terminate`, `rewrite_header_init_loops_lifts_defeated_counting_loop`

**`src/decompiler/tests/high_level/switches.rs`**: `high_level_recovers_switch_from_equality_chain`

**`src/decompiler/tests/high_level/try_blocks.rs`**: `high_level_lifts_try_finally_blocks`, `high_level_lifts_try_catch_blocks`, `high_level_lifts_try_catch_finally_blocks`, `high_level_lifts_try_finally_with_throw_inside`, `high_level_lifts_try_catch_with_abort_in_catch`, `high_level_models_catch_entry_stack_with_exception_value`, `malformed_try_out_of_bounds_handlers_keep_braces_balanced`

**`src/decompiler/tests/high_level/loops/break_continue.rs`**: `high_level_emits_break_and_continue`

**`src/decompiler/tests/high_level/loops/do_while.rs`**: `high_level_lifts_do_while_loop`

**`src/decompiler/tests/high_level/loops/for_loop.rs`**: `high_level_lifts_for_loop`

**`src/decompiler/tests/high_level/loops/inlining.rs`**: `loop_condition_temp_is_inlined`

**`src/decompiler/tests/high_level/loops/loopif_recovery.rs`**: `high_level_loopif_recovers_counting_loop`

**`src/decompiler/tests/high_level/loops/while_loop.rs`**: `high_level_lifts_simple_while_loop`

**`src/decompiler/tests/high_level/stack_ops/collections.rs`**: `high_level_packs_literal_arrays`, `high_level_rewrites_pickitem_as_indexing`, `high_level_rewrites_setitem_as_index_assignment`, `high_level_rewrites_haskey_as_function_call`, `high_level_pickitem_inside_call_keeps_brackets_balanced`, `high_level_istype_respects_operand_tag`, `pack_literal_underflow_renders_elision_marker_not_synthetic_temps`

**`src/decompiler/tests/high_level/stack_ops/control.rs`**: `high_level_pops_assert_condition`, `high_level_abort_clears_stack`, `high_level_throw_clears_stack`

**`src/decompiler/tests/high_level/stack_ops/slots.rs`**: `high_level_lifts_local_slots`, `high_level_lifts_all_local_slot_variants`, `high_level_lifts_all_argument_slot_variants`, `high_level_lifts_indexed_local_slot`, `high_level_lifts_indexed_argument_slot`

**`src/decompiler/tests/high_level/stack_ops/manipulation/basic.rs`**: `high_level_lifts_boolean_ops`, `high_level_handles_stack_manipulation_and_unary_ops`

**`src/decompiler/tests/high_level/stack_ops/manipulation/indexed.rs`**: `high_level_pick_of_literal_skips_temp`, `high_level_pick_of_side_effecting_value_materializes_temp`, `high_level_lifts_xdrop_with_literal_index`, `high_level_pick_preserves_packed_shape_for_unpack_reverse4`

**`src/decompiler/tests/high_level/stack_ops/manipulation/reorder.rs`**: `high_level_lifts_rot_operation`, `high_level_lifts_tuck_operation`

**`src/decompiler/tests/high_level/stack_ops/manipulation/reverse.rs`**: `high_level_lifts_reverse3_operation`, `high_level_lifts_reverse4_operation`, `high_level_lifts_reversen_operation`, `high_level_unpack_of_stored_packed_value_keeps_reverse3_stack_shape`

**`src/disassembler/tests.rs`**: `decodes_simple_sequence`, `errors_on_unknown_opcode`, `permits_unknown_opcode_when_configured`, `reports_warning_for_unknown_opcode_in_tolerant_mode`, `fails_on_truncated_operand`, `decodes_calla_no_operand`, `decodes_pushdata2`, `decodes_jump_long`, `decodes_pusha_backward_offset_as_signed_i32`, `decodes_pusha_forward_offset_as_signed_i32`, `pusha_truncated_operand_returns_unexpected_eof`, `pushdata2_truncated_length_prefix_returns_unexpected_eof`, `pushdata4_truncated_length_prefix_returns_unexpected_eof`, `decodes_syscall_operand_with_name`, `pushdata4_excessive_length_returns_operand_too_large`, `pushdata4_truncated_payload_returns_unexpected_eof`

**`src/manifest/describe.rs`**: `structured_trusts_object_with_groups_only_renders_typed_list`, `structured_trusts_object_with_hashes_only_renders_typed_list`, `structured_trusts_object_with_both_keys_concatenates_in_order`, `structured_trusts_with_unknown_key_falls_back_to_raw_json`, `structured_trusts_non_string_array_falls_back_to_raw_json`, `wildcard_trusts_describes_as_star`, `empty_contracts_trusts_describes_as_empty_brackets`

**`src/manifest/tests.rs`**: `parses_manifest_json`, `manifest_from_bytes_rejects_invalid_utf8`, `manifest_from_bytes_rejects_invalid_json`, `manifest_from_bytes_rejects_oversized_payloads`, `manifest_from_json_str_rejects_oversized_payloads`, `parses_wildcard_permission_variants`, `strict_manifest_parsing_accepts_valid_sample`, `strict_manifest_parsing_rejects_non_wildcard_permission_methods`, `strict_manifest_parsing_rejects_non_wildcard_trusts_string`, `classifies_official_string_permission_descriptors`, `strict_manifest_parsing_rejects_malformed_permission_descriptor`, `strict_manifest_parsing_rejects_non_empty_features`

**`src/native_contracts/tests.rs`**: `describes_known_native_method`, `falls_back_to_contract_name_when_method_unknown`, `describe_method_token_prefers_exact_case_match`, `lookup_finds_every_native_contract`, `lookup_unknown_hash_returns_none`, `native_contract_table_is_sorted_by_hash`, `native_method_hint_helpers_report_expected_state`, `native_contract_catalog_includes_latest_core_contracts`, `native_contract_catalog_keeps_legacy_token_contracts`

**`src/util/tests.rs`**: `writes_upper_hex`, `formats_hashes_in_both_endianness`, `computes_hash160_little_endian`

**`tests/corpus_replay.rs`**: `replay_all_fuzz_corpora_without_panics`, `decompile_all_artifacts_across_formats_without_panics`, `aef_parser_corpus_smoke`

**`tests/csharp_compile.rs`**: `representative_generated_csharp_compiles_with_roslyn`, `pinned_corpus_generated_csharp_compiles_with_roslyn`

**`tests/ir_pipeline.rs`**: `structured_ir_decodes_signed_wide_integer`, `structured_ir_recovers_pack_families_and_constant_unpack`, `structured_ir_preserves_setitem_mutation`, `structured_ir_preserves_assert_and_message`, `structured_ir_routes_natural_endtry_through_finally`, `structured_ir_routes_return_through_nested_finally`, `structured_ir_distinguishes_throw_abort_and_abort_message`, `structured_ir_seeds_catch_exception_value`, `structured_ir_keeps_value_shared_by_assert_and_branch_defined`, `structured_ir_keeps_assert_in_do_while_latch`, `structured_ir_rechecks_assert_in_while_header`, `structured_ir_keeps_assert_in_switch_comparison_block`, `ir_pipeline_recovers_a_switch_from_real_bytecode`, `ir_pipeline_recovers_an_if_from_a_real_artifact`, `ir_pipeline_is_well_formed_across_artifacts`, `ir_pipeline_renders_per_method_envelope_for_multimethod`, `ir_pipeline_loopif_envelope_preserves_while_loop`, `structured_ir_deversions_source_slots_before_phi_lowering`, `ir_pipeline_recovers_loopif_counting_loop`, `structured_ir_uses_manifest_parameter_names_in_signature_and_body`, `structured_ir_disambiguates_parameter_from_generated_ssa_name`, `structured_ir_keeps_recursive_private_return_contract_unknown`, `structured_ir_uses_resolved_internal_call_contract`, `structured_ir_applies_inferred_contract_to_helper_definition`, `structured_ir_infers_private_void_helper_without_phantom_result`, `structured_ir_infers_void_return_through_private_wrapper_chain`, `structured_ir_return_inference_keeps_unconsumed_entry_argument`, `structured_ir_seeds_inferred_entry_stack_arguments`, `structured_ir_preserves_offsetless_entry_arity_for_recursive_call`, `structured_ir_keeps_resolved_void_call_as_statement`, `structured_ir_keeps_resolved_void_call_before_branch`, `structured_ir_uses_manifest_argument_as_branch_condition`, `structured_ir_keeps_value_call_used_as_branch_condition`, `structured_ir_sanitizes_method_token_name`, `structured_ir_uses_unique_labels_for_colliding_manifest_names`, `structured_ir_uses_method_token_call_contract`, `structured_ir_renders_known_syscall_value`, `structured_ir_elides_known_syscall_temp_when_value_is_dropped`, `structured_ir_renders_known_syscall_void_as_statement`, `structured_ir_removes_pointer_materialization_for_resolved_calla`, `structured_ir_pusha_uses_absolute_target_offset`, `structured_ir_defines_stack_phi_before_resolved_call`, `structured_ir_stack_phi_preserves_short_path_underflow`, `structured_ir_distinguishes_user_append_from_vm_append`

**`tests/ssa_e2e.rs`**: `compute_ssa_produces_real_definitions_on_a_real_contract`, `compute_ssa_stats_report_statements_and_blocks`, `compute_ssa_is_idempotent`, `optimize_ssa_runs_without_panicking_and_keeps_form_consistent`, `render_optimized_ssa_produces_readable_block_text`, `render_structured_ir_emits_well_formed_output`

**`tests/typed_declarations.rs`**: `typed_declarations_annotate_inferred_integer_locals`, `typed_declarations_produce_valid_empty_type_fallback`, `typed_declarations_off_matches_default`, `typed_declarations_emit_static_slots_once_at_class_scope`

**`tests/web_api.rs`**: `web_info_report_exposes_hashes_and_manifest_summary`, `web_info_report_surfaces_manifest_extra_metadata`, `web_disasm_report_surfaces_unknown_opcode_warnings`, `web_decompile_report_exposes_high_level_and_csharp_outputs`, `web_decompile_report_exposes_private_void_method_contract`, `web_decompile_report_emit_trace_comments_re_enables_per_instruction_comments`, `web_decompile_report_defaults_to_csharp_output`

**`tests/cli_smoke/catalog.rs`**: `catalog_command_lists_syscalls`, `catalog_command_supports_syscall_json_output`, `catalog_command_supports_native_contract_json_output`, `catalog_command_lists_opcodes`, `catalog_command_supports_opcode_json_output`

**`tests/cli_smoke/cfg.rs`**: `cfg_command_outputs_dot`, `cfg_can_fail_on_unknown_opcodes`

**`tests/cli_smoke/decompile.rs`**: `decompile_command_outputs_csharp_by_default`, `decompile_command_uses_typed_csharp_declarations_by_default`, `decompile_command_accepts_inline_single_use_temps_flag`, `decompile_trace_comments_flag_re_enables_per_instruction_comments`, `decompile_no_inline_temps_keeps_let_t_lines_visible`, `decompile_command_supports_pseudocode_format`, `decompile_can_fail_on_unknown_opcodes`, `decompile_command_supports_csharp_format`, `decompile_output_format_accepts_csharp_and_legacy_alias`, `decompile_command_supports_json_format`, `decompile_command_uses_manifest_when_provided`, `decompile_command_strict_manifest_rejects_invalid_manifest_values`, `decompile_renders_requested_format_even_when_output_format_excludes_it`

**`tests/cli_smoke/disasm.rs`**: `disasm_command_outputs_instructions`, `disasm_can_fail_on_unknown_opcodes`

**`tests/cli_smoke/info.rs`**: `info_command_prints_header`, `info_command_supports_json_output`, `info_command_loads_manifest_when_available`, `info_command_rejects_large_aef`, `info_command_strict_manifest_rejects_invalid_manifest_values`

**`tests/cli_smoke/schema.rs`**: `schema_command_outputs_embedded_schema`

**`tests/cli_smoke/tokens.rs`**: `tokens_command_lists_entries`, `tokens_command_supports_json_output`, `tokens_command_handles_empty`, `tokens_command_rejects_large_aef`

**`tests/decompile_artifacts/expected_failures.rs`**: `registry_parser_sorts_deduplicates_and_preserves_expected_messages`, `it #`, `it /`

**`tests/decompile_artifacts/loop_parity.rs`**: `lambda_and_linq_scan_helpers_use_csharp_for_loops`

**`tests/decompile_artifacts/test.rs`**: `decompile_testing_artifacts_into_folder`

**`tests/decompile_artifacts/parity/calls_helpers.rs`**: `delegate_manifest_methods_do_not_swallow_private_initslot_bodies`, `inline_not_inline_case_does_not_require_spurious_call_argument`, `write_in_try_internal_calls_prefer_symbolic_targets_over_raw_offsets`, `initializer_anonymous_object_logs_use_emitted_getter_helpers_without_warnings`

**`tests/decompile_artifacts/parity/contracts.rs`**: `edgecase_csharp_output_stays_high_level`, `all_supported_artifacts_decompile_to_high_level_csharp_contracts`, `it :`

**`tests/decompile_artifacts/parity/properties.rs`**: `property_setters_without_initslot_keep_method_boundaries_and_stack_entry_args`, `null_contract_else_paths_keep_stack_shape_for_reverse4_sequences`, `nullconditional_post_ret_helpers_split_and_avoid_stack_underflow`, `property_inferred_helpers_without_initslot_receive_entry_stack_arguments`

**`tests/decompile_artifacts/parity/recursion.rs`**: `recursion_internal_calls_preserve_argument_expressions`, `recursion_even_odd_uses_branch_local_value_in_recursive_call`, `lambda_static_delegate_recursion_resolves_to_internal_calls`

**`tests/decompile_artifacts/parity/stack_shapes.rs`**: `foreach_contract_methods_use_structured_loops_without_unlifted_cfg_warnings`, `foreach_pack_helpers_do_not_emit_literal_pack_underflow_warnings`, `foreach_tuple_helper_underflow_stays_explicit_and_compile_safe`, `trycatch_handlers_do_not_underflow_on_catch_exception_slot_store`, `trycatch_contract_has_no_stack_underflow_warnings_after_catch_stack_modeling`, `aep11_balance_of_istype_and_unpack_stack_modeling_avoids_underflow_warnings`, `reentrancy_unknown_unpack_preserves_stack_for_reverse3_swap_helpers`, `tuple_unknown_unpack_preserves_stack_for_drop_stloc_drop_sequence`

**`tests/decompile_artifacts/parity/switches.rs`**: `switch_jmpif_chains_use_guarded_gotos_instead_of_invalid_nested_ifs`, `switch_inline_chain_is_rewritten_to_switch_cases`, `switch_long_guarded_goto_chain_is_rewritten_to_switch_cases`, `switch6_guarded_chain_is_rewritten_to_switch_cases`, `switch_long_long_rewrite_keeps_case_and_default_blocks_well_formed`


---

## ⚙️ Configuration & Manifest Inventory

**8 configuration files orchestrate this project.**

| File | Lines | Package | Version |
|---|---|---|---|
| `Cargo.toml` | 67 | atipicial-decompiler | 0.12.0 |
| `deny.toml` | 34 | - | - |
| `rustfmt.toml` | 8 | - | - |
| `.github/workflows/ci.yml` | 318 | wasm-bindgen | $(grep -A1 '^name =  |
| `.github/workflows/publish-web.yml` | 77 | wasm-bindgen | $(grep -A1 '^name =  |
| `fuzz/Cargo.toml` | 58 | atipicial-decompiler-fuzz | 0.0.0 |
| `js/package.json` | 48 | - | - |
| `web/package.json` | 50 | - | - |

---

## 🗺️ The Symbol Atlas — Signature by Signature

Every public symbol in the codebase with its exact Rust signature.
**279 exact signatures, copy-paste-ready.**

If it exists here, it exists in code — copy-paste-ready.


#### `src/aef.rs` — 1 symbols

```rust
pub const MAX_AEF_FILE_SIZE: u64 = 0x10_0000
```

#### `src/decompiler.rs` — 1 symbols

```rust
pub const MAX_AEF_FILE_SIZE: u64 = crate::aef::MAX_AEF_FILE_SIZE
```

#### `src/disassembler.rs` — 8 symbols

```rust
pub enum UnknownHandling
pub struct Disassembler
pub struct DisassemblyOutput
pub enum DisassemblyWarning
pub fn new() -> Self
pub fn with_unknown_handling(unknown: UnknownHandling) -> Self
pub fn disassemble(&self, bytecode: &[u8]) -> Result<Vec<Instruction>>
pub fn disassemble_with_warnings(&self, bytecode: &[u8]) -> Result<DisassemblyOutput>
```

#### `src/error.rs` — 2 symbols

```rust
pub type Result<T> = std::result::Result<T, Error>
pub enum Error
```

#### `src/native_contracts.rs` — 6 symbols

```rust
pub fn lookup(hash: &[u8
pub fn all() -> &'static [NativeContractInfo]
pub struct NativeMethodHint
pub fn formatted_label(&self, provided: &str) -> String
pub fn has_exact_method(&self) -> bool
pub fn describe_method_token(hash: &[u8
```

#### `src/native_contracts_generated.rs` — 2 symbols

```rust
pub struct NativeContractInfo
pub const NATIVE_CONTRACTS: &[NativeContractInfo] = &[
```

#### `src/opcodes_generated.rs` — 5 symbols

```rust
pub enum OpCode
pub fn from_byte(byte: u8) -> Self
pub fn byte(self) -> u8
pub fn mnemonic(self) -> &'static str
pub fn operand_encoding(self) -> OperandEncoding
```

#### `src/syscalls.rs` — 4 symbols

```rust
pub fn lookup(hash: u32) -> Option<&'static SyscallInfo>
pub fn returns_value(hash: u32) -> bool
pub fn all() -> &'static [SyscallInfo]
pub fn summarize(syscall: &SyscallInfo) -> (&'static str, &'static str, bool)
```

#### `src/syscalls_generated.rs` — 2 symbols

```rust
pub struct SyscallInfo
pub const SYSCALLS: &[SyscallInfo] = &[
```

#### `src/web.rs` — 9 symbols

```rust
pub struct WebDisasmOptions
pub struct WebDecompileOptions
pub fn info_report(aef_bytes: &[u8], manifest_json: Option<&str>) -> Result<WebInfoReport>
pub fn disasm_report(aef_bytes: &[u8], options: WebDisasmOptions) -> Result<WebDisasmReport>
pub fn decompile_report(
pub fn init_panic_hook()
pub fn info_report_wasm(
pub fn disasm_report_wasm(
pub fn decompile_report_wasm(
```

#### `src/aef/flags.rs` — 2 symbols

```rust
pub fn call_flag_labels(flags: u8) -> Vec<&'static str>
pub fn describe_call_flags(flags: u8) -> String
```

#### `src/aef/parser.rs` — 2 symbols

```rust
pub struct AefParser
pub fn new() -> Self
```

#### `src/aef/types.rs` — 7 symbols

```rust
pub struct AefHeader
pub struct MethodToken
pub struct AefFile
pub fn payload_len(&self) -> usize
pub fn script_hash(&self) -> [u8
pub fn script_hash_le(&self) -> [u8
pub fn script_hash_be(&self) -> [u8
```

#### `src/aef/parser/checksum.rs` — 1 symbols

```rust
pub fn calculate_checksum(payload: &[u8]) -> u32
```

#### `src/aef/parser/parse.rs` — 1 symbols

```rust
pub fn parse(&self, bytes: &[u8]) -> Result<AefFile>
```

#### `src/cli/args.rs` — 1 symbols

```rust
pub struct Cli
```

#### `src/cli/runner/mod.rs` — 1 symbols

```rust
pub fn run(&self) -> Result<()>
```

#### `src/decompiler/decompilation.rs` — 9 symbols

```rust
pub struct Decompilation
pub fn cfg_to_dot(&self) -> String
pub fn ssa(&self) -> Option<&SsaForm>
pub fn compute_ssa(&mut self)
pub fn optimize_ssa(&mut self) -> usize
pub fn render_optimized_ssa(&mut self) -> String
pub fn render_structured_ir(&mut self) -> String
pub fn ssa_stats(&self) -> Option<String>
pub fn render_ssa(&self) -> Option<String>
```

#### `src/decompiler/output_format.rs` — 1 symbols

```rust
pub enum OutputFormat
```

#### `src/decompiler/pipeline.rs` — 12 symbols

```rust
pub struct Decompiler
pub fn new() -> Self
pub fn with_unknown_handling(handling: UnknownHandling) -> Self
pub fn with_inline_single_use_temps(mut self, enabled: bool) -> Self
pub fn with_trace_comments(mut self, enabled: bool) -> Self
pub fn with_typed_declarations(mut self, enabled: bool) -> Self
pub fn decompile_bytes(&self, bytes: &[u8]) -> Result<Decompilation>
pub fn disassemble_bytes(&self, bytes: &[u8]) -> Result<DisassemblyOutput>
pub fn decompile_bytes_with_manifest(
pub fn decompile_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<Decompilation>
pub fn disassemble_file<P: AsRef<std::path::Path>>(
pub fn decompile_file_with_manifest<P, Q>(
```

#### `src/decompiler/analysis/call_graph.rs` — 4 symbols

```rust
pub enum CallTarget
pub struct CallEdge
pub struct CallGraph
pub fn build_call_graph(
```

#### `src/decompiler/analysis/method_contracts.rs` — 5 symbols

```rust
pub enum ReturnBehavior
pub struct MethodContract
pub struct MethodContracts
pub fn get(&self, offset: usize) -> Option<&MethodContract>
pub fn infer_method_contracts(
```

#### `src/decompiler/analysis/methods.rs` — 7 symbols

```rust
pub struct MethodRef
pub struct MethodTable
pub fn new(instructions: &[Instruction], manifest: Option<&ContractManifest>) -> Self
pub fn methods(&self) -> impl Iterator<Item = (usize, usize, &MethodRef)>
pub fn method_for_offset(&self, offset: usize) -> MethodRef
pub fn resolve_internal_target(&self, target_offset: usize) -> MethodRef
pub fn manifest_index_for_start(&self, offset: usize) -> Option<usize>
```

#### `src/decompiler/analysis/patterns.rs` — 4 symbols

```rust
pub enum PatternConfidence
pub struct PatternEvidence
pub struct PatternInfo
pub fn identify_patterns(
```

#### `src/decompiler/analysis/types.rs` — 4 symbols

```rust
pub enum ValueType
pub struct MethodTypes
pub struct TypeInfo
pub fn infer_types(instructions: &[Instruction], manifest: Option<&ContractManifest>) -> TypeInfo
```

#### `src/decompiler/analysis/xrefs.rs` — 5 symbols

```rust
pub enum SlotKind
pub struct SlotXref
pub struct MethodXrefs
pub struct Xrefs
pub fn build_xrefs(instructions: &[Instruction], manifest: Option<&ContractManifest>) -> Xrefs
```

#### `src/decompiler/cfg/builder.rs` — 4 symbols

```rust
pub struct CfgBuilder<'a>
pub fn new(instructions: &'a [Instruction]) -> Self
pub fn with_non_returning_calls(mut self, offsets: impl IntoIterator<Item = usize>) -> Self
pub fn build(mut self) -> Cfg
```

#### `src/decompiler/cfg/structure.rs` — 1 symbols

```rust
pub fn structure(ssa: &SsaForm) -> IrBlock
```

#### `src/decompiler/cfg/basic_block/block.rs` — 5 symbols

```rust
pub struct BasicBlock
pub fn new(
pub fn contains_offset(&self, offset: usize) -> bool
pub fn instruction_count(&self) -> usize
pub fn is_empty(&self) -> bool
```

#### `src/decompiler/cfg/basic_block/block_id.rs` — 4 symbols

```rust
pub struct BlockId(pub(crate) usize)
pub const ENTRY: BlockId = BlockId(0)
pub fn new(id: usize) -> Self
pub fn index(self) -> usize
```

#### `src/decompiler/cfg/basic_block/terminator.rs` — 4 symbols

```rust
pub enum Terminator
pub fn successors(&self) -> Vec<BlockId>
pub fn can_fallthrough(&self) -> bool
pub fn is_conditional(&self) -> bool
```

#### `src/decompiler/cfg/graph/core.rs` — 15 symbols

```rust
pub struct Cfg
pub fn new() -> Self
pub fn add_block(&mut self, block: BasicBlock)
pub fn add_edge(&mut self, from: BlockId, to: BlockId, kind: EdgeKind)
pub fn block(&self, id: BlockId) -> Option<&BasicBlock>
pub fn block_mut(&mut self, id: BlockId) -> Option<&mut BasicBlock>
pub fn entry_block(&self) -> Option<&BasicBlock>
pub fn blocks(&self) -> impl Iterator<Item = &BasicBlock>
pub fn block_count(&self) -> usize
pub fn edges(&self) -> &[Edge]
pub fn successors(&self, id: BlockId) -> &[BlockId]
pub fn predecessors(&self, id: BlockId) -> &[BlockId]
pub fn edge_kind(&self, from: BlockId, to: BlockId) -> Option<EdgeKind>
pub fn exit_blocks(&self) -> &BTreeSet<BlockId>
pub fn block_at_offset(&self, offset: usize) -> Option<&BasicBlock>
```

#### `src/decompiler/cfg/graph/dot.rs` — 1 symbols

```rust
pub fn to_dot(&self) -> String
```

#### `src/decompiler/cfg/graph/edge.rs` — 2 symbols

```rust
pub struct Edge
pub enum EdgeKind
```

#### `src/decompiler/cfg/graph/reachability.rs` — 3 symbols

```rust
pub fn reachable_blocks(&self) -> BTreeSet<BlockId>
pub fn unreachable_blocks(&self) -> BTreeSet<BlockId>
pub fn is_reachable(&self, id: BlockId) -> bool
```

#### `src/decompiler/cfg/graph/traversal.rs` — 1 symbols

```rust
pub fn reverse_postorder(&self) -> Vec<BlockId>
```

#### `src/decompiler/cfg/ssa/builder.rs` — 3 symbols

```rust
pub struct SsaBuilder<'a>
pub fn new(cfg: &'a Cfg, instructions: &'a [Instruction]) -> Self
pub fn build(self) -> SsaForm
```

#### `src/decompiler/cfg/ssa/context.rs` — 3 symbols

```rust
pub enum CollectionShape
pub struct CollectionShapeFacts
pub enum CollectionArgumentEffect
```

#### `src/decompiler/cfg/ssa/dominance.rs` — 7 symbols

```rust
pub struct DominanceInfo
pub fn new() -> Self
pub fn idom(&self, block: BlockId) -> Option<BlockId>
pub fn children(&self, block: BlockId) -> &[BlockId]
pub fn dominance_frontier_vec(&self, block: BlockId) -> Vec<BlockId>
pub fn strictly_dominates(&self, a: BlockId, b: BlockId) -> bool
pub fn compute(cfg: &Cfg) -> DominanceInfo
```

#### `src/decompiler/cfg/ssa/form.rs` — 21 symbols

```rust
pub struct SsaForm
pub fn new(cfg: Cfg, dominance: DominanceInfo) -> Self
pub fn add_block(&mut self, id: BlockId, block: SsaBlock)
pub fn block(&self, id: BlockId) -> Option<&SsaBlock>
pub fn blocks_iter(&self) -> impl Iterator<Item = (&BlockId, &SsaBlock)>
pub fn block_count(&self) -> usize
pub fn add_definition(&mut self, var: SsaVariable, block: BlockId)
pub fn add_use(&mut self, var: SsaVariable, site: UseSite)
pub fn uses_of(&self, var: &SsaVariable) -> Option<&BTreeSet<UseSite>>
pub fn render(&self) -> String
pub fn stats(&self) -> SsaStats
pub struct SsaStats
pub struct SsaBlock
pub fn new() -> Self
pub fn add_phi(&mut self, phi: PhiNode)
pub fn add_stmt(&mut self, stmt: SsaStmt)
pub fn is_empty(&self) -> bool
pub fn phi_count(&self) -> usize
pub fn stmt_count(&self) -> usize
pub struct UseSite
pub const fn new(block: BlockId, stmt_index: usize) -> Self
```

#### `src/decompiler/cfg/ssa/optimize.rs` — 1 symbols

```rust
pub fn optimize(ssa: &mut SsaForm) -> usize
```

#### `src/decompiler/cfg/ssa/to_ir.rs` — 2 symbols

```rust
pub fn ssa_expr_to_ir(expr: &SsaExpr) -> Expr
pub fn render_ssa_form(ssa: &SsaForm) -> String
```

#### `src/decompiler/cfg/ssa/variable.rs` — 9 symbols

```rust
pub struct SsaVariable
pub const fn new(base: String, version: usize) -> Self
pub fn initial(base: String) -> Self
pub fn next(&self) -> Self
pub const fn is_initial(&self) -> bool
pub struct PhiNode
pub const fn new(target: SsaVariable) -> Self
pub fn add_operand(&mut self, predecessor: BlockId, var: SsaVariable)
pub fn operand_count(&self) -> usize
```

#### `src/decompiler/cfg/ssa/form/expr.rs` — 7 symbols

```rust
pub enum SsaExpr
pub fn var(var: SsaVariable) -> Self
pub const fn lit(literal: Literal) -> Self
pub fn binary(op: BinOp, left: SsaExpr, right: SsaExpr) -> Self
pub fn unary(op: UnaryOp, operand: SsaExpr) -> Self
pub fn call(target: SemanticCallTarget, args: Vec<SsaExpr>) -> Self
pub fn unresolved_call(display_name: impl Into<String>, args: Vec<SsaExpr>) -> Self
```

#### `src/decompiler/cfg/ssa/form/stmt.rs` — 9 symbols

```rust
pub enum SsaStmt
pub fn assign(target: SsaVariable, value: SsaExpr) -> Self
pub fn expr(value: SsaExpr) -> Self
pub fn ret(value: Option<SsaExpr>) -> Self
pub fn throw(value: Option<SsaExpr>) -> Self
pub fn abort(message: Option<SsaExpr>) -> Self
pub fn assert(condition: SsaExpr, message: Option<SsaExpr>) -> Self
pub const fn phi(phi: PhiNode) -> Self
pub const fn other(stmt: Stmt) -> Self
```

#### `src/decompiler/ir/control_flow.rs` — 7 symbols

```rust
pub enum ControlFlow
pub fn if_then(condition: Expr, then_branch: Block) -> Self
pub fn if_else(condition: Expr, then_branch: Block, else_branch: Block) -> Self
pub fn while_loop(condition: Expr, body: Block) -> Self
pub fn do_while(body: Block, condition: Expr) -> Self
pub fn for_loop(
pub fn try_catch(
```

#### `src/decompiler/ir/semantic.rs` — 4 symbols

```rust
pub enum Intrinsic
pub fn display_name(self) -> String
pub enum SemanticCallTarget
pub fn display_name(&self) -> String
```

#### `src/decompiler/ir/statement.rs` — 16 symbols

```rust
pub struct BlockLabel(pub usize)
pub enum Stmt
pub fn assign(target: impl Into<String>, value: Expr) -> Self
pub fn ret(value: Expr) -> Self
pub fn ret_void() -> Self
pub fn throw(value: Option<Expr>) -> Self
pub fn abort(message: Option<Expr>) -> Self
pub fn assert(condition: Expr, message: Option<Expr>) -> Self
pub fn expr(e: Expr) -> Self
pub fn comment(text: impl Into<String>) -> Self
pub struct Block
pub fn new() -> Self
pub fn with_stmts(stmts: Vec<Stmt>) -> Self
pub fn push(&mut self, stmt: Stmt)
pub fn is_empty(&self) -> bool
pub fn len(&self) -> usize
```

#### `src/decompiler/ir/expression/expr.rs` — 8 symbols

```rust
pub enum Expr
pub fn int(n: i64) -> Self
pub fn var(name: impl Into<String>) -> Self
pub fn binary(op: BinOp, left: Expr, right: Expr) -> Self
pub fn unary(op: UnaryOp, operand: Expr) -> Self
pub fn call(target: SemanticCallTarget, args: Vec<Expr>) -> Self
pub fn unresolved_call(display_name: impl Into<String>, args: Vec<Expr>) -> Self
pub fn index(base: Expr, index: Expr) -> Self
```

#### `src/decompiler/ir/expression/literal.rs` — 1 symbols

```rust
pub enum Literal
```

#### `src/decompiler/ir/expression/operators.rs` — 2 symbols

```rust
pub enum BinOp
pub enum UnaryOp
```

#### `src/decompiler/ir/render/expr.rs` — 1 symbols

```rust
pub fn render_expr(expr: &Expr) -> String
```

#### `src/decompiler/ir/render/stmt/mod.rs` — 2 symbols

```rust
pub fn render_stmt(stmt: &Stmt, indent: usize) -> String
pub fn render_block(block: &Block, indent: usize) -> String
```

#### `src/error/aef.rs` — 1 symbols

```rust
pub enum AefError
```

#### `src/error/disassembly.rs` — 1 symbols

```rust
pub enum DisassemblyError
```

#### `src/error/manifest.rs` — 1 symbols

```rust
pub enum ManifestError
```

#### `src/instruction/model.rs` — 2 symbols

```rust
pub struct Instruction
pub fn new(offset: usize, opcode: OpCode, operand: Option<Operand>) -> Self
```

#### `src/instruction/opcode.rs` — 2 symbols

```rust
pub enum OperandEncoding
pub fn all_known() -> Vec<OpCode>
```

#### `src/instruction/operand.rs` — 1 symbols

```rust
pub enum Operand
```

#### `src/manifest/describe.rs` — 3 symbols

```rust
pub fn describe(&self) -> String
pub fn describe(&self) -> String
pub fn describe(&self) -> String
```

#### `src/manifest/parse.rs` — 6 symbols

```rust
pub fn from_reader<R: Read>(reader: R) -> Result<Self>
pub fn from_json_str(input: &str) -> Result<Self>
pub fn from_json_str_strict(input: &str) -> Result<Self>
pub fn from_bytes(bytes: &[u8]) -> Result<Self>
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self>
pub fn from_file_strict<P: AsRef<Path>>(path: P) -> Result<Self>
```

#### `src/manifest/model/abi.rs` — 4 symbols

```rust
pub struct ManifestAbi
pub struct ManifestMethod
pub struct ManifestParameter
pub struct ManifestEvent
```

#### `src/manifest/model/contract.rs` — 2 symbols

```rust
pub struct ContractManifest
pub struct ManifestGroup
```

#### `src/manifest/model/permissions.rs` — 3 symbols

```rust
pub struct ManifestPermission
pub enum ManifestPermissionContract
pub enum ManifestPermissionMethods
```

#### `src/manifest/model/trusts.rs` — 1 symbols

```rust
pub enum ManifestTrusts
```

#### `src/web/report.rs` — 3 symbols

```rust
pub struct WebInfoReport
pub struct WebDisasmReport
pub struct WebDecompileReport
```

---

## 🧱 Type Anatomy — Structs & Enums, Field by Field


#### `src/disassembler.rs`

**83 types dissected, field by field.**

**enum `UnknownHandling`** (2 members)
```rust
    Error
    Permit
```

**struct `Disassembler`** (1 members)
```rust
    unknown: UnknownHandling
```

**struct `DisassemblyOutput`** (2 members)
```rust
    pub instructions: Vec<Instruction>
    pub warnings: Vec<DisassemblyWarning>
```

**enum `DisassemblyWarning`** (3 members)
```rust
    UnknownOpcode {
    opcode: u8
    offset: usize
```


#### `src/error.rs`

**enum `Error`** (4 members)
```rust
    Aef(#[from] AefError)
    Disassembly(#[from] DisassemblyError)
    Io(#[from] io::Error)
    Manifest(#[from] ManifestError)
```


#### `src/native_contracts.rs`

**struct `NativeMethodHint`** (2 members)
```rust
    pub contract: &'static str
    pub canonical_method: Option<&'static str>
```


#### `src/native_contracts_generated.rs`

**struct `NativeContractInfo`** (3 members)
```rust
    pub name: &'static str
    pub script_hash: [u8; 20]
    pub methods: &'static [&'static str]
```


#### `src/opcodes_generated.rs`

**enum `OpCode`** (197 members)
```rust
    Pushint8
    Pushint16
    Pushint32
    Pushint64
    Pushint128
    Pushint256
    PushT
    PushF
    PushA
    PushNull
    Pushdata1
    Pushdata2
    Pushdata4
    PushM1
    Push0
    Push1
    Push2
    Push3
    Push4
    Push5
    Push6
    Push7
    Push8
    Push9
    Push10
    // …172 more members
```


#### `src/syscalls_generated.rs`

**struct `SyscallInfo`** (7 members)
```rust
    pub hash: u32
    pub name: &'static str
    pub handler: &'static str
    pub price: &'static str
    pub call_flags: &'static str
    pub returns_value: bool
    pub param_count: u8
```


#### `src/web.rs`

**struct `WebDisasmOptions`** (1 members)
```rust
    pub fail_on_unknown_opcodes: bool
```

**struct `WebDecompileOptions`** (7 members)
```rust
    pub manifest_json: Option<String>
    pub strict_manifest: bool
    pub fail_on_unknown_opcodes: bool
    pub inline_single_use_temps: bool
    pub emit_trace_comments: bool
    pub typed_declarations: bool
    pub output_format: OutputFormat
```


#### `src/aef/types.rs`

**struct `AefHeader`** (3 members)
```rust
    pub magic: [u8; 4]
    pub compiler: String
    pub source: String
```

**struct `MethodToken`** (5 members)
```rust
    pub hash: [u8; 20]
    pub method: String
    pub parameters_count: u16
    pub has_return_value: bool
    pub call_flags: u8
```

**struct `AefFile`** (4 members)
```rust
    pub header: AefHeader
    pub method_tokens: Vec<MethodToken>
    pub script: Vec<u8>
    pub checksum: u32
```


#### `src/cli/args.rs`

**struct `Cli`** (4 members)
```rust
    pub(super) manifest: Option<PathBuf>
    pub(super) json_compact: bool
    pub(super) strict_manifest: bool
    pub(super) command: Command
```


#### `src/decompiler/decompilation.rs`

**struct `Decompilation`** (14 members)
```rust
    pub aef: AefFile
    pub manifest: Option<ContractManifest>
    pub warnings: Vec<String>
    pub instructions: Vec<Instruction>
    pub cfg: Cfg
    pub call_graph: CallGraph
    pub method_contracts: MethodContracts
    pub patterns: PatternInfo
    pub xrefs: Xrefs
    pub types: TypeInfo
    pub pseudocode: Option<String>
    pub high_level: Option<String>
    pub csharp: Option<String>
    pub(crate) ssa: Option<SsaForm>
```


#### `src/decompiler/output_format.rs`

**enum `OutputFormat`** (4 members)
```rust
    Pseudocode
    HighLevel
    CSharp
    All
```


#### `src/decompiler/pipeline.rs`

**struct `Decompiler`** (5 members)
```rust
    parser: AefParser
    disassembler: Disassembler
    inline_single_use_temps: bool
    emit_trace_comments: bool
    typed_declarations: bool
```


#### `src/decompiler/analysis/call_graph.rs`

**enum `CallTarget`** (2 members)
```rust
    Internal {
    method: MethodRef
```

**struct `CallEdge`** (4 members)
```rust
    pub caller: MethodRef
    pub call_offset: usize
    pub opcode: String
    pub target: CallTarget
```

**struct `CallGraph`** (2 members)
```rust
    pub methods: Vec<MethodRef>
    pub edges: Vec<CallEdge>
```


#### `src/decompiler/analysis/method_contracts.rs`

**enum `ReturnBehavior`** (3 members)
```rust
    Value
    Void
    Unknown
```

**struct `MethodContract`** (9 members)
```rust
    pub method: MethodRef
    pub argument_count: usize
    pub return_behavior: ReturnBehavior
    pub may_return: bool
    pub return_shape: Option<CollectionShape>
    pub(crate) return_collection_facts: Option<CollectionShapeFacts>
    pub argument_effects: Vec<CollectionArgumentEffect>
    pub argument_collection_facts: Vec<CollectionShapeFacts>
    pub argument_field_writes: Vec<BTreeMap<usize, CollectionShape>>
```

**struct `MethodContracts`** (2 members)
```rust
    pub methods: Vec<MethodContract>
    pub static_collection_facts: BTreeMap<usize, CollectionShapeFacts>
```


#### `src/decompiler/analysis/methods.rs`

**struct `MethodRef`** (2 members)
```rust
    pub offset: usize
    pub name: String
```

**struct `MethodTable`** (2 members)
```rust
    spans: Vec<MethodSpan>
    manifest_index_by_start: BTreeMap<usize, usize>
```


#### `src/decompiler/analysis/patterns.rs`

**enum `PatternConfidence`** (4 members)
```rust
    High
    Medium
    Low
    Unknown
```

**struct `PatternEvidence`** (2 members)
```rust
    pub source: String
    pub value: String
```

**struct `PatternInfo`** (6 members)
```rust
    pub standards: Vec<String>
    pub patterns: Vec<String>
    pub language: Option<String>
    pub compiler: Option<String>
    pub confidence: PatternConfidence
    pub evidence: Vec<PatternEvidence>
```


#### `src/decompiler/analysis/types.rs`

**enum `ValueType`** (12 members)
```rust
    Unknown
    Any
    Null
    Boolean
    Integer
    ByteString
    Buffer
    Array
    Struct
    Map
    InteropInterface
    Pointer
```

**struct `MethodTypes`** (3 members)
```rust
    pub method: MethodRef
    pub arguments: Vec<ValueType>
    pub locals: Vec<ValueType>
```

**struct `TypeInfo`** (2 members)
```rust
    pub methods: Vec<MethodTypes>
    pub statics: Vec<ValueType>
```


#### `src/decompiler/analysis/xrefs.rs`

**enum `SlotKind`** (3 members)
```rust
    Local
    Argument
    Static
```

**struct `SlotXref`** (3 members)
```rust
    pub index: usize
    pub reads: Vec<usize>
    pub writes: Vec<usize>
```

**struct `MethodXrefs`** (4 members)
```rust
    pub method: MethodRef
    pub locals: Vec<SlotXref>
    pub arguments: Vec<SlotXref>
    pub statics: Vec<SlotXref>
```

**struct `Xrefs`** (1 members)
```rust
    pub methods: Vec<MethodXrefs>
```


#### `src/decompiler/cfg/builder.rs`

**struct `CfgBuilder`** (4 members)
```rust
    instructions: &'a [Instruction]
    offset_to_index: BTreeMap<usize, usize>
    leaders: BTreeSet<usize>
    non_returning_calls: BTreeSet<usize>
```


#### `src/decompiler/cfg/basic_block/block.rs`

**struct `BasicBlock`** (5 members)
```rust
    pub id: BlockId
    pub start_offset: usize
    pub end_offset: usize
    pub instruction_range: Range<usize>
    pub terminator: Terminator
```


#### `src/decompiler/cfg/basic_block/terminator.rs`

**enum `Terminator`** (2 members)
```rust
    Fallthrough {
    target: BlockId
```


#### `src/decompiler/cfg/graph/core.rs`

**struct `Cfg`** (7 members)
```rust
    pub(super) blocks: BTreeMap<BlockId, BasicBlock>
    pub(super) edges: Vec<Edge>
    pub(super) entry: BlockId
    pub(super) exits: BTreeSet<BlockId>
    pub(super) successors: BTreeMap<BlockId, Vec<BlockId>>
    pub(super) predecessors: BTreeMap<BlockId, Vec<BlockId>>
    pub(super) offset_to_block: BTreeMap<usize, BlockId>
```


#### `src/decompiler/cfg/graph/edge.rs`

**struct `Edge`** (3 members)
```rust
    pub from: BlockId
    pub to: BlockId
    pub kind: EdgeKind
```

**enum `EdgeKind`** (7 members)
```rust
    Unconditional
    ConditionalTrue
    ConditionalFalse
    Exception
    Finally
    FinallyException
    FinallyContinuation
```


#### `src/decompiler/cfg/ssa/builder.rs`

**struct `SsaBuilder`** (4 members)
```rust
    cfg: &'a Cfg
    instructions: &'a [Instruction]
    dominance: DominanceInfo
    method_context: Option<&'a MethodContext>
```


#### `src/decompiler/cfg/ssa/context.rs`

**enum `CollectionShape`** (2 members)
```rust
    Array(usize)
    Struct(usize)
```

**struct `CollectionShapeFacts`** (2 members)
```rust
    pub shape: Option<CollectionShape>
    pub indexed: BTreeMap<usize, CollectionShape>
```

**enum `CollectionArgumentEffect`** (3 members)
```rust
    Unknown
    ReadOnly
    PreservesShape
```


#### `src/decompiler/cfg/ssa/dominance.rs`

**struct `DominanceInfo`** (3 members)
```rust
    pub idom: BTreeMap<BlockId, Option<BlockId>>
    pub dominator_tree: BTreeMap<BlockId, Vec<BlockId>>
    pub dominance_frontier: BTreeMap<BlockId, BTreeSet<BlockId>>
```


#### `src/decompiler/cfg/ssa/form.rs`

**struct `SsaForm`** (5 members)
```rust
    pub cfg: Cfg
    pub dominance: DominanceInfo
    pub blocks: BTreeMap<BlockId, SsaBlock>
    pub definitions: BTreeMap<SsaVariable, BlockId>
    pub uses: BTreeMap<SsaVariable, BTreeSet<UseSite>>
```

**struct `SsaStats`** (4 members)
```rust
    pub block_count: usize
    pub total_phi_nodes: usize
    pub total_statements: usize
    pub total_variables: usize
```

**struct `SsaBlock`** (2 members)
```rust
    pub phi_nodes: Vec<PhiNode>
    pub stmts: Vec<SsaStmt>
```

**struct `UseSite`** (2 members)
```rust
    pub block: BlockId
    pub stmt_index: usize
```


#### `src/decompiler/cfg/ssa/variable.rs`

**struct `SsaVariable`** (2 members)
```rust
    pub base: String
    pub version: usize
```

**struct `PhiNode`** (2 members)
```rust
    pub target: SsaVariable
    pub operands: std::collections::BTreeMap<BlockId, SsaVariable>
```


#### `src/decompiler/cfg/ssa/form/expr.rs`

**enum `SsaExpr`** (6 members)
```rust
    Variable(SsaVariable)
    Literal(Literal)
    Binary {
    op: BinOp
    left: Box<SsaExpr>
    right: Box<SsaExpr>
```


#### `src/decompiler/cfg/ssa/form/stmt.rs`

**enum `SsaStmt`** (3 members)
```rust
    Assign {
    target: SsaVariable
    value: SsaExpr
```


#### `src/decompiler/ir/control_flow.rs`

**enum `ControlFlow`** (4 members)
```rust
    If {
    condition: Expr
    then_branch: Block
    else_branch: Option<Block>
```


#### `src/decompiler/ir/semantic.rs`

**enum `Intrinsic`** (2 members)
```rust
    Opcode(OpCode)
    UnpackPackStruct
```

**enum `SemanticCallTarget`** (3 members)
```rust
    Internal {
    offset: usize
    name: String
```


#### `src/decompiler/ir/statement.rs`

**enum `Stmt`** (1 members)
```rust
    Assign { target: String, value: Expr
```

**struct `Block`** (1 members)
```rust
    pub stmts: Vec<Stmt>
```


#### `src/decompiler/ir/expression/expr.rs`

**enum `Expr`** (7 members)
```rust
    Unknown
    Literal(Literal)
    Variable(String)
    Binary {
    op: BinOp
    left: Box<Expr>
    right: Box<Expr>
```


#### `src/decompiler/ir/expression/literal.rs`

**enum `Literal`** (6 members)
```rust
    Int(i64)
    BigInt(String)
    Bool(bool)
    String(String)
    Bytes(Vec<u8>)
    Null
```


#### `src/decompiler/ir/expression/operators.rs`

**enum `BinOp`** (19 members)
```rust
    Add
    Sub
    Mul
    Div
    Mod
    Pow
    And
    Or
    Xor
    Shl
    Shr
    Eq
    Ne
    Lt
    Le
    Gt
    Ge
    LogicalAnd
    LogicalOr
```

**enum `UnaryOp`** (7 members)
```rust
    Neg
    Not
    LogicalNot
    Inc
    Dec
    Abs
    Sign
```


#### `src/error/aef.rs`

**enum `AefError`** (1 members)
```rust
    TooShort
```


#### `src/error/disassembly.rs`

**enum `DisassemblyError`** (0 members)
```rust
```


#### `src/error/manifest.rs`

**enum `ManifestError`** (0 members)
```rust
```


#### `src/instruction/model.rs`

**struct `Instruction`** (3 members)
```rust
    pub offset: usize
    pub opcode: OpCode
    pub operand: Option<Operand>
```


#### `src/instruction/opcode.rs`

**enum `OperandEncoding`** (15 members)
```rust
    None
    I8
    I16
    I32
    I64
    Bytes(usize)
    Data1
    Data2
    Data4
    Jump8
    Jump32
    U8
    U16
    U32
    Syscall
```


#### `src/instruction/operand.rs`

**enum `Operand`** (13 members)
```rust
    I8(i8)
    I16(i16)
    I32(i32)
    I64(i64)
    Bytes(Vec<u8>)
    Jump(i8)
    Jump32(i32)
    Syscall(u32)
    U8(u8)
    U16(u16)
    U32(u32)
    Bool(bool)
    Null
```


#### `src/manifest/model/abi.rs`

**struct `ManifestAbi`** (2 members)
```rust
    pub methods: Vec<ManifestMethod>
    pub events: Vec<ManifestEvent>
```

**struct `ManifestMethod`** (5 members)
```rust
    pub name: String
    pub parameters: Vec<ManifestParameter>
    pub return_type: String
    pub offset: Option<i32>
    pub safe: bool
```

**struct `ManifestParameter`** (2 members)
```rust
    pub name: String
    pub kind: String
```

**struct `ManifestEvent`** (2 members)
```rust
    pub name: String
    pub parameters: Vec<ManifestParameter>
```


#### `src/manifest/model/contract.rs`

**struct `ContractManifest`** (8 members)
```rust
    pub name: String
    pub groups: Vec<ManifestGroup>
    pub features: serde_json::Map<String, Value>
    pub supported_standards: Vec<String>
    pub abi: ManifestAbi
    pub permissions: Vec<ManifestPermission>
    pub trusts: Option<ManifestTrusts>
    pub extra: Option<Value>
```

**struct `ManifestGroup`** (2 members)
```rust
    pub pubkey: String
    pub signature: String
```


#### `src/manifest/model/permissions.rs`

**struct `ManifestPermission`** (2 members)
```rust
    pub contract: ManifestPermissionContract
    pub methods: ManifestPermissionMethods
```

**enum `ManifestPermissionContract`** (3 members)
```rust
    Wildcard(String)
    Hash {
    hash: String
```

**enum `ManifestPermissionMethods`** (2 members)
```rust
    Wildcard(String)
    Methods(Vec<String>)
```


#### `src/manifest/model/trusts.rs`

**enum `ManifestTrusts`** (3 members)
```rust
    Wildcard(String)
    Contracts(Vec<String>)
    Other(Value)
```


#### `src/web/report.rs`

**struct `WebInfoReport`** (9 members)
```rust
    compiler: String
    source: Option<String>
    script_length: usize
    script_hash_le: String
    script_hash_be: String
    checksum: String
    method_tokens: Vec<MethodTokenReport>
    manifest: Option<ManifestSummary>
    warnings: Vec<String>
```

**struct `WebDisasmReport`** (4 members)
```rust
    script_hash_le: String
    script_hash_be: String
    instructions: Vec<InstructionReport>
    warnings: Vec<String>
```

**struct `WebDecompileReport`** (12 members)
```rust
    compiler: String
    source: Option<String>
    script_hash_le: String
    script_hash_be: String
    csharp: String
    high_level: String
    pseudocode: String
    instructions: Vec<InstructionReport>
    method_tokens: Vec<MethodTokenReport>
    manifest: Option<ManifestSummary>
    analysis: AnalysisReport
    warnings: Vec<String>
```


---

## 🚨 Error Message Atlas — Every Way This System Can Fail


**`src/error/aef.rs`**

**26 distinct error messages.**

- `file too short to contain a AEF header`
- `invalid magic bytes: expected {expected:?}, got {actual:?}`
- `checksum mismatch: expected {expected:#010x}, calculated {calculated:#010x}`
- `unexpected trailing data after checksum (extra {extra} bytes)`
- `compiler field is not valid UTF-8`
- `reserved byte at offset {offset} must be zero (found {value:#04X})`
- `reserved word at offset {offset} must be zero (found {value:#06X})`
- `unexpected end of data at offset {offset}`
- `invalid method token at index {index}`
- `varint exceeds supported range at offset {offset}`
- `varstring contains invalid utf-8 at offset {offset}`
- `script section cannot be empty`
- `source string exceeds maximum length ({length} > {max})`
- `method token count exceeds maximum ({count} > {max})`
- `script exceeds maximum size ({length} > {max})`
- `method token name {name:?} is not permitted`
- `method token call flags 0x{flags:02X} contain unsupported bits (allowed mask 0x{allowed:02X})`
- `file size {size} exceeds maximum ({max} bytes)`

**`src/error/disassembly.rs`**

- `unexpected end of bytecode at offset {offset}`
- `unknown opcode 0x{opcode:02X} at offset {offset}`
- `operand length {len} exceeds maximum at offset {offset}`

**`src/error/manifest.rs`**

- `failed to read manifest: {0}`
- `manifest size {size} exceeds maximum {max}`
- `manifest json parse error: {0}`
- `manifest contains invalid utf-8: {source}`
- `manifest validation error: {message}`

---

## ⚙️ Function Inventory — src/, Complete


**`src/decompiler.rs`** — 1 functions:
`is_exact_csharp_syscall`

**2,380 functions inventoried.**


**`src/disassembler.rs`** — 7 functions:
`fmt`, `default`, `new`, `with_unknown_handling`, `disassemble`, `disassemble_with_warnings`, `decode_known_instruction`


**`src/main.rs`** — 1 functions:
`main`


**`src/native_contracts.rs`** — 5 functions:
`lookup`, `all`, `formatted_label`, `has_exact_method`, `describe_method_token`


**`src/opcodes_generated.rs`** — 4 functions:
`from_byte`, `byte`, `mnemonic`, `operand_encoding`


**`src/syscalls.rs`** — 10 functions:
`lookup`, `returns_value`, `all`, `summarize`, `lookup_finds_every_syscall`, `lookup_unknown_hash_returns_none`, `returns_value_matches_table_for_known_syscalls`, `returns_value_defaults_to_true_for_unknown_syscalls`, `syscall_table_is_sorted_by_hash`, `summarize_matches_syscall_fields`


**`src/util.rs`** — 5 functions:
`write_upper_hex`, `upper_hex_string`, `format_hash`, `format_hash_be`, `hash160`


**`src/web.rs`** — 15 functions:
`default`, `info_report`, `disasm_report`, `decompile_report`, `parse_manifest`, `unknown_handling`, `default`, `init_panic_hook`, `report_to_js`, `info_report_wasm`, `disasm_report_wasm`, `decompile_report_wasm`, `parse_js_options`, `parse_output_format`, `to_js_error`


**`src/aef/encoding.rs`** — 8 functions:
`read_u16_le`, `read_u32_le`, `read_u64_le`, `read_varint`, `encoded_method_tokens_size`, `varint_encoded_len`, `read_varstring`, `read_varbytes`


**`src/aef/flags.rs`** — 2 functions:
`call_flag_labels`, `describe_call_flags`


**`src/aef/parser.rs`** — 1 functions:
`new`


**`src/aef/types.rs`** — 4 functions:
`payload_len`, `script_hash`, `script_hash_le`, `script_hash_be`


**`src/aef/parser/checksum.rs`** — 2 functions:
`checksum_prefix`, `calculate_checksum`


**`src/aef/parser/method_tokens.rs`** — 2 functions:
`read_u16_le`, `parse_method_tokens`


**`src/aef/parser/parse.rs`** — 1 functions:
`parse`


**`src/aef/parser/parse/header.rs`** — 3 functions:
`read_magic`, `read_compiler`, `read_source`


**`src/aef/parser/parse/reserved.rs`** — 3 functions:
`read_u16_le`, `expect_reserved_byte_zero`, `expect_reserved_word_zero`


**`src/aef/parser/parse/script.rs`** — 1 functions:
`read_script`


**`src/aef/parser/parse/trailer.rs`** — 4 functions:
`read_u32_le`, `read_checksum`, `verify_checksum`, `expect_end_of_file`


**`src/aef/tests/flags.rs`** — 2 functions:
`describes_call_flags`, `call_flag_labels_report_individual_bits`


**`src/aef/tests/limits.rs`** — 5 functions:
`rejects_source_too_long`, `rejects_source_length_before_allocation`, `rejects_script_too_large`, `rejects_script_length_before_allocation`, `rejects_files_larger_than_limit`


**`src/aef/tests/mod.rs`** — 2 functions:
`write_varint`, `build_sample`


**`src/aef/tests/parse.rs`** — 10 functions:
`parses_valid_aef`, `rejects_bad_magic`, `rejects_bad_checksum`, `rejects_truncated_checksum_instead_of_panicking`, `rejects_trailing_bytes`, `rejects_nonzero_reserved_byte`, `rejects_nonzero_reserved_word`, `rejects_oversized_u64_varint_for_source_length`, `accepts_non_canonical_varint_for_source_length`, `rejects_leading_0xff_magic`


**`src/aef/tests/method_tokens/errors.rs`** — 9 functions:
`rejects_overlong_method_token_name`, `rejects_method_name_with_leading_underscore`, `rejects_call_flags_with_unsupported_bits`, `rejects_too_many_method_tokens`, `rejects_oversized_u64_varint_for_method_token_count`, `accepts_non_canonical_varint_for_method_token_count`, `rejects_method_name_longer_than_32_bytes`, `rejects_more_than_128_method_tokens_and_accepts_exactly_128`, `accepts_method_name_of_exactly_32_bytes`


**`src/aef/tests/method_tokens/parse.rs`** — 1 functions:
`parses_method_tokens`


**`src/cli/catalog.rs`** — 3 functions:
`build_syscall_catalog_entries`, `build_native_contract_catalog_entries`, `build_opcode_catalog_entries`


**`src/cli/args/catalog.rs`** — 1 functions:
`as_str`


**`src/cli/reports/instructions.rs`** — 4 functions:
`from`, `operand_kind_name`, `operand_value_report`, `returns_value_for_instruction`


**`src/cli/reports/method_tokens.rs`** — 3 functions:
`format_method_token_line`, `build_method_token_report`, `collect_warnings`


**`src/cli/reports/manifest/build.rs`** — 1 functions:
`summarize_manifest`


**`src/cli/reports/manifest/convert.rs`** — 3 functions:
`from`, `from`, `from`


**`src/cli/runner/catalog.rs`** — 4 functions:
`run_catalog`, `print_syscall_catalog`, `print_native_contract_catalog`, `print_opcode_catalog`


**`src/cli/runner/cfg.rs`** — 1 functions:
`run_cfg`


**`src/cli/runner/common.rs`** — 9 functions:
`write_stdout`, `resolve_manifest_path`, `render_json`, `print_json`, `print_catalog_json`, `read_aef_bytes`, `unknown_handling`, `load_manifest`, `write_warnings`


**`src/cli/runner/decompile.rs`** — 1 functions:
`run_decompile`


**`src/cli/runner/disasm.rs`** — 1 functions:
`run_disasm`


**`src/cli/runner/info.rs`** — 1 functions:
`run_info`


**`src/cli/runner/mod.rs`** — 1 functions:
`run`


**`src/cli/runner/schema.rs`** — 2 functions:
`run_schema`, `validate_against_schema`


**`src/cli/runner/tokens.rs`** — 1 functions:
`run_tokens`


**`src/cli/runner/info/json.rs`** — 1 functions:
`print_info_json`


**`src/cli/runner/info/text.rs`** — 1 functions:
`print_info_text`


**`src/cli/schema/metadata.rs`** — 1 functions:
`report`


**`src/decompiler/csharp.rs`** — 1 functions:
`is_exact_syscall`


**`src/decompiler/decompilation.rs`** — 10 functions:
`cfg_to_dot`, `cfg_dot_title`, `ssa`, `compute_ssa`, `optimize_ssa`, `render_optimized_ssa`, `render_structured_ir`, `render_structured_ir_single_cfg`, `ssa_stats`, `render_ssa`


**`src/decompiler/native_method_types.rs`** — 9 functions:
`lookup`, `returns_value`, `return_type`, `parse_hash`, `resolves_only_hash_bound_native_signatures`, `maps_framework_string_and_collection_returns`, `maps_framework_native_contract_returns`, `maps_additional_framework_native_returns`, `distinguishes_known_native_void_methods_from_values`


**`src/decompiler/output_format.rs`** — 4 functions:
`wants_pseudocode`, `wants_high_level`, `wants_csharp`, `defaults_to_the_csharp_contract_view`


**`src/decompiler/pipeline.rs`** — 9 functions:
`new`, `with_unknown_handling`, `with_inline_single_use_temps`, `with_trace_comments`, `with_typed_declarations`, `decompile_bytes`, `disassemble_bytes`, `decompile_bytes_with_manifest`, `decompile_file_with_manifest`


**`src/decompiler/pseudocode.rs`** — 1 functions:
`render`


**`src/decompiler/syscall_types.rs`** — 5 functions:
`lookup`, `return_type`, `resolves_catalog_bound_framework_returns`, `preserves_framework_scalar_and_collection_return_types`, `unknown_hashes_remain_untyped`


**`src/decompiler/analysis/call_graph.rs`** — 2 functions:
`build_call_graph`, `relative_target_isize`


**`src/decompiler/analysis/method_contracts.rs`** — 4 functions:
`get`, `argument_counts_by_offset`, `returns_value_by_offset`, `infer_method_contracts`


**`src/decompiler/analysis/methods.rs`** — 11 functions:
`synthetic`, `largest_le`, `new`, `resolve_argument_target_for_method`, `spans`, `methods`, `method_for_offset`, `resolve_internal_target`, `direct_call_target`, `manifest_index_for_start`, `methods_iterates_spans_in_order`


**`src/decompiler/analysis/patterns.rs`** — 1 functions:
`identify_patterns`


**`src/decompiler/analysis/types.rs`** — 6 functions:
`join`, `fmt`, `unknown`, `with_type`, `integer_literal`, `infer_types`


**`src/decompiler/analysis/xrefs.rs`** — 5 functions:
`build_xrefs`, `slot_access`, `slot_from_operand`, `scan_slot_counts`, `scan_static_slot_count`


**`src/decompiler/analysis/call_graph/pointers.rs`** — 20 functions:
`calla_target_from_pusha`, `pusha_absolute_target`, `local_load_index`, `static_load_index`, `arg_load_index`, `slot_store_domain`, `resolve_slot_pointer_target`, `trace_pointer_target_from_value_source`, `calla_ldarg_index`, `trace_argument_index_from_value_source`, `resolve_pickitem_pointer_target`, `trace_container_domain_from_value_source`, `trace_stack_value_producer_before`, `stack_effect`, `find_resolution_start_index`, `find_slot_store_before`, `is_pointer_resolution_boundary`, `previous_non_nop_index`, `initslot_arg_count_at`, `trace_call_arg_source`


**`src/decompiler/analysis/call_graph/pointers/arguments.rs`** — 2 functions:
`resolve_ldarg_calla_targets`, `resolve_argument_target_recursive`


**`src/decompiler/analysis/call_graph/pointers/static_values.rs`** — 2 functions:
`resolve_constant_static_pointer_target`, `trace_constant_pointer_source`


**`src/decompiler/analysis/method_contracts/calls.rs`** — 5 functions:
`method_return_facts`, `method_may_return`, `method_has_only_bare_returns`, `calls_for_view`, `build_call_contracts`


**`src/decompiler/analysis/method_contracts/collection.rs`** — 8 functions:
`infer_argument_field_writes`, `infer_entry_and_static_collection_facts`, `method_collection_analysis`, `aggregate_static_collection_facts`, `intersect_static_writes`, `aggregate_private_argument_facts`, `static_load_index`, `static_store_index`


**`src/decompiler/analysis/method_contracts/tests.rs`** — 31 functions:
`manifest`, `analyze`, `analyze_with_tokens`, `standard_manifest`, `infers_private_void_leaf_with_entry_arity`, `infers_fixed_struct_shape_from_all_reachable_returns`, `infers_nested_private_entry_facts_through_static_constructor_chain`, `distinguishes_shape_preserving_and_resizing_argument_effects`, `returned_argument_alias_does_not_preserve_collection_shape`, `known_zero_argument_syscall_does_not_hide_shape_preserving_receiver`, `static_and_nested_argument_aliases_remain_unknown`, `converted_argument_aliases_escape_shape_preservation`, `converted_argument_alias_passed_to_method_token_is_unknown`, `static_fact_intersection_rejects_unknown_and_conflicting_writes`, `private_entry_facts_require_every_direct_incoming_call_and_exclude_public_entries`, `infers_five_entry_arguments_for_private_memcpy_helper`, `converges_private_void_wrapper_chain_from_leaf_to_caller`, `keeps_recursive_private_method_unknown`, `keeps_mixed_return_private_method_unknown`, `keeps_private_method_without_return_unknown`, `method_token_contract_drives_private_void_inference`, `manifest_declaration_overrides_private_return_inference_and_arity`, `manifest_void_declaration_overrides_value_left_on_stack`, `offsetless_manifest_entry_uses_declared_contract`, `sorts_and_deduplicates_call_graph_methods_by_offset`, `serializes_return_behaviors_as_lowercase_strings`, `infers_non_returning_effect_through_manifest_wrapper`, `keeps_may_return_when_any_reachable_path_returns`, `get_returns_contract_at_requested_offset`, `map_projections_include_all_contracts_and_treat_unknown_as_value`, `contract`


**`src/decompiler/analysis/method_contracts/collection/effects.rs`** — 4 functions:
`method_argument_effects`, `collect_escaping_argument_origins`, `possible_alias_source`, `collect_argument_origins`


**`src/decompiler/analysis/patterns/abi.rs`** — 1 functions:
`infer_abi_patterns`


**`src/decompiler/analysis/patterns/language.rs`** — 2 functions:
`infer_language`, `infer_language_from_source`


**`src/decompiler/analysis/patterns/native_patterns.rs`** — 3 functions:
`infer_native_patterns`, `infer_stdlib_patterns`, `add`


**`src/decompiler/analysis/patterns/syscall_patterns.rs`** — 2 functions:
`infer_syscall_patterns`, `add`


**`src/decompiler/analysis/patterns/tests.rs`** — 26 functions:
`aef`, `manifest_standard_is_high_confidence`, `weak_metadata_does_not_claim_a_standard`, `csharp_source_paths_infer_only_the_supported_target`, `unsupported_source_metadata_is_not_claimed_as_a_renderer`, `short_csharp_compiler_tags_infer_language`, `compiler_tags_require_explicit_csharp_tokens`, `backward_jump_reports_loops_pattern`, `events_manifest_reports_events_pattern_with_evidence`, `crypto_syscalls_report_signature_and_multisig_patterns`, `check_witness_reports_authorization_pattern`, `caller_and_signer_syscalls_report_context_patterns`, `storage_runtime_and_account_syscalls_report_behavior_patterns`, `wildcard_permissions_are_reported_as_behavior_evidence`, `abi_events_are_reported_as_a_contract_pattern`, `transfer_event_and_method_report_token_transfer_behavior`, `owner_and_transfer_methods_report_ownership_pattern`, `token_lifecycle_methods_report_conservative_behavior_patterns`, `royalty_info_reports_aep24_and_royalties_patterns`, `token_payment_callbacks_report_receiver_behavior_without_standard_guess`, `method_tokens_and_calls_are_reported_without_standard_guesses`, `native_oracle_method_tokens_report_oracle_behavior`, `native_contract_management_update_reports_upgradeability`, `native_role_management_method_tokens_report_role_management`, `native_policy_method_tokens_report_policy_management`, `native_method_tokens_report_fine_grained_behavior_patterns`


**`src/decompiler/analysis/types/infer.rs`** — 12 functions:
`infer_types_in_slice`, `reverse_top`, `pop_or_unknown`, `push_slot`, `push_fixed_slot`, `push_indexed_slot`, `store_slot`, `store_indexed_slot`, `scan_slot_counts`, `scan_static_slot_count`, `int_literal_from_operand`, `type_from_manifest`


**`src/decompiler/cfg/builder.rs`** — 3 functions:
`new`, `with_non_returning_calls`, `build`


**`src/decompiler/cfg/method_body.rs`** — 6 functions:
`exact`, `finish`, `primary_issue`, `lower_method_body`, `return_behavior`, `register_structured_temporaries`


**`src/decompiler/cfg/method_body_names.rs`** — 4 functions:
`collect_block_names`, `collect_statement_names`, `collect_control_names`, `collect_expr_names`


**`src/decompiler/cfg/method_body_symbols.rs`** — 7 functions:
`allocate_source_symbols`, `register_source_family`, `ssa_variables`, `slot_index`, `is_source_family`, `is_stack_phi_base`, `symbol_type`


**`src/decompiler/cfg/method_body_tests.rs`** — 18 functions:
`instruction`, `all_known_opcodes_have_an_explicit_classification`, `type_operand_opcodes_are_exact_once_tags_are_preserved`, `cat_temporaries_preserve_known_byte_container_types`, `typed_array_index_temporaries_keep_their_element_type`, `dynamic_stack_opcodes_defer_fidelity_to_literal_resolution`, `report_finish_sorts_and_deduplicates_by_diagnostic_identity`, `lowers_only_the_exact_slice_with_neutral_source_symbols`, `catch_exception_symbol_is_a_dynamic_vm_payload`, `phi_assignments_refine_common_value_types`, `phi_assignments_keep_conflicting_value_types_dynamic`, `local_and_static_assignments_refine_only_unanimous_types`, `local_assignments_with_conflicting_or_unknown_paths_stay_dynamic`, `pusha_literal_values_remain_pointer_typed_for_csharp_refinement`, `source_map_unions_offsets_for_folded_return`, `rejects_an_oversized_slice_before_cfg_construction`, `unknown_merge_value_keeps_the_method_incomplete`, `preserves_unknown_return_behavior`


**`src/decompiler/cfg/method_body_types.rs`** — 11 functions:
`intrinsic_result_type`, `merge_value_types`, `register_structured_temporaries`, `register_structured_temporaries_with_call_types`, `refine_structured_types`, `collect_definition_types`, `collect_statement_definition_types`, `collect_control_definition_types`, `widen_exception_payload_copies`, `collect_direct_copy_edges`, `structured_expr_type`


**`src/decompiler/cfg/method_view.rs`** — 12 functions:
`extract_method_cfgs`, `render_method_body`, `manifest_method_for_view`, `render_envelope`, `append_fidelity_warnings`, `calls_for_view`, `build_call_contracts`, `ins`, `extract_builds_local_cfgs_and_rewrites_cross_range_jump`, `render_method_body_emits_fn_with_return_type`, `render_void_method_does_not_return_ambient_value_across_call`, `render_method_body_does_not_associate_manifest_by_name_only`


**`src/decompiler/cfg/phi_lowering.rs`** — 21 functions:
`from_ssa`, `as_variable`, `into_expr`, `new`, `edge_statements`, `entry_statements`, `fresh_name`, `schedule`, `variable`, `phi`, `ssa_with_phis`, `source_names`, `assign`, `groups_live_phi_operands_by_incoming_edge`, `de_versioned_slot_phi_does_not_emit_identity_copies`, `fills_missing_real_predecessor_with_unknown`, `separates_virtual_entry_from_real_backedge`, `lowers_vm_null_phi_operands_to_literal_assignments`, `schedules_acyclic_parallel_copies_without_clobbering_sources`, `breaks_parallel_copy_cycle_with_one_unique_temporary`, `fresh_helper_name_avoids_lowered_source_names`


**`src/decompiler/cfg/structure.rs`** — 15 functions:
`structure`, `structure_with_source_names`, `emit_body`, `emit_ssa_stmt`, `block_has_explicit_return`, `block_has_explicit_failure`, `emit_body_except_condition`, `structure_edge_region`, `loop_follow`, `orient_branch_loop`, `loop_transfer`, `leave_transfer`, `return_through_finally`, `structure_set_edge_region`, `structure_set`


**`src/decompiler/cfg/tests.rs`** — 1 functions:
`make_instr`


**`src/decompiler/cfg/basic_block/block.rs`** — 4 functions:
`new`, `contains_offset`, `instruction_count`, `is_empty`


**`src/decompiler/cfg/basic_block/block_id.rs`** — 4 functions:
`new`, `index`, `from`, `fmt`


**`src/decompiler/cfg/basic_block/terminator.rs`** — 3 functions:
`successors`, `can_fallthrough`, `is_conditional`


**`src/decompiler/cfg/builder/blocks.rs`** — 1 functions:
`create_blocks`


**`src/decompiler/cfg/builder/edges.rs`** — 1 functions:
`build_cfg`


**`src/decompiler/cfg/builder/finally.rs`** — 10 functions:
`lexically_contains`, `apply_finally_routing`, `collect_try_regions`, `assign_endfinally_offsets`, `assign_region_parents`, `assign_endtry_owners`, `refine_catch_continuations`, `record_endtry_owner`, `local_resume_bound`, `catch_continuation`


**`src/decompiler/cfg/builder/leaders.rs`** — 1 functions:
`find_leaders`


**`src/decompiler/cfg/builder/offsets.rs`** — 5 functions:
`offset_to_block_id`, `instruction_end_offset`, `instruction_len_fallback`, `bytes_len`, `end_offset`


**`src/decompiler/cfg/builder/targets.rs`** — 2 functions:
`jump_target`, `try_targets`


**`src/decompiler/cfg/builder/terminator.rs`** — 1 functions:
`compute_terminator`


**`src/decompiler/cfg/graph/core.rs`** — 15 functions:
`new`, `add_block`, `add_edge`, `block`, `block_mut`, `entry_block`, `blocks`, `block_count`, `edges`, `successors`, `predecessors`, `edge_kind`, `exit_blocks`, `block_at_offset`, `default`


**`src/decompiler/cfg/graph/dot.rs`** — 1 functions:
`to_dot`


**`src/decompiler/cfg/graph/reachability.rs`** — 3 functions:
`reachable_blocks`, `unreachable_blocks`, `is_reachable`


**`src/decompiler/cfg/graph/traversal.rs`** — 1 functions:
`reverse_postorder`


**`src/decompiler/cfg/method_body/cfg.rs`** — 3 functions:
`build_method_cfg`, `build_method_cfg_with_non_returning_calls`, `control_transfer_leaves_method`


**`src/decompiler/cfg/method_body/opcode.rs`** — 2 functions:
`classify_opcode`, `classify_instruction`


**`src/decompiler/cfg/method_body/source_map.rs`** — 3 functions:
`build_source_map`, `collect_source_origins`, `collect_nested_source_origins`


**`src/decompiler/cfg/method_body/validation.rs`** — 5 functions:
`validate_renderable`, `validate_block`, `validate_statement`, `validate_control`, `validate_expr`


**`src/decompiler/cfg/ssa/builder.rs`** — 10 functions:
`new`, `with_method_context`, `build`, `build_with_report`, `phi_var`, `fresh_var`, `unknown_var`, `is_unknown`, `is_unknown_or_tainted`, `reverse_top`


**`src/decompiler/cfg/ssa/context.rs`** — 8 functions:
`is_empty`, `new`, `with_may_return`, `with_return_shape`, `with_return_facts`, `with_argument_effects`, `with_argument_field_writes`, `source_names`


**`src/decompiler/cfg/ssa/dominance.rs`** — 27 functions:
`new`, `idom`, `children`, `dominance_frontier_vec`, `strictly_dominates`, `default`, `compute`, `compute_immediate_dominators`, `intersect_dominators`, `find_common_dominator`, `depth_in_dominator_tree`, `idom_parent`, `reverse_post_order`, `dfs_post_order`, `build_dominator_tree`, `compute_df`, `test_dominance_empty_cfg`, `test_dominance_single_block`, `test_dominance_linear_chain`, `test_dominance_diamond`, `test_dominator_tree_structure`, `create_linear_cfg`, `diamond_cfg_dominance_frontier`, `loop_cfg_dominance_frontier`, `loop_latch_is_dominated_by_header_and_header_is_a_loop_header`, `create_loop_cfg`, `create_diamond_cfg`


**`src/decompiler/cfg/ssa/effects.rs`** — 10 functions:
`stack_effect`, `is_stack_reorder`, `is_stack_special`, `push_opcodes_produce_one_value`, `slot_loads_push_one`, `slot_stores_pop_one`, `collection_ops_have_correct_effects`, `conditional_jumps_pop_conditions`, `reorders_and_specials_are_neutral_in_the_table`, `control_flow_is_neutral_and_calls_produce_values`


**`src/decompiler/cfg/ssa/form.rs`** — 30 functions:
`new`, `add_block`, `block`, `blocks_iter`, `block_count`, `add_definition`, `add_use`, `uses_of`, `render`, `stats`, `fmt`, `new`, `add_phi`, `add_stmt`, `is_empty`, `phi_count`, `stmt_count`, `fmt`, `test_ssa_form_creation`, `test_ssa_block_additions`, `test_dominance_info_empty`, `test_ssa_expr_constructors`, `test_use_site`, `test_ssa_expr_display`, `test_ssa_stmt_display`, `test_ssa_block_display`, `test_ssa_form_render`, `test_ssa_stats`, `test_ssa_expr_complex`, `phi_placement_diamond_cfg`


**`src/decompiler/cfg/ssa/mod.rs`** — 1 functions:
`test_dominance_via_cfg`


**`src/decompiler/cfg/ssa/optimize.rs`** — 14 functions:
`optimize`, `one_round`, `normalize_substitutions`, `resolve_once`, `rewrite_expr`, `go`, `as_literal`, `is_slot_var`, `fold_binary`, `fold_unary`, `collect_used`, `collect_expr_vars`, `go`, `rebuild_test_form`


**`src/decompiler/cfg/ssa/to_ir.rs`** — 14 functions:
`ssa_expr_to_ir`, `ssa_expr_to_ir_with_source_names`, `render_ssa_form`, `render_ssa_stmt`, `render_ir_expr`, `render_phi`, `ssa_var_name`, `is_unknown`, `v`, `lowers_binary_and_literal_to_ir`, `lowers_vm_null_sentinel_to_null_literal`, `semantic_call_identity_survives_ssa_to_ir`, `render_form_shows_block_header_and_assignments`, `render_phi_lists_one_operand_per_predecessor`


**`src/decompiler/cfg/ssa/variable.rs`** — 17 functions:
`initial`, `next`, `vm_null`, `is_vm_null`, `exception_payload`, `is_exception_payload`, `fmt`, `add_operand`, `operand_count`, `fmt`, `test_ssa_variable_versioning`, `test_ssa_variable_display_hides_version`, `exception_payloads_are_handler_scoped`, `test_ssa_variable_ord`, `test_phi_node_creation`, `test_phi_node_add_operands`, `test_phi_node_display`


**`src/decompiler/cfg/ssa/builder/collection.rs`** — 29 functions:
`record_definition_facts`, `indexed_collection_shape_for_access`, `indexed_collection_shapes_for_elements`, `opcode_produces_integer_literal`, `is_collection_fact`, `resolve_collection_fact`, `resolve_collection_shape`, `collection_shape_facts_for_variable`, `collection_shape_facts_for_variable_from_state`, `collection_shape_facts_for_variable_parts`, `collection_shape_facts_for_expression_from_state`, `collection_shape_for_expression`, `unanimous_collection_shape`, `unanimous_collection_facts`, `unanimous_argument_field_writes`, `collection_fact_root`, `mark_static_collection_alias`, `record_static_alias_mutation`, `record_static_call_argument_effects`, `invalidate_collection_aliases`, `apply_argument_field_writes`, `update_indexed_shape_for_setitem`, `clear_indexed_collection_shapes`, `invalidate_collection_contents`, `invalidate_all_collection_facts`, `invalidate_all_collection_facts_except`, `resolve_nonnegative_literal`, `resolves_to_null`, `resolve_nonnegative_i32_literal`


**`src/decompiler/cfg/ssa/builder/diagnostics.rs`** — 6 functions:
`record_instruction_ceiling`, `record_missing_operand_metadata`, `operand_matches_encoding`, `record_incomplete_issue`, `record_stack_underflow`, `fixed_reorder_arity`


**`src/decompiler/cfg/ssa/builder/expr.rs`** — 2 functions:
`build_expr`, `intrinsic_call`


**`src/decompiler/cfg/ssa/builder/helpers.rs`** — 13 functions:
`literal_for_push`, `binary_op_for`, `is_boolean_branch`, `comparison_branch_op`, `is_effectful_collection`, `is_collection_mutation`, `is_shape_preserving_collection_mutation`, `unary_op_for`, `mnemonic`, `call_name`, `context_free_call_target`, `collect_expr_uses`, `collect_expr_uses_into`


**`src/decompiler/cfg/ssa/builder/instructions.rs`** — 3 functions:
`apply_drop_bare_throw`, `apply_unpack_packstruct`, `apply_instruction`


**`src/decompiler/cfg/ssa/builder/joins.rs`** — 10 functions:
`is_slot_load_opcode`, `tainted_phi_targets`, `compute_join_entry`, `recover_dup_join_value`, `compute_join_slots`, `compute_join_collection_invalidations`, `initial_entry_stack`, `reserve_argument_versions`, `seed_context_collection_facts`, `static_collection_facts_for_instruction`


**`src/decompiler/cfg/ssa/builder/pipeline.rs`** — 2 functions:
`build_ssa_blocks`, `execute_block`


**`src/decompiler/cfg/ssa/builder/slots.rs`** — 7 functions:
`is_static_slot_name`, `static_load_index`, `static_store_index`, `absent_slot_value`, `slot_name_for`, `indexed_slot`, `requires_reaching_slot_definition`


**`src/decompiler/cfg/ssa/builder/tests.rs`** — 8 functions:
`linear`, `instr`, `uneven_stack_merge`, `first_nonliteral_assignment`, `optimized_return_expression`, `optimized_collection_expression`, `has_unpack_packstruct_intrinsic`, `has_payloadless_throw`


**`src/decompiler/cfg/ssa/builder/instructions/calls.rs`** — 3 functions:
`apply_opaque_call`, `apply_known_call`, `apply_known_tail_call`


**`src/decompiler/cfg/ssa/builder/instructions/indexed.rs`** — 1 functions:
`apply_indexed_stack_operation`


**`src/decompiler/cfg/ssa/builder/instructions/reorder.rs`** — 1 functions:
`apply_reorder`


**`src/decompiler/cfg/ssa/builder/instructions/special.rs`** — 1 functions:
`apply_special`


**`src/decompiler/cfg/ssa/builder/instructions/syscall.rs`** — 1 functions:
`apply_syscall`


**`src/decompiler/cfg/ssa/builder/tests/calls.rs`** — 15 functions:
`linear_compute_produces_real_binary_expr`, `dup_creates_a_copy_definition`, `call_results_replace_pre_call_stack_values_at_ret`, `dropping_opaque_call_result_does_not_expose_pre_call_values`, `known_call_contract_preserves_stack_and_uses_source_argument_order`, `known_tail_jump_returns_resolved_call_with_source_argument_order`, `known_call_contract_emits_void_call_without_phantom_result`, `known_calla_contract_consumes_pointer_without_rendering_it_as_an_argument`, `collection_mutations_emit_ordered_effect_calls`, `collection_mutation_underflow_preserves_declared_arity`, `structured_known_syscall_value_uses_catalog_contract`, `structured_known_syscall_void_preserves_ambient_stack_value`, `structured_known_syscall_preserves_declaration_order`, `structured_syscall_fallback_keeps_missing_known_argument_visible`, `structured_syscall_fallback_unknown_hash_uses_opaque_barrier`


**`src/decompiler/cfg/ssa/builder/tests/collection_facts.rs`** — 18 functions:
`aliased_unpack_preserves_unmodified_collection_provenance`, `slot_round_trip_preserves_unmodified_collection_provenance`, `collection_mutation_invalidates_all_alias_provenance`, `setitem_invalidates_contents_but_preserves_collection_shape`, `dynamic_pickitem_uses_a_uniform_nested_collection_shape`, `internal_call_return_facts_reach_dynamic_pickitem_unpack`, `argument_field_writes`, `argument_field_writes_reject_dynamic_conflicting_partial_and_overwritten_facts`, `static_alias_resize_and_unknown_call_prevent_reusing_seeded_shape`, `value_returning_collection_mutation_invalidates_all_alias_provenance`, `known_call_invalidates_collection_argument_provenance`, `collection_returning_call_does_not_preserve_argument_shape_across_alias_mutation`, `shape_preserving_internal_call_discards_contents_but_retains_arity`, `syscall_invalidates_collection_argument_provenance`, `opaque_call_invalidates_all_collection_provenance`, `internal_call_invalidates_static_collection_provenance`, `later_internal_call_does_not_retroactively_invalidate_collection_provenance`, `loop_backedge_mutation_invalidates_header_collection_provenance`


**`src/decompiler/cfg/ssa/builder/tests/control_flow.rs`** — 16 functions:
`entry_loop_keeps_manifest_arguments_as_incoming_slots`, `inferred_entry_stack_arguments_follow_vm_order`, `entry_loop_keeps_inferred_arguments_as_incoming_stack_values`, `store_local_emits_a_slot_assignment`, `store_then_load_connects_within_a_block`, `diamond_places_a_phi_at_the_merge`, `diamond_places_a_phi_for_a_slot`, `partially_initialized_slot_merge_is_incomplete`, `initslot_seeds_locals_with_null_before_partial_assignment`, `first_static_load_establishes_snapshot_for_non_writing_branch`, `loop_phi_uses_ambient_static_value_on_preheader`, `exception_edges_supply_their_payload_at_mixed_joins`, `exceptional_finally_entry_does_not_taint_normal_return_stack`, `known_non_returning_call_does_not_produce_a_stack_value`, `dup_conditional_join_reuses_a_shorter_prefix_top_value`, `entry_loop_slot_without_virtual_initial_value_is_incomplete`


**`src/decompiler/cfg/ssa/builder/tests/dynamic_stack.rs`** — 12 functions:
`reported_build_marks_clean_method_exact`, `reported_build_marks_literal_pack_exact`, `literal_dynamic_stack_operations_apply_exact_vm_order`, `literal_pick_creates_a_fresh_ssa_copy`, `literal_dynamic_stack_operand_resolves_through_an_ssa_copy`, `literal_dynamic_stack_operations_accept_zero_and_depth_boundary`, `literal_dynamic_stack_operations_reject_positions_beyond_depth`, `dynamic_stack_literals_must_be_nonnegative_i32_integers`, `dynamic_stack_i32_max_resolves_before_stack_bounds_check`, `runtime_variable_dynamic_stack_operands_remain_incomplete`, `literal_dynamic_stack_operations_report_unknown_selected_values`, `reported_build_keeps_dynamic_pack_incomplete`


**`src/decompiler/cfg/ssa/builder/tests/fidelity.rs`** — 14 functions:
`reported_build_records_unresolved_call_at_the_call_site`, `reported_build_records_explicitly_unresolved_call_target`, `reported_build_records_missing_operand_metadata_at_the_instruction`, `unreachable_underflow_is_covered_without_reducing_semantic_fidelity`, `reported_build_records_unknown_value_reaching_return`, `reported_build_records_unknown_phi_reaching_return`, `reported_build_records_unknown_phi_consumed_by_resolved_call`, `reported_build_records_unknown_phi_consumed_by_drop`, `reported_build_records_fixed_reorder_underflow_at_the_instruction`, `reported_build_keeps_uncertain_syscall_overloads_conservative`, `reported_build_does_not_warn_for_exact_csharp_syscall_bindings`, `reported_build_records_unsupported_control_at_the_instruction`, `reported_build_records_stack_underflow_at_the_instruction`, `reported_build_records_slot_load_without_reaching_definition`


**`src/decompiler/cfg/ssa/builder/tests/stack_ops.rs`** — 28 functions:
`convert_consumes_one_value`, `istype_preserves_target_tag`, `convert_and_istype_reject_any_target_tag`, `newarray_t_accepts_any_target_tag`, `newarray_t_preserves_element_type`, `pack_preserves_elements`, `pack_accepts_nonnegative_wide_literal_count`, `packstruct_preserves_elements`, `reports_only_unanimous_unmodified_collection_return_shapes`, `packmap_preserves_pairs_in_source_order`, `unpack_constant_pack_pushes_literal_count`, `unpack_constant_pack_replays_vm_element_order`, `unpack_shaped_call_result_uses_runtime_indexes_once_in_vm_order`, `mutation_invalidates_shaped_call_result_before_unpack`, `adjacent_drop_bare_throw_preserves_the_empty_stack_fault_exactly`, `drop_throw_with_an_ambient_value_keeps_the_throw_payload`, `drop_bare_throw_is_not_fused_across_a_basic_block_boundary`, `adjacent_unpack_packstruct_becomes_exact_clone_intrinsic`, `unpack_packstruct_fusion_preserves_ambient_stack_values`, `non_adjacent_unpack_packstruct_is_not_fused`, `unpack_packstruct_is_not_fused_across_basic_block_boundary`, `unpack_packstruct_fusion_preserves_source_underflow_diagnostic`, `unpack_packstruct_fusion_preserves_unknown_source_diagnostic`, `signed_wide_pushes_decode_to_decimal`, `printable_pushdata_becomes_string_literal`, `nonprintable_pushdata_remains_bytes`, `user_append_call_remains_internal_while_vm_append_is_intrinsic`, `context_free_calls_preserve_encoded_identity`


**`src/decompiler/cfg/ssa/form/expr.rs`** — 6 functions:
`var`, `binary`, `unary`, `call`, `unresolved_call`, `fmt`


**`src/decompiler/cfg/ssa/form/stmt.rs`** — 7 functions:
`assign`, `expr`, `ret`, `throw`, `abort`, `assert`, `fmt`


**`src/decompiler/cfg/ssa/optimize/indexes.rs`** — 2 functions:
`rebuild_indexes`, `add_expr_uses`


**`src/decompiler/cfg/ssa/optimize/tests.rs`** — 21 functions:
`v`, `assign_str`, `does_not_propagate_constant_through_a_slot_variable`, `propagates_vm_null_through_a_slot_load_alias`, `folds_constant_binary_and_propagates`, `does_not_fold_negative_integer_exponents`, `does_not_fold_i64_overflow_as_wrapping_vm_arithmetic`, `propagates_copy_chains`, `eliminates_trivial_phi`, `retargets_terminator_use_when_removing_variable_trivial_phi`, `retargets_terminator_through_variable_trivial_phi_chain`, `rewrites_expression_through_variable_trivial_phi_chain`, `rewrites_expression_before_pruning_surviving_phi_operands`, `leaves_rooted_cyclic_phis_stable`, `preserves_literal_trivial_phi_used_by_terminator`, `preserves_literal_trivial_phi_used_by_nontrivial_phi`, `removes_dead_phi_and_releases_operand_definition`, `converges_long_reverse_copy_chain_in_one_call`, `removes_dead_mutually_dependent_phi_component`, `eliminates_dead_constant_def`, `effect_statement_keeps_input_definition_live`


**`src/decompiler/cfg/structure/analysis.rs`** — 9 functions:
`compute_loop_headers`, `compute_postdominators`, `find_irreducible_region`, `reachable_cfg`, `reverse_reachable_cfg`, `collect_structural_uses`, `collect_expr_uses`, `collect_leave_targets`, `resolve_leave_target_cfg`


**`src/decompiler/cfg/structure/branches.rs`** — 7 functions:
`handle_branch`, `handle_branch_in_set`, `comparison_condition_for_block`, `condition_for_block`, `condition_variable_for_block`, `condition_expression`, `can_inline_condition`


**`src/decompiler/cfg/structure/cleanup.rs`** — 7 functions:
`simplify_unreachable_control`, `collect_goto_labels`, `collect_statement_goto_labels`, `simplify_block`, `simplify_statement`, `block_always_terminates`, `statement_always_terminates`


**`src/decompiler/cfg/structure/for_loops.rs`** — 18 functions:
`try_promote_for`, `update_and_variable`, `update_shape`, `arithmetic_update_shape`, `is_one_literal`, `normalized_update_shape`, `is_scalar_normalization`, `is_normalization_target`, `is_scalar_expression`, `is_side_effect_free_expression`, `size_refresh_target`, `is_generated_name`, `is_zero_initializer`, `statement_mentions_variable`, `symbol_base`, `contains_variable`, `arithmetic_update_recovers_increment_and_decrement`, `normalized_update_rejects_source_state_after_increment`


**`src/decompiler/cfg/structure/graph.rs`** — 9 functions:
`find_merge`, `postdominates`, `shortest_distances`, `reachable`, `natural_loop_blocks`, `closest_loop_merge`, `loop_common_postdominators`, `loop_distances`, `terminator`


**`src/decompiler/cfg/structure/loops.rs`** — 4 functions:
`build_loop`, `find_dowhile_latch`, `find_unconditional_latch`, `try_emit_infinite_branch_loop`


**`src/decompiler/cfg/structure/regions.rs`** — 4 functions:
`structure_irreducible`, `emit_irreducible_entry`, `irreducible_edge_block`, `structure_region`


**`src/decompiler/cfg/structure/switches.rs`** — 5 functions:
`try_switch`, `can_promote_switch_comparison`, `extract_eq_cond`, `is_literal`, `is_slot_load`


**`src/decompiler/cfg/structure/tests.rs`** — 11 functions:
`v`, `diamond_cfg`, `block_with`, `phi`, `block_contains_call`, `stmt_contains_call`, `control_flow_contains_call`, `expr_contains_call`, `collect_transfers`, `entry_self_loop_structure`, `pathological_dense_jmpif_cfg_structures_promptly`


**`src/decompiler/cfg/structure/tests_branches_loops.rs`** — 22 functions:
`bypassable_loop_node_is_not_a_shared_merge`, `bypassable_acyclic_join_is_not_selected_as_branch_merge`, `branch_headed_loop_with_terminal_exit_is_unconditional`, `removes_unreferenced_leave_label_after_terminal_try`, `keeps_referenced_label_after_terminal_transfer`, `unreachable_goto_does_not_keep_its_label_alive`, `constant_false_continue_self_loop_recovers_do_while`, `structures_a_diamond_into_an_if_else`, `direct_branch_to_merge_copy_stays_inside_selected_arm`, `degenerate_same_target_branch_emits_one_edge_copy`, `analysis_ssa_retains_phi_while_structured_ir_lowers_it`, `inlines_branch_comparison_condition_and_does_not_duplicate_it`, `straight_line_cfg_emits_flat_block`, `structures_a_back_edge_into_a_while_loop`, `nearest_loop_diamond_merge_stays_after_both_branch_arms`, `unconditional_backedge_to_try_entry_becomes_while_true`, `nonlocal_plain_endtry_returns_from_try_entry_loop`, `structures_early_break_and_continue`, `structures_false_edge_loop_body_with_nested_break`, `promotes_explicit_induction_loop_to_for`, `promotes_compiler_copy_chain_induction_loop_to_for`, `promotes_scalar_normalized_induction_loop_to_for`


**`src/decompiler/cfg/structure/tests_do_while_switch.rs`** — 3 functions:
`structures_a_bottom_tested_loop_into_do_while`, `do_while_phi_backedge_copy_stays_in_body`, `structures_an_equality_cascade_into_a_switch`


**`src/decompiler/cfg/structure/tests_entry_phi.rs`** — 12 functions:
`structure_initializes_virtual_entry_phi_once`, `entry_self_loop_keeps_virtual_initialization_separate`, `structure_emits_jump_edge_copy_before_merge_body`, `single_block_ssa`, `adjacent_single_use_call_temp_is_returned_directly`, `unused_call_temp_is_an_expression_statement`, `missing_use_index_keeps_referenced_call_temp_assigned`, `missing_cross_block_use_index_keeps_call_temp_assigned`, `multi_use_call_temp_remains_assigned`, `named_slot_call_remains_assigned_when_unused`, `unused_non_call_temp_remains_assigned`, `call_temp_used_as_call_argument_remains_assigned`


**`src/decompiler/cfg/structure/tests_irreducible_phi.rs`** — 3 functions:
`irreducible_region_uses_typed_labels`, `infinite_loop_phi_copies_cover_both_arms_and_backedge`, `while_phi_copies_run_in_preheader_and_latch`


**`src/decompiler/cfg/structure/tests_terminal_loops.rs`** — 3 functions:
`terminal_return_scan_loop`, `promotes_terminal_return_scan_loop_to_for`, `terminal_return_scan_loop_keeps_while_with_extra_update_effect`


**`src/decompiler/cfg/structure/tests_try_regions.rs`** — 4 functions:
`structures_a_try_entry_into_try_catch`, `direct_leave_successor_is_hoisted_as_the_branch_merge`, `try_phi_copies_stay_in_their_selected_region`, `endtry_continuation_copy_is_shared_after_all_regions`


**`src/decompiler/cfg/structure/try_regions.rs`** — 5 functions:
`handle_try`, `catch_variable`, `endtries_for_region`, `find_endtry_for_arms`, `try_has_nonlocal_leave`


**`src/decompiler/cfg/structure/cleanup/int_normalization.rs`** — 19 functions:
`collapse_int32_wrappers`, `collapse_children`, `render`, `match_int32_wrapper`, `direct_copy_target`, `mask_path_target`, `matches_bound_check`, `is_variable`, `is_integer_literal`, `v`, `assign`, `mask_path`, `wrapper`, `collapses_exact_wrapper_to_signed_mask_expression`, `leaves_partial_wrapper_untouched`, `collapses_wrappers_inside_loop_bodies`, `collapses_wrapper_when_the_normalized_value_is_returned`, `rewrites_setitem_value_after_normalization`, `preserves_normalized_value_after_slot_assignment`


**`src/decompiler/cfg/structure/cleanup/int_normalization_uses.rs`** — 2 functions:
`statement_uses_variable`, `expression_uses_variable`


**`src/decompiler/cfg/structure/cleanup/loops.rs`** — 12 functions:
`recover_header_init_loops`, `try_lift_header_init_while`, `promote_adjacent_for_loops`, `try_promote_while_at`, `peel_unit_update`, `unit_update_shape`, `rewrite_control_flow_children`, `is_constant_initializer`, `is_one_literal`, `symbol_base`, `expr_mentions_base`, `block_assigns_base`


**`src/decompiler/cfg/structure/cleanup/size_normalization.rs`** — 24 functions:
`i32_normalization`, `i64_normalization`, `collapse_size_wrappers`, `collapse_children`, `render`, `match_size_wrapper`, `matches_size_check`, `matches_size_overflow_check`, `size_normalization_for_bound`, `matches_bound_check`, `bound_value`, `mask_path_target`, `direct_copy_target`, `is_variable`, `integer_literal_value`, `literal_matches`, `v`, `assign`, `mask_path_with`, `wrapper`, `wrapper_with`, `collapses_exact_size_guarded_wrapper`, `collapses_i64_size_guarded_wrapper`, `leaves_unrecognized_size_bound_untouched`


**`src/decompiler/cfg/structure/cleanup/temps.rs`** — 20 functions:
`reduce_temporaries`, `v`, `assign`, `render`, `propagates_single_use_copy_into_next_statement`, `propagates_copy_chain_through_multiple_steps`, `keeps_multi_use_temporaries`, `does_not_propagate_across_interfering_assignment`, `propagates_into_nested_branch_of_following_statement`, `skips_propagation_into_loop_that_reassigns_free_variable`, `collapses_nested_dynamic_and_identity_casts`, `drops_casts_of_literals_to_natural_types`, `removes_dead_store_and_its_now_unused_source`, `keeps_side_effecting_rhs_as_expression_statement`, `keeps_dead_division_that_can_fault`, `keeps_dead_index_read_that_can_fault`, `static_stores_are_never_dead`, `phi_branch_merge_folds_into_conditional_expression`, `bool_false_arm_folds_into_logical_and`, `call_results_are_not_propagated`


**`src/decompiler/cfg/structure/cleanup/temps/arrays.rs`** — 3 functions:
`fold_array_initializers`, `new_array_target`, `const_int`


**`src/decompiler/cfg/structure/cleanup/temps/casts.rs`** — 3 functions:
`simplify_casts`, `simplify_cast_expr`, `literal_cast_is_redundant`


**`src/decompiler/cfg/structure/cleanup/temps/copies.rs`** — 3 functions:
`propagate_single_use_copies`, `propagate_one`, `copy_candidate_at`


**`src/decompiler/cfg/structure/cleanup/temps/dead_stores.rs`** — 1 functions:
`eliminate_dead_stores`


**`src/decompiler/cfg/structure/cleanup/temps/merges.rs`** — 4 functions:
`fold_branch_value_merges`, `fold`, `match_branch_merge`, `is_merge_temporary`


**`src/decompiler/cfg/structure/cleanup/temps/queries.rs`** — 12 functions:
`statement_uses_variable`, `count_expr_uses`, `statement_assigns_variable`, `block_assigns_variable`, `expression_has_effectful_call`, `expression_may_fault`, `is_pure`, `is_pure_call_target`, `collect_collection_bases`, `statement_mutates_collections`, `expr_mentions_any`, `is_static_slot`


**`src/decompiler/cfg/structure/cleanup/temps/support.rs`** — 10 functions:
`of`, `total`, `for_each_child_block_mut`, `visit_stmt_exprs`, `visit_block_exprs`, `visit_expr`, `mutate_stmt_exprs`, `substitute_variable`, `mutate_block_exprs`, `mutate_expr_postorder`


**`src/decompiler/cfg/structure/for_loops/terminal_update.rs`** — 7 functions:
`terminal_update_shape`, `rewrite_terminal_update`, `strictly_terminates`, `contains_continue`, `block_update_shape`, `terminal_update_requires_a_single_induction_assignment`, `terminal_update_rejects_extra_effects_and_continue`


**`src/decompiler/cfg/tests/basic.rs`** — 5 functions:
`empty_instructions_produces_empty_cfg`, `single_block_linear_code`, `block_contains_offset`, `block_instruction_count`, `block_id_display`


**`src/decompiler/cfg/tests/dot.rs`** — 1 functions:
`cfg_to_dot_produces_valid_output`


**`src/decompiler/cfg/tests/jumps.rs`** — 6 functions:
`unconditional_jump_creates_two_blocks`, `conditional_branch_creates_multiple_blocks`, `multiple_exit_blocks`, `successors_and_predecessors`, `edge_count_matches_terminators`, `long_jump_creates_blocks`


**`src/decompiler/cfg/tests/reachability.rs`** — 1 functions:
`unreachable_blocks_detect_dead_code_after_jump`


**`src/decompiler/cfg/tests/rpo.rs`** — 1 functions:
`reverse_postorder_visits_all_blocks`


**`src/decompiler/cfg/tests/terminators.rs`** — 6 functions:
`throw_creates_exit_block`, `abort_creates_exit_block`, `resolved_non_returning_call_terminates_its_block`, `terminator_successors`, `terminator_properties`, `endtry_is_modeled_as_endtry_terminator`


**`src/decompiler/cfg/tests/try_blocks.rs`** — 10 functions:
`try_entry_adds_exception_and_finally_edges`, `endtry_routes_through_finally_to_the_natural_continuation`, `nonlocal_finally_continuation_routes_to_its_explicit_target`, `nested_endtry_at_inner_resume_boundary_routes_through_outer_finally`, `nested_catch_resume_boundary_belongs_to_the_enclosing_try`, `nested_body_resume_boundary_belongs_to_the_enclosing_catch`, `triple_nested_endtry_chain_unwinds_one_parent_at_a_time`, `shared_finally_dispatches_to_each_saved_normal_continuation`, `catch_region_distinguishes_natural_and_nonlocal_endtry_targets`, `catch_endtry_defines_natural_continuation_when_try_body_throws`


**`src/decompiler/csharp/helpers.rs`** — 8 functions:
`legacy_statement_to_csharp`, `declaration_type`, `parse_slot_index`, `legacy_statement_to_csharp_with_context`, `legacy_statement_to_csharp_untyped`, `csharpize_vm_condition`, `format_vm_truthiness`, `format_vm_assertion`


**`src/decompiler/csharp/render.rs`** — 10 functions:
`is_exact_syscall`, `method`, `record`, `render_csharp`, `vm_exception_type_name`, `contract_member_names`, `assert_message_helper_name`, `unpack_packstruct_helper_name`, `bare_throw_helper_name`, `tagged_opcode_helpers`


**`src/decompiler/csharp/helpers/legacy_expression.rs`** — 2 functions:
`is_decimal_integer_literal`, `legacy_expression_to_csharp`


**`src/decompiler/csharp/helpers/legacy_expression_scanner.rs`** — 6 functions:
`matching_paren`, `split_top_level_colon`, `split_top_level_args`, `find_matching_close_paren`, `split_top_level_comma`, `rewrite_cat_operator`


**`src/decompiler/csharp/helpers/signatures.rs`** — 7 functions:
`collect_csharp_parameters`, `format_csharp_parameters`, `format_manifest_type_csharp`, `format_method_signature`, `sanitize_csharp_identifier`, `is_csharp_keyword`, `escape_csharp_string`


**`src/decompiler/csharp/helpers/legacy_expression/collections.rs`** — 7 functions:
`match_map_literal`, `match_collection_constructor`, `match_unary_pattern`, `wrap_int_cast_unless_literal`, `is_decimal_integer_literal`, `match_simple_unary`, `match_method_call`


**`src/decompiler/csharp/helpers/legacy_expression/literals.rs`** — 3 functions:
`match_big_integer_literal`, `match_big_byte_literal`, `decimal_exceeds_u64`


**`src/decompiler/csharp/helpers/legacy_expression/numeric.rs`** — 4 functions:
`rewrite_numeric_helpers`, `match_numeric_helper`, `format_helper_with_casts`, `wrap_int_cast_unless_literal`


**`src/decompiler/csharp/render/body.rs`** — 4 functions:
`render_method_body`, `plan_method_declarations`, `long_relative_call_underflow_uses_compatibility_recovery`, `non_void_partial_body_gets_fail_closed_fallthrough`


**`src/decompiler/csharp/render/events.rs`** — 2 functions:
`event_signatures`, `write_events`


**`src/decompiler/csharp/render/methods.rs`** — 6 functions:
`write_manifest_methods`, `write_inferred_methods`, `write_script_entry_if_needed`, `write_fallback_entry`, `write_body`, `write_method_attributes`


**`src/decompiler/csharp/render/body/fidelity.rs`** — 7 functions:
`semantic_warnings`, `requires_structured_stub`, `recover_with_compatibility`, `throwing_stub`, `throwing_stub_with_fidelity`, `fidelity_issue`, `issue_kind_label`


**`src/decompiler/csharp/render/body/recovery.rs`** — 7 functions:
`recovered_result`, `render_placeholder_arguments`, `indent_body`, `ensure_non_void_termination`, `prepend_argument_underflow_comment`, `compatibility_recovery_does_not_invent_zero_argument_placeholders`, `compatibility_recovery_bounds_large_placeholder_argument_lists`


**`src/decompiler/csharp/render/body/recovery_underflow.rs`** — 3 functions:
`underflow_call_targets`, `underflow_placeholder`, `relative_target`


**`src/decompiler/csharp/render/header/helpers.rs`** — 6 functions:
`write_vm_exception_type`, `write_assert_message_helper`, `write_bare_throw_helper`, `write_unpack_packstruct_helper`, `write_tagged_opcode_helpers`, `write_unresolved_call_helper`


**`src/decompiler/csharp/render/header/metadata.rs`** — 2 functions:
`write_method_tokens_comment`, `write_trusts_comment`


**`src/decompiler/csharp/render/header/mod.rs`** — 6 functions:
`write_preamble`, `write_contract_open`, `write_pattern_comments`, `pattern_confidence_label`, `write_static_fields`, `write_contract_close`


**`src/decompiler/csharp/render/structured/declaration_type_catalog.rs`** — 9 functions:
`csharp_type_for_value_type`, `csharp_type_value_type`, `csharp_array_element_value_type`, `csharp_array_element_type`, `homogeatipicialus_csharp_array_type`, `csharp_member_type`, `concrete_csharp_type_name`, `concrete_type_matches_value_type`, `array_element_type`


**`src/decompiler/csharp/render/structured/declaration_types.rs`** — 9 functions:
`concrete_definition_type`, `concrete_definition_type_with_symbols`, `concrete_definition_type_with_symbols_and_known_types`, `concrete_definition_type_with_symbols_and_known_types_and_calls`, `concrete_call_type`, `concrete_expression_type`, `concrete_expression_type_with_symbols_and_known`, `byte_container_result_type`, `pickitem_result_type`


**`src/decompiler/csharp/render/structured/declarations.rs`** — 18 functions:
`new`, `root`, `scope_at`, `parent_of`, `add_child`, `nearest_common_ancestor`, `common_ancestor`, `parent`, `with_static_field_types`, `plan_declarations`, `plan_declarations_with_known_types`, `plan_declarations_with_known_types_and_calls`, `collect_index_defined_symbols`, `collect_indexed_base_symbols`, `plan_contract_symbols`, `declaration_issue`, `merge_value_types`, `csharp_type`


**`src/decompiler/csharp/render/structured/expr.rs`** — 15 functions:
`new`, `in_context`, `render_expr`, `render_vm_condition`, `render_expr_prec`, `render_expr_node`, `is_array_creation_base`, `renders_to_object`, `dynamic_object_operand`, `render_math_arg`, `render_binary`, `low_level_binary_opcode`, `binary_spelling`, `fold_overflowing_int_literals`, `render_unary`


**`src/decompiler/csharp/render/structured/expr_calls.rs`** — 1 functions:
`render_call`


**`src/decompiler/csharp/render/structured/expr_context.rs`** — 19 functions:
`for_block`, `with_emitted_names`, `with_concrete_types`, `with_typed_array_literals`, `with_tagged_opcode_helper_calls`, `with_unpack_packstruct_helper_call`, `with_internal_call_return_types`, `with_vm_argument_underflow`, `with_event_signatures`, `exact_csharp_type`, `exact_internal_call_value_type`, `is_inlined`, `singleton_array_element`, `array_elements`, `event_signature`, `is_debug_singleton_array_target`, `is_event_array_target`, `value_type`, `concrete_value_type`


**`src/decompiler/csharp/render/structured/expr_context_index.rs`** — 4 functions:
`is_statically_exact_csharp_type`, `exact_literal_index_type`, `constant_index`, `resolve`


**`src/decompiler/csharp/render/structured/expr_context_intrinsics.rs`** — 1 functions:
`value_type`


**`src/decompiler/csharp/render/structured/expr_context_patterns.rs`** — 5 functions:
`collect_notification_state_targets`, `collect_block`, `collect_statement`, `collect_control`, `collect_expr`


**`src/decompiler/csharp/render/structured/expr_context_types.rs`** — 4 functions:
`collect_array_values`, `collect_array_aliases`, `exact_common_value_type`, `is_concrete_value_type`


**`src/decompiler/csharp/render/structured/expr_inline.rs`** — 8 functions:
`child_scope`, `visit_block`, `visit_statement`, `visit_control`, `visit_expr`, `take_order`, `is_inline_pure`, `expression_value_type`


**`src/decompiler/csharp/render/structured/expr_intrinsics.rs`** — 1 functions:
`render_intrinsic`


**`src/decompiler/csharp/render/structured/expr_low_level.rs`** — 6 functions:
`render_low_level_opcode`, `render_tagged_type_opcode`, `render_tagged_type_opcode_source`, `tagged_opcode_helper_key`, `default_tagged_opcode_helper_name`, `render_low_level_boolean_binary_opcode`


**`src/decompiler/csharp/render/structured/expr_native.rs`** — 3 functions:
`render_method_token_call`, `render_native_args`, `is_native_property`


**`src/decompiler/csharp/render/structured/expr_syscalls.rs`** — 9 functions:
`is_exact_syscall`, `render_syscall`, `render_event_argument`, `render_syscall_arguments`, `render_typed_receiver`, `render_exact_or_cast`, `render_low_level_syscall`, `syscall_arguments`, `known_syscall_is_classified`


**`src/decompiler/csharp/render/structured/expr_syscalls_catalog.rs`** — 3 functions:
`is_always_renderable`, `is_exact_binding`, `known_syscall_api`


**`src/decompiler/csharp/render/structured/expr_values.rs`** — 6 functions:
`render_expr_list`, `render_object_array_literal`, `int_cast`, `render_new_array`, `render_literal`, `escape_csharp_string`


**`src/decompiler/csharp/render/structured/mod.rs`** — 1 functions:
`is_exact_syscall`


**`src/decompiler/csharp/render/structured/native_framework.rs`** — 5 functions:
`method_name`, `is_supported_contract`, `is_supported_method`, `maps_vm_method_casing_to_framework_spelling`, `rejects_catalog_methods_without_framework_bindings`


**`src/decompiler/csharp/render/structured/nullability.rs`** — 6 functions:
`null_checked_argument_indices`, `null_checked_argument_source`, `trace_argument_value`, `local_load_index`, `local_store_index`, `argument_load_index`


**`src/decompiler/csharp/render/structured/plan.rs`** — 13 functions:
`emitted_names`, `parameter_names`, `synthetic_entry`, `fallback_entry`, `manifest_method`, `inferred_method`, `method_labels_by_offset`, `method_arg_counts_by_offset`, `method_return_types_by_offset`, `method_symbol_maps`, `index_defined_statics`, `has_unresolved_calls`, `index`


**`src/decompiler/csharp/render/structured/plan_activity.rs`** — 15 functions:
`new`, `with_symbol_types`, `record_definition`, `resolve_concrete_definition_types_with_known_types_and_calls`, `record_use`, `occurrence`, `index_defined_symbols`, `indexed_base_symbols`, `unused_copy_symbols`, `propagate_copy_definitions`, `propagate_indexed_base_symbols`, `is_nullable_csharp_type`, `is_nullable_definition`, `is_nullable_value_type`, `is_null_definition`


**`src/decompiler/csharp/render/structured/plan_helpers.rs`** — 9 functions:
`draft_method_context`, `cross_range_tail_target`, `synthetic_entry_draft`, `manifest_method_draft`, `method_end`, `parameter_type_signature`, `make_unique_method_name`, `return_value_option`, `method_symbol_types`


**`src/decompiler/csharp/render/structured/plan_methods.rs`** — 1 functions:
`build_csharp_method_plans`


**`src/decompiler/csharp/render/structured/stmt.rs`** — 15 functions:
`render_block`, `render_block_with_trace`, `render_block_with_trace_and_underflow`, `terminates`, `terminates_statement`, `render_block_at`, `render_block_at_with_facts`, `render_block_at_omitting`, `render_trace_comments`, `render_statement`, `should_omit_statement`, `typed_array_csharp_type`, `new`, `next_child`, `line`


**`src/decompiler/csharp/render/structured/stmt_control_flow.rs`** — 5 functions:
`render_control_flow`, `render_for`, `fresh_switch_value`, `fresh_reserved_name`, `replace_last_line`


**`src/decompiler/csharp/render/structured/stmt_facts.rs`** — 5 functions:
`update_definition_facts`, `control_defines_variable`, `block_defines_variable`, `statement_defines_variable`, `expr_contains_call`


**`src/decompiler/csharp/render/structured/stmt_foreach.rs`** — 11 functions:
`render_foreach`, `detect_foreach`, `inferred_foreach_item_type`, `resolve_collection_expression`, `concrete_value_type`, `is_zero_literal`, `indexed_collection`, `resolve_collection_alias`, `is_collection_bound`, `is_collection_bound_inner`, `is_collection_expression`


**`src/decompiler/csharp/render/structured/stmt_foreach_guards.rs`** — 14 functions:
`block_mentions_variable`, `block_assigns_variable`, `statement_mentions_variable`, `statement_assigns_variable`, `control_mentions_variable`, `control_assigns_variable`, `expr_mentions_variable`, `block_has_opaque_calls`, `statement_has_opaque_calls`, `control_has_opaque_calls`, `expr_has_opaque_call`, `block_writes_static`, `statement_writes_static`, `control_writes_static`


**`src/decompiler/csharp/render/structured/stmt_values.rs`** — 9 functions:
`render_for_initializer`, `render_for_update`, `render_expression_statement`, `render_exception`, `render_vm_throw`, `render_return`, `render_assignment`, `render_typed_value`, `hoisted_declarations`


**`src/decompiler/csharp/render/structured/tests.rs`** — 2 functions:
`method_contract`, `expr_context_with_types`


**`src/decompiler/csharp/render/structured/tests_events.rs`** — 1 functions:
`manifest_event_notify_omits_only_an_exact_packed_state_temp`


**`src/decompiler/csharp/render/structured/tests_expr_collections.rs`** — 4 functions:
`collection_intrinsics_use_the_receiver_container_type`, `ambiguous_collection_intrinsics_use_low_level_wrappers`, `indexing_intrinsics_guard_unsupported_receivers`, `byte_intrinsics_use_framework_compatible_conversions`


**`src/decompiler/csharp/render/structured/tests_expr_core.rs`** — 1 functions:
`renders_all_expression_variants`


**`src/decompiler/csharp/render/structured/tests_expr_formatting.rs`** — 7 functions:
`map_intrinsics_guard_known_non_map_receivers`, `unmodeled_intrinsic_uses_a_low_level_wrapper`, `renders_expression_precedence_from_structure`, `nested_predicate_calls_parenthesize_ternary_operands`, `negative_integer_literal_does_not_form_a_decrement_token`, `csharp_strings_escape_unicode_line_separators`, `typed_shift_counts_render_as_int`


**`src/decompiler/csharp/render/structured/tests_expr_operators.rs`** — 6 functions:
`value_equality_uses_csharp_operators_only_for_known_value_types`, `logical_not_uses_vm_truthiness_for_integer_operands`, `isnull_is_false_for_non_nullable_value_types`, `isnull_preserves_ambiguous_integer_aliases`, `numeric_operators_use_vm_wrappers_for_static_any_values`, `vm_boolean_binary_operators_are_eager_only_for_known_booleans`


**`src/decompiler/csharp/render/structured/tests_expr_syscalls.rs`** — 11 functions:
`syscall_rendering_uses_hash_identity_and_drops_display_metadata`, `compiler_debug_notify_lowers_only_proven_singleton_string_states`, `manifest_event_notify_lifts_only_an_exact_packed_state`, `typed_syscall_fallbacks_preserve_catalog_return_types`, `check_witness_requires_explicit_framework_overload_evidence`, `check_witness_uses_proven_address_types_without_redundant_casts`, `syscall_arguments_match_framework_signatures`, `syscall_metadata_is_removed_only_from_the_extra_selector_slot`, `storage_syscalls_select_overloads_from_neutral_types`, `storage_syscalls_use_validated_csharp_types_when_vm_types_are_unknown`, `every_known_syscall_has_an_explicit_csharp_policy`


**`src/decompiler/csharp/render/structured/tests_expr_types.rs`** — 16 functions:
`resolved_internal_call_return_types_drive_expression_typing`, `resolved_boolean_internal_call_avoids_dynamic_truthiness_cast`, `proven_literal_array_indexes_preserve_selected_element_types`, `literal_array_index_provenance_stays_conservative_at_invalid_indexes`, `object_array_literal_indexes_are_runtime_typed_but_not_static_exact`, `known_native_method_tokens_drive_exact_csharp_expression_types`, `framework_native_alias_types_remain_concrete_in_expression_context`, `additional_framework_returns_remain_concrete_in_expression_context`, `native_array_returns_preserve_index_element_types`, `framework_object_members_preserve_concrete_types`, `validated_csharp_variable_types_refine_unknown_value_types`, `planned_alias_types_flow_into_member_and_index_inference`, `planned_parameter_types_flow_into_member_inference`, `known_syscalls_drive_exact_csharp_expression_types`, `proven_expression_shapes_keep_concrete_value_types`, `intrinsic_and_member_shapes_keep_concrete_value_types`


**`src/decompiler/csharp/render/structured/tests_plan_declarations.rs`** — 8 functions:
`plans_declarations`, `for_body_definition_used_by_update_is_hoisted_to_the_loop_scope`, `missing_uninitialized_symbol_is_a_lost_stack_value`, `stack_placeholder_is_a_lost_stack_value`, `unused_local_copy_is_removed_without_dropping_its_source`, `csharp_emits_static_referenced_beyond_type_info`, `referenced_static_beyond_type_info_reserves_the_method_name`, `static_fields_reserve_contract_member_names`


**`src/decompiler/csharp/render/structured/tests_plan_methods.rs`** — 17 functions:
`plans_overloads_and_calls_together`, `infers_concrete_return_type_for_private_literal_helper`, `infers_private_parameter_type_from_unanimous_internal_calls`, `conflicting_private_parameter_calls_remain_dynamic`, `null_checked_private_parameter_stays_dynamic`, `null_checked_private_array_parameter_uses_the_proven_reference_type`, `indexed_private_parameter_stays_dynamic`, `indexed_private_array_parameter_uses_the_proven_array_type`, `infers_exact_string_return_type_for_private_native_helper`, `unresolved_private_call_keeps_helper_return_dynamic`, `propagates_private_return_type_through_helper_chain`, `mixed_private_returns_remain_dynamic`, `plans_cross_range_tail_jump_with_detached_helper_arity`, `null_checked_value_parameters_use_dynamic_csharp_signatures`, `null_checked_local_aliases_use_dynamic_csharp_signatures`, `infers_private_helper_parameter_name_from_manifest_argument`, `conflicting_helper_parameter_name_votes_keep_placeholder`


**`src/decompiler/csharp/render/structured/tests_plan_types.rs`** — 14 functions:
`infers_concrete_types_for_common_structured_expressions`, `symbol_aware_expression_types_cover_index_and_numeric_copies`, `framework_native_aliases_are_accepted_for_typed_declarations`, `internal_call_return_types_drive_typed_local_aliases`, `typed_array_element_types_survive_aliases_but_unknown_arrays_stay_dynamic`, `unknown_pickitem_provenance_remains_dynamic`, `repeated_concrete_definitions_keep_their_shared_csharp_type`, `repeated_conflicting_definitions_remain_dynamic`, `repeated_nullable_reference_definitions_keep_the_reference_type`, `nullable_reference_aliases_keep_the_proven_reference_type`, `nullable_reference_aliases_can_start_with_a_null_slot_type`, `nullable_typed_array_aliases_stay_dynamic_at_unknown_mutation_boundaries`, `nullable_reference_provenance_does_not_cross_boolean_type_checks`, `nullable_value_definitions_remain_dynamic`


**`src/decompiler/csharp/render/structured/tests_stmt_control.rs`** — 6 functions:
`renders_all_control_flow_variants`, `typed_statement_termination_is_recursive`, `typed_statement_returns_follow_the_method_contract`, `typed_statement_rendering_removes_inlined_temporary_definitions`, `typed_for_rendering_preserves_the_planned_loop_scope`, `typed_expression_statements_are_compile_valid_and_effect_preserving`


**`src/decompiler/csharp/render/structured/tests_stmt_foreach.rs`** — 8 functions:
`typed_literal_array_foreach_uses_uniform_element_type_through_aliases`, `typed_array_index_loops_render_as_foreach_when_the_index_is_private`, `typed_array_index_loops_follow_size_aliases`, `typed_array_index_loops_keep_for_when_bound_is_not_collection_size`, `typed_array_index_loops_keep_for_when_counter_escapes_after_loop`, `typed_array_index_loops_keep_for_when_extraction_temporary_escapes`, `typed_array_index_loops_keep_for_when_the_counter_escapes`, `typed_array_index_loops_keep_for_across_opaque_body_calls`


**`src/decompiler/csharp/render/structured/tests_stmt_inlining.rs`** — 12 functions:
`inlines_only_pure_single_use_temporaries`, `observable_state_and_allocations_are_not_inlineable`, `temporary_inlining_does_not_move_throwing_expressions`, `temporary_inlining_does_not_move_wrapper_backed_predicates`, `temporary_inlining_does_not_move_casts`, `temporary_inlining_does_not_move_values_into_while_conditions`, `temporary_inlining_does_not_move_for_initializers_into_conditions`, `temporary_inlining_does_not_move_for_initializers_into_updates`, `temporary_inlining_does_not_move_values_into_do_while_conditions`, `temporary_inlining_rejects_reassigned_dependencies`, `temporary_inlining_requires_a_concrete_value_type`, `temporary_inlining_requires_definition_before_use`


**`src/decompiler/csharp/render/structured/tests_stmt_types.rs`** — 16 functions:
`typed_statement_references_use_planned_csharp_identifiers`, `typed_boundaries_render_valid_explicit_conversions`, `runtime_typed_literal_indexes_keep_object_array_cast_boundaries`, `typed_internal_call_boundaries_use_exact_resolved_return_types`, `typed_ambient_assignments_render_boundary_conversions`, `typed_static_field_boundaries_use_contract_field_types`, `concrete_native_reference_arrays_flow_into_object_array_storage`, `known_native_void_calls_render_as_statements`, `compiler_debug_notify_omits_the_packed_state_temp`, `typed_index_definitions_ignore_stale_slot_collection_types`, `typed_index_copy_provenance_converges_independently_of_statement_order`, `typed_index_assignments_dynamicize_parameter_and_static_storage`, `hoisted_phi_declarations_are_default_initialized`, `missing_phi_definition_gets_a_conservative_default`, `typed_boundaries_bridge_incompatible_known_types`, `typed_boundaries_box_value_array_literals_into_object_arrays`


**`src/decompiler/csharp/render/structured/expr_intrinsics/bytes.rs`** — 3 functions:
`render_byte_concat`, `render_byte_slice`, `render_memcpy`


**`src/decompiler/csharp/render/structured/expr_intrinsics/nullability.rs`** — 3 functions:
`definitely_non_null_value`, `exact_non_nullable_value`, `is_non_nullable_value_type`


**`src/decompiler/csharp/render/structured/plan_activity/visitor.rs`** — 6 functions:
`visit_block`, `visit_statement`, `visit_control`, `visit_child_block`, `visit_expr`, `record_index_base_symbols`


**`src/decompiler/csharp/render/structured/plan_methods/calls.rs`** — 1 functions:
`attach_call_plans`


**`src/decompiler/csharp/render/structured/plan_methods/parameter_calls.rs`** — 5 functions:
`collect_internal_call_arguments`, `collect_block_calls`, `collect_control_calls`, `collect_expr_calls`, `is_concrete_type`


**`src/decompiler/csharp/render/structured/plan_methods/parameter_names.rs`** — 9 functions:
`infer_private_parameter_names`, `recovered_call_shapes_match`, `offer`, `unanimous`, `is_placeholder`, `name_hint`, `collect_name_hints`, `collect_definitions`, `resolve_copy`


**`src/decompiler/csharp/render/structured/plan_methods/parameter_types.rs`** — 9 functions:
`infer_private_parameter_types`, `indexed_parameter_indices`, `expected_private_calls`, `invalid_private_targets`, `concrete_return_types_by_offset`, `invalidate`, `is_concrete_type`, `is_indexable_csharp_type`, `is_nullable_csharp_type`


**`src/decompiler/csharp/render/structured/plan_methods/return_types.rs`** — 8 functions:
`infer_private_return_types`, `concrete_return_types_by_offset`, `csharp_return_type`, `inferred_return_type`, `collect_return_type`, `collect_statement_return_type`, `merge_return_type`, `collect_control_return_type`


**`src/decompiler/csharp/render/structured/plan_methods/parameter_names/tests.rs`** — 4 functions:
`hints_through_wrappers`, `rejects_placeholders_and_expressions`, `votes_conflict_on_disagreement`, `call_arity_disagreement_blocks_name_inference`


**`src/decompiler/csharp/render/structured/plan_methods/parameter_names/traversal.rs`** — 3 functions:
`visit_statement_exprs`, `visit_block_exprs`, `visit_expr`


**`src/decompiler/helpers/identifiers.rs`** — 2 functions:
`sanitize_identifier`, `make_unique_identifier`


**`src/decompiler/helpers/lifted.rs`** — 8 functions:
`build_method_arg_counts_by_offset`, `build_method_returns_value_by_offset`, `infer_entry_stack_arg_count_for_inferred_start`, `estimate_required_entry_stack_depth`, `stack_effect_for_arg_inference`, `build_calla_targets_by_offset`, `build_call_targets_by_offset`, `memcpy_requires_five_entry_stack_arguments`


**`src/decompiler/helpers/manifest.rs`** — 3 functions:
`extract_contract_name`, `format_permission_entry`, `render_extra_scalar`


**`src/decompiler/helpers/methods.rs`** — 12 functions:
`find_manifest_entry_method`, `offset_as_usize`, `inferred_method_starts`, `next_inferred_method_offset`, `initslot_argument_count_at`, `collect_initslot_offsets`, `collect_call_targets`, `collect_post_ret_method_offsets`, `collect_control_flow_edges`, `relative_target`, `relative_target_with_delta`, `build_method_labels_by_offset`


**`src/decompiler/helpers/parameters.rs`** — 2 functions:
`format_manifest_parameters`, `sanitize_parameter_names`


**`src/decompiler/helpers/types.rs`** — 5 functions:
`inferred_type_to_csharp`, `format_manifest_type`, `known_kinds_normalise_regardless_of_case`, `unknown_kinds_preserve_original_case`, `inferred_csharp_types_map_correctly`


**`src/decompiler/helpers/vm_values.rs`** — 5 functions:
`value_type_from_operand`, `stack_item_type_tag`, `printable_utf8`, `signed_le_bytes_to_decimal`, `stack_item_type_tags_round_trip`


**`src/decompiler/high_level/render.rs`** — 2 functions:
`render_high_level`, `build_noreturn_method_offsets`


**`src/decompiler/high_level/emitter/core.rs`** — 17 functions:
`with_program`, `set_argument_labels`, `set_callt_labels`, `set_callt_param_counts`, `set_callt_returns_value`, `set_method_labels_by_offset`, `set_method_arg_counts_by_offset`, `set_method_returns_value_by_offset`, `set_call_targets_by_offset`, `set_calla_targets_by_offset`, `set_noreturn_method_offsets`, `set_inline_single_use_temps`, `set_emit_trace_comments`, `set_returns_void`, `advance_to`, `finish`, `open_brace_depth`


**`src/decompiler/high_level/emitter/dispatch.rs`** — 1 functions:
`emit_instruction`


**`src/decompiler/high_level/emitter/helpers.rs`** — 6 functions:
`literal_from_operand`, `format_pushdata`, `convert_target_name`, `format_int_bytes_as_decimal`, `format_type_operand`, `strip_outer_parens`


**`src/decompiler/high_level/emitter/slots.rs`** — 8 functions:
`emit_init_static_slots`, `emit_init_slots`, `emit_load_slot`, `emit_load_slot_from_operand`, `emit_store_slot`, `emit_store_slot_from_operand`, `slot_label`, `slot_index_from_operand`


**`src/decompiler/high_level/emitter/util.rs`** — 7 functions:
`note`, `warn`, `stack_underflow`, `push_comment`, `next_temp`, `take_usize_literal`, `transfer_label_name`


**`src/decompiler/high_level/emitter/control_flow/branches.rs`** — 10 functions:
`is_conditional_branch`, `has_internal_crossing_branch`, `has_crossing_closer`, `emit_conditional_goto`, `emit_comparison_if_block`, `emit_if_block`, `emit_jmpif_block`, `emit_unary_if_block`, `detect_else`, `detect_implicit_else`


**`src/decompiler/high_level/emitter/control_flow/jumps.rs`** — 8 functions:
`normalize_internal_call_target`, `resolve_internal_call_name`, `emit_internal_call`, `emit_relative_call`, `emit_relative`, `emit_indirect_call`, `emit_jump`, `emit_endtry`


**`src/decompiler/high_level/emitter/control_flow/loops.rs`** — 10 functions:
`analyze_do_while_loops`, `try_emit_do_while_tail`, `try_emit_do_while_negated_tail`, `try_emit_do_while_comparison_tail`, `detect_loop_back`, `try_emit_loop_jump`, `close_loops_at`, `pop_loops_with_continue`, `is_loop_control_target`, `pre_register_backward_jump_labels`


**`src/decompiler/high_level/emitter/control_flow/targets.rs`** — 3 functions:
`forward_jump_target`, `branch_width`, `jump_target`


**`src/decompiler/high_level/emitter/control_flow/try_blocks/emit.rs`** — 1 functions:
`emit_try_block`


**`src/decompiler/high_level/emitter/control_flow/try_blocks/search.rs`** — 2 functions:
`find_endtry_target`, `find_endfinally_end`


**`src/decompiler/high_level/emitter/control_flow/try_blocks/targets.rs`** — 1 functions:
`try_handler_targets`


**`src/decompiler/high_level/emitter/dispatch/collections.rs`** — 1 functions:
`try_emit_collection_ops`


**`src/decompiler/high_level/emitter/dispatch/control_flow.rs`** — 1 functions:
`try_emit_control_flow`


**`src/decompiler/high_level/emitter/dispatch/literals.rs`** — 2 functions:
`resolve_pusha_display`, `try_emit_literals`


**`src/decompiler/high_level/emitter/dispatch/math.rs`** — 1 functions:
`try_emit_math`


**`src/decompiler/high_level/emitter/dispatch/slots.rs`** — 1 functions:
`try_emit_slot_ops`


**`src/decompiler/high_level/emitter/dispatch/stack_ops.rs`** — 1 functions:
`try_emit_stack_ops`


**`src/decompiler/high_level/emitter/postprocess/compound_assign.rs`** — 7 functions:
`rewrite_compound_assignments`, `rewrite_increment`, `rewrite_rhs`, `is_identifier`, `rewrites_simple_assignment_into_compound_form`, `does_not_rewrite_let_bindings`, `rewrites_for_header_increment_expression`


**`src/decompiler/high_level/emitter/postprocess/else_if.rs`** — 3 functions:
`rewrite_else_if_chains`, `find_matching_close`, `remove_one_closer`


**`src/decompiler/high_level/emitter/postprocess/for_loops.rs`** — 8 functions:
`rewrite_for_loops`, `precompute_block_ends`, `assert_matches_find_block_end`, `precompute_block_ends_matches_find_block_end_simple`, `precompute_block_ends_matches_find_block_end_nested`, `precompute_block_ends_matches_find_block_end_sequential_and_else`, `precompute_block_ends_matches_find_block_end_unclosed_headers`, `precompute_block_ends_matches_find_block_end_balanced_inline_braces`


**`src/decompiler/high_level/emitter/postprocess/indexing.rs`** — 7 functions:
`rewrite_indexing_syntax`, `rewrite_set_item`, `rewrite_expr`, `find_expr_op`, `split_args`, `rewrite_indexing_preserves_get_token_inside_string_literal`, `rewrite_indexing_still_rewrites_real_get_operator`


**`src/decompiler/high_level/emitter/postprocess/join_close_chain.rs`** — 1 functions:
`join_close_brace_with_chain`


**`src/decompiler/high_level/emitter/postprocess/labels.rs`** — 1 functions:
`remove_orphaned_labels`


**`src/decompiler/high_level/emitter/postprocess/overflow_collapse.rs`** — 22 functions:
`collapse_overflow_checks`, `next_code_line`, `try_match_overflow`, `apply_collapse`, `fixup_downstream_reference`, `parse_bare_assignment`, `is_temp_identifier`, `leading_whitespace`, `parse_let_assignment`, `find_overflow_block_end`, `find_matching_brace`, `stmts`, `collapses_unchecked_int32_add`, `collapses_checked_int32_add`, `collapses_unsigned_range_check`, `collapses_int64_range_check`, `does_not_match_unrelated_if`, `handles_negate_equality_check`, `skips_interleaved_comments`, `handles_if_else_without_comments`, `preserves_indentation`, `find_matching_brace_closes_at_combined_else_line`


**`src/decompiler/high_level/emitter/postprocess/simplify.rs`** — 16 functions:
`collapse_if_true`, `invert_empty_if_else`, `remove_empty_if`, `is_pure_rhs`, `reduce_double_parens`, `strip_stack_comments`, `is_temp_ident`, `temp_tokens`, `negate_condition`, `rhs_calls_only_pure_helpers`, `is_pure_helper_identifier`, `brace_balance`, `invert_empty_if_else_keeps_braces_balanced`, `eliminate_identity_temps_substitutes_forward`, `eliminate_identity_temps_skips_lhs_used_before_definition`, `invert_empty_if_else_wraps_compound_condition`


**`src/decompiler/high_level/emitter/postprocess/switches.rs`** — 31 functions:
`rewrite_switch_statements`, `extract_block_body`, `collect_trivia`, `resolve_condition_expression`, `parse_case_sides`, `resolve_case_value`, `split_equals`, `is_literal`, `is_temp`, `find_first_if_in_range`, `find_next_if_after_case_prelude`, `temp_consumed_by_next_code`, `find_next_guarded_header_after_case_prelude`, `case_body_is_switch_safe`, `body_ends_with_terminator`, `is_terminator_statement`, `statement_reassigns`, `parse_inline_if_goto`, `parse_plain_goto_label`, `parse_label_line`, `parse_guarded_switch_body_header`, `collect_label_bodies`, `find_label_after`, `find_label_in_range`, `find_else_block_after`, `find_label_body_end`, `end_of_if_chain`, `switch_fold_preserves_non_temp_inter_case_statement`, `switch_fold_preserves_side_effecting_temp_between_cases`, `switch_fold_still_applies_to_consecutive_cases`, `switch_fold_accepts_negated_final_equality_case`


**`src/decompiler/high_level/emitter/postprocess/while_loops.rs`** — 10 functions:
`is_numeric_literal`, `condition_mentions_ident`, `rewrite_header_init_loops`, `rewrite_goto_do_while`, `rewrite_switch_break_gotos`, `rewrite_if_goto_to_while`, `rewrite_label_goto_to_loop`, `eliminate_fallthrough_gotos`, `next_if_line`, `next_code_line`


**`src/decompiler/high_level/emitter/postprocess/else_if/tests.rs`** — 3 functions:
`collapses_else_if_chain`, `preserves_simple_else`, `extracts_if_condition`


**`src/decompiler/high_level/emitter/postprocess/inline/condition.rs`** — 5 functions:
`inline_condition_temps`, `condition_inline_candidate`, `inlines_negated_loop_condition_temp`, `inlines_negated_for_condition_temp`, `still_inlines_bare_condition_temp`


**`src/decompiler/high_level/emitter/postprocess/inline/for_increment.rs`** — 3 functions:
`inline_for_increment_temps`, `does_not_inline_for_increment_temp_used_elsewhere`, `inlines_pure_single_use_for_increment_temp`


**`src/decompiler/high_level/emitter/postprocess/inline/single_use.rs`** — 1 functions:
`inline_single_use_temps`


**`src/decompiler/high_level/emitter/postprocess/inline/single_use/analysis.rs`** — 1 functions:
`collect_candidates`


**`src/decompiler/high_level/emitter/postprocess/inline/single_use/rewrite.rs`** — 1 functions:
`apply_inlining`


**`src/decompiler/high_level/emitter/postprocess/inline/single_use/tests.rs`** — 6 functions:
`single_use_temp_is_inlined_into_first_use_site`, `single_use_temp_is_not_inlined_into_control_flow_conditions`, `single_use_literal_temp_is_inlined_into_control_flow_conditions`, `non_temp_identifiers_are_not_inlined`, `temp_replacement_respects_identifier_boundaries`, `chained_inline_preserves_equality_operator`


**`src/decompiler/high_level/emitter/postprocess/inline/single_use/util.rs`** — 10 functions:
`is_safe_to_inline`, `rhs_calls_only_pure_helpers`, `is_pure_helper_identifier`, `needs_parens`, `is_control_flow_condition`, `is_trivial_inline_rhs`, `is_temp_identifier`, `is_numeric_literal`, `is_string_literal`, `is_identifier`


**`src/decompiler/high_level/emitter/postprocess/simplify/temps.rs`** — 3 functions:
`eliminate_identity_temps`, `collapse_temp_into_store`, `eliminate_dead_temps`


**`src/decompiler/high_level/emitter/postprocess/switches/chain.rs`** — 1 functions:
`try_build_switch`


**`src/decompiler/high_level/emitter/postprocess/switches/guarded.rs`** — 1 functions:
`try_build_guarded_goto_switch`


**`src/decompiler/high_level/emitter/postprocess/util/analysis.rs`** — 3 functions:
`is_numeric_or_simple_literal`, `find_increment_assignment`, `should_inline_condition`


**`src/decompiler/high_level/emitter/postprocess/util/blocks.rs`** — 2 functions:
`find_block_end`, `brace_delta`


**`src/decompiler/high_level/emitter/postprocess/util/ident.rs`** — 4 functions:
`contains_identifier`, `replace_identifier`, `is_identifier_char`, `is_identifier_boundary`


**`src/decompiler/high_level/emitter/postprocess/util/parsing.rs`** — 5 functions:
`extract_while_condition`, `parse_assignment`, `extract_if_condition`, `parse_for_parts`, `is_valid_lhs`


**`src/decompiler/high_level/emitter/postprocess/util/patterns.rs`** — 6 functions:
`is_if_open`, `is_else_open`, `is_else_if_open`, `extract_if_condition`, `extract_else_if_condition`, `extract_any_if_condition`


**`src/decompiler/high_level/emitter/postprocess/util/scan.rs`** — 2 functions:
`find_initializer_index`, `previous_code_line`


**`src/decompiler/high_level/emitter/stack/manipulation.rs`** — 2 functions:
`is_simple_literal_or_identifier`, `string_has_only_escaped_quotes`


**`src/decompiler/high_level/emitter/stack/expressions/collections.rs`** — 6 functions:
`emit_pack`, `emit_unpack`, `infer_unpack_element_count`, `is_single_pop`, `emit_convert`, `emit_is_type`


**`src/decompiler/high_level/emitter/stack/expressions/core.rs`** — 8 functions:
`pop_stack_value_with_literal`, `pop_stack_value`, `emit_call`, `push_literal`, `binary_op`, `binary_index`, `unary_op`, `pusha_target`


**`src/decompiler/high_level/emitter/stack/expressions/flow.rs`** — 6 functions:
`emit_return`, `emit_syscall`, `missing_syscall_argument_context`, `store_slot_context`, `operand_slot_index`, `format_slot_label`


**`src/decompiler/high_level/emitter/stack/manipulation/basic.rs`** — 5 functions:
`drop_top`, `dup_top`, `over_second`, `swap_top`, `nip_second`


**`src/decompiler/high_level/emitter/stack/manipulation/indexed.rs`** — 3 functions:
`emit_pick`, `emit_roll`, `emit_xdrop`


**`src/decompiler/high_level/emitter/stack/manipulation/reorder.rs`** — 2 functions:
`emit_rot`, `emit_tuck`


**`src/decompiler/high_level/emitter/stack/manipulation/reverse.rs`** — 2 functions:
`emit_reverse_fixed`, `emit_reverse_n`


**`src/decompiler/high_level/render/body.rs`** — 1 functions:
`write_method_body`


**`src/decompiler/high_level/render/entry.rs`** — 1 functions:
`write_entry_method`


**`src/decompiler/high_level/render/header.rs`** — 1 functions:
`write_contract_header`


**`src/decompiler/high_level/render/manifest_summary.rs`** — 1 functions:
`write_manifest_summary`


**`src/decompiler/high_level/render/method_tokens.rs`** — 1 functions:
`write_method_tokens`


**`src/decompiler/high_level/render/methods.rs`** — 2 functions:
`write_manifest_methods`, `write_inferred_methods`


**`src/decompiler/ir/control_flow.rs`** — 6 functions:
`if_then`, `if_else`, `while_loop`, `do_while`, `for_loop`, `try_catch`


**`src/decompiler/ir/semantic.rs`** — 4 functions:
`display_name`, `display_name`, `method_token_display_name_is_a_safe_identifier_without_losing_metadata`, `known_syscall_keeps_name_metadata_and_generic_spelling`


**`src/decompiler/ir/statement.rs`** — 14 functions:
`assign`, `ret`, `ret_void`, `throw`, `abort`, `assert`, `expr`, `comment`, `new`, `with_stmts`, `push`, `is_empty`, `len`, `from`


**`src/decompiler/ir/expression/expr.rs`** — 7 functions:
`int`, `var`, `binary`, `unary`, `call`, `unresolved_call`, `index`


**`src/decompiler/ir/expression/literal.rs`** — 1 functions:
`fmt`


**`src/decompiler/ir/expression/operators.rs`** — 2 functions:
`fmt`, `fmt`


**`src/decompiler/ir/render/expr.rs`** — 2 functions:
`render_expr`, `render_literal`


**`src/decompiler/ir/render/stmt/control_flow.rs`** — 1 functions:
`render_control_flow`


**`src/decompiler/ir/render/stmt/mod.rs`** — 2 functions:
`render_stmt`, `render_block`


**`src/decompiler/ir/tests/block.rs`** — 3 functions:
`test_block_rendering`, `test_block_empty`, `test_block_push`


**`src/decompiler/ir/tests/control_flow_rendering.rs`** — 4 functions:
`test_if_statement_rendering`, `test_if_else_rendering`, `test_while_loop_rendering`, `test_try_catch_rendering`


**`src/decompiler/ir/tests/expression_rendering.rs`** — 8 functions:
`test_literal_rendering`, `test_variable_rendering`, `test_binary_expression_rendering`, `test_unary_expression_rendering`, `test_call_expression_rendering`, `test_index_expression_rendering`, `test_cast_expression_rendering`, `test_array_rendering`


**`src/decompiler/ir/tests/statement_rendering.rs`** — 3 functions:
`test_assignment_statement_rendering`, `test_return_statement_rendering`, `test_comment_rendering`


**`src/decompiler/pipeline/io.rs`** — 2 functions:
`read_aef_file`, `io_decompile_file_with_manifest`


**`src/decompiler/tests/csharp.rs`** — 1 functions:
`render_csharp_with_coverage`


**`src/decompiler/tests/csharp_body_core.rs`** — 16 functions:
`csharp_multimethod_uses_structured_constant_fold`, `csharp_typed_declarations_preserve_loop_counter_type`, `csharp_loopif_deversions_local_slot`, `csharp_loopif_recovers_counting_loop_without_defeated_condition`, `csharp_assert_uses_structured_body`, `csharp_separates_clr_exception_transport_from_vm_payload`, `csharp_assert_preserves_non_scalar_vm_truthiness`, `csharp_assert_message_preserves_eager_vm_message_validation`, `csharp_assert_message_helper_call_ignores_parameter_shadowing`, `csharp_assert_uses_globally_qualified_framework_intrinsic`, `csharp_assert_message_uses_globally_qualified_opcode_attribute`, `csharp_failure_statements_preserve_semantics_and_abort_warning`, `csharp_trace_failure_statements_preserve_semantics_and_abort_warning`, `csharp_trace_assert_converts_numeric_local_to_boolean`, `csharp_trace_assert_converts_named_numeric_parameter_to_boolean`, `csharp_trace_assert_prefers_emitted_parameter_name_over_raw_slot_syntax`


**`src/decompiler/tests/csharp_control_flow.rs`** — 7 functions:
`csharp_recovers_counting_loop_from_header_init_back_edge`, `csharp_translates_switch_to_idiomatic_c_sharp`, `csharp_else_if_chain_uses_parenthesised_conditions`, `csharp_view_respects_manifest_metadata_and_parameters`, `csharp_view_escapes_all_manifest_attribute_controls`, `high_level_view_renders_manifest_groups_block`, `csharp_view_renders_non_string_scalar_extra_metadata`


**`src/decompiler/tests/csharp_coverage.rs`** — 8 functions:
`collect_aef_files`, `collect_renderer_sources`, `load_expected_invalid`, `artifact_id`, `find_expected_invalid`, `csharp_renderer_has_no_legacy_body_dependencies`, `csharp_corpus_has_zero_structured_fallback`, `assert_pinned_incomplete_baseline`


**`src/decompiler/tests/csharp_fidelity.rs`** — 10 functions:
`csharp_invalid_any_convert_and_istype_use_whole_method_fallback`, `csharp_unknown_source_and_unresolved_call_use_whole_method_fallback`, `csharp_detached_packstruct_helper_keeps_vm_underflow_explicit`, `csharp_typed_map_temporary_keeps_its_receiver_type`, `csharp_fallback_primary_issue_is_the_first_incomplete_issue`, `csharp_coverage_retains_same_name_overloads_at_one_offset`, `csharp_synthetic_script_entry_exposes_initslot_args_and_preserves_return`, `csharp_omits_trailing_return_in_void_methods`, `csharp_private_void_call_preserves_ambient_return_value`, `csharp_keeps_explicit_return_value_in_non_void_methods`


**`src/decompiler/tests/csharp_legacy.rs`** — 1 functions:
`legacy_statement_to_csharp_converts_known_forms`


**`src/decompiler/tests/csharp_literals.rs`** — 9 functions:
`legacy_statement_to_csharp_does_not_panic_on_degenerate_headers`, `csharpize_nested_helper_calls_in_cast_path_helpers`, `legacy_expression_to_csharp_preserves_multibyte_in_cat_path`, `csharp_escapes_control_chars_in_pushdata_string_literal`, `csharp_non_void_method_with_empty_body_throws_not_implemented`, `csharp_void_event_parameter_renders_as_object_not_void`, `csharp_wraps_only_oversized_integer_literals`, `csharp_renders_oversized_hex_blob_as_byte_array`, `csharp_renders_map_literal_as_collection_initializer`


**`src/decompiler/tests/csharp_metadata.rs`** — 7 functions:
`csharp_trims_initslot_boundaries`, `csharp_multi_entry_typed_trusts_render_as_block`, `header_surfaces_aef_compiler_and_source_fields`, `csharp_header_surfaces_inferred_patterns_and_language`, `csharp_header_renders_method_tokens_block`, `csharp_header_omits_method_tokens_block_when_none`, `csharp_single_entry_typed_trusts_stay_on_one_line`


**`src/decompiler/tests/csharp_methods.rs`** — 22 functions:
`csharp_resolves_internal_calls_to_method_names`, `csharp_internal_call_uses_duplicate_signature_suffix`, `csharp_ambiguous_internal_call_emits_unresolved_call_helper`, `csharp_manifest_void_internal_call_is_a_statement`, `csharp_manifest_void_resolved_calla_is_a_statement`, `csharp_offsetless_manifest_void_internal_call_is_a_statement`, `csharp_offsetless_manifest_void_resolved_calla_is_a_statement`, `csharp_manifest_void_tail_call_does_not_return_ambient_stack_value`, `csharp_manifest_void_internal_call_underflow_keeps_call_visible`, `csharp_manifest_long_internal_call_underflow_keeps_call_visible`, `csharp_manifest_value_tail_call_underflow_still_returns_call`, `csharp_manifest_value_internal_call_still_produces_a_value`, `csharp_unknown_resolved_calla_still_produces_a_value`, `csharp_emits_inferred_helper_methods`, `csharp_inferred_nonvoid_helpers_do_not_emit_bare_return`, `csharp_includes_offsetless_manifest_methods_as_stubs`, `csharp_includes_manifest_events`, `csharp_disambiguates_events_against_contract_members`, `csharp_escapes_reserved_keywords`, `csharp_uses_label_style_for_transfer_placeholders`, `csharp_mismatch_offset_emits_script_entry_and_manifest_method`, `csharp_missing_manifest_offset_uses_first_method_as_entry_signature`


**`src/decompiler/tests/csharp_packing.rs`** — 9 functions:
`csharp_constant_pack_is_structured_while_trace_mode_selects_legacy`, `csharp_structures_pack_families_and_constant_unpack`, `csharp_structures_printable_raw_and_wide_literals`, `csharp_type_tag_operands_use_structured_renderer`, `csharp_type_tag_helper_avoids_contract_member_collisions`, `csharp_unpack_packstruct_helper_preserves_opcode_order_and_avoids_collisions`, `csharp_bare_throw_helper_preserves_the_opcode_and_avoids_collisions`, `csharp_static_initializer_keeps_adjacent_static_and_local_slot_prologues_together`, `csharp_type_tags_preserve_bytestring_and_struct_identity`


**`src/decompiler/tests/mod.rs`** — 11 functions:
`write_varint`, `build_aef`, `build_aef_with_single_token`, `sample_aef`, `sample_manifest`, `testing_artifact_dir`, `load_testing_aef`, `try_load_testing_aef`, `load_testing_manifest`, `try_load_testing_manifest`, `debug_inferred_method_starts_contract_delegate`


**`src/decompiler/tests/core/analysis.rs`** — 40 functions:
`decompilation_includes_call_graph_syscalls`, `decompilation_includes_call_graph_internal_calls`, `call_graph_resolves_relative_call_from_opcode_offset`, `call_graph_out_of_range_call_target_is_unresolved`, `call_graph_out_of_range_calla_target_is_indirect`, `decompilation_includes_call_graph_method_tokens`, `decompilation_includes_indirect_calls`, `call_graph_resolves_static_pointer_initialized_after_caller`, `decompilation_resolves_pusha_calla_to_internal_call_edge`, `decompilation_resolves_local_pointer_flow_into_calla_edge`, `decompilation_resolves_local_pointer_flow_with_nop_before_calla`, `decompilation_resolves_multi_hop_local_pointer_flow_into_calla_edge`, `decompilation_does_not_resolve_local_pointer_across_method_boundary`, `type_inference_uses_manifest_parameter_types_for_offsetless_entry_method`, `type_inference_tracks_read_only_fixed_argument_slots_without_initslot`, `call_graph_attributes_helper_syscall_to_inferred_helper_method`, `call_graph_attributes_pusha_calla_helper_syscall_to_inferred_helper_method`, `call_graph_attributes_ldarg_calla_helper_syscall_to_inferred_helper_method`, `call_graph_attributes_ldloc_from_argument_calla_helper_syscall_to_inferred_helper_method`, `call_graph_resolves_nested_pusha_argument_through_calla_helper`, `call_graph_resolves_nested_pusha_argument_through_calla_helper_without_initslot`, `call_graph_resolves_two_level_nested_calla_argument_chain`, `inferred_method_starts_tolerate_malformed_tryl_operand`, `decompilation_resolves_pickitem_delegate_array_into_calla_edge`, `decompilation_resolves_pickitem_delegate_array_through_local_alias`, `decompilation_resolves_duplicated_pointer_into_calla_edge`, `decompilation_includes_slot_xrefs`, `decompilation_includes_argument_slot_xrefs`, `decompilation_includes_indexed_slot_xrefs`, `decompilation_includes_static_slot_xrefs`, `decompilation_includes_indexed_static_slot_xrefs`, `decompilation_infers_collection_types_for_locals`, `type_inference_consumes_reverseitems_operand`, `type_inference_consumes_all_memcpy_operands`, `type_inference_consumes_known_syscall_arguments`, `type_inference_does_not_reuse_values_below_a_syscall_result`, `decompilation_propagates_manifest_argument_types`, `decompilation_infers_static_slot_types`, `decompilation_infers_packmap_types`, `decompilation_infers_convert_target_types`


**`src/decompiler/tests/core/decompile.rs`** — 32 functions:
`disassemble_bytes_returns_instruction_stream_without_rendering`, `decompile_end_to_end`, `decompile_with_manifest_produces_contract_name`, `cfg_to_dot_includes_contract_name_and_script_hash_in_label`, `decompile_lifts_indirect_calls_without_not_yet_translated_warning`, `decompile_uses_method_token_signature_for_callt_arguments_and_returns`, `restricted_native_callt_does_not_emit_a_qualified_label`, `decompile_lifts_relative_calls_without_control_flow_warning`, `decompile_resolves_relative_call_target_to_inferred_method_name`, `decompile_relative_call_passes_known_method_arguments`, `decompile_infers_entry_stack_argument_for_syscall_only_helper`, `decompile_lifts_unconditional_jumps_without_control_flow_warning`, `decompile_manifestless_entry_surfaces_initslot_args`, `decompile_known_syscall_drops_redundant_hash_comment_in_clean_mode`, `decompile_unknown_syscall_keeps_unknown_annotation`, `decompile_lifts_endtry_transfers_without_control_flow_warning`, `decompile_uses_label_style_for_unresolved_jump_targets`, `decompile_uses_label_style_for_unresolved_endtry_targets`, `decompile_calla_with_stack_setup`, `decompile_resolves_pusha_calla_to_internal_call_placeholder`, `decompile_resolves_local_pointer_flow_into_calla`, `decompile_resolves_static_pointer_flow_into_calla`, `decompile_multiple_sequential_calls`, `decompile_nested_loop_in_if`, `decompile_try_in_loop`, `decompile_nested_if_else`, `decompile_all_comparison_jumps`, `packmap_pops_key_value_pairs_and_renders_entries`, `huge_pack_count_terminates_quickly`, `huge_packmap_count_terminates_quickly`, `invalid_type_bytes_render_as_raw_hex`, `oversized_method_hits_high_level_lifting_cap`


**`src/decompiler/tests/core/entry_point.rs`** — 3 functions:
`renames_script_entry_using_manifest_signature`, `mismatch_offset_emits_synthetic_entry_and_keeps_manifest_method`, `missing_manifest_offset_uses_first_method_as_entry_signature`


**`src/decompiler/tests/core/identifiers.rs`** — 4 functions:
`contract_name_is_sanitized_with_manifest`, `high_level_sanitizes_manifest_method_and_parameter_names`, `sanitize_identifier_handles_edge_cases`, `high_level_disambiguates_colliding_method_names`


**`src/decompiler/tests/core/syscalls.rs`** — 12 functions:
`decompile_syscall_includes_human_name`, `void_syscall_does_not_push_stack_value`, `unknown_syscall_is_assumed_to_return_value`, `syscall_arguments_render_in_declaration_order`, `storage_put_arguments_render_in_pop_order`, `void_storage_syscall_is_emitted_as_statement`, `void_storage_local_syscall_is_emitted_as_statement`, `syscall_contract_call_returns_value`, `syscall_runtime_log_is_void`, `syscall_runtime_log_missing_argument_emits_warning`, `syscall_runtime_log_after_packed_store_reports_consumed_slot_context`, `syscall_check_witness_returns_value`


**`src/decompiler/tests/core/unknowns.rs`** — 2 functions:
`untranslated_opcode_inline_comment_survives_clean_mode`, `tolerant_mode_emits_unknown_opcode`


**`src/decompiler/tests/high_level/branches.rs`** — 9 functions:
`high_level_lifts_simple_if_block`, `high_level_closes_if_at_end`, `high_level_lifts_if_else_block`, `high_level_lifts_jmpeq_forward_branch`, `high_level_lifts_jmpif_forward_branch`, `high_level_lifts_jmpif_l_forward_branch`, `high_level_else_branch_restores_pre_branch_stack_snapshot`, `crossing_comparison_branch_does_not_emit_malformed_double_else`, `crossing_unary_branch_does_not_emit_malformed_double_else`


**`src/decompiler/tests/high_level/entry_range.rs`** — 3 functions:
`high_level_limits_instructions_to_entry_range`, `high_level_trims_initslot_boundaries`, `high_level_private_void_call_preserves_ambient_return_value`


**`src/decompiler/tests/high_level/postprocess.rs`** — 28 functions:
`strip_stack_comments_removes_swap_annotations`, `reduce_double_parens_collapses_nested_pairs`, `eliminate_dead_temps_strips_unused_arithmetic_expression`, `eliminate_dead_temps_keeps_calls_for_their_side_effects`, `eliminate_dead_temps_keeps_used_temps`, `eliminate_dead_temps_keeps_potentially_throwing_division_or_indexing`, `eliminate_fallthrough_gotos_strips_goto_followed_by_label`, `eliminate_fallthrough_gotos_strips_leave_followed_by_label`, `eliminate_fallthrough_gotos_strips_leave_through_close_braces`, `eliminate_fallthrough_gotos_keeps_leave_when_intervening_code_present`, `eliminate_fallthrough_gotos_keeps_leave_when_target_is_distant`, `rewrite_for_loops_handles_temp_increment_chain`, `rewrite_for_loops_handles_direct_increment`, `rewrite_indexing_syntax_rewrites_conditions_and_assignments`, `rewrite_switch_statements_supports_temp_case_values`, `rewrite_switch_statements_supports_string_literal_case_values`, `rewrite_switch_statements_rewrites_long_guarded_goto_chains`, `rewrite_switch_statements_rewrites_guarded_chain_with_else_embedded_default_label`, `rewrite_switch_statements_rewrites_guarded_chain_with_else_case_and_external_default_label`, `rewrite_switch_statements_flattens_else_blocks_with_nested_chains`, `rewrite_switch_statements_skips_duplicate_cases`, `rewrite_switch_statements_skips_non_literal_cases`, `rewrite_switch_statements_collapses_consecutive_standalone_ifs`, `rewrite_switch_statements_skips_two_consecutive_standalone_ifs`, `rewrite_switch_statements_skips_consecutive_ifs_with_different_scrutinee`, `rewrite_switch_keeps_if_chain_when_case_body_reassigns_scrutinee`, `rewrite_switch_folds_standalone_ifs_when_bodies_terminate`, `rewrite_header_init_loops_lifts_defeated_counting_loop`


**`src/decompiler/tests/high_level/switches.rs`** — 1 functions:
`high_level_recovers_switch_from_equality_chain`


**`src/decompiler/tests/high_level/try_blocks.rs`** — 7 functions:
`high_level_lifts_try_finally_blocks`, `high_level_lifts_try_catch_blocks`, `high_level_lifts_try_catch_finally_blocks`, `high_level_lifts_try_finally_with_throw_inside`, `high_level_lifts_try_catch_with_abort_in_catch`, `high_level_models_catch_entry_stack_with_exception_value`, `malformed_try_out_of_bounds_handlers_keep_braces_balanced`


**`src/decompiler/tests/high_level/loops/break_continue.rs`** — 1 functions:
`high_level_emits_break_and_continue`


**`src/decompiler/tests/high_level/loops/do_while.rs`** — 1 functions:
`high_level_lifts_do_while_loop`


**`src/decompiler/tests/high_level/loops/for_loop.rs`** — 1 functions:
`high_level_lifts_for_loop`


**`src/decompiler/tests/high_level/loops/inlining.rs`** — 1 functions:
`loop_condition_temp_is_inlined`


**`src/decompiler/tests/high_level/loops/loopif_recovery.rs`** — 1 functions:
`high_level_loopif_recovers_counting_loop`


**`src/decompiler/tests/high_level/loops/while_loop.rs`** — 1 functions:
`high_level_lifts_simple_while_loop`


**`src/decompiler/tests/high_level/stack_ops/collections.rs`** — 7 functions:
`high_level_packs_literal_arrays`, `high_level_rewrites_pickitem_as_indexing`, `high_level_rewrites_setitem_as_index_assignment`, `high_level_rewrites_haskey_as_function_call`, `high_level_pickitem_inside_call_keeps_brackets_balanced`, `high_level_istype_respects_operand_tag`, `pack_literal_underflow_renders_elision_marker_not_synthetic_temps`


**`src/decompiler/tests/high_level/stack_ops/control.rs`** — 3 functions:
`high_level_pops_assert_condition`, `high_level_abort_clears_stack`, `high_level_throw_clears_stack`


**`src/decompiler/tests/high_level/stack_ops/slots.rs`** — 5 functions:
`high_level_lifts_local_slots`, `high_level_lifts_all_local_slot_variants`, `high_level_lifts_all_argument_slot_variants`, `high_level_lifts_indexed_local_slot`, `high_level_lifts_indexed_argument_slot`


**`src/decompiler/tests/high_level/stack_ops/manipulation/basic.rs`** — 2 functions:
`high_level_lifts_boolean_ops`, `high_level_handles_stack_manipulation_and_unary_ops`


**`src/decompiler/tests/high_level/stack_ops/manipulation/indexed.rs`** — 4 functions:
`high_level_pick_of_literal_skips_temp`, `high_level_pick_of_side_effecting_value_materializes_temp`, `high_level_lifts_xdrop_with_literal_index`, `high_level_pick_preserves_packed_shape_for_unpack_reverse4`


**`src/decompiler/tests/high_level/stack_ops/manipulation/reorder.rs`** — 2 functions:
`high_level_lifts_rot_operation`, `high_level_lifts_tuck_operation`


**`src/decompiler/tests/high_level/stack_ops/manipulation/reverse.rs`** — 4 functions:
`high_level_lifts_reverse3_operation`, `high_level_lifts_reverse4_operation`, `high_level_lifts_reversen_operation`, `high_level_unpack_of_stored_packed_value_keeps_reverse3_stack_shape`


**`src/disassembler/operand.rs`** — 8 functions:
`read_i16_le`, `read_i32_le`, `read_i64_le`, `read_u16_le`, `read_u32_le`, `read_operand`, `read_bytes_prefixed`, `read_slice`


**`src/disassembler/tests.rs`** — 16 functions:
`decodes_simple_sequence`, `errors_on_unknown_opcode`, `permits_unknown_opcode_when_configured`, `reports_warning_for_unknown_opcode_in_tolerant_mode`, `fails_on_truncated_operand`, `decodes_calla_no_operand`, `decodes_pushdata2`, `decodes_jump_long`, `decodes_pusha_backward_offset_as_signed_i32`, `decodes_pusha_forward_offset_as_signed_i32`, `pusha_truncated_operand_returns_unexpected_eof`, `pushdata2_truncated_length_prefix_returns_unexpected_eof`, `pushdata4_truncated_length_prefix_returns_unexpected_eof`, `decodes_syscall_operand_with_name`, `pushdata4_excessive_length_returns_operand_too_large`, `pushdata4_truncated_payload_returns_unexpected_eof`


**`src/disassembler/operand/immediates.rs`** — 1 functions:
`immediate_constant`


**`src/instruction/model.rs`** — 1 functions:
`new`


**`src/instruction/opcode.rs`** — 2 functions:
`all_known`, `fmt`


**`src/instruction/operand.rs`** — 1 functions:
`fmt`


**`src/manifest/describe.rs`** — 14 functions:
`describe`, `describe`, `describe`, `describe_other_trusts_value`, `describe_structured_trusts`, `parse_typed_entries`, `collect_string_entries`, `structured_trusts_object_with_groups_only_renders_typed_list`, `structured_trusts_object_with_hashes_only_renders_typed_list`, `structured_trusts_object_with_both_keys_concatenates_in_order`, `structured_trusts_with_unknown_key_falls_back_to_raw_json`, `structured_trusts_non_string_array_falls_back_to_raw_json`, `wildcard_trusts_describes_as_star`, `empty_contracts_trusts_describes_as_empty_brackets`


**`src/manifest/parse.rs`** — 7 functions:
`validate_manifest_strict`, `ensure_manifest_size`, `from_reader`, `from_json_str`, `from_json_str_strict`, `from_bytes`, `from_str`


**`src/manifest/tests.rs`** — 13 functions:
`sample_manifest_json`, `parses_manifest_json`, `manifest_from_bytes_rejects_invalid_utf8`, `manifest_from_bytes_rejects_invalid_json`, `manifest_from_bytes_rejects_oversized_payloads`, `manifest_from_json_str_rejects_oversized_payloads`, `parses_wildcard_permission_variants`, `strict_manifest_parsing_accepts_valid_sample`, `strict_manifest_parsing_rejects_non_wildcard_permission_methods`, `strict_manifest_parsing_rejects_non_wildcard_trusts_string`, `classifies_official_string_permission_descriptors`, `strict_manifest_parsing_rejects_malformed_permission_descriptor`, `strict_manifest_parsing_rejects_non_empty_features`


**`src/manifest/model/permissions.rs`** — 5 functions:
`classify`, `is_hash_descriptor`, `is_group_descriptor`, `deserialize`, `default`


**`src/native_contracts/tests.rs`** — 9 functions:
`describes_known_native_method`, `falls_back_to_contract_name_when_method_unknown`, `describe_method_token_prefers_exact_case_match`, `lookup_finds_every_native_contract`, `lookup_unknown_hash_returns_none`, `native_contract_table_is_sorted_by_hash`, `native_method_hint_helpers_report_expected_state`, `native_contract_catalog_includes_latest_core_contracts`, `native_contract_catalog_keeps_legacy_token_contracts`


**`src/util/tests.rs`** — 3 functions:
`writes_upper_hex`, `formats_hashes_in_both_endianness`, `computes_hash160_little_endian`


**`src/web/report.rs`** — 13 functions:
`build_info_report`, `build_disasm_report`, `build_decompile_report`, `from`, `operand_kind_name`, `operand_value_report`, `returns_value_for_instruction`, `build_method_token_report`, `collect_warnings`, `summarize_manifest`, `from`, `from`, `from`


---

## 📚 Documentation Index

**1. [`docs/testing-artifacts.md`](docs/testing-artifacts.md)** — Testing Artifacts
> Testing Artifacts Guide

**2. [`docs/plans/2025-01-24-ssa-transformation-design.md`](docs/plans/2025-01-24-ssa-transformation-design.md)** — 2025 01 24 Ssa Transformation Design
> SSA Transformation Design

**3. [`docs/plans/2026-03-09-aef-hardening-and-consistency.md`](docs/plans/2026-03-09-aef-hardening-and-consistency.md)** — 2026 03 09 Aef Hardening And Consistency
> AEF Hardening And Consistency Implementation Plan

**4. [`docs/plans/2026-03-19-js-decompiler-parity-hardening.md`](docs/plans/2026-03-19-js-decompiler-parity-hardening.md)** — 2026 03 19 Js Decompiler Parity Hardening
> JavaScript Decompiler Parity Hardening Implementation Plan

**5. [`docs/plans/2026-03-19-js-decompiler-port.md`](docs/plans/2026-03-19-js-decompiler-port.md)** — 2026 03 19 Js Decompiler Port
> Atipicial Decompiler JavaScript Port Implementation Plan

**6. [`docs/plans/2026-03-19-web-npm-package.md`](docs/plans/2026-03-19-web-npm-package.md)** — 2026 03 19 Web Npm Package
> Atipicial Decompiler Web NPM Package Implementation Plan

**7. [`docs/plans/2026-03-19-web-wasm-bindings.md`](docs/plans/2026-03-19-web-wasm-bindings.md)** — 2026 03 19 Web Wasm Bindings
> Atipicial Decompiler Web/WASM Implementation Plan

**8. [`docs/schema/README.md`](docs/schema/README.md)** — Readme
> JSON Schemas

**9. [`docs/superpowers/plans/2026-06-24-advanced-decompiler-phase-0-1.md`](docs/superpowers/plans/2026-06-24-advanced-decompiler-phase-0-1.md)** — 2026 06 24 Advanced Decompiler Phase 0 1
> Advanced Decompiler Evolution — Phase 0 + 1 Implementation Plan

**10. [`docs/superpowers/plans/2026-06-24-ssa-slot-modeling.md`](docs/superpowers/plans/2026-06-24-ssa-slot-modeling.md)** — 2026 06 24 Ssa Slot Modeling
> SSA Over Named Slots — Implementation Plan

**11. [`docs/superpowers/plans/2026-06-24-structured-ir-per-method.md`](docs/superpowers/plans/2026-06-24-structured-ir-per-method.md)** — 2026 06 24 Structured Ir Per Method
> Per-Method Structured IR + Contract Envelope — Implementation Plan

**12. [`docs/superpowers/plans/2026-06-27-simplification-quick-wins.md`](docs/superpowers/plans/2026-06-27-simplification-quick-wins.md)** — 2026 06 27 Simplification Quick Wins
> Implementation Plan: Architecture & Code Simplification — Quick Wins

**13. [`docs/superpowers/plans/2026-07-10-structured-ir-call-temp-elision.md`](docs/superpowers/plans/2026-07-10-structured-ir-call-temp-elision.md)** — 2026 07 10 Structured Ir Call Temp Elision
> Structured IR Call-Temp Elision Implementation Plan

**14. [`docs/superpowers/plans/2026-07-10-structured-ir-method-contracts.md`](docs/superpowers/plans/2026-07-10-structured-ir-method-contracts.md)** — 2026 07 10 Structured Ir Method Contracts
> Structured IR Method Contracts Implementation Plan

**15. [`docs/superpowers/plans/2026-07-10-structured-ir-syscalls.md`](docs/superpowers/plans/2026-07-10-structured-ir-syscalls.md)** — 2026 07 10 Structured Ir Syscalls
> Structured IR Syscall Lifting Implementation Plan

**16. [`docs/superpowers/plans/2026-07-11-shared-method-contract-analysis.md`](docs/superpowers/plans/2026-07-11-shared-method-contract-analysis.md)** — 2026 07 11 Shared Method Contract Analysis
> Shared Method Contract Analysis Implementation Plan

**17. [`docs/superpowers/plans/2026-07-11-structured-collection-mutations.md`](docs/superpowers/plans/2026-07-11-structured-collection-mutations.md)** — 2026 07 11 Structured Collection Mutations
> Structured Collection Mutation Lifting Implementation Plan

**18. [`docs/superpowers/plans/2026-07-11-structured-ir-csharp-body-migration.md`](docs/superpowers/plans/2026-07-11-structured-ir-csharp-body-migration.md)** — 2026 07 11 Structured Ir Csharp Body Migration
> Structured IR C# Body Migration Implementation Plan

**19. [`docs/superpowers/plans/2026-07-11-structured-ir-phi-lowering.md`](docs/superpowers/plans/2026-07-11-structured-ir-phi-lowering.md)** — 2026 07 11 Structured Ir Phi Lowering
> Structured IR Phi Lowering Implementation Plan

**20. [`docs/superpowers/reviews/2026-06-24-codebase-review.md`](docs/superpowers/reviews/2026-06-24-codebase-review.md)** — 2026 06 24 Codebase Review
> Atipicial Decompiler — Codebase Review (2026-06-24)

**21. [`docs/superpowers/reviews/2026-07-12-structured-csharp-cutover.md`](docs/superpowers/reviews/2026-07-12-structured-csharp-cutover.md)** — 2026 07 12 Structured Csharp Cutover
> Structured C# Cutover Review

**22. [`docs/superpowers/specs/2026-06-24-advanced-decompiler-design.md`](docs/superpowers/specs/2026-06-24-advanced-decompiler-design.md)** — 2026 06 24 Advanced Decompiler Design
> Atipicial Decompiler — Advanced Decompiler Evolution

**23. [`docs/superpowers/specs/2026-06-24-ssa-slot-modeling-design.md`](docs/superpowers/specs/2026-06-24-ssa-slot-modeling-design.md)** — 2026 06 24 Ssa Slot Modeling Design
> Atipicial Decompiler — SSA over Named Slots

**24. [`docs/superpowers/specs/2026-06-24-structured-ir-per-method-design.md`](docs/superpowers/specs/2026-06-24-structured-ir-per-method-design.md)** — 2026 06 24 Structured Ir Per Method Design
> Per-Method Structured IR + Contract Envelope

**25. [`docs/superpowers/specs/2026-06-27-simplification-quick-wins-design.md`](docs/superpowers/specs/2026-06-27-simplification-quick-wins-design.md)** — 2026 06 27 Simplification Quick Wins Design
> Architecture & Code Simplification — Quick Wins

**26. [`docs/superpowers/specs/2026-07-10-structured-ir-call-temp-elision-design.md`](docs/superpowers/specs/2026-07-10-structured-ir-call-temp-elision-design.md)** — 2026 07 10 Structured Ir Call Temp Elision Design
> Structured IR Call-Temp Elision Design

**27. [`docs/superpowers/specs/2026-07-10-structured-ir-syscalls-design.md`](docs/superpowers/specs/2026-07-10-structured-ir-syscalls-design.md)** — 2026 07 10 Structured Ir Syscalls Design
> Structured IR Syscall Lifting Design

**28. [`docs/superpowers/specs/2026-07-11-shared-method-contract-analysis-design.md`](docs/superpowers/specs/2026-07-11-shared-method-contract-analysis-design.md)** — 2026 07 11 Shared Method Contract Analysis Design
> Shared Method Contract Analysis Design

**29. [`docs/superpowers/specs/2026-07-11-structured-collection-mutations-design.md`](docs/superpowers/specs/2026-07-11-structured-collection-mutations-design.md)** — 2026 07 11 Structured Collection Mutations Design
> Structured Collection Mutation Lifting Design

**30. [`docs/superpowers/specs/2026-07-11-structured-ir-csharp-body-migration-design.md`](docs/superpowers/specs/2026-07-11-structured-ir-csharp-body-migration-design.md)** — 2026 07 11 Structured Ir Csharp Body Migration Design
> Structured IR to C# Body Migration Design

**31. [`docs/superpowers/specs/2026-07-11-structured-ir-phi-lowering-design.md`](docs/superpowers/specs/2026-07-11-structured-ir-phi-lowering-design.md)** — 2026 07 11 Structured Ir Phi Lowering Design
> Structured IR Phi Lowering Design


---

## 🤝 Contributing

Read the design laws above. Priorities, in order: deterministic correctness → exact parity → clear boundaries → measured performance → usability. Determinism is not negotiable.

---

<div align="center">

## 👑 FOUNDER

<table>
<tr><td align="center" width="33%">

### **xmoohad**

</td></tr>
<tr><td align="center">

**Founder · Architect · Blockchain Scientist · Computer Programmer**

</td></tr>
<tr><td align="center">

*Atipicial Chain is a sovereign Layer-1 blockchain for smart contracts,
digital assets, and decentralized applications.*

Every crate, every opcode, every line of this repository descends from a
single engineering vision: **build a reliable blockchain platform for
smart contracts, digital assets, and decentralized applications.**

</td></tr>
</table>

`ATC` — Atipicial Coin · `ATD` — AtipicialDollar · addresses begin with **A**

</div>

---

*© Atipicial Chain · Founded by xmoohad · MIT License*
