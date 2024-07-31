import {
  DeployCoreConfig,
  DeployFullConfig,
  DeploymentSelection,
  DeployNodeConfig,
  DeploySensitiveConfig,
  readDeployConfig,
  readDeploySensitiveConfig,
} from './deployConfig';
import { deployLitCoreContracts } from './deployLitCore';
import { deployLitNodeContracts } from './deploy_lit_node_contracts';
import { fundAndStakeNodes } from './fundAndStakeNodes';
import { postDeploy } from './postDeploy';
import PQueue from 'p-queue';

/**
 *
 * NOTE: This script MUST be run with the following env variables set:
 * - DEPLOY_FULL_CONFIG_PATH
 *
 */
async function deployWithConfig() {
  const isVerbose = process.env.IS_VERBOSE === 'true';
  if (isVerbose) {
    console.info('Verbose mode enabled.');
    console.info('Network config', hre.network.config);
  }

  // Validate required env variable is set.
  const deployFullConfigPath = process.env.DEPLOY_FULL_CONFIG_PATH;
  if (
    typeof deployFullConfigPath !== 'string' ||
    deployFullConfigPath.length === 0
  ) {
    throw new Error('DEPLOY_FULL_CONFIG_PATH must be set');
  }

  // Read file and parse into DeployFullConfig.
  console.info(`Deploy config path specified: ${deployFullConfigPath}.`);
  const deployFullConfig = await readDeployConfig(deployFullConfigPath);
  const { deployCoreConfig, deployNodeConfig, deploymentSelection } =
    deployFullConfig;
  console.info(
    `Deploy config path ${deployFullConfigPath} parsed. Deploying...`,
    { deployFullConfig }
  );

  // Read sensitive config.
  const deploySensitiveConfig = await readDeploySensitiveConfig();

  // activate the verification queue
  startQueue();

  switch (deploymentSelection) {
    case DeploymentSelection.LIT_CORE:
      return _deployLitCoreContracts(deployCoreConfig);
    case DeploymentSelection.LIT_NODE:
      return _deployLitNodeContracts(deployNodeConfig!, deploySensitiveConfig);
    case DeploymentSelection.LIT_CORE_AND_LIT_NODE:
      return _deployLitCoreAndNodeContracts(
        deployCoreConfig,
        deployNodeConfig!,
        deploySensitiveConfig
      );
    default:
      throw new Error('Invalid deployment selection');
  }
}

async function _deployLitCoreContracts(
  deployCoreConfig: DeployCoreConfig
): Promise<any> {
  return deployLitCoreContracts(deployCoreConfig);
}

async function _deployLitNodeContracts(
  deployNodeConfig: DeployNodeConfig,
  deploySensitiveConfig: DeploySensitiveConfig
): Promise<any> {
  // Deploy lit-node contracts.
  await deployLitNodeContracts(deployNodeConfig);

  // Run the scripts/fund_and_stake_nodes.js script.
  const { nodeOperatorsCredentials, contracts } = await fundAndStakeNodes(
    deployNodeConfig
  );

  // Run the post-deploy script.
  return postDeploy(
    deployNodeConfig,
    deploySensitiveConfig,
    nodeOperatorsCredentials,
    contracts
  );
}

async function _deployLitCoreAndNodeContracts(
  deployCoreConfig: DeployCoreConfig,
  deployNodeConfig: DeployNodeConfig,
  deploySensitiveConfig: DeploySensitiveConfig
): Promise<any> {
  // Deploy lit-core contracts.
  const deployLitCoreOutput = await deployLitCoreContracts(deployCoreConfig);

  // Deploy lit-node contracts.
  if (deployNodeConfig.useLitCoreDeploymentResolverContractAddress) {
    // Use the output of the lit-core contracts to deploy the lit-node contracts.
    deployNodeConfig.resolverContractAddress =
      deployLitCoreOutput.contractResolver;
    await deployLitNodeContracts(deployNodeConfig);
  } else {
    await deployLitNodeContracts(deployNodeConfig);
  }

  // Run the scripts/fund_and_stake_nodes.js script.
  const { nodeOperatorsCredentials, contracts } = await fundAndStakeNodes(
    deployNodeConfig
  );

  // Run the post-deploy script.
  return postDeploy(
    deployNodeConfig,
    deploySensitiveConfig,
    nodeOperatorsCredentials,
    contracts
  );
}

// when deploying, we verify contracts in the background
// previously, we would just spawn verification in the background
// but this was exhausting my system memory causing a crash
// this new queue has a pool and will only allow N items to execute in parallel.
// this prevents memory exhaustion at the cost of the deploy script taking longer
function startQueue() {
  var queuedItems = 0;
  // @ts-ignore
  globalThis.queue = new PQueue({ concurrency: 10 });
  // @ts-ignore
  globalThis.queue.on('add', () => {
    console.log(`Added item ${queuedItems++} to verification queue`);
  });
  // @ts-ignore
  globalThis.queue.on('active', () => {
    console.log(`Started processing item in verification queue`);
  });
  // @ts-ignore
  globalThis.queue.on('error', (error) => {
    console.log(`Error in verification queue: ${error}`);
  });
  // @ts-ignore
  globalThis.queue.on('next', (result) => {
    console.log(`Completed item ${--queuedItems} from verification queue`);
  });
  // @ts-ignore
  globalThis.queue.on('idle', () => {
    console.log(`Queue is idle and all verification tasks have been completed`);
    process.exit(0);
  });
}

deployWithConfig();
