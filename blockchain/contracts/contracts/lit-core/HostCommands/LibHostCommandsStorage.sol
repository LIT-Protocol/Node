//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";

library LibHostCommandsStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    bytes32 constant HOST_COMMANDS_POSITION = keccak256("hostcommands.storage");

    struct HostCommandsStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        address authorizedCommandSender;
    }

    // Return the storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (HostCommandsStorage storage storageStruct)
    {
        bytes32 position = HOST_COMMANDS_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
