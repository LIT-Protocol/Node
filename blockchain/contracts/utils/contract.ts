import ethers, { Signer, keccak256, toBigInt, toUtf8Bytes } from 'ethers';
import {
  BackupRecovery,
  ContractResolver,
  DomainWalletRegistryFacet,
  KeyDeriver,
  PKPNFTFacet,
  PKPNFTMetadata,
  PKPPermissions,
  PKPPermissionsFacet,
  PubkeyRouterFacet,
  StakingAdminFacet,
  StakingBalancesFacet,
  StakingFacet,
} from '../typechain-types';
import { LITToken } from '../typechain-types/contracts/lit-node/LITToken';
import { ip2int } from './index.js';
import { CloneNetFacet } from '../typechain-types/contracts/lit-node/CloneNet/CloneNetFacet.sol';

export enum Environment {
  DEV,
  STAGING,
  PROD,
}

export enum StakingState {
  Active,
  NextValidatorSetLocked,
  ReadyForNextEpoch,
  Unlocked,
  Paused,
}

export async function setContractResolver(
  contractResolver: ContractResolver,
  env: Environment,
  {
    backupRecoveryContract,
    cloneNetContract,
    tokenContract,
    stakingContract,
    stakingBalancesContract,
    pkpContract,
    pkpPermissionsContract,
    pkpHelperContract,
    pkpNftMetadataContract,
    domainWalletRegistryContract,
    hdKeyDeriverContract,
    pubkeyRouterContract,
    stylusContractP256,
    stylusContractK256,
  }: {
    backupRecoveryContract?: BackupRecovery;
    cloneNetContract?: CloneNetFacet;
    tokenContract?: LITToken;
    stakingContract?: StakingFacet;
    stakingBalancesContract?: StakingBalancesFacet;
    pkpContract?: PKPNFTFacet;
    pkpPermissionsContract?: PKPPermissionsFacet;
    pkpHelperContract?: PKPPermissions;
    pkpNftMetadataContract?: PKPNFTMetadata;
    domainWalletRegistryContract?: DomainWalletRegistryFacet;
    hdKeyDeriverContract?: KeyDeriver;
    pubkeyRouterContract?: PubkeyRouterFacet;
    stylusContractP256?: string;
    stylusContractK256?: string;
  }
) {
  if (cloneNetContract) {
    await contractResolver.setContract(
      await contractResolver.CLONE_NET_CONTRACT(),
      env,
      await cloneNetContract.getAddress()
    );
  }

  if (tokenContract) {
    await contractResolver.setContract(
      await contractResolver.LIT_TOKEN_CONTRACT(),
      env,
      await tokenContract.getAddress()
    );
  }

  if (stakingContract) {
    await contractResolver.setContract(
      await contractResolver.STAKING_CONTRACT(),
      env,
      await stakingContract.getAddress()
    );
  }

  if (stakingBalancesContract) {
    await contractResolver.setContract(
      await contractResolver.STAKING_BALANCES_CONTRACT(),
      env,
      await stakingBalancesContract.getAddress()
    );
  }

  if (pkpContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_NFT_CONTRACT(),
      env,
      await pkpContract.getAddress()
    );
  }

  if (pkpPermissionsContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_PERMISSIONS_CONTRACT(),
      env,
      await pkpPermissionsContract.getAddress()
    );
  }

  if (pkpHelperContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_HELPER_CONTRACT(),
      env,
      await pkpHelperContract.getAddress()
    );
  }

  if (pkpNftMetadataContract) {
    await contractResolver.setContract(
      await contractResolver.PKP_NFT_METADATA_CONTRACT(),
      env,
      await pkpNftMetadataContract.getAddress()
    );
  }

  if (domainWalletRegistryContract) {
    await contractResolver.setContract(
      await contractResolver.DOMAIN_WALLET_REGISTRY(),
      env,
      await domainWalletRegistryContract.getAddress()
    );
  }

  if (hdKeyDeriverContract) {
    await contractResolver.setContract(
      await contractResolver.HD_KEY_DERIVER_CONTRACT(),
      env,
      await hdKeyDeriverContract.getAddress()
    );
  }

  if (pubkeyRouterContract) {
    await contractResolver.setContract(
      await contractResolver.PUB_KEY_ROUTER_CONTRACT(),
      env,
      await pubkeyRouterContract.getAddress()
    );
  }

  if (backupRecoveryContract) {
    await contractResolver.setContract(
      await contractResolver.BACKUP_RECOVERY_CONTRACT(),
      env,
      await backupRecoveryContract.getAddress()
    );
  }

  if (stylusContractP256) {
    await contractResolver.setContract(
      keccak256(toUtf8Bytes('HD_KEY_DERIVER_CONTRACT_P256')),
      env,
      stylusContractP256
    );
  }

  if (stylusContractK256) {
    await contractResolver.setContract(
      keccak256(toUtf8Bytes('HD_KEY_DERIVER_CONTRACT_K256')),
      env,
      stylusContractK256
    );
  }
}

export interface SetupStakingOptions {
  numValidators: number;
  ipAddress: string;
  startingPort: number;
}

export interface CreateValidatorOptions {
  ipAddress: string;
  port: number;
}

export interface StakingAccount {
  nodeAddress: ethers.HDNodeWallet;
  stakingAddress: ethers.HDNodeWallet;
  commsKeys: {
    sender: bigint;
    receiver: bigint;
  };
  ip?: string;
  port?: number;
}

type RootKey = [Uint8Array, number];

export async function allNodesVoteForRootKeys(
  ethers: any,
  pubkeyRouterContract: PubkeyRouterFacet,
  stakingContract: StakingFacet,
  stakingAccounts: StakingAccount[]
): Promise<RootKey[]> {
  const rootKeys: RootKey[] = [
    [
      ethers.getBytes(
        '0x028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x02a89cb5090c0aaee9c5831df939abbeab2e0f62b5d54ceae6e816a9fe87c8ca32'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x033e0c9d93b41414c3a8d287bb40ab024fbf176cb45c6616a3bf74e97bb68b5165'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x03a0c18f5d9db21fec597edef52f7a26449cdd90357532704a1ede6c27981a31b8'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x02794db35a0b6a6968ba4ed059630d788d591f083778dac9a45935549ca5f75ea6'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x03b398a663086dc7f1b5948d2195b176a7705fe71b0ad07110f57975254e601598'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x0215f2cddeb89428f74132a84acf7e1a344f2ed9a39768f7006c9b8843e513dc55'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x0297d2a91f5a52e98873b7a4946c47d7736d6661cebace9c160d955999be971492'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x03d2ee101c65ca0d60b5bc27ca1859c968984b1096d742874649bdc4fac6e9498a'
      ),
      2,
    ],
    [
      ethers.getBytes(
        '0x02bb0deb45aefb171e7117390991c2a230218fda04d9bb3cfd343f56ab61c3e390'
      ),
      2,
    ],
  ];

  for (let i = 0; i < stakingAccounts.length; i++) {
    // vote with the nodes
    await pubkeyRouterContract
      .connect(stakingAccounts[i].nodeAddress)
      .voteForRootKeys(
        // @ts-ignore
        await stakingContract.getAddress(),
        rootKeys
      );
  }

  return rootKeys;
}

export async function createValidatorAndStake(
  ethers: any,
  stakingContract: StakingFacet,
  stakingBalancesContract: StakingBalancesFacet,
  tokenContract: LITToken,
  deployer: Signer,
  options: CreateValidatorOptions
): Promise<StakingAccount> {
  const minStake = await stakingBalancesContract.minimumStake();
  const totalToStake = minStake * 3n; // 3 times the minimum stake

  let provider = deployer.provider!;
  console.log(
    `deployer has ${await provider.getBalance(
      deployer
    )} eth.  Funding validator...`
  );

  const ethForGas = ethers.parseEther('1.0');

  const stakingAccount: StakingAccount = {
    stakingAddress: ethers.Wallet.createRandom().connect(provider),
    nodeAddress: ethers.Wallet.createRandom().connect(provider),
    commsKeys: {
      sender: toBigInt(ethers.randomBytes(32)),
      receiver: toBigInt(ethers.randomBytes(32)),
    },
    ip: options.ipAddress,
    port: options.port,
  };

  const stakingAddress = stakingAccount.stakingAddress.address;
  const nodeAddress = stakingAccount.nodeAddress.address;

  // send them some gas
  await deployer.sendTransaction({
    to: stakingAddress,
    value: ethForGas,
  });
  await deployer.sendTransaction({
    to: nodeAddress,
    value: ethForGas,
  });
  // send them some tokens
  tokenContract = tokenContract.connect(deployer);
  await tokenContract.transfer(stakingAddress, totalToStake);
  tokenContract = tokenContract.connect(stakingAccount.stakingAddress);
  await tokenContract.approve(
    await stakingBalancesContract.getAddress(),
    totalToStake
  );

  stakingContract = stakingContract.connect(stakingAccount.stakingAddress);

  await stakingContract.stake(minStake);

  return stakingAccount;
}

export async function setupStakingWithValidatorsAndAdvance(
  ethers: any,
  stakingContract: StakingFacet,
  stakingAdminContract: StakingAdminFacet,
  stakingBalancesContract: StakingBalancesFacet,
  tokenContract: LITToken,
  deployer: Signer,
  options: SetupStakingOptions
): Promise<StakingAccount[]> {
  // Validate number of validators is greater than 1
  if (options.numValidators < 3) {
    throw new Error('Must have at least 3 validators');
  }

  console.log(`deploying staking with ${options.numValidators} validators`);

  const provider = deployer.provider!;

  // set epoch length to 1 so that we can test quickly
  await stakingAdminContract.setEpochLength(1);

  let stakingAccounts: StakingAccount[] = [];
  for (let i = 0; i < options.numValidators; i++) {
    console.log(`creating validator ${i}`);
    const stakingAccount = await createValidatorAndStake(
      ethers,
      stakingContract,
      stakingBalancesContract,
      tokenContract,
      deployer,
      {
        ipAddress: options.ipAddress,
        port: options.startingPort + i + 1,
      }
    );
    console.log('requesting to join');
    // Call requestToJoin for each validator
    await stakingContract
      .connect(stakingAccount.stakingAddress)
      .requestToJoin(
        ip2int(stakingAccount.ip!),
        0,
        stakingAccount.port!,
        stakingAccount.nodeAddress.address,
        stakingAccount.commsKeys.sender,
        stakingAccount.commsKeys.receiver
      );
    stakingAccounts.push(stakingAccount);
  }

  // set next epoch end time to 10 seconds ago
  await stakingAdminContract.setEpochEndTime(
    Math.floor(Date.now() / 1000) - 10
  );
  // unpause staking contract
  await stakingAdminContract.setEpochState(StakingState.Active);

  // uncomment for debugging Staking contract state
  // const stakingViewsFacet = await ethers.getContractAt(
  //   'StakingViewsFacet',
  //   await stakingContract.getAddress()
  // );
  // const epoch = await stakingViewsFacet.epoch();
  // const epochNumber = epoch[1];
  // const endTime = epoch[2];
  // console.log('epoch number', epochNumber);
  // console.log('epoch end time', endTime);
  // // get latest block timestamp
  // const latestBlockNumber = await provider.getBlockNumber();
  // const blockTimestamp = (await provider.getBlock(latestBlockNumber))
  //   ?.timestamp;
  // console.log('block timestamp', blockTimestamp);
  // const state = await stakingViewsFacet.state();
  // console.log('epoch state', state);
  // const nextSet = await stakingViewsFacet.getValidatorsInNextEpoch();
  // console.log('next set length', nextSet.length);

  // okay now that we're all staked, let's kickoff the first epoch
  console.log('locking validators for next epoch');
  await stakingContract.lockValidatorsForNextEpoch();

  console.log('signalling ready for next epoch');
  for (let i = 0; i < options.numValidators; i++) {
    stakingContract = stakingContract.connect(stakingAccounts[i].nodeAddress);
    await stakingContract.signalReadyForNextEpoch(1);
  }

  console.log('advancing epoch');
  await stakingContract.advanceEpoch();

  console.info(
    `Finished setting up staking with ${options.numValidators} validators and advanced epoch.`
  );

  return stakingAccounts;
}
