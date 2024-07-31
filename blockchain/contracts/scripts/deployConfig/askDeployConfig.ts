import inquirer from 'inquirer';
import {
  askForConfirm,
  askForEnvConfirmOrInput,
  askForEnvConfirmOrListChoice,
} from './common';
import { askDeployCoreConfig } from './deployCoreConfig';
import { askDeployNodeConfig } from './deployNodeConfig';
import {
  DeployEnvironment,
  DeployFullConfig,
  DeploymentSelection,
} from './models';

export async function askDeployConfig(
  networkName: string
): Promise<DeployFullConfig> {
  const deploymentSelection = await askForDeploymentSelection();

  const environment = (await askForEnv()) as DeployEnvironment;

  const deployCoreConfig = await askDeployCoreConfig(environment, networkName);

  if (deploymentSelection === DeploymentSelection.LIT_CORE) {
    console.info('Deployment config', { deployCoreConfig });
    await askForDeployConfigConfirmation();
    return { deploymentSelection, deployCoreConfig };
  }

  const deployNodeConfig = await askDeployNodeConfig(
    deploymentSelection,
    environment,
    networkName
  );
  console.info('Deployment config', { deployCoreConfig, deployNodeConfig });
  await askForDeployConfigConfirmation();
  return { deploymentSelection, deployNodeConfig, deployCoreConfig };
}

async function askForEnv(): Promise<string> {
  return askForEnvConfirmOrListChoice(
    'LIT_ENV',
    `Are you sure you want to deploy with "${process.env.LIT_ENV}" as the environment?`,
    'environment',
    'What is the environment?',
    Object.values(DeployEnvironment)
  );
}

export async function askForDeploymentSelection(): Promise<DeploymentSelection> {
  const answer = await inquirer.prompt([
    {
      type: 'list',
      name: 'deploymentSelection',
      message: 'What would you like to deploy?',
      choices: Object.values(DeploymentSelection),
    },
  ]);

  return answer.deploymentSelection;
}

// Finally, ask the user to confirm the config used for deployment.
async function askForDeployConfigConfirmation(): Promise<void> {
  const confirmed = await askForConfirm(
    'deployConfig',
    `Would you like to deploy with the above configuration?`
  );
  if (!confirmed) {
    throw new Error('Deployment configuration rejected');
  }
  return;
}
