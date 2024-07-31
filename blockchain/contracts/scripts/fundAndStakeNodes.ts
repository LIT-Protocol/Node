// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.

import fs from 'fs';
import hre from 'hardhat';
import {
  DeployNodeConfig,
  FundAndStakeNodesOutput,
  NodeOperatorCredentials,
  ParsedNodeContracts,
} from './deployConfig';

import { TransactionResponse } from '@ethersproject/abstract-provider';

import { ContractTransactionResponse } from 'ethers';
import { StakingFacet } from '../typechain-types';
import { CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY } from './constants';
import {
  contractAddressAlreadyExists,
  generateWallets,
  getContractInstance,
} from './utils';

const { ethers } = hre;
const wlitAddress = hre.network.config.wlitAddress || false;

// how much gas to send to the nodes, and to the staker addresses.
// note that this will be divided up by the walletCount
const nodeAmount = ethers.parseEther('10');
const stakerAmount = ethers.parseEther('1');

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const ip2int = (ip: string) => {
  return (
    ip.split('.').reduce(function (ipInt, octet) {
      return (ipInt << 8) + parseInt(octet, 10);
    }, 0) >>> 0
  );
};

const getSigner = async () => {
  const [deployer] = await ethers.getSigners();
  return deployer;
};

const fundWalletsWithGas = async (
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) => {
  const signer = await getSigner();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    contracts.multisenderContractAddress,
    signer
  );
  console.log(
    'multisender contract address is ',
    await multisenderContract.getAddress()
  );

  const nodeTx = await multisenderContract.sendEth(
    nodeOperatorsCredentials.map((w) => w.nodeWallet.address),
    { value: nodeAmount }
  );
  console.log('fundWalletsWithGas nodeTx: ', nodeTx);

  const stakerTx = await multisenderContract.sendEth(
    nodeOperatorsCredentials.map((w) => w.stakerWallet.address),
    { value: stakerAmount }
  );
  console.log('fundWalletsWithGas stakerTx: ', stakerTx);

  await Promise.all([nodeTx.wait(), stakerTx.wait()]);
  console.log('mined nodeTx and stakerTx');
};

const fundWalletsWithTokens = async (
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) => {
  const signer = await getSigner();

  const multisenderContract = await ethers.getContractAt(
    'Multisender',
    contracts.multisenderContractAddress,
    signer
  );

  const stakerTx = await multisenderContract.sendTokens(
    nodeOperatorsCredentials.map((w) => w.stakerWallet.address),
    contracts.litTokenContractAddress
  );
  console.log('fundWalletsWithTokens stakerTx: ', stakerTx);
  await stakerTx.wait();
  console.log('stakerTx mined');
};

const getStakingContract = async (
  contracts: ParsedNodeContracts,
  signer: any
): Promise<StakingFacet> => {
  return ethers.getContractAt(
    'StakingFacet',
    contracts.stakingContractAddress,
    signer
  );
};

const stakeTokensAndLockValidatorSet = async (
  allNodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  numberOfStakedAndJoinedWallets: number,
  numberOfStakedOnlyWallets: number,
  contracts: ParsedNodeContracts
) => {
  const signer = await getSigner();

  const stakingBalancesContract = await ethers.getContractAt(
    'StakingBalancesFacet',
    contracts.stakingBalancesContractAddress,
    signer
  );

  const stakingContract = await getStakingContract(contracts, signer);

  let litTokenContract;
  if (wlitAddress) {
    // use wlit
    litTokenContract = await ethers.getContractAt(
      'WLIT',
      contracts.litTokenContractAddress,
      signer
    );
  } else {
    litTokenContract = await ethers.getContractAt(
      'LITToken',
      contracts.litTokenContractAddress,
      signer
    );
  }

  // approve all stakers
  const amountToStake = await stakingBalancesContract.minimumStake();
  console.log('amountToStake: ', amountToStake);
  console.log('sending approval txns now');
  const totalStakers =
    numberOfStakedAndJoinedWallets + numberOfStakedOnlyWallets;
  const approvalPromises = [];
  for (let i = 0; i < totalStakers; i++) {
    const nodeOperatorCredential = allNodeOperatorsCredentials[i];

    const connectedStakerWallet = nodeOperatorCredential.stakerWallet.connect(
      ethers.provider
    );

    const litTokenContractAsStaker = litTokenContract.connect(
      connectedStakerWallet
    );

    console.log(
      'stakeTokens - approving tokens for staker: ',
      connectedStakerWallet.address
    );
    const approvalTx = await litTokenContractAsStaker.approve(
      contracts.stakingBalancesContractAddress,
      amountToStake
    );
    console.log('approvalTx for wallet ' + i + ': ', approvalTx);

    approvalPromises.push(approvalTx);
  }

  console.log('awaiting approval txns to be mined...');
  await Promise.all(
    approvalPromises.map((tx: ContractTransactionResponse) => {
      return tx.wait();
    })
  );

  // stake all the stakers
  console.log('sending staking txns now');
  const stakingPromises = [];
  for (let i = 0; i < totalStakers; i++) {
    const nodeOperatorCredential = allNodeOperatorsCredentials[i];

    const connectedStakerWallet = nodeOperatorCredential.stakerWallet.connect(
      ethers.provider
    );

    const stakingContractAsStaker = stakingContract.connect(
      connectedStakerWallet
    );
    // check balance
    const balance = await litTokenContract.balanceOf(
      connectedStakerWallet.address
    );
    console.log(`balance for ${connectedStakerWallet.address}: `, balance);
    console.log(
      'stakeTokens - staking tokens for staker: ',
      connectedStakerWallet.address
    );
    const tx = await stakingContractAsStaker.stake(amountToStake);
    console.log('stakeTokens tx for wallet ' + i + ': ', tx);

    stakingPromises.push(tx);
  }
  console.log(`awaiting ${stakingPromises.length} staking txns to be mined...`);
  await Promise.all(
    stakingPromises.map((tx: TransactionResponse) => {
      return tx.wait();
    })
  );

  // request to join for the stakers that are joining
  console.log('sending requestToJoin txns now');
  const requestToJoinPromises = [];
  for (let i = 0; i < numberOfStakedAndJoinedWallets; i++) {
    const nodeOperatorCredential = allNodeOperatorsCredentials[i];

    const ipAsInt = ip2int(contracts.litNodeDomainName);
    const ip = BigInt(ipAsInt);
    const ipv6 = 0n;
    const basePort = BigInt(contracts.litNodePort);
    const port = basePort + BigInt(i);

    const connectedStakerWallet = nodeOperatorCredential.stakerWallet.connect(
      ethers.provider
    );

    const stakingContractAsStaker = stakingContract.connect(
      connectedStakerWallet
    );
    const tx = await stakingContractAsStaker.requestToJoin(
      ip,
      ipv6,
      port,
      nodeOperatorCredential.nodeWallet.address,
      nodeOperatorCredential.comsKeysSender.publicKey,
      nodeOperatorCredential.comsKeysReceiver.publicKey
    );
    console.log('requestToJoin tx for wallet ' + i + ': ', tx);

    requestToJoinPromises.push(tx);
  }

  console.log(
    `awaiting ${requestToJoinPromises.length} requestToJoin txns to be mined...`
  );
  await Promise.all(
    requestToJoinPromises.map((tx: TransactionResponse) => {
      return tx.wait();
    })
  );

  console.log('locking the validator set');
  const lockTx = await stakingContract.lockValidatorsForNextEpoch();
  console.log('lockTx: ', lockTx.hash);
  await lockTx.wait();
  console.log('lockTx mined');

  // set epoch length to 5 mins - this is placed after we lock the validator set
  // so we don't have to wait so long to lock.
  console.log('setting epoch length to 5 mins');
  const epochLength = 300n;
  const setEpochLengthTx = await stakingContract.setEpochLength(epochLength);
  console.log('setEpochLengthTx: ', setEpochLengthTx.hash);
  await setEpochLengthTx.wait();
  console.log('setEpochLengthTx mined');
};

async function setIpAddresses(
  ipAddresses: string[],
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) {
  for (let i = 0; i < nodeOperatorsCredentials.length; i++) {
    const nodeOperatorCredential = nodeOperatorsCredentials[i];
    const signer = new ethers.Wallet(
      nodeOperatorCredential.stakerWallet.privateKey,
      ethers.provider
    );
    const stakingContract = await getStakingContract(contracts, signer);

    // prompt for ip address to set for the node
    const ipAddress = ipAddresses[i];
    let ip = ipAddress;
    let port = 443;
    if (ipAddress.includes(':')) {
      const parts = ipAddress.split(':');
      ip = parts[0];
      port = parseInt(parts[1]);
    }

    const ipBn = BigInt(ip2int(ip));
    const ipv6 = 0n;
    const portBn = BigInt(port);

    console.log('setting ip address for node: ', {
      ipBn,
      ipv6,
      portBn,
      nodeOperatorCredential,
    });

    const txn =
      await stakingContract.setIpPortNodeAddressAndCommunicationPubKeys(
        ipBn,
        ipv6,
        portBn,
        nodeOperatorCredential.nodeWallet.address,
        nodeOperatorCredential.comsKeysSender.publicKey,
        nodeOperatorCredential.comsKeysReceiver.publicKey
      );
    console.log(`Transaction hash: ${txn.hash}`);
    await txn.wait();
    console.log(
      `Transaction mined: https://chain.litprotocol.com/tx/${txn.hash}`
    );
  }
}

export async function fundAndStakeNodes(
  deployNodeConfig: DeployNodeConfig
): Promise<FundAndStakeNodesOutput> {
  const fileName = deployNodeConfig.outputTempFilePath;
  console.log('reading from file: ' + fileName);
  let contractsJsonStr = fs.readFileSync(fileName);
  const contracts: ParsedNodeContracts = JSON.parse(
    contractsJsonStr.toString()
  );

  const totalStakers =
    deployNodeConfig.numberOfStakedOnlyWallets +
    deployNodeConfig.numberOfStakedAndJoinedWallets;
  if (totalStakers === 0) {
    console.log('No node wallets to fund and stake');
    return { nodeOperatorsCredentials: [], contracts };
  }

  const chainName = deployNodeConfig.networkName;

  console.log('Funding and staking to network', {
    deployNodeConfig,
    chainName,
  });

  // *** 1. Generate or use existing wallets
  let nodeOperatorsCredentials = generateWallets(ethers, totalStakers);

  // *** 2. Fund node and staker wallets with gas
  await fundWalletsWithGas(nodeOperatorsCredentials, contracts);

  // *** 3. Fund staker wallets with LIT
  await fundWalletsWithTokens(nodeOperatorsCredentials, contracts);

  // *** 4. Print balances of node and staker wallets
  await logGasAndTokenBalances(nodeOperatorsCredentials, contracts);

  // *** 5. Stake and lock validator set.  only stake for numberOfStakedWallets nodes
  await stakeTokensAndLockValidatorSet(
    nodeOperatorsCredentials,
    deployNodeConfig.numberOfStakedAndJoinedWallets,
    deployNodeConfig.numberOfStakedOnlyWallets,
    contracts
  );

  if (deployNodeConfig.ipAddresses) {
    // *** 6. Set the IP addresses in the staking contract ONLY for the nodes that are joining
    await setIpAddresses(
      deployNodeConfig.ipAddresses,
      nodeOperatorsCredentials.slice(
        0,
        deployNodeConfig.numberOfStakedAndJoinedWallets
      ),
      contracts
    );
  }

  // Finally, transfer ownership of the contracts.
  await transferContractsOwnership(contracts, deployNodeConfig);

  return {
    nodeOperatorsCredentials,
    contracts,
  };
}

async function logGasAndTokenBalances(
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) {
  const signer = await getSigner();

  const litTokenContract = await ethers.getContractAt(
    'LITToken',
    contracts.litTokenContractAddress,
    signer
  );

  // Log the gas and token balance of the signer
  const signerEthBalance = await ethers.provider.getBalance(signer.address);
  const signerTokenBalance = await litTokenContract.balanceOf(signer.address);

  console.log(
    `Signer ${
      signer.address
    } - gas balance: ${signerEthBalance.toString()}, token balance: ${signerTokenBalance.toString()}`
  );

  for (let i = 0; i < nodeOperatorsCredentials.length; i++) {
    const nodeOperatorCredential = nodeOperatorsCredentials[i];

    // Get gas and token balance of node wallet
    const nodeWalletEthBalance = await ethers.provider.getBalance(
      nodeOperatorCredential.nodeWallet.address
    );
    const nodeWalletTokenBalance = await litTokenContract.balanceOf(
      nodeOperatorCredential.nodeWallet.address
    );

    // Get gas and token balance of staker wallet
    const stakerWalletEthBalance = await ethers.provider.getBalance(
      nodeOperatorCredential.stakerWallet.address
    );
    const stakerWalletTokenBalance = await litTokenContract.balanceOf(
      nodeOperatorCredential.stakerWallet.address
    );

    // Log
    console.log(
      `Node wallet (${i}) ${
        nodeOperatorCredential.nodeWallet.address
      } - gas balance: ${nodeWalletEthBalance.toString()}, token balance: ${nodeWalletTokenBalance.toString()}`
    );
    console.log(
      `Staker wallet (${i}) ${
        nodeOperatorCredential.stakerWallet.address
      } - gas balance: ${stakerWalletEthBalance.toString()}, token balance: ${stakerWalletTokenBalance.toString()}`
    );
  }
}

async function transferContractsOwnership(
  contracts: ParsedNodeContracts,
  deployNodeConfig: DeployNodeConfig
) {
  const signer = await getSigner();
  const newOwnerAddress = deployNodeConfig.newOwnerAddress;
  if (signer.address === newOwnerAddress) {
    console.info('Signer is already the new owner. Skipping...');
    return;
  }

  const allowlistContract = await ethers.getContractAt(
    'Allowlist',
    contracts.allowlistContractAddress,
    signer
  );
  console.info('Transferring ownership for Allowlist contract...');
  await transferOwnershipToNewOwner(allowlistContract, newOwnerAddress);

  const stakingContract = await getContractInstance(
    ethers,
    'Staking',
    contracts.stakingContractAddress,
    signer,
    true
  );
  console.info('Transferring ownership for Staking contract...');
  await transferOwnershipToNewOwner(stakingContract, newOwnerAddress);

  const remainingContracts = [
    { name: 'RateLimitNFT', isDiamond: true },
    { name: 'PubkeyRouter', isDiamond: true },
    { name: 'Multisender', isDiamond: false },
    { name: 'PKPPermissions', isDiamond: true },
    { name: 'PKPHelper', isDiamond: false },
    { name: 'PKPNFT', isDiamond: true },
    { name: 'DomainWalletRegistry', isDiamond: true },
  ];

  for (const remainingContract of remainingContracts) {
    await transferOwnershipIfNotAlreadyExist(
      contracts,
      deployNodeConfig,
      remainingContract,
      signer
    );
  }
}

async function transferOwnershipIfNotAlreadyExist(
  contracts: ParsedNodeContracts,
  deployNodeConfig: DeployNodeConfig,
  contract: { name: string; isDiamond: boolean },
  signer: any
) {
  if (
    !contractAddressAlreadyExists(
      deployNodeConfig.existingContracts as ParsedNodeContracts,
      contract.name
    )
  ) {
    const contractInstance = await getContractInstance(
      ethers,
      contract.name,
      // @ts-ignore
      contracts[
        // @ts-ignore
        CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY[contract.name]
      ],
      signer,
      contract.isDiamond
    );
    console.info(`Transferring ownership for ${contract.name} contract...`);
    await transferOwnershipToNewOwner(
      contractInstance,
      deployNodeConfig.newOwnerAddress
    );
  }
}

async function transferOwnershipToNewOwner(
  contract: any,
  newOwnerAddress: string
) {
  console.log(`Setting new owner to ${newOwnerAddress}`);
  const tx = await contract.transferOwnership(newOwnerAddress);
  const txReceipt = await tx.wait();
  console.log('New owner set.');

  return txReceipt;
}
