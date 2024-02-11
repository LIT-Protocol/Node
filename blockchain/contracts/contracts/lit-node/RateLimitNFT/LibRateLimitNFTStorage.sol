//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";

library LibRateLimitNFTStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    struct RateLimit {
        uint256 requestsPerKilosecond;
        uint256 expiresAt;
    }

    bytes32 constant RATELIMIT_NFT_POSITION = keccak256("ratelimitnft.storage");

    struct RateLimitNFTStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        address freeMintSigner;
        uint256 additionalRequestsPerKilosecondCost;
        uint256 tokenIdCounter;
        uint256 defaultRateLimitWindowSeconds;
        uint256 RLIHolderRateLimitWindowSeconds;
        uint256 freeRequestsPerRateLimitWindow;
        mapping(uint256 => RateLimit) capacity;
        mapping(bytes32 => bool) redeemedFreeMints;
        uint256 maxRequestsPerKilosecond;
        uint256 maxExpirationSeconds;
        // maps midnight timestamps (divisible by 86400) to the total requests sold that will expire at that time
        mapping(uint256 => uint256) totalSoldRequestsPerKilosecondByExpirationTime;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (RateLimitNFTStorage storage storageStruct)
    {
        bytes32 position = RATELIMIT_NFT_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
