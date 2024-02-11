/* global ethers */
/* eslint prefer-const: "off" */

import hre from "hardhat";
import path from "path";
import fs from "fs";
const { getSelectors, FacetCutAction } = require("./libraries/diamond");
const { verifyContractInBg } = require("./utils");
const { ethers } = hre;

export async function deployDiamond(
    diamondName: string,
    contractResolver: string,
    env: number,
    additionalFacets = [],
    useErc165Loupe = false
) {
    const accounts = await ethers.getSigners();
    const contractOwner = accounts[0];

    const networkName = hre.network.name;

    // Deploy DiamondInit
    // DiamondInit provides a function that is called when the diamond is upgraded or deployed to initialize state variables
    // Read about how the diamondCut function works in the EIP2535 Diamonds standard
    const diamondInit = await ethers.deployContract("DiamondInit");
    await diamondInit.waitForDeployment();
    verifyContractInBg(networkName, await diamondInit.getAddress());

    // Deploy facets and set the `facetCuts` variable
    const FacetNames = [
        "DiamondCutFacet",
        useErc165Loupe ? "DiamondLoupeFacet" : "DiamondLoupeFacetNoERC165",
        "OwnershipFacet",
        ...additionalFacets,
    ];
    // The `facetCuts` variable is the FacetCut[] that contains the functions to add during diamond deployment
    const facetCuts = [];
    const facets = [];
    for (const FacetName of FacetNames) {
        const facet = await ethers.deployContract(FacetName);
        await facet.waitForDeployment();
        const facetAddress = await facet.getAddress();
        verifyContractInBg(networkName, facetAddress);
        facets.push({ facetName: FacetName, facetAddress });
        facetCuts.push({
            facetAddress: facetAddress,
            action: FacetCutAction.Add,
            functionSelectors: getSelectors(facet),
        });
    }

    // Creating a function call
    // This call gets executed during deployment and can also be executed in upgrades
    // It is executed with delegatecall on the DiamondInit address.
    let functionCall = diamondInit.interface.encodeFunctionData("init");

    // Setting arguments that will be used in the diamond constructor
    const diamondArgs = {
        owner: contractOwner.address,
        init: await diamondInit.getAddress(),
        initCalldata: functionCall,
        contractResolver,
        env,
    };

    // deploy Diamond
    const diamond = await ethers.deployContract(diamondName, [
        facetCuts,
        diamondArgs,
    ]);
    await diamond.waitForDeployment();
    verifyContractInBg(networkName, await diamond.getAddress(), [
        facetCuts,
        diamondArgs,
    ]);

    // returning the address of the diamond
    return {
        diamond,
        facets,
    };
}

export async function deployDiamondContract(
    chainName: string,
    contractName: string,
    args: any[] = [],
    facets: any[] = [],
    useErc165Loupe = false
) {
    console.log(`Deploying ${contractName} with args ${args}`);
    const deployed = await deployDiamond(
        contractName,
        // @ts-ignore
        ...args,
        facets,
        useErc165Loupe
    );
    const diamondContract = deployed.diamond;
    const deployedFacets = deployed.facets;
    console.log(
        `${contractName} deployed to ${await diamondContract.getAddress()}`
    );
    // verifyContractInBg(chainName, await diamondContract.getAddress(), args);  // this is done inside deployDiamond
    const abiPath = path.resolve(
        __dirname,
        `../artifacts/hardhat-diamond-abi/${contractName}Diamond.sol/${contractName}Diamond.json`
    );
    // @ts-ignore
    const abi = JSON.parse(fs.readFileSync(abiPath)).abi;
    const [deployer] = await ethers.getSigners();
    const diamond = new ethers.Contract(
        await diamondContract.getAddress(),
        abi,
        deployer
    );
    return { diamond, deployedFacets };
}

if (require.main === module) {
    async function run() {
        await deployDiamondContract(
            hre.network.name,
            "BackupRecovery",
            ["0xB0cb99e69c01Bd481aeCc6DD0155d4147e96C746", 0],
            ["BackupRecoveryFacet"]
        );
    }

    run();
}
