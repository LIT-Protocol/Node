import { Wallet } from 'ethers';

export enum DeployEnvironment {
  DEV = 'dev',
  STAGING = 'staging',
  PROD = 'prod',
}

export interface DeploySensitiveConfig {
  ipfsApiKey: string;
}

export interface DeployBaseConfig {
  environment: DeployEnvironment;
  networkName: string;
}

export interface DeployNodeConfig extends DeployBaseConfig {
  newOwnerAddress: string;
  newDomainWalletAdminAddress?: string;
  numberOfStakedOnlyWallets: number;
  numberOfStakedAndJoinedWallets: number;
  useLitCoreDeploymentResolverContractAddress: boolean;
  outputTempFilePath: string;
  resolverContractAddress: string;
  copyNodeConfigsToRustProject: boolean;
  ipAddresses?: string[];
  existingContracts?: Partial<ParsedNodeContracts>;
  keyTypes: number[];
  nodePrivateKeys?: string[];
  chainPollingInterval?: string;
  customNodeRuntimeConfigPath?: string;
  backupRecoveryAddresses?: string[];
  backupRecoveryKeys?: string[];
}

export interface DeployCoreConfig extends DeployBaseConfig {
  subnetAdminPublicKey: string;
  subnetOwnerAddress: string;
  subnetProvAddress: string;
  outputTempFilePath: string;
}

export interface DeployCoreOutput {
  contractResolver: string;
  releaseRegisterContractAddress: string;
}

export interface FundAndStakeNodesOutput {
  nodeOperatorsCredentials: Array<NodeOperatorCredentials>;
  contracts: ParsedNodeContracts;
}

export enum DeploymentSelection {
  LIT_CORE_AND_LIT_NODE = 'lit-core + lit-node',
  LIT_CORE = 'lit-core',
  LIT_NODE = 'lit-node',
}

export interface DeployFullConfig {
  deployCoreConfig: DeployCoreConfig;
  deployNodeConfig?: DeployNodeConfig;
  deploymentSelection: DeploymentSelection;
}

export interface ParsedDomainWalletContracts {
  domainWalletRegistryAddress: string;
}

export interface ParsedNodeContracts extends ParsedDomainWalletContracts {
  stakingBalancesContractAddress: string;
  stakingContractAddress: string;
  cloneNetContractAddress: string;
  multisenderContractAddress: string;
  litTokenContractAddress: string;
  pubkeyRouterContractAddress: string;
  pkpNftContractAddress: string;
  rateLimitNftContractAddress: string;
  pkpHelperContractAddress: string;
  pkpPermissionsContractAddress: string;
  pkpNftMetadataContractAddress: string;
  allowlistContractAddress: string;
  resolverContractAddress: string;
  stylusContractP256Address?: string;
  stylusContractK256Address?: string;
  chainId: number;
  rpcUrl: string;
  chainName: string;
  litNodeDomainName: string;
  litNodePort: number;
  rocketPort: number;
}

export interface ComsKeys {
  publicKey: string;
  privateKey: string;
}

export interface NodeOperatorCredentials {
  nodeWallet: Wallet;
  stakerWallet: Wallet;
  comsKeysSender: ComsKeys;
  comsKeysReceiver: ComsKeys;
}
