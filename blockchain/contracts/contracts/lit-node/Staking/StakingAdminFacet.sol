//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { StakingBalancesFacet } from "../StakingBalances/StakingBalancesFacet.sol";
import { StakingFacet } from "./StakingFacet.sol";
import { StakingUtilsLib } from "./StakingUtilsLib.sol";
import { LibStakingStorage } from "./LibStakingStorage.sol";

// import "hardhat/console.sol";

contract StakingAdminFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== Modifiers ========== */

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner())
            revert StakingUtilsLib.CallerNotOwner();
        _;
    }

    modifier onlyOwnerOrDevopsAdmin() {
        if (
            msg.sender != LibDiamond.contractOwner() &&
            msg.sender != s().devopsAdmin
        ) revert StakingUtilsLib.CallerNotOwnerOrDevopsAdmin();
        _;
    }

    /* ========== VIEWS ========== */
    function s()
        internal
        pure
        returns (LibStakingStorage.StakingStorage storage)
    {
        return LibStakingStorage.getStorage();
    }

    function views() internal view returns (StakingViewsFacet) {
        return StakingViewsFacet(address(this));
    }

    function mutableEpoch()
        internal
        view
        returns (LibStakingStorage.Epoch storage)
    {
        return s().epochs[0];
    }

    function mutableConfig()
        internal
        view
        returns (LibStakingStorage.Config storage)
    {
        return s().configs[0];
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function setEpochLength(uint256 newEpochLength) external onlyOwner {
        mutableEpoch().epochLength = newEpochLength;
        emit EpochLengthSet(newEpochLength);
    }

    function setEpochTimeout(uint256 newEpochTimeout) external onlyOwner {
        mutableEpoch().timeout = newEpochTimeout;
        emit EpochTimeoutSet(newEpochTimeout);
    }

    function setEpochEndTime(uint256 newEpochEndTime) external onlyOwner {
        mutableEpoch().endTime = newEpochEndTime;
        emit EpochEndTimeSet(newEpochEndTime);
    }

    function setContractResolver(
        address newResolverAddress
    ) external onlyOwner {
        s().contractResolver = ContractResolver(newResolverAddress);
        emit ResolverContractAddressSet(newResolverAddress);
    }

    function setKickPenaltyPercent(
        uint256 reason,
        uint256 newKickPenaltyPercent
    ) external onlyOwner {
        s()
            .complaintReasonToConfig[reason]
            .kickPenaltyPercent = newKickPenaltyPercent;
        emit KickPenaltyPercentSet(reason, newKickPenaltyPercent);
    }

    function setEpochState(
        LibStakingStorage.States newState
    ) external onlyOwnerOrDevopsAdmin {
        s().state = newState;
        emit StakingUtilsLib.StateChanged(newState);
    }

    function adminKickValidatorInNextEpoch(
        address validatorStakerAddress
    ) external onlyOwnerOrDevopsAdmin {
        // block them from rejoining the next epoch
        s().validatorsKickedFromNextEpoch.add(validatorStakerAddress);

        // remove from the validator set
        StakingUtilsLib.removeValidatorFromNextEpoch(
            s(),
            validatorStakerAddress
        );

        // if they're in the current set, we need to mark them as kicked from the current set too
        bool isValidatorInCurrentSet = s().validatorsInCurrentEpoch.contains(
            validatorStakerAddress
        );
        if (isValidatorInCurrentSet) {
            s().currentValidatorsKickedFromNextEpoch.add(
                validatorStakerAddress
            );
        }
        emit StakingUtilsLib.ValidatorKickedFromNextEpoch(
            validatorStakerAddress,
            0
        );

        // // if we're in the locked state, then we need to unlock, because we kicked a node
        if (
            s().state == LibStakingStorage.States.NextValidatorSetLocked ||
            s().state == LibStakingStorage.States.ReadyForNextEpoch
        ) {
            StakingUtilsLib.unlockEpoch(s());
        }
    }

    function adminSlashValidator(
        address validatorStakerAddress,
        uint256 amountToPenalize
    ) external onlyOwner {
        StakingBalancesFacet stakingBalances = StakingBalancesFacet(
            views().getStakingBalancesAddress()
        );
        stakingBalances.penalizeTokens(
            amountToPenalize,
            validatorStakerAddress
        );
    }

    function adminRejoinValidator(
        address staker
    ) external onlyOwnerOrDevopsAdmin {
        if (
            !(s().state == LibStakingStorage.States.Active ||
                s().state == LibStakingStorage.States.Unlocked ||
                s().state == LibStakingStorage.States.Paused)
        ) {
            revert StakingUtilsLib.MustBeInActiveOrUnlockedOrPausedState(
                s().state
            );
        }

        // remove from next validator kicked list
        s().validatorsKickedFromNextEpoch.remove(staker);
        // remove from current validator kicked list
        s().currentValidatorsKickedFromNextEpoch.remove(staker);
        // add to next validator set
        s().validatorsInNextEpoch.add(staker);
        emit ValidatorRejoinedNextEpoch(staker);
    }

    function setConfig(
        LibStakingStorage.Config memory newConfig
    ) external onlyOwner {
        require(
            newConfig.minTripleCount <= newConfig.maxTripleCount,
            "min triple count must be less than or equal to max triple count"
        );

        mutableConfig().tokenRewardPerTokenPerEpoch = newConfig
            .tokenRewardPerTokenPerEpoch;
        mutableConfig().keyTypes = newConfig.keyTypes;
        mutableConfig().minimumValidatorCount = newConfig.minimumValidatorCount;
        mutableConfig().maxConcurrentRequests = newConfig.maxConcurrentRequests;
        mutableConfig().maxTripleCount = newConfig.maxTripleCount;
        mutableConfig().minTripleCount = newConfig.minTripleCount;
        mutableConfig().peerCheckingIntervalSecs = newConfig
            .peerCheckingIntervalSecs;
        mutableConfig().maxTripleConcurrency = newConfig.maxTripleConcurrency;
        mutableConfig().rpcHealthcheckEnabled = newConfig.rpcHealthcheckEnabled;

        emit ConfigSet(
            newConfig.tokenRewardPerTokenPerEpoch,
            newConfig.keyTypes,
            newConfig.minimumValidatorCount,
            newConfig.maxConcurrentRequests,
            newConfig.maxTripleCount,
            newConfig.minTripleCount,
            newConfig.peerCheckingIntervalSecs,
            newConfig.maxTripleConcurrency,
            newConfig.rpcHealthcheckEnabled
        );
    }

    function setComplaintConfig(
        uint256 reason,
        LibStakingStorage.ComplaintConfig memory config
    ) external onlyOwner {
        s().complaintReasonToConfig[reason] = config;
    }

    function adminResetEpoch() external onlyOwner {
        require(s().env == ContractResolver.Env.Dev, "only for dev env");

        // clear out validators in current epoch
        StakingUtilsLib.clearEnumerableAddressSet(s().validatorsInCurrentEpoch);

        // clear out validators in next epoch
        StakingUtilsLib.clearEnumerableAddressSet(s().validatorsInNextEpoch);

        // clear out the validators kicked from next epoch
        StakingUtilsLib.clearEnumerableAddressSet(
            s().validatorsKickedFromNextEpoch
        );

        // clear out the current validators kicked from next epoch
        StakingUtilsLib.clearEnumerableAddressSet(
            s().currentValidatorsKickedFromNextEpoch
        );

        // reset the epoch
        mutableEpoch().number = 1;
        mutableEpoch().endTime = block.timestamp + mutableEpoch().epochLength;
        mutableEpoch().retries = 0;

        // reset the state
        s().state = LibStakingStorage.States.Paused;
        emit StakingUtilsLib.StateChanged(s().state);
    }

    function adminStakeForValidator(
        address staker,
        uint256 amount
    ) external onlyOwner {
        StakingBalancesFacet stakingBalances = StakingBalancesFacet(
            views().getStakingBalancesAddress()
        );
        stakingBalances.stakeForValidator(amount, staker, msg.sender);
    }

    function setDevopsAdmin(address newDevopsAdmin) external onlyOwner {
        s().devopsAdmin = newDevopsAdmin;
        emit DevopsAdminSet(newDevopsAdmin);
    }

    function emitClearOfflinePhaseData(uint256 dataType) external onlyOwner {
        emit ClearOfflinePhaseData(dataType);
    }

    function emitCountOfflinePhaseData(uint256 dataType) external onlyOwner {
        emit CountOfflinePhaseData(dataType);
    }

    /* ========== EVENTS ========== */

    event EpochLengthSet(uint256 newEpochLength);
    event EpochTimeoutSet(uint256 newEpochTimeout);
    event EpochEndTimeSet(uint256 newEpochEndTime);
    event StakingTokenSet(address newStakingTokenAddress);
    event KickPenaltyPercentSet(uint256 reason, uint256 newKickPenaltyPercent);
    event ResolverContractAddressSet(address newResolverContractAddress);
    event ConfigSet(
        uint256 newTokenRewardPerTokenPerEpoch,
        uint256[] newKeyTypes,
        uint256 newMinimumValidatorCount,
        uint256 newMaxConcurrentRequests,
        uint256 newMaxTripleCount,
        uint256 newMinTripleCount,
        uint256 newPeerCheckingIntervalSecs,
        uint256 newMaxTripleConcurrency,
        bool newRpcHealthcheckEnabled
    );
    event ComplaintConfigSet(
        uint256 reason,
        LibStakingStorage.ComplaintConfig config
    );
    event ValidatorRejoinedNextEpoch(address staker);
    event DevopsAdminSet(address newDevopsAdmin);
    event ClearOfflinePhaseData(uint256 dataType);
    event CountOfflinePhaseData(uint256 dataType);
}
