/* global ethers */
/* eslint prefer-const: "off" */

const hre = require("hardhat");
const { getSelectors, FacetCutAction } = require("./libraries/diamond");
const { verifyContractInBg } = require("./utils");

async function replaceFacet(newFacet, diamondAddress) {
    const accounts = await ethers.getSigners();

    const networkName = hre.network.name;

    // deploy the new facet
    const facet = await ethers.deployContract(newFacet);
    await facet.waitForDeployment();
    verifyContractInBg(networkName, await facet.getAddress());
    const facetCut = {
        facetAddress: await facet.getAddress(),
        action: FacetCutAction.Replace,
        functionSelectors: getSelectors(facet),
    };

    // load the diamond
    const diamondCutFacet = await ethers.getContractAt(
        "DiamondCutFacet",
        diamondAddress
    );

    tx = await diamondCutFacet.diamondCut([facetCut], ethers.ZeroAddress, "0x");
    receipt = await tx.wait();
    console.log("Success");
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
if (require.main === module) {
    replaceFacet(
        "PubkeyRouterFacet",
        "0xBC7F8d7864002b6629Ab49781D5199C8dD1DDcE1"
    )
        .then(() => process.exit(0))
        .catch((error) => {
            console.error(error);
            process.exit(1);
        });
}

exports.replaceFacet = replaceFacet;
