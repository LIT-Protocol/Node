import { glob } from 'glob';
import inquirer from 'inquirer';
import {
  askDeployConfig,
  DEPLOY_CONFIGS_DIR,
  DEPLOY_CONFIGURATION_PATTERN,
  readDeployConfig,
  saveDeployConfig,
} from '.';
import { checkPathExists } from '../utils';
import { DeployFullConfig } from './models';

/**
 *
 * Note that this function is an INTERACTIVE function that will ask the user for input.
 *
 */
export async function getDeployConfig(networkName: string): Promise<{
  deployFullConfig: DeployFullConfig;
  deployFullConfigPath: string;
}> {
  // Check if the deploy configs dir exists.
  const deployConfigsDirExists = await checkPathExists(DEPLOY_CONFIGS_DIR);
  if (!deployConfigsDirExists) {
    console.debug('No deploy configs exist.');
    const deployFullConfig = await askDeployConfig(networkName);
    const deployFullConfigPath = await saveDeployConfig(deployFullConfig);
    validateDeployConfig(deployFullConfig); // throws on error
    return { deployFullConfig, deployFullConfigPath };
  }

  // List all files in the configs directory using fs.
  const deploymentConfigurations = await glob(
    `${DEPLOY_CONFIGS_DIR}/${DEPLOY_CONFIGURATION_PATTERN}`
  );
  if (deploymentConfigurations.length === 0) {
    console.debug('No deploy configs exist.');
    const deployFullConfig = await askDeployConfig(networkName);
    const deployFullConfigPath = await saveDeployConfig(deployFullConfig);
    validateDeployConfig(deployFullConfig); // throws on error
    return { deployFullConfig, deployFullConfigPath };
  }

  // Deploy configurations exist, ask the user to either select one to deploy or create a new one.
  const newOrExistingDeploymentConfig =
    await askForNewOrExistingDeploymentConfig(deploymentConfigurations);
  if (newOrExistingDeploymentConfig.createNewDeploymentConfig) {
    console.debug('Creating new deployment config.');
    const deployFullConfig = await askDeployConfig(networkName);
    const deployFullConfigPath = await saveDeployConfig(deployFullConfig);
    validateDeployConfig(deployFullConfig); // throws on error
    return { deployFullConfig, deployFullConfigPath };
  }

  // Use the existing deployment config.
  const deployConfigPath =
    newOrExistingDeploymentConfig.deploymentConfigurationPath!;
  console.info(`Using existing deployment config: ${deployConfigPath}.`);

  const deployFullConfig = await readDeployConfig(deployConfigPath);

  console.info(`Deploy config path ${deployConfigPath} parsed. Deploying...`, {
    deployFullConfig,
  });
  validateDeployConfig(deployFullConfig); // throws on error
  return { deployFullConfig, deployFullConfigPath: deployConfigPath };
}

async function askForNewOrExistingDeploymentConfig(
  existingDeploymentConfigurationPaths: string[]
): Promise<{
  createNewDeploymentConfig: boolean;
  deploymentConfigurationPath?: string;
}> {
  const CREATE_NEW_OPTION = 'Create a new deployment config';

  const answer = await inquirer.prompt([
    {
      type: 'list',
      name: 'newOrExistingDeploymentConfig',
      message:
        'Existing deployment configurations detected. Would you like to use one of these or create a new one?',
      choices: [CREATE_NEW_OPTION, ...existingDeploymentConfigurationPaths],
    },
  ]);

  return {
    createNewDeploymentConfig:
      answer.newOrExistingDeploymentConfig === CREATE_NEW_OPTION,
    deploymentConfigurationPath:
      answer.newOrExistingDeploymentConfig === CREATE_NEW_OPTION
        ? undefined
        : answer.newOrExistingDeploymentConfig,
  };
}

// Early validation of the DeployConfig before the transactions are sent
function validateDeployConfig(deployFullConfig: DeployFullConfig) {
  if (
    (deployFullConfig.deployNodeConfig?.ipAddresses?.length ?? 0) !==
    (deployFullConfig.deployNodeConfig?.numberOfStakedAndJoinedWallets ?? 0) +
      (deployFullConfig.deployNodeConfig?.numberOfStakedOnlyWallets ?? 0)
  ) {
    // Not enough IP addresses were provided for all network participants
    console.error(
      'Number of provided IP addresses (%d) does not match number of total network participants (joined:%d + staked:%d)',
      deployFullConfig.deployNodeConfig?.ipAddresses?.length ?? 0,
      deployFullConfig.deployNodeConfig?.numberOfStakedAndJoinedWallets ?? 0,
      deployFullConfig.deployNodeConfig?.numberOfStakedOnlyWallets ?? 0
    );
    throw new Error('Mismatching number of IP Addresses in config');
  }
}
