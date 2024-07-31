import hre from 'hardhat';
import path from 'path';
import { SimpleGit, simpleGit } from 'simple-git';
import yargs from 'yargs';
import { hardhatCompile } from '../utils';
import {
  MANIFESTS_DIR,
  appendDiamondCutOperationToManifest,
  executeDiamondCutOperations,
} from './lib/diamondCutManifest';
import { removeFacet } from './lib/removeFacet';

const { ethers } = hre;

const FACETS_TO_REPLACE: FacetToReplace[] = [
  {
    contractName: 'BackupRecoveryFacet',
    diamondAddress: '0xf4B146FbA71F41E0592668ffbF264F1D186b2Ca8',
    newFacetAddress: '0x28cA4b9B360eD4F918081C921B8a299fd491e96a',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'PKPNFTFacet',
    diamondAddress: '0x70e0bA845a1A0F2DA3359C97E0285013525FFC49',
    newFacetAddress: '0xBf7D170515a7D956951bF2f35Cea9c3793b10c1b',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'PKPPermissionsFacet',
    diamondAddress: '0x21dF544947ba3E8b3c32561399E88B52Dc8b2823',
    newFacetAddress: '0x8eA150155C63b3A2e34B61409FB65E19F1BD48E7',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'PubkeyRouterFacet',
    diamondAddress: '0x82e01223d51Eb87e16A03E24687EDF0F294da6f1',
    newFacetAddress: '0x4bb266678E7116D8A1df7aAe7625f9347b01eE85',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'RateLimitNFTFacet',
    diamondAddress: '0x809d550fca64d94Bd9F66E60752A544199cfAC3D',
    newFacetAddress: '0xaD56B19064CCbf47Ba266b88082e5E2f5D936d67',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'RateLimitNFTViewsFacet',
    diamondAddress: '0x809d550fca64d94Bd9F66E60752A544199cfAC3D',
    newFacetAddress: '0x1B182f4150E9cC1f611BbB36FE1B19B7048CBa2d',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'StakingFacet',
    diamondAddress: '0xE6E340D132b5f46d1e472DebcD681B2aBc16e57E',
    newFacetAddress: '0x5aE74C06DA7de27Fcd906b0C94c5eB6B654ac9eb',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'StakingVersionFacet',
    diamondAddress: '0xE6E340D132b5f46d1e472DebcD681B2aBc16e57E',
    newFacetAddress: '0xc365FBba5cAbeDC3d559088C701A8577825A441E',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'StakingViewsFacet',
    diamondAddress: '0xE6E340D132b5f46d1e472DebcD681B2aBc16e57E',
    newFacetAddress: '0x73b6bB5e8320aBf903dF44942a48fe96e97534Ab',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
  {
    contractName: 'StakingBalancesFacet',
    diamondAddress: '0x59b670e9fA9D0A427751Af201D676719a970857b',
    newFacetAddress: '0x7d856FcdCA403E1fCA8525d90370B8C449b79357',
    lastDeployedGitSha: '4a8a448040bfefd2feefb9f0ff38eab459f1a6a6',
  },
];

async function run() {
  const { diamondOwnerSignerPrivateKey } = await getInputsFromCliOptions();

  const git: SimpleGit = simpleGit();

  await hardhatCompile();

  // Attempt to replace all the facets.
  // We will use a custom manifest file for the diamond cut operations.
  // The manifest file will consist of the datetime and the git SHA.
  const currentGitSha = await git.revparse(['HEAD']);
  const manifestFilePath = path.join(
    __dirname,
    MANIFESTS_DIR,
    `diamondCutManifest_${new Date().toISOString()}_${currentGitSha}.json`
  );

  for (const facet of FACETS_TO_REPLACE) {
    console.info(`Attempting to replace facet: ${facet.contractName}`);

    // Get the contract object for the new facet.
    const newFacet = await ethers.getContractAt(
      facet.contractName,
      facet.newFacetAddress
    );

    // Append the diamond cut operation to the manifest file.
    await appendDiamondCutOperationToManifest(
      manifestFilePath,
      facet.diamondAddress,
      newFacet,
      1,
      facet.contractName
    );
  }

  // Execute diamond cut operations
  console.info('Executing diamond cut operations...');
  const { failedDiamondCutOperations } = await executeDiamondCutOperations(
    diamondOwnerSignerPrivateKey,
    manifestFilePath
  );
  if (failedDiamondCutOperations.length === 0) {
    console.info('All diamond cut operations succeeded');
    return;
  }

  console.error(
    `Failed diamond cut operation IDs: [${failedDiamondCutOperations
      .map((o) => o.operationId)
      .join(', ')}]`
  );
  console.info(
    'For each of these failed operations, we will attempt to remove the old facet and add the new facet. We will only proceed with each facet if the corresponding lastDeployedGitSha parameter is set.'
  );

  for (const failedOperation of failedDiamondCutOperations) {
    // Find entry in FACETS_TO_REPLACE.
    const facet = FACETS_TO_REPLACE.find(
      (f) => f.contractName === failedOperation.facetName
    );
    if (!facet) {
      console.error(
        `Failed to find facet in FACETS_TO_REPLACE: ${failedOperation.facetName}`
      );
      continue;
    } else if (!facet.lastDeployedGitSha) {
      console.error(
        `Invalid lastDeployedGitSha for facet ${
          facet!.contractName
        }. You will have to handle this facet manually.`
      );
      continue;
    }

    // Attempt to remove the old facet.
    console.info(`Attempting to remove facet: ${facet!.contractName}`);
    const err = await removeFacet(
      facet.lastDeployedGitSha,
      facet.contractName,
      diamondOwnerSignerPrivateKey,
      facet.diamondAddress
    );
    if (err) {
      console.error(
        `Failed to remove facet: ${
          facet!.contractName
        }. You will have to handle this facet manually.`
      );
      continue;
    }
    console.info(`Successfully removed facet: ${facet.contractName}`);

    // After removing the old facet, we need to add the new facet.
    await hardhatCompile();

    // Get the contract object for the new facet.
    const newFacet = await ethers.getContractAt(
      facet.contractName,
      facet.newFacetAddress
    );

    // Append the diamond cut operation to the manifest file.
    // We will use a new manifest for this addition operation.
    const addFacetManifestFilePath = path.join(
      __dirname,
      MANIFESTS_DIR,
      `diamondCutManifest_${new Date().toISOString()}_${currentGitSha}.json`
    );
    await appendDiamondCutOperationToManifest(
      addFacetManifestFilePath,
      facet.diamondAddress,
      newFacet,
      0,
      facet.contractName
    );

    // Execute diamond cut operation
    console.info('Executing diamond cut operation...');
    const { failedDiamondCutOperations: failedOperations } =
      await executeDiamondCutOperations(
        diamondOwnerSignerPrivateKey,
        addFacetManifestFilePath
      );
    if (failedOperations.length > 0) {
      console.error(
        `Failed to add new facet: ${
          facet!.contractName
        }. You will have to handle this facet manually.`
      );
      continue;
    }
    console.info(`Successfully added new facet: ${facet.contractName}`);
  }
}

run();

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'diamond-owner-signer-private-key': {
      type: 'string',
      describe:
        'Private key of the wallet that is the owner of the diamond contract',
      required: true,
    },
  }).argv;

  return argv;
}

interface Inputs {
  diamondOwnerSignerPrivateKey: string;
}

interface FacetToReplace {
  newFacetAddress: string;
  diamondAddress: string;
  contractName: string;
  lastDeployedGitSha?: string;
}
