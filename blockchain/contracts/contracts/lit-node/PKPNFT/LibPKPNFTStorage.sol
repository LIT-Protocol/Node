//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { IPubkeyRouter } from "../PubkeyRouter/LibPubkeyRouterStorage.sol";

library LibPKPNFTStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    bytes32 constant PKP_NFT_POSITION = keccak256("pkpnft.storage");

    struct ClaimMaterial {
        uint256 keyType;
        bytes32 derivedKeyId;
        IPubkeyRouter.Signature[] signatures;
    }

    struct PKPNFTStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        uint256 mintCost;
        address freeMintSigner;
        mapping(uint256 => bool) redeemedFreeMintIds;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (PKPNFTStorage storage storageStruct)
    {
        bytes32 position = PKP_NFT_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
