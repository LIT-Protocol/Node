/* global ethers */
/* eslint prefer-const: "off" */

const hre = require("hardhat");
const { getSelectors, FacetCutAction } = require("./libraries/diamond");
const { verifyContractInBg } = require("./utils");

async function addFacet(facetName, diamondAddress) {
    const accounts = await ethers.getSigners();

    const networkName = hre.network.name;

    // deploy the new facet
    const facet = await ethers.deployContract(facetName);
    await facet.waitForDeployment();
    const facetAddress = await facet.getAddress();
    verifyContractInBg(networkName, facetAddress);

    const facetCut = {
        facetAddress,
        action: FacetCutAction.Add,
        functionSelectors: getSelectors(facet),
    };

    // load the diamond
    const diamondCutFacet = await ethers.getContractAt(
        "DiamondCutFacet",
        diamondAddress
    );

    tx = await diamondCutFacet.diamondCut([facetCut], ethers.ZeroAddress, "0x");
    console.log("tx sent", tx);
    await tx.wait();
    console.log("tx confirmed", tx);

    // // if your run into errors you can use the below to log the txn and use hardhat-tracer to find the real revert reason
    // try {
    //     tx = await diamondCutFacet.diamondCut.populateTransaction([facetCut], ethers.ZeroAddress, "0x");
    //     console.log('tx', tx);
    // } catch (e) {
    //     console.log(e)
    // }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
if (require.main === module) {
    addFacet(
        "StakingViewsFacet", // facet name
        "0xBC7F8d7864002b6629Ab49781D5199C8dD1DDcE1" // contract address
    )
        .then(() => process.exit(0))
        .catch((error) => {
            console.error(error);
            process.exit(1);
        });
}
