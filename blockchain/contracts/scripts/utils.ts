import TOML from "@iarna/toml";
import { spawn } from "child_process";
import { Wallet } from "ethers";
import fs from "fs";
import { access, copyFile, readdir } from "fs/promises";
import path from "path";
import nacl from "tweetnacl";
import {
    CONTRACT_NAME_TO_DIAMOND_ABI_PATH,
    CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY,
} from "./constants";
import {
    ComsKeys,
    DeployEnvironment,
    NodeOperatorCredentials,
    ParsedNodeContracts,
} from "./deployConfig";

export const mapEnvToEnum = (env: DeployEnvironment) => {
    switch (env) {
        case DeployEnvironment.DEV:
            return 0;
        case DeployEnvironment.STAGING:
            return 1;
        case DeployEnvironment.PROD:
            return 2;
        default:
            throw new Error("ENV " + env + " is invalid");
    }
};

export function tryParseJson<T>(jsonStr: string): [T | null, Error | null] {
    try {
        const parsedObject: T = JSON.parse(jsonStr);
        return [parsedObject, null];
    } catch (e) {
        const parseErr = new Error(`Unable to parse JSON: ${e}`);
        return [null, parseErr];
    }
}

export async function checkPathExists(path: string): Promise<boolean> {
    try {
        await access(path);
        return true;
    } catch (e) {
        return false;
    }
}

export function contractAddressAlreadyExists(
    existingContracts: ParsedNodeContracts,
    contractName: string
) {
    const contractAddressKey =
        // @ts-ignore
        CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY[contractName];
    // @ts-ignore
    return existingContracts && existingContracts[contractAddressKey];
}

export async function getContractInstance(
    ethers: any,
    contractName: string,
    contractAddress: string,
    signer: any,
    isDiamondContract: boolean = false
) {
    if (!isDiamondContract) {
        return ethers.getContractAt(contractName, contractAddress, signer);
    }

    // use combined diamond ABI for contract
    const abiPath = path.resolve(
        __dirname,
        // @ts-ignore
        CONTRACT_NAME_TO_DIAMOND_ABI_PATH[contractName]
    );
    const abi = JSON.parse(fs.readFileSync(abiPath, "utf8")).abi;
    return new ethers.Contract(contractAddress, abi, signer);
}

/**
 * Remove all undefined values from an object, recursively.
 */
export const removeEmpty = (obj: any) => {
    let newObj = {};
    Object.keys(obj).forEach((key) => {
        // @ts-ignore
        if (obj[key] === Object(obj[key])) newObj[key] = removeEmpty(obj[key]);
        // @ts-ignore
        else if (obj[key] !== undefined) newObj[key] = obj[key];
    });
    return newObj;
};

/**
 * Convert all values to strings, recursively.
 *
 * Exceptions:
 * - booleans  <<< NO EXCEPTIONS.  Toml doesn't support booleans in the node.  We either fix in the node, or convert to strings.
 */
export const stringifyAllValues = (obj: any) => {
    let newObj = {};
    Object.keys(obj).forEach((key) => {
        if (obj[key] === Object(obj[key]))
            // @ts-ignore
            newObj[key] = stringifyAllValues(obj[key]);
        // @ts-ignore
        // else if (typeof obj[key] === "boolean") {
        //     // @ts-ignore
        //     newObj[key] = obj[key];
        // }
        // @ts-ignore
        else newObj[key] = obj[key].toString();
    });
    return newObj;
};

export function verifyContractInBg(
    chainName: string,
    address: string,
    args = []
) {
    // write arguments to file
    const argsFileName = "/tmp/args.js";
    const template = `module.exports = ${jsonStringify(args)}`;
    fs.writeFileSync(argsFileName, template);
    let verify = spawn(
        "bash",
        [
            "./scripts/verifyOnChain.sh",
            chainName,
            `--constructor-args ${argsFileName}`,
            address,
        ],
        {
            detached: true, // run in BG
        }
    );

    verify.unref(); // don't wait for it to finish

    // uncomment if you want to see the output of the verify script
    // verify.stdout.on("data", (data) => {
    //     console.log(`stdout: ${data}`);
    // });

    // verify.stderr.on("data", (data) => {
    //     console.error(`stderr: ${data}`);
    // });
}

export function jsonStringify(obj: object, spaces = undefined) {
    return JSON.stringify(
        obj,
        (key, value) => (typeof value === "bigint" ? value.toString() : value), // return everything else unchanged
        spaces
    );
}

export function generateWallets(
    ethers: any,
    walletCount: number
): Array<NodeOperatorCredentials> {
    const newWallets = [];
    for (let i = 0; i < walletCount; i++) {
        newWallets.push({
            nodeWallet: generateWallet(ethers),
            stakerWallet: generateWallet(ethers),
            comsKeysSender: generateComsKeys(),
            comsKeysReceiver: generateComsKeys(),
        });
    }
    return newWallets;
}

const generateWallet = (ethers: any): Wallet => {
    const wallet = ethers.Wallet.createRandom();
    return wallet;
};

export function generateComsKeys(): ComsKeys {
    const keys = nacl.box.keyPair();
    return {
        publicKey: "0x" + Buffer.from(keys.publicKey).toString("hex"),
        privateKey: "0x" + Buffer.from(keys.secretKey).toString("hex"),
    };
}

export function saveConfigFiles(
    nodeOperatorsCredentials: Array<NodeOperatorCredentials>,
    contracts: ParsedNodeContracts,
    nodeRuntimeConfigParameters: {
        chainPollingInterval?: string;
        adminAddress: string;
        ipAddresses?: string[];
    },
    options?: {
        customNodeRuntimeConfigPath?: string;
        saveToDirectory?: string;
        filePrefix?: string;
    }
) {
    const ipAddressesAndPorts = [];
    if (nodeRuntimeConfigParameters.ipAddresses) {
        // split into ip and port
        for (
            let i = 0;
            i < nodeRuntimeConfigParameters.ipAddresses.length;
            i++
        ) {
            let ipAndPort = [nodeRuntimeConfigParameters.ipAddresses[i], "443"];
            if (nodeRuntimeConfigParameters.ipAddresses[i].includes(":")) {
                ipAndPort =
                    nodeRuntimeConfigParameters.ipAddresses[i].split(":");
            }
            ipAddressesAndPorts.push({
                ip: ipAndPort[0],
                port: parseInt(ipAndPort[1]),
            });
        }
    } else {
        for (let i = 0; i < nodeOperatorsCredentials.length; i++) {
            ipAddressesAndPorts.push({
                ip: contracts.litNodeDomainName,
                port: contracts.rocketPort + i,
            });
        }
    }

    for (let i = 0; i < nodeOperatorsCredentials.length; i++) {
        const nodeRuntimeBaseConfig = generateBaseConfig(
            contracts,
            nodeOperatorsCredentials[i]
        );
        const nodeRuntimeConfig = generateNodeRuntimeConfig(
            nodeRuntimeBaseConfig,
            contracts,
            nodeOperatorsCredentials[i],
            nodeRuntimeConfigParameters.adminAddress,
            ipAddressesAndPorts[i],
            nodeRuntimeConfigParameters.chainPollingInterval
        );

        // Read the custom config and merge, if provided.
        if (!!options?.customNodeRuntimeConfigPath) {
            console.info(
                `Merging custom node config from ${options.customNodeRuntimeConfigPath}`
            );
            const customNodeRuntimeConfig: any = TOML.parse(
                fs
                    .readFileSync(options.customNodeRuntimeConfigPath)
                    .toString("utf-8")
            );
            nodeRuntimeConfig.node = {
                ...nodeRuntimeConfig.node,
                ...customNodeRuntimeConfig.node,
            };
        }

        const saveToDirectory = options?.saveToDirectory || "./node_configs";
        if (!fs.existsSync(saveToDirectory)) {
            fs.mkdirSync(saveToDirectory);
        }
        const filePath = `${saveToDirectory}/${
            options?.filePrefix ? options?.filePrefix + "_" : ""
        }lit_config${i}.toml`;
        console.info("Saving to file", filePath);
        fs.writeFileSync(
            filePath,
            TOML.stringify(stringifyAllValues(removeEmpty(nodeRuntimeConfig)))
        );
    }
}

export function serializeWallets(
    nodeOperatorsCredentials: Array<NodeOperatorCredentials>
) {
    console.log("Serializing wallets", nodeOperatorsCredentials);
    const allWallets = nodeOperatorsCredentials.map(
        (nodeOperatorCredentials, idx) => {
            return {
                idx,
                node: {
                    address: nodeOperatorCredentials.nodeWallet.address,
                    privateKey:
                        nodeOperatorCredentials.nodeWallet.signingKey
                            .privateKey,
                    publicKey:
                        nodeOperatorCredentials.nodeWallet.signingKey.publicKey,
                    comsKeysSender: nodeOperatorCredentials.comsKeysSender,
                    comsKeysReceiver: nodeOperatorCredentials.comsKeysReceiver,
                },
                staker: {
                    address: nodeOperatorCredentials.stakerWallet.address,
                    privateKey:
                        nodeOperatorCredentials.stakerWallet.signingKey
                            .privateKey,
                    publicKey:
                        nodeOperatorCredentials.stakerWallet.signingKey
                            .publicKey,
                },
            };
        }
    );
    return allWallets;
}

export async function copyDirFiles(fromDir: string, toDir: string) {
    const files = await readdir(fromDir);
    for (const file of files) {
        const src = path.join(fromDir, file);
        const dest = path.join(toDir, file);
        console.info(`Copying ${src} to ${dest}`);
        await copyFile(src, dest);
    }
}

export async function logBalances(
    ethers: any,
    addresses: string[]
): Promise<void> {
    const addressBalances: any = {};

    const balancePromises = [];
    for (const address of addresses) {
        balancePromises.push(ethers.provider.getBalance(address));
    }

    const balances = await Promise.all(balancePromises);
    for (let i = 0; i < addresses.length; i++) {
        addressBalances[addresses[i]] = ethers.formatEther(balances[i]);
    }

    console.info("Address balances", addressBalances);
}

function generateNodeRuntimeConfig(
    nodeRuntimeBaseConfig: NodeRuntimeBaseConfig,
    contracts: ParsedNodeContracts,
    nodeOperatorCredential: NodeOperatorCredentials,
    adminAddress: string,
    ipAddressAndPort: {
        ip: string;
        port: number;
    },
    chainPollingInterval?: string
): NodeRuntimeConfig {
    return {
        ...nodeRuntimeBaseConfig,
        "node.http": {
            port: ipAddressAndPort.port,
        },
        node: {
            domain: ipAddressAndPort.ip,
            rpc_url: contracts.rpcUrl,
            enable_rate_limiting: false,
            enable_actions_allowlist: false,
            enable_epoch_transitions: true,
            enable_ecdsa_dkg: true,
            webauthn_allowed_origins: "http://*/",
            chain_polling_interval: chainPollingInterval,
            staker_address: nodeOperatorCredential.stakerWallet.address,
            admin_address: adminAddress,
            coms_keys_sender_privkey:
                nodeOperatorCredential.comsKeysSender.privateKey,
            coms_keys_receiver_privkey:
                nodeOperatorCredential.comsKeysReceiver.privateKey,
        },
    };
}

function generateBaseConfig(
    contracts: ParsedNodeContracts,
    nodeOperatorCredential: NodeOperatorCredentials
): NodeRuntimeBaseConfig {
    return {
        lit: {
            env: "dev",
        },
        "blockchain.wallet.default": {
            private_key: nodeOperatorCredential.nodeWallet.privateKey,
        },
        blockchain: {
            chain_id: contracts.chainId,
            chain_name: contracts.chainName,
        },
        subnet: {
            id: contracts.stakingContractAddress.slice(2),
        },
        ipfs: {
            gateway: `https://litnodes.mypinata.cloud/ipfs/{}?pinataGatewayToken=NkOJGWDsFcLTn7gXH37bS85HIMJJ4-d-r2qVHJWBXOXyxJYtG7FbyXATZCEAyf2s`,
        },
    };
}

interface NodeRuntimeBaseConfig {
    lit: {
        env: string;
    };
    "blockchain.wallet.default": {
        private_key: string;
    };
    blockchain: {
        chain_id: number;
        chain_name: string;
    };
    subnet: {
        id: string;
    };
    ipfs: {
        gateway: string;
    };
}

interface NodeRuntimeConfig extends NodeRuntimeBaseConfig {
    "node.http": {
        port: number;
    };
    node: {
        domain: string;
        rpc_url: string;
        enable_rate_limiting: boolean;
        enable_actions_allowlist: boolean;
        enable_epoch_transitions: boolean;
        enable_ecdsa_dkg: boolean;
        webauthn_allowed_origins: string;
        chain_polling_interval?: string;
        staker_address: string;
        admin_address: string;
        coms_keys_sender_privkey: string;
        coms_keys_receiver_privkey: string;
    };
}
