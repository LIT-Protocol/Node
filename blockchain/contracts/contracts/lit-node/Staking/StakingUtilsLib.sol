//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
library StakingUtilsLib {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== ERRORS ========== */
    error MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
        LibStakingStorage.States state
    );
    error MustBeInActiveOrUnlockedOrPausedState(LibStakingStorage.States state);
    error CallerNotOwner();
    error CallerNotOwnerOrDevopsAdmin();
    error NotEnoughValidatorsInNextEpoch(
        uint256 validatorCount,
        uint256 minimumValidatorCount
    );
    /* ========== INTERNAL FUNCTIONS ========== */

    function clearEnumerableAddressSet(
        EnumerableSet.AddressSet storage set
    ) internal {
        while (set.length() > 0) {
            set.remove(set.at(0));
        }
    }

    function unlockEpoch(LibStakingStorage.StakingStorage storage s) internal {
        // this should only be callable from the ReadyForNextEpoch state or the NextValidatorSetLocked state
        if (
            !(s.state == LibStakingStorage.States.ReadyForNextEpoch ||
                s.state == LibStakingStorage.States.NextValidatorSetLocked)
        ) {
            revert MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
                s.state
            );
        }
        // clear out readyForNextEpoch for current nodes
        uint256 validatorLength = s.validatorsInCurrentEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            s.readyForNextEpoch[s.validatorsInCurrentEpoch.at(i)] = false;
        }

        // clear out readyForNextEpoch for next nodes
        validatorLength = s.validatorsInNextEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            s.readyForNextEpoch[s.validatorsInNextEpoch.at(i)] = false;
        }

        s.state = LibStakingStorage.States.Unlocked;
        s.epochs[0].retries++;
        emit StateChanged(s.state);
    }

    function removeValidatorFromNextEpoch(
        LibStakingStorage.StakingStorage storage s,
        address staker
    ) internal {
        if (s.validatorsInNextEpoch.contains(staker)) {
            // remove them
            s.validatorsInNextEpoch.remove(staker);
        }
        LibStakingStorage.Validator memory validator = s.validators[staker];
        bytes32 commsKeysHash = keccak256(
            abi.encodePacked(validator.senderPubKey, validator.receiverPubKey)
        );
        s.usedCommsKeys[commsKeysHash] = false;
    }

    function checkNextSetAboveThreshold(
        LibStakingStorage.StakingStorage storage s
    ) internal view {
        // never let the network go below 3
        if (
            s.validatorsInNextEpoch.length() <
            s.configs[0].minimumValidatorCount
        ) {
            revert NotEnoughValidatorsInNextEpoch(
                s.validatorsInNextEpoch.length(),
                s.configs[0].minimumValidatorCount
            );
        }

        StakingViewsFacet svf = StakingViewsFacet(address(this));

        // prevent locking if it would drop us below threshold due to kicking.  we require at least `threshold` validators to be active from the current epoch
        if (
            s.epochs[0].number > 1 &&
            (s.validatorsInCurrentEpoch.length() -
                s.currentValidatorsKickedFromNextEpoch.length()) <
            svf.currentValidatorCountForConsensus()
        ) {
            revert NotEnoughValidatorsInNextEpoch(
                s.validatorsInCurrentEpoch.length() -
                    s.currentValidatorsKickedFromNextEpoch.length(),
                svf.currentValidatorCountForConsensus()
            );
        }
    }

    /* ========== EVENTS ========== */

    event StateChanged(LibStakingStorage.States newState);
    event ValidatorKickedFromNextEpoch(
        address indexed staker,
        uint256 amountBurned
    );
}
