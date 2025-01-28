// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the

import {
  DeployCoreConfig,
  DeployCoreOutput,
  DeployEnvironment,
  DEPLOY_LIT_CORE_OUTPUT_TEMP_FILE_PATH,
} from './deployConfig';
import {
  hardhatDeployAndVerifySingleContract,
  mapEnvToEnum,
  verifyContractInBg,
} from './utils';

// global scope, and execute the script.
import hre from 'hardhat';
import fs from 'fs';
import { deployDiamondContract } from './deployDiamond';
const { ethers } = hre;

export const grantRolesTo = async (
  contract: any,
  address: string,
  roles: any[]
) => {
  let txs = [];
  for (let i = 0; i < roles.length; i++) {
    txs.push(await contract.grantRole(roles[i], address));
  }

  await Promise.all(txs);
};

export async function deployLitCoreContracts(
  deployCoreConfig: DeployCoreConfig
): Promise<DeployCoreOutput> {
  const [deployer] = await ethers.getSigners();
  const chainName = deployCoreConfig.networkName;

  console.log('Deploying contracts to network ' + chainName);

  // *** 1. Deploy ContractResolver
  const contractResolver = await hardhatDeployAndVerifySingleContract(
    ethers,
    deployCoreConfig.networkName,
    'ContractResolver',
    {
      deploymentArgs: [mapEnvToEnum(deployCoreConfig.environment)],
    }
  );

  // *** 2.1 Deploy ReleaseRegister
  const releaseRegister = await hardhatDeployAndVerifySingleContract(
    ethers,
    deployCoreConfig.networkName,
    'ReleaseRegister',
    {
      deploymentArgs: [mapEnvToEnum(deployCoreConfig.environment)],
    }
  );

  // *** 2.2 Set roles for ReleaseRegister
  console.log('Setting roles for ReleaseRegister contract...');

  await grantRolesTo(releaseRegister, deployCoreConfig.subnetOwnerAddress, [
    await releaseRegister.ADMIN_ROLE(),
    await releaseRegister.CREATOR_ROLE(),
    await releaseRegister.ACTIVATOR_ROLE(),
    await releaseRegister.DEACTIVATOR_ROLE(),
    await releaseRegister.BURNER_ROLE(),
  ]);
  await grantRolesTo(releaseRegister, deployCoreConfig.subnetProvAddress, [
    await releaseRegister.CREATOR_ROLE(),
    await releaseRegister.ACTIVATOR_ROLE(),
  ]);

  // *** 2.3 Adding admin public keys to ReleaseRegister
  console.log(
    'Adding allowed admin public keys to ReleaseRegister contract...'
  );

  await releaseRegister.addAllowedAdminSigningPublicKey(
    deployCoreConfig.subnetAdminPublicKey
  );

  // *** 3.1 Deploy HostCommands Contract
  console.log('Deploying HostCommands contract');
  const hostCommandsDiamond = await deployDiamondContract(
    chainName,
    'HostCommands',
    [
      await contractResolver.getAddress(),
      mapEnvToEnum(deployCoreConfig.environment),
    ],
    ['HostCommandsFacet'],
    false
  );
  // *** 3.2 Set the contract deployer to be authorized Sender by default
  const hostCommandsContract = await ethers.getContractAt(
    'HostCommandsFacet',
    await hostCommandsDiamond.diamond.getAddress()
  );
  let set_authorized_sender_tx =
    await hostCommandsContract.setAuthorizedCommandSender(deployer.address);
  await set_authorized_sender_tx.wait();

  // *** 6. Set the active contract address
  console.log('Setting active contract addresses');

  await contractResolver.setContract(
    await contractResolver.RELEASE_REGISTER_CONTRACT(),
    mapEnvToEnum(deployCoreConfig.environment),
    await releaseRegister.getAddress()
  );
  await contractResolver.setContract(
    await contractResolver.HOST_COMMANDS_CONTRACT(),
    mapEnvToEnum(deployCoreConfig.environment),
    await hostCommandsDiamond.diamond.getAddress()
  );

  // *** 7. Renouncing ADMIN role.
  await releaseRegister.renounceRole(
    await releaseRegister.ADMIN_ROLE(),
    deployer.address
  ); // Lock the release register.
  console.log('Renouncing ADMIN role for deployer in release register');

  const finalJson = {
    contractResolver: await contractResolver.getAddress(),
    releaseRegisterContractAddress: await releaseRegister.getAddress(),
    hostCommandsContractAddress: await hostCommandsDiamond.diamond.getAddress(),
  };

  console.log('final JSON: ');
  console.log(JSON.stringify(finalJson, null, 2));

  // *** 5. Write to file
  const fileName = DEPLOY_LIT_CORE_OUTPUT_TEMP_FILE_PATH;
  console.log('Writing to file: ' + fileName);
  fs.writeFileSync(fileName, JSON.stringify(finalJson, null, 2));

  return finalJson;
}
