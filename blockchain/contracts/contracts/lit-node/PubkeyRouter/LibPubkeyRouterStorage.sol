//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";

interface IPubkeyRouter {
    struct RootKey {
        bytes pubkey;
        uint256 keyType; // 1 = BLS, 2 = ECDSA.  Not doing this in an enum so we can add more keytypes in the future without redeploying.
    }

    struct Signature {
        bytes32 r;
        bytes32 s;
        uint8 v;
    }
}

library LibPubkeyRouterStorage {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using EnumerableSet for EnumerableSet.UintSet;
    using BytesLib for bytes;
    using BitMaps for BitMaps.BitMap;

    bytes32 constant PUBKEY_ROUTER_POSITION = keccak256("pubkeyrouter.storage");

    struct PubkeyRoutingData {
        bytes pubkey;
        uint256 keyType; // 1 = BLS, 2 = ECDSA.  Not doing this in an enum so we can add more keytypes in the future without redeploying.
        bytes32 derivedKeyId;
    }

    struct VoteToRegisterRootKey {
        uint256 votes;
        mapping(address => bool) voted;
    }

    struct PubkeyRouterStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        // map staking address -> uncompressed pubkey -> VoteToRegisterRootKey
        mapping(address => mapping(bytes => VoteToRegisterRootKey)) votesToRegisterRootKeys;
        // map the keccack256(uncompressed pubkey) -> PubkeyRoutingData
        mapping(uint256 => PubkeyRoutingData) pubkeys;
        // map the eth address to a pkp id
        mapping(address => uint256) ethAddressToPkpId;
        // map staking contract to root keys
        mapping(address => IPubkeyRouter.RootKey[]) rootKeys;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (PubkeyRouterStorage storage storageStruct)
    {
        bytes32 position = PUBKEY_ROUTER_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
