import { mkdir, readFile, writeFile } from 'fs/promises';
import inquirer from 'inquirer';
import { DEPLOY_CONFIGS_DIR } from '.';
import { checkPathExists, tryParseJson } from '../utils';
import { DeployFullConfig, DeploySensitiveConfig } from './models';

// This function is used to return a function that can be used to ask the user for confirmation or input.
// A confirmation is presented if the provided environment variable is set.
// Otherwise, the user is asked to input the value.
export async function askForEnvConfirmOrInput(
  envVarToDetect: string,
  valueUsedFor: string,
  inputMessage: string,
  answerKeyPrefix: string,
  inputValidateFn?: (input: string) => boolean | string,
  inputDefault?: string,
  defaultConfirmChoice: boolean = false
): Promise<string> {
  const envVarIsSet =
    !!process.env[envVarToDetect] && process.env[envVarToDetect] !== '';

  if (envVarIsSet) {
    return askForConfirmParameter(
      answerKeyPrefix,
      process.env[envVarToDetect]!,
      valueUsedFor,
      defaultConfirmChoice
    );
  } else {
    const answer = await inquirer.prompt([
      {
        type: 'input',
        name: answerKeyPrefix,
        message: inputMessage,
        validate: inputValidateFn,
        default: inputDefault,
      },
    ]);

    return answer[answerKeyPrefix];
  }
}

export async function askForEnvConfirmOrListChoice(
  envVarToDetect: string,
  valueUsedFor: string,
  answerKeyPrefix: string,
  listChoiceMessage: string,
  listChoices: string[]
) {
  const envVarIsSet =
    !!process.env[envVarToDetect] && process.env[envVarToDetect] !== '';

  if (envVarIsSet) {
    return askForConfirmParameter(
      answerKeyPrefix,
      process.env[envVarToDetect]!,
      valueUsedFor
    );
  } else {
    const answer = await inquirer.prompt([
      {
        type: 'list',
        name: answerKeyPrefix,
        message: listChoiceMessage,
        choices: listChoices,
      },
    ]);

    return answer[answerKeyPrefix];
  }
}

export async function askForConfirmParameter(
  answerKeyPrefix: string,
  valueToConfirm: string,
  valueUsedFor: string,
  defaultConfirmChoice: boolean = false
): Promise<string> {
  const answerKey = `${answerKeyPrefix}Confirmation`;

  const answer = await inquirer.prompt([
    {
      type: 'confirm',
      name: answerKey,
      message: `Are you sure you want to use ${valueToConfirm} for ${valueUsedFor}?`,
      default: defaultConfirmChoice,
    },
  ]);

  if (!answer[answerKey]) {
    const errMsg = `User failed to confirm.`;
    console.error(errMsg);
    throw new Error(errMsg);
  }

  return valueToConfirm;
}

export async function askForConfirm(
  answerKeyPrefix: string,
  confirmationMessage: string,
  defaultConfirmChoice: boolean = false
): Promise<boolean> {
  const answerKey = `${answerKeyPrefix}Confirmation`;

  const answer = await inquirer.prompt([
    {
      type: 'confirm',
      name: answerKey,
      message: confirmationMessage,
      default: defaultConfirmChoice,
    },
  ]);

  if (typeof answer[answerKey] === 'string') {
    const errMsg = `User failed to confirm.`;
    console.error(errMsg);
    throw new Error(errMsg);
  }

  return answer[answerKey];
}

// Ask the user to confirm the network they are deploying to.
export async function askForNetworkNameConfirmation(
  networkName: string
): Promise<string> {
  return askForConfirmParameter(
    'networkName',
    networkName,
    'the network you are deploying to',
    true
  );
}

export async function readDeployConfig(
  deployConfigPath: string
): Promise<DeployFullConfig> {
  console.log('Trying to read deploy config from', deployConfigPath);
  const deployConfigExists = await checkPathExists(deployConfigPath);
  if (!deployConfigExists) {
    throw new Error(`Deploy config path ${deployConfigPath} does not exist.`);
  }

  // Read JSON file using fs/promises.
  const deployConfigStr = await readFile(deployConfigPath, 'utf8');

  // Validate that we can parse into a deployment configuration object.
  const parseRes = tryParseJson<DeployFullConfig>(deployConfigStr);
  if (!!parseRes[1]) {
    throw new Error(
      `Deploy config path ${deployConfigPath} is not a valid deploy config.`
    );
  }

  // Fill missing values
  const deployFullConfig = parseRes[0]!;
  fillMissingValues(deployFullConfig);

  return deployFullConfig;
}

function fillMissingValues(deployFullConfig: DeployFullConfig) {
  if (!deployFullConfig.deployNodeConfig!.numberOfStakedOnlyWallets) {
    deployFullConfig.deployNodeConfig!.numberOfStakedOnlyWallets = 0;
  }
}

// Save this to disk in the configs directory with a timestamp.
// The file format is deploy-config-<datetime>.json where timestamp is YYYY-MM-DD HH:mm:ss.
export async function saveDeployConfig(
  deployFullConfig: DeployFullConfig
): Promise<string> {
  // Create the directory if it doesn't exist.
  if (!(await checkPathExists(DEPLOY_CONFIGS_DIR))) {
    await mkdir(DEPLOY_CONFIGS_DIR);
  }

  const filePath = `${DEPLOY_CONFIGS_DIR}/deploy-config-${new Date().toISOString()}.json`;
  await writeFile(filePath, JSON.stringify(deployFullConfig, null, 4));
  return filePath;
}

const REQUIRED_SENSITIVE_CONFIG_KEYS = ['IPFS_API_KEY'];

export async function readDeploySensitiveConfig(): Promise<DeploySensitiveConfig> {
  console.info('Trying to read deploy sensitive config from environment');

  for (const key of REQUIRED_SENSITIVE_CONFIG_KEYS) {
    if (!process.env[key]) {
      throw new Error(
        `Missing required sensitive config key ${key} in environment.`
      );
    }
  }

  return {
    ipfsApiKey: process.env.IPFS_API_KEY!,
  };
}
