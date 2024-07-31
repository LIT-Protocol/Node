//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { StakingViewsFacet } from "./StakingViewsFacet.sol";
import { StakingBalancesFacet } from "../StakingBalances/StakingBalancesFacet.sol";

import { LibStakingStorage } from "./LibStakingStorage.sol";

// import "hardhat/console.sol";

contract StakingFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    // errors
    error MustBeInActiveOrUnlockedState(LibStakingStorage.States state);
    error MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
        LibStakingStorage.States state
    );
    error MustBeInNextValidatorSetLockedState(LibStakingStorage.States state);
    error MustBeInReadyForNextEpochState(LibStakingStorage.States state);
    error MustBeInActiveOrUnlockedOrPausedState(LibStakingStorage.States state);
    error MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
        LibStakingStorage.States state
    );
    error NotEnoughValidatorsInNextEpoch(
        uint256 validatorCount,
        uint256 minimumValidatorCount
    );
    error ValidatorIsNotInNextEpoch(
        address validator,
        address[] validatorsInNextEpoch
    );
    error NotEnoughValidatorsReadyForNextEpoch(
        uint256 currentReadyValidatorCount,
        uint256 nextReadyValidatorCount,
        uint256 minimumValidatorCountToBeReady
    );
    error CannotKickBelowCurrentValidatorThreshold();
    error CannotStakeZero();
    error CannotRejoinUntilNextEpochBecauseKicked(address stakingAddress);
    error ActiveValidatorsCannotLeave();
    error TryingToWithdrawMoreThanStaked(
        uint256 yourBalance,
        uint256 requestedWithdrawlAmount
    );
    error CouldNotMapNodeAddressToStakerAddress(address nodeAddress);
    error MustBeValidatorInNextEpochToKick(address stakerAddress);
    error CannotVoteTwice(address stakerAddress);
    error NotEnoughTimeElapsedSinceLastEpoch(
        uint256 currentTimestamp,
        uint256 epochEndTime
    );
    error NotEnoughTimeElapsedForTimeoutSinceLastEpoch(
        uint256 currentTimestamp,
        uint256 epochEndTime,
        uint256 timeout
    );
    error CannotWithdrawZero();
    error CannotReuseCommsKeys(uint256 senderPubKey, uint256 receiverPubKey);
    error StakerNotPermitted(address stakerAddress);
    error SignaledReadyForWrongEpochNumber(
        uint256 currentEpochNumber,
        uint256 receivedEpochNumber
    );
    error CallerNotOwner();

    /* ========== Modifiers ========== */

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
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

    /// Lock in the validators for the next epoch
    function lockValidatorsForNextEpoch() external {
        if (block.timestamp < mutableEpoch().endTime) {
            revert NotEnoughTimeElapsedSinceLastEpoch(
                block.timestamp,
                mutableEpoch().endTime
            );
        }
        if (
            !(s().state == LibStakingStorage.States.Active ||
                s().state == LibStakingStorage.States.Unlocked)
        ) {
            revert MustBeInActiveOrUnlockedState(s().state);
        }
        if (
            s().validatorsInNextEpoch.length() <
            mutableConfig().minimumValidatorCount
        ) {
            revert NotEnoughValidatorsInNextEpoch(
                s().validatorsInNextEpoch.length(),
                mutableConfig().minimumValidatorCount
            );
        }

        s().state = LibStakingStorage.States.NextValidatorSetLocked;
        emit StateChanged(s().state);
    }

    /// After proactive secret sharing is complete, the nodes may signal that they are ready for the next epoch.  Note that this function is called by the node itself, and so msg.sender is the nodeAddress and not the stakerAddress.
    function signalReadyForNextEpoch(uint256 epochNumber) external {
        if (mutableEpoch().number != epochNumber) {
            revert SignaledReadyForWrongEpochNumber(
                mutableEpoch().number,
                epochNumber
            );
        }

        address stakerAddress = s().nodeAddressToStakerAddress[msg.sender];
        if (
            !(s().state == LibStakingStorage.States.NextValidatorSetLocked ||
                s().state == LibStakingStorage.States.ReadyForNextEpoch ||
                s().state == LibStakingStorage.States.Restore)
        ) {
            revert MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
                s().state
            );
        }
        // at the first epoch, validatorsInCurrentEpoch is empty
        if (mutableEpoch().number != 1) {
            if (!s().validatorsInNextEpoch.contains(stakerAddress)) {
                revert ValidatorIsNotInNextEpoch(
                    stakerAddress,
                    views().getValidatorsInNextEpoch()
                );
            }
        }
        s().readyForNextEpoch[stakerAddress] = true;
        emit ReadyForNextEpoch(stakerAddress, mutableEpoch().number);

        if (views().isReadyForNextEpoch()) {
            s().state = LibStakingStorage.States.ReadyForNextEpoch;
            emit StateChanged(s().state);
        }
    }

    /// Advance to the next Epoch.  Rewards validators, adds the joiners, and removes the leavers
    function advanceEpoch() external {
        if (block.timestamp < mutableEpoch().endTime) {
            revert NotEnoughTimeElapsedSinceLastEpoch(
                block.timestamp,
                mutableEpoch().endTime
            );
        }
        if (s().state != LibStakingStorage.States.ReadyForNextEpoch) {
            revert MustBeInReadyForNextEpochState(s().state);
        }
        if (!views().isReadyForNextEpoch()) {
            revert NotEnoughValidatorsReadyForNextEpoch(
                views().countOfCurrentValidatorsReadyForNextEpoch(),
                views().countOfNextValidatorsReadyForNextEpoch(),
                views().currentValidatorCountForConsensus()
            );
        }

        // reward the validators
        uint256 validatorLength = s().validatorsInCurrentEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            address validatorAddress = s().validatorsInCurrentEpoch.at(i);
            IERC20Metadata stakingToken = IERC20Metadata(
                views().getTokenAddress()
            );
            StakingBalancesFacet stakingBalances = StakingBalancesFacet(
                views().getStakingBalancesAddress()
            );
            uint256 reward = (mutableConfig().tokenRewardPerTokenPerEpoch *
                stakingBalances.balanceOf(validatorAddress)) /
                10 ** stakingToken.decimals();
            stakingBalances.rewardValidator(reward, validatorAddress);
        }

        // set the validators to the new validator set
        // ideally we could just do this:
        // validatorsInCurrentEpoch = validatorsInNextEpoch;
        // but solidity doesn't allow that, so we have to do it manually

        // clear out validators in current epoch
        clearEnumerableAddressSet(s().validatorsInCurrentEpoch);

        // copy validators from next epoch to current epoch
        validatorLength = s().validatorsInNextEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            s().validatorsInCurrentEpoch.add(s().validatorsInNextEpoch.at(i));

            // clear out readyForNextEpoch
            s().readyForNextEpoch[s().validatorsInNextEpoch.at(i)] = false;
        }

        // clear out the validators kicked from next epoch
        clearEnumerableAddressSet(s().validatorsKickedFromNextEpoch);

        // clear out the current validators kicked from next epoch
        clearEnumerableAddressSet(s().currentValidatorsKickedFromNextEpoch);

        mutableEpoch().number++;
        mutableEpoch().endTime = block.timestamp + mutableEpoch().epochLength;

        s().state = LibStakingStorage.States.Active;
        emit StateChanged(s().state);
    }

    /// Stake and request to join the validator set
    /// @param amount The amount of tokens to stake
    /// @param ip The IP address of the node
    /// @param port The port of the node
    function stakeAndJoin(
        uint256 amount,
        uint32 ip,
        uint128 ipv6,
        uint32 port,
        address nodeAddress,
        uint256 senderPubKey,
        uint256 receiverPubKey
    ) external {
        stake(amount);
        requestToJoin(
            ip,
            ipv6,
            port,
            nodeAddress,
            senderPubKey,
            receiverPubKey
        );
    }

    function stake(uint256 amount) public {
        StakingBalancesFacet stakingBalances = StakingBalancesFacet(
            views().getStakingBalancesAddress()
        );
        stakingBalances.stake(amount, msg.sender);
    }

    function requestToJoin(
        uint32 ip,
        uint128 ipv6,
        uint32 port,
        address nodeAddress,
        uint256 senderPubKey,
        uint256 receiverPubKey
    ) public {
        StakingBalancesFacet stakingBalances = StakingBalancesFacet(
            views().getStakingBalancesAddress()
        );
        stakingBalances.checkStakingAmounts(msg.sender);

        if (
            !(s().state == LibStakingStorage.States.Active ||
                s().state == LibStakingStorage.States.Unlocked ||
                s().state == LibStakingStorage.States.Paused)
        ) {
            revert MustBeInActiveOrUnlockedOrPausedState(s().state);
        }

        // make sure they haven't been kicked
        if (s().validatorsKickedFromNextEpoch.contains(msg.sender)) {
            revert CannotRejoinUntilNextEpochBecauseKicked(msg.sender);
        }

        bytes32 commsKeysHash = keccak256(
            abi.encodePacked(senderPubKey, receiverPubKey)
        );
        if (s().usedCommsKeys[commsKeysHash]) {
            revert CannotReuseCommsKeys(senderPubKey, receiverPubKey);
        }
        s().usedCommsKeys[commsKeysHash] = true;

        if (stakingBalances.permittedStakersOn()) {
            if (!stakingBalances.isPermittedStaker(msg.sender)) {
                revert StakerNotPermitted(msg.sender);
            }
        }

        s().validators[msg.sender].ip = ip;
        s().validators[msg.sender].ipv6 = ipv6;
        s().validators[msg.sender].port = port;
        s().validators[msg.sender].nodeAddress = nodeAddress;
        s().validators[msg.sender].senderPubKey = senderPubKey;
        s().validators[msg.sender].receiverPubKey = receiverPubKey;
        s().nodeAddressToStakerAddress[nodeAddress] = msg.sender;

        s().validatorsInNextEpoch.add(msg.sender);

        emit RequestToJoin(msg.sender);
    }

    /// Withdraw staked tokens.  This can only be done by users who are not active in the validator set.
    /// @param amount The amount of tokens to withdraw
    function withdraw(uint256 amount) external {
        StakingBalancesFacet stakingBalances = StakingBalancesFacet(
            views().getStakingBalancesAddress()
        );
        stakingBalances.withdraw(amount, msg.sender);
    }

    /// Request to leave in the next Epoch
    function requestToLeave() external {
        stakerRequestToLeave(msg.sender);
    }

    function requestToLeaveAsNode() external {
        address stakerAddress = s().nodeAddressToStakerAddress[msg.sender];
        if (stakerAddress == address(0)) {
            revert CouldNotMapNodeAddressToStakerAddress(msg.sender);
        }
        stakerRequestToLeave(stakerAddress);
    }

    function stakerRequestToLeave(address staker) internal {
        if (
            !(s().state == LibStakingStorage.States.Active ||
                s().state == LibStakingStorage.States.Unlocked ||
                s().state == LibStakingStorage.States.Paused)
        ) {
            revert MustBeInActiveOrUnlockedOrPausedState(s().state);
        }
        if (
            s().validatorsInNextEpoch.length() - 1 <
            mutableConfig().minimumValidatorCount
        ) {
            revert NotEnoughValidatorsInNextEpoch(
                s().validatorsInNextEpoch.length(),
                mutableConfig().minimumValidatorCount
            );
        }
        removeValidatorFromNextEpoch(staker);
        emit RequestToLeave(staker);
    }

    /// Transfer any outstanding reward tokens
    function getReward() external {
        StakingBalancesFacet stakingBalances = StakingBalancesFacet(
            views().getStakingBalancesAddress()
        );
        stakingBalances.getReward(msg.sender);
    }

    /// Exit staking and get any outstanding rewards
    function exit() external {
        StakingBalancesFacet stakingBalances = StakingBalancesFacet(
            views().getStakingBalancesAddress()
        );
        stakingBalances.withdraw(
            stakingBalances.balanceOf(msg.sender),
            msg.sender
        );
        stakingBalances.getReward(msg.sender);
    }

    /// If more than the threshold of validators vote to kick someone, kick them.
    /// It's expected that this will be called by the node directly, so msg.sender will be the nodeAddress
    function kickValidatorInNextEpoch(
        address validatorStakerAddress,
        uint256 reason,
        bytes calldata data
    ) external {
        address stakerAddressOfSender = s().nodeAddressToStakerAddress[
            msg.sender
        ];
        if (stakerAddressOfSender == address(0)) {
            revert CouldNotMapNodeAddressToStakerAddress(msg.sender);
        }
        if (!s().validatorsInNextEpoch.contains(stakerAddressOfSender)) {
            revert MustBeValidatorInNextEpochToKick(stakerAddressOfSender);
        }
        if (
            s()
            .votesToKickValidatorsInNextEpoch[mutableEpoch().number][
                validatorStakerAddress
            ].voted[stakerAddressOfSender]
        ) {
            revert CannotVoteTwice(stakerAddressOfSender);
        }

        // A threshold number of validators from the current validator set MUST NOT
        // be kicked in order for DKG resharing to be successful.
        // This is only valid for epoch 2+ since epoch 1 has no current validator set,
        // and if we enforce this in epoch 1, we are effectively prohibiting any votes
        // to kick.
        bool isValidatorInCurrentSet = s().validatorsInCurrentEpoch.contains(
            validatorStakerAddress
        );
        if (
            views().epoch().number > 1 &&
            s().currentValidatorsKickedFromNextEpoch.length() >=
            (views().getValidatorsInCurrentEpoch().length -
                views().currentValidatorCountForConsensus())
        ) {
            revert CannotKickBelowCurrentValidatorThreshold();
        }

        // Vote to kick
        s()
        .votesToKickValidatorsInNextEpoch[mutableEpoch().number][
            validatorStakerAddress
        ].votes++;
        s()
        .votesToKickValidatorsInNextEpoch[mutableEpoch().number][
            validatorStakerAddress
        ].voted[stakerAddressOfSender] = true;

        if (
            s().validatorsInNextEpoch.contains(validatorStakerAddress) &&
            views().shouldKickValidator(validatorStakerAddress)
        ) {
            // remove them from the validator set
            removeValidatorFromNextEpoch(validatorStakerAddress);
            // block them from rejoining the next epoch
            s().validatorsKickedFromNextEpoch.add(validatorStakerAddress);
            // mark them if they are in the current validator set
            if (isValidatorInCurrentSet) {
                s().currentValidatorsKickedFromNextEpoch.add(
                    validatorStakerAddress
                );
            }

            // slash the stake
            uint256 kickPenaltyPercent = s()
                .complaintReasonToConfig[reason]
                .kickPenaltyPercent;

            StakingBalancesFacet stakingBalances = StakingBalancesFacet(
                views().getStakingBalancesAddress()
            );
            uint256 amountToPenalize = (stakingBalances.balanceOf(
                validatorStakerAddress
            ) * kickPenaltyPercent) / 100;

            stakingBalances.penalizeTokens(
                amountToPenalize,
                validatorStakerAddress
            );

            // shame them with an event
            emit ValidatorKickedFromNextEpoch(
                validatorStakerAddress,
                amountToPenalize
            );

            // if we're in the locked state, then we need to unlock, because we kicked a node
            if (
                s().state == LibStakingStorage.States.NextValidatorSetLocked ||
                s().state == LibStakingStorage.States.ReadyForNextEpoch
            ) {
                unlockEpoch();
            } else if (s().state == LibStakingStorage.States.Active) {
                // if we're in the active state, then we need to lock, because we kicked a node
                // we want to kick off the next epoch transition to remove this node from the set
                s().state = LibStakingStorage.States.NextValidatorSetLocked;
                emit StateChanged(s().state);
                // change the epoch end time to now
                mutableEpoch().endTime = block.timestamp;
            }
        }

        emit VotedToKickValidatorInNextEpoch(
            stakerAddressOfSender,
            validatorStakerAddress,
            reason,
            data
        );
    }

    function clearEnumerableAddressSet(
        EnumerableSet.AddressSet storage set
    ) internal {
        while (set.length() > 0) {
            set.remove(set.at(0));
        }
    }

    /// Set the IP and port of your node
    /// @param ip The ip address of your node
    /// @param port The port of your node
    function setIpPortNodeAddressAndCommunicationPubKeys(
        uint32 ip,
        uint128 ipv6,
        uint32 port,
        address nodeAddress,
        uint256 senderPubKey,
        uint256 receiverPubKey
    ) external {
        s().validators[msg.sender].ip = ip;
        s().validators[msg.sender].ipv6 = ipv6;
        s().validators[msg.sender].port = port;
        s().validators[msg.sender].nodeAddress = nodeAddress;
        s().validators[msg.sender].senderPubKey = senderPubKey;
        s().validators[msg.sender].receiverPubKey = receiverPubKey;

        // don't let them overwrite an existing mapping
        // becuase it could belong to someone else,
        // but let them create a new mapping
        if (s().nodeAddressToStakerAddress[nodeAddress] == address(0)) {
            s().nodeAddressToStakerAddress[nodeAddress] = msg.sender;
        }
    }

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
    ) external onlyOwner {
        s().state = newState;
        emit StateChanged(newState);
    }

    function adminKickValidatorInNextEpoch(
        address validatorStakerAddress
    ) external onlyOwner {
        // block them from rejoining the next epoch
        s().validatorsKickedFromNextEpoch.add(validatorStakerAddress);

        // remove from the validator set
        removeValidatorFromNextEpoch(validatorStakerAddress);

        // if they're in the current set, we need to mark them as kicked from the current set too
        bool isValidatorInCurrentSet = s().validatorsInCurrentEpoch.contains(
            validatorStakerAddress
        );
        if (isValidatorInCurrentSet) {
            s().currentValidatorsKickedFromNextEpoch.add(
                validatorStakerAddress
            );
        }
        emit ValidatorKickedFromNextEpoch(validatorStakerAddress, 0);

        // // if we're in the locked state, then we need to unlock, because we kicked a node
        if (
            s().state == LibStakingStorage.States.NextValidatorSetLocked ||
            s().state == LibStakingStorage.States.ReadyForNextEpoch
        ) {
            unlockEpoch();
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

    function removeValidatorFromNextEpoch(address staker) internal {
        if (s().validatorsInNextEpoch.contains(staker)) {
            // remove them
            s().validatorsInNextEpoch.remove(staker);
        }
        LibStakingStorage.Validator memory validator = s().validators[staker];
        bytes32 commsKeysHash = keccak256(
            abi.encodePacked(validator.senderPubKey, validator.receiverPubKey)
        );
        s().usedCommsKeys[commsKeysHash] = false;
    }

    function adminRejoinValidator(address staker) external onlyOwner {
        if (
            !(s().state == LibStakingStorage.States.Active ||
                s().state == LibStakingStorage.States.Unlocked ||
                s().state == LibStakingStorage.States.Paused)
        ) {
            revert MustBeInActiveOrUnlockedOrPausedState(s().state);
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
        clearEnumerableAddressSet(s().validatorsInCurrentEpoch);

        // clear out validators in next epoch
        clearEnumerableAddressSet(s().validatorsInNextEpoch);

        // clear out the validators kicked from next epoch
        clearEnumerableAddressSet(s().validatorsKickedFromNextEpoch);

        // clear out the current validators kicked from next epoch
        clearEnumerableAddressSet(s().currentValidatorsKickedFromNextEpoch);

        // reset the epoch
        mutableEpoch().number = 1;
        mutableEpoch().endTime = block.timestamp + mutableEpoch().epochLength;
        mutableEpoch().retries = 0;

        // reset the state
        s().state = LibStakingStorage.States.Paused;
        emit StateChanged(s().state);
    }

    function unlockEpoch() internal {
        // this should only be callable from the ReadyForNextEpoch state or the NextValidatorSetLocked state
        if (
            !(s().state == LibStakingStorage.States.ReadyForNextEpoch ||
                s().state == LibStakingStorage.States.NextValidatorSetLocked)
        ) {
            revert MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
                s().state
            );
        }
        // clear out readyForNextEpoch for current nodes
        uint256 validatorLength = s().validatorsInCurrentEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            s().readyForNextEpoch[s().validatorsInCurrentEpoch.at(i)] = false;
        }

        // clear out readyForNextEpoch for next nodes
        validatorLength = s().validatorsInNextEpoch.length();
        for (uint256 i = 0; i < validatorLength; i++) {
            s().readyForNextEpoch[s().validatorsInNextEpoch.at(i)] = false;
        }

        s().state = LibStakingStorage.States.Unlocked;
        s().epochs[0].retries++;
        emit StateChanged(s().state);
    }

    /* ========== EVENTS ========== */

    event RewardsDurationUpdated(uint256 newDuration);
    event RequestToJoin(address indexed staker);
    event RequestToLeave(address indexed staker);
    event Recovered(address token, uint256 amount);
    event ReadyForNextEpoch(address indexed staker, uint256 epochNumber);
    event StateChanged(LibStakingStorage.States newState);
    event VotedToKickValidatorInNextEpoch(
        address indexed reporter,
        address indexed validatorStakerAddress,
        uint256 indexed reason,
        bytes data
    );
    event ValidatorKickedFromNextEpoch(
        address indexed staker,
        uint256 amountBurned
    );

    // onlyOwner events
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
}
