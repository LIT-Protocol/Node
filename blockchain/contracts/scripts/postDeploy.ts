import fs from 'fs';
import {
  DeployNodeConfig,
  DeploySensitiveConfig,
  NodeOperatorCredentials,
  ParsedNodeContracts,
} from './deployConfig';
import { copyDirFiles, saveConfigFiles, serializeWallets } from './utils';
import path from 'path';

export async function postDeploy(
  deployNodeConfig: DeployNodeConfig,
  deploySensitiveConfig: DeploySensitiveConfig,
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
  contracts: ParsedNodeContracts
) {
  const chainName = deployNodeConfig.networkName;
  const totalNodeWallets =
    deployNodeConfig.numberOfStakedOnlyWallets +
    deployNodeConfig.numberOfStakedAndJoinedWallets;

  // Make sure the directories exist and create them if not
  const dirs = ['./wallets', './node_configs'];
  dirs.forEach((dir) => {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir);
    }
  });

  // Generate env vars and conf files
  await saveConfigFiles(
    nodeOperatorsCredentials,
    contracts,
    {
      chainPollingInterval: deployNodeConfig.chainPollingInterval,
      adminAddress: deployNodeConfig.newOwnerAddress,
      ipAddresses: deployNodeConfig.ipAddresses,
      ipfsApiKey: deploySensitiveConfig.ipfsApiKey,
    },
    {
      customNodeRuntimeConfigPath: deployNodeConfig.customNodeRuntimeConfigPath,
    }
  );

  // Save wallets
  const date = new Date().getTime();
  const walletFilename = `./wallets/wallets-${date}-${chainName}-${totalNodeWallets}.json`;
  const serialized = serializeWallets(nodeOperatorsCredentials);
  fs.writeFileSync(walletFilename, JSON.stringify(serialized, null, 2));

  // Copy node configs to rust project if specified
  if (deployNodeConfig.copyNodeConfigsToRustProject) {
    console.info('Copying node configs to rust project...');
    await copyDirFiles('./node_configs', '../../rust/lit-node/config');
  }

  // get the path for the network context
  const networkContextPath = path.join(__dirname, '../networkContext.json');

  // log the path out if it exists
  if (fs.existsSync(networkContextPath)) {
    console.log('networkContext JSON exists at:', networkContextPath);

    // read the file as JSON
    const networkContext = JSON.parse(
      fs.readFileSync(networkContextPath, 'utf8')
    );
    console.log(
      'Contract resolver address:',
      networkContext['ContractResolver'].address
    );
  }

  console.log(
    'Done deploying.  Contract verification is continuing in the background, but you can start using the new contracts now.'
  );
}
