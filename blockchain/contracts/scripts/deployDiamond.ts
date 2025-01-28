/* global ethers */
/* eslint prefer-const: "off" */

import hre from 'hardhat';
import path from 'path';
import fs from 'fs';
import { FacetCutAction } from './diamondContracts/lib/types';
import { hardhatDeployAndVerifySingleContract } from './utils';
import { Signer } from 'ethers';
import { getSelectors } from './diamondContracts/lib/utils';

const { verifyContractInBg } = require('./utils');
const { ethers } = hre;

export async function deployDiamond(
  diamondName: string,
  contractResolver: string | null,
  env: number | null,
  {
    additionalFacets = [],
    useErc165Loupe = false,
    signer,
    waitForDeployment = true,
    verifyContracts = true,
  }: {
    additionalFacets?: string[];
    useErc165Loupe?: boolean;
    signer?: Signer;
    waitForDeployment?: boolean;
    verifyContracts?: boolean;
  } = {}
) {
  const accounts = await ethers.getSigners();
  const contractOwner = accounts[0];

  const networkName = hre.network.name;

  // Deploy DiamondInit
  // DiamondInit provides a function that is called when the diamond is upgraded or deployed to initialize state variables
  // Read about how the diamondCut function works in the EIP2535 Diamonds standard
  const diamondInit = await hardhatDeployAndVerifySingleContract(
    ethers,
    hre.network.name,
    'DiamondInit',
    {
      signer,
      verifyContracts,
      waitForDeployment,
    }
  );

  // Deploy facets and set the `facetCuts` variable
  const FacetNames = [
    'DiamondCutFacet',
    useErc165Loupe ? 'DiamondLoupeFacet' : 'DiamondLoupeFacetNoERC165',
    'OwnershipFacet',
    ...additionalFacets,
  ];
  // The `facetCuts` variable is the FacetCut[] that contains the functions to add during diamond deployment
  const facetCuts = [];
  const facets = [];
  for (const FacetName of FacetNames) {
    console.debug(`Deploying ${FacetName}`);
    const facet = await hardhatDeployAndVerifySingleContract(
      ethers,
      hre.network.name,
      FacetName,
      {
        signer,
        verifyContracts,
        waitForDeployment,
      }
    );
    const facetAddress = await facet.getAddress();

    facets.push({ facetName: FacetName, facetAddress });
    facetCuts.push({
      facetAddress: facetAddress,
      action: FacetCutAction.Add,
      functionSelectors: getSelectors(facet).map((s) => s.selector),
    });
  }

  // Creating a function call
  // This call gets executed during deployment and can also be executed in upgrades
  // It is executed with delegatecall on the DiamondInit address.
  let functionCall = diamondInit.interface.encodeFunctionData('init');

  // Setting arguments that will be used in the diamond constructor
  const diamondArgs: any = {
    owner: contractOwner.address,
    init: await diamondInit.getAddress(),
    initCalldata: functionCall,
  };

  if (contractResolver !== null) {
    diamondArgs['contractResolver'] = contractResolver;
  }
  if (env !== null) {
    diamondArgs['env'] = env;
  }

  // deploy Diamond
  const diamond = await hardhatDeployAndVerifySingleContract(
    ethers,
    hre.network.name,
    diamondName,
    {
      signer,
      deploymentArgs: [facetCuts, diamondArgs],
      verifyContracts,
      waitForDeployment,
    }
  );

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
  const deployed = await deployDiamond(
    contractName,
    args.length == 2 ? args[0] : null, // contract resolver address
    args.length == 2 ? args[1] : null, // env
    {
      additionalFacets: facets,
      useErc165Loupe,
    }
  );
  const diamondContract = deployed.diamond;
  const deployedFacets = deployed.facets;
  console.log(
    `${contractName} deployed to ${await diamondContract.getAddress()}`
  );
  console.log('Deployed facets:' + JSON.stringify(deployedFacets, null, 2));
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

export async function getDiamondContract(
  contractName: string,
  diamondAddress: string
) {
  const abiPath = path.resolve(
    __dirname,
    `../artifacts/hardhat-diamond-abi/${contractName}Diamond.sol/${contractName}Diamond.json`
  );
  // @ts-ignore
  const abi = JSON.parse(fs.readFileSync(abiPath)).abi;
  const [deployer] = await ethers.getSigners();
  const diamond = new ethers.Contract(diamondAddress, abi, deployer);
  return { diamond };
}

if (require.main === module) {
  async function run() {
    await deployDiamondContract(
      hre.network.name,
      'BackupRecovery',
      ['0x5305D4139121ACdECdF24E1D2b9bF6d197E6B95a', 0],
      ['BackupRecoveryFacet']
    );
  }

  run();
}
