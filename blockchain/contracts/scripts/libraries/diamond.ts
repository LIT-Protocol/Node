/* global ethers */

import { Contract, Fragment } from "ethers";

const FacetCutAction = { Add: 0, Replace: 1, Remove: 2 };

// get function selectors from ABI
function getSelectors(contract: Contract) {
    // Extract all the function fragments
    const functionFragments = contract.interface.fragments.filter(
        Fragment.isFunction
    );
    const selectors = functionFragments.map((f) => f.selector);
    return selectors;
}

exports.getSelectors = getSelectors;
exports.FacetCutAction = FacetCutAction;
