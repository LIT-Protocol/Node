//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ERC165Upgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/utils/introspection/ERC165Upgradeable.sol";
import { IERC165Upgradeable } from "@gnus.ai/contracts-upgradeable-diamond/contracts/utils/introspection/IERC165Upgradeable.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import { LibHostCommandsStorage } from "./LibHostCommandsStorage.sol";
import { ContractResolver } from "../ContractResolver.sol";

import "hardhat/console.sol";

/// @title Host Commands
///
/// @dev This is the contract used for sending admin commands to the host boxes.  Things like reboots or network upgrades.  Gated by a multisig.
contract HostCommandsFacet {
    error CallerNotOwner();
    error CallerNotAuthorizedCommandSender();
    error InvalidExpirationTime();

    /* ========== Modifiers ========== */

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    modifier onlyAuthorizedCommandSender() {
        if (msg.sender != s().authorizedCommandSender)
            revert CallerNotAuthorizedCommandSender();
        _;
    }

    /* ========== VIEWS ========== */

    function s()
        internal
        pure
        returns (LibHostCommandsStorage.HostCommandsStorage storage)
    {
        return LibHostCommandsStorage.getStorage();
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function restart(
        address stakeAddress,
        uint256 expirationTime,
        bool force
    ) public onlyAuthorizedCommandSender {
        validateExpirationTime(expirationTime);
        emit Restart(stakeAddress, expirationTime, force);
    }

    function upgrade(
        address stakeAddress,
        uint256 expirationTime,
        string memory releaseBranchName,
        string memory releaseId,
        bool force
    ) public onlyAuthorizedCommandSender {
        validateExpirationTime(expirationTime);
        emit Upgrade(
            stakeAddress,
            expirationTime,
            releaseBranchName,
            releaseId,
            force
        );
    }

    function setAuthorizedCommandSender(
        address _newAuthorizedCommandSender
    ) public onlyOwner {
        s().authorizedCommandSender = _newAuthorizedCommandSender;
        emit AuthorizedCommandSenderUpdated(_newAuthorizedCommandSender);
    }

    /* ========== INTERNAL FUNCTIONS ========== */

    function validateExpirationTime(uint256 expirationTime) internal view {
        if (
            expirationTime < block.timestamp ||
            expirationTime > block.timestamp + 1 days
        ) revert InvalidExpirationTime();
    }

    /* ========== EVENTS ========== */

    event Restart(
        address indexed stakeAddress,
        uint256 expirationTime,
        bool force
    );
    event Upgrade(
        address indexed stakeAddress,
        uint256 expirationTime,
        string releaseBranchName,
        string releaseId,
        bool force
    );
    event AuthorizedCommandSenderUpdated(address newAuthorizedCommandSender);
}
