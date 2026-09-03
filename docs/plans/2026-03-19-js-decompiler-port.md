<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial Decompiler JavaScript Port Implementation Plan

> **For Implementer:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Status:** Completed (shipped in v0.6.0)

**Goal:** Build a real plain-JavaScript decompiler package that parses Atipicial AEF files, disassembles bytecode, and emits the same pseudocode listing as the Rust core.

**Architecture:** Start with the stable low-level layers from the Rust codebase: AEF parsing, method-token decoding, opcode metadata, operand decoding, and pseudocode rendering. Package the port as a standalone ESM/npm module under `js/`, with tests that compare JS behavior against the same sample AEF fixtures already used by Rust.

**Tech Stack:** JavaScript (ESM), Node test runner, npm package tooling

### Task 1: Add a failing JS package test

**Files:**
- Create: `js/package.json`
- Create: `js/test/decompiler.test.mjs`

**Step 1: Write the failing test**

```js
import { decompileBytes } from "../src/index.js";

test("decompiles a sample AEF into pseudocode", () => {
  const result = decompileBytes(sampleBytes);
  assert.match(result.pseudocode, /0000: PUSH1/);
});
```

**Step 2: Run test to verify it fails**

Run: `cd js && npm test`
Expected: FAIL because the JS package does not exist yet.

### Task 2: Add generated opcode metadata for JS

**Files:**
- Create: `js/scripts/generate-opcodes.mjs`
- Create: `js/src/generated/opcodes.js`

**Step 1: Generate a JS opcode table from the same source used by Rust**

```js
export const OPCODES = new Map([[0x10, { mnemonic: "PUSH0", operandEncoding: "None" }]]);
```

**Step 2: Re-run targeted tests**

Run: `cd js && npm test`
Expected: still FAIL, but module resolution should advance.

### Task 3: Port the AEF parser

**Files:**
- Create: `js/src/aef.js`
- Create: `js/src/errors.js`
- Create: `js/src/util.js`

**Step 1: Port varint/string/bytes decoding and checksum verification**

```js
export function parseAef(bytes) { ... }
```

**Step 2: Re-run targeted tests**

Run: `cd js && npm test`
Expected: fail later in disassembly/pseudocode if parser works.

### Task 4: Port the disassembler

**Files:**
- Create: `js/src/disassembler.js`
- Create: `js/src/pseudocode.js`

**Step 1: Decode instructions and warnings**

```js
export function disassemble(bytecode, options) { ... }
```

**Step 2: Render pseudocode**

```js
export function renderPseudocode(instructions) { ... }
```

### Task 5: Expose a package entrypoint

**Files:**
- Create: `js/src/index.js`
- Modify: `js/package.json`
- Modify: `README.md`

**Step 1: Export the JS API**

```js
export { parseAef, disassemble, decompileBytes };
```

**Step 2: Verify**

Run: `cd js && npm test`
Expected: PASS

Run: `cargo test --features web --test web_api -q`
Expected: PASS

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
