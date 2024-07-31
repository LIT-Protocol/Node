//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { LibDomainWalletRegistryStorage } from "./LibDomainWalletRegistryStorage.sol";

contract DomainWalletRegistryViewsFacet {
    /* ========== ERRORS ========== */
    error DomainAlreadyRegistered(bytes uri, uint256 pkpTokenId);

    /* ========== VIEWS ========== */

    function getStorage()
        internal
        pure
        returns (
            LibDomainWalletRegistryStorage.DomainWalletRegistryStorage storage
        )
    {
        return LibDomainWalletRegistryStorage.getStorage();
    }

    function getDomainCharacterLimit() public view returns (uint) {
        return getStorage().domainCharLimit;
    }

    function getDomainWalletRegistryAddress() public view returns (address) {
        return
            getStorage().contractResolver.getContract(
                getStorage().contractResolver.DOMAIN_WALLET_REGISTRY(),
                getStorage().env
            );
    }

    function getPkpHelperAddress() public view returns (address) {
        return
            getStorage().contractResolver.getContract(
                getStorage().contractResolver.PKP_HELPER_CONTRACT(),
                getStorage().env
            );
    }

    function getPkpTokenId(uint64 id) public view returns (uint256) {
        return
            getStorage()
                .pkpOwners[getStorage().domainWallets[id].pkpTokenId]
                .pkpTokenId;
    }

    function getDomainIdByTokenId(
        uint256 pkpTokenId
    ) public view returns (uint64) {
        return getStorage().pkpOwners[pkpTokenId].id;
    }

    function isOwner(uint256 pkpTokenId) public view returns (bool) {
        return getStorage().pkpOwners[pkpTokenId].isRegistered;
    }

    function isRouted(uint256 pkpTokenId) public view returns (bool) {
        return getStorage().pkpOwners[pkpTokenId].isRegistered;
    }

    function hasOwner(uint256 pkpTokenId) public view returns (bool) {
        return getStorage().pkpOwners[pkpTokenId].isRegistered;
    }

    function getDomainTokenIdByUri(
        bytes memory uri
    ) public view returns (uint256) {
        return getStorage().ownerLookup[uri];
    }

    function getDomainUri(
        uint256 pkpTokenId
    ) public view returns (bytes memory) {
        return getStorage().pkpOwners[pkpTokenId].uri;
    }

    function getExpiration(uint256 pkpTokenId) public view returns (uint) {
        return getStorage().pkpOwners[pkpTokenId].ttl;
    }

    function getRecord(uint256 pkpTokenId) public view returns (bytes memory) {
        return getStorage().pkpOwners[pkpTokenId].cname;
    }

    function checkRegistration(bytes memory uri) public view {
        if (getStorage().ownerLookup[uri] != 0) {
            revert DomainAlreadyRegistered(uri, getStorage().ownerLookup[uri]);
        }
    }
}
