/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');
const argv = yargs(hideBin(process.argv)).argv;
const { ip2int, int2ip } = require('../../utils');

const requiredArgs = ['stakingContractAddress', 'stakerAddress'];

async function getNodeVersionsAndAddresses({
  stakingContractAddress,
  stakerAddress,
}) {
  const staking = await ethers.getContractAt(
    'StakingViewsFacet',
    stakingContractAddress
  );
  const contractResolver = await ethers.getContractAt(
    'ContractResolver',
    await staking.contractResolver()
  );
  const stakingBalancesContract = await ethers.getContractAt(
    'StakingBalancesFacet',
    await contractResolver.getContract(
      await contractResolver.STAKING_BALANCES_CONTRACT(),
      2
    )
  );

  const nodeInfo = await staking.validators(stakerAddress);
  console.log(`got node info: ${nodeInfo}`);

  const stakingBalance = await stakingBalancesContract.balanceOf(stakerAddress);
  const minStake = await stakingBalancesContract.minimumStake();
  const maxStake = await stakingBalancesContract.maximumStake();

  // parse node info
  const parsed = {
    ip: int2ip(nodeInfo.ip),
    port: nodeInfo.port.toString(),
    address: nodeInfo.nodeAddress,
    reward: nodeInfo.reward.toString(),
    comsKeySender: nodeInfo.senderPubKey.toString(),
    comsKeyReceiver: nodeInfo.receiverPubKey.toString(),
    staked: stakingBalance.toString(),
    minStake: minStake.toString(),
    maxStake: maxStake.toString(),
    stakingAmountInRange:
      stakingBalance > minStake && stakingBalance < maxStake,
    stakingAmountAboveRange: stakingBalance > maxStake,
    stakingAmountBelowRange: stakingBalance < minStake,
  };
  console.log(`parsed node info: ${JSON.stringify(parsed, null, 2)}`);
}

if (require.main === module) {
  // for (let arg of requiredArgs) {
  //     if (!argv[arg]) {
  //         console.log(`Missing required argument: ${arg}`);
  //         return;
  //     }
  // }
  getNodeVersionsAndAddresses({
    stakingContractAddress: '0xde8627067188C0063384eC682D9187c7d7673934',
    stakerAddress: '0xaa4389f15B926087976764f315032F2bF6905a5b',
  });
}

module.exports = { getNodeVersionsAndAddresses };
