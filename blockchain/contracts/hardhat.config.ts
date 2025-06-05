import '@nomicfoundation/hardhat-toolbox';
// import "hardhat-ethernal"; // required for ethernal - removing this will only break deploy scripts that are linked to ethernal .
// Commenting out lines in these deploy scripts doesn't break the deploy, only the synchronization with Ethernal - ie, safe to do.
// Tests & regular deploys continue to work normally.
// Ethernal is a web based block explorer that syncs from any EVM chain - easy way to "view" hardhat data, and execute contracts!
// https://www.tryethernal.com

import 'hardhat-contract-sizer';

import '@kingdomstudios/hardhat-diamond-abi';

// Uncomment the below to enable the tracer.
// FIXME: There is an unresolved issue whether running the npm run deploy script with
// the tracer enabled will prevent the child process from exiting.
// Issue: https://github.com/zemse/hardhat-tracer/issues/57
import 'hardhat-tracer';

import { HardhatUserConfig } from 'hardhat/types';
// import "hardhat-tracer";

import { ProviderWrapper } from 'hardhat/plugins';

// @ts-ignore
extendProvider(async (provider, config, network: string) => {
  if (network === 'lit') {
    return new OldGethProvider(provider);
  } else if (
    network === 'localchain' &&
    config.networks['localchain'].slowDownProvider
  ) {
    return new SlowerGethProvider(provider);
  }

  return provider;
});

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
// @ts-ignore
task('accounts', 'Prints the list of accounts', async () => {
  // @ts-ignore
  const accounts = await ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
const config: HardhatUserConfig = {
  solidity: {
    version: '0.8.17',
    settings: {
      outputSelection: {
        '*': {
          '*': ['storageLayout'],
        },
      },
      optimizer: {
        enabled: false,
        runs: 200,
      },
    },
  },
  networks: {
    hardhat: {
      hardfork: 'berlin',
    },
    celo: {
      url: 'https://forno.celo.org',
      ...(process.env['LIT_CELO_DEPLOYER_PRIVATE_KEY'] && {
        accounts: [process.env['LIT_CELO_DEPLOYER_PRIVATE_KEY']],
      }),
    },
    mumbai: {
      url: 'https://polygon-mumbai.g.alchemy.com/v2/onvoLvV97DDoLkAmdi0Cj7sxvfglKqDh',
      ...(process.env['LIT_MUMBAI_DEPLOYER_PRIVATE_KEY'] && {
        accounts: [process.env['LIT_MUMBAI_DEPLOYER_PRIVATE_KEY']],
      }),
    },
    alfajores: {
      url: 'https://alfajores-forno.celo-testnet.org',
      ...(process.env['LIT_ALFAJORES_DEPLOYER_PRIVATE_KEY'] && {
        accounts: [process.env['LIT_ALFAJORES_DEPLOYER_PRIVATE_KEY']],
      }),
    },
    polygon: {
      url: 'https://polygon-rpc.com',
      ...(process.env['LIT_POLYGON_DEPLOYER_PRIVATE_KEY'] && {
        accounts: [process.env['LIT_POLYGON_DEPLOYER_PRIVATE_KEY']],
      }),
    },
    yellowstone: {
      url: 'https://yellowstone-rpc.litprotocol.com',
      ...(process.env['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY'] && {
        accounts: [process.env['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY']],
      }),
      chainId: 175188,
      // @ts-ignore
      stylusContractsForTests: {
        p256:
          process.env.LIT_STYLUS_P256_CONTRACT_ADDRESS ||
          '0x8ea150155c63b3a2e34b61409fb65e19f1bd48e7',
        k256:
          process.env.LIT_STYLUS_K256_CONTRACT_ADDRESS ||
          '0x28ca4b9b360ed4f918081c921b8a299fd491e96a',
      },
      wlitAddress: '0xd78089bAAe410f5d0eae31D0D56157c73a3Ff98B',
      trustedForwarderAddress: '0x63F15BC77EA4C0b4361a879F6D16D7a5C635DB6C',
    },
    etna: {
      url: 'https://etna-testnet.rpc.caldera.xyz/http',
      ...(process.env['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY'] && {
        accounts: [process.env['LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY']],
      }),
      chainId: 175178,
      // @ts-ignore
      wlitAddress: '0xCbb5E24a8d7d1E704cA6939c0e673569030ceeC1',
    },
    localchain: {
      url: 'http://127.0.0.1:8545',
      accounts: [
        '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80', // default anvil private key,
        '0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d',
        '0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a',
        '0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6',
        '0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a',
        '0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba',
        '0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e',
        '0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356',
        '0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97',
        '0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6',
      ],
      chainId: 31337,
      // @ts-ignore
      slowDownProvider:
        (process.env['LIT_SLOW_DOWN_PROVIDER'] || 'false') === 'true',
      // @ts-ignore
      wlitAddress: '0xA51c1fc2f0D1a1b8494Ed1FE312d7C3a78Ed91C0', // this gets deployed by deploy_everything.js but because it's deterministic we know it will end up at this address.  but only if you start anvil fresh for the deploy!
    },
    localchainArbitrum: {
      url: 'http://127.0.0.1:8547',
      accounts: [
        // we use the same wallets as anvil for convenience
        '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80', // 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
        '0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d', // 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
        '0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a', // 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
        '0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6', // 0x90F79bf6EB2c4f870365E785982E1f101E93b906
        '0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a', // 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65
        '0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba', // 0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc
        '0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e', // 0x976EA74026E726554dB657fA54763abd0C3a0aa9
        '0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356', // 0x14dC79964da2C08b23698B3D3cc7Ca32193d9955
        '0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97', // 0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f
        '0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6', // 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720
      ],
      chainId: 412346,
      // @ts-ignore
      stylusContractsForTests: {
        p256: process.env.LIT_STYLUS_P256_CONTRACT_ADDRESS,
        k256: process.env.LIT_STYLUS_K256_CONTRACT_ADDRESS,
      },
    },
    sepolia: {
      url: 'https://sepolia.infura.io/v3/ddf1ca3700f34497bca2bf03607fde38',
      ...(process.env['LIT_SEPOLIA_DEPLOYER_PRIVATE_KEY'] && {
        accounts: [process.env['LIT_SEPOLIA_DEPLOYER_PRIVATE_KEY']],
      }),
      chainId: 11155111,
    },
  },
  etherscan: {
    apiKey: {
      celo: process.env.LIT_CELOSCAN_API_KEY!,
      mumbai: process.env.LIT_POLYGONSCAN_API_KEY!,
      polygon: process.env.LIT_POLYGONSCAN_API_KEY!,
      yellowstone: 'woof', // Blockscout does not require an API key but needs the string to be populated
      sepolia: process.env.LIT_ETHERSCAN_API_KEY!,
    },
    customChains: [
      {
        network: 'celo',
        chainId: 42220,
        urls: {
          apiURL: 'https://api.celoscan.io/api',
          browserURL: 'https://celoscan.io',
        },
      },
      {
        network: 'alfajores',
        chainId: 44787,
        urls: {
          apiURL: 'https://api-alfajores.celoscan.io/api',
          browserURL: 'https://alfajores.celoscan.io/',
        },
      },
      {
        network: 'mumbai',
        chainId: 80001,
        urls: {
          apiURL: 'https://api-testnet.polygonscan.com/api',
          browserURL: 'https://mumbai.polygonscan.com',
        },
      },
      {
        network: 'polygon',
        chainId: 137,
        urls: {
          apiURL: 'https://api.polygonscan.com/api',
          browserURL: 'https://polygonscan.com',
        },
      },
      {
        network: 'yellowstone',
        chainId: 175188,
        urls: {
          apiURL: 'https://yellowstone-explorer.litprotocol.com/api',
          browserURL: 'https://yellowstone-explorer.litprotocol.com/',
        },
      },
      {
        network: 'etna',
        chainId: 175178,
        urls: {
          apiURL: 'https://etna-testnet.explorer.caldera.xyz/api',
          browserURL: 'https://etna-testnet.explorer.caldera.xyz/',
        },
      },
    ],
  },
  tenderly: {
    project: 'litnodecontracts',
    username: 'rwiggum',
    privateVerification: false,
  },
  diamondAbi: [
    {
      // (required) The name of your Diamond ABI.
      name: 'StakingDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'StakingFacet',
        'StakingViewsFacet',
        'StakingVersionFacet',
        'StakingAdminFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacet',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'PKPPermissionsDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'PKPPermissionsFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacet',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'PubkeyRouterDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'PubkeyRouterFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacet',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'RateLimitNFTDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'RateLimitNFTFacet',
        'RateLimitNFTViewsFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacetNoERC165',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'StakingBalancesDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'StakingBalancesFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacet',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'PKPNFTDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'PKPNFTFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacetNoERC165',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'BackupRecoveryDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'BackupRecoveryFacet',
        'BackupRecoveryTestFacet',
        'BackupRecoveryNodeStatusFacet',
        'BackupRecoveryViewFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacetNoERC165',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'DomainWalletRegistryDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'DomainWalletRegistryFacet',
        'DomainWalletRegistryViewsFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacet',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'PaymentDelegationDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'PaymentDelegationFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacet',
      ],
    },
    {
      // (required) The name of your Diamond ABI.
      name: 'CloneNetDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: [
        'OwnershipFacet',
        'CloneNetFacet',
        'DiamondCutFacet',
        'DiamondLoupeFacet',
      ],
    },
    {
      name: 'HostCommandsDiamond',
      // (optional) An array of strings, matched against fully qualified contract names, to
      // determine which contracts are included in your Diamond ABI.
      include: ['HostCommandsFacet', 'DiamondCutFacet', 'DiamondLoupeFacet'],
    },
  ],
  mocha: {
    timeout: 60000,
  },
};

export default config;

/**
 * This provider implements workarounds for old geth versions that do not support:
 * - An additional parameter in eth_estimateGas, so we strip it out.
 */
class OldGethProvider extends ProviderWrapper {
  constructor(protected readonly _wrappedProvider: any) {
    super(_wrappedProvider);

    console.info('Using OldGethProvider');
  }

  public async request(args: any) {
    if (args.method === 'eth_estimateGas') {
      const adjustedRequest = {
        ...args,
        params: args.params.slice(0, args.params.length - 1),
      };
      return this._wrappedProvider.request(adjustedRequest);
    }

    return this._wrappedProvider.request(args);
  }
}

class SlowerGethProvider extends ProviderWrapper {
  constructor(protected readonly _wrappedProvider: any) {
    super(_wrappedProvider);

    console.info('Using SlowerGethProvider');
  }

  public async request(args: any) {
    await new Promise((resolve) => setTimeout(resolve, 50));
    return this._wrappedProvider.request(args);
  }
}
