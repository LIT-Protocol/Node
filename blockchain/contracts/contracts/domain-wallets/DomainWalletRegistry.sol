//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { ContractResolver } from "../lit-core/ContractResolver.sol";
import { LibDiamond } from "../libraries/LibDiamond.sol";
import { IDiamondCut } from "../interfaces/IDiamondCut.sol";
import { IDiamondLoupe } from "../interfaces/IDiamondLoupe.sol";

import { LibDomainWalletRegistryStorage } from "./DomainWalletRegistry/LibDomainWalletRegistryStorage.sol";

// When no function exists for function called
error FunctionNotFound(bytes4 _functionSelector);

struct ConstructorArgs {
    address owner;
    address init;
    bytes initCalldata;
    address contractResolver;
    ContractResolver.Env env;
}

contract DomainWalletRegistry {
    constructor(
        IDiamondCut.FacetCut[] memory _diamondCut,
        ConstructorArgs memory _args
    ) payable {
        LibDiamond.setContractOwner(_args.owner);
        LibDiamond.diamondCut(_diamondCut, _args.init, _args.initCalldata);

        LibDomainWalletRegistryStorage.getStorage().domainCharLimit = 32;
        // Code can be added here to perform actions and set state variables.
        LibDomainWalletRegistryStorage
            .getStorage()
            .contractResolver = ContractResolver(_args.contractResolver);
        LibDomainWalletRegistryStorage.getStorage().env = _args.env;
    }

    // Find facet for function that is called and execute the
    // function if a facet is found and return any value.
    fallback() external payable {
        LibDiamond.DiamondStorage storage ds;
        bytes32 position = LibDiamond.DIAMOND_STORAGE_POSITION;
        // get diamond storage
        assembly {
            ds.slot := position
        }
        // get facet from function selector
        address facet = ds
            .facetAddressAndSelectorPosition[msg.sig]
            .facetAddress;
        if (facet == address(0)) {
            revert FunctionNotFound(msg.sig);
        }
        // Execute external function from facet using delegatecall and return any value.
        assembly {
            // copy function selector and any arguments
            calldatacopy(0, 0, calldatasize())
            // execute function call using the facet
            let result := delegatecall(gas(), facet, 0, calldatasize(), 0, 0)
            // get any return value
            returndatacopy(0, 0, returndatasize())
            // return any return value or error back to the caller
            switch result
            case 0 {
                revert(0, returndatasize())
            }
            default {
                return(0, returndatasize())
            }
        }
    }
}
