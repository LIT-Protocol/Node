//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { ReentrancyGuard } from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

import "hardhat/console.sol";

contract Allowlist is Ownable, ReentrancyGuard {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== STATE VARIABLES ========== */

    mapping(bytes32 => bool) public allowedItems;
    EnumerableSet.AddressSet admins;
    bool public allowAll;

    /* ========== CONSTRUCTOR ========== */
    constructor() {
        admins.add(msg.sender);
    }

    /* ========== VIEWS ========== */

    function isAllowed(bytes32 key) external view returns (bool) {
        if (allowAll) {
            return true;
        }
        return allowedItems[key];
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    function setAllowed(bytes32 key) external {
        require(admins.contains(msg.sender), "Not an admin");
        allowedItems[key] = true;
        emit ItemAllowed(key);
    }

    function setNotAllowed(bytes32 key) external {
        require(admins.contains(msg.sender), "Not an admin");
        allowedItems[key] = false;
        emit ItemNotAllowed(key);
    }

    function addAdmin(address newAdmin) public onlyOwner {
        admins.add(newAdmin);
        emit AdminAdded(newAdmin);
    }

    function removeAdmin(address newAdmin) public onlyOwner {
        admins.remove(newAdmin);
        emit AdminRemoved(newAdmin);
    }

    function setAllowAll(bool _allowAll) public onlyOwner {
        allowAll = _allowAll;
    }

    /* ========== EVENTS ========== */

    event ItemAllowed(bytes32 indexed key);
    event ItemNotAllowed(bytes32 indexed key);
    event AdminAdded(address indexed newAdmin);
    event AdminRemoved(address indexed newAdmin);
}
