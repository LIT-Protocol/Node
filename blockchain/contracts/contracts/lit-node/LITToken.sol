//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { ERC20Burnable } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import { ERC20Capped } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Capped.sol";
import { ERC20Pausable } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import { ERC20Permit } from "@openzeppelin/contracts/token/ERC20/extensions/draft-ERC20Permit.sol";
import { ERC20Votes } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";

/// @title Lit Protocol Token
///
/// @dev This is the contract for the Lit Protocol governance token.
///
/// Initially, the contract deployer is given both the admin and minter role. This allows them to pre-mine tokens,
/// transfer admin to a timelock contract, and lastly, grant the staking pools the minter role. After this is done,
/// the deployer must revoke their admin role and minter role.
///
/// This contract was initially borrowed from Alchemix, and modified extensively, from here: https://github.com/alchemix-finance/alchemix-protocol/blob/master/contracts/AlchemixToken.sol
contract LITToken is
    AccessControl,
    ERC20("Lit Test Token", "testLIT"),
    ERC20Burnable,
    ERC20Capped,
    ERC20Pausable,
    ERC20Permit,
    ERC20Votes
{
    /// @dev The identifier of the role which maintains other roles.
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN");

    /// @dev The identifier of the role which allows accounts to mint tokens.
    bytes32 public constant MINTER_ROLE = keccak256("MINTER");

    /// @dev The identifier of the role which allows accounts to pause the token.
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    constructor(uint256 cap) ERC20Capped(cap) ERC20Permit("Lit Protocol") {
        _setupRole(ADMIN_ROLE, msg.sender);
        _setupRole(MINTER_ROLE, msg.sender);
        _setupRole(PAUSER_ROLE, msg.sender);
        _setRoleAdmin(MINTER_ROLE, ADMIN_ROLE);
        _setRoleAdmin(ADMIN_ROLE, ADMIN_ROLE);
        _setRoleAdmin(PAUSER_ROLE, ADMIN_ROLE);
    }

    /// @dev A modifier which checks that the caller has the minter role.
    modifier onlyMinter() {
        require(hasRole(MINTER_ROLE, msg.sender), "LITToken: only minter");
        _;
    }

    /// @dev Mints tokens to a recipient.
    ///
    /// This function reverts if the caller does not have the minter role.
    ///
    /// @param _recipient the account to mint tokens to.
    /// @param _amount    the amount of tokens to mint.
    function mint(address _recipient, uint256 _amount) external onlyMinter {
        _mint(_recipient, _amount);
    }

    /**
     * @dev Pauses all token transfers.
     *
     * See {ERC20Pausable} and {Pausable-_pause}.
     *
     * Requirements:
     *
     * - the caller must have the `PAUSER_ROLE`.
     */
    function pause() public virtual {
        require(
            hasRole(PAUSER_ROLE, _msgSender()),
            "ERC20PresetMinterPauser: must have pauser role to pause"
        );
        _pause();
    }

    /**
     * @dev Unpauses all token transfers.
     *
     * See {ERC20Pausable} and {Pausable-_unpause}.
     *
     * Requirements:
     *
     * - the caller must have the `PAUSER_ROLE`.
     */
    function unpause() public virtual {
        require(
            hasRole(PAUSER_ROLE, _msgSender()),
            "ERC20PresetMinterPauser: must have pauser role to unpause"
        );
        _unpause();
    }

    /* Overrides */

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual override(ERC20, ERC20Pausable) {
        super._beforeTokenTransfer(from, to, amount);
    }

    function _afterTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual override(ERC20, ERC20Votes) {
        super._afterTokenTransfer(from, to, amount);
    }

    function _burn(
        address account,
        uint256 amount
    ) internal virtual override(ERC20, ERC20Votes) {
        super._burn(account, amount);
    }

    function _mint(
        address account,
        uint256 amount
    ) internal virtual override(ERC20, ERC20Votes, ERC20Capped) {
        super._mint(account, amount);
    }
}
