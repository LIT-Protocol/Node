// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/diamond/diamondCut.ts

import yargs from 'yargs';
import { executeDiamondCutOperations } from './lib/diamondCutManifest';

async function run() {
  const { diamondOwnerSignerPrivateKey } = await getInputsFromCliOptions();

  // Read the manifest file
  const fs = require('fs');
  const path = require('path');
  const manifestPath = path.join(__dirname, 'manifests');
  const manifestFilePath = path.join(manifestPath, 'diamondCutManifest.json');
  if (!fs.existsSync(manifestFilePath)) {
    throw new Error('diamondCutManifest.json not found');
  }

  console.log('Executing diamond cut operations');
  const { failedDiamondCutOperations } = await executeDiamondCutOperations(
    diamondOwnerSignerPrivateKey,
    manifestFilePath
  );
  if (failedDiamondCutOperations.length > 0) {
    console.error('Failed diamond cut operations:', failedDiamondCutOperations);
    throw new Error('Failed diamond cut operations');
  }
}

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

run();

interface Inputs {
  diamondOwnerSignerPrivateKey: string;
}
