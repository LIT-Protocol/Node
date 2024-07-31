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

  const [deployer, holder, ...signers] = await ethers.getSigners();

  const token = await ethers.deployContract('LITToken', [
    ethers.parseUnits('1000000000', 18), // 1 billion
  ]);

  const address = await token.getAddress();
  console.log('Deployed at address: ', address);

  // mint 100m tokens to the deployer
  await token.mint(
    await deployer.getAddress(),
    ethers.parseUnits('100000000', 18)
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
