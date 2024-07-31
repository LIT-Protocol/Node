// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibBackupRecoveryStorage } from "./LibBackupRecoveryStorage.sol";
import { StakingViewsFacet } from "./../Staking/StakingViewsFacet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import "hardhat/console.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

contract BackupRecoveryNodeStatusFacet {
    uint256 public constant CURRENT = 0;

    function s()
        internal
        pure
        returns (LibBackupRecoveryStorage.BackupRecoveryStorage storage)
    {
        return LibBackupRecoveryStorage.getStorage();
    }

    /**
     * @dev
     * Registers the status of the node
     */
    function setNodeRecoveryStatus(
        LibBackupRecoveryStorage.NodeRecoveryStatus status
    ) external {
        require(
            checkValidatorSetForAddress(msg.sender),
            "BackupRecovery: not a member of the validator set"
        );

        for (uint256 i = 0; i < s().nodeStatusMap[CURRENT].length; i++) {
            if (s().nodeStatusMap[CURRENT][i].node_address == msg.sender) {
                s().nodeStatusMap[CURRENT][i].status = status;
                return;
            }
        }

        s().nodeStatusMap[CURRENT].push(
            LibBackupRecoveryStorage.NodeRecoveryStatusMap(msg.sender, status)
        );
    }

    /**
     * @dev
     * Reports the recovery status of the nodes
     */
    function getNodeRecoveryStatus()
        external
        view
        returns (LibBackupRecoveryStorage.NodeRecoveryStatusMap[] memory)
    {
        return s().nodeStatusMap[CURRENT];
    }

    /**
     * @dev
     * Clears the recovery status of the nodes after the recovery
     */
    function clearNodeRecoveryStatus() external {
        require(
            msg.sender == LibDiamond.contractOwner(),
            "BackupRecovery: caller not the admin"
        );

        delete s().nodeStatusMap[CURRENT];
    }

    function getStakingViewFacet() internal view returns (StakingViewsFacet) {
        address stakingAddress = s().resolver.getContract(
            s().resolver.STAKING_CONTRACT(),
            s().env
        );
        return StakingViewsFacet(stakingAddress);
    }

    function checkValidatorSetForAddress(
        address sender
    ) internal view returns (bool) {
        address[] memory validators = getStakingViewFacet()
            .getValidatorsInCurrentEpoch();

        for (uint256 i = 0; i < validators.length; i++) {
            address stakerAddressOfSender = getStakingViewFacet()
                .nodeAddressToStakerAddress(sender);
            if (validators[i] == stakerAddressOfSender) {
                return true;
            }
        }

        return false;
    }
}
