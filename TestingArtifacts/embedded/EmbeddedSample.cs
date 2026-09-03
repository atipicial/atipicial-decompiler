using System;
using System.Numerics;
using Atipicial.SmartContract.Framework;
using Atipicial.SmartContract.Framework.Services;

public class EmbeddedSample : SmartContract
{
    // Artifact generated for decompiler tests
    private static readonly ContractManifest Manifest = ContractManifest.Parse(@"{
    "name": "EmbeddedSample",
    "groups": [],
    "features": {
        "storage": false,
        "payable": false
    },
    "supportedstandards": [],
    "abi": {
        "methods": [
            {
                "name": "main",
                "parameters": [],
                "returntype": "Integer",
                "offset": 0,
                "safe": false
            }
        ],
        "events": []
    },
    "permissions": [],
    "trusts": "*"
}");

    private static readonly byte[] AefBytes = Convert.FromBase64String(@"TkVGM2NzX18AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIRQJRZrG4=");

    public static int Main()
    {
        return 1;
    }
}
