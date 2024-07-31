//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";

import { ContractResolver } from "../../lit-core/ContractResolver.sol";

library LibPKPPermissionsStorage {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using EnumerableSet for EnumerableSet.UintSet;
    using BytesLib for bytes;
    using BitMaps for BitMaps.BitMap;

    bytes32 constant PKP_PERMISSIONS_POSITION =
        keccak256("pkppermissions.storage");

    struct AuthMethod {
        uint256 authMethodType; // 1 = address, 2 = action, 3 = WebAuthn, 4 = Discord, 5 = Google, 6 = Google JWT, 7 = OTP, 8 = Apple JWT.  Not doing this in an enum so that we can add more auth methods in the future without redeploying.
        bytes id; // the id of the auth method.  For address, this is an eth address.  For action, this is an IPFS CID.  For WebAuthn, this is the credentialId.  For Discord, this is the user's Discord ID.  For Google, this is the user's Google ID.
        bytes userPubkey; // the user's pubkey.  This is used for WebAuthn.
    }

    struct PKPPermissionsStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        // map the keccack256(uncompressed pubkey) -> set of auth methods
        mapping(uint256 => EnumerableSet.UintSet) permittedAuthMethods;
        // map the keccack256(uncompressed pubkey) -> auth_method_id -> scope id
        mapping(uint256 => mapping(uint256 => BitMaps.BitMap)) permittedAuthMethodScopes;
        // map the keccack256(authMethodType, userId) -> the actual AuthMethod struct
        mapping(uint256 => AuthMethod) authMethods;
        // map the AuthMethod hash to the pubkeys that it's allowed to sign for
        // this makes it possible to be given a discord id and then lookup all the pubkeys that are allowed to sign for that discord id
        mapping(uint256 => EnumerableSet.UintSet) authMethodToPkpIds;
        // map the keccack256(uncompressed pubkey) -> (group => merkle tree root hash)
        mapping(uint256 => mapping(uint256 => bytes32)) _rootHashes;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (PKPPermissionsStorage storage storageStruct)
    {
        bytes32 position = PKP_PERMISSIONS_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
