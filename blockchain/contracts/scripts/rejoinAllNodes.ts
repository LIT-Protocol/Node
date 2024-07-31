import { Signer, toBigInt } from 'ethers';
import hre from 'hardhat';
import yargs from 'yargs';
import { StakingBalancesFacet, StakingFacet } from '../typechain-types';
import { ip2int } from '../utils';
const { ethers } = hre;
const fs = require('fs');

// this script is used to rejoin all nodes to the internaldev network usign the adminRejoinValidator() function

// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/rejoinAllNodes.ts --staking-address <STAKING_CONTRACT_ADDRESS>
// for example HARDHAT_NETWORK=lit npx ts-node --files scripts/rejoinAllNodes.ts --staking-address 0x7329FABF0a725AF8A58AcE1C41715Dd0BbB2177c --wallet-path /Users/chris/Documents/WorkStuff/Lit/SecretsAndKeysAndPrivateKeys/InternalDev/wallets-1714461597048-lit-10.json

async function run() {
  const inputs = await getInputsFromCliOptions();

  // Get signer
  const signer = new ethers.Wallet(
    process.env.LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY!
  ).connect(ethers.provider);
  console.log('signer address', signer.address);

  await rejoinNodes(inputs.walletPath, signer, {
    stakingAddress: inputs.stakingAddress,
  });
}

async function rejoinNodes(
  walletPath: string,
  walletToJoinWith: Signer,
  contractAddresses: {
    stakingAddress: string;
  }
) {
  const staking: StakingFacet = await ethers.getContractAt(
    'StakingFacet',
    contractAddresses.stakingAddress,
    walletToJoinWith
  );

  // get all the nodes staking addresses
  const wallets = JSON.parse(fs.readFileSync(walletPath, 'utf8'));
  const walletAddresses = wallets.map((wallet: any) => wallet.staker.address);

  for (const walletAddress of walletAddresses) {
    await rejoinNode(staking, walletAddress);
  }
}

async function rejoinNode(staking: StakingFacet, walletAddress: string) {
  console.log('rejoining', walletAddress);
  const tx = await staking.adminRejoinValidator(walletAddress);
  console.log(`tx hash for ${walletAddress}`, tx.hash);
  const receipt = await tx.wait();
  console.log(`rejoined ${walletAddress}`);
}

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'staking-address': {
      type: 'string',
      describe: 'Staking contract address',
      required: true,
    },
    'wallet-path': {
      type: 'string',
      describe: 'Path to the wallet file',
      required: true,
    },
  }).argv;

  return argv;
}

run();

interface Inputs {
  stakingAddress: string;
  walletPath: string;
}
