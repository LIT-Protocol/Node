# Instructions

## Upgrading Diamond Facets (High-Level, for Beginners)

The simplest way to upgrade is to replace all the diamond contract facets:

1. First, deploy the new facet contracts, using `scripts/deployContract.ts`. The full command is shown at the top of the file.
2. Once the contract has been deployed, update `FACETS_TO_REPLACE` in `scripts/diamondContracts/tryReplaceAllFacets.ts` with the new contract addresses in the `newFacetAddress` fields.
3. Run the script using `HARDHAT_NETWORK=localchain npx ts-node --files scripts/diamondContracts/tryReplaceAllFacets.ts --diamond-owner-signer-private-key <PRIVATE_KEY>`.

## Upgrading Diamond Facets (Low-Level, for Advanced Users)

These are the instructions for upgrading your diamond contract facets:

1. First, deploy the new facet contracts, using `scripts/deployContract.ts`. The full command is shown at the top of the file.
2. Once the contract has been deployed, make a note of the deployed address.
3. Start building your manifest for cutting diamond facets, using `scripts/diamondContracts/appendDiamondCutManifest.ts` and the command at the top of the file. Each run of this script will **append** a new `DiamondCutOperation` to the manifest file in `scripts/diamondContracts/manifests/diamondCutManifest.json`.
   - You should use the generated deployed address from step 2 as input to the script in step 3.
4. Feel free to make any edits as necessary to the manifest file, for example removing some function selectors that you do not wish to operate on.
5. When your manifest is ready, execute the diamond cuts using `scripts/diamondContracts/diamondCut.ts` and the command at the top of the file.
   - Should you encounter any errors, feel free to make note of the `Error: ` logs and use `npx hardhat decode --data <DATA>` to decode the reverts. You will need to enable `hardhat-tracer` in `hardhat.config.ts` before doing so.

For example, here are the steps I would do for me to introduce contract **changes and additions**:

1. Deploy the new facet contract: `HARDHAT_NETWORK=localchain npx ts-node --files scripts/deployContract.ts --deployer-private-key <PRIVATE_KEY> --new-contract-name StakingViewsFacet`
2. Append a new manifest operation to replace existing function selector logic: `HARDHAT_NETWORK=localchain npx ts-node --files scripts/diamondContracts/appendDiamondCutManifest.ts --contract-name StakingViewsFacet --facet-cut-action 1 --new-facet-address 0xeCfe8CF2dc9b62Fd4858b7539A8F0e9143B0a5Cf`
3. If there exists any function selectors that are new in this replacement operation, the transaction will fail. So, I remove them from the first operation in the manifest.
4. Append a new manifest operation to add new function selectors: `HARDHAT_NETWORK=localchain npx ts-node --files scripts/diamondContracts/appendDiamondCutManifest.ts --contract-name StakingViewsFacet --facet-cut-action 0 --new-facet-address 0xeCfe8CF2dc9b62Fd4858b7539A8F0e9143B0a5Cf`
5. I remove all existing function selectors from the second operation and only include the new one.
6. Execute the `diamondCut` operations per the manifest against the diamond contract: `HARDHAT_NETWORK=localchain npx ts-node --files scripts/diamondContracts/diamondCut.ts --diamond-owner-signer-private-key c44642c263a6f41d71d36cb79f4a7506d1be7fe83b0360fd659380f8ed62ca34`
