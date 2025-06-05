// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
*
* Implementation of a diamond.
/******************************************************************************/

import { LibDiamond } from "../libraries/LibDiamond.sol";
import { IDiamondCut } from "../interfaces/IDiamondCut.sol";
import { IDiamondLoupe } from "../interfaces/IDiamondLoupe.sol";
import { IERC173 } from "../interfaces/IERC173.sol";
import { IERC165 } from "../interfaces/IERC165.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { ContractResolver } from "../lit-core/ContractResolver.sol";
import { LibStakingStorage } from "./Staking/LibStakingStorage.sol";

// When no function exists for function called
error FunctionNotFound(bytes4 _functionSelector);

// This is used in diamond constructor
// more arguments are added to this struct
// this avoids stack too deep errors
struct StakingArgs {
    address owner;
    address init;
    bytes initCalldata;
    address contractResolver;
    ContractResolver.Env env;
}

contract Staking {
    constructor(
        IDiamondCut.FacetCut[] memory _diamondCut,
        StakingArgs memory _args
    ) payable {
        LibDiamond.setContractOwner(_args.owner);
        LibDiamond.diamondCut(_diamondCut, _args.init, _args.initCalldata);

        // Code can be added here to perform actions and set state variables.
        LibStakingStorage.getStorage().contractResolver = ContractResolver(
            _args.contractResolver
        );
        LibStakingStorage.getStorage().env = _args.env;

        // 0.05 tokens per token staked meaning a 5% per epoch inflation rate
        uint256[] memory keyTypesTemp = new uint256[](2);
        keyTypesTemp[0] = 1;
        keyTypesTemp[1] = 2;

        LibStakingStorage.getStorage().configs[0] = LibStakingStorage.Config({
            tokenRewardPerTokenPerEpoch: (10 ** 18) / 20, // 18 decimal places in token
            DEPRECATED_complaintTolerance: 10,
            DEPRECATED_complaintIntervalSecs: 900,
            keyTypes: keyTypesTemp,
            minimumValidatorCount: 3,
            maxConcurrentRequests: 1000,
            maxTripleCount: 25,
            minTripleCount: 10,
            peerCheckingIntervalSecs: 10000,
            maxTripleConcurrency: 2,
            rpcHealthcheckEnabled: true,
            litActionConfig: LibStakingStorage.LitActionConfig({
                timeoutMs: 30000,
                memoryLimitMb: 256,
                maxCodeLength: 16 * 1024 * 1024,
                maxResponseLength: 100 * 1024,
                maxFetchCount: 50,
                maxSignCount: 10,
                maxContractCallCount: 30,
                maxBroadcastAndCollectCount: 30,
                maxCallDepth: 5,
                maxRetries: 3
            }),
            // NOTE: heliosEnabled currently unused in node code,
            // see:  https://github.com/LIT-Protocol/lit-assets/pull/1981
            heliosEnabled: true
        });
        uint256 epochLengthSeconds = 1;
        LibStakingStorage.getStorage().epochs[0] = LibStakingStorage.Epoch({
            epochLength: epochLengthSeconds,
            number: 1,
            endTime: block.timestamp + epochLengthSeconds,
            retries: 0,
            timeout: 60
        });
        LibStakingStorage.getStorage().state = LibStakingStorage.States.Paused;

        // set sane defaults for min and max version
        LibStakingStorage.getStorage().versionRequirements[
            0
        ] = LibStakingStorage.Version({ major: 0, minor: 0, patch: 0 });
        LibStakingStorage.getStorage().versionRequirements[
            1
        ] = LibStakingStorage.Version({ major: 10000, minor: 0, patch: 0 });

        // Set up complaint configs
        // Unresponsive reasons - 1
        LibStakingStorage.getStorage().complaintReasonToConfig[
            1
        ] = LibStakingStorage.ComplaintConfig({
            tolerance: 6,
            intervalSecs: 90,
            kickPenaltyPercent: 5
        });
        // Non-Participation reasons - 2
        LibStakingStorage.getStorage().complaintReasonToConfig[
            2
        ] = LibStakingStorage.ComplaintConfig({
            tolerance: 10,
            intervalSecs: 90,
            kickPenaltyPercent: 5
        });
        // Incorrect info - 3
        LibStakingStorage.getStorage().complaintReasonToConfig[
            3
        ] = LibStakingStorage.ComplaintConfig({
            tolerance: 10,
            intervalSecs: 90,
            kickPenaltyPercent: 5
        });
        // Unknown error - 4 (not used right now, but just in case)
        LibStakingStorage.getStorage().complaintReasonToConfig[
            4
        ] = LibStakingStorage.ComplaintConfig({
            tolerance: 10,
            intervalSecs: 90,
            kickPenaltyPercent: 5
        });

        LibStakingStorage.getStorage().devopsAdmin = _args.owner;
    }

    // Find facet for function that is called and execute the
    // function if a facet is found and return any value.
    fallback() external {
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
