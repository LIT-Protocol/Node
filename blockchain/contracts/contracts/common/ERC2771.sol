// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { LibDiamond } from "../libraries/LibDiamond.sol";
import { LibERC2771 } from "../libraries/LibERC2771.sol";

contract ERC2771 {
    error CallerNotOwner();

    modifier onlyOwner() {
        if (LibERC2771._msgSender() != LibDiamond.contractOwner())
            revert CallerNotOwner();
        _;
    }

    function erc2771Storage()
        internal
        pure
        returns (LibERC2771.ERC2771Storage storage es)
    {
        bytes32 position = LibERC2771.ERC2771_STORAGE_POSITION;
        assembly {
            es.slot := position
        }
    }

    function setTrustedForwarder(address forwarder) public onlyOwner {
        erc2771Storage().trustedForwarder = forwarder;
        emit TrustedForwarderSet(forwarder);
    }

    function getTrustedForwarder() public view returns (address) {
        return erc2771Storage().trustedForwarder;
    }

    event TrustedForwarderSet(address newTrustedForwarder);
}
