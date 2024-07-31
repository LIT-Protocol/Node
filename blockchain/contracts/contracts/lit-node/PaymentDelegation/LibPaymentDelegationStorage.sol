//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

library LibPaymentDelegationStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    bytes32 constant PAYMENT_DELEGATION_POSITION =
        keccak256("payment_delegation.storage");

    // this lets the app put restrictions on how much they want to pay in a given period, per user.
    // this is set globally for each payer, but is applied on a per user basis.
    // so if you had a restriction of 10 requests per 10 minutes, that means 1 million users can make 10 requests each in the 10 minute window for 10 million total requests in 10 minutes.
    struct Restriction {
        uint requestsPerPeriod;
        uint periodSeconds;
    }

    struct PaymentDelegationStorage {
        mapping(address => EnumerableSet.AddressSet) payers; // maps user wallet to payer wallet
        mapping(address => Restriction) restrictions; // maps payer wallet to restrictions.  these are optional.
        mapping(address => EnumerableSet.AddressSet) users; // maps payer wallet to user wallets
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (PaymentDelegationStorage storage storageStruct)
    {
        bytes32 position = PAYMENT_DELEGATION_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
