// Run: STAKING_CONTRACT_OWNER_PRIVATE_KEY=<PRIVATE_KEY> npx hardhat run --network <NETWORK> scripts/lockValidators.ts

import hre from 'hardhat';
import { StakingFacet } from '../typechain-types';
const { ethers } = hre;

// TOUCH THIS //
const STAKING_CONTRACT_ADDRESS = '0xd816ba9aDee089DaCaaD6c9F09879d830Aec1629';

// DO NOT TOUCH THIS //
async function run() {
  // Get signer
  const signer = new ethers.Wallet(
    process.env.STAKING_CONTRACT_OWNER_PRIVATE_KEY || 'dummy-key'
  ).connect(ethers.provider);
  console.log('signer address', signer.address);

  // Get Staking contract using signer.
  const stakingContract: StakingFacet = await ethers.getContractAt(
    'StakingFacet',
    STAKING_CONTRACT_ADDRESS,
    signer
  );

  // Send TX to lock the validator set.
  console.log('locking the validator set');
  const lockTx = await stakingContract.lockValidatorsForNextEpoch();
  console.log('lockTx: ', lockTx.hash);
  await lockTx.wait();
  console.log('lockTx mined');
}

run();
