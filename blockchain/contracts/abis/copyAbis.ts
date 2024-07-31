// npx ts-node abis/copyAbis.ts

import { ParsedNodeContracts } from '../scripts/deployConfig';

const fs = require('fs');
const path = require('path');

const BASE_ABI_PATH = './artifacts/contracts';
const COPY_DIR_PATH = './';
const CONTRACTS_DW = ['DomainWalletRegistry'];
const CONTRACTS_NODES = [
  'PKPHelper',
  'PKPHelperV2',
  'PKPNFTMetadata',
  'PKPNFT',
  'PKPPermissions',
  'LITToken',
];
const CONTRACTS_CORE = ['ContractResolver'];
const DEPLOYED_CONTRACTS =
  process.argv[2] === 'test'
    ? './deployed-dw-test-contracts.json'
    : './deployed-lit-node-contracts-temp.json';

const NAME_TO_KEY_MAP = {
  BackupRecovery: 'backupREcoveryContractAddress',
  PKPNFT: 'pkpNftContractAddress',
  PKPPermissions: 'pkpPermissionsContractAddress',
  PaymentDelegation: 'paymentDelegationContractAddress',
  PubkeyRouter: 'pubkeyRouterContractAddress',
  RateLimitNFT: 'rateLimitNftContractAddress',
  DomainWalletRegistry: 'domainWalletRegistryAddress',
  ContractResolver: 'resolverContractAddress',
  LITToken: 'LITTokenContractAddress',
};

const DIAMOND_CONTRACTS_TO_FACETS = {
  BackupRecovery: [
    'BackupRecoveryFacet',
    'BackupRecoveryNodeStatusFacet',
    'BackupRecoveryViewFacet',
  ],
  PKPNFT: ['PKPNFTFacet'],
  PKPPermissions: ['PKPPermissionsFacet'],
  PaymentDelegation: ['PaymentDelegationFacet'],
  PubkeyRouter: ['PubkeyRouterFacet'],
  RateLimitNFT: ['RateLimitNFTFacet', 'RateLimitNFTViewsFacet'],
  DomainWalletRegistry: [
    'DomainWalletRegistryFacet',
    'DomainWalletRegistryViewsFacet',
  ],
};

function main() {
  console.info('looking for deployment addresses at: ', DEPLOYED_CONTRACTS);
  if (!fs.existsSync(path.join(COPY_DIR_PATH, 'abis'))) {
    console.log('creating abis directory...');
    fs.mkdirSync(path.join(COPY_DIR_PATH, 'abis'));
  }

  const deployedContracts: ParsedNodeContracts = JSON.parse(
    fs.readFileSync(DEPLOYED_CONTRACTS).toString('utf8')
  );

  for (const contract of CONTRACTS_DW) {
    const abiPath = path.join(
      BASE_ABI_PATH,
      'domain-wallets',
      contract + '.sol',
      contract + '.json'
    );
    updateAbi(abiPath, contract, deployedContracts);
    copyAbi(abiPath, COPY_DIR_PATH, contract);

    if (Object.keys(DIAMOND_CONTRACTS_TO_FACETS).includes(contract)) {
      // This is a diamond contract. Enumerate and copy the facets.
      for (const diamondFacet of DIAMOND_CONTRACTS_TO_FACETS[
        contract as keyof typeof DIAMOND_CONTRACTS_TO_FACETS
      ]) {
        const facetContract = diamondFacet;
        const facetAbiPath = path.join(
          BASE_ABI_PATH,
          'domain-wallets',
          contract,
          facetContract + '.sol',
          facetContract + '.json'
        );
        updateAbi(facetAbiPath, contract, deployedContracts);
        copyAbi(facetAbiPath, COPY_DIR_PATH, facetContract);
      }
    }
  }

  for (const contract of CONTRACTS_NODES) {
    const abiPath = path.join(
      BASE_ABI_PATH,
      'lit-node',
      contract + '.sol',
      contract + '.json'
    );
    updateAbi(abiPath, contract, deployedContracts);
    copyAbi(abiPath, COPY_DIR_PATH, contract);

    if (Object.keys(DIAMOND_CONTRACTS_TO_FACETS).includes(contract)) {
      // This is a diamond contract. Enumerate and copy the facets.
      for (const diamondFacet of DIAMOND_CONTRACTS_TO_FACETS[
        contract as keyof typeof DIAMOND_CONTRACTS_TO_FACETS
      ]) {
        const facetContract = diamondFacet;
        const facetAbiPath = path.join(
          BASE_ABI_PATH,
          'lit-node',
          contract,
          facetContract + '.sol',
          facetContract + '.json'
        );
        updateAbi(facetAbiPath, contract, deployedContracts);
        copyAbi(facetAbiPath, COPY_DIR_PATH, facetContract);
      }
    }
  }

  for (const contract of CONTRACTS_CORE) {
    const abiPath = path.join(
      BASE_ABI_PATH,
      'lit-core',
      contract + '.sol',
      contract + '.json'
    );
    updateAbi(abiPath, contract, deployedContracts);
    copyAbi(abiPath, COPY_DIR_PATH, contract);

    if (Object.keys(DIAMOND_CONTRACTS_TO_FACETS).includes(contract)) {
      // This is a diamond contract. Enumerate and copy the facets.
      for (const diamondFacet of DIAMOND_CONTRACTS_TO_FACETS[
        contract as keyof typeof DIAMOND_CONTRACTS_TO_FACETS
      ]) {
        const facetContract = diamondFacet;
        const facetAbiPath = path.join(
          BASE_ABI_PATH,
          'lit-core',
          contract,
          facetContract + '.sol',
          facetContract + '.json'
        );
        updateAbi(facetAbiPath, contract, deployedContracts);
        copyAbi(facetAbiPath, COPY_DIR_PATH, facetContract);
      }
    }
  }

  console.log('done copying abi files');
}

function updateAbi(
  abiPath: string,
  contractName: string,
  deployedContracts: ParsedNodeContracts
) {
  let abi = fs.readFileSync(abiPath);
  abi = abi.toString('utf8');
  abi = JSON.parse(abi);
  abi.address =
    deployedContracts[
      NAME_TO_KEY_MAP[
        contractName as keyof typeof NAME_TO_KEY_MAP
      ] as keyof ParsedNodeContracts
    ];
  fs.writeFileSync(abiPath, JSON.stringify(abi, null, 2));
}

function copyAbi(abiPath: string, copyDestDir: string, contractName: string) {
  console.debug('copying abi at ', abiPath);
  fs.copyFileSync(
    path.join(abiPath),
    path.join(copyDestDir, 'abis', contractName + '.json')
  );
}

main();
