// Copyright (C) 2015-2026 The Atipicial Project.
//
// Treasury.cs file belongs to the atipicial project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

#nullable enable
#pragma warning disable IDE0051

using Atipicial.SmartContract.Manifest;
using Atipicial.VM.Types;
using System.Collections.Immutable;
using System.Numerics;

namespace Atipicial.SmartContract.Native
{
    /// <summary>
    /// The Treasury native contract used for manage the treasury funds.
    /// </summary>
    public sealed class Treasury : NativeContract
    {
        internal Treasury() : base() { }

        public override ImmutableHashSet<Hardfork?> Activations => [Hardfork.HF_Faun];

        protected override void OnManifestCompose(IsHardforkEnabledDelegate hfChecker, uint blockHeight, ContractManifest manifest)
        {
            manifest.SupportedStandards = ["AEP-26", "AEP-27", "AEP-30"];
        }

        /// <summary>
        /// Verify checks whether the transaction is signed by the committee.
        /// </summary>
        /// <param name="engine">ApplicationEngine</param>
        /// <returns>Whether transaction is valid.</returns>
        [ContractMethod(CpuFee = 1 << 5, RequiredCallFlags = CallFlags.ReadStates)]
        private bool Verify(ApplicationEngine engine) => CheckCommittee(engine);

        /// <summary>
        /// OnAEP17Payment callback.
        /// </summary>
        /// <param name="engine">ApplicationEngine</param>
        /// <param name="from">GAS sender</param>
        /// <param name="amount">The amount of GAS sent</param>
        /// <param name="data">Optional data</param>
        [ContractMethod(CpuFee = 1 << 5, RequiredCallFlags = CallFlags.None)]
        private void OnAEP17Payment(ApplicationEngine engine, UInt160 from, BigInteger amount, StackItem data) { }

        /// <summary>
        /// OnAEP11Payment callback.
        /// </summary>
        /// <param name="engine">ApplicationEngine</param>
        /// <param name="from">GAS sender</param>
        /// <param name="amount">The amount of GAS sent</param>
        /// <param name="tokenId">Aep11 token Id</param>
        /// <param name="data">Optional data</param>
        [ContractMethod(CpuFee = 1 << 5, RequiredCallFlags = CallFlags.None)]
        private void OnAEP11Payment(ApplicationEngine engine, UInt160 from, BigInteger amount, byte[] tokenId, StackItem data) { }
    }
}

#nullable disable
