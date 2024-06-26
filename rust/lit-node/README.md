# Lit Node

## Setup

Before you do anything else, you'll need to setup the project. This will check out any dependent repos in the parent directory as well as perform any other tasks neccessary.

```shell
make setup
```

## Prerequisites for building

On Linux:

```shell
sudo apt install libudev-dev libsqlite3-dev
```

## Running

1. Deploy the contracts. You can do this running `npx ts-node scripts/deploy.ts --network lit` in the /blockchain/contracts directory. When it asks if you want to copy configs over to the node folder, select yes. This will copy the node config files generated by the deploy script into the /rust/lit-node/config directory. Note, you must have these env vars set:

```shell
LIT_ROLLUP_MAINNET_DEPLOYER_PRIVATE_KEY = the private key of an address on Chronicle which is funded
```

2. Run the node itself. Go to /rust/lit-node and run `./scripts/start_dev.sh X` were X is the number of nodes you wish to run. Note, this must match the same number of nodes you set up when you ran the deploy script above. This will spin up a local network of nodes and print logs from the first one.

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

## Upgrading Deno

Do the following to upgrade Deno to a new version:

1. Find out the versions of the crates `deno_runtime`, `deno_core`, and `deno_ast` by looking at the `Cargo.toml` files of the Deno version you want to upgrade to, e.g. <https://github.com/denoland/deno/tree/v1.36>
2. Modify [Cargo.toml](Cargo.toml) to use the new Deno crates
3. Copy all JS files from <https://github.com/denoland/deno/tree/v1.36/runtime/js> to [js_libs/deno](js_libs/deno)
4. Check the history of <https://github.com/denoland/deno/blob/v1.36/runtime/build.rs> to see if there are any changes, e.g. new extensions, that need to be applied to [build.rs](build.rs) too
5. Remove any unnecessary ops from the build script and from the corresponding files in [js_libs/deno](js_libs/deno)
6. Make sure `cargo build` works and commit the changes

## Compile errors on AMD SEV

Compiling and seeing this error on SEV guest? `undefined symbol: ERR_get_error_all` Then you probably need to point the rust compiler at openssl:

First, run:

```shell
sudo ldconfig /usr/local/lib64
```

then:

```shell
cargo clean
OPENSSL_INCLUDE_DIR=/usr/local/include/openssl OPENSSL_LIB_DIR=/usr/local/lib64/ cargo build
```

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

## Setting Up Local Network

These are the instructions to spin up a network running locally:

1. Follow the instructions [here](https://github.com/LIT-Protocol/Node/blob/develop/blockchain/contracts/README.md#deploying) to deploy + configure smart contracts locally.
2. Run `cp rpc-config.example.yaml rpc-config.yaml` and make sure you configure the RPC provider URLs correctly for the chain you would like to spin up the network against.
3. Run `cargo build` in this project.
4. If you are using AMD CPUs, set the environment variable `LIT_ATTESTATION_TYPE_OVERRIDE` to anything other than `"AMD_SEV_SNP"`. Otherwise the nodes will require AMD_SEV_SNP attestations from peers.
5. Run `./scripts/start_dev.sh 3` to spin up a 3 node network. If you have specified a different network size in step 1, change `3` to the correct value accordingly.
