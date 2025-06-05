# Lit Node

## Setup

Before you do anything else, you'll need to setup the project. This will check out any dependent repos in the parent directory as well as perform any other tasks neccessary.

```shell
make setup-local-files
```

## Prerequisites for building

- Nodejs >= 20
- Rust >= 1.77

On Linux:

```shell
sudo apt install libudev-dev libsqlite3-dev
```

## Running a local 3 node network on Yellowstone

1. Deploy the contracts. More docs are available [here](https://github.com/LIT-Protocol/lit-assets/blob/develop/blockchain/contracts/README.md#deploying)
   1. cd into the lit-assets/blockchain/contracts folder.
   2. Run `npm i`
   3. Run `npm i -g concurrently`
   4. Get an API key from https://pinata.cloud, and a private key with tokens on Yellowstone. If you need tokens, ask someone at Lit.
   5. Copy .env.example to .env and set the appropriate env vars based on the info you gathered above.
   6. Run the deploy script:
   ```shell
   npx ts-node scripts/deploy.ts --deploy-config scripts/deployConfig/configs/three-local-nodes.json
   ```
2. Run the node itself. Go to lit-assets/rust/lit-node and run `./scripts/start_dev.sh 3` were 3 is the number of nodes you will run. This will spin up a local network of nodes and print logs from the first one.
3. You are now ready to use your local network. The nodes live at http://localhost:7470, http://localhost:7471, and http://localhost:7472. You can use these to interact with your local network.
4. To use this network with the JS SDK, you need to go and find the contract resolver address. You can find this in the lit-assets/blockchain/contracts/deployed-lit-core-contracts-temp.json in the `contractResolver` key. Replace it below. A JS SDK config that will work, and can be supplied to the LitNodeClient constructor, is below:

```js
const contractContext = {
  resolverAddress: '0x6a914884fb96EE0C0147Cee34Bd52B7174cFac69',
  abi: [
    {
      inputs: [
        {
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
      ],
      stateMutability: 'nonpayable',
      type: 'constructor',
    },
    { inputs: [], name: 'AdminRoleRequired', type: 'error' },
    {
      anonymous: false,
      inputs: [
        {
          indexed: false,
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
      ],
      name: 'AllowedEnvAdded',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: false,
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
      ],
      name: 'AllowedEnvRemoved',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'bytes32',
          name: 'role',
          type: 'bytes32',
        },
        {
          indexed: true,
          internalType: 'bytes32',
          name: 'previousAdminRole',
          type: 'bytes32',
        },
        {
          indexed: true,
          internalType: 'bytes32',
          name: 'newAdminRole',
          type: 'bytes32',
        },
      ],
      name: 'RoleAdminChanged',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'bytes32',
          name: 'role',
          type: 'bytes32',
        },
        {
          indexed: true,
          internalType: 'address',
          name: 'account',
          type: 'address',
        },
        {
          indexed: true,
          internalType: 'address',
          name: 'sender',
          type: 'address',
        },
      ],
      name: 'RoleGranted',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: true,
          internalType: 'bytes32',
          name: 'role',
          type: 'bytes32',
        },
        {
          indexed: true,
          internalType: 'address',
          name: 'account',
          type: 'address',
        },
        {
          indexed: true,
          internalType: 'address',
          name: 'sender',
          type: 'address',
        },
      ],
      name: 'RoleRevoked',
      type: 'event',
    },
    {
      anonymous: false,
      inputs: [
        {
          indexed: false,
          internalType: 'bytes32',
          name: 'typ',
          type: 'bytes32',
        },
        {
          indexed: false,
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
        {
          indexed: false,
          internalType: 'address',
          name: 'addr',
          type: 'address',
        },
      ],
      name: 'SetContract',
      type: 'event',
    },
    {
      inputs: [],
      name: 'ADMIN_ROLE',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'ALLOWLIST_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'BACKUP_RECOVERY_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'DEFAULT_ADMIN_ROLE',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'DOMAIN_WALLET_ORACLE',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'DOMAIN_WALLET_REGISTRY',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'HD_KEY_DERIVER_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'LIT_TOKEN_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'MULTI_SENDER_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'PKP_HELPER_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'PKP_NFT_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'PKP_NFT_METADATA_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'PKP_PERMISSIONS_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'PUB_KEY_ROUTER_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'RATE_LIMIT_NFT_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'RELEASE_REGISTER_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'STAKING_BALANCES_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [],
      name: 'STAKING_CONTRACT',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
      ],
      name: 'addAllowedEnv',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        { internalType: 'bytes32', name: 'typ', type: 'bytes32' },
        {
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
      ],
      name: 'getContract',
      outputs: [{ internalType: 'address', name: '', type: 'address' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [{ internalType: 'bytes32', name: 'role', type: 'bytes32' }],
      name: 'getRoleAdmin',
      outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [
        { internalType: 'bytes32', name: 'role', type: 'bytes32' },
        { internalType: 'address', name: 'account', type: 'address' },
      ],
      name: 'grantRole',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        { internalType: 'bytes32', name: 'role', type: 'bytes32' },
        { internalType: 'address', name: 'account', type: 'address' },
      ],
      name: 'hasRole',
      outputs: [{ internalType: 'bool', name: '', type: 'bool' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [
        {
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
      ],
      name: 'removeAllowedEnv',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        { internalType: 'bytes32', name: 'role', type: 'bytes32' },
        { internalType: 'address', name: 'account', type: 'address' },
      ],
      name: 'renounceRole',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        { internalType: 'bytes32', name: 'role', type: 'bytes32' },
        { internalType: 'address', name: 'account', type: 'address' },
      ],
      name: 'revokeRole',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [{ internalType: 'address', name: 'newAdmin', type: 'address' }],
      name: 'setAdmin',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [
        { internalType: 'bytes32', name: 'typ', type: 'bytes32' },
        {
          internalType: 'enum ContractResolver.Env',
          name: 'env',
          type: 'uint8',
        },
        { internalType: 'address', name: 'addr', type: 'address' },
      ],
      name: 'setContract',
      outputs: [],
      stateMutability: 'nonpayable',
      type: 'function',
    },
    {
      inputs: [{ internalType: 'bytes4', name: 'interfaceId', type: 'bytes4' }],
      name: 'supportsInterface',
      outputs: [{ internalType: 'bool', name: '', type: 'bool' }],
      stateMutability: 'view',
      type: 'function',
    },
    {
      inputs: [
        { internalType: 'bytes32', name: '', type: 'bytes32' },
        {
          internalType: 'enum ContractResolver.Env',
          name: '',
          type: 'uint8',
        },
      ],
      name: 'typeAddresses',
      outputs: [{ internalType: 'address', name: '', type: 'address' }],
      stateMutability: 'view',
      type: 'function',
    },
  ],
  environment: 0,
  provider: new ethersv5.providers.JsonRpcProvider(
    'https://yellowstone-rpc.litprotocol.com'
  ),
};

const litNodeClient = new LitJsSdk.LitNodeClientNodeJs({
  litNetwork: 'custom',
  minNodeCount: 2,
  checkNodeAttestation: false,
  debug: false,
  contractContext,
  rpcUrl: 'https://yellowstone-rpc.litprotocol.com',
});
```

## Tests

You can run all the tests with `./scripts/run_ci_tests.sh` in the /rust/lit-node directory.

## Tracing with Jaeger

See <https://www.notion.so/litprotocol/Tracing-with-Jaeger-973d3cf1a1ad49df9781d0e9b9d08152>

## Deploying a node release to a Lit Centralized Testnet

### Prereqs

Make sure you have run `cargo install cargo-release` to install the cargo release tool.

### Process

_Please use either `cayenne` or `serrano` to replace the `<network>` placeholders below._

1. Bump the node version with `./scripts/bump.sh` and commit. I commit this straight to `develop` because it just changes the node version and a PR isn't worth it.
2. Create a branch of the format "release-<network>-YYYY-MM-DD" and push to github
3. Run `./scripts/deploy_<network>.sh release-<network>-YYYY-MM-DD` which will build and deploy your branch on <network>
4. At this point I usually run a test to make sure everything works. `yarn hello` from this repo should always work: https://github.com/LIT-Protocol/js-serverless-function-test

## Compile errors on mac

If you see this error message:

```
 = note: ld: warning: directory not found for option '-L/Users/chris/Documents/WorkStuff/LIT/RustNode/lit_node_rust/target/debug/build/libffi-sys-5368fcdd08bbc254/out/libffi-root/lib64'
          Undefined symbols for architecture arm64:
          ....
```

You may need to run:

```shell
brew install libffi
brew reinstall gmp
```

You also need to ensure you have XCode installed (from App Store, reboot after install).

Also ensure your environment (~/.zshrc, ~/.bashrc) is updated (change versions to reflect your install):

```shell
export LDFLAGS="-L/opt/homebrew/opt/libffi/lib"
export CPPFLAGS="-I/opt/homebrew/opt/libffi/include"
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/opt/libffi/lib:/opt/homebrew/Cellar/gmp/6.2.1_1/lib"
```
