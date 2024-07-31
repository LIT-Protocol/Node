/* This file is not completed yet, since the private key for the allowlist admin
 * lives in metamask.  This is probably almost working though
 */
const hre = require('hardhat');
const fs = require('fs');
var spawn = require('child_process').spawn;
const { getBytesFromMultihash } = require('../utils');
const { ethers } = hre;
const chainName = hre.network.name;
const rpcUrl = hre.network.config.url;

async function getChainId() {
  const { chainId } = await ethers.provider.getNetwork();
  return chainId;
}

const getSigner = async () => {
  const [deployer] = await ethers.getSigners();
  return deployer;
};

async function main() {
  const signer = await getSigner();

  const rl = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  const allowlistContractAddress = await new Promise((resolve) => {
    rl.question('What is the Allowlist contract address? ', resolve);
  });

  let ipfsId = await new Promise((resolve) => {
    rl.question('What is the IPFS CID? ', resolve);
  });

  // convert the ipfsId to bytes
  const ipfsIdBytes = getBytesFromMultihash(ipfsId);

  const allowlistContract = await ethers.getContractAt(
    'Allowlist',
    allowlistContractAddress,
    signer
  );

  const addTx = await allowlistContract.setAllowed(ipfsIdBytes);
  console.log('Success!');
  process.exit(0);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
