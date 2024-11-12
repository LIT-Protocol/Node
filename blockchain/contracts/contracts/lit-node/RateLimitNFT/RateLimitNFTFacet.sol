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
import { ReentrancyGuardUpgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/security/ReentrancyGuardUpgradeable.sol";
import { ERC165Upgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/utils/introspection/ERC165Upgradeable.sol";
import { IERC165Upgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/utils/introspection/IERC165Upgradeable.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { LibRateLimitNFTStorage } from "./LibRateLimitNFTStorage.sol";
import { RateLimitNFTViewsFacet } from "./RateLimitNFTViewsFacet.sol";

import "hardhat/console.sol";

/// @title Rate Limit NFT
///
/// @dev This is the contract for the Rate Limit NFTs
/// So the general idea here is that you can mint one of these NFTs to pay for service on Lit
/// And how it works, is that you can buy X requestsPerKilosecond over a period of time
/// 1 requestsPerKilosecond = 0.001 requests per second and
/// 1000 requestsPerKilosecond = 1 request per second
contract RateLimitNFTFacet is
    ERC721Upgradeable,
    ERC721BurnableUpgradeable,
    ERC721EnumerableUpgradeable,
    ReentrancyGuardUpgradeable
{
    using Strings for uint256;

    error CallerNotOwner();

    function initialize() public initializer {
        __ERC721_init("Rate Limit Increases on Lit Protocol", "RLI");
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
        returns (LibRateLimitNFTStorage.RateLimitNFTStorage storage)
    {
        return LibRateLimitNFTStorage.getStorage();
    }

    function views() internal view returns (RateLimitNFTViewsFacet) {
        return RateLimitNFTViewsFacet(address(this));
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
        return views().tokenSVG(tokenId);
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    /// mint a token with a certain number of requests per millisecond and a certain expiration time.  Requests per second is calculated from the msg.value amount.  You can find out the cost for a certain requests per second value by using the calculateCost() function.
    function mint(uint256 expiresAt) public payable returns (uint256) {
        s().tokenIdCounter++;
        uint256 tokenId = s().tokenIdCounter;

        uint256 requestsPerKilosecond = views().calculateRequestsPerKilosecond(
            msg.value,
            expiresAt
        );

        // sanity check
        uint256 cost = views().calculateCost(requestsPerKilosecond, expiresAt);

        require(
            msg.value > 0 && msg.value >= cost,
            "You must send the cost of this rate limit increase.  To check the cost, use the calculateCost function."
        );
        require(cost > 0, "The cost must be greater than 0");

        _mintWithoutValueCheck(tokenId, requestsPerKilosecond, expiresAt);

        return tokenId;
    }

    function freeMint(
        uint256 expiresAt,
        uint256 requestsPerKilosecond,
        bytes32 msgHash,
        uint8 v,
        bytes32 r,
        bytes32 sVal
    ) public returns (uint256) {
        s().tokenIdCounter++;
        uint256 tokenId = s().tokenIdCounter;

        // this will panic if the sig is bad
        views().freeMintSigTest(
            expiresAt,
            requestsPerKilosecond,
            msgHash,
            v,
            r,
            sVal
        );
        s().redeemedFreeMints[msgHash] = true;

        _mintWithoutValueCheck(tokenId, requestsPerKilosecond, expiresAt);

        return tokenId;
    }

    function _mintWithoutValueCheck(
        uint256 tokenId,
        uint256 requestsPerKilosecond,
        uint256 expiresAt
    ) internal {
        require(
            expiresAt < block.timestamp + s().maxExpirationSeconds,
            "You cannot purchase an expiration time that is more than the maxExpirationSeconds in the future"
        );
        require(expiresAt > block.timestamp, "Expiration time is in the past");

        // Check that it's midnight. A day in Unix timestamp is 86400 seconds, so if a timestamp is at
        // midnight, it should be divisible by 86400 with no remainder.
        require(
            expiresAt % 86400 == 0,
            "Expiration time must be set to midnight on any given day"
        );
        require(
            views().checkBelowMaxRequestsPerKilosecond(requestsPerKilosecond),
            "Can't allocate capacity beyond the global max requests per kilosecond"
        );
        _safeMint(msg.sender, tokenId);
        s().capacity[tokenId] = LibRateLimitNFTStorage.RateLimit(
            requestsPerKilosecond,
            expiresAt
        );
        // insert into total sold mapping
        s().totalSoldRequestsPerKilosecondByExpirationTime[
            expiresAt
        ] += requestsPerKilosecond;
    }

    function setAdditionalRequestsPerKilosecondCost(
        uint256 newAdditionalRequestsPerKilosecondCost
    ) public onlyOwner {
        s()
            .additionalRequestsPerKilosecondCost = newAdditionalRequestsPerKilosecondCost;
        emit AdditionalRequestsPerKilosecondCostSet(
            newAdditionalRequestsPerKilosecondCost
        );
    }

    function setFreeMintSigner(address newFreeMintSigner) public onlyOwner {
        s().freeMintSigner = newFreeMintSigner;
        emit FreeMintSignerSet(newFreeMintSigner);
    }

    function withdraw() public onlyOwner nonReentrant {
        uint256 withdrawAmount = address(this).balance;
        (bool sent, ) = payable(msg.sender).call{ value: withdrawAmount }("");
        require(sent);
        emit Withdrew(withdrawAmount);
    }

    function setRateLimitWindowSeconds(
        uint256 newRateLimitWindowSeconds
    ) public onlyOwner {
        s().defaultRateLimitWindowSeconds = newRateLimitWindowSeconds;
        emit RateLimitWindowSecondsSet(newRateLimitWindowSeconds);
    }

    function setRLIHolderRateLimitWindowSeconds(
        uint256 newRLIHolderRateLimitWindowSeconds
    ) public onlyOwner {
        s()
            .RLIHolderRateLimitWindowSeconds = newRLIHolderRateLimitWindowSeconds;
        emit RLIHolderRateLimitWindowSecondsSet(
            newRLIHolderRateLimitWindowSeconds
        );
    }

    function setFreeRequestsPerRateLimitWindow(
        uint256 newFreeRequestsPerRateLimitWindow
    ) public onlyOwner {
        s().freeRequestsPerRateLimitWindow = newFreeRequestsPerRateLimitWindow;
        emit FreeRequestsPerRateLimitWindowSet(
            newFreeRequestsPerRateLimitWindow
        );
    }

    function setMaxRequestsPerKilosecond(
        uint256 newMaxRequestsPerKilosecond
    ) public onlyOwner {
        s().maxRequestsPerKilosecond = newMaxRequestsPerKilosecond;
    }

    function setMaxExpirationSeconds(
        uint256 newMaxExpirationSeconds
    ) public onlyOwner {
        s().maxExpirationSeconds = newMaxExpirationSeconds;
    }

    function pruneExpired(address owner) public {
        uint256 balance = balanceOf(owner);
        uint256[] memory tokenIds = new uint256[](balance);
        for (uint256 i = 0; i < balance; i++) {
            uint256 tokenId = tokenOfOwnerByIndex(owner, i);
            if (s().capacity[tokenId].expiresAt < block.timestamp) {
                tokenIds[i] = tokenId;
            }
        }

        // burn the expired ones
        for (uint256 i = 0; i < balance; i++) {
            if (tokenIds[i] != 0) {
                _burn(tokenIds[i]);
            }
        }
    }

    /* ========== EVENTS ========== */

    event AdditionalRequestsPerKilosecondCostSet(
        uint256 newAdditionalRequestsPerKilosecondCost
    );
    event FreeMintSignerSet(address indexed newFreeMintSigner);
    event Withdrew(uint256 amount);
    event RateLimitWindowSecondsSet(uint256 newRateLimitWindowSeconds);
    event RLIHolderRateLimitWindowSecondsSet(
        uint256 newRLIHolderRateLimitWindowSeconds
    );
    event FreeRequestsPerRateLimitWindowSet(
        uint256 newFreeRequestsPerRateLimitWindow
    );
}
