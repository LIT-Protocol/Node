//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ContractResolver } from "../../lit-core/ContractResolver.sol";

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

library LibDomainWalletRegistryStorage {
    bytes32 constant DOMAIN_WALLET_REGISTRY_POSITION =
        keccak256("domainwalletregistry.storage");

    struct DomainWalletRegistryStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        // id to domain wallet instance
        mapping(uint64 => DomainWallet) domainWallets;
        // pkpTokenId to DomainWallet
        mapping(uint256 => DomainWallet) pkpOwners;
        // mapping of domain to tokenId for look ups
        mapping(bytes => uint256) ownerLookup;
        // counter for tracking registrations
        uint64 idCounter;
        //tracks max characters allowed in a domain name, set in the constructor
        uint domainCharLimit;
    }

    // Return storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (DomainWalletRegistryStorage storage storageStruct)
    {
        bytes32 position = DOMAIN_WALLET_REGISTRY_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
