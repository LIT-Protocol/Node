import hre from 'hardhat';
import {
  ContractResolver,
  PKPHelper,
  PKPNFTFacet,
  PKPPermissionsFacet,
  PubkeyRouterFacet,
  StakingBalancesFacet,
  StakingFacet,
} from '../typechain-types';
import { hardhatDeployAndVerifySingleContract } from './utils';
const { ethers } = hre;

// TOUCH THIS //
const OLD_CONTRACT_RESOLVER_ADDRESS =
  '0xB0cb99e69c01Bd481aeCc6DD0155d4147e96C746';
const ENV = 0;

// DO NOT TOUCH THIS //
async function run() {
  // 1. Load the current contract resolver
  const oldContractResolver: ContractResolver = await ethers.getContractAt(
    'ContractResolver',
    OLD_CONTRACT_RESOLVER_ADDRESS
  );

  // @ts-ignore
  // 2. Deploy a new contract resolver contract
  const newContractResolver: ContractResolver =
    await hardhatDeployAndVerifySingleContract(
      ethers,
      hre.network.name,
      'ContractResolver',
      {
        deploymentArgs: [ENV],
      }
    );

  // 3. Copy state of addresses to resolve of the current contract resolver to the new one
  const allTyps = await Promise.all([
    oldContractResolver.RELEASE_REGISTER_CONTRACT(),
    oldContractResolver.STAKING_CONTRACT(),
    oldContractResolver.STAKING_BALANCES_CONTRACT(),
    oldContractResolver.MULTI_SENDER_CONTRACT(),
    oldContractResolver.LIT_TOKEN_CONTRACT(),
    oldContractResolver.PUB_KEY_ROUTER_CONTRACT(),
    oldContractResolver.PKP_NFT_CONTRACT(),
    oldContractResolver.RATE_LIMIT_NFT_CONTRACT(),
    oldContractResolver.PKP_HELPER_CONTRACT(),
    oldContractResolver.PKP_PERMISSIONS_CONTRACT(),
    oldContractResolver.PKP_NFT_METADATA_CONTRACT(),
    oldContractResolver.ALLOWLIST_CONTRACT(),
    oldContractResolver.DOMAIN_WALLET_REGISTRY(),
    oldContractResolver.HD_KEY_DERIVER_CONTRACT(),
    oldContractResolver.BACKUP_RECOVERY_CONTRACT(),
  ]);
  const allTypsAddresses = await Promise.all(
    allTyps.map((typ) => oldContractResolver.typeAddresses(typ, 0))
  );
  console.info('This is the current state of the OLD contract resolver.');
  for (let i = 0; i < allTyps.length; i++) {
    console.info(`${allTyps[i]}: ${allTypsAddresses[i]}`);
  }

  console.info('Copying state to the NEW contract resolver.');
  for (let i = 0; i < allTyps.length; i++) {
    console.info(
      `Copying ${allTyps[i]}: ${allTypsAddresses[i]} to the NEW contract resolver.`
    );
    const setContractTx = await newContractResolver.setContract(
      allTyps[i],
      0,
      allTypsAddresses[i]
    );
    console.log('setContractTx: ', setContractTx.hash);
    await setContractTx.wait();
    console.log('setContractTx mined');
  }

  // 4. Update all contract references to the old contract resolver to the new one.
  const newContractResolverAddress = await newContractResolver.getAddress();

  const pkpHelper: PKPHelper = await ethers.getContractAt(
    'PKPHelper',
    await newContractResolver.typeAddresses(
      await newContractResolver.PKP_HELPER_CONTRACT(),
      0
    )
  );
  let updateTx = await pkpHelper.setContractResolver(
    newContractResolverAddress
  );
  console.log('updateTx: ', updateTx.hash);
  await updateTx.wait();
  console.log('updateTx mined');

  const backupRecovery = await ethers.getContractAt(
    'BackupRecoveryFacet',
    await newContractResolver.typeAddresses(
      await newContractResolver.BACKUP_RECOVERY_CONTRACT(),
      0
    )
  );
  updateTx = await backupRecovery.setContractResolver(
    newContractResolverAddress
  );
  console.log('updateTx: ', updateTx.hash);
  await updateTx.wait();
  console.log('updateTx mined');

  const pkpNft: PKPNFTFacet = await ethers.getContractAt(
    'PKPNFTFacet',
    await newContractResolver.typeAddresses(
      await newContractResolver.PKP_NFT_CONTRACT(),
      0
    )
  );
  updateTx = await pkpNft.setContractResolver(newContractResolverAddress);
  console.log('updateTx: ', updateTx.hash);
  await updateTx.wait();
  console.log('updateTx mined');

  const pkpPermissions: PKPPermissionsFacet = await ethers.getContractAt(
    'PKPPermissionsFacet',
    await newContractResolver.typeAddresses(
      await newContractResolver.PKP_PERMISSIONS_CONTRACT(),
      0
    )
  );
  updateTx = await pkpPermissions.setContractResolver(
    newContractResolverAddress
  );
  console.log('updateTx: ', updateTx.hash);
  await updateTx.wait();
  console.log('updateTx mined');

  const pubkeyRouter: PubkeyRouterFacet = await ethers.getContractAt(
    'PubkeyRouterFacet',
    await newContractResolver.typeAddresses(
      await newContractResolver.PUB_KEY_ROUTER_CONTRACT(),
      0
    )
  );
  updateTx = await pubkeyRouter.setContractResolver(newContractResolverAddress);
  console.log('updateTx: ', updateTx.hash);
  await updateTx.wait();
  console.log('updateTx mined');

  const staking: StakingFacet = await ethers.getContractAt(
    'StakingFacet',
    await newContractResolver.typeAddresses(
      await newContractResolver.STAKING_CONTRACT(),
      0
    )
  );
  updateTx = await staking.setContractResolver(newContractResolverAddress);
  console.log('updateTx: ', updateTx.hash);
  await updateTx.wait();
  console.log('updateTx mined');

  const stakingBalances: StakingBalancesFacet = await ethers.getContractAt(
    'StakingBalancesFacet',
    await newContractResolver.typeAddresses(
      await newContractResolver.STAKING_BALANCES_CONTRACT(),
      0
    )
  );
  updateTx = await stakingBalances.setContractResolver(
    newContractResolverAddress
  );
  console.log('updateTx: ', updateTx.hash);
  await updateTx.wait();
  console.log('updateTx mined');
}

run();
