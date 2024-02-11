import {
    DeployCoreConfig,
    DeployFullConfig,
    DeploymentSelection,
    DeployNodeConfig,
    readDeployConfig,
} from "./deployConfig";
import { deployLitCoreContracts } from "./deployLitCore";
import { deployLitNodeContracts } from "./deploy_lit_node_contracts";
import { fundAndStakeNodes } from "./fundAndStakeNodes";
import { postDeploy } from "./postDeploy";

/**
 *
 * NOTE: This script MUST be run with the following env variables set:
 * - DEPLOY_FULL_CONFIG_PATH
 *
 */
async function deployWithConfig() {
    const isVerbose = process.env.IS_VERBOSE === "true";
    if (isVerbose) {
        console.info("Verbose mode enabled.");
        console.info("Network config", hre.network.config);
    }

    // Validate required env variable is set.
    const deployFullConfigPath = process.env.DEPLOY_FULL_CONFIG_PATH;
    if (
        typeof deployFullConfigPath !== "string" ||
        deployFullConfigPath.length === 0
    ) {
        throw new Error("DEPLOY_FULL_CONFIG_PATH must be set");
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

    switch (deploymentSelection) {
        case DeploymentSelection.LIT_CORE:
            return _deployLitCoreContracts(deployCoreConfig);
        case DeploymentSelection.LIT_NODE:
            return _deployLitNodeContracts(deployNodeConfig!);
        case DeploymentSelection.LIT_CORE_AND_LIT_NODE:
            return _deployLitCoreAndNodeContracts(
                deployCoreConfig,
                deployNodeConfig!
            );
        default:
            throw new Error("Invalid deployment selection");
    }
}

async function _deployLitCoreContracts(
    deployCoreConfig: DeployCoreConfig
): Promise<any> {
    return deployLitCoreContracts(deployCoreConfig);
}

async function _deployLitNodeContracts(
    deployNodeConfig: DeployNodeConfig
): Promise<any> {
    // Deploy lit-node contracts.
    await deployLitNodeContracts(deployNodeConfig);

    // Run the scripts/fund_and_stake_nodes.js script.
    const { nodeOperatorsCredentials, contracts } = await fundAndStakeNodes(
        deployNodeConfig
    );

    // Run the post-deploy script.
    return postDeploy(deployNodeConfig, nodeOperatorsCredentials, contracts);
}

async function _deployLitCoreAndNodeContracts(
    deployCoreConfig: DeployCoreConfig,
    deployNodeConfig: DeployNodeConfig
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
    return postDeploy(deployNodeConfig, nodeOperatorsCredentials, contracts);
}

deployWithConfig();
