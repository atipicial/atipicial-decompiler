<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# TestingArtifacts

This folder is for real-world Atipicial contracts that should be exercised by the artifact decompilation test.

Supported layouts (recursively discovered):
- C# source containing both `ContractManifest.Parse(@...)` and `Convert.FromBase64String(@...)` snippets (matching the official compiler output). The test reads these blobs, writes the AEF/manifest into `TestingArtifacts/decompiled/<relative-path>/`, and verifies the decompilation.
- Paired files named `<contract>.aef` and `<contract>.manifest.json` living side-by-side. These are picked up directly and decompiled in place, preserving the relative directory structure under `decompiled/`.

Expected failures:
- `TestingArtifacts/known_unsupported.txt` is only for valid contracts that reach a known decompiler limitation. `info`, `disasm`, and `tokens` must still succeed; `decompile` must fail.
- `TestingArtifacts/expected_invalid.txt` is for intentionally malformed inputs. Every parser-backed CLI command must reject these artifacts. `CallFlagInvalid`, for example, contains call flags `0x10` outside the allowed `0x0F` mask.
- Both registries use one artifact name per line, allow `#` comments, and accept an optional expected error substring after `:`. Registered failures produce a non-empty `<contract>.error.txt` under `TestingArtifacts/decompiled/`; stale entries and unexpected successes fail the tests.

Outputs:
- All generated files mirror the source layout under `TestingArtifacts/decompiled/`, which is already git-ignored.

Contributor workflow and CI behavior are documented in `docs/testing-artifacts.md`.

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
