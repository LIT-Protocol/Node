export const CONTRACT_NAME_TO_JSON_CONTRACT_ADDRESS_KEY = {
  Staking: 'stakingContractAddress',
  Multisender: 'multisenderContractAddress',
  LITToken: 'litTokenContractAddress',
  AccessControlConditions: 'accessControlConditionsContractAddress',
  PubkeyRouter: 'pubkeyRouterContractAddress',
  PKPNFT: 'pkpNftContractAddress',
  RateLimitNFT: 'rateLimitNftContractAddress',
  PKPHelper: 'pkpHelperContractAddress',
  PKPPermissions: 'pkpPermissionsContractAddress',
  PKPNFTMetadata: 'pkpNftMetadataContractAddress',
  Allowlist: 'allowlistContractAddress',
  ContractResolver: 'resolverContractAddress',
  DomainWalletRegistry: 'domainWalletRegistryAddress',
  KeyDeriver: 'KeyDeriverAddress',
  PaymentDelegation: 'paymentDelegationAddress',
};

export const CONTRACT_NAME_TO_DIAMOND_ABI_PATH = {
  Staking:
    '../artifacts/hardhat-diamond-abi/StakingDiamond.sol/StakingDiamond.json',
  PKPPermissions:
    '../artifacts/hardhat-diamond-abi/PKPPermissionsDiamond.sol/PKPPermissionsDiamond.json',
  PubkeyRouter:
    '../artifacts/hardhat-diamond-abi/PubkeyRouterDiamond.sol/PubkeyRouterDiamond.json',
  RateLimitNFT:
    '../artifacts/hardhat-diamond-abi/RateLimitNFTDiamond.sol/RateLimitNFTDiamond.json',
  StakingBalances:
    '../artifacts/hardhat-diamond-abi/StakingBalancesDiamond.sol/StakingBalancesDiamond.json',
  PKPNFT:
    '../artifacts/hardhat-diamond-abi/PKPNFTDiamond.sol/PKPNFTDiamond.json',
  DomainWalletRegistry:
    '../artifacts/hardhat-diamond-abi/DomainWalletRegistryDiamond.sol/DomainWalletRegistryDiamond.json',
  PaymentDelegation:
    '../artifacts/hardhat-diamond-abi/PaymentDelegationDiamond.sol/PaymentDelegationDiamond.json',
};
