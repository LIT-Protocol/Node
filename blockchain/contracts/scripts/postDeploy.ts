import fs from "fs";
import {
    DeployNodeConfig,
    NodeOperatorCredentials,
    ParsedNodeContracts,
} from "./deployConfig";
import { copyDirFiles, saveConfigFiles, serializeWallets } from "./utils";

export async function postDeploy(
    deployNodeConfig: DeployNodeConfig,
    nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
    contracts: ParsedNodeContracts
) {
    const chainName = deployNodeConfig.networkName;
    const walletCount = deployNodeConfig.numberOfNodeWallets;

    // Make sure the directories exist and create them if not
    const dirs = ["./wallets", "./node_configs"];
    dirs.forEach((dir) => {
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir);
        }
    });

    // Generate env vars and conf files
    saveConfigFiles(
        nodeOperatorsCredentials,
        contracts,
        {
            chainPollingInterval: deployNodeConfig.chainPollingInterval,
            adminAddress: deployNodeConfig.newOwnerAddress,
            ipAddresses: deployNodeConfig.ipAddresses,
        },
        {
            customNodeRuntimeConfigPath:
                deployNodeConfig.customNodeRuntimeConfigPath,
        }
    );

    // Save wallets
    const date = new Date().getTime();
    const walletFilename = `./wallets/wallets-${date}-${chainName}-${walletCount}.json`;
    const serialized = serializeWallets(nodeOperatorsCredentials);
    fs.writeFileSync(walletFilename, JSON.stringify(serialized, null, 2));

    // Copy node configs to rust project if specified
    if (deployNodeConfig.copyNodeConfigsToRustProject) {
        console.info("Copying node configs to rust project...");
        await copyDirFiles("./node_configs", "../../rust/lit-node/config");
    }
    console.log("Done!");
    process.exit(0);
}
