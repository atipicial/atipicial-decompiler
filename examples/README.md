<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Examples

This directory collects small contracts that you can compile with the official
Atipicial C# compiler (`nccs`) and then feed into the `atipicial-decompiler` CLI. The
examples are intentionally minimal so you can inspect the resulting AEF payloads
and manifests without needing a full build system.

## Prerequisites

Install the C# compiler as a .NET global tool (requires the .NET SDK 6.0+):

```bash
dotnet tool install -g Atipicial.Compiler.CSharp
```

The command installs the `nccs` executable and makes it available on your
`PATH`. Verify the installation by running `nccs --help`.

## hello_world

`hello_world/HelloWorld.cs` contains a small contract that returns a string and
exposes a `Notify` method for demonstration purposes.

```bash
# Compile the contract to AEF and manifest files
nccs compile \
  examples/hello_world/HelloWorld.cs \
  --aef build/HelloWorld.aef \
  --manifest build/HelloWorld.manifest.json

# Inspect the output with the decompiler (auto-detects the manifest)
atipicial-decompiler decompile build/HelloWorld.aef

# Fail fast on unknown opcodes (default is tolerant)
atipicial-decompiler decompile --fail-on-unknown-opcodes build/HelloWorld.aef
```

The `decompile` command prints both the reconstructed high-level contract view
and the instruction listing. You can swap `decompile` for `info`, `disasm`, or
`tokens` if you want to target a specific portion of the tooling.

Feel free to copy the example contract, add additional methods, and re-run the
steps above to explore how the bytecode evolves.

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
