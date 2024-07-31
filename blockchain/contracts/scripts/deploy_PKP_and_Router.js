// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
const hre = require('hardhat');

async function main() {
  // Hardhat always runs the compile task when running scripts with its command
  // line interface.
  //
  // If this script is run directly using `node` you may want to call compile
  // manually to make sure everything is compiled
  // await hre.run('compile');

  // this can be removed if Ethernal isn't being used - it's handy to see validations.
  // Ethernal set up instructions:  https://www.tryethernal.com
  // In the hardhat.config.js file >>>  require('hardhat-ethernal');
  // hre.ethernalUploadAst = true; // clean and reset the workspace - ie, forget about old contracts, and start fresh.
  // await hre.ethernal.resetWorkspace("LocalHardHat"); /// <<< your ethernal project name.

  let RouterContractFactory;
  let TokenContractFactory;
  let TokenContract;

  // make this a public key that the nodes have generated.
  let pubkey =
    '0x034319b040a81f78d14b8efcf73f3120b28d88ac5ca316dbd1a83797defcf20b5a';

  RouterContractFactory = await ethers.getContractFactory(
    'PubkeyRouterAndPermissions'
  );
  TokenContractFactory = await ethers.getContractFactory('PKPNFT');

  [deployer, holder, ...signers] = await ethers.getSigners();

  // deploy the PKP contract
  TokenContract = await TokenContractFactory.deploy();
  await TokenContract.deployed();
  console.log('Contract for PKPNFT deployed to:', TokenContract.address);

  await TokenContract.connect(deployer);

  // Deploy the router contract.
  routerContract = await RouterContractFactory.deploy(TokenContract.address);
  await routerContract.deployed();
  console.log(
    'Contract for PubkeyRouterAndPermissions deployed to:',
    routerContract.address
  );

  // mint a token
  const txn = await TokenContract.mint(pubkey);
  const tx = await txn.wait();
  const event = tx.events[0];
  const tokenId = event.args[2];
  const token_address = event.address;

  // transfer the token to a known wallet :
  await TokenContract.transfer(deployer.address, holder.address, tokenId); // In this case using HardHat Accounnt #1

  const pubkeyHash = ethers.utils.keccak256(pubkey);

  // validate that it's unset
  const [keyPart1, keyPart2, keyLength, stakingContract, keyType] =
    await routerContract.getRoutingData(pubkeyHash);

  // set routing data
  const keyPart1Bytes = ethers.utils.hexDataSlice(pubkeyHash, 0, 32);
  const keyPart2Bytes = ethers.utils.hexZeroPad(
    ethers.utils.hexDataSlice(pubkeyHash, 32),
    32
  );

  const stakingContractAddress = '0x6cB3Cd5064692ac8e3368A1d6C29b36aE1143dF7'; // a random address, since we haven't tied it in but it's a required field - dependency for go-live!
  const keyLengthInput = 48;
  const keyTypeInput = 1;

  await routerContract.setRoutingData(
    pubkeyHash,
    keyPart1Bytes,
    keyPart2Bytes,
    keyLengthInput,
    stakingContractAddress,
    keyTypeInput
  );

  // validate that it was set
  const [
    keyPart1After,
    keyPart2After,
    keyLengthAfter,
    stakingContractAfter,
    keyTypeAfter,
  ] = await routerContract.getRoutingData(pubkeyHash);

  console.log('Confirming that we have storage: ', keyPart1After);

  // Get contract ASTs into ethernal
  // Doing this at the end because it can be a bit slow, and we can validate other things at the same time.
  // await hre.ethernal.push({
  //     name: "PubkeyRouterAndPermissions",
  //     address: routerContract.address,
  // });
  // await hre.ethernal.push({ name: "PKPNFT", address: TokenContract.address });
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
