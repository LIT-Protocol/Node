//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/access/AccessControl.sol";
import { DomainWalletOracle } from "./DomainWalletOracle.sol";
import { PKPHelper } from "./../lit-node/PKPHelper.sol";
import "hardhat/console.sol";
import { ContractResolver } from "../lit-core/ContractResolver.sol";

contract DomainWalletRegistry is AccessControl {
    ContractResolver public contractResolver;
    ContractResolver.Env public env;
    address private deployer;

    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN"); // 0xdf8b4c520ffe197c5343c6f5aec59570151ef9a492f2c624fd45ddde6135ec42

    error NonAdminCaller(address adminAddress, address account);
    error InvalidNftMetadataCollectionLength(
        uint256 metadataCount,
        uint256 validMetadataCount
    );

    constructor(address _resolver, ContractResolver.Env _env) {
        deployer = msg.sender;
        contractResolver = ContractResolver(_resolver);
        env = _env;

        _grantRole(ADMIN_ROLE, deployer);
        _setRoleAdmin(ADMIN_ROLE, ADMIN_ROLE);
    }

    function getDomainWalletRegistryAddress() internal view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.DOMAIN_WALLET_ORACLE(),
                env
            );
    }

    function getPkpHelperAddress() internal view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.PKP_HELPER_CONTRACT(),
                env
            );
    }

    function getPkpTokenId(uint64 id) public view returns (uint256) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.getPkpTokenId(id);
    }

    function getDomainIdByTokenId(
        uint256 pkpTokenId
    ) public view returns (uint64) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.getDomainIdByTokenId(pkpTokenId);
    }

    function isOwner(uint256 pkpTokenId) public view returns (bool) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.isOwner(pkpTokenId);
    }

    function hasExpired(uint256 pkpTokenId) public returns (bool) {
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert NonAdminCaller(deployer, msg.sender);
        }
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.hasExpired(pkpTokenId);
    }

    function isRouted(uint256 pkpTokenId) public view returns (bool) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.isRouted(pkpTokenId);
    }

    function hasOwner(uint256 pkpTokenId) public view returns (bool) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.hasOwner(pkpTokenId);
    }

    function getDomainUri(
        uint256 pkpTokenId
    ) public view returns (bytes memory) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.getDomainUri(pkpTokenId);
    }

    function getExpiration(uint256 pkpTokenId) public view returns (uint) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.getExpiration(pkpTokenId);
    }

    function getDomainTokenIdByUri(
        bytes memory uri
    ) public view returns (uint256) {
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.getDomainTokenIdByUri(uri);
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
    ) public payable returns (uint256) {
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert NonAdminCaller(deployer, msg.sender);
        }
        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        PKPHelper pkpHelper = PKPHelper(getPkpHelperAddress());

        domainWalletOracle.checkRegistration(uri);

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

        domainWalletOracle.registerDomain(userId, uri, tokenId, ttl);

        return tokenId;
    }

    function registerDomain(
        bytes memory userId,
        bytes memory uri,
        uint ttl,
        uint256 pkpTokenId,
        string[] memory nftMetadata
    ) public returns (uint256) {
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert NonAdminCaller(deployer, msg.sender);
        }

        require(
            nftMetadata.length == 2,
            "DomainWalletRegistry: metadata name and url must be set in metadata"
        );

        if (nftMetadata.length != 2) {
            revert InvalidNftMetadataCollectionLength(nftMetadata.length, 2);
        }

        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        PKPHelper pkpHelper = PKPHelper(getPkpHelperAddress());
        domainWalletOracle.registerDomain(userId, uri, pkpTokenId, ttl);
        pkpHelper.setPkpMetadata(pkpTokenId, nftMetadata);
        return pkpTokenId;
    }

    function setPKPMetadata(
        uint256 pkpTokenId,
        string[] memory nftMetadata
    ) public {
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert NonAdminCaller(deployer, msg.sender);
        }

        PKPHelper pkpHelper = PKPHelper(getPkpHelperAddress());
        pkpHelper.setPkpMetadata(pkpTokenId, nftMetadata);
    }

    function registerPKP(uint64 id, uint256 pkpTokenId) public returns (bool) {
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert NonAdminCaller(deployer, msg.sender);
        }

        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        return domainWalletOracle.registerPKP(id, pkpTokenId);
    }

    function removeDomain(uint256 pkpTokenId) public returns (bool) {
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert NonAdminCaller(deployer, msg.sender);
        }

        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        PKPHelper pkpHelper = PKPHelper(getPkpHelperAddress());
        pkpHelper.removePkpMetadata(pkpTokenId);

        return domainWalletOracle.removeDomain(pkpTokenId);
    }

    function revokeDomain(uint256 pkpTokenId) public returns (bool) {
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert NonAdminCaller(deployer, msg.sender);
        }

        DomainWalletOracle domainWalletOracle = DomainWalletOracle(
            getDomainWalletRegistryAddress()
        );
        PKPHelper pkpHelper = PKPHelper(getPkpHelperAddress());
        pkpHelper.removePkpMetadata(pkpTokenId);
        return domainWalletOracle.revokeDomain(pkpTokenId);
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
