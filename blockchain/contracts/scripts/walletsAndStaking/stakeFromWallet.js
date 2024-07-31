/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const { stake } = require('./utils/stake');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const argv = yargs(hideBin(process.argv)).argv;
const fs = require('fs');

const requiredArgs = ['ip', 'stakingContractAddress', 'walletPath'];

async function stakeFromWallet({ ip, stakingContractAddress, walletPath }) {
  let rawdata = fs.readFileSync(walletPath);
  let wallet = JSON.parse(rawdata);

  let stakerPrivateKey = wallet.staker.privateKey;
  let nodeAddress = wallet.node.address;
  let comsKeySender = wallet.senderComsKeys.publicKey;
  let comsKeyReceiver = wallet.receiverComsKeys.publicKey;

  await stake({
    stakingContractAddress,
    stakerPrivateKey,
    nodeAddress,
    comsKeySender,
    comsKeyReceiver,
    ip,
    port: 443,
  });
}

if (require.main === module) {
  // for (let arg of requiredArgs) {
  //     if (!argv[arg]) {
  //         console.log(`Missing required argument: ${arg}`)
  //         return
  //     }
  // }
  const argv = {
    ip: '199.115.117.114',
    stakingContractAddress: '0x0C22F6b7efe2Fd68fa2bc6A6b5Ba6Ff8C0EDb2A0',
    walletPath: '../../../SecretsAndKeysAndPrivateKeys/InternalDev/node10.json',
  };
  stakeFromWallet(argv);
}
