// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require('hardhat');
const fs = require('fs');
var spawn = require('child_process').spawn;
const nacl = require('tweetnacl');
const { ethers } = hre;
const chainName = hre.network.name;
const rpcUrl = hre.network.config.url;

const port = '7470';
const incrementPort = true;

async function getChainId() {
  const { chainId } = await ethers.provider.getNetwork();
  return chainId;
}

console.log('Setting IP addresses for chain' + chainName);

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const ip2int = (ip) => {
  return (
    ip.split('.').reduce(function (ipInt, octet) {
      return (ipInt << 8) + parseInt(octet, 10);
    }, 0) >>> 0
  );
};

async function main() {
  const rl = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  const stakingContractAddress = await new Promise((resolve) => {
    rl.question('What is the staking contract address? ', resolve);
  });

  const walletsJsonPath = await new Promise((resolve) => {
    rl.question('What is the wallets.json path? ', resolve);
  });
  let walletsJson = fs.readFileSync(walletsJsonPath);
  const wallets = JSON.parse(walletsJson);

  for (let i = 0; i < wallets.length; i++) {
    let w = wallets[i];
    const signer = new ethers.Wallet(w.staker.privateKey, ethers.provider);
    const stakingContract = await ethers.getContractAt(
      'StakingFacet',
      stakingContractAddress,
      signer
    );

    // prompt for ip address to set for the node
    let ip = await new Promise((resolve) => {
      rl.question(
        `What IP address do you want to use for node ${i + 1}? `,
        resolve
      );
    });

    // // prompt for port to set for the node
    // let port = await new Promise((resolve) => {
    //     rl.question(
    //         `What port do you want to use for node ${i + 1}? `,
    //         resolve
    //     );
    // });

    const ipAsInt = ip2int(ip);
    ip = BigInt(ipAsInt);
    const ipv6 = BigInt('0');
    const bigNumPort = BigInt(parseInt(port) + (incrementPort ? i : 0));

    const txn =
      await stakingContract.setIpPortNodeAddressAndCommunicationPubKeys(
        ip,
        ipv6,
        bigNumPort,
        w.node.address,
        w.node.comsKeysSender.publicKey,
        w.node.comsKeysReceiver.publicKey
      );
    console.log(`Transaction hash: ${txn.hash}`);
    await txn.wait();
    console.log(`Transaction mined`);
  }

  process.exit(0);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
