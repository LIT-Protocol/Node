// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require("hardhat");
const fs = require("fs");

const { ethers } = hre;
const rpcUrl = hre.network.config.url;
let wlitAddress = hre.network.config.wlitAddress || false;
const { DEPLOY_LIT_NODE_OUTPUT_TEMP_FILE_PATH } = require("./deployConfig");
const path = require("path");
const { deployDiamondContract } = require("./deployDiamond");
const { CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY } = require("./constants");
const {
    contractAddressAlreadyExists,
    verifyContractInBg,
    jsonStringify,
} = require("./utils");

async function getChainId() {
    const { chainId } = await ethers.provider.getNetwork();
    return chainId;
}

const grantRolesTo = async (contract, address, roles) => {
    let txs = [];
    for (let i = 0; i < roles.length; i++) {
        txs.push(await contract.grantRole(roles[i], address));
    }

    await Promise.all(txs);
};

const mapEnvToEnum = (env) => {
    switch (env) {
        case "dev":
            return 0;
        case "staging":
            return 1;
        case "prod":
            return 2;
        default:
            throw new Error("ENV is invalid");
    }
};

const createDomainWalletSigner = () => {
    let privateKey = process.env.DOMAIN_WALLET_REGISTRY_PRIVATE_KEY;
    if (!privateKey) {
        throw new Error(
            "cannot continue deployment without domain wallet registry key"
        );
    }

    return new ethers.Wallet(privateKey, ethers.provider);
};

const getOrDeployContract = async (
    existingContracts,
    chainName,
    contractName,
    args = [],
    diamond = false,
    facets = [],
    useErc165Loupe = false
) => {
    if (contractAddressAlreadyExists(existingContracts, contractName)) {
        const contractAddressKey =
            CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY[contractName];
        return getContract(contractName, existingContracts[contractAddressKey]);
    } else {
        if (diamond) {
            return deployDiamondContract(
                chainName,
                contractName,
                args,
                facets,
                useErc165Loupe
            );
        } else {
            return deployContract(chainName, contractName, args);
        }
    }
};

const deployContract = async (chainName, contractName, args = []) => {
    console.log(`Deploying ${contractName} with args ${args}`);
    const contract = await ethers.deployContract(contractName, args);
    const deployTransaction = contract.deploymentTransaction();
    console.log(`${contractName} deploy tx hash: ${deployTransaction.hash}`);
    await contract.waitForDeployment();
    console.log(`${contractName} deployed to ${await contract.getAddress()}`);
    // await hre.tenderly.persistArtifacts({
    //     name: contractName,
    //     address: contract.address,
    // });
    // await hre.tenderly.verify({
    //     name: contractName,
    //     address: contract.address,
    // });
    verifyContractInBg(chainName, await contract.getAddress(), args);
    return contract;
};

const getContract = async (contractName, addr) => {
    return ethers.getContractAt(contractName, addr);
};

async function deployLitNodeContracts(deployNodeConfig) {
    const deployedFacets = {};
    const chainName = deployNodeConfig.networkName;
    const deployEnvEnum = mapEnvToEnum(deployNodeConfig.environment);
    console.log(
        "Deploying contracts to network " +
            chainName +
            " in environment " +
            deployEnvEnum
    );

    const domainWalletRegistryAccount =
        deployNodeConfig.newDomainWalletAdminAddress;
    console.log(
        `domain wallet admin address is: `,
        domainWalletRegistryAccount
    );

    let resolverContractAddress = deployNodeConfig.resolverContractAddress;
    const resolverContract = await getContract(
        "ContractResolver",
        resolverContractAddress
    );

    if (chainName === "localchain") {
        // to make hardhat act like our rollup, we need to
        // deploy wlit as well and set the address
        // so we are simulating that hardhat's native token is lit
        const wlit = await deployContract(chainName, "WLIT");
        wlitAddress = wlit.address;
    }

    const [deployer] = await ethers.getSigners();

    // *** 1. Deploy LITToken
    // if we're deploying this on the rollup, then we don't need to deploy the token.  the token is the native gas itself.  so let's set litToken.address to the wlit address
    let litToken;
    if (wlitAddress) {
        console.log("Deploying on rollup, using wlit address");
        litToken = await getContract("WLIT", wlitAddress);
    } else {
        const tokenCap = ethers.parseUnits("1000000000", 18);
        litToken = await deployContract(chainName, "LITToken", [tokenCap]);

        // Mint 1b tokens
        const amountToMint = ethers.parseUnits("1000000000", 18);
        const mintTx = await litToken.mint(deployer.address, amountToMint);
        await mintTx.wait();
        verifyContractInBg(chainName, await litToken.getAddress());
    }
    await resolverContract.setContract(
        await resolverContract.LIT_TOKEN_CONTRACT(),
        deployEnvEnum,
        await litToken.getAddress()
    );

    // *** 2.0 Deploy Staking Balances Contract
    console.log(
        "Deploying Staking Balances Contract with resolver address " +
            deployNodeConfig.resolverContractAddress
    );
    let deployResult = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "StakingBalances",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum],
        true,
        ["StakingBalancesFacet"]
    );
    const stakingBalancesContract = deployResult.diamond;
    deployedFacets["StakingBalances"] = deployResult.deployedFacets;
    // *** 2.2 Set address in resolver contract
    await resolverContract.setContract(
        await resolverContract.STAKING_BALANCES_CONTRACT(),
        deployEnvEnum,
        await stakingBalancesContract.getAddress()
    );

    // *** 2.3 Deploy Staking Contract
    console.log(
        "Deploying Staking Contract with token address " +
            (await litToken.getAddress())
    );
    deployResult = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "Staking",
        [resolverContractAddress, deployEnvEnum],
        true,
        ["StakingFacet", "StakingViewsFacet", "StakingVersionFacet"]
    );
    const stakingContract = deployResult.diamond;
    deployedFacets["Staking"] = deployResult.deployedFacets;
    // *** 2.4 Set address in resolver contract
    await resolverContract.setContract(
        await resolverContract.STAKING_CONTRACT(),
        deployEnvEnum,
        await stakingContract.getAddress()
    );

    // *** 3.1 Deploy Allowlist Contract
    const allowlistContract = await deployContract(chainName, "Allowlist");
    // turn it off, by default
    let tx = await allowlistContract.setAllowAll(true);
    await tx.wait();

    const newOwner = deployNodeConfig.newOwnerAddress;

    // make the newOwner an admin
    tx = await allowlistContract.addAdmin(newOwner);

    // *** 4. Deploy PKPNFT Contract
    deployResult = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "PKPNFT",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum],
        true,
        ["PKPNFTFacet"]
    );
    const pkpNFTContract = deployResult.diamond;
    deployedFacets["PKPNFT"] = deployResult.deployedFacets;
    await resolverContract.setContract(
        await resolverContract.PKP_NFT_CONTRACT(),
        deployEnvEnum,
        await pkpNFTContract.getAddress()
    );

    // *** 5. Deploy RateLimitNft Contract
    deployResult = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "RateLimitNFT",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum],
        true,
        ["RateLimitNFTFacet", "RateLimitNFTViewsFacet"]
    );
    const rateLimitNftContract = deployResult.diamond;
    deployedFacets["RateLimitNFT"] = deployResult.deployedFacets;

    // *** 6. Deploy PubkeyRouter Contract
    deployResult = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "PubkeyRouter",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum],
        true,
        ["PubkeyRouterFacet"]
    );
    const pubkeyRouterContract = deployResult.diamond;
    deployedFacets["PubkeyRouter"] = deployResult.deployedFacets;

    await resolverContract.setContract(
        await resolverContract.PUB_KEY_ROUTER_CONTRACT(),
        deployEnvEnum,
        await pubkeyRouterContract.getAddress()
    );

    // *** 7. Deploy Multisender Contract
    const multisenderContract = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "Multisender"
    );

    // *** 9. Send tokens to multisender to be sent to stakers
    // if we're using the wrapped token, then we need to wrap first, and send a smaller amount
    let amountForStakers;
    if (wlitAddress) {
        amountForStakers = ethers.parseUnits(
            (deployNodeConfig.numberOfNodeWallets + 100).toString(), // send an extra 100 so the deployer has some
            18
        );
        const wrapTx = await litToken.deposit({ value: amountForStakers });
        console.log("wrap tx hash: " + wrapTx.hash);
        await wrapTx.wait();
    } else {
        console.log("Sending tokens to multisender");
        // 100m for stakers
        amountForStakers = ethers.parseUnits("100000000", 18);
    }
    let transferTx = await litToken.transfer(
        await multisenderContract.getAddress(),
        amountForStakers
    );
    console.log("Transfer tx hash: " + transferTx.hash);
    await transferTx.wait();

    // *** 10. Send remaining tokens to newOwner
    // only do this if we're not using the wrapped token
    if (!wlitAddress) {
        const amountRemaining = await litToken.balanceOf(deployer.address);
        transferTx = await litToken.transfer(newOwner, amountRemaining);
        await transferTx.wait();

        // *** 11. Set new owner of LITToken
        console.log("Setting new owner of LITToken contract...");
        /// @dev The identifier of the role which maintains other roles.
        const ADMIN_ROLE = ethers.keccak256(ethers.toUtf8Bytes("ADMIN"));
        /// @dev The identifier of the role which allows accounts to mint tokens.
        const MINTER_ROLE = ethers.keccak256(ethers.toUtf8Bytes("MINTER"));
        let adminTx = await litToken.grantRole(ADMIN_ROLE, newOwner);
        let minterTx = await litToken.grantRole(MINTER_ROLE, newOwner);
        await Promise.all([adminTx.wait(), minterTx.wait()]);
        console.log("New owner set.");
    }

    const pkpNftMetadataContract = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "PKPNFTMetadata",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum]
    );

    // *** 12. get chain id
    const chainId = await getChainId();

    // 21.1 Domain Wallet Contracts
    let domainWalletRegistry = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "DomainWalletRegistry",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum]
    );

    let domainWalletOracle = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "DomainWalletOracle",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum]
    );

    if (
        !contractAddressAlreadyExists(
            deployNodeConfig.existingContracts,
            "DomainWalletOracle"
        )
    ) {
        console.log("Setting new Domain Wallet oracle address");
        verifyContractInBg(chainName, await domainWalletOracle.getAddress(), [
            domainWalletRegistryAccount ?? newOwner,
        ]);

        console.log("Setting new owner for DomainWalletOracle");
        let tx = await domainWalletOracle.addAdmin(
            domainWalletRegistryAccount ?? newOwner
        );
        tx = await tx.wait();
    }

    // *** 13. Deploy PKPPermissions Contract
    deployResult = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "PKPPermissions",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum],
        true,
        ["PKPPermissionsFacet"]
    );
    const pkpPermissionsContract = deployResult.diamond;
    deployedFacets["PKPPermissions"] = deployResult.deployedFacets;

    // *** 14. Deploy PKPHelper Contract
    const pkpHelperContract = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "PKPHelper",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum]
    );

    // set PKPHelper contract address in registry and add new admin
    if (
        !contractAddressAlreadyExists(
            deployNodeConfig.existingContracts,
            "DomainWalletRegistry"
        )
    ) {
        verifyContractInBg(chainName, await domainWalletRegistry.getAddress());
        await tx.wait();

        console.log("Setting owner for domain wallet registry");
        tx = await domainWalletRegistry.addAdmin(
            domainWalletRegistryAccount ?? newOwner
        );

        await tx.wait();
    }

    // *** 16. Deploy HDKeyDeriver Contract
    const hdKeyDeriverContract = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "KeyDeriver"
    );

    // *** 17. Deploy BackupRecovery Contract
    let backup_recovery_facets = [];
    let backup_recovery_testing = false;
    if (
        deployNodeConfig.environment === "dev" &&
        deployNodeConfig.backupRecoveryKeys &&
        deployNodeConfig.backupRecoveryKeys.length == 2
    ) {
        console.log(
            "found mock recovery party in deployment config, deploying BackupRecoveryTestFacet"
        );
        backup_recovery_testing = true;
        backup_recovery_facets = [
            "BackupRecoveryFacet",
            "BackupRecoveryTestFacet",
        ];
    } else {
        backup_recovery_facets = ["BackupRecoveryFacet"];
    }

    deployResult = await getOrDeployContract(
        deployNodeConfig.existingContracts,
        chainName,
        "BackupRecovery",
        [deployNodeConfig.resolverContractAddress, deployEnvEnum],
        true,
        backup_recovery_facets
    );
    const backupRecoveryContract = deployResult.diamond;
    deployedFacets["BackupRecovery"] = deployResult.deployedFacets;
    console.log(deployResult.deployedFacets);

    if (backup_recovery_testing) {
        console.log("found mock recovery party in deployment config");
        const tx = await backupRecoveryContract.setBackupPartyState(
            deployNodeConfig.backupRecoveryKeys[0],
            deployNodeConfig.backupRecoveryKeys[1],
            deployNodeConfig.backupRecoveryAddresses
        );
        await tx.wait();
        console.log("done registering backup party state");
    } else if (
        deployNodeConfig.backupRecoveryAddresses &&
        deployNodeConfig.backupRecoveryAddresses.length > 0
    ) {
        console.log(
            "found backup addresses in deployment config, registering on chain"
        );
        const tx = await backupRecoveryContract.registerNewBackupParty(
            deployNodeConfig.backupRecoveryAddresses
        );
        await tx.wait();
        console.log("done registering backup party members");
    }

    // *** 18. Unpause the staking contract
    const state = await stakingContract.state();
    console.log("Contract state", state);
    console.log("Setting staking contract state to Active");
    tx = await stakingContract.setEpochState(0);
    await tx.wait();

    // *** 19. Set contract addresses in resolver contract
    console.log("Setting contract addresses in resolver...");

    let txs = [];

    txs.push(
        await resolverContract.setContract(
            await resolverContract.MULTI_SENDER_CONTRACT(),
            deployEnvEnum,
            await multisenderContract.getAddress()
        )
    );
    txs.push(
        await resolverContract.setContract(
            await resolverContract.RATE_LIMIT_NFT_CONTRACT(),
            deployEnvEnum,
            await rateLimitNftContract.getAddress()
        )
    );
    txs.push(
        await resolverContract.setContract(
            await resolverContract.PKP_HELPER_CONTRACT(),
            deployEnvEnum,
            await pkpHelperContract.getAddress()
        )
    );
    txs.push(
        await resolverContract.setContract(
            await resolverContract.PKP_PERMISSIONS_CONTRACT(),
            deployEnvEnum,
            await pkpPermissionsContract.getAddress()
        )
    );
    txs.push(
        await resolverContract.setContract(
            await resolverContract.PKP_NFT_METADATA_CONTRACT(),
            deployEnvEnum,
            await pkpNftMetadataContract.getAddress()
        )
    );
    txs.push(
        await resolverContract.setContract(
            await resolverContract.ALLOWLIST_CONTRACT(),
            deployEnvEnum,
            await allowlistContract.getAddress()
        )
    );

    txs.push(
        await resolverContract.setContract(
            await resolverContract.DOMAIN_WALLET_REGISTRY(),
            deployEnvEnum,
            await domainWalletRegistry.getAddress()
        )
    );

    txs.push(
        await resolverContract.setContract(
            await resolverContract.DOMAIN_WALLET_ORACLE(),
            deployEnvEnum,
            await domainWalletOracle.getAddress()
        )
    );

    txs.push(
        await resolverContract.setContract(
            await resolverContract.HD_KEY_DERIVER_CONTRACT(),
            deployEnvEnum,
            await hdKeyDeriverContract.getAddress()
        )
    );

    txs.push(
        await resolverContract.setContract(
            await resolverContract.BACKUP_RECOVERY_CONTRACT(),
            deployEnvEnum,
            await backupRecoveryContract.getAddress()
        )
    );

    const results = await Promise.all(txs);
    console.log("results from setting contracts in resolver", results);

    if (newOwner.toLowerCase() != deployer.address.toLowerCase()) {
        console.log("Adding new owner as admin");
        tx = await resolverContract.addAdmin(newOwner, {
            gasLimit: 1 * 10 ** 6,
        });
        await tx.wait();
        console.log("New owner added as admin");
    }

    const finalJson = {
        backupRecoveryContractAddress:
            await backupRecoveryContract.getAddress(),
        stakingBalancesContractAddress:
            await stakingBalancesContract.getAddress(),
        stakingContractAddress: await stakingContract.getAddress(),
        multisenderContractAddress: await multisenderContract.getAddress(),
        litTokenContractAddress: await litToken.getAddress(),
        // used for the config file generation
        pubkeyRouterContractAddress: await pubkeyRouterContract.getAddress(),
        pkpNftContractAddress: await pkpNFTContract.getAddress(),
        rateLimitNftContractAddress: await rateLimitNftContract.getAddress(),
        pkpHelperContractAddress: await pkpHelperContract.getAddress(),
        pkpPermissionsContractAddress:
            await pkpPermissionsContract.getAddress(),
        pkpNftMetadataContractAddress:
            await pkpNftMetadataContract.getAddress(),
        allowlistContractAddress: await allowlistContract.getAddress(),
        resolverContractAddress: await resolverContract.getAddress(),
        DomainWaleltRegistryAddress: await domainWalletRegistry.getAddress(),
        DomainWalletOracleAddress: await domainWalletOracle.getAddress(),
        hdKeyDeriverContractAddress: await hdKeyDeriverContract.getAddress(),
        chainId,
        rpcUrl,
        chainName,
        litNodeDomainName: "127.0.0.1",
        litNodePort: 7470,
        rocketPort: 7470,
        facets: deployedFacets,
    };

    console.log("final JSON: ");
    console.log(jsonStringify(finalJson, 2));

    // *** 20. Write to file
    const fileName = DEPLOY_LIT_NODE_OUTPUT_TEMP_FILE_PATH;
    console.log("Writing to file: " + fileName);
    fs.writeFileSync(fileName, jsonStringify(finalJson, 2));
}

module.exports = {
    deployLitNodeContracts,
};
