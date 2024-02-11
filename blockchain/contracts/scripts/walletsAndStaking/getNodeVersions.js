/* global ethers */
/* eslint prefer-const: "off" */

const hre = require("hardhat");
const { ip2int, int2ip } = require("../../utils");
const axios = require("axios");

const requiredArgs = ["stakingContractAddress"];

async function checkVersion({ validatorStruct }) {
    const ip = int2ip(validatorStruct.ip);
    const port = validatorStruct.port.toString();
    const url = `https://${ip}:${port}/connect/meow`;
    console.log(`checking version for ${url}`);
    try {
        const response = await axios.get(url, {
            headers: { "Content-Type": "application/json" },
            timeout: 15000,
        });
        // const json = await response.json();
        // console.log(`headers: ${JSON.stringify(response.headers, null, 2)}`);
        return response.headers["x-lit-node-version"];
    } catch (e) {
        console.log(`error: ${e}`);
        return "unknown";
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

async function getNodeVersionsAndAddresses({ stakingContractAddress }) {
    const staking = await ethers.getContractAt(
        "StakingViewsFacet",
        stakingContractAddress
    );

    const currentValidatorsAddresses =
        await staking.getValidatorsInCurrentEpoch();
    const currentValidators = await staking.getValidatorsStructs(
        currentValidatorsAddresses.map((a) => a.toString())
    );
    console.log(`got ${currentValidators.length} current validators`);

    const nextValidatorsAddresses = await staking.getValidatorsInNextEpoch();
    const nextValidators = await staking.getValidatorsStructs(
        nextValidatorsAddresses.map((a) => a.toString())
    );
    console.log(`got ${nextValidators.length} next validators`);

    const nextValidatorsWithStakingAddress = [];
    for (let i = 0; i < nextValidators.length; i++) {
        const stakerAddressFromMapping =
            await staking.nodeAddressToStakerAddress(
                nextValidators[i].nodeAddress
            );
        const version = await checkVersion({
            validatorStruct: nextValidators[i],
        });
        const parsed = parseValidatorStruct({ nodeInfo: nextValidators[i] });
        nextValidatorsWithStakingAddress.push({
            ...parsed,
            stakerAddressFromMapping,
            version,
            stakerAddressFromValidatorsArray: nextValidatorsAddresses[i],
        });
    }

    console.log(
        `Next Validators: ${JSON.stringify(
            nextValidatorsWithStakingAddress,
            null,
            2
        )}`
    );
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
        // stakingContractAddress: "0xde8627067188C0063384eC682D9187c7d7673934", // habanero
        stakingContractAddress: "0x4B90EB9BfCc55DcdBdd1A9FE77627397da7d43e1", // internaldev
    });
}

module.exports = { getNodeVersionsAndAddresses };
