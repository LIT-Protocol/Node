//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { PKPPermissions } from "./PKPPermissions.sol";
import { PKPNFT } from "./PKPNFT.sol";
import { PKPNFTMetadata } from "./PKPNFTMetadata.sol";
import { Base64 } from "@openzeppelin/contracts/utils/Base64.sol";
import { IERC721Receiver } from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import { ContractResolver } from "../lit-core/ContractResolver.sol";
import { PKPNFTFacet } from "./PKPNFT/PKPNFTFacet.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "hardhat/console.sol";

import { LibPKPPermissionsStorage } from "./PKPPermissions/LibPKPPermissionsStorage.sol";
import { PKPPermissionsFacet } from "./PKPPermissions/PKPPermissionsFacet.sol";
import { LibPKPNFTStorage } from "./PKPNFT/LibPKPNFTStorage.sol";

// TODO: tests for the mintGrantAndBurn function, withdraw function, some of the setters, transfer function, freeMint and freeMintGrantAndBurn

/// @title Programmable Keypair NFT
///
/// @dev This is the contract for the PKP NFTs
///
/// Simply put, whomever owns a PKP NFT can ask that PKP to sign a message.
/// The owner can also grant signing permissions to other eth addresses
/// or lit actions
contract PKPHelper is Ownable, IERC721Receiver, AccessControl {
    /* ========== CUSTOM STRUCTS ========== */
    struct AuthMethodData {
        uint256 keyType;
        bytes[] permittedIpfsCIDs;
        uint256[][] permittedIpfsCIDScopes;
        address[] permittedAddresses;
        uint256[][] permittedAddressScopes;
        uint256[] permittedAuthMethodTypes;
        bytes[] permittedAuthMethodIds;
        bytes[] permittedAuthMethodPubkeys;
        uint256[][] permittedAuthMethodScopes;
        bool addPkpEthAddressAsPermittedAddress;
        bool sendPkpToItself;
    }

    /* ========== STATE VARIABLES ========== */

    ContractResolver public contractResolver;
    ContractResolver.Env public env;

    /* ========== CONSTRUCTOR ========== */
    constructor(address _resolver, ContractResolver.Env _env) {
        contractResolver = ContractResolver(_resolver);
        env = _env;
    }

    /* ========== VIEWS ========== */
    function getPkpNftAddress() public view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.PKP_NFT_CONTRACT(),
                env
            );
    }

    function getPkpPermissionsAddress() public view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.PKP_PERMISSIONS_CONTRACT(),
                env
            );
    }

    function getPKPNftMetdataAddress() public view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.PKP_NFT_METADATA_CONTRACT(),
                env
            );
    }

    function getDomainWalletRegistry() public view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.DOMAIN_WALLET_REGISTRY(),
                env
            );
    }

    function getStakingAddress() public view returns (address) {
        return
            contractResolver.getContract(
                contractResolver.STAKING_CONTRACT(),
                env
            );
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function mintNextAndAddAuthMethods(
        uint256 keyType,
        uint256[] memory permittedAuthMethodTypes,
        bytes[] memory permittedAuthMethodIds,
        bytes[] memory permittedAuthMethodPubkeys,
        uint256[][] memory permittedAuthMethodScopes,
        bool addPkpEthAddressAsPermittedAddress,
        bool sendPkpToItself
    ) public payable returns (uint256) {
        return
            mintNextAndAddAuthMethodsWithTypes(
                keyType,
                new bytes[](0), // permitted ipfs CIDs
                new uint256[][](0), // permitted ipfs CIDs scopes
                new address[](0), // permitted addresses
                new uint256[][](0), // permitted addresses scopes
                permittedAuthMethodTypes,
                permittedAuthMethodIds,
                permittedAuthMethodPubkeys,
                permittedAuthMethodScopes,
                addPkpEthAddressAsPermittedAddress,
                sendPkpToItself
            );
    }

    /*
        Broken out into its own top level minting function as adding more parameters to
        mintNextAndAddAuthMethodsWithTypes throws compile time errors relating to the amount of variables overflowing
        the op code call stack.
        See here for more info: https://ethereum.stackexchange.com/questions/6061/error-while-compiling-stack-too-deep
    */
    function mintNextAndAddDomainWalletMetadata(
        uint256 keyType,
        uint256[] memory permittedAuthMethodTypes,
        bytes[] memory permittedAuthMethodIds,
        bytes[] memory permittedAuthMethodPubkeys,
        uint256[][] memory permittedAuthMethodScopes,
        string[] memory nftMetadata,
        bool addPkpEthAddressAsPermittedAddress,
        bool sendPkpToItself
    ) public payable returns (uint256) {
        require(
            msg.sender ==
                contractResolver.getContract(
                    contractResolver.DOMAIN_WALLET_REGISTRY(),
                    env
                ),
            "PKPHelper: only the Domain Wallet registry is allowed to mint domain wallets, who are you?"
        );

        // mint the nft and forward the funds
        uint256 tokenId = PKPNFTFacet(getPkpNftAddress()).mintNext{
            value: msg.value
        }(keyType);
        require(
            permittedAuthMethodTypes.length == permittedAuthMethodIds.length,
            "PKPHelper: auth method type and id array lengths must match"
        );
        require(
            permittedAuthMethodTypes.length ==
                permittedAuthMethodPubkeys.length,
            "PKPHelper: auth method type and pubkey array lengths must match"
        );
        require(
            permittedAuthMethodTypes.length == permittedAuthMethodScopes.length,
            "PKPHelper: auth method type and scopes array lengths must match"
        );

        // permit the auth method
        if (permittedAuthMethodTypes.length != 0) {
            for (uint256 i = 0; i < permittedAuthMethodTypes.length; i++) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAuthMethod(
                        tokenId,
                        LibPKPPermissionsStorage.AuthMethod(
                            permittedAuthMethodTypes[i],
                            permittedAuthMethodIds[i],
                            permittedAuthMethodPubkeys[i]
                        ),
                        permittedAuthMethodScopes[i]
                    );
            }
        }

        address pkpEthAddress = PKPPermissionsFacet(getPkpPermissionsAddress())
            .getEthAddress(tokenId);

        // add the pkp eth address as a permitted address
        if (addPkpEthAddressAsPermittedAddress) {
            PKPPermissionsFacet(getPkpPermissionsAddress()).addPermittedAddress(
                tokenId,
                pkpEthAddress,
                new uint256[](0)
            );
        }

        if (sendPkpToItself) {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                pkpEthAddress,
                tokenId
            );
        } else {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                msg.sender,
                tokenId
            );
        }

        if (nftMetadata.length > 0) {
            // first element is the full domain of the pkp (domain wallet url)
            PKPNFTMetadata(getPKPNftMetdataAddress()).setUrlForPKP(
                tokenId,
                nftMetadata[0]
            );
            // second element is the image url
            PKPNFTMetadata(getPKPNftMetdataAddress()).setProfileForPKP(
                tokenId,
                nftMetadata[1]
            );
        }

        return tokenId;
    }

    function mintNextAndAddAuthMethodsWithTypes(
        uint256 keyType,
        bytes[] memory permittedIpfsCIDs,
        uint256[][] memory permittedIpfsCIDScopes,
        address[] memory permittedAddresses,
        uint256[][] memory permittedAddressScopes,
        uint256[] memory permittedAuthMethodTypes,
        bytes[] memory permittedAuthMethodIds,
        bytes[] memory permittedAuthMethodPubkeys,
        uint256[][] memory permittedAuthMethodScopes,
        bool addPkpEthAddressAsPermittedAddress,
        bool sendPkpToItself
    ) public payable returns (uint256) {
        // mint the nft and forward the funds
        uint256 tokenId = PKPNFTFacet(getPkpNftAddress()).mintNext{
            value: msg.value
        }(keyType);

        // sanity checking array lengths
        require(
            permittedIpfsCIDs.length == permittedIpfsCIDScopes.length,
            "PKPHelper: ipfs cid and scope array lengths must match"
        );
        require(
            permittedAddresses.length == permittedAddressScopes.length,
            "PKPHelper: address and scope array lengths must match"
        );
        require(
            permittedAuthMethodTypes.length == permittedAuthMethodIds.length,
            "PKPHelper: auth method type and id array lengths must match"
        );
        require(
            permittedAuthMethodTypes.length ==
                permittedAuthMethodPubkeys.length,
            "PKPHelper: auth method type and pubkey array lengths must match"
        );
        require(
            permittedAuthMethodTypes.length == permittedAuthMethodScopes.length,
            "PKPHelper: auth method type and scopes array lengths must match"
        );

        // permit the action
        if (permittedIpfsCIDs.length != 0) {
            for (uint256 i = 0; i < permittedIpfsCIDs.length; i++) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAction(
                        tokenId,
                        permittedIpfsCIDs[i],
                        permittedIpfsCIDScopes[i]
                    );
            }
        }

        // permit the address
        if (permittedAddresses.length != 0) {
            for (uint256 i = 0; i < permittedAddresses.length; i++) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAddress(
                        tokenId,
                        permittedAddresses[i],
                        permittedAddressScopes[i]
                    );
            }
        }

        // permit the auth method
        if (permittedAuthMethodTypes.length != 0) {
            for (uint256 i = 0; i < permittedAuthMethodTypes.length; i++) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAuthMethod(
                        tokenId,
                        LibPKPPermissionsStorage.AuthMethod(
                            permittedAuthMethodTypes[i],
                            permittedAuthMethodIds[i],
                            permittedAuthMethodPubkeys[i]
                        ),
                        permittedAuthMethodScopes[i]
                    );
            }
        }

        address pkpEthAddress = PKPPermissionsFacet(getPkpPermissionsAddress())
            .getEthAddress(tokenId);

        // add the pkp eth address as a permitted address
        if (addPkpEthAddressAsPermittedAddress) {
            PKPPermissionsFacet(getPkpPermissionsAddress()).addPermittedAddress(
                tokenId,
                pkpEthAddress,
                new uint256[](0)
            );
        }

        if (sendPkpToItself) {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                pkpEthAddress,
                tokenId
            );
        } else {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                msg.sender,
                tokenId
            );
        }

        return tokenId;
    }

    // helper for the domain wallet registry registering pre existing pkps
    function setPkpMetadata(
        uint256 tokenId,
        string[] memory nftMetadata
    ) public {
        require(
            msg.sender ==
                contractResolver.getContract(
                    contractResolver.DOMAIN_WALLET_REGISTRY(),
                    env
                ),
            "PKPHelper: only the Domain Wallet registry is allowed to mint domain wallets, who are you?"
        );
        PKPNFTMetadata pkpNftMetadata = PKPNFTMetadata(
            getPKPNftMetdataAddress()
        );

        if (nftMetadata.length > 0) {
            // first element is a url name for the pkp (domain wallet url)
            pkpNftMetadata.setUrlForPKP(tokenId, nftMetadata[0]);
            // first element is a url name for the pkp (domain wallet profile image url)
            pkpNftMetadata.setProfileForPKP(tokenId, nftMetadata[1]);
        }
    }

    function removePkpMetadata(uint256 tokenId) public {
        require(
            msg.sender ==
                contractResolver.getContract(
                    contractResolver.DOMAIN_WALLET_REGISTRY(),
                    env
                ),
            "PKPHelper: only the Domain Wallet registry is allowed to mint domain wallets, who are you?"
        );
        PKPNFTMetadata pkpNftMetadata = PKPNFTMetadata(
            getPKPNftMetdataAddress()
        );
        pkpNftMetadata.removeProfileForPkp(tokenId);
        pkpNftMetadata.removeUrlForPKP(tokenId);
    }

    function claimAndMintNextAndAddAuthMethods(
        LibPKPNFTStorage.ClaimMaterial memory claimMaterial,
        AuthMethodData memory authMethodData
    ) public payable returns (uint256) {
        return
            claimAndMintNextAndAddAuthMethodsWithTypes(
                claimMaterial,
                authMethodData
            );
    }

    function claimAndMintNextAndAddAuthMethodsWithTypes(
        LibPKPNFTStorage.ClaimMaterial memory claimMaterial,
        AuthMethodData memory authMethodData
    ) public payable returns (uint256) {
        // Convert the claim material to the v2 struct
        LibPKPNFTStorage.ClaimMaterialV2
            memory claimMaterialV2 = LibPKPNFTStorage.ClaimMaterialV2(
                claimMaterial.keyType,
                claimMaterial.derivedKeyId,
                claimMaterial.signatures,
                getStakingAddress()
            );

        return
            claimAndMintNextAndAddAuthMethodsWithTypesV2(
                claimMaterialV2,
                authMethodData
            );
    }

    function claimAndMintNextAndAddAuthMethodsWithTypesV2(
        LibPKPNFTStorage.ClaimMaterialV2 memory claimMaterial,
        AuthMethodData memory authMethodData
    ) public payable returns (uint256) {
        require(
            claimMaterial.keyType == authMethodData.keyType,
            "PKPHelper: Claim key type must match Auth Method data key type"
        );
        // mint the nft and forward the funds
        uint256 tokenId = PKPNFTFacet(getPkpNftAddress()).claimAndMint{
            value: msg.value
        }(
            claimMaterial.keyType,
            claimMaterial.derivedKeyId,
            claimMaterial.signatures,
            claimMaterial.stakingContractAddress
        );

        require(
            authMethodData.permittedIpfsCIDs.length ==
                authMethodData.permittedIpfsCIDScopes.length,
            "PKPHelper: ipfs cid and scope array lengths must match"
        );
        // sanity checking array lengths
        require(
            authMethodData.permittedAddresses.length ==
                authMethodData.permittedAddressScopes.length,
            "PKPHelper: address and scope array lengths must match"
        );
        require(
            authMethodData.permittedAuthMethodTypes.length ==
                authMethodData.permittedAuthMethodIds.length,
            "PKPHelper: auth method type and id array lengths must match"
        );
        require(
            authMethodData.permittedAuthMethodTypes.length ==
                authMethodData.permittedAuthMethodPubkeys.length,
            "PKPHelper: auth method type and pubkey array lengths must match"
        );
        require(
            authMethodData.permittedAuthMethodTypes.length ==
                authMethodData.permittedAuthMethodScopes.length,
            "PKPHelper: auth method type and scopes array lengths must match"
        );

        if (authMethodData.permittedIpfsCIDs.length != 0) {
            for (
                uint256 i = 0;
                i < authMethodData.permittedIpfsCIDs.length;
                i++
            ) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAction(
                        tokenId,
                        authMethodData.permittedIpfsCIDs[i],
                        authMethodData.permittedIpfsCIDScopes[i]
                    );
            }
        }

        // permit the address
        if (authMethodData.permittedAddresses.length != 0) {
            for (
                uint256 i = 0;
                i < authMethodData.permittedAddresses.length;
                i++
            ) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAddress(
                        tokenId,
                        authMethodData.permittedAddresses[i],
                        authMethodData.permittedAddressScopes[i]
                    );
            }
        }

        // permit the auth method
        if (authMethodData.permittedAuthMethodTypes.length != 0) {
            for (
                uint256 i = 0;
                i < authMethodData.permittedAuthMethodTypes.length;
                i++
            ) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAuthMethod(
                        tokenId,
                        LibPKPPermissionsStorage.AuthMethod(
                            authMethodData.permittedAuthMethodTypes[i],
                            authMethodData.permittedAuthMethodIds[i],
                            authMethodData.permittedAuthMethodPubkeys[i]
                        ),
                        authMethodData.permittedAuthMethodScopes[i]
                    );
            }
        }

        address pkpEthAddress = PKPPermissionsFacet(getPkpPermissionsAddress())
            .getEthAddress(tokenId);

        // add the pkp eth address as a permitted address
        if (authMethodData.addPkpEthAddressAsPermittedAddress) {
            PKPPermissionsFacet(getPkpPermissionsAddress()).addPermittedAddress(
                tokenId,
                pkpEthAddress,
                new uint256[](0)
            );
        }

        if (authMethodData.sendPkpToItself) {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                pkpEthAddress,
                tokenId
            );
        } else {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                msg.sender,
                tokenId
            );
        }

        return tokenId;
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        contractResolver = ContractResolver(newResolverAddress);
        emit ContractResolverAddressSet(newResolverAddress);
    }

    function onERC721Received(
        address /* operator */,
        address /* from */,
        uint256 /* tokenId */,
        bytes calldata /* data */
    ) external view override returns (bytes4) {
        // only accept transfers from the pkpNft contract

        require(
            msg.sender == getPkpNftAddress(),
            "PKPHelper: only accepts transfers from the PKPNFT contract"
        );
        return this.onERC721Received.selector;
    }

    /* ========== EVENTS ========== */

    event ContractResolverAddressSet(address newResolverAddress);
}
