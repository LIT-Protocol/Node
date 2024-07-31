export * from './models';
export { askDeployConfig } from './askDeployConfig';
export { getDeployConfig } from './getDeployConfig';
export * from './common';

export const DEPLOY_CONFIGS_DIR = './scripts/deployConfig/configs';
export const DEPLOY_CONFIGURATION_PATTERN = 'deploy-config-*.json';
export const DEPLOY_LIT_CORE_OUTPUT_TEMP_FILE_PATH =
  './deployed-lit-core-contracts-temp.json';
export const DEPLOY_LIT_NODE_OUTPUT_TEMP_FILE_PATH =
  './deployed-lit-node-contracts-temp.json';
