import { expect } from 'chai';
import { BigNumberish, EventLog, Log, Signer } from 'ethers';
import hre from 'hardhat';
import { deployDiamond } from '../../scripts/deployDiamond';
import {
  DomainWalletRegistryFacet,
  DomainWalletRegistryViewsFacet,
  KeyDeriver,
  PKPHelper,
  PKPNFTFacet,
  PKPPermissionsFacet,
  PubkeyRouterFacet,
  StakingBalancesFacet,
  StakingFacet,
} from '../../typechain-types';
import {
  LITToken,
  PKPNFTMetadata,
} from '../../typechain-types/contracts/lit-node';
import {
  Environment,
  allNodesVoteForRootKeys,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
} from '../../utils/contract';
const { ethers } = hre;

describe('DomainWalletRegistry', function () {
  let deployer;
  let signers;
  let stakingAccounts;
  let pkpNftFacet: PKPNFTFacet;
  let pkpHelper: PKPHelper;
  let pkpPermissionsFacet: PKPPermissionsFacet;
  let pkpNftMetadata: PKPNFTMetadata;
  let pubkeyRouter: PubkeyRouterFacet;
  let keyDeriver: KeyDeriver;
  let domainWalletRegistryFacet: DomainWalletRegistryFacet;
  let domainWalletRegistryViewsFacet: DomainWalletRegistryViewsFacet;
  let stakingFacet: StakingFacet;
  let stakingBalancesFacet: StakingBalancesFacet;
  let token: LITToken;
  let pkpTokenId = parseInt(
    '04e4df1c02a7b9bd8f0668c344bbbae3a66b72273a50890ed8e653f513a247b8d2a39ddd5a6b6bb8ef1270fd105f73d0133886c9a4890777f2f4f8127cdf5cc1fc'
  );

  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals

  beforeEach(async function () {
    const contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);

    [deployer, ...signers] = await ethers.getSigners();

    const { diamond: domainWalletRegistryDiamond } = await deployDiamond(
      'DomainWalletRegistry',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: [
          'DomainWalletRegistryFacet',
          'DomainWalletRegistryViewsFacet',
        ],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );

    domainWalletRegistryFacet = await ethers.getContractAt(
      'DomainWalletRegistryFacet',
      await domainWalletRegistryDiamond.getAddress()
    );

    domainWalletRegistryViewsFacet = await ethers.getContractAt(
      'DomainWalletRegistryViewsFacet',
      await domainWalletRegistryDiamond.getAddress()
    );

    const { diamond: pkpDiamond } = await deployDiamond(
      'PKPNFT',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PKPNFTFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );

    pkpNftFacet = await ethers.getContractAt(
      'PKPNFTFacet',
      await pkpDiamond.getAddress()
    );

    pkpHelper = await ethers.deployContract('PKPHelper', [
      await contractResolver.getAddress(),
      Environment.DEV,
    ]);

    const { diamond: pkpPermissionsDiamond } = await deployDiamond(
      'PKPPermissions',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PKPPermissionsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );

    pkpPermissionsFacet = await ethers.getContractAt(
      'PKPPermissionsFacet',
      await pkpPermissionsDiamond.getAddress()
    );

    pkpNftMetadata = await ethers.deployContract('PKPNFTMetadata', [
      await contractResolver.getAddress(),
      Environment.DEV,
    ]);

    const { diamond: pubkeyRouterDiamond } = await deployDiamond(
      'PubkeyRouter',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PubkeyRouterFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );

    pubkeyRouter = await ethers.getContractAt(
      'PubkeyRouterFacet',
      await pubkeyRouterDiamond.getAddress()
    );

    keyDeriver = await ethers.deployContract('KeyDeriver');

    const { diamond: stakingDiamond } = await deployDiamond(
      'Staking',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: [
          'StakingFacet',
          'StakingViewsFacet',
          'StakingVersionFacet',
        ],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );

    stakingFacet = await ethers.getContractAt(
      'StakingFacet',
      await stakingDiamond.getAddress()
    );

    const { diamond: stakingBalancesDiamond } = await deployDiamond(
      'StakingBalances',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: ['StakingBalancesFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );

    stakingBalancesFacet = await ethers.getContractAt(
      'StakingBalancesFacet',
      await stakingBalancesDiamond.getAddress()
    );

    token = await ethers.deployContract('LITToken', [
      ethers.parseUnits('1000000000', 18), // 1b tokens
    ]);

    await setContractResolver(contractResolver, Environment.DEV, {
      domainWalletRegistryContract: domainWalletRegistryFacet,
      pkpContract: pkpNftFacet,
      pkpHelperContract: pkpHelper,
      pkpPermissionsContract: pkpPermissionsFacet,
      pkpNftMetadataContract: pkpNftMetadata,
      pubkeyRouterContract: pubkeyRouter,
      hdKeyDeriverContract: keyDeriver,
      stakingContract: stakingFacet,
      stakingBalancesContract: stakingBalancesFacet,
      tokenContract: token,
    });

    // Mint enough tokens for the deployer
    await token.mint(deployer.address, totalTokens);
    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingFacet,
      stakingBalancesFacet,
      token,
      deployer,
      {
        numValidators: 3,
        startingPort: 7777,
        ipAddress: '192.168.1.1',
      }
    );

    await allNodesVoteForRootKeys(
      ethers,
      pubkeyRouter,
      stakingFacet,
      stakingAccounts
    );
  });

  it('should register uri', async function () {
    let uri = `0x${Buffer.from('foobar.lit.id').toString('hex')}`;
    let userId = `0x${Buffer.from('foo@bar.baz').toString('hex')}`;
    const blockTimestamp = (await ethers.provider.getBlock('latest'))!
      .timestamp;
    let ttl = blockTimestamp! + 300; // set ttl to be 300s from now

    await domainWalletRegistryFacet.registerDomain(
      userId,
      uri,
      ttl,
      pkpTokenId,
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ]
    );

    let actualUri = await domainWalletRegistryViewsFacet.getDomainUri(
      pkpTokenId
    );

    expect(actualUri).to.equal(uri);
  });

  it('should register and mint domain and assert', async function () {
    let uri = `0x${Buffer.from('foobar.lit.id').toString('hex')}`;
    let userId = `0x${Buffer.from('foo@bar.baz').toString('hex')}`;
    const blockTimestamp = (await ethers.provider.getBlock('latest'))!
      .timestamp;
    let ttl = blockTimestamp + 300; // set ttl to be 300s from now

    const mintCost = await pkpNftFacet.mintCost();

    const tx = await domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );
    const txReceipt = await tx.wait();
    const pkpTokenId = getPKPMintedEventTokenId(pkpNftFacet, txReceipt?.logs!);

    const actualUri = await domainWalletRegistryViewsFacet.getDomainUri(
      pkpTokenId
    );
    const exp = await domainWalletRegistryViewsFacet.getExpiration(pkpTokenId);
    const isOwner = await domainWalletRegistryViewsFacet.hasOwner(pkpTokenId);
    expect(actualUri).to.equal(uri);
    expect(exp).to.equal(BigInt(ttl));
    expect(isOwner).to.be.true;

    // Assert has not expired yet by asserting absence of Expired event.
    const expireTx1 = await domainWalletRegistryFacet.hasExpired(pkpTokenId);
    const expireTx1Receipt = await expireTx1.wait();

    const expiredEventTopic =
      domainWalletRegistryFacet.getEvent('Expired').fragment.topicHash;
    const expired1EventLog = expireTx1Receipt!.logs.find((log) => {
      return log.topics[0] === expiredEventTopic;
    });
    expect(expired1EventLog).to.be.undefined;

    // Now, check expiration works by fast forwarding chain time.
    await ethers.provider.send('evm_setNextBlockTimestamp', [ttl + 1]);
    await ethers.provider.send('evm_mine');

    const expireTx2 = await domainWalletRegistryFacet.hasExpired(pkpTokenId);
    const expireTx2Receipt = await expireTx2.wait();

    const expired2EventLog = expireTx2Receipt!.logs.find((log) => {
      return log.topics[0] === expiredEventTopic;
    });
    const logTTL = domainWalletRegistryFacet.interface.parseLog(
      // @ts-ignore
      expired2EventLog!
    )?.args[2];
    expect(parseInt(logTTL, 10)).to.equal(ttl);
  });

  it('should register and mint and revoke domain and assert', async function () {
    let uri = `0x${Buffer.from('foobar.lit.id').toString('hex')}`;
    let userId = `0x${Buffer.from('foo@bar.baz').toString('hex')}`;
    const blockTimestamp = (await ethers.provider.getBlock('latest'))!
      .timestamp;
    let ttl = blockTimestamp! + 300; // set ttl to be 300s from now

    const mintCost = await pkpNftFacet.mintCost();

    const tx = await domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );
    const txReceipt = await tx.wait();
    const pkpTokenId = getPKPMintedEventTokenId(pkpNftFacet, txReceipt?.logs!);

    await domainWalletRegistryFacet.removeDomain(pkpTokenId);

    const isRouted = await domainWalletRegistryViewsFacet.isRouted(pkpTokenId);
    const isOwner = await domainWalletRegistryViewsFacet.hasOwner(pkpTokenId);
    const resolvedDomain = await domainWalletRegistryViewsFacet.getDomainUri(
      pkpTokenId
    );
    const resolvedTokenId = await domainWalletRegistryViewsFacet.getPkpTokenId(
      1
    );

    expect(isRouted).to.be.false;
    expect(isOwner).to.be.false;
    expect(resolvedDomain).to.equal('0x');
    expect(resolvedTokenId).to.equal(0);
  });

  it('should not register and mint domain exceeding maximum character limit', async function () {
    let domain = 'foobar.lit.id';
    for (let i = 0; i < 35; i++) {
      domain += 'a'; // just add a characrter to exceed.
    }

    let uri = `0x${Buffer.from(domain).toString('hex')}`;
    let userId = `0x${Buffer.from('foo@bar.baz').toString('hex')}`;
    const blockTimestamp = (await ethers.provider.getBlock('latest'))!
      .timestamp;
    let ttl = blockTimestamp! + 300; // set ttl to be 300s from now

    const mintCost = await pkpNftFacet.mintCost();

    const tx = await domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );

    const tx2 = domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );

    expect(tx2).to.be.revertedWith('MaximumCharacterLimitExceeded');
  });

  it('should register and mint domain with 32 characters', async function () {
    let domain = 'foobar.lit.id';
    while (domain.length < 32) {
      domain += 'a'; // just add a characrter to exceed.
    }

    let uri = `0x${Buffer.from(domain).toString('hex')}`;
    let userId = `0x${Buffer.from('foo@bar.baz').toString('hex')}`;
    const blockTimestamp = (await ethers.provider.getBlock('latest'))!
      .timestamp;
    let ttl = blockTimestamp! + 300; // set ttl to be 300s from now

    const mintCost = await pkpNftFacet.mintCost();

    const tx = await domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );

    const tx2 = domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );

    expect(tx2).to.be.revertedWith('MaximumCharacterLimitExceeded');
  });

  it('should not register and mint domain that is already registered', async function () {
    let uri = `0x${Buffer.from('foobar.lit.id').toString('hex')}`;
    let userId = `0x${Buffer.from('foo@bar.baz').toString('hex')}`;
    const blockTimestamp = (await ethers.provider.getBlock('latest'))!
      .timestamp;
    let ttl = blockTimestamp! + 300; // set ttl to be 300s from now

    const mintCost = await pkpNftFacet.mintCost();

    const tx = await domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );

    const tx2 = domainWalletRegistryFacet.registerDomainAndMintNext(
      userId,
      uri,
      ttl,
      [],
      [],
      [],
      [],
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ],
      {
        value: mintCost,
      }
    );

    expect(tx2).to.be.revertedWith('DomainAlreadyRegistered');
  });

  it('should not register domain that is already registered', async function () {
    let uri = `0x${Buffer.from('foobar.lit.id').toString('hex')}`;
    let userId = `0x${Buffer.from('foo@bar.baz').toString('hex')}`;
    const blockTimestamp = (await ethers.provider.getBlock('latest'))!
      .timestamp;
    let ttl = blockTimestamp! + 300; // set ttl to be 300s from now

    const tx = await domainWalletRegistryFacet.registerDomain(
      userId,
      uri,
      ttl,
      pkpTokenId,
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ]
    );

    const tx2 = domainWalletRegistryFacet.registerDomain(
      userId,
      uri,
      ttl,
      pkpTokenId,
      [
        'foobar.lit.id',
        'https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo',
      ]
    );

    expect(tx2).to.be.revertedWith('DomainAlreadyRegistered');
  });

  it('maximum character limit', async function () {
    const limit =
      await domainWalletRegistryViewsFacet.getDomainCharacterLimit();
    expect(limit).to.be.equal(32);
  });
});

const getPKPMintedEventTokenId = (
  pkpContract: PKPNFTFacet,
  logs: Array<EventLog | Log>
): BigNumberish => {
  const pkpMintedEventTopic =
    pkpContract.getEvent('PKPMinted').fragment.topicHash;

  // Find the log that matches the sigHash of the event
  const pkpMintedEventLog = logs.find((log) => {
    return log.topics[0] === pkpMintedEventTopic;
  });
  if (!pkpMintedEventLog) {
    throw new Error('PKPMinted event not found in logs');
  }

  // @ts-ignore
  return pkpContract.interface.parseLog(pkpMintedEventLog!)?.args.tokenId;
};
