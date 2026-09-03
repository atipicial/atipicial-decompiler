// Copyright (C) 2015-2026 The Atipicial Project.
//
// WhitelistedContract.cs file belongs to the atipicial project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Atipicial.Extensions;
using Atipicial.VM;
using Atipicial.VM.Types;

namespace Atipicial.SmartContract.Native
{
    class WhitelistedContract : IInteroperable
    {
        public required UInt160 ContractHash { get; set; }
        public required string Method { get; set; }
        public required int ArgCount { get; set; }
        public required long FixedFee { get; set; }

        public virtual void FromStackItem(StackItem stackItem)
        {
            var data = (Struct)stackItem;

            ContractHash = new UInt160(data[0].GetSpan());
            Method = data[1].GetString()!;
            ArgCount = (int)data[2].GetInteger();
            FixedFee = (long)data[3].GetInteger();
        }

        public virtual StackItem ToStackItem(IReferenceCounter? referenceCounter)
        {
            return new Struct(referenceCounter) { ContractHash.ToArray(), Method, ArgCount, FixedFee };
        }
    }
}
