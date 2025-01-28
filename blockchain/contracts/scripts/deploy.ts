import { spawn } from 'child_process';
import yargs from 'yargs/yargs';
import { getDeployConfig, readDeployConfig } from './deployConfig';
import { waitForProcessToExit } from './utils';

const SUPPORTED_NETWORKS = [
  'celo',
  'mumbai',
  'alfajores',
  'polygon',
  'litTestnet',
  'lit',
  'vesuvius',
  'localchain',
  'etna',
  'yellowstone',
];

const networkToRequiredEnvVars: { [key: string]: string[] } = {
  celo: ['LIT_CELOSCAN_API_KEY,LIT_CELO_DEPLOYER_PRIVATE_KEY'],
  mumbai: ['LIT_POLYGONSCAN_API_KEY,LIT_MUMBAI_DEPLOYER_PRIVATE_KEY'],
  alfajores: ['LIT_ALFAJORES_DEPLOYER_PRIVATE_KEY'],
  polygon: ['LIT_POLYGONSCAN_API_KEY,LIT_POLYGON_DEPLOYER_PRIVATE_KEY'],
  litTestnet: ['LIT_ROLLUP_TESTNET_DEPLOYER_PRIVATE_KEY'],
  lit: ['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY'],
  etna: ['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY'],
  vesuvius: ['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY'],
  yellowstone: ['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY'],
};

async function run() {
  console.info('Loading environment variables');
  require('dotenv').config();

  console.log('Running deploy script');
  // Check if there is --deploy-config option passed.
  const argv = await yargs(process.argv.slice(2)).options({
    deployConfig: { type: 'string' },
    network: {
      type: 'string',
      choices: SUPPORTED_NETWORKS,
    },
    verbose: {
      type: 'boolean',
      alias: 'v',
      describe: 'Run with verbose logging',
    },
  }).argv;

  // Get CLI parameters.
  const { deployFullConfigPath, networkName } =
    await getDeployCommandParameters(argv);

  // Validate environment variables.
  validateEnvVars(networkName);

  // Run deployWithConfig.ts as child_process with the deployConfigPath as an environment variable,
  // since hardhat doesn't support passing arguments to scripts.
  console.info('Deploying...');

  const deployWithHardhat = spawn(
    `npx hardhat run --network ${networkName} scripts/deployWithConfig.ts`,
    {
      stdio: 'inherit',
      shell: true,
      env: {
        ...process.env,
        IS_VERBOSE: argv.verbose ? 'true' : 'false',
        DEPLOY_FULL_CONFIG_PATH: deployFullConfigPath,
      },
    }
  );

  // only exit once the child process has exited.
  console.log('Waiting for deployWithHardhat to exit');
  const exitCode = await waitForProcessToExit(deployWithHardhat);
  console.log('deployWithHardhat exited with code', exitCode);
  process.exit(exitCode);
}

async function getDeployCommandParameters(argv: {
  deployConfig?: string;
  network?: string;
}): Promise<{
  deployFullConfigPath: string;
  networkName: string;
}> {
  const isDeployConfigSpecified = !!argv.deployConfig;

  if (!isDeployConfigSpecified) {
    // Check if network arg is specified.
    if (!argv.network) {
      throw new Error('Network must be specified');
    }

    const networkName = argv.network;

    // Note that this is an INTERACTIVE function - it will ask the user for input.
    const res = await getDeployConfig(networkName);

    return {
      deployFullConfigPath: res.deployFullConfigPath,
      networkName,
    };
  }

  // --deploy-config option passed, so pass to deployWithConfig.ts.
  const deployFullConfigPath = argv.deployConfig!;

  // We read the deploy config file to retrieve the network name.
  const deployFullConfig = await readDeployConfig(deployFullConfigPath);

  return {
    deployFullConfigPath,
    networkName: deployFullConfig.deployCoreConfig.networkName,
  };
}

function validateEnvVars(network: string): void {
  const requiredEnvVars = networkToRequiredEnvVars[network]
    ? networkToRequiredEnvVars[network]
    : [];
  for (const envVar of requiredEnvVars) {
    if (!process.env[envVar]) {
      throw new Error(`Missing environment variable: ${envVar}`);
    }
  }
}

run().catch((err) => {
  console.error(err.toString());
  process.exit(1);
});
