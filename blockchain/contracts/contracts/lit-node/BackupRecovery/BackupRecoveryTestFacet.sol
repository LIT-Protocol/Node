// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { LibBackupRecoveryStorage } from "./LibBackupRecoveryStorage.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import "hardhat/console.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

contract BackupRecoveryTestFacet {
    function s()
        internal
        pure
        returns (LibBackupRecoveryStorage.BackupRecoveryStorage storage)
    {
        return LibBackupRecoveryStorage.getStorage();
    }

    /**
     * @dev
     * Sets the contract state by registering a new set of backup members
     */
    function setBackupPartyState(
        bytes calldata bls12381G1EncKey,
        bytes calldata secp256K1EcdsaPubKey,
        address[] calldata partyMembers
    ) public {
        require(
            partyMembers.length > 1,
            "BackupRecovery: cannot vote for empty party set"
        );

        // Set a mock state
        s().recoveryState[0].bls12381G1EncKey = bls12381G1EncKey;
        s().recoveryState[0].secp256K1EcdsaPubKey = secp256K1EcdsaPubKey;
        if (partyMembers.length == 2) {
            s().recoveryState[0].partyThreshold = 2;
        } else {
            s().recoveryState[0].partyThreshold = (partyMembers.length * 2) / 3;
        }
        s().recoveryState[0].sessionId = "Test_session_id";
        s().recoveryState[0].partyMembers = partyMembers;
    }
}
