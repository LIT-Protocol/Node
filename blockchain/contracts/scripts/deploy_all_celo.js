// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
const hre = require('hardhat');
const { ip2int, int2ip } = require('../utils');
const fs = require('fs');
const { ethers, BigNumber } = require('ethers-v5');
require('dotenv').config();

// quick & dirty config writing helper!
let nodeFileId = 1;
const f = (n, v) => {
  if (v) {
    n = n + v;
  }
  fs.appendFileSync('lit_config' + nodeFileId + '.env', n + '\n', (err) => {
    if (err) {
      console.error(err);
    }
  });
};

// because ethers isn't completely compatible....
const getCeloProvider = () => {
  const provider = new ethers.providers.JsonRpcProvider(hre.network.config.url);

  const blockFormat = provider.formatter.formats.block;
  blockFormat.gasLimit = () => ethers.BigNumber.from(0);
  blockFormat.nonce = () => '';
  blockFormat.difficulty = () => 0;

  const blockWithTransactionsFormat =
    provider.formatter.formats.blockWithTransactions;
  blockWithTransactionsFormat.gasLimit = () => ethers.BigNumber.from(0);
  blockWithTransactionsFormat.nonce = () => '';
  blockWithTransactionsFormat.difficulty = () => 0;

  return provider;
};

async function main() {
  // we really only need a single deployer & are using HH accounts, or the default account for the connected chain,
  [deployer, ...signers] = await hre.ethers.getSigners(); /// the deployer is the first account lilsted

  //  const chainProvider = hre.ethers.provider;
  const chainProvider = getCeloProvider();

  // Set these values for the number of nodes to create, and funds to allocate
  const decimals = 18;
  const nodeCount = 10;
  const initialMintAmount = hre.ethers.utils.parseUnits('1000000', decimals);
  const nodeTransferAmount = hre.ethers.utils.parseUnits('10000', decimals);
  const staking_amount = hre.ethers.utils.parseUnits('100', decimals);

  // Deploy Contracts
  console.log('\n\n');
  console.log('Deploying contracts to : ', hre.network.name);
  console.log('Deployer Account:', deployer.address);

  // Deploy LIT Token contract
  const f_LITToken = await hre.ethers.getContractFactory('LITToken', {
    signer: deployer,
  });
  const c_LITToken = await f_LITToken.deploy(); /// currently the owner address is hardcoded.  To be replaced by public keys.
  await c_LITToken.deployed();
  console.log('Contract for LITToken deployed to:', c_LITToken.address);

  // Deploy staking contract
  const f_Staking = await hre.ethers.getContractFactory('Staking', {
    signer: deployer,
  });
  const c_Staking = await f_Staking.deploy(c_LITToken.address); /// currently the owner address is hardcoded.  To be replaced by public keys.
  await c_Staking.deployed();
  console.log('Contract for Staking deployed to:', c_Staking.address);

  // Deploy Access Control Conditions
  const f_AccessCtlConditions = await hre.ethers.getContractFactory(
    'AccessControlConditions',
    { signer: deployer }
  );
  const c_AccessCtlConditions = await f_AccessCtlConditions.deploy(); /// currently the owner address is hardcoded.  To be replaced by public keys.
  await c_AccessCtlConditions.deployed();
  console.log(
    'Contract for AccessControlConditions deployed to:',
    c_AccessCtlConditions.address
  );

  // // Deploy Condition Validations Contract
  const f_conditionValidation = await hre.ethers.getContractFactory(
    'ConditionValidations',
    { signer: deployer }
  );
  const c_conditionValidation = await f_conditionValidation.deploy(
    '0x804C96C9750a57FB841f26a7bC9f2815782D8529'
  ); /// currently the owner address is hardcoded.  To be replaced by public keys.
  await c_conditionValidation.deployed();
  console.log(
    'Contract for ConditionValidations deployed to:',
    c_conditionValidation.address
  );

  // deploy the PKP contract
  const TokenContractFactory = await hre.ethers.getContractFactory('PKPNFT');
  const TokenContract = await TokenContractFactory.deploy();
  await TokenContract.deployed();
  console.log('Contract for PKPNFT deployed to:', TokenContract.address);

  //  Deploy the router contract.
  const RouterContractFactory = await hre.ethers.getContractFactory(
    'PubkeyRouterAndPermissions'
  );
  const RouterContract = await RouterContractFactory.deploy(
    TokenContract.address
  );
  await RouterContract.deployed();
  console.log(
    'Contract for PubkeyRouterAndPermissions deployed to:',
    RouterContract.address
  );

  await TokenContract.setRouterAddress(RouterContract.address);

  //  Deploy the RateLimitNFT contract.
  const RateLimitNFTContractFactory = await hre.ethers.getContractFactory(
    'RateLimitNFT'
  );
  const RateLimitNFTContract = await RateLimitNFTContractFactory.deploy();
  await RateLimitNFTContract.deployed();
  console.log(
    'Contract for RateLimitNFT deployed to:',
    RateLimitNFTContract.address
  );

  // This is primarily useful for setting up test/dev environments.
  // The remainder of this script creates wallets for each of the nodes, deploys some ETH (for gas), and then deploys tokens.
  // With tokens in hand, the nodes can "stake & join" into the primary Staking contract and be run directly,
  // either through a script or via command line arguments.
  // During the deploy process, we also generate a config file for the scripts to use.

  // Mint some LIT Token!
  await c_LITToken.mint(await deployer.getAddress(), initialMintAmount);
  const InitialLITbalance = await c_LITToken.balanceOf(deployer.address);
  console.log('Minted amount of LIT =', InitialLITbalance.toNumber()); // this is a BigInt

  const wallets = new Array(nodeCount);
  let current_wallet;
  // Start Initializing nodes
  // in this case the nodes "own" their wallets - technically the

  for (i = 0; i < nodeCount; i++) {
    console.log('Processing Node#', i);

    nodeFileId = i;

    // Start building a configuration file!
    console.log('\n\nCreating Node.Config file.... \n\n');
    // Chain for condition storage and minting/staking contracts

    f('LIT_CHAIN_NAME = ', hre.network.name);
    f('LIT_CHAIN_ID = ', hre.network.config.chainId);
    f('LIT_CHAIN_RPC_URL =', hre.network.config.url);

    // Contract addresses (Condition Validations may also appear in other chains)
    f('\n');
    f('LIT_CONTRACT_LITTOKEN = ', c_LITToken.address);
    f('LIT_CONTRACT_STAKING = ', c_Staking.address);
    f('LIT_CONTRACT_CONDITIONVALIDATIONS = ', c_conditionValidation.address);
    f('LIT_CONTRACT_ACCESSCONTROLCONDITIONS = ', c_AccessCtlConditions.address);
    f('LIT_CONTRACT_PUBKEYROUTERANDPERMISSIONS = ', RouterContract.address);
    f('LIT_CONTRACT_PKPNFT = ', TokenContract.address);
    f('LIT_CONTRACT_RATELIMITNFT = ', RateLimitNFTContract.address);
    f('\n');

    const ipAddr = '127.0.0.1';
    const port = 7470 + i;
    const node_wallet = await hre.ethers.Wallet.createRandom().connect(
      chainProvider
    );
    console.log('Wallet Address:', node_wallet.address);

    wallets[i] = node_wallet;
    // config file entry
    f('LIT_NODE_ADDRESS = ', node_wallet.address.toLowerCase());
    f('LIT_NODE_PRIVATEKEY = ', node_wallet.privateKey);
    f('LIT_NODE_PUBLICKEY = ', node_wallet.publicKey);
    f('LIT_STAKER_ADDRESS = ', node_wallet.address.toLowerCase());
    f('LIT_NODE_DOMAIN_NAME = ', ipAddr);
    f('LIT_NODE_PORT = ', port);
    f('ROCKET_PORT = ', port);
    f('');

    // from the deployer, seed the new wallet with some  native coin to cover gas.
    var nativeTransactionReceipt = await deployer.sendTransaction({
      to: node_wallet.address,
      value: hre.ethers.utils.parseEther('0.1'), // Sends exactly 0.1 CELO coin
    });

    await nativeTransactionReceipt.wait(0);
    console.log(
      'Node Balance (native) =',
      await await chainProvider.getBalance(node_wallet.address)
    ); // this is a BigInt

    // the deployer is the minter / holder of LIT tokens
    const dLITToken = await c_LITToken.connect(deployer);
    await dLITToken.approve(node_wallet.address, nodeTransferAmount);
    await dLITToken.transfer(node_wallet.address, nodeTransferAmount);

    // approve the transfer to the contract per ERC20
    token = await c_LITToken.connect(node_wallet);
    console.log(
      'Node Balance (LIT-Approve) =',
      await token.balanceOf(node_wallet.address)
    ); // this is a BigInt
    await token.approve(c_Staking.address, staking_amount);

    var node_balance = await c_LITToken.balanceOf(node_wallet.address);
    console.log('Node Balance (LIT-initial) =', node_balance); // this is a BigInt
    console.log('Staking amount =', staking_amount); // this is a BigInt

    // stake & join for the first epoch.
    const node_staking = await c_Staking.connect(node_wallet);

    await node_staking.stakeAndJoin(
      staking_amount,
      ip2int(ipAddr),
      0,
      port,
      node_wallet.address
    );

    const post_node_balance = await c_LITToken.balanceOf(node_wallet.address);
    console.log('LITbalance (post) =', post_node_balance); // this is a BigInt

    await c_Staking.connect(node_wallet);
    const staking_contract_balance = await c_Staking.balanceOf(
      node_wallet.address
    );
    console.log('Staking balance (post) =', staking_contract_balance); // this is a BigInt

    current_wallet = node_wallet;
  }

  console.log('\n\n\nFinished creating log file.  \n');
  console.log(
    'next up validators set:',
    await c_Staking.getValidatorsInNextEpoch()
  );

  // if we want to prep our first set of nodes...  >>>
  // comment this out, if we want to run these commands from the nodes!

  // const node_staking = await c_Staking.connect(current_wallet);
  // console.log('Locking for first epoch...');
  // await node_staking.lockValidatorsForNextEpoch();

  // for (i = 0; i < nodeCount; i++)  {
  //   const wallet = wallets[i];
  //   const node_staking = await c_Staking.connect(wallet);
  //   console.log('Signal Ready for first epoch :', wallet.address );
  //   await node_staking.signalReadyForNextEpoch();
  // }
  // console.log('Advancing to first epoch...');
  // await node_staking.advanceEpoch();
  // console.log("Current validators:", await c_Staking.getValidatorsInCurrentEpoch() );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

/// test deployed to : 7.355248197 CELO ($5.617 USD) //  6.3439029795 CELO ($4.860 USD)
