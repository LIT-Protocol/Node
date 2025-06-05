// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require('hardhat');
const fs = require('fs');
const path = require('path');

const { ethers } = hre;
const rpcUrl = hre.network.config.url;
let wlitAddress = hre.network.config.wlitAddress || false;
const { DEPLOY_LIT_NODE_OUTPUT_TEMP_FILE_PATH } = require('./deployConfig');
const {
  deployDiamondContract,
  getDiamondContract,
} = require('./deployDiamond');
const { CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY } = require('./constants');
const {
  contractAddressAlreadyExists,
  verifyContractInBg,
  jsonStringify,
  hardhatDeployAndVerifySingleContract,
} = require('./utils');
const { cloneNetFacetAbi } = require('../abis/generated');

async function getChainId() {
  const { chainId } = await ethers.provider.getNetwork();
  return chainId;
}

const mapEnvToEnum = (env) => {
  switch (env) {
    case 'dev':
      return 0;
    case 'staging':
      return 1;
    case 'prod':
      return 2;
    default:
      throw new Error('ENV is invalid');
  }
};

function isArbitrumChain(chainName) {
  return chainName === 'localchainArbitrum' || chainName === 'yellowstone';
}

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
    if (diamond) {
      return getDiamondContract(
        contractName,
        existingContracts[contractAddressKey]
      );
    } else {
      return getContract(contractName, existingContracts[contractAddressKey]);
    }
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
      return hardhatDeployAndVerifySingleContract(
        ethers,
        chainName,
        contractName,
        {
          deploymentArgs: args,
        }
      );
    }
  }
};

const getContract = async (contractName, addr) => {
  return ethers.getContractAt(contractName, addr);
};

async function deployLitNodeContracts(deployNodeConfig) {
  const deployedFacets = {};
  const chainName = deployNodeConfig.networkName;
  const deployEnvEnum = mapEnvToEnum(deployNodeConfig.environment);
  console.log(
    'Deploying contracts to network ' +
      chainName +
      ' in environment ' +
      deployEnvEnum
  );

  const domainWalletRegistryAccount =
    deployNodeConfig.newDomainWalletAdminAddress;
  console.log(`domain wallet admin address is: `, domainWalletRegistryAccount);

  let resolverContractAddress = deployNodeConfig.resolverContractAddress;
  const resolverContract = await getContract(
    'ContractResolver',
    resolverContractAddress
  );

  if (chainName === 'localchain') {
    // to make hardhat act like our rollup, we need to
    // deploy wlit as well and set the address
    // so we are simulating that hardhat's native token is lit
    const wlit = await hardhatDeployAndVerifySingleContract(
      ethers,
      chainName,
      'WLIT'
    );
    wlitAddress = wlit.address;
  }

  const [deployer] = await ethers.getSigners();

  // *** 1. Deploy LITToken
  // if we're deploying this on the rollup, then we don't need to deploy the token.  the token is the native gas itself.  so let's set litToken.address to the wlit address
  let litToken;
  if (wlitAddress) {
    console.log('Deploying on rollup, using wlit address');
    litToken = await getContract('WLIT', wlitAddress);
  } else {
    const tokenCap = ethers.parseUnits('1000000000', 18);
    litToken = await hardhatDeployAndVerifySingleContract(
      ethers,
      chainName,
      'LITToken',
      {
        deploymentArgs: [tokenCap],
      }
    );

    // Mint 1b tokens
    const amountToMint = ethers.parseUnits('1000000000', 18);
    const mintTx = await litToken.mint(deployer.address, amountToMint);
    await mintTx.wait();
  }

  // *** 2.0 Deploy Staking Balances Contract
  console.log(
    'Deploying Staking Balances Contract with resolver address ' +
      deployNodeConfig.resolverContractAddress
  );
  let deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'StakingBalances',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum],
    true,
    ['StakingBalancesFacet']
  );
  const stakingBalancesContract = deployResult.diamond;
  deployedFacets['StakingBalances'] = deployResult.deployedFacets;

  // *** 2.3 Deploy Staking Contract
  console.log(
    'Deploying Staking Contract with token address ' +
      (await litToken.getAddress())
  );
  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'Staking',
    [resolverContractAddress, deployEnvEnum],
    true,
    [
      'StakingFacet',
      'StakingViewsFacet',
      'StakingVersionFacet',
      'StakingAdminFacet',
    ]
  );
  const stakingContract = deployResult.diamond;
  deployedFacets['Staking'] = deployResult.deployedFacets;

  // *** 2.4 Deploy CloneNet Contract
  console.log(
    'Deploying CloneNet Contract with resolver address ' +
      deployNodeConfig.resolverContractAddress
  );
  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'CloneNet',
    [resolverContractAddress, deployEnvEnum],
    true,
    ['CloneNetFacet']
  );
  const cloneNetContract = deployResult.diamond;
  deployedFacets['CloneNet'] = deployResult.deployedFacets;

  // Set the active staking contract to the staking contract we just deployed
  const cloneNetFacet = await ethers.getContractAt(
    'CloneNetFacet',
    await cloneNetContract.getAddress()
  );
  const setActiveStakingContractTx =
    await cloneNetFacet.adminAddActiveStakingContract(
      await stakingContract.getAddress()
    );
  await setActiveStakingContractTx.wait();

  // *** 3.1 Deploy Allowlist Contract
  const allowlistContract = await hardhatDeployAndVerifySingleContract(
    ethers,
    chainName,
    'Allowlist'
  );
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
    'PKPNFT',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum],
    true,
    ['PKPNFTFacet']
  );
  const pkpNFTContract = deployResult.diamond;
  deployedFacets['PKPNFT'] = deployResult.deployedFacets;

  // *** 5. Deploy RateLimitNft Contract
  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'RateLimitNFT',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum],
    true,
    ['RateLimitNFTFacet', 'RateLimitNFTViewsFacet']
  );
  const rateLimitNftContract = deployResult.diamond;
  deployedFacets['RateLimitNFT'] = deployResult.deployedFacets;

  // *** 6. Deploy PubkeyRouter Contract
  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'PubkeyRouter',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum],
    true,
    ['PubkeyRouterFacet']
  );
  const pubkeyRouterContract = deployResult.diamond;
  deployedFacets['PubkeyRouter'] = deployResult.deployedFacets;

  // *** 7. Deploy Multisender Contract
  const multisenderContract = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'Multisender'
  );

  // *** 9. Send tokens to multisender to be sent to stakers
  // if we're using the wrapped token, then we need to wrap first, and send a smaller amount
  let amountForStakers;
  if (wlitAddress) {
    const totalStakers =
      deployNodeConfig.numberOfStakedOnlyWallets +
      deployNodeConfig.numberOfStakedAndJoinedWallets;
    amountForStakers = ethers.parseUnits(
      (totalStakers + 100).toString(), // send an extra 100 so the deployer has some
      18
    );
    const wrapTx = await litToken.deposit({ value: amountForStakers });
    console.log('wrap tx hash: ' + wrapTx.hash);
    await wrapTx.wait();
  } else {
    console.log('Sending tokens to multisender');
    // 100m for stakers
    amountForStakers = ethers.parseUnits('100000000', 18);
  }
  let transferTx = await litToken.transfer(
    await multisenderContract.getAddress(),
    amountForStakers
  );
  console.log('Transfer tx hash: ' + transferTx.hash);
  await transferTx.wait();

  // *** 10. Send remaining tokens to newOwner
  // only do this if we're not using the wrapped token
  if (!wlitAddress) {
    const amountRemaining = await litToken.balanceOf(deployer.address);
    transferTx = await litToken.transfer(newOwner, amountRemaining);
    await transferTx.wait();

    // *** 11. Set new owner of LITToken
    console.log('Setting new owner of LITToken contract...');
    /// @dev The identifier of the role which maintains other roles.
    const ADMIN_ROLE = ethers.keccak256(ethers.toUtf8Bytes('ADMIN'));
    /// @dev The identifier of the role which allows accounts to mint tokens.
    const MINTER_ROLE = ethers.keccak256(ethers.toUtf8Bytes('MINTER'));
    let adminTx = await litToken.grantRole(ADMIN_ROLE, newOwner);
    let minterTx = await litToken.grantRole(MINTER_ROLE, newOwner);
    await Promise.all([adminTx.wait(), minterTx.wait()]);
    console.log('New owner set.');
  }

  const pkpNftMetadataContract = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'PKPNFTMetadata',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum]
  );

  // *** 12. get chain id
  const chainId = await getChainId();

  // 21.1 Domain Wallet Contracts
  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'DomainWalletRegistry',
    [resolverContractAddress, deployEnvEnum],
    true,
    ['DomainWalletRegistryFacet', 'DomainWalletRegistryViewsFacet']
  );
  const domainWalletRegistry = deployResult.diamond;
  deployedFacets['DomainWalletRegistry'] = deployResult.deployedFacets;

  // *** 13. Deploy PKPPermissions Contract
  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'PKPPermissions',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum],
    true,
    ['PKPPermissionsFacet']
  );
  const pkpPermissionsContract = deployResult.diamond;
  deployedFacets['PKPPermissions'] = deployResult.deployedFacets;

  // *** 14. Deploy PKPHelper Contract
  const pkpHelperContract = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'PKPHelper',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum]
  );

  // *** 15. Deploy PKPHelperV2 Contract
  const pkpHelperV2Contract = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'PKPHelperV2',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum]
  );

  // *** 16. Deploy HDKeyDeriver Contract
  let hdKeyDeriverContract;
  if (isArbitrumChain(deployNodeConfig.networkName)) {
    hdKeyDeriverContract = await getOrDeployContract(
      deployNodeConfig.existingContracts,
      chainName,
      'ArbitrumKeyDeriver',
      [deployNodeConfig.resolverContractAddress, deployEnvEnum]
    );
  } else {
    hdKeyDeriverContract = await getOrDeployContract(
      deployNodeConfig.existingContracts,
      chainName,
      'KeyDeriver'
    );
  }

  // *** 17. Deploy BackupRecovery Contract
  let backup_recovery_facets = [];
  let backup_recovery_testing = false;
  if (
    deployNodeConfig.environment === 'dev' &&
    deployNodeConfig.backupRecoveryKeys &&
    deployNodeConfig.backupRecoveryKeys.length == 2
  ) {
    console.log(
      'found mock recovery party in deployment config, deploying BackupRecoveryTestFacet'
    );
    backup_recovery_testing = true;
    backup_recovery_facets = [
      'BackupRecoveryFacet',
      'BackupRecoveryNodeStatusFacet',
      'BackupRecoveryTestFacet',
      'BackupRecoveryViewFacet',
    ];
  } else {
    backup_recovery_facets = [
      'BackupRecoveryFacet',
      'BackupRecoveryNodeStatusFacet',
      'BackupRecoveryViewFacet',
    ];
  }

  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'BackupRecovery',
    [deployNodeConfig.resolverContractAddress, deployEnvEnum],
    true,
    backup_recovery_facets
  );
  const backupRecoveryContract = deployResult.diamond;
  deployedFacets['BackupRecovery'] = deployResult.deployedFacets;
  console.log(deployResult.deployedFacets);

  if (backup_recovery_testing) {
    console.log('found mock recovery party in deployment config');
    const tx = await backupRecoveryContract.setBackupPartyState(
      deployNodeConfig.backupRecoveryKeys[0],
      deployNodeConfig.backupRecoveryKeys[1],
      deployNodeConfig.backupRecoveryAddresses
    );
    await tx.wait();
    console.log('done registering backup party state');
  } else if (
    deployNodeConfig.backupRecoveryAddresses &&
    deployNodeConfig.backupRecoveryAddresses.length > 0
  ) {
    console.log(
      'found backup addresses in deployment config, registering on chain'
    );
    const tx = await backupRecoveryContract.registerNewBackupParty(
      deployNodeConfig.backupRecoveryAddresses
    );
    await tx.wait();
    console.log('done registering backup party members');
  }

  // *** 17.1 Deploy Payment Delegation Contract
  console.log('Deploying Payment Delegation Contract');
  deployResult = await getOrDeployContract(
    deployNodeConfig.existingContracts,
    chainName,
    'PaymentDelegation',
    [],
    true,
    ['PaymentDelegationFacet']
  );
  const paymentDelegationContract = deployResult.diamond;
  deployedFacets['PaymentDelegation'] = deployResult.deployedFacets;

  // *** 18. Unpause the staking contract
  const state = await stakingContract.state();
  console.log('Contract state', state);
  console.log('Setting staking contract state to Active');
  tx = await stakingContract.setEpochState(0);
  await tx.wait();

  // *** 19. Set contract addresses in resolver contract
  console.log('Setting contract addresses in resolver...');

  let txs = [];

  txs.push(
    await resolverContract.setContract(
      await resolverContract.LIT_TOKEN_CONTRACT(),
      deployEnvEnum,
      await litToken.getAddress()
    )
  );

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
      await resolverContract.PUB_KEY_ROUTER_CONTRACT(),
      deployEnvEnum,
      await pubkeyRouterContract.getAddress()
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
      await resolverContract.PKP_HELPER_V2_CONTRACT(),
      deployEnvEnum,
      await pkpHelperV2Contract.getAddress()
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
      await resolverContract.PKP_NFT_CONTRACT(),
      deployEnvEnum,
      await pkpNFTContract.getAddress()
    )
  );

  txs.push(
    await resolverContract.setContract(
      await resolverContract.ALLOWLIST_CONTRACT(),
      deployEnvEnum,
      await allowlistContract.getAddress()
    )
  );

  if (isArbitrumChain(deployNodeConfig.networkName)) {
    const p256Address =
      deployNodeConfig.existingContracts?.stylusContractP256Address ??
      hre.network.config.stylusContractsForTests.p256;
    const k256Address =
      deployNodeConfig.existingContracts?.stylusContractK256Address ??
      hre.network.config.stylusContractsForTests.k256;
    if (!p256Address || !k256Address) {
      throw new Error(
        'Missing stylus contract addresses for localchainArbitrum'
      );
    }

    txs.push(
      await resolverContract.setContract(
        ethers.keccak256(ethers.toUtf8Bytes('HD_KEY_DERIVER_CONTRACT_P256')),
        deployEnvEnum,
        p256Address
      )
    );

    txs.push(
      await resolverContract.setContract(
        ethers.keccak256(ethers.toUtf8Bytes('HD_KEY_DERIVER_CONTRACT_K256')),
        deployEnvEnum,
        k256Address
      )
    );
  }

  async function getAbi(contractName) {
    const _abiPath = path.resolve(__dirname, `../abis/${contractName}.abi`);

    const _abi = JSON.parse(fs.readFileSync(_abiPath, 'utf-8'));

    return _abi;
  }

  // This is the context file required for the contracts-sdk
  // Allowlist: LitContract;
  // LITToken: LitContract;
  // Multisender: LitContract;
  // PKPHelper: LitContract;
  // PKPNFT: LitContract;
  // PKPNFTMetadata: LitContract;
  // PKPPermissions: LitContract;
  // PubkeyRouter: LitContract;
  // RateLimitNFT: LitContract;
  // Staking: LitContract;
  // StakingBalances: LitContract;
  const finalContext = {
    Allowlist: {
      address: await allowlistContract.getAddress(),
      abi: await getAbi('Allowlist'),
      name: 'Allowlist',
    },
    LITToken: {
      address: await litToken.getAddress(),
      abi: await getAbi('LITToken'),
      name: 'LITToken',
    },
    Multisender: {
      address: await multisenderContract.getAddress(),
      abi: await getAbi('Multisender'),
      name: 'Multisender',
    },
    PKPHelper: {
      address: await pkpHelperContract.getAddress(),
      abi: await getAbi('PKPHelper'),
      name: 'PKPHelper',
    },
    PKPNFT: {
      address: await pkpNFTContract.getAddress(),
      abi: await getAbi('PKPNFT'),
      name: 'PKPNFT',
    },
    PKPNFTMetadata: {
      address: await pkpNftMetadataContract.getAddress(),
      abi: await getAbi('PKPNFTMetadata'),
      name: 'PKPNFTMetadata',
    },
    PKPPermissions: {
      address: await pkpPermissionsContract.getAddress(),
      abi: await getAbi('PKPPermissions'),
      name: 'PKPPermissions',
    },
    PubkeyRouter: {
      address: await pubkeyRouterContract.getAddress(),
      abi: await getAbi('PubkeyRouter'),
      name: 'PubkeyRouter',
    },
    RateLimitNFT: {
      address: await rateLimitNftContract.getAddress(),
      abi: await getAbi('RateLimitNFT'),
      name: 'RateLimitNFT',
    },
    Staking: {
      address: await stakingContract.getAddress(),
      abi: await getAbi('Staking'),
      name: 'Staking',
    },
    StakingBalances: {
      address: await stakingBalancesContract.getAddress(),
      abi: await getAbi('StakingBalances'),
      name: 'StakingBalances',
    },
    CloneNet: {
      address: await cloneNetContract.getAddress(),
      abi: cloneNetFacetAbi,
      name: 'CloneNet',
    },
    ContractResolver: {
      address: await resolverContract.getAddress(),
      abi: await getAbi('ContractResolver'),
      name: 'ContractResolver',
    },
  };

  // write the final context to a file
  const finalContextJsonString = JSON.stringify(finalContext, null, 2);

  const contextPath = path.resolve(__dirname, '../networkContext.json');

  fs.writeFileSync(contextPath, finalContextJsonString);

  txs.push(
    await resolverContract.setContract(
      await resolverContract.DOMAIN_WALLET_REGISTRY(),
      deployEnvEnum,
      await domainWalletRegistry.getAddress()
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

  txs.push(
    await resolverContract.setContract(
      await resolverContract.STAKING_CONTRACT(),
      deployEnvEnum,
      await stakingContract.getAddress()
    )
  );

  txs.push(
    await resolverContract.setContract(
      await resolverContract.STAKING_BALANCES_CONTRACT(),
      deployEnvEnum,
      await stakingBalancesContract.getAddress()
    )
  );

  txs.push(
    await resolverContract.setContract(
      await resolverContract.CLONE_NET_CONTRACT(),
      deployEnvEnum,
      await cloneNetContract.getAddress()
    )
  );

  txs.push(
    await resolverContract.setContract(
      await resolverContract.PAYMENT_DELEGATION_CONTRACT(),
      deployEnvEnum,
      await paymentDelegationContract.getAddress()
    )
  );

  const results = await Promise.all(txs);
  console.log('results from setting contracts in resolver', results);

  if (newOwner.toLowerCase() != deployer.address.toLowerCase()) {
    console.log('Adding new owner as admin');
    tx = await resolverContract.addAdmin(newOwner, {
      gasLimit: 1 * 10 ** 6,
    });
    await tx.wait();
    console.log('New owner added as admin');
  }

  const finalJson = {
    backupRecoveryContractAddress: await backupRecoveryContract.getAddress(),
    stakingBalancesContractAddress: await stakingBalancesContract.getAddress(),
    stakingContractAddress: await stakingContract.getAddress(),
    cloneNetContractAddress: await cloneNetContract.getAddress(),
    multisenderContractAddress: await multisenderContract.getAddress(),
    litTokenContractAddress: await litToken.getAddress(),
    // used for the config file generation
    pubkeyRouterContractAddress: await pubkeyRouterContract.getAddress(),
    pkpNftContractAddress: await pkpNFTContract.getAddress(),
    rateLimitNftContractAddress: await rateLimitNftContract.getAddress(),
    pkpHelperContractAddress: await pkpHelperContract.getAddress(),
    pkpHelperV2ContractAddress: await pkpHelperV2Contract.getAddress(),
    pkpPermissionsContractAddress: await pkpPermissionsContract.getAddress(),
    pkpNftMetadataContractAddress: await pkpNftMetadataContract.getAddress(),
    allowlistContractAddress: await allowlistContract.getAddress(),
    resolverContractAddress: await resolverContract.getAddress(),
    domainWalletRegistryAddress: await domainWalletRegistry.getAddress(),
    hdKeyDeriverContractAddress: await hdKeyDeriverContract.getAddress(),
    paymentDelegationContractAddress:
      await paymentDelegationContract.getAddress(),
    chainId,
    rpcUrl,
    chainName,
    litNodeDomainName: '127.0.0.1',
    litNodePort: 7470,
    rocketPort: 7470,
    facets: deployedFacets,
  };

  console.log('final JSON: ');
  console.log(jsonStringify(finalJson, 2));

  // *** 20. Write to file
  const fileName = DEPLOY_LIT_NODE_OUTPUT_TEMP_FILE_PATH;
  console.log('Writing to file: ' + fileName);
  fs.writeFileSync(fileName, jsonStringify(finalJson, 2));
}

module.exports = {
  deployLitNodeContracts,
};
