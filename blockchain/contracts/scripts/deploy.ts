import { spawn } from "child_process";
import yargs from "yargs/yargs";
import { getDeployConfig, readDeployConfig } from "./deployConfig";

const SUPPORTED_NETWORKS = [
    "celo",
    "mumbai",
    "alfajores",
    "polygon",
    "litTestnet",
    "lit",
    "localchain",
];

const networkToRequiredEnvVars: { [key: string]: string[] } = {
    celo: ["LIT_CELOSCAN_API_KEY"],
    mumbai: ["LIT_POLYGONSCAN_API_KEY"],
    polygon: ["LIT_POLYGONSCAN_API_KEY"],
};

async function run() {
    console.log("Running deploy script");
    // Check if there is --deploy-config option passed.
    const argv = await yargs(process.argv.slice(2)).options({
        deployConfig: { type: "string" },
        network: {
            type: "string",
            choices: SUPPORTED_NETWORKS,
        },
        verbose: {
            type: "boolean",
            alias: "v",
            describe: "Run with verbose logging",
        },
    }).argv;

    // Get CLI parameters.
    const { deployFullConfigPath, networkName } =
        await getDeployCommandParameters(argv);

    // Validate environment variables.
    validateEnvVars(networkName);

    // Run deployWithConfig.ts as child_process with the deployConfigPath as an environment variable,
    // since hardhat doesn't support passing arguments to scripts.
    console.info("Deploying...");

    const deployWithHardhat = spawn(
        `npx hardhat run --network ${networkName} scripts/deployWithConfig.ts`,
        {
            stdio: "inherit",
            shell: true,
            env: {
                ...process.env,
                IS_VERBOSE: argv.verbose ? "true" : "false",
                DEPLOY_FULL_CONFIG_PATH: deployFullConfigPath,
            },
        }
    );

    // only exit once the child process has exited.
    try {
        const exitCode = await new Promise<number>((resolve, reject) => {
            deployWithHardhat.on("error", function (e) {
                reject(e);
            });
            deployWithHardhat.on("exit", function (code, signal) {
                resolve(code || 0);
            });
        });
        process.exit(exitCode);
    } catch (e) {
        console.error("Error deploying", e);
        process.exit(1);
    }
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
            throw new Error("Network must be specified");
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
