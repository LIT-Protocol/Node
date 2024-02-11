const fs = require("fs");
const path = require("path");

const BASE_ABI_PATH = "./artifacts/contracts";
const COPY_DIR_PATH = "./";
const CONTRACTS = ["DomainWalletOracle", "DomainWalletRegistry"];
const CONTRACTS_NODES = [
    "PKPHelper",
    "PKPNFTMetadata",
    "PKPNFT",
    "PKPPermissions",
    "LITToken",
];
const CONTRACTS_CORE = ["ContractResolver"];
const DEPLOYED_CONTRACTS =
    process.argv[2] === "test"
        ? "./deployed-dw-test-contracts.json"
        : "./deployed-lit-node-contracts-temp.json";

const NAME_TO_KEY_MAP = {
    PKPNFT: "pkpNftContractAddress",
    DomainWalletRegistry: "DomainWaleltRegistryAddress",
    DomainWalletOracle: "DomainWalletOracleAddress",
    PKPPermissions: "pkpPermissionsContractAddress",
    ContractResolver: "resolverContractAddress",
    LITToken: "LITTokenContractAddress",
};

function main() {
    let copyPath = COPY_DIR_PATH;
    console.info("looking for deployment addresses at: ", DEPLOYED_CONTRACTS);
    if (!fs.existsSync(path.join(COPY_DIR_PATH, "abis"))) {
        console.log("creating abis directory...");
        fs.mkdirSync(path.join(COPY_DIR_PATH, "abis"));
    }

    let deployedContracts = fs.readFileSync(DEPLOYED_CONTRACTS);
    deployedContracts = deployedContracts.toString("utf8");
    deployedContracts = JSON.parse(deployedContracts);

    for (const contract of CONTRACTS) {
        const dwPath = path.join(
            BASE_ABI_PATH,
            "domain-wallets",
            contract + ".sol",
            contract + ".json"
        );
        let abi = fs.readFileSync(dwPath);
        abi = abi.toString("utf8");
        abi = JSON.parse(abi);
        abi.address = deployedContracts[NAME_TO_KEY_MAP[contract]];
        fs.writeFileSync(dwPath, JSON.stringify(abi, null, 2));
        fs.copyFileSync(
            path.join(
                BASE_ABI_PATH,
                "domain-wallets",
                contract + ".sol",
                contract + ".json"
            ),
            path.join(copyPath, "abis", contract + ".json")
        );
    }

    for (const contract of CONTRACTS_NODES) {
        const nodePath = path.join(
            BASE_ABI_PATH,
            "lit-node",
            contract + ".sol",
            contract + ".json"
        );
        let abi = fs.readFileSync(nodePath);
        abi = abi.toString("utf8");
        abi = JSON.parse(abi);
        abi.address = deployedContracts[NAME_TO_KEY_MAP[contract]];
        fs.writeFileSync(nodePath, JSON.stringify(abi, null, 2));
        fs.copyFileSync(
            path.join(
                BASE_ABI_PATH,
                "lit-node",
                contract + ".sol",
                contract + ".json"
            ),
            path.join(copyPath, "abis", contract + ".json")
        );
    }

    for (const contract of CONTRACTS_CORE) {
        const nodePath = path.join(
            BASE_ABI_PATH,
            "lit-core",
            contract + ".sol",
            contract + ".json"
        );
        let abi = fs.readFileSync(nodePath);
        abi = abi.toString("utf8");
        abi = JSON.parse(abi);
        abi.address = deployedContracts[NAME_TO_KEY_MAP[contract]];
        fs.writeFileSync(nodePath, JSON.stringify(abi, null, 2));
        fs.copyFileSync(
            path.join(
                BASE_ABI_PATH,
                "lit-core",
                contract + ".sol",
                contract + ".json"
            ),
            path.join(copyPath, "abis", contract + ".json")
        );
    }

    console.log("done copying abi files");
}

main();
