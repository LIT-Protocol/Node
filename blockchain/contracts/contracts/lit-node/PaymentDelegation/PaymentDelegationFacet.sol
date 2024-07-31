//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";

import { LibPaymentDelegationStorage } from "./LibPaymentDelegationStorage.sol";

// import "hardhat/console.sol";

contract PaymentDelegationFacet {
    using EnumerableSet for EnumerableSet.AddressSet;

    /* ========== VIEWS ========== */
    function s()
        internal
        pure
        returns (LibPaymentDelegationStorage.PaymentDelegationStorage storage)
    {
        return LibPaymentDelegationStorage.getStorage();
    }

    function getPayersAndRestrictions(
        address[] memory users
    )
        public
        view
        returns (
            address[][] memory,
            LibPaymentDelegationStorage.Restriction[][] memory
        )
    {
        address[][] memory payers = new address[][](users.length);
        LibPaymentDelegationStorage.Restriction[][]
            memory restrictions = new LibPaymentDelegationStorage.Restriction[][](
                users.length
            );
        for (uint i = 0; i < users.length; i++) {
            payers[i] = s().payers[users[i]].values();
            LibPaymentDelegationStorage.Restriction[]
                memory tempRestrictionsArray = new LibPaymentDelegationStorage.Restriction[](
                    payers[i].length
                );
            for (uint j = 0; j < payers[i].length; j++) {
                tempRestrictionsArray[j] = s().restrictions[payers[i][j]];
            }
            restrictions[i] = tempRestrictionsArray;
        }
        return (payers, restrictions);
    }

    function getUsers(address payer) public view returns (address[] memory) {
        return s().users[payer].values();
    }

    function getRestriction(
        address payer
    ) public view returns (LibPaymentDelegationStorage.Restriction memory) {
        return s().restrictions[payer];
    }

    function getPayers(address user) public view returns (address[] memory) {
        return s().payers[user].values();
    }

    /* ========== MUTATIVE FUNCTIONS ========== */
    function delegatePayments(address user) public {
        s().payers[user].add(msg.sender); // this guarantees the auth - that the payer made this txn, because msg.sender is authed.
        s().users[msg.sender].add(user);
    }

    function undelegatePayments(address user) public {
        require(
            s().payers[user].contains(msg.sender),
            "not authorized to undelegate payments"
        );
        s().payers[user].remove(msg.sender);
        s().users[msg.sender].remove(user);
    }

    function delegatePaymentsBatch(address[] memory users) public {
        for (uint i = 0; i < users.length; i++) {
            s().payers[users[i]].add(msg.sender);
            s().users[msg.sender].add(users[i]);
        }
    }

    function undelegatePaymentsBatch(address[] memory users) public {
        for (uint i = 0; i < users.length; i++) {
            require(
                s().payers[users[i]].contains(msg.sender),
                "not authorized to undelegate payments"
            );
            s().payers[users[i]].remove(msg.sender);
            s().users[msg.sender].remove(users[i]);
        }
    }

    function setRestriction(
        LibPaymentDelegationStorage.Restriction memory r
    ) public {
        s().restrictions[msg.sender] = r;
    }

    /* ========== EVENTS ========== */

    event RestrictionSet(
        address indexed payer,
        LibPaymentDelegationStorage.Restriction restriction
    );
}
