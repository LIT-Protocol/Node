//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ERC721Upgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/token/ERC721/ERC721Upgradeable.sol";
import { IERC721 } from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import { ERC721BurnableUpgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/token/ERC721/extensions/ERC721BurnableUpgradeable.sol";
import { ERC721EnumerableUpgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/token/ERC721/extensions/ERC721EnumerableUpgradeable.sol";
import { IERC721Enumerable } from "@openzeppelin/contracts/token/ERC721/extensions/IERC721Enumerable.sol";
import { IERC721Metadata } from "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";
import { Base64 } from "@openzeppelin/contracts/utils/Base64.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { ERC165Upgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/utils/introspection/ERC165Upgradeable.sol";
import { IERC165Upgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/utils/introspection/IERC165Upgradeable.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { LibPKPNFTStorage } from "./LibPKPNFTStorage.sol";
import { IPubkeyRouter } from "../PubkeyRouter/LibPubkeyRouterStorage.sol";
import { PubkeyRouterFacet } from "../PubkeyRouter/PubkeyRouterFacet.sol";
import { PKPNFTMetadata } from "../PKPNFTMetadata.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { PKPPermissionsFacet } from "../PKPPermissions/PKPPermissionsFacet.sol";

import "hardhat/console.sol";

// TODO: tests for the mintGrantAndBurn function, withdraw function, some of the setters, transfer function, freeMint and freeMintGrantAndBurn

/// @title Programmable Keypair NFT
///
/// @dev This is the contract for the PKP NFTs
///
/// Simply put, whomever owns a PKP NFT can ask that PKP to sign a message.
/// The owner can also grant signing permissions to other eth addresses
/// or lit actions
contract PKPNFTFacet is
    ERC721Upgradeable,
    ERC721BurnableUpgradeable,
    ERC721EnumerableUpgradeable
{
    using Strings for uint256;

    error CallerNotOwner();

    function initialize() public initializer {
        __ERC721_init("Programmable Keypair", "PKP");
    }

    /* ========== Modifiers ========== */

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibPKPNFTStorage.PKPNFTStorage storage)
    {
        return LibPKPNFTStorage.getStorage();
    }

    /// get the staking address from the resolver
    function getStakingAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.STAKING_CONTRACT(),
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

    function getPkpNftMetadataAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.PKP_NFT_METADATA_CONTRACT(),
                s().env
            );
    }

    function getPkpPermissionsAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.PKP_PERMISSIONS_CONTRACT(),
                s().env
            );
    }

    function mintCost() public view returns (uint256) {
        return s().mintCost;
    }

    function freeMintSigner() public view returns (address) {
        return s().freeMintSigner;
    }

    function redeemedFreeMintIds(uint256 tokenId) public view returns (bool) {
        return s().redeemedFreeMintIds[tokenId];
    }

    /// get the eth address for the keypair
    function getEthAddress(uint256 tokenId) public view returns (address) {
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        return router.getEthAddress(tokenId);
    }

    /// includes the 0x04 prefix so you can pass this directly to ethers.utils.computeAddress
    function getPubkey(uint256 tokenId) public view returns (bytes memory) {
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        return router.getPubkey(tokenId);
    }

    /**
     * @dev See {IERC165-supportsInterface}.
     */
    function supportsInterface(
        bytes4 interfaceId
    )
        public
        view
        virtual
        override(ERC721Upgradeable, ERC721EnumerableUpgradeable)
        returns (bool)
    {
        return
            super.supportsInterface(interfaceId) ||
            LibDiamond.diamondStorage().supportedInterfaces[interfaceId];
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId,
        uint256 batchSize
    )
        internal
        virtual
        override(ERC721Upgradeable, ERC721EnumerableUpgradeable)
    {
        ERC721EnumerableUpgradeable._beforeTokenTransfer(
            from,
            to,
            tokenId,
            batchSize
        );
    }

    function tokenURI(
        uint256 tokenId
    ) public view override returns (string memory) {
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        bytes memory pubKey = router.getPubkey(tokenId);
        address ethAddress = router.getEthAddress(tokenId);

        PKPNFTMetadata pkpNftMetadata = PKPNFTMetadata(
            getPkpNftMetadataAddress()
        );
        return pkpNftMetadata.tokenURI(tokenId, pubKey, ethAddress);
    }

    // Builds a prefixed hash to mimic the behavior of eth_sign.
    function prefixed(bytes32 hash) public pure returns (bytes32) {
        return
            keccak256(
                abi.encodePacked("\x19Ethereum Signed Message:\n32", hash)
            );
    }

    function exists(uint256 tokenId) public view returns (bool) {
        return _exists(tokenId);
    }

    function getNextDerivedKeyId() public view returns (bytes32) {
        uint256 tokenCount = totalSupply() + 1;
        // hash(tokenCount, previousBlockHash)
        bytes32 derivedKeyId = keccak256(
            abi.encodePacked(tokenCount, blockhash(block.number - 1))
        );

        return derivedKeyId;
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function mintNext(uint256 keyType) public payable returns (uint256) {
        require(msg.value == s().mintCost, "You must pay exactly mint cost");
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        bytes32 derivedKeyId = getNextDerivedKeyId();
        bytes memory pubkey = router.getDerivedPubkey(
            getStakingAddress(),
            derivedKeyId
        );
        uint256 tokenId = uint256(keccak256(pubkey));
        routeDerivedKey(keyType, derivedKeyId);
        _mintWithoutValueCheck(tokenId, msg.sender);
        return tokenId;
    }

    function claimAndMint(
        uint256 keyType,
        bytes32 derivedKeyId,
        IPubkeyRouter.Signature[] memory signatures,
        address stakingContractAddress
    ) public payable returns (uint256) {
        require(msg.value == s().mintCost, "You must pay exactly mint cost");
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        router.checkNodeSignatures(
            signatures,
            abi.encodePacked(derivedKeyId),
            stakingContractAddress
        );
        bytes memory pubkey = router.getDerivedPubkey(
            stakingContractAddress,
            derivedKeyId
        );
        uint256 tokenId = uint256(keccak256(pubkey));

        routeDerivedKey(keyType, derivedKeyId);
        _mintWithoutValueCheck(tokenId, msg.sender);

        return tokenId;
    }

    function mintGrantAndBurnNext(
        uint256 keyType,
        bytes memory ipfsCID
    ) public payable returns (uint256) {
        require(msg.value == s().mintCost, "You must pay exactly mint cost");
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        bytes32 derivedKeyId = getNextDerivedKeyId();
        bytes memory pubkey = router.getDerivedPubkey(
            getStakingAddress(),
            derivedKeyId
        );
        uint256 tokenId = uint256(keccak256(pubkey));
        routeDerivedKey(keyType, derivedKeyId);
        _mintWithoutValueCheck(tokenId, address(this));
        uint256[] memory scopes = new uint256[](1);
        scopes[0] = 1;
        PKPPermissionsFacet(getPkpPermissionsAddress()).addPermittedAction(
            tokenId,
            ipfsCID,
            scopes
        );
        _burn(tokenId);
        return tokenId;
    }

    function routeDerivedKey(uint256 keyType, bytes32 derivedKeyId) internal {
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        bytes memory pubkey = router.getDerivedPubkey(
            getStakingAddress(),
            derivedKeyId
        );
        uint256 tokenId = uint256(keccak256(pubkey));

        PubkeyRouterFacet(getRouterAddress()).setRoutingData(
            tokenId,
            pubkey,
            address(getStakingAddress()),
            keyType,
            derivedKeyId
        );
    }

    function _mintWithoutValueCheck(uint256 tokenId, address to) internal {
        PubkeyRouterFacet router = PubkeyRouterFacet(getRouterAddress());
        require(router.isRouted(tokenId), "This PKP has not been routed yet");

        if (to == address(this)) {
            // permit unsafe transfer only to this contract, because it's going to be burned
            _mint(to, tokenId);
        } else {
            _safeMint(to, tokenId);
        }
        emit PKPMinted(tokenId, getPubkey(tokenId));
    }

    function setMintCost(uint256 newMintCost) public onlyOwner {
        s().mintCost = newMintCost;
        emit MintCostSet(newMintCost);
    }

    function setFreeMintSigner(address newFreeMintSigner) public onlyOwner {
        s().freeMintSigner = newFreeMintSigner;
        emit FreeMintSignerSet(newFreeMintSigner);
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        s().contractResolver = ContractResolver(newResolverAddress);
        emit ContractResolverAddressSet(newResolverAddress);
    }

    function withdraw() public onlyOwner {
        uint256 withdrawAmount = address(this).balance;
        (bool sent, ) = payable(msg.sender).call{ value: withdrawAmount }("");
        require(sent);
        emit Withdrew(withdrawAmount);
    }

    /* ========== EVENTS ========== */

    event MintCostSet(uint256 newMintCost);
    event FreeMintSignerSet(address indexed newFreeMintSigner);
    event Withdrew(uint256 amount);
    event PKPMinted(uint256 indexed tokenId, bytes pubkey);
    event ContractResolverAddressSet(address newResolverAddress);
}
