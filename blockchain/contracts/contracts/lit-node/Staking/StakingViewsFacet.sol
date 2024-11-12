//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { StakingBalancesFacet } from "../StakingBalances/StakingBalancesFacet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";

import { LibStakingStorage } from "./LibStakingStorage.sol";

// import "hardhat/console.sol";

contract StakingViewsFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== VIEWS ========== */
    function stakingStorage()
        internal
        pure
        returns (LibStakingStorage.StakingStorage storage)
    {
        return LibStakingStorage.getStorage();
    }

    function epoch() public view returns (LibStakingStorage.Epoch memory) {
        return stakingStorage().epochs[0];
    }

    function config() public view returns (LibStakingStorage.Config memory) {
        return stakingStorage().configs[0];
    }

    function complaintConfig(
        uint256 reason
    ) public view returns (LibStakingStorage.ComplaintConfig memory) {
        return stakingStorage().complaintReasonToConfig[reason];
    }

    function getKeyTypes() external view returns (uint256[] memory) {
        return config().keyTypes;
    }

    function contractResolver() external view returns (address) {
        return address(stakingStorage().contractResolver);
    }

    function kickPenaltyPercentByReason(
        uint256 reason
    ) external view returns (uint256) {
        return
            stakingStorage().complaintReasonToConfig[reason].kickPenaltyPercent;
    }

    function nodeAddressToStakerAddress(
        address nodeAddress
    ) external view returns (address) {
        return stakingStorage().nodeAddressToStakerAddress[nodeAddress];
    }

    function readyForNextEpoch(
        address stakerAddress
    ) external view returns (bool) {
        return stakingStorage().readyForNextEpoch[stakerAddress];
    }

    function state() external view returns (LibStakingStorage.States) {
        return stakingStorage().state;
    }

    /// get the token address from the resolver
    function getTokenAddress() public view returns (address) {
        return
            stakingStorage().contractResolver.getContract(
                stakingStorage().contractResolver.LIT_TOKEN_CONTRACT(),
                stakingStorage().env
            );
    }

    // get the staking balances address from the resolver
    function getStakingBalancesAddress() public view returns (address) {
        return
            stakingStorage().contractResolver.getContract(
                stakingStorage().contractResolver.STAKING_BALANCES_CONTRACT(),
                stakingStorage().env
            );
    }

    function validators(
        address stakerAddress
    ) public view returns (LibStakingStorage.Validator memory) {
        return stakingStorage().validators[stakerAddress];
    }

    function isActiveValidator(address account) external view returns (bool) {
        return stakingStorage().validatorsInCurrentEpoch.contains(account);
    }

    function isActiveValidatorByNodeAddress(
        address account
    ) external view returns (bool) {
        return
            stakingStorage().validatorsInCurrentEpoch.contains(
                stakingStorage().nodeAddressToStakerAddress[account]
            );
    }

    function getVotingStatusToKickValidator(
        uint256 epochNumber,
        address validatorStakerAddress,
        address voterStakerAddress
    ) external view returns (uint256, bool) {
        LibStakingStorage.VoteToKickValidatorInNextEpoch
            storage votingStatus = stakingStorage()
                .votesToKickValidatorsInNextEpoch[epochNumber][
                    validatorStakerAddress
                ];
        return (votingStatus.votes, votingStatus.voted[voterStakerAddress]);
    }

    function getValidatorsInCurrentEpoch()
        public
        view
        returns (address[] memory)
    {
        address[] memory values = new address[](
            stakingStorage().validatorsInCurrentEpoch.length()
        );
        uint256 validatorLength = stakingStorage()
            .validatorsInCurrentEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            values[i] = stakingStorage().validatorsInCurrentEpoch.at(i);
        }
        return values;
    }

    function getValidatorsInCurrentEpochLength()
        external
        view
        returns (uint256)
    {
        return stakingStorage().validatorsInCurrentEpoch.length();
    }

    function getValidatorsInNextEpoch() public view returns (address[] memory) {
        address[] memory values = new address[](
            stakingStorage().validatorsInNextEpoch.length()
        );
        uint256 validatorLength = stakingStorage()
            .validatorsInNextEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            values[i] = stakingStorage().validatorsInNextEpoch.at(i);
        }
        return values;
    }

    function getValidatorsStructs(
        address[] memory addresses
    ) public view returns (LibStakingStorage.Validator[] memory) {
        LibStakingStorage.Validator[]
            memory values = new LibStakingStorage.Validator[](addresses.length);
        for (uint256 i = 0; i < addresses.length; i++) {
            values[i] = stakingStorage().validators[addresses[i]];
        }
        return values;
    }

    function getValidatorsStructsInCurrentEpoch()
        external
        view
        returns (LibStakingStorage.Validator[] memory)
    {
        address[] memory addresses = getValidatorsInCurrentEpoch();
        return getValidatorsStructs(addresses);
    }

    function getValidatorsStructsInNextEpoch()
        external
        view
        returns (LibStakingStorage.Validator[] memory)
    {
        address[] memory addresses = getValidatorsInNextEpoch();
        return getValidatorsStructs(addresses);
    }

    function getNodeStakerAddressMappings(
        address[] memory addresses
    ) public view returns (LibStakingStorage.AddressMapping[] memory) {
        LibStakingStorage.AddressMapping[]
            memory values = new LibStakingStorage.AddressMapping[](
                addresses.length
            );
        for (uint256 i = 0; i < addresses.length; i++) {
            values[i].nodeAddress = addresses[i];
            values[i].stakerAddress = stakingStorage()
                .nodeAddressToStakerAddress[addresses[i]];
        }
        return values;
    }

    function countOfCurrentValidatorsReadyForNextEpoch()
        public
        view
        returns (uint256)
    {
        uint256 total = 0;
        uint256 validatorLength = stakingStorage()
            .validatorsInCurrentEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            if (
                stakingStorage().readyForNextEpoch[
                    stakingStorage().validatorsInCurrentEpoch.at(i)
                ]
            ) {
                total++;
            }
        }
        return total;
    }

    function countOfNextValidatorsReadyForNextEpoch()
        public
        view
        returns (uint256)
    {
        uint256 total = 0;
        uint256 validatorLength = stakingStorage()
            .validatorsInNextEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            if (
                stakingStorage().readyForNextEpoch[
                    stakingStorage().validatorsInNextEpoch.at(i)
                ]
            ) {
                total++;
            }
        }
        return total;
    }

    function isReadyForNextEpoch() public view returns (bool) {
        // confirm that current validator set is ready
        if (
            countOfCurrentValidatorsReadyForNextEpoch() <
            currentValidatorCountForConsensus()
        ) {
            return false;
        }

        // confirm that next validator set is ready
        if (
            countOfNextValidatorsReadyForNextEpoch() <
            nextValidatorCountForConsensus()
        ) {
            return false;
        }

        return true;
    }

    function shouldKickValidator(
        address stakerAddress
    ) public view returns (bool) {
        // 2/3 of validators must vote
        // and we don't want to kick below the threshold
        if (
            stakingStorage()
            .votesToKickValidatorsInNextEpoch[epoch().number][stakerAddress]
                .votes >= currentValidatorCountForConsensus()
        ) {
            return true;
        }
        return false;
    }

    // currently set to 2/3.  this could be changed to be configurable.
    function currentValidatorCountForConsensus() public view returns (uint256) {
        if (stakingStorage().validatorsInCurrentEpoch.length() == 2) {
            return 1;
        }

        return (stakingStorage().validatorsInCurrentEpoch.length() * 2) / 3;
    }

    /// require all nodes in the next validator set to vote that they're ready
    /// any offline nodes will be kicked from the next validator set so that's why this is safe
    function nextValidatorCountForConsensus() public view returns (uint256) {
        return stakingStorage().validatorsInNextEpoch.length();
    }

    function getKickedValidators() public view returns (address[] memory) {
        address[] memory values = new address[](
            stakingStorage().validatorsKickedFromNextEpoch.length()
        );
        uint256 validatorLength = stakingStorage()
            .validatorsKickedFromNextEpoch
            .length();
        for (uint256 i = 0; i < validatorLength; i++) {
            values[i] = stakingStorage().validatorsKickedFromNextEpoch.at(i);
        }
        return values;
    }

    function getActiveUnkickedValidators()
        public
        view
        returns (address[] memory)
    {
        uint256 currentCount = stakingStorage()
            .validatorsInCurrentEpoch
            .length();
        uint256 kickedCountInNextEpoch = stakingStorage()
            .validatorsKickedFromNextEpoch
            .length();
        uint256 activeCount = currentCount - kickedCountInNextEpoch;
        address[] memory values = new address[](activeCount);
        uint256 index = 0;
        for (uint256 i = 0; i < currentCount; i++) {
            address validator = stakingStorage().validatorsInCurrentEpoch.at(i);
            if (
                stakingStorage().validatorsKickedFromNextEpoch.contains(
                    validator
                )
            ) {
                continue;
            }
            values[index] = validator;
            index++;
        }
        return values;
    }

    function getActiveUnkickedValidatorStructs()
        public
        view
        returns (LibStakingStorage.Validator[] memory)
    {
        address[] memory activeValidators = getActiveUnkickedValidators();
        return getValidatorsStructs(activeValidators);
    }

    function getActiveUnkickedValidatorStructsAndCounts()
        public
        view
        returns (
            LibStakingStorage.Epoch memory,
            uint256,
            LibStakingStorage.Validator[] memory
        )
    {
        return (
            epoch(),
            currentValidatorCountForConsensus(),
            getActiveUnkickedValidatorStructs()
        );
    }
}
