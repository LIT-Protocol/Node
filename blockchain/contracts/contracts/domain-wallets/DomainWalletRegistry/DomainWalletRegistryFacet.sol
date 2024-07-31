//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { LibDomainWalletRegistryStorage } from "./LibDomainWalletRegistryStorage.sol";
import { PKPHelper } from "../../lit-node/PKPHelper.sol";
import { DomainWalletRegistryViewsFacet } from "./DomainWalletRegistryViewsFacet.sol";

contract DomainWalletRegistryFacet {
    /* ========== ERRORS ========== */
    error InvalidNftMetadataCollectionLength(
        uint256 metadataCount,
        uint256 validMetadataCount
    );
    error MaximumCharacterLimitExceeded(uint length, bytes uri);
    error CallerNotOwner();

    /* ========== EVENTS ========== */
    event Registered(uint64 id, bytes subDomain, uint ttl, uint256 tokenId);
    event Removed(uint256 tokenId, bytes subDomain);
    event Revoked(uint256 tokenId, bytes subDomain);
    event Expired(bytes subDomain, uint256 tokenId, uint ttl);

    /* ========== Modifiers ========== */

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    /* ========== Views ========== */

    function s()
        internal
        pure
        returns (
            LibDomainWalletRegistryStorage.DomainWalletRegistryStorage storage
        )
    {
        return LibDomainWalletRegistryStorage.getStorage();
    }

    function views() internal view returns (DomainWalletRegistryViewsFacet) {
        return DomainWalletRegistryViewsFacet(address(this));
    }

    /* ========== Mutative Functions ========== */

    function hasExpired(uint256 pkpTokenId) public onlyOwner returns (bool) {
        uint ttl = s().pkpOwners[pkpTokenId].ttl;
        bytes memory uri = s().pkpOwners[pkpTokenId].uri;
        bool isExpired = ttl < block.timestamp;

        if (isExpired) {
            emit Expired(uri, pkpTokenId, ttl);
            return true;
        }
    }

    function setPKPMetadata(
        uint256 pkpTokenId,
        string[] memory nftMetadata
    ) public onlyOwner {
        PKPHelper pkpHelper = PKPHelper(views().getPkpHelperAddress());
        pkpHelper.setPkpMetadata(pkpTokenId, nftMetadata);
    }

    function registerDomain(
        bytes memory userId,
        bytes memory uri,
        uint ttl,
        uint256 pkpTokenId,
        string[] memory nftMetadata
    ) public onlyOwner returns (uint256) {
        require(
            nftMetadata.length == 2,
            "DomainWalletRegistry: metadata name and url must be set in metadata"
        );
        if (nftMetadata.length != 2) {
            revert InvalidNftMetadataCollectionLength(nftMetadata.length, 2);
        }
        if (uri.length > s().domainCharLimit) {
            revert MaximumCharacterLimitExceeded(uri.length, uri);
        }

        PKPHelper pkpHelper = PKPHelper(views().getPkpHelperAddress());
        _registerDomain(userId, uri, pkpTokenId, ttl);
        pkpHelper.setPkpMetadata(pkpTokenId, nftMetadata);
        return pkpTokenId;
    }

    function _registerDomain(
        bytes memory userId,
        bytes memory uri,
        uint256 pkpTokenId,
        uint ttl
    ) internal {
        views().checkRegistration(uri);

        s().idCounter = s().idCounter + 1;
        uint64 id = s().idCounter;
        s().domainWallets[id].user_id = userId;
        s().domainWallets[id].pkpTokenId = pkpTokenId;
        s().domainWallets[id].uri = uri;
        s().domainWallets[id].ttl = ttl;
        s().domainWallets[id].isRegistered = true;
        s().domainWallets[id].isWallet = true;
        s().domainWallets[id].id = id;
        // Set the value in the pkpOwners map from the domainWallets map to avoid memory storage.
        s().pkpOwners[pkpTokenId] = s().domainWallets[id];
        // Set the domain as a lookup to the pkp
        s().ownerLookup[uri] = pkpTokenId;

        emit Registered(id, userId, ttl, pkpTokenId);
    }

    function registerPKP(
        uint64 id,
        uint256 pkpTokenId
    ) public onlyOwner returns (bool) {
        if (s().domainWallets[id].isWallet == false) {
            return false;
        }

        s().domainWallets[id].pkpTokenId = pkpTokenId;

        return true;
    }

    function removeDomain(uint256 pkpTokenId) public onlyOwner returns (bool) {
        PKPHelper pkpHelper = PKPHelper(views().getPkpHelperAddress());
        pkpHelper.removePkpMetadata(pkpTokenId);

        return _removeDomain(pkpTokenId);
    }

    function _removeDomain(uint256 pkpTokenId) internal returns (bool) {
        if (s().pkpOwners[pkpTokenId].isWallet == false) {
            return false;
        }

        uint64 id = s().pkpOwners[pkpTokenId].id;

        delete s().pkpOwners[pkpTokenId]; // delete the pkp ownership relationship
        delete s().ownerLookup[s().domainWallets[id].uri];

        s().domainWallets[id].isRegistered = false;
        s().domainWallets[id].isWallet = false;
        s().domainWallets[id].pkpTokenId = 0; // null the tokenId since the owner is no longer this pkp
        s().domainWallets[id].user_id = hex"00"; // 0 out the bytes for the user id
        s().domainWallets[id].cname = hex"00";
        emit Removed(id, s().domainWallets[id].user_id);

        return true;
    }

    function updateDomainRecord(
        uint256 pkpTokenId,
        bytes memory record
    ) public onlyOwner returns (bool) {
        if (s().pkpOwners[pkpTokenId].isWallet == false) {
            return false;
        }
        s().pkpOwners[pkpTokenId].cname = record;
        s().domainWallets[s().pkpOwners[pkpTokenId].id].cname = s()
            .pkpOwners[pkpTokenId]
            .cname;
        return true;
    }

    function revokeDomain(uint256 pkpTokenId) public onlyOwner returns (bool) {
        PKPHelper pkpHelper = PKPHelper(views().getPkpHelperAddress());
        pkpHelper.removePkpMetadata(pkpTokenId);
        return _revokeDomain(pkpTokenId);
    }

    function _revokeDomain(uint256 pkpTokenId) internal returns (bool) {
        if (s().pkpOwners[pkpTokenId].isRegistered == false) {
            return false;
        }

        s().pkpOwners[pkpTokenId].isRegistered = false;
        s().pkpOwners[pkpTokenId].pkpTokenId = 0; // null the tokenId since the owner is no longer this pkp;
        s().pkpOwners[pkpTokenId].user_id = hex"00"; // 0 out the bytes for the user id
        s().pkpOwners[pkpTokenId].cname = hex"00"; // 0 out the bytes for the record

        s().domainWallets[s().pkpOwners[pkpTokenId].id] = s().pkpOwners[
            pkpTokenId
        ];

        emit Revoked(pkpTokenId, s().pkpOwners[pkpTokenId].uri);

        return true;
    }

    function registerDomainAndMintNext(
        bytes memory userId,
        bytes memory uri,
        uint ttl,
        uint256[] memory permittedAuthMethodTypes,
        bytes[] memory permittedAuthMethodIds,
        bytes[] memory permittedAuthMethodPubkeys,
        uint256[][] memory permittedAuthMethodScopes,
        string[] memory nftMetadata
    ) public payable onlyOwner returns (uint256) {
        PKPHelper pkpHelper = PKPHelper(views().getPkpHelperAddress());

        views().checkRegistration(uri);

        if (nftMetadata.length != 2) {
            revert InvalidNftMetadataCollectionLength(nftMetadata.length, 2);
        }
        // pass the message value down to the minting so the pkp mint has the correct mint cost.
        uint256 tokenId = pkpHelper.mintNextAndAddDomainWalletMetadata{
            value: msg.value
        }(
            2,
            permittedAuthMethodTypes,
            permittedAuthMethodIds,
            permittedAuthMethodPubkeys,
            permittedAuthMethodScopes,
            nftMetadata,
            true,
            true
        );

        _registerDomain(userId, uri, tokenId, ttl);

        return tokenId;
    }
}
