// Run: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/requestToLeave.ts --staker-wallet-private-key <PRIVATE_KEY> --staking-address <STAKING_CONTRACT_ADDRESS>

import hre from 'hardhat';
import yargs from 'yargs';
import { StakingFacet } from '../typechain-types';
const { ethers } = hre;

async function run() {
  const inputs = await getInputsFromCliOptions();

  // Get signer
  const signer = new ethers.Wallet(inputs.stakerWalletPrivateKey).connect(
    ethers.provider
  );
  console.log('signer address', signer.address);

  // Get Staking contract using signer.
  const stakingContract: StakingFacet = await ethers.getContractAt(
    'StakingFacet',
    inputs.stakingAddress,
    signer
  );

  // Send TX to request to leave.
  console.log('request to leave the validator set');
  try {
    const requestToLeaveTx = await stakingContract.requestToLeave();
    console.log('requestToLeaveTx: ', requestToLeaveTx.hash);
    await requestToLeaveTx.wait();
    console.log('requestToLeaveTx mined');
  } catch (e: any) {
    console.error(
      'Unable to request to leave',
      stakingContract.interface.parseError(e.data)
    );
    throw e;
  }
}

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'staker-wallet-private-key': {
      type: 'string',
      describe:
        'Existing wallet private key that will be used to add a new alias against',
      required: true,
    },
    'staking-address': {
      type: 'string',
      describe: 'Staking contract address',
      required: true,
    },
  }).argv;

  return argv;
}

run();

interface Inputs {
  stakerWalletPrivateKey: string;
  stakingAddress: string;
}
