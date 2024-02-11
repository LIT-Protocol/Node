//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ContractResolver } from "../lit-core/ContractResolver.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "hardhat/console.sol";
struct DomainWallet {
    bytes user_id;
    uint64 id;
    uint256 pkpTokenId;
    bytes uri;
    uint ttl;
    bool isWallet;
    bool isRegistered;
    bytes cname;
}

contract DomainWalletOracle is AccessControl {
    // id to domain wallet instance
    mapping(uint64 => DomainWallet) domainWallets;
    // pkpTokenId to DomainWallet
    mapping(uint256 => DomainWallet) pkpOwners;
    // mapping of domain to tokenId for look ups
    mapping(bytes => uint256) ownerLookup;

    uint calcTime = 0;
    uint nonce = 0;
    uint64 idCounter = 0;

    ContractResolver public contractResolver;
    ContractResolver.Env public env;

    /* ========== ROLES ========== */
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN"); // 0xdf8b4c520ffe197c5343c6f5aec59570151ef9a492f2c624fd45ddde6135ec42

    /* ========== EVENTS ========== */
    event Registered(uint64 id, bytes subDomain, uint ttl, uint256 tokenId);
    event Removed(uint256 tokenId, bytes subDomain);
    event Revoked(uint256 tokenId, bytes subDomain);
    event Expired(bytes subDomain, uint256 tokenId, uint ttl);

    /* ========== ERRORS ========== */
    error NonRegistryCaller(address registryAddress, address account);
    error DomainAlreadyRegistered(bytes uri, uint256 pkpTokenId);

    /* ========== CONSTRUCTOR ========== */
    constructor(address _contractResolver, ContractResolver.Env _env) {
        contractResolver = ContractResolver(_contractResolver);
        env = _env;
        _grantRole(ADMIN_ROLE, msg.sender);
    }

    function getDomainWalletRegistryAddress() internal view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.DOMAIN_WALLET_REGISTRY(),
                env
            );
    }

    function isOwner(uint256 pkpTokenId) public view returns (bool) {
        return pkpOwners[pkpTokenId].isRegistered;
    }

    function isRouted(uint256 pkpTokenId) public view returns (bool) {
        return pkpOwners[pkpTokenId].isRegistered;
    }

    function hasOwner(uint256 pkpTokenId) public view returns (bool) {
        return pkpOwners[pkpTokenId].isRegistered;
    }

    function hasExpired(uint256 pkpTokenId) public returns (bool) {
        uint ttl = pkpOwners[pkpTokenId].ttl;
        bytes memory uri = pkpOwners[pkpTokenId].uri;
        bool isExpired = ttl < block.timestamp;

        if (isExpired) {
            emit Expired(uri, pkpTokenId, ttl);
            return true;
        }

        return false;
    }

    function getDomainTokenIdByUri(
        bytes memory uri
    ) public view returns (uint256) {
        return ownerLookup[uri];
    }

    function getDomainUri(
        uint256 pkpTokenId
    ) public view returns (bytes memory) {
        return pkpOwners[pkpTokenId].uri;
    }

    function getExpiration(uint256 pkpTokenId) public view returns (uint) {
        return pkpOwners[pkpTokenId].ttl;
    }

    function getPkpTokenId(uint64 id) public view returns (uint256) {
        return pkpOwners[domainWallets[id].pkpTokenId].pkpTokenId;
    }

    function getDomainIdByTokenId(
        uint256 pkpTokenId
    ) public view returns (uint64) {
        return pkpOwners[pkpTokenId].id;
    }

    function getRecord(uint256 pkpTokenId) public view returns (bytes memory) {
        return pkpOwners[pkpTokenId].cname;
    }

    function checkRegistration(bytes memory uri) public view {
        if (ownerLookup[uri] != 0) {
            revert DomainAlreadyRegistered(uri, ownerLookup[uri]);
        }
    }

    function registerDomain(
        bytes memory userId,
        bytes memory uri,
        uint256 pkpTokenId,
        uint ttl
    ) public {
        if (
            msg.sender != getDomainWalletRegistryAddress() &&
            !hasRole(ADMIN_ROLE, msg.sender)
        ) {
            revert NonRegistryCaller(
                getDomainWalletRegistryAddress(),
                msg.sender
            );
        }

        if (ownerLookup[uri] != 0) {
            revert DomainAlreadyRegistered(uri, ownerLookup[uri]);
        }
        idCounter = idCounter + 1;
        uint64 id = idCounter;
        domainWallets[id].user_id = userId;
        domainWallets[id].pkpTokenId = pkpTokenId;
        domainWallets[id].uri = uri;
        domainWallets[id].ttl = ttl;
        domainWallets[id].isRegistered = true;
        domainWallets[id].isWallet = true;
        domainWallets[id].id = id;
        // Set the value in the pkpOwners map from the domainWallets map to avoid memory storage.
        pkpOwners[pkpTokenId] = domainWallets[id];
        // Set the domain as a lookup to the pkp
        ownerLookup[uri] = pkpTokenId;

        emit Registered(id, userId, ttl, pkpTokenId);
    }

    function registerPKP(uint64 id, uint256 pkpTokenId) public returns (bool) {
        if (
            msg.sender != getDomainWalletRegistryAddress() &&
            !hasRole(ADMIN_ROLE, msg.sender)
        ) {
            revert NonRegistryCaller(
                getDomainWalletRegistryAddress(),
                msg.sender
            );
        }

        if (domainWallets[id].isWallet == false) {
            return false;
        }

        domainWallets[id].pkpTokenId = pkpTokenId;

        return true;
    }

    function removeDomain(uint256 pkpTokenId) public returns (bool) {
        if (
            msg.sender != getDomainWalletRegistryAddress() &&
            !hasRole(ADMIN_ROLE, msg.sender)
        ) {
            revert NonRegistryCaller(
                getDomainWalletRegistryAddress(),
                msg.sender
            );
        }

        if (pkpOwners[pkpTokenId].isWallet == false) {
            return false;
        }

        uint64 id = pkpOwners[pkpTokenId].id;

        delete pkpOwners[pkpTokenId]; // delete the pkp ownership relationship
        delete ownerLookup[domainWallets[id].uri];

        domainWallets[id].isRegistered = false;
        domainWallets[id].isWallet = false;
        domainWallets[id].pkpTokenId = 0; // null the tokenId since the owner is no longer this pkp
        domainWallets[id].user_id = hex"00"; // 0 out the bytes for the user id
        domainWallets[id].cname = hex"00";
        emit Removed(id, domainWallets[id].user_id);

        return true;
    }

    function updateDomainRecord(
        uint256 pkpTokenId,
        bytes memory record
    ) public returns (bool) {
        if (
            msg.sender != getDomainWalletRegistryAddress() &&
            !hasRole(ADMIN_ROLE, msg.sender)
        ) {
            revert NonRegistryCaller(
                getDomainWalletRegistryAddress(),
                msg.sender
            );
        }

        if (pkpOwners[pkpTokenId].isWallet == false) {
            return false;
        }
        pkpOwners[pkpTokenId].cname = record;
        domainWallets[pkpOwners[pkpTokenId].id].cname = pkpOwners[pkpTokenId]
            .cname;
        return true;
    }

    function revokeDomain(uint256 pkpTokenId) public returns (bool) {
        if (
            msg.sender != getDomainWalletRegistryAddress() &&
            !hasRole(ADMIN_ROLE, msg.sender)
        ) {
            revert NonRegistryCaller(
                getDomainWalletRegistryAddress(),
                msg.sender
            );
        }

        if (pkpOwners[pkpTokenId].isRegistered == false) {
            return false;
        }

        pkpOwners[pkpTokenId].isRegistered = false;
        pkpOwners[pkpTokenId].pkpTokenId = 0; // null the tokenId since the owner is no longer this pkp;
        pkpOwners[pkpTokenId].user_id = hex"00"; // 0 out the bytes for the user id
        pkpOwners[pkpTokenId].cname = hex"00"; // 0 out the bytes for the record

        domainWallets[pkpOwners[pkpTokenId].id] = pkpOwners[pkpTokenId];

        emit Revoked(pkpTokenId, pkpOwners[pkpTokenId].uri);

        return true;
    }

    function addAdmin(address newAdmin) public onlyRole(ADMIN_ROLE) {
        _grantRole(ADMIN_ROLE, newAdmin);
    }

    function removeAdmin(
        address adminBeingRemoved
    ) public onlyRole(ADMIN_ROLE) {
        require(
            adminBeingRemoved != msg.sender,
            "Cannot remove self as admin.  Have the new admin do it."
        );
        _grantRole(ADMIN_ROLE, adminBeingRemoved);
    }
}
