import { Contract } from 'ethers';
import fs from 'fs/promises';
import {
  DiamondCutManifest,
  DiamondCutOperation,
  FacetCutAction,
  FunctionSelector,
} from './types';
import { getSelectors } from './utils';
import { checkIfPathExists, ensureDirExists } from '../../utils';
import path from 'path';
import hre from 'hardhat';
import { DiamondCutFacet } from '../../../typechain-types';
const { ethers } = hre;

export const MANIFESTS_DIR = 'manifests';

const DEFAULT_MANIFEST: DiamondCutManifest = {
  operations: [],
};

export async function appendDiamondCutOperationToManifest(
  manifestFilePath: string,
  diamondAddress: string,
  facetContract: Contract,
  facetCutAction: FacetCutAction,
  facetName: string,
  oldFacetAddress?: string
): Promise<DiamondCutManifest> {
  // Ensure the manifests directory exists.
  await ensureDirExists(path.join(__dirname, `../${MANIFESTS_DIR}`));

  // Init manifest file if it doesn't exist
  await initManifestFile(manifestFilePath);

  // Read the manifest file
  const manifest: DiamondCutManifest = JSON.parse(
    await fs.readFile(manifestFilePath, 'utf8')
  );

  // Parse the number of existing operations written to this file. This is used to generate the next operation ID
  const operationId = manifest.operations.length;

  let functionSelectors: FunctionSelector[] = [];
  if (facetCutAction === FacetCutAction.Remove) {
    // go find the methods from the live facet and remove them.
    // don't use the local contracts - this gets really confusing because you have to load
    // old contracts in to the repo in order to remove them.

    const liveDiamondContract = await ethers.getContractAt(
      'DiamondLoupeFacetNoERC165',
      diamondAddress
    );
    if (!oldFacetAddress) {
      throw new Error('Old facet address is required for removal');
    }
    const selectors: string[] =
      await liveDiamondContract.facetFunctionSelectors(oldFacetAddress);
    functionSelectors = selectors.map((s) => ({
      selector: s,
      signature: 'unknown',
    }));
  } else {
    // Get the selectors for the new facet
    functionSelectors = getSelectors(facetContract);
  }

  // Write the new operation to the manifest
  manifest.operations.push({
    operationId,
    action: facetCutAction,
    facetName,
    diamondAddress,
    facetAddress:
      facetCutAction !== FacetCutAction.Remove
        ? await facetContract.getAddress()
        : ethers.ZeroAddress,
    functionSelectors,
  });

  // Write the manifest back to the file
  await fs.writeFile(manifestFilePath, JSON.stringify(manifest, null, 2));

  return manifest;
}

export async function initManifestFile(manifestFilePath: string) {
  if (!(await checkIfPathExists(manifestFilePath))) {
    await fs.writeFile(
      manifestFilePath,
      JSON.stringify(DEFAULT_MANIFEST, null, 2)
    );
  }
}

export async function executeDiamondCutOperations(
  diamondOwnerSignerPrivateKey: string,
  manifestFilePath: string
): Promise<{ failedDiamondCutOperations: DiamondCutOperation[] }> {
  // Read the manifest file
  const manifest: DiamondCutManifest = JSON.parse(
    await fs.readFile(manifestFilePath, 'utf8')
  );

  // Get diamond contract owner signer
  const diamondOwnerSigner = new ethers.Wallet(
    diamondOwnerSignerPrivateKey
  ).connect(ethers.provider);
  console.debug(
    'Addressed used to execute diamond cut operations',
    diamondOwnerSigner.address
  );

  // Perform each operation in the manifest
  let failedDiamondCutOperations: DiamondCutOperation[] = [];
  for (const operation of manifest.operations) {
    // load the diamond cut facet on the diamond contract
    const diamondContractAddress = operation.diamondAddress;
    const diamondCutFacet: DiamondCutFacet = await ethers.getContractAt(
      'DiamondCutFacet',
      diamondContractAddress,
      diamondOwnerSigner
    );

    const facetCut = {
      facetAddress: operation.facetAddress,
      action: operation.action,
      functionSelectors: operation.functionSelectors.map((s) => s.selector),
    };
    console.info(
      `Performing diamondCut operation on ${operation.facetName} with facet cut`,
      facetCut
    );

    try {
      const tx = await diamondCutFacet
        .connect(diamondOwnerSigner)
        .diamondCut([facetCut], ethers.ZeroAddress, '0x');

      console.debug('tx sent', tx);
      await tx.wait();
      console.debug('tx confirmed', tx);
      console.info(`Operation complete on ${operation.facetName}`);
    } catch (e: any) {
      // If this error is logged, you can enable hardhat-tracer in hardhat.config.ts, and then
      // use the following command to decode the error:
      // npx hardhat decode --data <DATA>
      // You will need to enable `hardhat-tracer` in hardhat.config.ts
      console.error('Unable to execute diamondCut operation', e, e.data);
      failedDiamondCutOperations.push(operation);
    }
  }

  return { failedDiamondCutOperations };
}
