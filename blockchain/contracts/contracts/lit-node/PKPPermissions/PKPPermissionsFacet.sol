//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";

import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { PubkeyRouterFacet } from "../PubkeyRouter/PubkeyRouterFacet.sol";
import { PKPNFTFacet } from "../PKPNFT/PKPNFTFacet.sol";

import { LibPKPPermissionsStorage } from "./LibPKPPermissionsStorage.sol";
import { LibERC2771 } from "../../libraries/LibERC2771.sol";
import { ERC2771 } from "../../common/ERC2771.sol";

import "hardhat/console.sol";

contract PKPPermissionsFacet is ERC2771 {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using EnumerableSet for EnumerableSet.UintSet;
    using BytesLib for bytes;
    using BitMaps for BitMaps.BitMap;

    enum AuthMethodType {
        NULLMETHOD, // 0
        ADDRESS, // 1
        ACTION, // 2
        WEBAUTHN, // 3
        DISCORD, // 4
        GOOGLE, // 5
        GOOGLE_JWT, // 6
        OTP, // 7
        APPLE_JWT, // 8
        STYTCH_JWT, // 9
        STYTCH_JWT_EMAIL_FACTOR, // 10
        STYTCH_JWT_SMS_FACTOR, // 11
        STYTCH_JWT_WHATS_APP_FACTOR, // 12
        STYTCH_JWT_TOTP_FACTOR // 13
    }

    /* ========== Modifiers ========== */
    modifier onlyPKPOwner(uint256 tokenId) {
        // check that user is allowed to set this
        PKPNFTFacet pkpNFT = PKPNFTFacet(getPkpNftAddress());
        address nftOwner = pkpNFT.ownerOf(tokenId);
        require(LibERC2771._msgSender() == nftOwner, "Not PKP NFT owner");
        _;
    }

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibPKPPermissionsStorage.PKPPermissionsStorage storage)
    {
        return LibPKPPermissionsStorage.getStorage();
    }

    function getPkpNftAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.PKP_NFT_CONTRACT(),
                s().env
            );
    }

    function getRouterAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.PUB_KEY_ROUTER_CONTRACT(),
                s().env
            );
    }

    /// get the eth address for the keypair, as long as it's an ecdsa keypair
    function getEthAddress(uint256 tokenId) public view returns (address) {
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        return router.getEthAddress(tokenId);
    }

    /// includes the 0x04 prefix so you can pass this directly to ethers.utils.computeAddress
    function getPubkey(uint256 tokenId) public view returns (bytes memory) {
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        return router.getPubkey(tokenId);
    }

    function getAuthMethodId(
        uint256 authMethodType,
        bytes memory id
    ) public pure returns (uint256) {
        return uint256(keccak256(abi.encode(authMethodType, id)));
    }

    /// get the user's pubkey given their authMethodType and userId
    function getUserPubkeyForAuthMethod(
        uint256 authMethodType,
        bytes calldata id
    ) external view returns (bytes memory) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);
        LibPKPPermissionsStorage.AuthMethod memory am = s().authMethods[
            authMethodId
        ];
        return am.userPubkey;
    }

    function getTokenIdsForAuthMethod(
        uint256 authMethodType,
        bytes calldata id
    ) external view returns (uint256[] memory) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);

        uint256 pkpIdsLength = s().authMethodToPkpIds[authMethodId].length();
        uint256[] memory allPkpIds = new uint256[](pkpIdsLength);

        for (uint256 i = 0; i < pkpIdsLength; i++) {
            allPkpIds[i] = s().authMethodToPkpIds[authMethodId].at(i);
        }

        return allPkpIds;
    }

    function getPermittedAuthMethods(
        uint256 tokenId
    ) external view returns (LibPKPPermissionsStorage.AuthMethod[] memory) {
        uint256 permittedAuthMethodsLength = s()
            .permittedAuthMethods[tokenId]
            .length();
        LibPKPPermissionsStorage.AuthMethod[]
            memory allPermittedAuthMethods = new LibPKPPermissionsStorage.AuthMethod[](
                permittedAuthMethodsLength
            );

        for (uint256 i = 0; i < permittedAuthMethodsLength; i++) {
            uint256 authMethodHash = s().permittedAuthMethods[tokenId].at(i);
            allPermittedAuthMethods[i] = s().authMethods[authMethodHash];
        }

        return allPermittedAuthMethods;
    }

    function getPermittedAuthMethodScopes(
        uint256 tokenId,
        uint256 authMethodType,
        bytes calldata id,
        uint256 maxScopeId
    ) public view returns (bool[] memory) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);
        BitMaps.BitMap storage permittedScopesBitMap = s()
            .permittedAuthMethodScopes[tokenId][authMethodId];
        bool[] memory allScopes = new bool[](maxScopeId);

        for (uint256 i = 0; i < maxScopeId; i++) {
            allScopes[i] = permittedScopesBitMap.get(i);
        }

        return allScopes;
    }

    function getPermittedActions(
        uint256 tokenId
    ) public view returns (bytes[] memory) {
        uint256 permittedAuthMethodsLength = s()
            .permittedAuthMethods[tokenId]
            .length();

        // count the number of auth methods that are actions
        uint256 permittedActionsLength = 0;
        for (uint256 i = 0; i < permittedAuthMethodsLength; i++) {
            uint256 authMethodHash = s().permittedAuthMethods[tokenId].at(i);
            LibPKPPermissionsStorage.AuthMethod memory am = s().authMethods[
                authMethodHash
            ];
            if (am.authMethodType == uint256(AuthMethodType.ACTION)) {
                permittedActionsLength++;
            }
        }

        bytes[] memory allPermittedActions = new bytes[](
            permittedActionsLength
        );

        uint256 permittedActionsIndex = 0;
        for (uint256 i = 0; i < permittedAuthMethodsLength; i++) {
            uint256 authMethodHash = s().permittedAuthMethods[tokenId].at(i);
            LibPKPPermissionsStorage.AuthMethod memory am = s().authMethods[
                authMethodHash
            ];
            if (am.authMethodType == uint256(AuthMethodType.ACTION)) {
                allPermittedActions[permittedActionsIndex] = am.id;
                permittedActionsIndex++;
            }
        }

        return allPermittedActions;
    }

    function getPermittedAddresses(
        uint256 tokenId
    ) public view returns (address[] memory) {
        uint256 permittedAuthMethodsLength = s()
            .permittedAuthMethods[tokenId]
            .length();

        // count the number of auth methods that are addresses
        uint256 permittedAddressLength = 0;
        for (uint256 i = 0; i < permittedAuthMethodsLength; i++) {
            uint256 authMethodHash = s().permittedAuthMethods[tokenId].at(i);
            LibPKPPermissionsStorage.AuthMethod memory am = s().authMethods[
                authMethodHash
            ];
            if (am.authMethodType == uint256(AuthMethodType.ADDRESS)) {
                permittedAddressLength++;
            }
        }

        PKPNFTFacet pkpNFT = PKPNFTFacet(getPkpNftAddress());
        bool tokenExists = pkpNFT.exists(tokenId);
        address[] memory allPermittedAddresses;
        uint256 permittedAddressIndex = 0;
        if (tokenExists) {
            // token is not burned, so add the owner address
            allPermittedAddresses = new address[](permittedAddressLength + 1);

            // always add nft owner in first slot
            address nftOwner = pkpNFT.ownerOf(tokenId);
            allPermittedAddresses[0] = nftOwner;
            permittedAddressIndex++;
        } else {
            // token is burned, so don't add the owner address
            allPermittedAddresses = new address[](permittedAddressLength);
        }

        for (uint256 i = 0; i < permittedAuthMethodsLength; i++) {
            uint256 authMethodHash = s().permittedAuthMethods[tokenId].at(i);
            LibPKPPermissionsStorage.AuthMethod memory am = s().authMethods[
                authMethodHash
            ];
            if (am.authMethodType == uint256(AuthMethodType.ADDRESS)) {
                address parsed;
                bytes memory id = am.id;

                // address was packed using abi.encodedPacked(address), so you need to pad left to get the correct bytes back
                assembly {
                    parsed := div(
                        mload(add(id, 32)),
                        0x1000000000000000000000000
                    )
                }
                allPermittedAddresses[permittedAddressIndex] = parsed;
                permittedAddressIndex++;
            }
        }

        return allPermittedAddresses;
    }

    function getPKPPubKeysByAuthMethod(
        uint256 authMethodType,
        bytes memory id
    ) external view returns (bytes[] memory) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);

        uint256 pkpIdsLength = s().authMethodToPkpIds[authMethodId].length();
        bytes[] memory allPkpPubkeys = new bytes[](pkpIdsLength);

        for (uint256 i = 0; i < pkpIdsLength; i++) {
            allPkpPubkeys[i] = getPubkey(
                s().authMethodToPkpIds[authMethodId].at(i)
            );
        }

        return allPkpPubkeys;
    }
    /// get if a user is permitted to use a given pubkey.  returns true if it is permitted to use the pubkey in the permittedAuthMethods[tokenId] struct.
    function isPermittedAuthMethod(
        uint256 tokenId,
        uint256 authMethodType,
        bytes memory id
    ) public view returns (bool) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);
        bool permitted = s().permittedAuthMethods[tokenId].contains(
            authMethodId
        );
        if (!permitted) {
            return false;
        }
        return true;
    }

    function isPermittedAuthMethodScopePresent(
        uint256 tokenId,
        uint256 authMethodType,
        bytes calldata id,
        uint256 scopeId
    ) public view returns (bool) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);
        bool present = s().permittedAuthMethodScopes[tokenId][authMethodId].get(
            scopeId
        );
        return present;
    }

    function isPermittedAction(
        uint256 tokenId,
        bytes calldata ipfsCID
    ) public view returns (bool) {
        return
            isPermittedAuthMethod(
                tokenId,
                uint256(AuthMethodType.ACTION),
                ipfsCID
            );
    }

    function isPermittedAddress(
        uint256 tokenId,
        address user
    ) public view returns (bool) {
        PKPNFTFacet pkpNFT = PKPNFTFacet(getPkpNftAddress());
        bool userIsOwner = false;
        if (pkpNFT.exists(tokenId)) {
            address nftOwner = pkpNFT.ownerOf(tokenId);
            userIsOwner = nftOwner == user;
        }
        return
            isPermittedAuthMethod(
                tokenId,
                uint256(AuthMethodType.ADDRESS),
                abi.encodePacked(user)
            ) || userIsOwner;
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function batchAddRemoveAuthMethods(
        uint256 tokenId,
        uint256[] memory permittedAuthMethodTypesToAdd,
        bytes[] memory permittedAuthMethodIdsToAdd,
        bytes[] memory permittedAuthMethodPubkeysToAdd,
        uint256[][] calldata permittedAuthMethodScopesToAdd,
        uint256[] memory permittedAuthMethodTypesToRemove,
        bytes[] memory permittedAuthMethodIdsToRemove
    ) public onlyPKPOwner(tokenId) {
        require(
            permittedAuthMethodTypesToAdd.length ==
                permittedAuthMethodIdsToAdd.length &&
                permittedAuthMethodIdsToAdd.length ==
                permittedAuthMethodPubkeysToAdd.length &&
                permittedAuthMethodPubkeysToAdd.length ==
                permittedAuthMethodScopesToAdd.length,
            "Must have same number of auth methods, ids, pubkeys, and scopes to add"
        );

        require(
            permittedAuthMethodTypesToRemove.length ==
                permittedAuthMethodIdsToRemove.length,
            "Must have same number of auth methods and ids to remove"
        );

        for (uint256 i = 0; i < permittedAuthMethodTypesToAdd.length; i++) {
            addPermittedAuthMethod(
                tokenId,
                LibPKPPermissionsStorage.AuthMethod(
                    permittedAuthMethodTypesToAdd[i],
                    permittedAuthMethodIdsToAdd[i],
                    permittedAuthMethodPubkeysToAdd[i]
                ),
                permittedAuthMethodScopesToAdd[i]
            );
        }

        for (uint256 i = 0; i < permittedAuthMethodTypesToRemove.length; i++) {
            removePermittedAuthMethod(
                tokenId,
                permittedAuthMethodTypesToRemove[i],
                permittedAuthMethodIdsToRemove[i]
            );
        }
    }

    /// Add a permitted auth method for a given pubkey
    function addPermittedAuthMethod(
        uint256 tokenId,
        LibPKPPermissionsStorage.AuthMethod memory authMethod,
        uint256[] calldata scopes
    ) public onlyPKPOwner(tokenId) {
        uint256 authMethodId = getAuthMethodId(
            authMethod.authMethodType,
            authMethod.id
        );

        // we need to ensure that someone with the same auth method type and id can't add a different pubkey
        if (authMethod.authMethodType == uint(AuthMethodType.WEBAUTHN)) {
            require(
                s().authMethods[authMethodId].userPubkey.length == 0 ||
                    keccak256(s().authMethods[authMethodId].userPubkey) ==
                    keccak256(authMethod.userPubkey),
                "Cannot add a different pubkey for the same auth method type and id"
            );
        }

        s().authMethods[authMethodId] = authMethod;

        EnumerableSet.UintSet storage newPermittedAuthMethods = s()
            .permittedAuthMethods[tokenId];
        newPermittedAuthMethods.add(authMethodId);

        EnumerableSet.UintSet storage newPkpIds = s().authMethodToPkpIds[
            authMethodId
        ];
        newPkpIds.add(tokenId);

        for (uint256 i = 0; i < scopes.length; i++) {
            uint256 scopeId = scopes[i];

            s().permittedAuthMethodScopes[tokenId][authMethodId].set(scopeId);

            emit PermittedAuthMethodScopeAdded(
                tokenId,
                authMethodId,
                authMethod.id,
                scopeId
            );
        }

        emit PermittedAuthMethodAdded(
            tokenId,
            authMethod.authMethodType,
            authMethod.id,
            authMethod.userPubkey
        );
    }

    // Remove a permitted auth method for a given pubkey
    function removePermittedAuthMethod(
        uint256 tokenId,
        uint256 authMethodType,
        bytes memory id
    ) public onlyPKPOwner(tokenId) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);

        EnumerableSet.UintSet storage newPermittedAuthMethods = s()
            .permittedAuthMethods[tokenId];
        newPermittedAuthMethods.remove(authMethodId);

        EnumerableSet.UintSet storage newPkpIds = s().authMethodToPkpIds[
            authMethodId
        ];
        newPkpIds.remove(tokenId);

        emit PermittedAuthMethodRemoved(tokenId, authMethodId, id);
    }

    function addPermittedAuthMethodScope(
        uint256 tokenId,
        uint256 authMethodType,
        bytes calldata id,
        uint256 scopeId
    ) public onlyPKPOwner(tokenId) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);

        s().permittedAuthMethodScopes[tokenId][authMethodId].set(scopeId);

        emit PermittedAuthMethodScopeAdded(tokenId, authMethodId, id, scopeId);
    }

    function removePermittedAuthMethodScope(
        uint256 tokenId,
        uint256 authMethodType,
        bytes calldata id,
        uint256 scopeId
    ) public onlyPKPOwner(tokenId) {
        uint256 authMethodId = getAuthMethodId(authMethodType, id);

        s().permittedAuthMethodScopes[tokenId][authMethodId].unset(scopeId);

        emit PermittedAuthMethodScopeRemoved(
            tokenId,
            authMethodType,
            id,
            scopeId
        );
    }

    /// Add a permitted action for a given pubkey
    function addPermittedAction(
        uint256 tokenId,
        bytes calldata ipfsCID,
        uint256[] calldata scopes
    ) public {
        addPermittedAuthMethod(
            tokenId,
            LibPKPPermissionsStorage.AuthMethod(
                uint256(AuthMethodType.ACTION),
                ipfsCID,
                ""
            ),
            scopes
        );
    }

    function removePermittedAction(
        uint256 tokenId,
        bytes calldata ipfsCID
    ) public {
        removePermittedAuthMethod(
            tokenId,
            uint256(AuthMethodType.ACTION),
            ipfsCID
        );
    }

    function addPermittedAddress(
        uint256 tokenId,
        address user,
        uint256[] calldata scopes
    ) public {
        addPermittedAuthMethod(
            tokenId,
            LibPKPPermissionsStorage.AuthMethod(
                uint256(AuthMethodType.ADDRESS),
                abi.encodePacked(user),
                ""
            ),
            scopes
        );
    }

    function removePermittedAddress(uint256 tokenId, address user) public {
        removePermittedAuthMethod(
            tokenId,
            uint256(AuthMethodType.ADDRESS),
            abi.encodePacked(user)
        );
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        s().contractResolver = ContractResolver(newResolverAddress);
        emit ContractResolverAddressSet(newResolverAddress);
    }

    /**
     * Update the root hash of the merkle tree representing off-chain states for the PKP
     */
    function setRootHash(
        uint256 tokenId,
        uint256 group,
        bytes32 root
    ) public onlyPKPOwner(tokenId) {
        s()._rootHashes[tokenId][group] = root;
        emit RootHashUpdated(tokenId, group, root);
    }

    /**
     * Verify the given leaf existing in the merkle tree
     */
    function verifyState(
        uint256 tokenId,
        uint256 group,
        bytes32[] memory proof,
        bytes32 leaf
    ) public view returns (bool) {
        bytes32 root = s()._rootHashes[tokenId][group];
        if (root == bytes32(0)) return false;
        return MerkleProof.verify(proof, root, leaf);
    }

    /**
     * Verify the given leaves existing in the merkle tree
     */
    function verifyStates(
        uint256 tokenId,
        uint256 group,
        bytes32[] memory proof,
        bool[] memory proofFlags,
        bytes32[] memory leaves
    ) public view returns (bool) {
        bytes32 root = s()._rootHashes[tokenId][group];
        if (root == bytes32(0)) return false;
        return MerkleProof.multiProofVerify(proof, proofFlags, root, leaves);
    }

    /* ========== EVENTS ========== */

    event PermittedAuthMethodAdded(
        uint256 indexed tokenId,
        uint256 authMethodType,
        bytes id,
        bytes userPubkey
    );
    event PermittedAuthMethodRemoved(
        uint256 indexed tokenId,
        uint256 authMethodType,
        bytes id
    );
    event PermittedAuthMethodScopeAdded(
        uint256 indexed tokenId,
        uint256 authMethodType,
        bytes id,
        uint256 scopeId
    );
    event PermittedAuthMethodScopeRemoved(
        uint256 indexed tokenId,
        uint256 authMethodType,
        bytes id,
        uint256 scopeId
    );
    event RootHashUpdated(
        uint256 indexed tokenId,
        uint256 indexed group,
        bytes32 root
    );
    event ContractResolverAddressSet(address newResolverAddress);
}
