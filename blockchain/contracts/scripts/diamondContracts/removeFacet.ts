// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/diamond/removeFacet.ts --git-sha <GIT_SHA> --contract-name <CONTRACT_NAME> --diamond-owner-signer-private-key <PRIVATE_KEY> --diamond-contract-address <DIAMOND_CONTRACT_ADDRESS>

import yargs from 'yargs';
import { removeFacet } from './lib/removeFacet';

async function run() {
  const {
    gitSha,
    contractName,
    diamondOwnerSignerPrivateKey,
    diamondContractAddress,
  } = await getInputsFromCliOptions();

  await removeFacet(
    gitSha,
    contractName,
    diamondOwnerSignerPrivateKey,
    diamondContractAddress
  );
}

run();

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'git-sha': {
      type: 'string',
      describe: 'Git SHA to checkout facet contracts that should be removed',
      required: true,
    },
    'contract-name': {
      type: 'string',
      describe: 'Name of the facet contract',
      required: true,
    },
    'diamond-owner-signer-private-key': {
      type: 'string',
      describe:
        'Private key of the wallet that is the owner of the diamond contract',
      required: true,
    },
    'diamond-contract-address': {
      type: 'string',
      describe: 'Address of the diamond contract',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  contractName: string;
  gitSha: string;
  diamondOwnerSignerPrivateKey: string;
  diamondContractAddress: string;
}
