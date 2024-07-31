/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const { ip2int, int2ip } = require('../../utils');
const axios = require('axios');

const requiredArgs = ['stakingContractAddress'];

async function checkVersion({ validatorStruct }) {
  const ip = int2ip(validatorStruct.ip);
  // const ip = validatorStruct.ip;
  const port = validatorStruct.port.toString();
  const url = `https://${ip}:${port}/connect/meow`;
  console.log(`checking version for ${url}`);
  try {
    const response = await axios.get(url, {
      headers: { 'Content-Type': 'application/json' },
      timeout: 15000,
    });
    // const json = await response.json();
    // console.log(`headers: ${JSON.stringify(response.headers, null, 2)}`);
    return response.headers['x-lit-node-version'];
  } catch (e) {
    console.log(`error: ${e}`);
    return 'unknown';
  }
}

function parseValidatorStruct({ nodeInfo }) {
  return {
    ip: int2ip(nodeInfo.ip),
    port: nodeInfo.port.toString(),
    address: nodeInfo.nodeAddress,
    reward: nodeInfo.reward.toString(),
    comsKeySender: nodeInfo.senderPubKey.toString(),
    comsKeyReceiver: nodeInfo.receiverPubKey.toString(),
  };
}
async function getKickedNodes({ staking }) {
  const filter = staking.filters.ValidatorKickedFromNextEpoch;
  const events = await staking.queryFilter(filter, 1133745, 'latest');
  const nodes = [];
  for (let i = 0; i < events.length; i++) {
    const evt = events[i];
    // console.log(evt);
    nodes.push(evt.args[0]);
  }
  return nodes;
}

async function getInfoForNodes({ stakingViews, stakerAddresses }) {
  const nodeStructs = await stakingViews.getValidatorsStructs(
    stakerAddresses.map((a) => a.toString())
  );
  const nodes = [];
  for (let i = 0; i < nodeStructs.length; i++) {
    const stakerAddressFromMapping =
      await stakingViews.nodeAddressToStakerAddress(nodeStructs[i].nodeAddress);
    const version = await checkVersion({
      validatorStruct: nodeStructs[i],
    });
    const parsed = parseValidatorStruct({ nodeInfo: nodeStructs[i] });
    nodes.push({
      ...parsed,
      stakerAddressFromMapping,
      version,
      stakerAddressFromValidatorsArray: stakerAddresses[i],
    });
  }
  return nodes;
}

async function getNodeVersionsAndAddresses({ stakingContractAddress }) {
  const stakingViews = await ethers.getContractAt(
    'StakingViewsFacet',
    stakingContractAddress
  );

  const staking = await ethers.getContractAt(
    'StakingFacet',
    stakingContractAddress
  );

  const kickedValidatorsAddresses = await getKickedNodes({ staking });
  console.log(`got ${kickedValidatorsAddresses.length} kicked validators`);
  const currentValidatorsAddresses =
    await stakingViews.getValidatorsInCurrentEpoch();
  console.log(`got ${currentValidatorsAddresses.length} current validators`);
  const nextValidatorsAddresses = await stakingViews.getValidatorsInNextEpoch();
  console.log(`got ${nextValidatorsAddresses.length} next validators`);

  // dedup
  const allAddresses = [];
  const addressProvenance = {};

  // for (let i = 0; i < kickedValidatorsAddresses.length; i++) {
  //     const address = kickedValidatorsAddresses[i];
  //     allAddresses.push(address);
  //     addressProvenance[address] = ["kicked"];
  // }

  for (let i = 0; i < currentValidatorsAddresses.length; i++) {
    const address = currentValidatorsAddresses[i];
    if (allAddresses.indexOf(address) === -1) {
      allAddresses.push(address);
    }
    if (!addressProvenance[address]) {
      addressProvenance[address] = [];
    }
    addressProvenance[address].push('current');
  }

  for (let i = 0; i < nextValidatorsAddresses.length; i++) {
    const address = nextValidatorsAddresses[i];
    if (allAddresses.indexOf(address) === -1) {
      allAddresses.push(address);
    }
    if (!addressProvenance[address]) {
      addressProvenance[address] = [];
    }
    addressProvenance[address].push('next');
  }

  const nodes = await getInfoForNodes({
    stakingViews,
    stakerAddresses: allAddresses,
  });

  // put back address provenance info
  for (let i = 0; i < nodes.length; i++) {
    const node = nodes[i];
    node.provenance = addressProvenance[node.stakerAddressFromValidatorsArray];
  }

  // const nodeIps = [
  //     { ip: "23.82.129.77", port: "443" },
  //     { ip: "199.115.115.103", port: "443" },
  //     { ip: "207.244.113.222", port: "443" },
  //     { ip: "104.245.147.218", port: "443" },
  //     { ip: "207.244.91.227", port: "443" },
  //     { ip: "198.7.62.151", port: "443" },
  //     { ip: "108.59.12.12", port: "443" },
  //     { ip: "178.162.172.88", port: "443" },
  //     { ip: "199.115.116.98", port: "443" },
  // ];

  // for (let i = 0; i < nodeIps.length; i++) {
  //     const version = await checkVersion({
  //         validatorStruct: nodeIps[i],
  //     });
  //     nextValidatorsWithStakingAddress.push({
  //         ip: nodeIps[i].ip,
  //         port: nodeIps[i].port,
  //         version,
  //     });
  // }

  console.log(`Nodes: ${JSON.stringify(nodes, null, 2)}`);
}

if (require.main === module) {
  // for (let arg of requiredArgs) {
  //     if (!argv[arg]) {
  //         console.log(`Missing required argument: ${arg}`);
  //         return;
  //     }
  // }
  getNodeVersionsAndAddresses({
    // stakingContractAddress: "0xBC7F8d7864002b6629Ab49781D5199C8dD1DDcE1" // manzano
    stakingContractAddress: '0xde8627067188C0063384eC682D9187c7d7673934', // habanero
    // stakingContractAddress: "0x4B90EB9BfCc55DcdBdd1A9FE77627397da7d43e1", // internaldev
  });
}

module.exports = { getNodeVersionsAndAddresses };
