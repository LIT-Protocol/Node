//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { LibCloneNetStorage } from "./LibCloneNetStorage.sol";
import { LibStakingStorage } from "../Staking/LibStakingStorage.sol";

contract CloneNetFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    error CallerNotOwner();

    /* ========== Modifiers ========== */

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    function s()
        internal
        pure
        returns (LibCloneNetStorage.CloneNetStorage storage)
    {
        return LibCloneNetStorage.getStorage();
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function adminAddActiveStakingContract(
        address stakingContractAddress
    ) external onlyOwner {
        s().activeStakingContractAddresses.add(stakingContractAddress);
    }

    function adminRemoveActiveStakingContract(
        address stakingContractAddress
    ) external onlyOwner {
        s().activeStakingContractAddresses.remove(stakingContractAddress);
    }

    /* ========== VIEWS ========== */

    function numActiveStakingContracts() external view returns (uint256) {
        return s().activeStakingContractAddresses.length();
    }

    function getActiveStakingContracts()
        public
        view
        returns (address[] memory)
    {
        return s().activeStakingContractAddresses.values();
    }

    /* ========== AGGREGATE VIEWS ========== */

    function getAllActiveUnkickedValidatorStructsAndCounts()
        external
        view
        returns (LibStakingStorage.KeyedStakingAggregateDetails[] memory)
    {
        // First get all the active staking contract addresses.
        address[]
            memory activeStakingContractAddresses = getActiveStakingContracts();

        // Then, for each active staking contract, get the epoch, current validator count, and active unkicked validator structs.
        LibStakingStorage.KeyedStakingAggregateDetails[]
            memory stakingAggregateDetails = new LibStakingStorage.KeyedStakingAggregateDetails[](
                activeStakingContractAddresses.length
            );

        for (uint256 i = 0; i < activeStakingContractAddresses.length; i++) {
            address stakingContractAddress = activeStakingContractAddresses[i];
            StakingViewsFacet stakingViewsFacet = StakingViewsFacet(
                stakingContractAddress
            );

            // Get the epoch, current validator count, and active unkicked validator structs.
            (
                LibStakingStorage.Epoch memory epoch,
                uint256 currentValidatorCountForConsensus,
                LibStakingStorage.Validator[] memory activeUnkickedValidators
            ) = stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts();

            // Then, store them in the stakingAggregateDetails array.
            stakingAggregateDetails[i] = LibStakingStorage
                .KeyedStakingAggregateDetails({
                    stakingContractAddress: stakingContractAddress,
                    details: LibStakingStorage.StakingAggregateDetails({
                        epoch: epoch,
                        currentValidatorCountForConsensus: currentValidatorCountForConsensus,
                        activeUnkickedValidators: activeUnkickedValidators
                    })
                });
        }

        return stakingAggregateDetails;
    }
}
