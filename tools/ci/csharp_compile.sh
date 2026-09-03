#!/usr/bin/env bash
set -euo pipefail

command -v dotnet >/dev/null || { echo "dotnet is required" >&2; exit 1; }
: "${ATC_SMARTCONTRACT_FRAMEWORK_DLL:?ATC_SMARTCONTRACT_FRAMEWORK_DLL is required}"

if [[ ! -f "$ATC_SMARTCONTRACT_FRAMEWORK_DLL" ]]; then
    echo "ATC_SMARTCONTRACT_FRAMEWORK_DLL does not point to a file: $ATC_SMARTCONTRACT_FRAMEWORK_DLL" >&2
    exit 1
fi

if [[ -n "${ATC_CSHARP_TARGET_FRAMEWORK:-}" ]] &&
    [[ ! "$ATC_CSHARP_TARGET_FRAMEWORK" =~ ^net[[:alnum:].]+$ ]]; then
    echo "ATC_CSHARP_TARGET_FRAMEWORK must be a target moniker such as net8.0" >&2
    exit 1
fi

cargo test --locked --test csharp_compile -- --ignored --nocapture
