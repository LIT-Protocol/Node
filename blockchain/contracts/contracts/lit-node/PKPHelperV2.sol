//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IERC721Receiver } from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import { ContractResolver } from "../lit-core/ContractResolver.sol";
import { PKPNFTFacet } from "./PKPNFT/PKPNFTFacet.sol";
import "hardhat/console.sol";

import { LibPKPPermissionsStorage } from "./PKPPermissions/LibPKPPermissionsStorage.sol";
import { PKPPermissionsFacet } from "./PKPPermissions/PKPPermissionsFacet.sol";
import { LibPKPNFTStorage } from "./PKPNFT/LibPKPNFTStorage.sol";

/// @title Version 2 of the stateless PKP Helper Contract
///
/// @dev Permits a bunch of atomic operations on the PKP NFTs
contract PKPHelperV2 is Ownable, IERC721Receiver {
    /* ========== CUSTOM STRUCTS ========== */
    struct NewPKPParams {
        uint256 keyType;
        uint256[] permittedAuthMethodTypes;
        bytes[] permittedAuthMethodIds;
        bytes[] permittedAuthMethodPubkeys;
        uint256[][] permittedAuthMethodScopes;
        bool addPkpEthAddressAsPermittedAddress;
        uint256[] pkpEthAddressScopes;
        bool sendPkpToItself;
        bool burnPkp;
        address sendToAddressAfterMinting;
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

    /* ========== MUTATIVE FUNCTIONS ========== */

    function mintNextAndAddAuthMethods(
        NewPKPParams memory params
    ) public payable returns (uint256) {
        return mintNextAndAddAuthMethodsWithTypes(params);
    }

    function mintNextAndAddAuthMethodsWithTypes(
        NewPKPParams memory params
    ) public payable returns (uint256) {
        // mint the nft and forward the funds
        uint256 tokenId = PKPNFTFacet(getPkpNftAddress()).mintNext{
            value: msg.value
        }(params.keyType);

        require(
            params.permittedAuthMethodTypes.length ==
                params.permittedAuthMethodIds.length,
            "PKPHelper: auth method type and id array lengths must match"
        );
        require(
            params.permittedAuthMethodTypes.length ==
                params.permittedAuthMethodPubkeys.length,
            "PKPHelper: auth method type and pubkey array lengths must match"
        );
        require(
            params.permittedAuthMethodTypes.length ==
                params.permittedAuthMethodScopes.length,
            "PKPHelper: auth method type and scopes array lengths must match"
        );

        // permit the auth methoda
        if (params.permittedAuthMethodTypes.length != 0) {
            for (
                uint256 i = 0;
                i < params.permittedAuthMethodTypes.length;
                i++
            ) {
                PKPPermissionsFacet(getPkpPermissionsAddress())
                    .addPermittedAuthMethod(
                        tokenId,
                        LibPKPPermissionsStorage.AuthMethod(
                            params.permittedAuthMethodTypes[i],
                            params.permittedAuthMethodIds[i],
                            params.permittedAuthMethodPubkeys[i]
                        ),
                        params.permittedAuthMethodScopes[i]
                    );
            }
        }

        address pkpEthAddress = PKPPermissionsFacet(getPkpPermissionsAddress())
            .getEthAddress(tokenId);

        // add the pkp eth address as a permitted address
        if (params.addPkpEthAddressAsPermittedAddress) {
            PKPPermissionsFacet(getPkpPermissionsAddress()).addPermittedAddress(
                tokenId,
                pkpEthAddress,
                params.pkpEthAddressScopes
            );
        }

        if (params.sendPkpToItself) {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                pkpEthAddress,
                tokenId
            );
        } else if (params.burnPkp) {
            PKPNFTFacet(getPkpNftAddress()).burn(tokenId);
        } else if (params.sendToAddressAfterMinting != address(0)) {
            PKPNFTFacet(getPkpNftAddress()).safeTransferFrom(
                address(this),
                params.sendToAddressAfterMinting,
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
