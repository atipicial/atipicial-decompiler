// Copyright (C) 2015-2026 The Atipicial Project.
//
// Role.cs file belongs to the atipicial project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

namespace Atipicial.SmartContract.Native
{
    /// <summary>
    /// Represents the roles in the ATC system.
    /// </summary>
    public enum Role : byte
    {
        /// <summary>
        /// The validators of state. Used to generate and sign the state root.
        /// </summary>
        StateValidator = 4,

        /// <summary>
        /// The nodes used to process Oracle requests.
        /// </summary>
        Oracle = 8,

        /// <summary>
        /// AtipicialFs Alphabet nodes.
        /// </summary>
        AtipicialFsAlphabetNode = 16,

        /// <summary>
        /// P2P Notary nodes used to process P2P notary requests.
        /// </summary>
        P2PNotary = 32
    }
}
