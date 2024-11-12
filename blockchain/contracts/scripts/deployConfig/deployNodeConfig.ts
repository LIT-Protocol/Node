import { isAddress } from 'ethers';
import inquirer from 'inquirer';
import { DEPLOY_LIT_NODE_OUTPUT_TEMP_FILE_PATH } from '.';
import { askForConfirm, askForEnvConfirmOrInput } from './common';
import {
  DeployEnvironment,
  DeploymentSelection,
  DeployNodeConfig,
  ParsedDomainWalletContracts,
  ParsedNodeContracts,
} from './models';

export async function askDeployNodeConfig(
  deploymentSelection: DeploymentSelection,
  environment: DeployEnvironment,
  networkName: string
): Promise<DeployNodeConfig> {
  let useLitCoreDeploymentResolverContractAddress = false;
  let resolverContractAddress = 'TBD';

  if (deploymentSelection === DeploymentSelection.LIT_CORE_AND_LIT_NODE) {
    useLitCoreDeploymentResolverContractAddress = true;
  } else {
    resolverContractAddress = await askForResolverContractAddress();
  }

  const copyNodeConfigsToRustProject =
    await askForCopyNodeConfigsToRustProject();

  const newOwnerAddress = await askForAdminAddress();
  const newDomainWalletAdminAddress = newOwnerAddress;
  const numberOfStakedOnlyWallets = await askForNumberOfStakedOnlyWallets();
  const numberOfStakedAndJoinedWallets =
    await askForNumberOfStakedAndJoinedWallets();

  const keyTypes = await askForKeyTypes();

  const ipAddresses = (
    await askForIpAddresses(
      numberOfStakedAndJoinedWallets + numberOfStakedOnlyWallets
    )
  )
    .split(',')
    .filter((a) => a.length > 0);

  let existingContracts = undefined;
  if (await askToKeepExistingContracts()) {
    existingContracts = await askForExistingContractAddresses();
  }

  let bpAddresses: string[] | undefined = await askForBackupRecoveryMembers();
  let bpKeys: string[] | undefined = await askForBackupRecoveryKeys();

  const deployConfig: DeployNodeConfig = {
    environment,
    networkName,
    newOwnerAddress,
    newDomainWalletAdminAddress,
    numberOfStakedOnlyWallets,
    numberOfStakedAndJoinedWallets,
    resolverContractAddress,
    useLitCoreDeploymentResolverContractAddress,
    outputTempFilePath: DEPLOY_LIT_NODE_OUTPUT_TEMP_FILE_PATH,
    copyNodeConfigsToRustProject,
    ipAddresses,
    existingContracts,
    keyTypes,
    backupRecoveryAddresses: bpAddresses,
    backupRecoveryKeys: bpKeys,
  };

  return deployConfig;
}

const SUPPORTED_KEY_TYPES = ['1', '2'];
async function askForKeyTypes(): Promise<number[]> {
  const promptResult = await inquirer.prompt([
    {
      type: 'input',
      name: 'keyTypes',
      message: `Enter the key types would you like to use, separated by commas. 1 - BLS, 2 - ECDSA`,
      validate: (input) => {
        try {
          const keyTypes = input.split(',');
          if (keyTypes.length === 0) {
            return `Please enter at least one key type`;
          }

          for (const keyType of keyTypes) {
            if (!SUPPORTED_KEY_TYPES.includes(keyType)) {
              return `Key type ${keyType} is not supported`;
            }
          }
        } catch (e) {
          return `Error parsing input: ${e}`;
        }
        return true;
      },
      default: SUPPORTED_KEY_TYPES.join(','),
    },
  ]);

  return promptResult.keyTypes
    .split(',')
    .map((keyType: string) => parseInt(keyType));
}

async function askForExistingContractAddresses(): Promise<ParsedNodeContracts> {
  const existingAddresses = await inquirer.prompt([
    {
      type: 'editor',
      name: 'existingContracts',
      message: `Enter the JSON for the any existing contracts.  Whatever contract addresses you provide will be preserved.`,
      validate: (input) => {
        try {
          const a = JSON.parse(input);
          if (a) {
            return true;
          }
          return false;
        } catch (e) {
          return e;
        }
      },
    },
  ]);
  return JSON.parse(existingAddresses.existingContracts);
}

async function askForExistingDomainWalletContractAddresses(): Promise<ParsedDomainWalletContracts> {
  const existingAddresses = await inquirer.prompt([
    {
      type: 'editor',
      name: 'existingRouterAndPkpContracts',
      message: `Enter the JSON for the existing Domain Wallet contracts`,
      validate: (input) => {
        try {
          const a = JSON.parse(input);
          if (a) {
            return true;
          }
          return false;
        } catch (e) {
          return e;
        }
      },
    },
  ]);
  return JSON.parse(existingAddresses.existingRouterAndPkpContracts);
}

async function askForCopyNodeConfigsToRustProject(): Promise<boolean> {
  return askForConfirm(
    'copyNodeConfigsToRustProject',
    `Would you like to copy the node configs to the rust project?`
  );
}

async function askToKeepExistingContracts(): Promise<boolean> {
  return askForConfirm(
    'keepRouterAndPkpContracts',
    `Would you like keep some existing contracts?`
  );
}

async function askToKeepExistingPkpHelperAndMetadataContracts(): Promise<boolean> {
  return askForConfirm(
    'keepPkpHelperAndMetadata',
    'Would you like to redeploy pkp helper and metadata contracts?'
  );
}

async function askForUsingLitCoreOutputForResolverContractAddress(): Promise<boolean> {
  try {
    await askForConfirm(
      'useLitCoreDeploymentResolverContractAddress',
      `Would you like to use the resolver contract address from the lit-core deployment for the lit-node deployment?`
    );
    return true;
  } catch (e) {
    // User answered no.
    return false;
  }
}

async function askForResolverContractAddress(): Promise<string> {
  return askForEnvConfirmOrInput(
    'LIT_RESOLVER_CONTRACT_ADDRESS',
    `resolver contract address`,
    `What is the resolver contract address?`,
    'resolverContractAddress',
    (input) => {
      if (!isAddress(input)) {
        return `${input} is not a valid address.`;
      }
      return true;
    }
  );
}

// Ask the user to confirm what the newOwner / admin will be.
async function askForAdminAddress(): Promise<string> {
  return askForEnvConfirmOrInput(
    'LIT_OWNER_WALLET_ID',
    `newOwner / admin wallet address`,
    `What is the newOwner / admin wallet address?`,
    'newOwnerAddress',
    (input) => {
      if (!isAddress(input)) {
        return `${input} is not a valid address.`;
      }
      return true;
    },
    undefined,
    true
  );
}

async function askForNumberOfStakedOnlyWallets(): Promise<number> {
  const number = await inquirer.prompt([
    {
      type: 'number',
      name: 'numberOfStakedOnlyWallets',
      message: `How many wallets would you like to stake (but not request to join the network)?`,
      default: 10,
      validate: (input) => input >= 0,
    },
  ]);

  return number.numberOfStakedOnlyWallets;
}

async function askForNumberOfStakedAndJoinedWallets(): Promise<number> {
  const number = await inquirer.prompt([
    {
      type: 'number',
      name: 'numberOfStakedAndJoinedWallets',
      message: `How many wallets would you like to stake and request to join the network?`,
      default: 10,
      validate: (input) => input >= 0,
    },
  ]);

  return number.numberOfStakedAndJoinedWallets;
}

async function askForIpAddresses(
  totalNumberOfStakedWallets: number
): Promise<string> {
  return askForEnvConfirmOrInput(
    'IP_ADDRESSES',
    `the node ip addresses`,
    `Enter all the IP addresses, separated by commas: `,
    'ipAddresses',
    (input) => {
      try {
        const ipAddresses = input.split(',').filter((a) => a.length > 0);
      } catch (e) {
        return `Error parsing input: ${e}`;
      }
      return true;
    },
    generateDefaultIpAddresses(totalNumberOfStakedWallets)
  );
}

async function askForBackupRecoveryMembers(): Promise<string[]> {
  const addresses = await inquirer.prompt([
    {
      type: 'input',
      name: 'backupPartyMembers',
      message: 'Enter backup party addresses (seperated by comma)',
      default: 'none',
      validate: (input) => {
        if (input === 'none') {
          return true;
        }
        let addresses = (input as string).split(',');
        if (addresses.length < 2) {
          return false;
        } else {
          return true;
        }
      },
    },
  ]);

  let bpAddrs = addresses.backupPartyMembers.split(',');
  if (bpAddrs.length > 1) {
    return bpAddrs;
  } else {
    return [];
  }
}

async function askForBackupRecoveryKeys(): Promise<string[]> {
  const keys = await inquirer.prompt([
    {
      type: 'input',
      name: 'backupPartyKeys',
      message: 'Enter encryption (backup party pub) keys, seperated by comma',
      default: 'none',
      validate: (input) => {
        if (input === 'none') {
          return true;
        }
        let keys = (input as string).split(',');
        if (keys.length == 2) {
          return true;
        } else {
          return false;
        }
      },
    },
  ]);

  let bpKeys = keys.backupPartyKeys.split(',');
  if (bpKeys.length == 2) {
    return bpKeys;
  } else {
    return [];
  }
}

function generateDefaultIpAddresses(num: number): string {
  let ipAddresses = '';
  for (let i = 0; i < num; i++) {
    ipAddresses += `127.0.0.1:${7470 + i},`;
  }

  // remove trailing comma
  ipAddresses = ipAddresses.slice(0, -1);

  return ipAddresses;
}
