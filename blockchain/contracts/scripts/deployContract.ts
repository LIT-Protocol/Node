// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/deployContract.ts --deployer-private-key <PRIVATE_KEY> --new-contract-name <NEW_CONTRACT_NAME>

import hre from 'hardhat';
import yargs from 'yargs';

import { hardhatDeployAndVerifySingleContract } from './utils';

const { ethers } = hre;

// CONFIGURE THIS //
const args: any[] = [];

async function run() {
  const inputs = await getInputsFromCliOptions();

  const deployer = new ethers.Wallet(inputs.deployerPrivateKey).connect(
    ethers.provider
  );
  hardhatDeployAndVerifySingleContract(
    ethers,
    hre.network.name,
    inputs.newContractName,
    {
      signer: deployer,
      deploymentArgs: args,
    }
  );
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
    'new-contract-name': {
      type: 'string',
      describe: 'Name of the new contract',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  deployerPrivateKey: string;
  newContractName: string;
}
