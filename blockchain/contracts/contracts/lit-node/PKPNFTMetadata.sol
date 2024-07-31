//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { Base64 } from "@openzeppelin/contracts/utils/Base64.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

import "hardhat/console.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import { ContractResolver } from "../lit-core/ContractResolver.sol";

/// @title Programmable Keypair NFT Metadata
///
/// @dev This is the contract for the PKP NFTs
///
/// Simply put, whomever owns a PKP NFT can ask that PKP to sign a message.
/// The owner can also grant signing permissions to other eth addresses
/// or lit actions
contract PKPNFTMetadata {
    using Strings for uint256;

    ContractResolver public contractResolver;
    ContractResolver.Env public env;

    /* ========== STATE VARIABLES ========== */
    mapping(uint256 => string) pkpUrls;
    mapping(uint256 => string) pkpProfileImg;

    /* ========== CONSTRUCTOR ========== */
    constructor(address _resolver, ContractResolver.Env _env) {
        contractResolver = ContractResolver(_resolver);
        env = _env;
    }

    /* ========== VIEWS ========== */

    function bytesToHex(
        bytes memory buffer
    ) public pure returns (string memory) {
        // Fixed buffer size for hexadecimal convertion
        bytes memory converted = new bytes(buffer.length * 2);

        bytes memory _base = "0123456789abcdef";

        for (uint256 i = 0; i < buffer.length; i++) {
            converted[i * 2] = _base[uint8(buffer[i]) / _base.length];
            converted[i * 2 + 1] = _base[uint8(buffer[i]) % _base.length];
        }

        return string(abi.encodePacked("0x", converted));
    }

    function setUrlForPKP(uint256 tokenId, string memory url) public {
        require(
            msg.sender ==
                contractResolver.getContract(
                    contractResolver.PKP_HELPER_CONTRACT(),
                    env
                ),
            "PKPHelper: only the Domain Wallet registry is allowed to mint domain wallets"
        );

        pkpUrls[tokenId] = url;
    }

    function setProfileForPKP(uint256 tokenId, string memory imgUrl) public {
        require(
            msg.sender ==
                contractResolver.getContract(
                    contractResolver.PKP_HELPER_CONTRACT(),
                    env
                ),
            "PKPHelper: only the Domain Wallet registry is allowed to mint domain wallets"
        );

        pkpProfileImg[tokenId] = imgUrl;
    }

    function removeUrlForPKP(uint256 tokenId) public {
        require(
            msg.sender ==
                contractResolver.getContract(
                    contractResolver.PKP_HELPER_CONTRACT(),
                    env
                ),
            "PKPHelper: only the Domain Wallet registry is allowed to mint domain wallets"
        );

        pkpUrls[tokenId] = "";
    }

    function removeProfileForPkp(uint256 tokenId) public {
        require(
            msg.sender ==
                contractResolver.getContract(
                    contractResolver.PKP_HELPER_CONTRACT(),
                    env
                ),
            "PKPHelper: only the Domain Wallet registry is allowed to mint domain wallets"
        );

        pkpProfileImg[tokenId] = "";
    }

    function tokenURI(
        uint256 tokenId,
        bytes memory pubKey,
        address ethAddress
    ) public view returns (string memory) {
        string memory json = resolveMetaData(tokenId, pubKey, ethAddress);
        return string(abi.encodePacked("data:application/json;base64,", json));
    }

    function resolveMetaData(
        uint256 tokenId,
        bytes memory pubKey,
        address ethAddress
    ) private view returns (string memory) {
        string
            memory svgData = "<svg xmlns='http://www.w3.org/2000/svg' width='1080' height='1080' fill='none' xmlns:v='https://vecta.io/nano'><path d='M363.076 392.227s-.977 18.524-36.874 78.947c-41.576 70.018-45.481 151.978-3.017 220.4 89.521 144.245 332.481 141.52 422.556.089 34.832-54.707 44.816-117.479 32.924-181.248 0 0-28.819-133.144-127.237-217.099 1.553 1.308 5.369 19.122 6.101 26.722 2.241 23.354.045 47.838-7.787 70.062-5.746 16.33-13.711 30.467-27.178 41.368 0-3.811-.954-10.635-.976-12.918-.644-46.508-18.659-89.582-48.011-125.743-25.647-31.552-60.812-53.089-97.84-68.932.931 3.191 2.662 16.419 2.906 19.033 1.908 21.958 2.263 52.713-.621 74.649s-7.832 33.878-14.554 54.441c-10.184 31.175-24.05 54.285-41.621 82.004-3.24 5.096-12.913 19.078-18.082 26.146 0 0-8.897-56.191-40.667-87.921h-.022z' fill='#000'/><path d='M562.5 27.28l410.279 236.874c13.923 8.039 22.5 22.895 22.5 38.971v473.75c0 16.076-8.577 30.932-22.5 38.971L562.5 1052.72c-13.923 8.04-31.077 8.04-45 0L107.221 815.846c-13.923-8.039-22.5-22.895-22.5-38.971v-473.75a45 45 0 0 1 22.5-38.971L517.5 27.28a45 45 0 0 1 45 0z' stroke='#000' stroke-width='24.75'/></svg>";

        string memory pubkeyStr = bytesToHex(pubKey);
        // console.log("pubkeyStr");
        // console.log(pubkeyStr);

        string memory ethAddressStr = Strings.toHexString(ethAddress);
        // console.log("ethAddressStr");
        // console.log(ethAddressStr);

        string memory tokenIdStr = Strings.toString(tokenId);

        string memory name = pkpUrls[tokenId];

        string memory profileImage = pkpProfileImg[tokenId];

        /// name is not registed
        if (bytes(name).length == 0 && bytes(profileImage).length != 0) {
            name = string.concat("Lit PKP #", tokenIdStr);
            /// profile image is not defined
        } else if (bytes(name).length != 0 && bytes(profileImage).length == 0) {
            profileImage = svgData;
            /// neither name or profile url are defined
        } else if (bytes(name).length == 0 && bytes(profileImage).length == 0) {
            name = string.concat("Lit PKP #", tokenIdStr);
            profileImage = svgData;
        }

        return
            Base64.encode(
                bytes(
                    string(
                        abi.encodePacked(
                            '{"name":"',
                            name,
                            '", "description": "This NFT entitles the holder to use a Lit Protocol PKP, and to grant access to other users and Lit Actions to use this PKP","image_data": "',
                            bytes(profileImage),
                            '","attributes": [{"trait_type": "Public Key", "value": "',
                            pubkeyStr,
                            '"}, {"trait_type": "ETH Wallet Address", "value": "',
                            ethAddressStr,
                            '"}, {"trait_type": "Token ID", "value": "',
                            tokenIdStr,
                            '"}]}'
                        )
                    )
                )
            );
    }
}
