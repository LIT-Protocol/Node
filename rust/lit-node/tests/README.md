# Overview

The node testing suite contains both unit and integration tests.   Integration tests include deployment of the network's contracts, and creation of configuration files for the nodes involved.

Integration tests typically involve a testnet (representing the underlying chain) and a validator collection.   Running a test often has configurable parameters - ie, *mint a PKP on `mumbai` using a 6 node network and sign a lit action*.  

Integration tests contain a number of helpers to assist with configuring the network, compiling & deploying contracts, creating node configuration files and spinning up the nodes before actually executing the tests.

*The testnet setup process scans for files named  `test_{chainname}.toml` placed in the `./config/test/` folder during it's initialization, and will use the contained information to help run the test.   If this file isn't found, a fresh deployment is performed.   See the section below titled "Default chains & test configuration files" for file structure and additional information*.

## Prerequisites:

### Supported chains

Tests that involve interaction with chains can use either `Hardhat` (running in `lit-assets/blockchain/contracts`), `Anvil` or Polygon's `mumbai` network.  

### Hardhat Pre-reqs:

No additional pre-reqs are required.

### Anvil Pre-reqs:

No additional pre-reqs are required.

### Mumbai Pre-reqs:

A known account (used as a deployer) with at least 1 MATIC per node and 2 additional MATIC should be available to test a full deployment, and execute a test.   Thus a 3 node network should have an account with 5 MATIC  (3 * 1 + 2) available to act as a deployer.   Deployment and configuration of all contracts currently costs less than a MATIC, and each node is provided with .5 MATIC to make contract calls - ie, to stake, mint PKPs, etc. 

### Default chains & test configuration files

Tests all have default chains, but can read configuration files that contain information about the chain to use, deployer addresses and paths to existing node configuration files.   Below is a sample of a test configuration file that can be passed to a test using the `testconfigfile` arg.

**test_mumbai.toml**
```
# the target network.  either "mumbai" or "hardhat"
[network]
chain_name = "mumbai"

# This value is required for chains that use actual tokens.
[deployer]
secret = "0x3cdc7fc976be61950cb4ba082eb3da79ccdf8fe5315480add5953c41e86f8cf4"

# This is an optional section, providing the location of pre-built configuration files (usually from an earlier test)
[reuse]
config_path = "mumbai/20230125_110539"
```

It is also possible to use config files for the default chain for a given test.   The testnet setup process looks for files named  `test_{chainname}.toml` placed in the `./config/test/ folder`.   If this file isn't found, a fresh deployment is performed.


# Running:


To run the chain tests, from the top level do (nocapture is optional).
```
cargo test --test integration_tests -- --nocapture
```


Run only a specific test within a specific integration test, saving the log:
```
rm addthenremove.log; cargo test chained_addthenremove --test integration_tests -- --nocapture > addthenremove.log
```

## Test scenario re-use (mumbai only)

Tests that don't require completely fresh state may re-use existing deployments, greatly reducing the testing time involved and native tokens consumed.   During re-use it is assumed that all contracts have been properly deployed and configured.   

Since funds may be required to run certain tests (minting, configuring, etc), the deployer will scan all nodes and top up funds of the chain's token (MATIC for now), as required.   This allows for rapid CI testing.



# Using logs.
Example
```
RUST_LOG=lit_node=trace cargo test initial_only --test integration_tests -- --nocapture > the.log 2>&1
```

# Developer Notes

Each test file in the `tests` folder generates a seperate crate.  Shared libraries are stored in `tests/common/` and are organized according to feature.    If the core of a test needs to be re-used, it is generally advisable to store shared functionality in the `common` folder.