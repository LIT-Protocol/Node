//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";

library LibCloneNetStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    bytes32 constant CLONE_NET_POSITION = keccak256("clonenet.storage");

    struct CloneNetStorage {
        /// @notice The list of staking contract addresses that are currently active and
        /// are clones of each other.
        EnumerableSet.AddressSet activeStakingContractAddresses;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (CloneNetStorage storage storageStruct)
    {
        bytes32 position = CLONE_NET_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
