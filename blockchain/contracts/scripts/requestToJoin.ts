// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/requestToJoin.ts --staker-wallet-private-key <PRIVATE_KEY> --staking-address <STAKING_CONTRACT_ADDRESS> --staking-balances-address <STAKING_BALANCES_CONTRACT_ADDRESS> --validator-ip <VALIDATOR_IP> --validator-port <VALIDATOR_PORT> --validator-node-address <VALIDATOR_NODE_ADDRESS> --validator-comms-sender-pubkey <VALIDATOR_COMMS_SENDER_PUBKEY> --validator-comms-receiver-pubkey <VALIDATOR_COMMS_RECEIVER_PUBKEY>

import { Signer, toBigInt } from 'ethers';
import hre from 'hardhat';
import yargs from 'yargs';
import { StakingBalancesFacet, StakingFacet } from '../typechain-types';
import { ip2int } from '../utils';
const { ethers } = hre;

async function run() {
  const inputs = await getInputsFromCliOptions();

  // Get signer
  const signer = new ethers.Wallet(inputs.stakerWalletPrivateKey).connect(
    ethers.provider
  );
  console.log('signer address', signer.address);

  await joinNetwork(
    signer,
    {
      stakingAddress: inputs.stakingAddress,
      stakingBalancesAddress: inputs.stakingBalancesAddress,
    },
    {
      ip: inputs.validatorIp,
      port: inputs.validatorPort,
      nodeAddress: inputs.validatorNodeAddress,
      commsSenderPubkey: inputs.validatorCommsSenderPubkey,
      commsReceiverPubkey: inputs.validatorCommsReceiverPubkey,
    }
  );
}

async function joinNetwork(
  walletToJoinWith: Signer,
  contractAddresses: {
    stakingAddress: string;
    stakingBalancesAddress: string;
  },
  validatorConfig: {
    ip: string;
    port: number;
    nodeAddress: string;
    commsSenderPubkey: string;
    commsReceiverPubkey: string;
  }
) {
  const staking: StakingFacet = await ethers.getContractAt(
    'StakingFacet',
    contractAddresses.stakingAddress,
    walletToJoinWith
  );
  const stakingBalances: StakingBalancesFacet = await ethers.getContractAt(
    'StakingBalancesFacet',
    contractAddresses.stakingBalancesAddress,
    walletToJoinWith
  );

  console.info('Requesting to join network', {
    balanceBefore: ethers.formatEther(
      await ethers.provider.getBalance(walletToJoinWith.getAddress())
    ),
    ip: toBigInt(ip2int(validatorConfig.ip)),
    port: toBigInt(validatorConfig.port),
    nodeAddress: validatorConfig.nodeAddress,
    commsSenderPubkey: validatorConfig.commsSenderPubkey,
    commsReceiverPubkey: validatorConfig.commsReceiverPubkey,
  });

  try {
    const requestToJoinTx = await staking.requestToJoin(
      toBigInt(ip2int(validatorConfig.ip)),
      0n,
      toBigInt(validatorConfig.port),
      validatorConfig.nodeAddress,
      validatorConfig.commsSenderPubkey,
      validatorConfig.commsReceiverPubkey
    );
    const requestToJoinTxReceipt = await requestToJoinTx.wait();
    console.info('tx receipt for request to join', {
      requestToJoinTxReceipt,
    });
  } catch (e: any) {
    if (!!staking.interface.parseError(e.data)) {
      console.error(
        'Error requesting to join network',
        e,
        staking.interface.parseError(e.data)
      );
    } else if (!!stakingBalances.interface.parseError(e.data)) {
      console.error(
        'Error requesting to join network',
        e,
        stakingBalances.interface.parseError(e.data)
      );
    } else {
      console.error('Error requesting to join network', e);
    }

    throw new Error(`Error requesting to join network: ${e}`);
  }
}

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'staker-wallet-private-key': {
      type: 'string',
      describe:
        'Private key of the wallet that will be used to request to join the network',
      required: true,
    },
    'staking-address': {
      type: 'string',
      describe: 'Staking contract address',
      required: true,
    },
    'staking-balances-address': {
      type: 'string',
      describe: 'Staking balances contract address',
      required: true,
    },
    'validator-ip': {
      type: 'string',
      describe: 'Validator IP address',
      required: true,
    },
    'validator-port': {
      type: 'number',
      describe: 'Validator port',
      required: true,
    },
    'validator-node-address': {
      type: 'string',
      describe: 'Validator node address',
      required: true,
    },
    'validator-comms-sender-pubkey': {
      type: 'string',
      describe: 'Validator comms sender pubkey',
      required: true,
    },
    'validator-comms-receiver-pubkey': {
      type: 'string',
      describe: 'Validator comms receiver pubkey',
      required: true,
    },
  }).argv;

  return argv;
}

run();

interface Inputs {
  stakerWalletPrivateKey: string;
  stakingAddress: string;
  stakingBalancesAddress: string;

  // Validator info
  validatorIp: string;
  validatorPort: number;
  validatorNodeAddress: string;
  validatorCommsSenderPubkey: string;
  validatorCommsReceiverPubkey: string;
}
