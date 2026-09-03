<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial Decompiler Web Package

This folder contains the publishable npm package for the Rust crate compiled to
WebAssembly. It adds a thin TypeScript wrapper over the wasm bindings so browser
code gets a stable, typed API without duplicating the decompiler logic.

## Build

From this directory:

```bash
npm install
npm run build:wasm
```

That runs `wasm-pack build .. --target web --out-dir web/dist/pkg --features web --no-default-features`
followed by `scripts/prepare-wasm-package.mjs`, generating the wasm glue under
`web/dist/pkg/`.

Then build the TypeScript wrapper:

```bash
npm run build:ts
```

For a full package build:

```bash
npm run build:package
```

## Serve

From this directory:

```bash
python3 -m http.server 4173
```

Then open `http://localhost:4173`.

## JS API

```js
import {
  init,
  initPanicHook,
  infoReport,
  disasmReport,
  decompileReport,
} from "atipicial-decompiler-web";

await init();
initPanicHook();

const info = infoReport(aefBytes, {
  manifestJson,
  strictManifest: false,
});

const disasm = disasmReport(aefBytes, {
  failOnUnknownOpcodes: false,
});

const decompile = decompileReport(aefBytes, {
  manifestJson,
  strictManifest: false,
  failOnUnknownOpcodes: false,
  inlineSingleUseTemps: true,
  typedDeclarations: true,
  outputFormat: "csharp",
});
```

The decompile report defaults to the generated C# contract. Pass `"all"` when
the intermediate high-level and pseudocode views are also needed. Set
`typedDeclarations: false` to retain compatibility-oriented dynamic/var
declarations.

`aefBytes` should be a `Uint8Array`. `manifestJson` should be a UTF-8 JSON string.

The wrapper accepts camelCase JS options and translates them into the snake_case
ABI expected by the wasm bindings. The published npm tarball includes the
compiled TypeScript wrapper plus the wasm artifacts under `dist/pkg/`.

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
