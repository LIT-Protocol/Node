/* global ethers */
/* eslint prefer-const: "off" */

const hre = require("hardhat");
const { getSelectors, FacetCutAction } = require("./libraries/diamond");
const { verifyContractInBg } = require("./utils");

async function removeFacet(facetName, oldFacetAddress, diamondAddress) {
    const accounts = await ethers.getSigners();

    const networkName = hre.network.name;

    // get the old facet
    const facet = await ethers.getContractAt(facetName, oldFacetAddress);
    const facetCut = {
        facetAddress: ethers.ZeroAddress,
        action: FacetCutAction.Remove,
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
    removeFacet(
        "StakingViewsFacet", // facet name
        "0x191E7b0b20D050D8ED85B5451E0823A39B6D28FF", // facet address
        "0xBC7F8d7864002b6629Ab49781D5199C8dD1DDcE1" // diamond contract address
    )
        .then(() => process.exit(0))
        .catch((error) => {
            console.error(error);
            process.exit(1);
        });
}
