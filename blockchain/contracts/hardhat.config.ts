import "@nomicfoundation/hardhat-toolbox";
// import "hardhat-ethernal"; // required for ethernal - removing this will only break deploy scripts that are linked to ethernal .
// Commenting out lines in these deploy scripts doesn't break the deploy, only the synchronization with Ethernal - ie, safe to do.
// Tests & regular deploys continue to work normally.
// Ethernal is a web based block explorer that syncs from any EVM chain - easy way to "view" hardhat data, and execute contracts!
// https://www.tryethernal.com

import "hardhat-contract-sizer";

import "@kingdomstudios/hardhat-diamond-abi";

// Uncomment the below to enable the tracer.
// FIXME: There is an unresolved issue whether running the npm run deploy script with
// the tracer enabled will prevent the child process from exiting.
// Issue: https://github.com/zemse/hardhat-tracer/issues/57
// import "hardhat-tracer";

import { HardhatUserConfig } from "hardhat/types";
// import "hardhat-tracer";

import { ProviderWrapper } from "hardhat/plugins";

// @ts-ignore
extendProvider(async (provider, config, network: string) => {
    if (network === "lit") {
        return new OldGethProvider(provider);
    } else if (
        network === "localchain" &&
        config.networks["localchain"].slowDownProvider
    ) {
        return new SlowerGethProvider(provider);
    }

    return provider;
});

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
// @ts-ignore
task("accounts", "Prints the list of accounts", async () => {
    // @ts-ignore
    const accounts = await ethers.getSigners();

    for (const account of accounts) {
        console.log(account.address);
    }
});

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

function envVarOrDefault(envVar: string, defaultValue: string): string {
    if (!process.env[envVar]) {
        return defaultValue;
    }
    return process.env[envVar] || "";
}

// NOTE, below we use the privkey key 0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166
// as a default.  it is empty.  DO NOT SEND MONEY TO IT.  it's public.

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
const config: HardhatUserConfig = {
    solidity: {
        version: "0.8.17",
        settings: {
            outputSelection: {
                "*": {
                    "*": ["storageLayout"],
                },
            },
            optimizer: {
                enabled: false,
                runs: 200,
            },
        },
    },
    networks: {
        celo: {
            url: "https://forno.celo.org",
            accounts: [
                envVarOrDefault(
                    "LIT_CELO_DEPLOYER_PRIVATE_KEY",
                    "0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166"
                ),
            ],
        },
        mumbai: {
            url: "https://polygon-mumbai.g.alchemy.com/v2/onvoLvV97DDoLkAmdi0Cj7sxvfglKqDh",
            accounts: [
                envVarOrDefault(
                    "LIT_MUMBAI_DEPLOYER_PRIVATE_KEY",
                    "0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166"
                ),
            ],
        },
        alfajores: {
            url: "https://alfajores-forno.celo-testnet.org",
            accounts: [
                envVarOrDefault(
                    "LIT_ALFAJORES_DEPLOYER_PRIVATE_KEY",
                    "0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166"
                ),
            ],
        },
        polygon: {
            url: "https://polygon-rpc.com",
            accounts: [
                envVarOrDefault(
                    "LIT_POLYGON_DEPLOYER_PRIVATE_KEY",
                    "0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166"
                ),
            ],
        },
        litTestnet: {
            url: "https://lit-test.calderachain.xyz/http",
            accounts: [
                envVarOrDefault(
                    "LIT_ROLLUP_TESTNET_DEPLOYER_PRIVATE_KEY",
                    "0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166"
                ),
            ],
            chainId: 987,
        },
        lit: {
            url: "https://lit-protocol.calderachain.xyz/http",
            accounts: [
                envVarOrDefault(
                    "LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY",
                    "0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166"
                ),
            ],
            chainId: 175177,
            // @ts-ignore
            wlitAddress: "0x53695556f8a1a064EdFf91767f15652BbfaFaD04",
        },
        localchain: {
            url: "http://127.0.0.1:8545",
            accounts: [
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", // default anvil private key,
                "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
                "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
                "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6",
                "0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a",
                "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba",
                "0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e",
                "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356",
                "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97",
                "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
            ],
            chainId: 31337,
            // @ts-ignore
            slowDownProvider:
                envVarOrDefault("LIT_SLOW_DOWN_PROVIDER", "false") === "true",
            // @ts-ignore
            wlitAddress: "0xA51c1fc2f0D1a1b8494Ed1FE312d7C3a78Ed91C0", // this gets deployed by deploy_everything.js but because it's deterministic we know it will end up at this address.  but only if you start anvil fresh for the deploy!
        },
        sepolia: {
            url: "https://sepolia.infura.io/v3/ddf1ca3700f34497bca2bf03607fde38",
            accounts: [
                envVarOrDefault(
                    "LIT_ROLLUP_TESTNET_DEPLOYER_PRIVATE_KEY",
                    "0x3178746f7ae6a309d14444b4c6c85a96a4be2f53fa8950dea241d232f3e6c166"
                ),
            ],
            chainId: 11155111,
        },
    },
    etherscan: {
        apiKey: {
            celo: process.env.LIT_CELOSCAN_API_KEY!,
            mumbai: process.env.LIT_POLYGONSCAN_API_KEY!,
            polygon: process.env.LIT_POLYGONSCAN_API_KEY!,
            litTestnet: "meow",
            lit: "woof",
            sepolia: process.env.LIT_ETHERSCAN_API_KEY!,
        },
        customChains: [
            {
                network: "celo",
                chainId: 42220,
                urls: {
                    apiURL: "https://api.celoscan.io/api",
                    browserURL: "https://celoscan.io",
                },
            },
            {
                network: "alfajores",
                chainId: 44787,
                urls: {
                    apiURL: "https://api-alfajores.celoscan.io/api",
                    browserURL: "https://alfajores.celoscan.io/",
                },
            },
            {
                network: "mumbai",
                chainId: 80001,
                urls: {
                    apiURL: "https://api-testnet.polygonscan.com/api",
                    browserURL: "https://mumbai.polygonscan.com",
                },
            },
            {
                network: "polygon",
                chainId: 137,
                urls: {
                    apiURL: "https://api.polygonscan.com/api",
                    browserURL: "https://polygonscan.com",
                },
            },
            {
                network: "litTestnet",
                chainId: 987,
                urls: {
                    apiURL: "https://lit-test.calderaexplorer.xyz/api",
                    browserURL: "https://lit-test.calderaexplorer.xyz",
                },
            },
            {
                network: "lit",
                chainId: 175177,
                urls: {
                    apiURL: "https://lit-protocol.calderaexplorer.xyz/api",
                    browserURL: "https://lit-protocol.calderaexplorer.xyz",
                },
            },
        ],
    },
    tenderly: {
        project: "litnodecontracts",
        username: "rwiggum",
        privateVerification: false,
    },
    diamondAbi: [
        {
            // (required) The name of your Diamond ABI.
            name: "StakingDiamond",
            // (optional) An array of strings, matched against fully qualified contract names, to
            // determine which contracts are included in your Diamond ABI.
            include: [
                "OwnershipFacet",
                "StakingFacet",
                "StakingViewsFacet",
                "StakingVersionFacet",
                "DiamondCutFacet",
                "DiamondLoupeFacet",
            ],
        },
        {
            // (required) The name of your Diamond ABI.
            name: "PKPPermissionsDiamond",
            // (optional) An array of strings, matched against fully qualified contract names, to
            // determine which contracts are included in your Diamond ABI.
            include: [
                "OwnershipFacet",
                "PKPPermissionsFacet",
                "DiamondCutFacet",
                "DiamondLoupeFacet",
            ],
        },
        {
            // (required) The name of your Diamond ABI.
            name: "PubkeyRouterDiamond",
            // (optional) An array of strings, matched against fully qualified contract names, to
            // determine which contracts are included in your Diamond ABI.
            include: [
                "OwnershipFacet",
                "PubkeyRouterFacet",
                "DiamondCutFacet",
                "DiamondLoupeFacet",
            ],
        },
        {
            // (required) The name of your Diamond ABI.
            name: "RateLimitNFTDiamond",
            // (optional) An array of strings, matched against fully qualified contract names, to
            // determine which contracts are included in your Diamond ABI.
            include: [
                "OwnershipFacet",
                "RateLimitNFTFacet",
                "RateLimitNFTViewsFacet",
                "DiamondCutFacet",
                "DiamondLoupeFacetNoERC165",
            ],
        },
        {
            // (required) The name of your Diamond ABI.
            name: "StakingBalancesDiamond",
            // (optional) An array of strings, matched against fully qualified contract names, to
            // determine which contracts are included in your Diamond ABI.
            include: [
                "OwnershipFacet",
                "StakingBalancesFacet",
                "DiamondCutFacet",
                "DiamondLoupeFacet",
            ],
        },
        {
            // (required) The name of your Diamond ABI.
            name: "PKPNFTDiamond",
            // (optional) An array of strings, matched against fully qualified contract names, to
            // determine which contracts are included in your Diamond ABI.
            include: [
                "OwnershipFacet",
                "PKPNFTFacet",
                "DiamondCutFacet",
                "DiamondLoupeFacetNoERC165",
            ],
        },
        {
            // (required) The name of your Diamond ABI.
            name: "BackupRecoveryDiamond",
            // (optional) An array of strings, matched against fully qualified contract names, to
            // determine which contracts are included in your Diamond ABI.
            include: [
                "OwnershipFacet",
                "BackupRecoveryFacet",
                "BackupRecoveryTestFacet",
                "DiamondCutFacet",
                "DiamondLoupeFacetNoERC165",
            ],
        },
    ],
    mocha: {
        timeout: 60000,
    },
};

export default config;

/**
 * This provider implements workarounds for old geth versions that do not support:
 * - An additional parameter in eth_estimateGas, so we strip it out.
 */
class OldGethProvider extends ProviderWrapper {
    constructor(protected readonly _wrappedProvider: any) {
        super(_wrappedProvider);

        console.info("Using OldGethProvider");
    }

    public async request(args: any) {
        if (args.method === "eth_estimateGas") {
            const adjustedRequest = {
                ...args,
                params: args.params.slice(0, args.params.length - 1),
            };
            return this._wrappedProvider.request(adjustedRequest);
        }

        return this._wrappedProvider.request(args);
    }
}

class SlowerGethProvider extends ProviderWrapper {
    constructor(protected readonly _wrappedProvider: any) {
        super(_wrappedProvider);

        console.info("Using SlowerGethProvider");
    }

    public async request(args: any) {
        await new Promise((resolve) => setTimeout(resolve, 50));
        return this._wrappedProvider.request(args);
    }
}
