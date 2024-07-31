/* global ethers */
/* eslint prefer-const: "off" */

const hre = require('hardhat');
const { hardhatDeployAndVerifySingleContract } = require('./utils');
const chainName = hre.network.name;
const { grantRolesTo } = require('./deployLitCore');

async function setRoles(
  releaseRegister,
  { subnetOwnerAddress, subnetProvAddress, subnetAdminPublicKey }
) {
  console.log('Setting roles for ReleaseRegister contract...');

  await grantRolesTo(releaseRegister, subnetOwnerAddress, [
    await releaseRegister.ADMIN_ROLE(),
    await releaseRegister.CREATOR_ROLE(),
    await releaseRegister.ACTIVATOR_ROLE(),
    await releaseRegister.DEACTIVATOR_ROLE(),
    await releaseRegister.BURNER_ROLE(),
  ]);
  await grantRolesTo(releaseRegister, subnetProvAddress, [
    await releaseRegister.CREATOR_ROLE(),
    await releaseRegister.ACTIVATOR_ROLE(),
  ]);

  console.log(
    'Adding allowed admin public keys to ReleaseRegister contract...'
  );

  await releaseRegister.addAllowedAdminSigningPublicKey(subnetAdminPublicKey);

  console.log('Done!');
}

async function deployContractAndSetRoles(
  contractName,
  args = [],
  releaseRegisterKeys
) {
  const contract = await hardhatDeployAndVerifySingleContract(
    hre.ethers,
    hre.network.name,
    contractName,
    {
      deploymentArgs: args,
    }
  );
  await setRoles(contract, releaseRegisterKeys);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
if (require.main === module) {
  // regular contract
  deployContractAndSetRoles('ReleaseRegister', [2n], {
    subnetOwnerAddress: '0x67EE5dDE3a1b522176A3676D7f74c759dFb98951',
    subnetProvAddress: '0x0E718ca881D47f942983DdA74A2Fe2b8D884f98d',
    subnetAdminPublicKey:
      '0x047f56f199ca45f3ffe645191975ffc3acf2e3bce83cba7e5a07d412d6097ee940fb16a24ff0ce7e69ad8d646e218aa6daf385634792f21ac6aa4b76d359e576d0',
  }); // 2n is the env == prod
}
