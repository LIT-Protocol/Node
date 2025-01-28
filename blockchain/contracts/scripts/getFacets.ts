// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/getFacets.ts --facet-address <FACET_ADDRESS> --diamond-address <DIAMOND_ADDRESS>

import hre from 'hardhat';
import yargs from 'yargs';

import { hardhatDeployAndVerifySingleContract } from './utils';

const { ethers } = hre;

async function run() {
  const inputs = await getInputsFromCliOptions();

  const DiamondLoupeFacetNoERC165 = await ethers.getContractAt(
    'DiamondLoupeFacet',
    inputs.diamondAddress
  );
  const selectors = await DiamondLoupeFacetNoERC165.facetFunctionSelectors(
    inputs.facetAddress
  );
  console.log(selectors);
}

run();

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'facet-address': {
      type: 'string',
      describe: 'Address of the facet',
      required: true,
    },
    'diamond-address': {
      type: 'string',
      describe: 'Address of the diamond',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  facetAddress: string;
  diamondAddress: string;
}
