/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const { deployDiamond } = require('./deployDiamond');
const { verifyContractInBg } = require('./utils');

async function deployContract(contractName, args = [], facets) {
  const accounts = await ethers.getSigners();

  const networkName = hre.network.name;

  console.log(`Deploying ${contractName} with args ${args}`);
  const deployed = await deployDiamond(contractName, ...args, facets, true);

  const diamondContract = deployed.diamond;
  const deployedFacets = deployed.facets;
  console.log(
    `${contractName} deployed to ${await diamondContract.getAddress()}`
  );

  console.log('contract address: ', await diamondContract.getAddress());
  console.log('facets: ', JSON.stringify(deployedFacets, null, 2));
}

async function deployRegularContract(contractName, args = []) {
  const contract = await ethers.deployContract(contractName, args);
  await contract.waitForDeployment();
  console.log('contract address: ', await contract.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
if (require.main === module) {
  // diamond deploy
  // const args = [
  //   '0x9F0Ede26261451C5E784DC799D71ECf766EB7562', // resolver address
  //   0, // env
  // ];
  // deployContract('BackupRecovery', args, ['BackupRecoveryFacet']);

  // regular contract
  deployRegularContract('PKPHelperV2', [
    '0xe5a7C5d908EE8996332F488cE5f636d4EBff8522',
    2n,
  ]);
}
