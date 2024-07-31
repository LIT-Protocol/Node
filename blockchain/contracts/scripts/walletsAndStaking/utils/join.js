/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const argv = yargs(hideBin(process.argv)).argv;
const { ip2int, int2ip } = require('../../../utils');

const requiredArgs = [
  'stakingContractAddress',
  'stakerPrivateKey',
  'nodeAddress',
  'comsKeySender',
  'comsKeyReceiver',
  'ip',
  'port',
];

async function join({
  stakingContractAddress,
  stakerPrivateKey,
  nodeAddress,
  comsKeySender,
  comsKeyReceiver,
  ip,
  port,
}) {
  const wallet = new ethers.Wallet(stakerPrivateKey, ethers.provider);
  const staking = await ethers.getContractAt(
    'StakingFacet',
    stakingContractAddress,
    wallet
  );
  let tx = await staking.requestToJoin(
    ip2int(ip),
    0,
    port,
    nodeAddress,
    comsKeySender,
    comsKeyReceiver
  );
  await tx.wait();
  console.log('Requested to join!');
}

if (require.main === module) {
  for (let arg of requiredArgs) {
    if (!argv[arg]) {
      console.log(`Missing required argument: ${arg}`);
      return;
    }
  }
  join(argv);
}

module.exports = { join };
