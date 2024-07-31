//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";

library LibStakingBalancesStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    struct VoteToKickValidatorInNextEpoch {
        uint256 votes;
        mapping(address => bool) voted;
    }

    bytes32 constant STAKING_BALANCES_POSITION =
        keccak256("stakingbalances.storage");

    struct StakingBalancesStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        mapping(address => uint256) balances;
        mapping(address => uint256) rewards;
        // allowed stakers
        mapping(address => bool) permittedStakers;
        // maps alias address to real staker address
        mapping(address => address) aliases;
        // maps staker address to alias count
        mapping(address => uint256) aliasCounts;
        uint256 minimumStake;
        uint256 maximumStake;
        uint256 totalStaked;
        bool permittedStakersOn;
        uint256 maxAliasCount;
        uint256 penaltyBalance;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (StakingBalancesStorage storage storageStruct)
    {
        bytes32 position = STAKING_BALANCES_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
