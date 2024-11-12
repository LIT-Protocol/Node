// Full Command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/diamondContracts/appendDiamondCutManifest.ts --contract-name <CONTRACT_NAME> --facet-cut-action <FACET_CUT_ACTION> --new-facet-address <NEW_FACET_ADDRESS> --diamond-contract-address <DIAMOND_CONTRACT_ADDRESS>

import hre from 'hardhat';
import yargs from 'yargs';
const { ethers } = hre;

import path from 'path';
import {
  MANIFESTS_DIR,
  appendDiamondCutOperationToManifest,
} from './lib/diamondCutManifest';

async function run() {
  const inputs = await getInputsFromCliOptions();

  // Read the new facet contract
  const newFacet = await ethers.getContractAt(
    inputs.contractName,
    inputs.newFacetAddress
  );

  // Create a new file: manifests/diamondCutManifest.json if not exist.
  const manifestFilePath = path.join(
    __dirname,
    MANIFESTS_DIR,
    'diamondCutManifest.json'
  );

  await appendDiamondCutOperationToManifest(
    manifestFilePath,
    inputs.diamondContractAddress,
    newFacet,
    inputs.facetCutAction,
    inputs.contractName
  );
}

run()
  .then(() => {
    console.log('Diamond cut manifest updated');
    process.exit(0);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'contract-name': {
      type: 'string',
      describe: 'Name of the facet contract',
      required: true,
    },
    'facet-cut-action': {
      type: 'number',
      describe: 'Facet cut action - 0 (Add), 1 (Replace), or 2 (Remove)',
      required: true,
    },
    'new-facet-address': {
      type: 'string',
      describe: 'Address of the new facet contract',
      required: true,
    },
    'diamond-contract-address': {
      type: 'string',
      describe: 'Address of the diamond contract',
      required: true,
    },
  }).argv;

  if (argv.facetCutAction < 0 || argv.facetCutAction > 2) {
    throw new Error(
      'Facet cut action must be 0 (Add), 1 (Replace), or 2 (Remove)'
    );
  }

  return argv;
}

interface Inputs {
  contractName: string;
  facetCutAction: number;
  newFacetAddress: string;
  diamondContractAddress: string;
}
