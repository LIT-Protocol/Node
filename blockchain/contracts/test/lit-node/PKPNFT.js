const { expect } = require('chai');
const { ethers } = require('hardhat');
const { getBytesFromMultihash, getParamsFromPKPMint } = require('../../utils');
const { deployDiamond } = require('../../scripts/deployDiamond');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  allNodesVoteForRootKeys,
} = require('../../utils/contract');

describe('PKPNFT', function () {
  let deployer;
  let signers;
  let pkpContract;
  let router;
  let pkpPermissions;
  let pkpNftMetadata;
  let contractResolver;
  let stakingContract;
  let tokenContract;
  let stakingAccounts = [];
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals

  before(async () => {
    // Validation
    if (
      supportsArbitrumStylus(hre.network.config) &&
      (!hre.network.config.stylusContractsForTests.p256 ||
        !hre.network.config.stylusContractsForTests.k256)
    ) {
      console.log(
        'Please set the Stylus contract addresses using env variables defined in the hardhat.config.ts file'
      );
      process.exit(1);
    }
  });

  beforeEach(async () => {
    [deployer, ...signers] = await ethers.getSigners();

    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);

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
    pkpContract = await ethers.getContractAt(
      'PKPNFTFacet',
      await pkpDiamond.getAddress()
    );
    const { diamond: routerDiamond } = await deployDiamond(
      'PubkeyRouter',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PubkeyRouterFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    router = await ethers.getContractAt(
      'PubkeyRouterFacet',
      await routerDiamond.getAddress()
    );
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
    pkpPermissions = await ethers.getContractAt(
      'PKPPermissionsFacet',
      await pkpPermissionsDiamond.getAddress()
    );
    pkpNftMetadata = await ethers.deployContract('PKPNFTMetadata', [
      await contractResolver.getAddress(),
      Environment.DEV,
    ]);
    tokenContract = await ethers.deployContract(
      'LITToken',
      [ethers.parseUnits('1000000000', 18)] // 1b tokens
    );

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
    stakingContract = await ethers.getContractAt(
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
    stakingBalances = await ethers.getContractAt(
      'StakingBalancesFacet',
      await stakingBalancesDiamond.getAddress()
    );

    if (hre.network.name === 'localchainArbitrum') {
      keyDeriver = await ethers.deployContract('ArbitrumKeyDeriver', [
        await contractResolver.getAddress(),
        Environment.DEV,
      ]);
    } else {
      keyDeriver = await ethers.deployContract('KeyDeriver');
    }

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract,
      stakingContract,
      stakingBalancesContract: stakingBalances,
      pkpContract,
      pkpPermissionsContract: pkpPermissions,
      pkpNftMetadataContract: pkpNftMetadata,
      hdKeyDeriverContract: keyDeriver,
      pubkeyRouterContract: router,
      stylusContractP256: supportsArbitrumStylus(hre.network.config)
        ? hre.network.config.stylusContractsForTests.p256
        : undefined,
      stylusContractK256: supportsArbitrumStylus(hre.network.config)
        ? hre.network.config.stylusContractsForTests.k256
        : undefined,
    });

    // Mint enough tokens for the deployer
    await tokenContract.mint(deployer.address, totalTokens);
    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingContract,
      stakingBalances,
      tokenContract,
      deployer,
      {
        numValidators: 2,
        startingPort: 7777,
        ipAddress: '192.168.1.1',
      }
    );
    await allNodesVoteForRootKeys(
      ethers,
      router,
      stakingContract,
      stakingAccounts
    );
  });

  describe('Attempt to Mint PKP NFT', async () => {
    let minter;

    beforeEach(async function () {
      [minter, ...signers] = signers;
      pkpContract = pkpContract.connect(minter);
    });

    it('refuses to mint for free', async () => {
      expect(pkpContract.mintNext(2)).revertedWith(
        'You must pay exactly mint cost'
      );
    });

    it('mints successfully', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const tx = await pkpContract.mintNext(2, transaction);
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      // check the metadata
      const pkpEthAddress = await pkpContract.getEthAddress(tokenId);

      const tokenUri = await pkpContract.tokenURI(tokenId);
      const metadata = tokenUri.substring(29);
      const decodedUint8Array = ethers.decodeBase64(metadata);
      const decoded = ethers.toUtf8String(decodedUint8Array);
      const parsed = JSON.parse(decoded);

      expect(parsed['name']).to.equal('Lit PKP #' + tokenId.toString());
      expect(parsed['attributes'][0]['value']).to.equal(pubkey);
      expect(parsed['attributes'][1]['value'].toLowerCase()).to.equal(
        pkpEthAddress.toLowerCase()
      );
      expect(parsed['attributes'][2]['value']).to.equal(tokenId.toString());
    });
  });

  describe('Attempt to claim derived PKP NFT', async () => {
    let minter;

    beforeEach(async function () {
      [minter, ...signers] = signers;
      pkpContract = pkpContract.connect(minter);
    });

    it('mints successfully', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const derivedKeyId = ethers.randomBytes(32);
      const sigs = await Promise.all(
        stakingAccounts.map(async (stakingAccount) =>
          ethers.Signature.from(
            await stakingAccount.nodeAddress.signMessage(derivedKeyId)
          )
        )
      );
      const tx = await pkpContract.claimAndMint(
        2,
        derivedKeyId,
        sigs,
        transaction
      );
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      // check the metadata
      const pkpEthAddress = await pkpContract.getEthAddress(tokenId);

      const tokenUri = await pkpContract.tokenURI(tokenId);
      const metadata = tokenUri.substring(29);
      const decodedUint8Array = ethers.decodeBase64(metadata);
      const decoded = ethers.toUtf8String(decodedUint8Array);
      const parsed = JSON.parse(decoded);

      expect(parsed['name']).to.equal('Lit PKP #' + tokenId.toString());
      expect(parsed['attributes'][0]['value']).to.equal(pubkey);
      expect(parsed['attributes'][1]['value'].toLowerCase()).to.equal(
        pkpEthAddress.toLowerCase()
      );
      expect(parsed['attributes'][2]['value']).to.equal(tokenId.toString());
    });
  });

  describe('Test Mint Grant And Burn', async () => {
    let minter;

    beforeEach(async function () {
      [minter, ...signers] = signers;
      pkpContract = pkpContract.connect(minter);
    });

    it('mints, grants, and burns successfully', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const ipfsIdToPermit = 'QmW6uH8p17DcfvZroULkdEDAKThWzEDeNtwi9oezURDeXN';
      const ipfsIdBytes = getBytesFromMultihash(ipfsIdToPermit);

      const tx = await pkpContract.mintGrantAndBurnNext(
        2,
        ipfsIdBytes,
        transaction
      );
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);
      expect(tokenId.toString().length).to.be.greaterThan(0);
      expect(pubkey.length).to.be.equal(132);

      // check the token was minted and burned
      expect(pkpContract.ownerOf(tokenId)).revertedWith(
        'ERC721: invalid token ID'
      );

      const actionIsPermitted = await pkpPermissions.isPermittedAction(
        tokenId,
        ipfsIdBytes
      );

      expect(actionIsPermitted).to.equal(true);
    });
  });
});

function supportsArbitrumStylus(hreNetworkConfig) {
  return !!hreNetworkConfig.stylusContractsForTests;
}
