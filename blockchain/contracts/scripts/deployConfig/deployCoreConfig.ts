import { DEPLOY_LIT_CORE_OUTPUT_TEMP_FILE_PATH } from '.';
import {
  askForConfirm,
  askForConfirmParameter,
  askForNetworkNameConfirmation,
} from './common';
import { DeployCoreConfig, DeployEnvironment } from './models';

export async function askDeployCoreConfig(
  environment: DeployEnvironment,
  networkName: string
): Promise<DeployCoreConfig> {
  await askForNetworkNameConfirmation(networkName);
  const ownerAddress = await getOwnerAddress(environment);
  const adminPublicKey = await getAdminPublicKey(environment);
  const provAddress = await getProvAddress(environment);

  const deployConfig: DeployCoreConfig = {
    environment,
    networkName,
    subnetOwnerAddress: ownerAddress,
    subnetAdminPublicKey: adminPublicKey,
    subnetProvAddress: provAddress,
    outputTempFilePath: DEPLOY_LIT_CORE_OUTPUT_TEMP_FILE_PATH,
  };

  return deployConfig;
}

const getOwnerAddress = async (
  environment: DeployEnvironment
): Promise<string> => {
  if (process.env.LIT_SUBNET_OWNER_ADDRESS) {
    return askForConfirmParameter(
      'subnetOwnerAddress',
      process.env.LIT_SUBNET_OWNER_ADDRESS,
      'the subnet owner address'
    );
  }

  switch (environment) {
    case DeployEnvironment.DEV:
      return '0xB77AEBbC262Bb809933D991A919A0e4A6A3b2f65';
    case DeployEnvironment.STAGING:
      return '0x8660Fa1d5Ca425B5098830cf6d6E343C76a5eb45';
    case DeployEnvironment.PROD:
      throw new Error(
        'owner for prod not implemented (use LIT_SUBNET_OWNER_ADDRESS)'
      );
    default:
      throw new Error('ENV is invalid');
  }
};

const getAdminPublicKey = async (
  environment: DeployEnvironment
): Promise<string> => {
  if (process.env.LIT_SUBNET_ADMIN_PUBLIC_KEY) {
    return askForConfirmParameter(
      'subnetAdminPublicKey',
      process.env.LIT_SUBNET_ADMIN_PUBLIC_KEY,
      'the subnet admin public key'
    );
  }

  switch (environment) {
    case DeployEnvironment.DEV:
      return '0x045f96e860435fccf287d9c2592fa129edfca7159c8dd2260cf2def38a9d5ee627ba73afef636467bc95fe551f10c862e910f18eafb751226d6901eab7d5b2794a';
    case DeployEnvironment.STAGING:
      return '0x04befba05710dc3eed6508a31d9515a80a5c036c9a165c2db19eb44fc3dceedad44572aa11c832ed4e4e98fa087017c950232ad5810a923599d724fc9d292e356d';
    case DeployEnvironment.PROD:
      throw new Error(
        'admin public key for prod not implemented (use LIT_SUBNET_ADMIN_PUBLIC_KEY)'
      );
    default:
      throw new Error('ENV is invalid');
  }
};

const getProvAddress = async (
  environment: DeployEnvironment
): Promise<string> => {
  if (process.env.LIT_SUBNET_PROV_ADDRESS) {
    return askForConfirmParameter(
      'subnetProvAddress',
      process.env.LIT_SUBNET_PROV_ADDRESS,
      'the subnet prov address'
    );
  }

  switch (environment) {
    case DeployEnvironment.DEV:
      return '0x3324439C8b9181eF07D54030E32d2CD22FF0C6A7';
    case DeployEnvironment.STAGING:
      return '0xAd50f87Ea7d17D1a830AEC612751155726d8854F';
    case DeployEnvironment.PROD:
      throw new Error(
        'prov for prod not implemented (use LIT_SUBNET_PROV_ADDRESS)'
      );
    default:
      throw new Error('ENV is invalid');
  }
};
