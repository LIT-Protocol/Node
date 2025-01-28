// this script sets an entry in the contract resolver
// Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/setContractResolverValue.ts --resolver-address <RESOLVER_ADDRESS> --signer-private-key <SIGNER_PRIVATE_KEY> --env <ENV> --resolver-entry-key <RESOLVER_ENTRY_KEY> --resolver-entry-value <RESOLVER_ENTRY_VALUE>

import { Signer, toBigInt } from 'ethers';
import hre from 'hardhat';
import yargs from 'yargs';
import {
  ContractResolver,
  StakingBalancesFacet,
  StakingFacet,
} from '../typechain-types';
import { ip2int } from '../utils';
const { ethers } = hre;
const fs = require('fs');

async function run() {
  const inputs = await getInputsFromCliOptions();

  // Get signer
  const signer = new ethers.Wallet(inputs.signerPrivateKey).connect(
    ethers.provider
  );
  console.log('signer address', signer.address);

  await setContractResolverValue(
    inputs.resolverAddress,
    inputs.env,
    inputs.resolverEntryKey,
    inputs.resolverEntryValue,
    signer
  );
}

async function setContractResolverValue(
  resolverAddress: string,
  env: number,
  resolverEntryKey: string,
  resolverEntryValue: string,
  signer: Signer
) {
  const resolver: ContractResolver = await ethers.getContractAt(
    'ContractResolver',
    resolverAddress,
    signer
  );

  // create the resolver key
  const resolverKey = ethers.keccak256(ethers.toUtf8Bytes(resolverEntryKey));

  // set the resolver value
  const tx = await resolver.setContract(resolverKey, env, resolverEntryValue);
  console.log(
    `tx hash for setting ${resolverKey}(for ${resolverEntryKey}) to ${resolverEntryValue}`,
    tx.hash
  );
  const receipt = await tx.wait();
  console.log(`set ${resolverEntryKey} to ${resolverEntryValue}`);
}

async function getInputsFromCliOptions(): Promise<Inputs> {
  const argv = await yargs(process.argv.slice(2)).options({
    'resolver-address': {
      type: 'string',
      describe: 'Contract resolver address',
      required: true,
    },
    'signer-private-key': {
      type: 'string',
      describe: 'Signer private key',
      required: true,
    },
    env: {
      type: 'number',
      describe: 'Environment - 0 for dev, 1 for staging, 2 for prod',
      required: true,
    },
    'resolver-entry-key': {
      type: 'string',
      describe: 'Resolver entry key',
      required: true,
    },
    'resolver-entry-value': {
      type: 'string',
      describe: 'Resolver entry value',
      required: true,
    },
  }).argv;

  return argv;
}

run();

interface Inputs {
  resolverAddress: string;
  signerPrivateKey: string;
  env: number;
  resolverEntryKey: string;
  resolverEntryValue: string;
}
