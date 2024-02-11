These contracts govern the Lit Nodes and various PKP things. Currently in testnet only.

Learn more here: https://developer.litprotocol.com/docs/litactionsandpkps/whatarelitactionsandpkps/

# TODO

- Tests for the token reward in the staking contracts (Staking.sol and Staking.js)
- Make it so that the nodes can't accidently kick eachother to below the threshold. Limit the number of nodes that can be kicked per epoch? Have the ability to rejoin if kicked and recovered?

# Running Tests

In order to run the contract tests, you will need to follow these instrucitons:

1. Start an instance of our forked Anvil locally. The forked Anvil contains a precompile for the key derivation function and can be found here: https://github.com/LIT-Protocol/foundry
2. Run `npm run test`

# How to verify contracts

```shell
npx hardhat verify --network celo 0x5Ef8A5e3b74DE013d608740F934c14109ae12a81 \
  "0x0008a7B1Ce657E78b4eDC6FC40078ce8bf08329A"
```

The second param is any constructor params.

# Deploying

1. Run `npm install` to install project dependencies.
2. Run `npm run test` to test the smart contracts.
3. Export the private key to the environment depending on your deployment target - refer to `hardhat.config.ts` for more details. For example, if deploying to Polygon Mumbia, export `LIT_MUMBAI_DEPLOYER_PRIVATE_KEY=<YOUR-PRIVATE-KEY>`. If deploying to LIT Rollup (Chronicle), export `LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY=<YOUR-PRIVATE-KEY>`.
4. `npm run deploy -- --network <NETWORK>`

- If you know exactly which deployment full config file you would like to use, you can do `npm run deploy -- --deploy-config <DEPLOY_FULL_CONFIG_PATH>`. The `--network` option is not needed here as the deploy config file contains that parameter.

**Note**: The wallet you provide should have at least 10 LIT for the gas to complete the entire deployment process which includes funding & staking the nodes which is called internally in the deploy script. If you don't have that much LIT you may ask Chris on Slack for it. You could also get some from the Chronicle Faucet but it only gives out a tiny amount so you would have to modify the deploy scripts to fund each of the node wallets with less tokens.

- [Chronicle Faucet](https://faucet.litprotocol.com/)

**Note**: The deploy script will set the ownership of each contract to the `newOwner` address defined in scripts/deploy_lit_node_contracts.js. If you need to call owner / admin functions on the contracts after they're deployed, you can set that `newOwner` address to something you control. If you're just using the contracts with the nodes you probably don't need to do this.

Once this script is done running, if you answered "y" to "Should we copy the node config files into the node folder", there will be config files for you generated in /node_configs of this repo. You can copy these to the /config folder of the lit_node_rust repo.

## Local Deployment

These are the instructions for deploying + the smart contracts locally:

1. Run a local blockchain / testnet using [Hardhat](https://hardhat.org/hardhat-network/docs/overview#running-stand-alone-in-order-to-support-wallets-and-other-software), [Anvil](https://book.getfoundry.sh/anvil/) or other software. It must be listening on port `8545` on `localhost`.
2. `npm run deploy -- --network localchain` and follow the interactive prompts.
3. Select `dev` for the environment.
4. Specify a wallet address that you own / have access to when specifying the `newOwnerAddress`.
5. Accept to copy the node configs to the Rust project.
6. Choose anywhere from 3 to 10 for the number of node wallets.
7. Use the default IP addresses as suggested.

Here is an example deployment configuration:

```json
{
  "deploymentSelection": "lit-core + lit-node",
  "deployNodeConfig": {
    "environment": "dev",
    "networkName": "localchain",
    "newOwnerAddress": "0x4259E44670053491E7b4FE4A120C70be1eAD646b",
    "numberOfNodeWallets": 3,
    "resolverContractAddress": "TBD",
    "useLitCoreDeploymentResolverContractAddress": true,
    "outputTempFilePath": "./deployed-lit-node-contracts-temp.json",
    "copyNodeConfigsToRustProject": true,
    "ipAddresses": ["127.0.0.1:7470", "127.0.0.1:7471", "127.0.0.1:7472"]
  },
  "deployCoreConfig": {
    "environment": "dev",
    "networkName": "localchain",
    "subnetOwnerAddress": "0xB77AEBbC262Bb809933D991A919A0e4A6A3b2f65",
    "subnetAdminPublicKey": "0x045f96e860435fccf287d9c2592fa129edfca7159c8dd2260cf2def38a9d5ee627ba73afef636467bc95fe551f10c862e910f18eafb751226d6901eab7d5b2794a",
    "subnetProvAddress": "0x3324439C8b9181eF07D54030E32d2CD22FF0C6A7",
    "outputTempFilePath": "./deployed-lit-core-contracts-temp.json"
  }
}
```

# Contract Deployment Tooling

We have developed a tool that makes it convenient to deploy and configure our suite of smart contracts before spinning up a network of nodes against them. Specifically, our tool helps with:

- Deploying Lit Core and/or Lit Node smart contracts to any supported chain
- After deploying smart contracts, configure the smart contract parameters and settings per each of the node operators

## Technical Details

- The tool is available as a `npm` script - `npm run deploy -- --network <NETWORK>`
  - The currently supported network names are:
    - `celo`
    - `mumbai`
    - `alfajores`
    - `polygon`
    - `litTestnet`
    - `lit`
    - `localchain`
- At a high-level, the tool consists of 2 main steps:
  1. An **interactive** step that determines the entire set of deployment configurations that will be used.
  2. A **non-interactive** step that takes a deployment configuration and deploys and configures a set of smart contracts accodingly.
- Running the entire tool as it is will involve an interactive experience (eg. command-line experience). If you wish to have a non-interactive experience, that is only available by running the tool and specifying exactly which deployment configuration you would like to use, ie. `npm run deploy -- --deploy-config <DEPLOY_FULL_CONFIG_PATH>`. The `--network` option is not needed here as the deploy config file contains that parameter.
- The non-interactive deployment step is run as a child process that is spawned. All environment variables are inherited in the spawned environment. We pass in additional environment variables.
- When running the interactive first step,
  - you will have the option to choose whether to deploy:
    1. Only the Lit Core contracts
    2. Only the Lit Node contracts
    3. Both the Lit Core and Lit Node contracts, and in that order.
  - you will have the option to choose a previously generated deployment configuration.
    - **This is the true power of this tool, in allowing you to reference EXACTLY a deployment configuration that has worked well for your needs previously**.
- This tool is environment-aware and will ask for confirmation before proceeding to use parameters specified in your shell session. Refer to the Deployment Configuration Reference section below for more details.
- This tool uses [`inquirer.js`](https://github.com/SBoudrias/Inquirer.js) to create the interactive experience.

## Deployment Configuration

The deployment configuration refers to the entire set of parameters that will be used for deploying and configuring the smart contracts.

### Persistence

- Each deployment configuration is persisted to disk locally under the `scripts/deployConfig/configs` directory.
- In order for a persisted deployment configuration to be detected by the tool, it must match the following pattern `deploy-config-*.json`

### Reference

Here is an explanation for each of the fields in the deployment configuration:

| Key Path                                                       | Type       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| -------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `deploymentSelection`                                          | `string`   | An enum of either `lit-core`, `lit-core + lit-node` or `lit-node` describing which set of smart contracts should be deployed.                                                                                                                                                                                                                                                                                                                                                                  |
| `deployNodeConfig`                                             | `object`   | The deployment configuration parameters that relate to deploying the Lit Node smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                 |
| `deployNodeConfig.environment`                                 | `string`   | An enum of either `dev`, `staging` or `prod` describing which deployment environment the Lit Node contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                             |
| `deployNodeConfig.networkName`                                 | `string`   | The name of the network (chain) the Lit Node contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `deployNodeConfig.newOwnerAddress`                             | `string`   | The EVM-compatible address that will be given ownership and configuration permissions once the tool finishes. While the tool uses the deployer address to configure the smart contract parameters after deployment, you would most likely want to revoke admin / owner permissions from the deployer and grant your own address such permissions after the tool finishes. If `LIT_OWNER_WALLET_ID` is set in your environment, the tool will ask for confirmation before using this parameter. |
| `deployNodeConfig.numberOfNodeWallets`                         | `number`   | The number of nodes (and node operators) to configure the network for.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `deployNodeConfig.resolverContractAddress`                     | `string`   | The Lit Core `ContractResolver` contract address that will be referenced. It will be marked as `TBD` when `deployNodeConfig.useLitCoreDeploymentResolverContractAddress` is set to `true`, since we won't know the smart contract address until the non-interactive deployment step of the tool. If `LIT_RESOLVER_CONTRACT_ADDRESS` is set in your environment, the tool will ask for confirmation before using this parameter.                                                                |
| `deployNodeConfig.useLitCoreDeploymentResolverContractAddress` | `boolean`  | Whether to use the `ContractResolver` contract address from deploying the Lit Core contracts.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `deployNodeConfig.outputTempFilePath`                          | `string`   | The path to the file containing the addresses of the deployed Lit Node smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                        |
| `deployNodeConfig.copyNodeConfigsToRustProject`                | `boolean`  | Whether to copy the generated node configs over to the Rust project. You will likely need this when spinning up a network locally on your machine.                                                                                                                                                                                                                                                                                                                                             |
| `deployNodeConfig.ipAddresses`                                 | `string[]` | An array of strings representing the IP addresses of the node operators. You will likely need this when spinning up a network locally on your machine. If `IP_ADDRESSES` is set in your environment, the tool will ask for confirmation before using this parameter.                                                                                                                                                                                                                           |
| `deployNodeConfig.existingRouterAndPkpContracts`               | `object`   | An object containing the addresses of smart contracts from a prior deployment to be referenced again in this current deployment.                                                                                                                                                                                                                                                                                                                                                               |
| `deployCoreConfig`                                             | `object`   | The deployment configuration parameters that relate to deploying the Lit Core smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                 |
| `deployCoreConfig.environment`                                 | `string`   | An enum of either `dev`, `staging` or `prod` describing which deployment environment the Lit Core contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                             |
| `deployCoreConfig.networkName`                                 | `string`   | The name of the network (chain) the Lit Core contracts should be deployed to.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `deployCoreConfig.subnetOwnerAddress`                          | `string`   | The address of the subnet owner.                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `deployCoreConfig.subnetAdminPublicKey`                        | `string`   | The public key of the subnet admin.                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `deployCoreConfig.subnetProvAddress`                           | `string`   | The address of the wallet that provisions the subnet.                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `deployCoreConfig.outputTempFilePath`                          | `string`   | The path to the file containing the addresses of the deployed Lit Core smart contracts.                                                                                                                                                                                                                                                                                                                                                                                                        |

# Deployed Contract Addresses

Deployed contract addresses are listed by network in this repo: https://github.com/LIT-Protocol/networks
