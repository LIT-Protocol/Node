import hre from 'hardhat';
import path from 'path';
import { SimpleGit, simpleGit } from 'simple-git';
import { hardhatCompile } from '../../utils';
import {
  MANIFESTS_DIR,
  appendDiamondCutOperationToManifest,
  executeDiamondCutOperations,
} from './diamondCutManifest';

const { ethers } = hre;

/**
 * This function will remove a facet from the diamond contract.
 *
 * The function will:
 * 1. Checkout the specified git SHA.
 * 2. Compile the contracts.
 * 3. Get the contract object for the facet to be removed.
 * 4. Create a manifest file with the diamond cut operation.
 * 5. Execute the diamond cut operation.
 *
 * Whether the promise is resolved or rejected, the function will checkout the original branch.
 *
 * ## Errors
 * This function never throws an error. Any error that occurs during the process is logged to the console.
 */
export async function removeFacet(
  gitShaToCheckout: string,
  contractName: string,
  diamondOwnerSignerPrivateKey: string,
  diamondContractAddress: string
): Promise<Error | undefined> {
  const git: SimpleGit = simpleGit();

  // Get current branch
  const currentBranch = await git.branch();
  console.info('Current branch:', currentBranch.current);

  let error: Error | undefined;
  try {
    await _removeFacet(
      gitShaToCheckout,
      contractName,
      diamondOwnerSignerPrivateKey,
      diamondContractAddress
    );
  } catch (e: any) {
    console.error('Error removing facet:', e);
    error = e;
  } finally {
    // Checkout the original branch
    console.info('Checking out original branch:', currentBranch.current);
    await git.checkout(currentBranch.current);
    return error;
  }
}

async function _removeFacet(
  gitShaToCheckout: string,
  contractName: string,
  diamonOwnerSignerPrivateKey: string,
  diamondContractAddress: string
) {
  const git: SimpleGit = simpleGit();

  // Checkout specified git SHA
  console.info('Checking out git SHA:', gitShaToCheckout);
  await git.checkout(gitShaToCheckout);

  await hardhatCompile();

  // Get the contract object for the facet to be removed.
  // We just need the contract object to get the function selectors.
  const facetContract = await ethers.getContractAt(
    contractName,
    '0x0000000000000000000000000000000000000000'
  );

  // We will use a custom manifest file for the diamond cut operation.
  // The manifest file will consist of the datetime and the git SHA.
  const manifestFilePath = path.join(
    __dirname,
    `../${MANIFESTS_DIR}`,
    `diamondCutManifest_${new Date().toISOString()}_${gitShaToCheckout}.json`
  );
  await appendDiamondCutOperationToManifest(
    manifestFilePath,
    diamondContractAddress,
    facetContract,
    2,
    contractName
  );
  console.info('Manifest file created at:', manifestFilePath);

  // Execute diamond cut operations
  console.info('Executing diamond cut operations...');
  await executeDiamondCutOperations(
    diamonOwnerSignerPrivateKey,
    manifestFilePath
  );
}
