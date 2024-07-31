import fs from 'fs';
import hre from 'hardhat';
import yargs from 'yargs/yargs';

import { Signer, Wallet, toBigInt } from 'ethers';
import { StakingBalancesFacet, StakingFacet } from '../typechain-types';
import { ip2int } from '../utils';
import { ParsedNodeContracts } from './deployConfig';
import {
  copyDirFiles,
  generateWallets,
  saveConfigFiles,
  serializeWallets,
} from './utils';
const { ethers } = hre;

// NOTE: This does not work currently since when this script is run as `npx hardhat`, hardhat does not
// recognize these options. It only works when run as `node ./scripts/generate_wallet_and_add_as_alias.js`.
async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'deployed-node-contracts-path': {
      type: 'string',
      describe: 'Path to the deployed node contracts manifest',
      required: true,
    },
    'existing-staker-wallet-private-key': {
      type: 'string',
      describe:
        'Existing wallet private key that will be used to add a new alias against',
      required: true,
    },
    'node-config-ipfs-api-key': {
      type: 'string',
      describe: 'IPFS API key for the generated node config',
      required: true,
    },
    'node-config-admin-address': {
      type: 'string',
      describe: 'Address of the admin wallet in the generated node config',
      required: true,
    },
    'node-custom-runtime-config-path': {
      type: 'string',
      describe: 'Path to the custom node runtime config',
    },
    'alias-ip': {
      type: 'string',
      describe: 'IP address of the node to add as an alias',
      required: true,
    },
    'alias-port': {
      type: 'number',
      describe: 'Port of the node to add as an alias',
      required: true,
    },
  }).argv;

  return argv;
}

const MANIFEST_FILE_PATH =
  'scripts/generate_wallet_and_add_as_alias_manifest.json';
async function getInputsFromManifest(): Promise<Inputs> {
  // Read file and parse JSON.
  if (!fs.existsSync(MANIFEST_FILE_PATH)) {
    throw new Error(`Could not find manifest at ${MANIFEST_FILE_PATH}`);
  }

  const manifestJsonStr = fs.readFileSync(MANIFEST_FILE_PATH);
  const manifest: {
    deployedNodeContractsPath: string;
    existingStakerWalletPrivateKey: string;
    nodeConfigIpfsApiKey: string;
    nodeConfigAdminAddress: string;
    aliasIp: string;
    aliasPort: number;

    // Optional
    nodeCustomRuntimeConfigPath?: string;
  } = JSON.parse(manifestJsonStr.toString());

  return manifest;
}

async function run() {
  // Get inputs from file.
  const inputs = await getInputsFromManifest();

  // Parse the deployed contracts.
  console.info(
    'Parsing deployed node contracts from file: ' +
      inputs.deployedNodeContractsPath
  );
  let contractsJsonStr = fs.readFileSync(inputs.deployedNodeContractsPath);
  const contracts: ParsedNodeContracts = JSON.parse(
    contractsJsonStr.toString()
  );
  console.info('Parsed deployed node contracts', contracts);

  // Validate the input parameters.
  if (
    !ethers.isAddress(contracts.stakingContractAddress) ||
    !ethers.isAddress(contracts.stakingBalancesContractAddress) ||
    !ethers.isAddress(inputs.nodeConfigAdminAddress)
  ) {
    throw new Error('Invalid addresses');
  } else if (
    ethers.isHexString(inputs.existingStakerWalletPrivateKey) === false ||
    inputs.existingStakerWalletPrivateKey.length !== 66
  ) {
    throw new Error('Invalid existing wallet private key');
  }

  // Parse the existing wallet private key as a signer.
  const existingWallet = new ethers.Wallet(
    inputs.existingStakerWalletPrivateKey
  ).connect(ethers.provider);

  const [signerWithGas, ...signers] = await ethers.getSigners();
  console.info('Using signer to fund new wallet some gas', {
    signerWithGas: signerWithGas.address,
  });

  // Generate a new node operator credential (wallet)
  const newNodeOperatorCredential = generateWallets(ethers, 1)[0];

  // Fund the staker wallet with gas
  await fundWallet(
    signerWithGas,
    newNodeOperatorCredential.stakerWallet.address
  );

  // Also fund the node wallet with gas
  await fundWallet(signerWithGas, newNodeOperatorCredential.nodeWallet.address);

  // Add the new wallet as an alias.
  await addWalletAsAlias(
    newNodeOperatorCredential.stakerWallet,
    existingWallet,
    contracts
  );

  // Save config files
  const aliasNodeConfigDir = './alias_node_configs';
  await saveConfigFiles(
    [newNodeOperatorCredential],
    contracts,
    {
      adminAddress: inputs.nodeConfigAdminAddress,
      ipAddresses: [`${inputs.aliasIp}:${inputs.aliasPort}`],
      ipfsApiKey: inputs.nodeConfigIpfsApiKey,
    },
    {
      customNodeRuntimeConfigPath: inputs.nodeCustomRuntimeConfigPath,
      saveToDirectory: aliasNodeConfigDir,
      filePrefix: 'alias',
    }
  );

  // Save new alias wallet
  const date = new Date().getTime();
  const walletFilename = `./wallets/alias-wallets-${date}.json`;
  const serialized = serializeWallets([newNodeOperatorCredential]);
  fs.writeFileSync(walletFilename, JSON.stringify(serialized, null, 2));

  // Copy node config to rust project
  console.info('Copying node configs to rust project...');
  await copyDirFiles(aliasNodeConfigDir, '../../rust/lit-node/config');
}

async function fundWallet(
  walletToFundFrom: Signer,
  addressToFund: string
): Promise<void> {
  // Get balance of wallet before funding new wallet.
  console.info('Funding wallet', {
    walletToFundFromBalanceBefore: ethers.formatEther(
      await ethers.provider.getBalance(walletToFundFrom.getAddress())
    ),
    addressToFund,
  });

  const sendGasTx = await walletToFundFrom.sendTransaction({
    to: addressToFund,
    value: ethers.parseEther('1'),
  });
  const sendGasTxReceipt = await sendGasTx.wait();
  console.info('tx receipt for sending gas', { sendGasTxReceipt });
}

async function addWalletAsAlias(
  aliasWallet: Wallet,
  existingStakedWallet: Signer,
  contracts: ParsedNodeContracts
) {
  // Add the new wallet as an alias.
  console.info('Adding new wallet as an alias', {
    newWallet: aliasWallet.address,
    existingStakedWalletBalanceBefore: ethers.formatEther(
      await ethers.provider.getBalance(existingStakedWallet.getAddress())
    ),
  });
  const stakingBalances: StakingBalancesFacet = await ethers.getContractAt(
    'StakingBalancesFacet',
    contracts.stakingBalancesContractAddress,
    existingStakedWallet
  );

  const addAliasTx = await stakingBalances.addAlias(aliasWallet.address);
  const addAliasTxReceipt = await addAliasTx.wait();
  console.info('tx receipt for adding alias', { addAliasTxReceipt });
}

run();

interface Inputs {
  deployedNodeContractsPath: string;
  existingStakerWalletPrivateKey: string;
  nodeConfigAdminAddress: string;
  nodeConfigIpfsApiKey: string;
  aliasIp: string;
  aliasPort: number;
  nodeCustomRuntimeConfigPath?: string;
}
