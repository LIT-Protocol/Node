//RUN: npx hardhat run --network localchain scripts/set_epoch_state_restore.ts
import hre from 'hardhat';

const { ethers } = hre;
async function run() {
  if (!process.env.ADMIN_PRIVATE_KEY) {
    throw new Error('Provide your admin private key');
  }
  if (!process.env.STAKING_CONTRACT_ADDRESS) {
    throw new Error('Provide the staking contract address');
  }

  const signer = new ethers.Wallet(
    process.env.ADMIN_PRIVATE_KEY as string
  ).connect(ethers.provider);

  const stakingViews = await ethers.getContractAt(
    'StakingViewsFacet',
    process.env.STAKING_CONTRACT_ADDRESS as string,
    signer
  );
  const staking = await ethers.getContractAt(
    'StakingFacet',
    process.env.STAKING_CONTRACT_ADDRESS as string,
    signer
  );

  const tx = await staking.setEpochState(5); // RECOVERY=5
  await tx.wait();
  console.log('Updated epoch state to Recovery');

  const state = await stakingViews.state();
  console.log('Epoch state- ', state);
}

run();
