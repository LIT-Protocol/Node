//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";

import { PKPNFT } from "../PKPNFT.sol";
import { Staking } from "../Staking.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { IKeyDeriver } from "../HDKeyDeriver.sol";
import { StakingFacet } from "../Staking/StakingFacet.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { PKPNFTFacet } from "../PKPNFT/PKPNFTFacet.sol";

import { LibPubkeyRouterStorage, IPubkeyRouter } from "./LibPubkeyRouterStorage.sol";

import "hardhat/console.sol";

// TODO: make the tests send PKPNFT into the constructor
// TODO: test interaction between PKPNFT and this contract, like mint a keypair and see if you can access it
// TODO: setRoutingData() for a batch of keys

contract PubkeyRouterFacet {
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using EnumerableSet for EnumerableSet.UintSet;
    using BytesLib for bytes;

    /* ========== Errors ========== */
    error CallerNotOwner();

    /* ========== Modifiers ========== */
    modifier onlyPKPOwner(uint256 tokenId) {
        // check that user is allowed to set this
        PKPNFTFacet pkpNFT = PKPNFTFacet(getPkpNftAddress());
        address nftOwner = pkpNFT.ownerOf(tokenId);
        require(msg.sender == nftOwner, "Not PKP NFT owner");
        _;
    }

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibPubkeyRouterStorage.PubkeyRouterStorage storage)
    {
        return LibPubkeyRouterStorage.getStorage();
    }

    function ethAddressToPkpId(
        address ethAddress
    ) public view returns (uint256) {
        return s().ethAddressToPkpId[ethAddress];
    }

    function pubkeys(
        uint256 tokenId
    ) public view returns (LibPubkeyRouterStorage.PubkeyRoutingData memory) {
        return s().pubkeys[tokenId];
    }

    function getPkpNftAddress() public view returns (address) {
        return
            s().contractResolver.getContract(
                s().contractResolver.PKP_NFT_CONTRACT(),
                s().env
            );
    }

    /// get root keys for a given staking contract
    function getRootKeys(
        address stakingContract
    ) public view returns (IPubkeyRouter.RootKey[] memory) {
        return s().rootKeys[stakingContract];
    }

    /// get the routing data for a given key hash
    function getRoutingData(
        uint256 tokenId
    ) external view returns (LibPubkeyRouterStorage.PubkeyRoutingData memory) {
        return s().pubkeys[tokenId];
    }

    /// get if a given pubkey has routing data associated with it or not
    function isRouted(uint256 tokenId) public view returns (bool) {
        LibPubkeyRouterStorage.PubkeyRoutingData memory prd = s().pubkeys[
            tokenId
        ];
        return
            prd.pubkey.length != 0 &&
            prd.keyType != 0 &&
            prd.derivedKeyId != bytes32(0);
    }

    /// get the eth address for the keypair, as long as it's an ecdsa keypair
    function getEthAddress(uint256 tokenId) public view returns (address) {
        return deriveEthAddressFromPubkey(s().pubkeys[tokenId].pubkey);
    }

    /// includes the 0x04 prefix so you can pass this directly to ethers.utils.computeAddress
    function getPubkey(uint256 tokenId) public view returns (bytes memory) {
        return s().pubkeys[tokenId].pubkey;
    }

    function deriveEthAddressFromPubkey(
        bytes memory pubkey
    ) public pure returns (address) {
        // remove 0x04 prefix
        bytes32 hashed = keccak256(pubkey.slice(1, 64));
        return address(uint160(uint256(hashed)));
    }

    function checkNodeSignatures(
        IPubkeyRouter.Signature[] memory signatures,
        bytes memory signedMessage,
        address stakingContractAddress
    ) public view returns (bool) {
        StakingViewsFacet stakingContract = StakingViewsFacet(
            stakingContractAddress
        );
        require(
            signatures.length >=
                stakingContract.currentValidatorCountForConsensus(),
            "PubkeyRouter: incorrect number of signatures on a given root key"
        );
        for (uint256 i = 0; i < signatures.length; i++) {
            IPubkeyRouter.Signature memory sig = signatures[i];
            address signer = ECDSA.recover(
                ECDSA.toEthSignedMessageHash(signedMessage),
                sig.v,
                sig.r,
                sig.s
            );
            // console.log("signer: ");
            // console.log(signer);
            require(
                stakingContract.isActiveValidatorByNodeAddress(signer),
                "PubkeyRouter: signer is not active validator"
            );
        }
        return true;
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    /// register a pubkey and routing data for a given key hash
    function setRoutingData(
        uint256 tokenId,
        bytes memory pubkey,
        address stakingContractAddress,
        uint256 keyType,
        bytes32 derivedKeyId
    ) public {
        require(
            msg.sender == address(getPkpNftAddress()),
            "setRoutingData must be called by PKPNFT contract"
        );

        require(
            tokenId == uint256(keccak256(pubkey)),
            "tokenId does not match hashed pubkey"
        );
        require(
            !isRouted(tokenId),
            "PubkeyRouter: pubkey already has routing data"
        );

        s().pubkeys[tokenId].pubkey = pubkey;
        s().pubkeys[tokenId].keyType = keyType;
        s().pubkeys[tokenId].derivedKeyId = derivedKeyId;

        address pkpAddress = deriveEthAddressFromPubkey(pubkey);
        s().ethAddressToPkpId[pkpAddress] = tokenId;

        emit PubkeyRoutingDataSet(
            tokenId,
            pubkey,
            stakingContractAddress,
            keyType,
            derivedKeyId
        );
    }

    /// Set the pubkey and routing data for a given key hash
    // this is only used by an admin in case of emergency.  can prob be removed.
    function setRoutingDataAsAdmin(
        uint256 tokenId,
        bytes memory pubkey,
        address stakingContract,
        uint256 keyType,
        bytes32 derivedKeyId
    ) public onlyOwner {
        s().pubkeys[tokenId].pubkey = pubkey;
        s().pubkeys[tokenId].keyType = keyType;
        s().pubkeys[tokenId].derivedKeyId = derivedKeyId;

        address pkpAddress = deriveEthAddressFromPubkey(pubkey);
        s().ethAddressToPkpId[pkpAddress] = tokenId;

        emit PubkeyRoutingDataSet(
            tokenId,
            pubkey,
            stakingContract,
            keyType,
            derivedKeyId
        );
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        s().contractResolver = ContractResolver(newResolverAddress);
        emit ContractResolverAddressSet(newResolverAddress);
    }

    function voteForRootKeys(
        address stakingContractAddress,
        IPubkeyRouter.RootKey[] memory newRootKeys
    ) public {
        StakingViewsFacet stakingContract = StakingViewsFacet(
            stakingContractAddress
        );
        require(
            stakingContract.isActiveValidatorByNodeAddress(msg.sender),
            "PubkeyRouter: txn sender is not active validator"
        );

        require(
            s().rootKeys[stakingContractAddress].length == 0,
            "PubkeyRouter: root keys already set for this staking contract"
        );

        // record the votes
        for (uint i = 0; i < newRootKeys.length; i++) {
            IPubkeyRouter.RootKey memory rootKey = newRootKeys[i];
            require(
                s()
                .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                    .voted[msg.sender] == false,
                "PubkeyRouter: validator has already voted for this root key"
            );
            s()
            .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                .votes += 1;
            s()
            .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                .voted[msg.sender] = true;

            // if it has enough votes, register it
            if (
                s()
                .votesToRegisterRootKeys[stakingContractAddress][rootKey.pubkey]
                    .votes ==
                stakingContract.getValidatorsInCurrentEpochLength()
            ) {
                s().rootKeys[stakingContractAddress].push(rootKey);
                emit RootKeySet(stakingContractAddress, rootKey);
            }
        }
    }

    function getDerivedPubkey(
        address stakingContract,
        bytes32 derivedKeyId
    ) public view returns (bytes memory) {
        IPubkeyRouter.RootKey[] memory rootPubkeys = getRootKeys(
            stakingContract
        );

        bytes memory pubkey = _computeHDPubkey(derivedKeyId, rootPubkeys, 2);

        return pubkey;
    }

    function adminResetRootKeys(address stakingContract) public onlyOwner {
        require(s().env == ContractResolver.Env.Dev, "only for dev env");

        delete s().rootKeys[stakingContract];
    }

    function _computeHDPubkey(
        bytes32 derivedKeyId,
        IPubkeyRouter.RootKey[] memory rootHDKeys,
        uint256 keyType
    ) internal view returns (bytes memory) {
        address deriverAddr = s().contractResolver.getContract(
            s().contractResolver.HD_KEY_DERIVER_CONTRACT(),
            s().env
        );
        (bool success, bytes memory pubkey) = IKeyDeriver(deriverAddr)
            .computeHDPubKey(derivedKeyId, rootHDKeys, keyType);

        require(success, "PubkeyRouter: Failed public key calculation");
        return pubkey;
    }

    /* ========== EVENTS ========== */

    event PubkeyRoutingDataSet(
        uint256 indexed tokenId,
        bytes pubkey,
        address stakingContract,
        uint256 keyType,
        bytes32 derivedKeyId
    );
    event ContractResolverAddressSet(address newResolverAddress);
    event RootKeySet(address stakingContract, IPubkeyRouter.RootKey rootKey);
}
