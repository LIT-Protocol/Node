// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Library for ERC-2771 support
library LibERC2771 {
    struct ERC2771Storage {
        address trustedForwarder;
    }

    bytes32 constant ERC2771_STORAGE_POSITION = keccak256("erc2771.storage");

    function erc2771Storage()
        internal
        pure
        returns (ERC2771Storage storage es)
    {
        bytes32 position = ERC2771_STORAGE_POSITION;
        assembly {
            es.slot := position
        }
    }

    function _msgSender() internal view returns (address sender) {
        ERC2771Storage storage es = erc2771Storage();
        if (msg.sender == es.trustedForwarder && msg.data.length >= 20) {
            assembly {
                sender := shr(96, calldataload(sub(calldatasize(), 20)))
            }
        } else {
            sender = msg.sender;
        }
    }

    function _msgData() internal view returns (bytes memory) {
        ERC2771Storage storage es = erc2771Storage();
        if (msg.sender == es.trustedForwarder && msg.data.length >= 20) {
            return msg.data[0:msg.data.length - 20];
        } else {
            return msg.data;
        }
    }
}
