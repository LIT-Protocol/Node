/* global ethers */
/* eslint prefer-const: "off" */

const hre = require("hardhat");
const { getSelectors, FacetCutAction } = require("./libraries/diamond");
const { verifyContractInBg } = require("./utils");
const { replaceFacet } = require("./replaceDiamondFacet");
const fs = require("fs");

const facetToDiamondMap = {
    PKPNFTFacet: "pkpNftContractAddress",
    PKPPermissionsFacet: "pkpPermissionsContractAddress",
    PubkeyRouterFacet: "pubkeyRouterContractAddress",
    RateLimitNFTFacet: "rateLimitNftContractAddress",
    RateLimitNFTViewsFacet: "rateLimitNftContractAddress",
    StakingFacet: "stakingContractAddress",
    StakingVersionFacet: "stakingContractAddress",
    StakingViewsFacet: "stakingContractAddress",
    StakingBalancesFacet: "stakingBalancesContractAddress",
};

async function go() {
    // load the diamond addresses
    const addresses = JSON.parse(
        fs.readFileSync(
            "../../../networks/manzano/deployed-lit-node-contracts-temp.json"
        )
    );

    // loop over the facets and replace them all
    for (const key in facetToDiamondMap) {
        console.log("Replacing facet: " + key);
        let diamondAddress = addresses[facetToDiamondMap[key]];
        if (diamondAddress === undefined) {
            console.log(`Diamond address for ${key} not found`);
            continue;
        }
        const txn = await replaceFacet(key, diamondAddress);
    }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
if (require.main === module) {
    go();
}
