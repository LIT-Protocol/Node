// SPDX-License-Identifier: MIT

import { LibBackupRecoveryStorage } from "./LibBackupRecoveryStorage.sol";
import { StakingViewsFacet } from "./../Staking/StakingViewsFacet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

contract BackupRecoveryViewFacet {
    function getNonSubmitingBackupMembersInNextState()
        public
        view
        returns (address[] memory missingRecoveryMembers)
    {
        LibBackupRecoveryStorage.NextState
            storage nextState = LibBackupRecoveryStorage.getStorage().nextState[
                0
            ];

        // Alloc the size of the whole backup party set on the stack
        // this allows us to not use sotrage and keep the method view
        // the returned array will contained zeroized elements for the indexes in
        // the array which map to the parties position in their storage array.
        address[] memory nonRegisteredPartyMembers = new address[](
            nextState.partyMembers.length
        );
        for (uint256 i = 0; i < nextState.partyMembers.length; i++) {
            if (!nextState.keysRecieved[nextState.partyMembers[i]]) {
                nonRegisteredPartyMembers[i] = nextState.partyMembers[i];
            }
        }

        return nonRegisteredPartyMembers;
    }
}
