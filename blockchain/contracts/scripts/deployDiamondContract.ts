import hre from 'hardhat';
import yargs from 'yargs';

import { Environment } from '../utils/contract';
import { deployDiamond } from './deployDiamond';

const { ethers } = hre;

async function run() {
  const { newDiamondName, facets, deployerPrivateKey, contractResolver, env } =
    await getInputsFromCliOptions();

  const deployer = new ethers.Wallet(deployerPrivateKey).connect(
    ethers.provider
  );

  console.log(`Deploying ${newDiamondName}`, {
    facets,
    env,
    contractResolver,
  });
  const deployed = await deployDiamond(newDiamondName, contractResolver, env, {
    additionalFacets: facets,
    signer: deployer,
  });

  const diamondContract = deployed.diamond;
  const deployedFacets = deployed.facets;
  console.log(
    `${newDiamondName} deployed to ${await diamondContract.getAddress()}`
  );

  console.log('contract address: ', await diamondContract.getAddress());
  console.log('facets: ', JSON.stringify(deployedFacets, null, 2));
}

run();

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'deployer-private-key': {
      type: 'string',
      describe:
        'Private key of the wallet that will be used to deploy the new contract',
      required: true,
    },
    'new-diamond-name': {
      type: 'string',
      describe: 'Name of the new diamond contract',
      required: true,
    },
    facets: {
      type: 'array',
      describe: 'List of facets to deploy',
      required: true,
    },
    'contract-resolver': {
      type: 'string',
      describe: 'Resolver contract address',
      required: true,
    },
    env: {
      type: 'number',
      describe: 'Environment',
      required: true,
    },
  }).argv;

  for (const facet of argv.facets) {
    if (typeof facet !== 'string') {
      throw new Error('Facets must be strings');
    }
  }

  return argv as Inputs;
}

interface Inputs {
  deployerPrivateKey: string;
  newDiamondName: string;
  facets: string[];
  contractResolver: string;
  env: Environment;
}
