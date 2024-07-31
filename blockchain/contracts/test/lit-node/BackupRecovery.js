const chai = require('chai');
const { BigNumber, utils } = require('ethers-v5');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  StakingState,
} = require('./../../utils/contract');
const { expect } = chai;
const { deployDiamond } = require('./../../scripts/deployDiamond');
const { ethers } = require('hardhat');

describe('BackupRecovery', function () {
  let deployer;
  let signers;
  let token;
  let routerContract;
  let pkpNft;
  let stakingAccount1;
  let stakingAccount2;
  let stakingContract;
  let stakingBalances;
  let contractResolver;
  let backupRecoveryContract;
  let minStake;
  let stakingAccounts = [];
  let backupPartyAccounts;
  let backupAddresses = [];
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals
  const stakingAccount1IpAddress = '192.168.1.1';
  const stakingAccount1Port = 7777;
  const stakingAccountCount = 10;
  const backupPartyCount = 5;

  function generateBackupPartyWallets(provider, count = backupPartyCount) {
    const partyMembers = [];
    for (let i = 0; i < count; i++) {
      const wallet = ethers.Wallet.createRandom().connect(provider);
      partyMembers.push({
        signer: wallet,
        address: wallet.address,
      });
    }

    return partyMembers;
  }

  beforeEach(async function () {
    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);

    // deploy token
    [
      deployer,
      stakingAccount1,
      nodeAccount1,
      stakingAccount2,
      nodeAccount2,
      ...signers
    ] = await ethers.getSigners();
    token = await ethers.deployContract(
      'LITToken',
      [ethers.parseUnits('1000000000', 18)] // 1b tokens
    );
    token = token.connect(deployer);

    // deploy staking balances
    const { diamond: stakingBalancesDiamond } = await deployDiamond(
      'StakingBalances',
      contractResolver.getAddress(),
      Environment.DEV,
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

    // deploy staking contract
    const { diamond: stakingDiamond } = await deployDiamond(
      'Staking',
      await contractResolver.getAddress(),
      Environment.DEV,
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
    stakingViewsFacet = await ethers.getContractAt(
      'StakingViewsFacet',
      await stakingDiamond.getAddress()
    );
    ownershipFacet = await ethers.getContractAt(
      'OwnershipFacet',
      await stakingDiamond.getAddress()
    );
    stakingVersionFacet = await ethers.getContractAt(
      'StakingVersionFacet',
      await stakingDiamond.getAddress()
    );

    // deploy pkpNft
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
    pkpNft = await ethers.getContractAt(
      'PKPNFTFacet',
      await pkpDiamond.getAddress()
    );

    // deploy router
    let deployResult = await deployDiamond(
      'PubkeyRouter',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PubkeyRouterFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    routerDiamond = deployResult.diamond;
    routerContract = await ethers.getContractAt(
      'PubkeyRouterFacet',
      await routerDiamond.getAddress()
    );

    backupPartyAccounts = generateBackupPartyWallets(deployer.provider);
    backupAddresses = [];
    for (let i = 0; i < backupPartyAccounts.length; i++) {
      backupAddresses.push(backupPartyAccounts[i].address);
    }

    const { diamond: backupRecoveryDiamond } = await deployDiamond(
      'BackupRecovery',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['BackupRecoveryFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    backupRecoveryContract = await ethers.getContractAt(
      'BackupRecoveryFacet',
      await backupRecoveryDiamond.getAddress()
    );

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract: token,
      stakingContract: stakingContract,
      stakingBalancesContract: stakingBalances,
      pkpContract: pkpNft,
      pubkeyRouterContract: routerContract,
      backupRecoveryContract: backupRecoveryContract,
    });

    await token.mint(deployer.address, totalTokens);
    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingContract,
      stakingBalances,
      token,
      deployer,
      {
        numValidators: stakingAccountCount,
        startingPort: stakingAccount1Port,
        ipAddress: stakingAccount1IpAddress,
      }
    );

    // fund stakingAccount1, stakingAccount2 with tokens
    minStake = await stakingBalances.minimumStake();
    const totalToStake = minStake * 3n; // 3 times the minimum stake
    await token.transfer(stakingAccount1.address, totalToStake);
    await token
      .connect(stakingAccount1)
      .approve(await stakingBalances.getAddress(), totalToStake);
    await token.transfer(stakingAccount2.address, totalToStake);
    await token
      .connect(stakingAccount2)
      .approve(await stakingBalances.getAddress(), totalToStake);

    for (let i = 0; i < backupPartyAccounts.length; i++) {
      await deployer.sendTransaction({
        to: backupAddresses[i],
        value: ethers.parseEther('1.0'),
      });
    }

    await backupRecoveryContract.registerNewBackupParty(backupAddresses);
  });

  it('should have 2/3 threshold of party members', async () => {
    const threshold = await backupRecoveryContract.getDecryptionThreshold();
    expect(threshold).equal(3n);
  });

  it('should not move nextState until all backup parties register keys', async () => {
    let backupContract;

    let pubkey =
      '028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071';

    let blsKey =
      '028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071';

    let curve = Buffer.from('SECP256K1').toString('hex');
    let sessionId = `0x${pubkey}${blsKey}`;

    for (let i = 0; i < backupPartyAccounts.length - 1; i++) {
      backupContract = backupRecoveryContract.connect(
        backupPartyAccounts[i].signer
      );

      const tx = await backupContract.recieveNewKeySet(
        `0x${pubkey}`,
        `0x${blsKey}`,
        sessionId
      );
      await tx.wait();
    }

    let recState = await backupContract.getBackupPartyState();

    expect(recState[0]).equal('0x');
    expect(recState[1]).equal(`0x`);
    expect(recState[2]).equal(`0x`);
    expect(recState[3]).equal(0n);
  });

  it('should revert in getting node addresses for dkg', async () => {
    let backupContract;
    let tx = await backupRecoveryContract.registerNewBackupParty(
      backupAddresses
    );
    await tx.wait();
    for (let i = 0; i < 2; i++) {
      backupContract = backupRecoveryContract.connect(
        stakingAccounts[i].nodeAddress
      );

      let nodeForDkgTx = await backupContract.setMemberForDkg();
      let receipt = await nodeForDkgTx.wait();
      let bp = await backupContract.getMemberForNodeDkg();
      expect(backupAddresses.indexOf(bp) > -1).to.be.true;
      let isMember = await backupContract.isNodeForDkg();
      expect(isMember).to.be.true;
    }

    expect(backupContract.getNodeAddressesForDkg()).to.be.rejectedWith(
      'BackupSetIncomplete'
    );
  });

  it('should use the same backup party address once mapped to a node', async () => {
    let backupContract;

    backupContract = backupRecoveryContract.connect(
      stakingAccounts[0].nodeAddress
    );

    let nodeForDkgTx = await backupContract.setMemberForDkg();
    let receipt = await nodeForDkgTx.wait();
    let bp = await backupContract.getMemberForNodeDkg();

    let nodeForDkgTx2 = await backupContract.setMemberForDkg();
    let receipt2 = await nodeForDkgTx.wait();
    let bp2 = await backupContract.getMemberForNodeDkg();
    expect(bp).to.equal(bp2);
  });

  it('should register keys and return backup state', async () => {
    let backupContract;

    let pubkey =
      '028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071';

    let blsKey =
      '028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071';

    let sessionId = `0x${pubkey}${blsKey}`;

    for (let i = 0; i < backupPartyAccounts.length; i++) {
      backupContract = backupRecoveryContract.connect(
        backupPartyAccounts[i].signer
      );

      const tx = await backupContract.recieveNewKeySet(
        `0x${pubkey}`,
        `0x${blsKey}`,
        sessionId
      );
      await tx.wait();
    }

    let recState = await backupContract.getBackupPartyState();

    expect(recState[0]).equal(sessionId);
    expect(recState[1]).equal(`0x${pubkey}`);
    expect(recState[2]).equal(`0x${blsKey}`);
    expect(recState[3]).equal(3n);
  });

  it('should register new backup party', async () => {
    const newBackupPartyAccounts = generateBackupPartyWallets(
      deployer.provider,
      backupPartyCount + 2
    );

    const newBackupAddresses = [];
    for (let i = 0; i < newBackupPartyAccounts.length; i++) {
      newBackupAddresses.push(newBackupPartyAccounts[i].address);
    }

    // connect the account which deployed the contract, the owner in this case
    backupContract = backupRecoveryContract.connect(deployer);
    const partyTx = await backupContract.registerNewBackupParty(
      newBackupAddresses
    );
    await partyTx.wait();

    const threshold = await backupContract.getDecryptionThreshold();
    expect(threshold).equal(4n);
  });

  it('should register recovery keys', async () => {
    let backupContract;

    let tx = await backupRecoveryContract.registerNewBackupParty(
      backupAddresses
    );
    await tx.wait();

    // First set backup to peer mappings
    for (let i = 0; i < stakingAccounts.length; i++) {
      backupContract = backupRecoveryContract.connect(
        stakingAccounts[i].nodeAddress
      );
      if (i < backupPartyCount) {
        let nodeForDkgTx = await backupContract.setMemberForDkg();
        let receipt = await nodeForDkgTx.wait();
        let bp = await backupContract.getMemberForNodeDkg();
        expect(backupAddresses.indexOf(bp) > -1).to.be.true;

        let isMember = await backupContract.isNodeForDkg();
        expect(isMember).to.be.true;
      }
    }

    const blsKey = {
      pubkey:
        '0x028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071',
      keyType: 1,
    };

    // ECDSA pubKey should be different from BLS pubKey
    const ecdsaKey = {
      pubkey:
        '0x018506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071',
      keyType: 2,
    };

    // Non-backup mapped node fails to register recovery keys
    backupContract = backupRecoveryContract.connect(
      stakingAccounts[backupPartyCount].nodeAddress
    );
    await expect(
      backupContract.registerRecoveryKeys([blsKey, ecdsaKey])
    ).to.be.revertedWith(
      'BackupRecovery: not a member of the Recovery DKG peer group'
    );

    // Register Recovery Pubkeys
    for (let i = 0; i < backupPartyCount - 1; i++) {
      backupContract = backupRecoveryContract.connect(
        stakingAccounts[i].nodeAddress
      );

      const tx = await backupContract.registerRecoveryKeys([blsKey, ecdsaKey]);
      await tx.wait();

      expect(await backupRecoveryContract.isRecoveryDkgCompleted()).to.be.false;
    }

    // Can't register keys twice
    backupContract = backupRecoveryContract.connect(
      stakingAccounts[0].nodeAddress
    );
    await expect(
      backupContract.registerRecoveryKeys([blsKey, ecdsaKey])
    ).to.be.revertedWith(
      'BackupRecovery: validator has already voted for this recovery key'
    );

    // Final registration should set the key
    backupContract = backupRecoveryContract.connect(
      stakingAccounts[backupPartyCount - 1].nodeAddress
    );
    tx = await backupContract.registerRecoveryKeys([blsKey, ecdsaKey]);
    await tx.wait();
    expect(await backupRecoveryContract.isRecoveryDkgCompleted()).to.be.true;

    // Can't register keys once it's already set for the Recovery DKG
    backupContract = backupRecoveryContract.connect(
      stakingAccounts[0].nodeAddress
    );
    await expect(
      backupContract.registerRecoveryKeys([blsKey, ecdsaKey])
    ).to.be.revertedWith(
      'BackupRecovery: recovery keys already set for this Recovery DKG'
    );
  });

  it('should map validators to backup until equals backup count', async () => {
    let backupContract;

    let tx = await backupRecoveryContract.registerNewBackupParty(
      backupAddresses
    );
    await tx.wait();

    for (let i = 0; i < stakingAccounts.length; i++) {
      backupContract = backupRecoveryContract.connect(
        stakingAccounts[i].nodeAddress
      );
      if (i < backupPartyCount) {
        let nodeForDkgTx = await backupContract.setMemberForDkg();
        let receipt = await nodeForDkgTx.wait();
        let bp = await backupContract.getMemberForNodeDkg();
        expect(backupAddresses.indexOf(bp) > -1).to.be.true;

        let isMember = await backupContract.isNodeForDkg();
        expect(isMember).to.be.true;
      } else {
        let isMember = await backupContract.isNodeForDkg();
        expect(isMember).to.be.false;
        let nodeForDkgTx = backupContract.setMemberForDkg();
        expect(nodeForDkgTx).to.be.rejectedWith(
          'NodesAllMappedToBackupMembers'
        );

        let peerAddresses = await backupContract.getNodeAddressesForDkg();
        expect(peerAddresses.length).to.equal(backupAddresses.length);
        break; // break out of the loop once there is a revert
      }
    }

    for (let i = 0; i < backupPartyAccounts.length; i++) {
      backupContract = backupRecoveryContract.connect(
        backupPartyAccounts[i].signer
      );
      let pubkey =
        '028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071';

      let blsKey =
        '028506cbedca1d12788d6bc74627d99263c93204d2e9565d861b7c1270736b0071';

      let sessionId = `0x${pubkey}${blsKey}`;
      const tx = await backupContract.recieveNewKeySet(
        `0x${pubkey}`,
        `0x${blsKey}`,
        sessionId
      );
      await tx.wait();
    }
  });

  it('should submit valid proof', async () => {
    let backupContract = backupRecoveryContract.connect(
      backupPartyAccounts[0].signer
    );

    let proof = new Uint8Array([
      1, 0, 0, 0, 0, 101, 92, 43, 178, 0, 157, 137, 108, 202, 42, 239, 133, 106,
      124, 17, 78, 140, 254, 165, 166, 3, 68, 236, 72, 237, 26, 60, 125, 231,
      225, 12, 198, 231, 69, 129, 98, 109, 10, 125, 19, 128, 146, 177, 152, 192,
      20, 234, 151, 23, 232, 132, 192, 3, 16, 94, 72, 223, 175, 141, 9, 136,
      150, 119, 236, 165, 211, 136, 243, 6, 175, 213, 176, 39, 182, 105, 20,
      182, 3, 76, 186, 159, 25, 55, 132, 193, 4, 131, 33, 255, 109, 25, 248, 87,
      34, 197, 244, 124, 144, 117, 142, 200, 243, 140, 168, 103, 244, 154, 71,
      158, 211, 131, 180, 42, 189, 242, 137, 170, 2, 61, 106, 241, 24, 60, 97,
      169, 160, 126, 36, 139, 117, 207, 195, 70, 18, 148, 72, 60, 5, 98, 15,
      242, 4, 228, 55, 81, 61, 187, 184, 79, 250, 202, 214, 148, 29, 54, 183,
      128, 31, 56, 98, 216, 97, 144, 112, 206, 7, 62, 245, 2, 197, 51, 240, 12,
      2, 139, 72, 208, 82, 192, 50, 72, 237, 47, 90, 92, 197, 233, 31, 36, 161,
      76, 144, 79, 52, 57, 215, 43, 204, 175, 236, 205, 109, 130, 15, 40, 158,
      218, 244, 129, 136, 4, 126, 85, 15, 34, 7, 188, 110, 29, 83, 56, 69, 229,
      9, 136, 65, 119, 65, 76, 68, 21, 191, 241, 236, 148, 123, 0, 117, 226,
      132, 199, 220, 249, 105, 68, 218, 45, 248, 229, 104, 106, 172, 219, 254,
      141, 225, 65, 209, 175, 70, 179,
    ]);
    const tx = await backupContract.recieveProofBls12381G1(proof);
    await tx.wait();
    let proofCount =
      await backupContract.getProofSubmissionForBackupPartyMember();
    console.log('proof resut: ', proofCount);
  });
});
