const chai = require('chai');
const { ip2int, int2ip } = require('../../utils');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  StakingState,
  createValidatorAndStake,
} = require('../../utils/contract');
const { expect } = chai;
const { deployDiamond } = require('../../scripts/deployDiamond');
const { toBigInt, randomBytes } = require('ethers');
const { staking } = require('../../typechain-types/contracts/lit-node');

describe('Staking', function () {
  let deployer;
  let signers;
  let token;
  let routerContract;
  let pkpNft;
  let stakingAccount1;
  let nodeAccount1;
  let stakingAccount2;
  let nodeAccount2;
  let stakingContract;
  let stakingViewsFacet;
  let stakingVersionFacet;
  let stakingAdminFacet;
  let ownershipFacet;
  let stakingBalances;
  let contractResolver;
  let minStake;
  let stakingAccounts = [];
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals
  const stakingAccount1IpAddress = '192.168.1.1';
  const stakingAccount1Port = 7777;
  const stakingAccount2IpAddress = '192.168.1.2';
  const stakingAccount2Port = 7777;
  const stakingAccountCount = 10;
  let snapshotId;

  before(async function () {
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

    // deploy staking contract
    const { diamond: stakingDiamond } = await deployDiamond(
      'Staking',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: [
          'StakingFacet',
          'StakingViewsFacet',
          'StakingVersionFacet',
          'StakingAdminFacet',
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
    stakingAdminFacet = await ethers.getContractAt(
      'StakingAdminFacet',
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
      0,
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
      0,
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

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract: token,
      stakingContract,
      stakingBalancesContract: stakingBalances,
      pkpContract: pkpNft,
      pubkeyRouterContract: routerContract,
    });

    await token.mint(deployer.address, totalTokens);
    stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingContract,
      stakingAdminFacet,
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
  });

  beforeEach(async () => {
    if (snapshotId) {
      await network.provider.send('evm_revert', [snapshotId]);
    }
    snapshotId = await network.provider.send('evm_snapshot');
  });

  describe('Constructor & Settings', function () {
    it('should set owner on constructor', async function () {
      const ownerAddress = await ownershipFacet.owner();
      expect(ownerAddress, deployer).is.equal;
    });
  });

  describe('querying state', () => {
    it('has the default validator set', async () => {
      const validators = await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validators.length).equal(10);
    });

    it('works with all the batch validator retrieval methods', async () => {
      const validatorsStructs =
        await stakingViewsFacet.getValidatorsStructsInCurrentEpoch();
      expect(validatorsStructs.length).equal(10);
      const validatorStakingAddresses =
        await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validatorStakingAddresses.length).equal(10);
      const ipAddress = ip2int(stakingAccount1IpAddress);
      for (let i = 0; i < validatorsStructs.length; i++) {
        const validator = validatorsStructs[i];
        const balance = await stakingBalances.balanceOf(
          validatorStakingAddresses[i]
        );
        expect(validator.nodeAddress).equal(
          await stakingAccounts[i].nodeAddress.address
        );
        expect(validator.ip).equal(ipAddress);
        expect(validator.port).equal(stakingAccount1Port + i + 1);
        expect(balance).equal(minStake);
      }
      const nodeConnectionInfo =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts();
      console.log(
        `nodeConnectionInfo: ${JSON.stringify(
          nodeConnectionInfo,
          (key, value) => (typeof value === 'bigint' ? value.toString() : value)
        )}`
      );
      const epoch = nodeConnectionInfo[0];
      const currentValidatorCountForConsensus = nodeConnectionInfo[1];
      const validators = nodeConnectionInfo[2];
      expect(epoch.number).equal(2);
      expect(currentValidatorCountForConsensus).equal(6);
      expect(validators.length).equal(10);
      for (let i = 0; i < validators.length; i++) {
        const validator = validators[i];
        const balance = await stakingBalances.balanceOf(
          validatorStakingAddresses[i]
        );
        expect(validator.nodeAddress).equal(
          await stakingAccounts[i].nodeAddress.address
        );
        expect(validator.ip).equal(ipAddress);
        expect(validator.port).equal(stakingAccount1Port + i + 1);
        expect(balance).equal(minStake);
      }
    });
  });

  describe('invalid staking scenarios', function () {
    it('cannot stake 0', async () => {
      stakingContract = stakingContract.connect(stakingAccount1);
      expect(
        stakingContract.stakeAndJoin(
          0,
          ip2int(stakingAccount1IpAddress),
          0,
          7777,
          nodeAccount1.address,
          toBigInt(randomBytes(32)),
          toBigInt(randomBytes(32))
        )
      ).revertedWithCustomError(stakingContract, 'CannotStakeZero');
    });

    it('cannot stake less than the minimum stake', async () => {
      stakingContract = stakingContract.connect(stakingAccount2);
      token = token.connect(stakingAccount2);
      await token.approve(await stakingBalances.getAddress(), minStake);

      expect(
        stakingContract.stakeAndJoin(
          minStake - 1n,
          ip2int(stakingAccount2IpAddress),
          0,
          7777,
          nodeAccount2.address,
          toBigInt(randomBytes(32)),
          toBigInt(randomBytes(32))
        )
      ).revertedWithCustomError(
        stakingBalances,
        'StakeMustBeGreaterThanMinimumStake'
      );
    });

    it('devopsAdmin functionality works', async () => {
      stakingAdminFacet = stakingAdminFacet.connect(stakingAccount1);

      const stakingUtilsLib = await ethers.getContractAt(
        'StakingUtilsLib',
        await stakingAdminFacet.getAddress()
      );

      // try to use an admin function without being the devopsAdmin
      await expect(
        stakingAdminFacet.adminKickValidatorInNextEpoch(
          stakingAccounts[0].stakingAddress
        )
      ).to.be.revertedWithCustomError(
        stakingUtilsLib,
        'CallerNotOwnerOrDevopsAdmin'
      );

      // try to set the devopsAdmin to a non-owner
      await expect(
        stakingAdminFacet.setDevopsAdmin(stakingAccount2.address)
      ).to.be.revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner');

      // actually set the devopsAdmin
      stakingAdminFacet = stakingAdminFacet.connect(deployer);
      await stakingAdminFacet.setDevopsAdmin(stakingAccount1.address);
      stakingAdminFacet = stakingAdminFacet.connect(stakingAccount1);

      // now the admin function should work
      stakingAdminFacet = stakingAdminFacet.connect(stakingAccount1);
      await stakingAdminFacet.adminKickValidatorInNextEpoch(
        stakingAccounts[0].stakingAddress
      );

      // confirm the validator was kicked
      const validatorsInNextEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpoch.length).equal(9);
      expect(validatorsInNextEpoch.includes(stakingAccounts[0].stakingAddress))
        .to.be.false;
    });
  });

  describe('validator joining', function () {
    it('cannot join with the same comms keys', async function () {
      // cannot join with the same comms keys
      stakingContract = stakingContract.connect(stakingAccount1);
      await expect(
        stakingContract.stakeAndJoin(
          minStake,
          ip2int(stakingAccount1IpAddress),
          0,
          stakingAccount1Port,
          nodeAccount1.address,
          stakingAccounts[0].commsKeys.sender,
          stakingAccounts[0].commsKeys.receiver
        )
      )
        .to.be.revertedWithCustomError(stakingContract, `CannotReuseCommsKeys`)
        .withArgs(
          stakingAccounts[0].commsKeys.sender,
          stakingAccounts[0].commsKeys.receiver
        );
    });

    it('can join as a validator, and cannot leave as a staker if below min validator count', async () => {
      let initialStakeBal = await stakingBalances.balanceOf(
        stakingAccount1.address
      );
      let initialTokenBalance = await token.balanceOf(stakingAccount1.address);
      let initialValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );
      const initialIpAddress = initialValidatorEntry.ip;
      const initialPort = initialValidatorEntry.port;
      const initialNodeAddresss = initialValidatorEntry.nodeAddress;
      const initialSenderPubKey = initialValidatorEntry.senderPubKey;
      const initialReceiverPubKey = initialValidatorEntry.receiverPubKey;
      let initialReward = initialValidatorEntry.reward;
      const initialNodeAddressToStakerAddress =
        await stakingViewsFacet.nodeAddressToStakerAddress(
          nodeAccount1.address
        );

      // generate new unused communication keys
      const communicationSenderPubKey = toBigInt(randomBytes(32));
      const communicationReceiverPubKey = toBigInt(randomBytes(32));

      // can only join if permitted
      await stakingBalances.setPermittedStakersOn(true);

      stakingContract = stakingContract.connect(stakingAccount1);
      await expect(
        stakingContract.stakeAndJoin(
          minStake,
          ip2int(stakingAccount1IpAddress),
          0,
          stakingAccount1Port,
          nodeAccount1.address,
          communicationSenderPubKey,
          communicationReceiverPubKey
        )
      )
        .to.be.revertedWithCustomError(stakingContract, `StakerNotPermitted`)
        .withArgs(stakingAccount1.address);

      // permit the staker
      stakingBalances = stakingBalances.connect(deployer);
      await stakingBalances.addPermittedStaker(stakingAccount1.address);
      stakingContract = stakingContract.connect(stakingAccount1);

      await stakingContract.stakeAndJoin(
        minStake,
        ip2int(stakingAccount1IpAddress),
        0,
        stakingAccount1Port,
        nodeAccount1.address,
        communicationSenderPubKey,
        communicationReceiverPubKey
      );

      let postStakeBal = await stakingBalances.balanceOf(
        stakingAccount1.address
      );
      let postTokenBalance = await token.balanceOf(stakingAccount1.address);
      let postValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );
      const postIpAddress = postValidatorEntry.ip;
      const postPort = postValidatorEntry.port;
      const postNodeAddress = postValidatorEntry.nodeAddress;
      const postSenderPubKey = postValidatorEntry.senderPubKey;
      const postReceiverPubKey = postValidatorEntry.receiverPubKey;
      let postBalance = await stakingBalances.balanceOf(
        stakingAccount1.address
      );
      let postReward = postValidatorEntry.reward;
      let postNodeAddressToStakerAddress =
        await stakingViewsFacet.nodeAddressToStakerAddress(
          nodeAccount1.address
        );

      expect(postTokenBalance).to.be.lt(initialTokenBalance);
      expect(postStakeBal).to.be.gt(initialStakeBal);
      expect(initialIpAddress).to.equal(0);
      expect(int2ip(postIpAddress)).to.equal(stakingAccount1IpAddress);
      expect(initialPort).to.equal(0);
      expect(postPort).to.equal(stakingAccount1Port);
      expect(initialNodeAddresss).to.equal(
        '0x0000000000000000000000000000000000000000'
      );
      expect(postNodeAddress).to.equal(await nodeAccount1.address);
      expect(initialSenderPubKey).to.equal(0);
      expect(postSenderPubKey).to.be.equal(communicationSenderPubKey);
      expect(initialReceiverPubKey).to.equal(0);
      expect(postReceiverPubKey).to.equal(communicationReceiverPubKey);
      expect(initialStakeBal).to.equal(0);
      expect(postBalance).to.equal(minStake);
      expect(initialReward).to.equal(0);
      expect(postReward).to.equal(0);

      expect(initialNodeAddressToStakerAddress).to.equal(
        '0x0000000000000000000000000000000000000000'
      );
      expect(postNodeAddressToStakerAddress).to.equal(
        await stakingAccount1.address
      );

      // turn off permitted stakers
      await stakingBalances.setPermittedStakersOn(false);

      // set min validator count to 11 which is the current validator count
      stakingAdminFacet = stakingAdminFacet.connect(deployer);

      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        11n
      );

      // we will leave with stakingAccount1
      token = token.connect(stakingAccount1);
      stakingContract = stakingContract.connect(stakingAccount1);

      initialStakeBal = await stakingBalances.balanceOf(
        stakingAccount1.address
      );
      initialTokenBalance = await token.balanceOf(stakingAccount1.address);
      initialValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );
      const initialBalance = await stakingBalances.balanceOf(
        stakingAccount1.address
      );
      initialReward = initialValidatorEntry.reward;

      expect(stakingContract.requestToLeave()).revertedWithCustomError(
        stakingContract,
        'NotEnoughValidatorsInNextEpoch'
      );

      stakingContract = stakingContract.connect(deployer);
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        7n
      );

      postStakeBal = await stakingBalances.balanceOf(stakingAccount1.address);
      postTokenBalance = await token.balanceOf(stakingAccount1.address);
      postValidatorEntry = await stakingViewsFacet.validators(
        stakingAccount1.address
      );

      postBalance = await stakingBalances.balanceOf(stakingAccount1.address);
      postReward = postValidatorEntry.reward;
      postNodeAddressToStakerAddress =
        await stakingViewsFacet.nodeAddressToStakerAddress(
          nodeAccount1.address
        );

      expect(postTokenBalance).to.equal(initialTokenBalance);
      expect(postStakeBal).to.equal(initialStakeBal);
      expect(initialBalance).to.equal(postBalance);
      expect(postBalance).to.equal(minStake);
      expect(initialReward).to.equal(0);
      expect(postReward).to.equal(0);
      expect(postNodeAddressToStakerAddress).to.equal(
        await stakingAccount1.address
      );
    });

    it('works with kicked nodes from the current epoch', async function () {
      const validatorsInNextEpochBeforeTest =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochBeforeTest.length).equal(10);

      let [epoch, currentValidatorCountForConsensus, allStructs] =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts();
      expect(allStructs.length).equal(10);

      const stakingAsAdmin = stakingAdminFacet.connect(deployer);

      // kick the first 3 nodes
      for (let i = 0; i < 3; i++) {
        const stakingAddress = stakingViewsFacet.nodeAddressToStakerAddress(
          allStructs[i].nodeAddress
        );
        await stakingAsAdmin.adminKickValidatorInNextEpoch(stakingAddress);
      }

      [epoch, currentValidatorCountForConsensus, allStructs] =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts();
      expect(allStructs.length).equal(7);
    });

    it('works with kicked nodes from the next epoch', async function () {
      const validatorsInNextEpochBeforeTest =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochBeforeTest.length).equal(10);

      let [epoch, currentValidatorCountForConsensus, allStructs] =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts();
      expect(allStructs.length).equal(10);

      const stakingAsAccount1 = stakingContract.connect(stakingAccount1);
      // generate new unused communication keys
      const communicationSenderPubKey = toBigInt(randomBytes(32));
      const communicationReceiverPubKey = toBigInt(randomBytes(32));
      await stakingAsAccount1.stakeAndJoin(
        minStake,
        ip2int(stakingAccount1IpAddress),
        0,
        stakingAccount1Port,
        nodeAccount1.address,
        communicationSenderPubKey,
        communicationReceiverPubKey
      );

      // still 10 nodes
      [epoch, currentValidatorCountForConsensus, allStructs] =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts();
      expect(allStructs.length).equal(10);

      // kick 1
      const stakingAsAdmin = stakingAdminFacet.connect(deployer);
      await stakingAsAdmin.adminKickValidatorInNextEpoch(
        stakingAccount1.address
      );

      // still 10 nodes
      [epoch, currentValidatorCountForConsensus, allStructs] =
        await stakingViewsFacet.getActiveUnkickedValidatorStructsAndCounts();
      expect(allStructs.length).equal(10);
    });

    it('can join as a validator and can leave', async function () {
      // stakingAccount1 requests to join
      // generate new unused communication keys
      const communicationSenderPubKey = toBigInt(randomBytes(32));
      const communicationReceiverPubKey = toBigInt(randomBytes(32));

      // can only join if permitted
      await stakingBalances.setPermittedStakersOn(true);
      // permit the staker
      stakingBalances = stakingBalances.connect(deployer);
      await stakingBalances.addPermittedStaker(stakingAccount1.address);
      stakingContract = stakingContract.connect(stakingAccount1);

      await stakingContract.stakeAndJoin(
        minStake,
        ip2int(stakingAccount1IpAddress),
        0,
        stakingAccount1Port,
        nodeAccount1.address,
        communicationSenderPubKey,
        communicationReceiverPubKey
      );

      // turn off permitted stakers
      await stakingBalances.setPermittedStakersOn(false);

      const validatorsInNextEpochBeforeTest =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochBeforeTest.length).equal(11);
      expect(
        validatorsInNextEpochBeforeTest.includes(await stakingAccount1.address)
      ).is.true;

      const epochBeforeAdvancingEpoch = await stakingViewsFacet.epoch();

      let currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      // make sure that we can't lock if less than min validator count
      stakingAdminFacet = stakingAdminFacet.connect(deployer);
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        12n
      );

      expect(
        stakingContract.lockValidatorsForNextEpoch()
      ).revertedWithCustomError(
        stakingContract,
        'NotEnoughValidatorsInNextEpoch'
      );
      // reset it back to 7
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        7n
      );

      // lock new validators
      await stakingContract.lockValidatorsForNextEpoch();

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      // validators should be unchanged
      const validators = await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validators.length).equal(10);

      // validators in next epoch should include stakingAccount1
      const validatorsInNextEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpoch.length).equal(11);
      expect(validatorsInNextEpoch[10]).equal(await stakingAccount1.address);

      // signal that we are ready to advance epoch

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const { nodeAddress } = stakingAccount;
        stakingContract = stakingContract.connect(nodeAddress);
        await stakingContract.signalReadyForNextEpoch(2);
      }

      // try advancing before validators all signalled
      await expect(stakingContract.advanceEpoch())
        .revertedWithCustomError(
          stakingContract,
          'MustBeInReadyForNextEpochState'
        )
        .withArgs(1);

      stakingContract = stakingContract.connect(nodeAccount1);
      await stakingContract.signalReadyForNextEpoch(2);

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingContract = stakingContract.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingContract.advanceEpoch();

      const epochAfterAdvancingEpoch = await stakingViewsFacet.epoch();

      // advancing the epoch should reset validatorsForNextEpochLocked
      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      expect(epochAfterAdvancingEpoch.number).to.equal(
        epochBeforeAdvancingEpoch.number + 1n
      );

      // validators should include stakingAccount1
      let validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validatorsAfterAdvancingEpoch.length).equal(11);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).is.true;

      const validatorsInNextEpochBefore =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochBefore.length).equal(11);
      expect(
        validatorsInNextEpochBefore.includes(await stakingAccount1.address)
      ).is.true;

      // attempt to leave
      await stakingContract.requestToLeave();

      const validatorsInNextEpochAfter =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochAfter.length).equal(10);
      expect(validatorsInNextEpochAfter.includes(await stakingAccount1.address))
        .is.false;

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      // create the new validator set
      await stakingContract.lockValidatorsForNextEpoch();

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const nodeAddress = stakingAccount.nodeAddress;
        stakingContract = stakingContract.connect(nodeAddress);
        await stakingContract.signalReadyForNextEpoch(3);
      }

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingContract = stakingContract.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingContract.advanceEpoch();

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validatorsAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).to.be.false;

      const validatorsInNextEpochAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsInNextEpochAfterAdvancingEpoch.includes(
          await stakingAccount1.address
        )
      ).to.be.false;
    });

    it('can join as a validator and the node can request to leave', async function () {
      // stakingAccount1 requests to join
      // generate new unused communication keys
      const communicationSenderPubKey = toBigInt(randomBytes(32));
      const communicationReceiverPubKey = toBigInt(randomBytes(32));

      // can only join if permitted
      await stakingBalances.setPermittedStakersOn(true);
      // permit the staker
      stakingBalances = stakingBalances.connect(deployer);
      await stakingBalances.addPermittedStaker(stakingAccount1.address);
      stakingContract = stakingContract.connect(stakingAccount1);

      await stakingContract.stakeAndJoin(
        minStake,
        ip2int(stakingAccount1IpAddress),
        0,
        stakingAccount1Port,
        nodeAccount1.address,
        communicationSenderPubKey,
        communicationReceiverPubKey
      );

      // turn off permitted stakers
      await stakingBalances.setPermittedStakersOn(false);

      const validatorsInNextEpochBeforeTest =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochBeforeTest.length).equal(11);
      expect(
        validatorsInNextEpochBeforeTest.includes(await stakingAccount1.address)
      ).is.true;

      const epochBeforeAdvancingEpoch = await stakingViewsFacet.epoch();

      let currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      // make sure that we can't lock if less than min validator count
      stakingAdminFacet = stakingAdminFacet.connect(deployer);
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        12n
      );

      expect(
        stakingContract.lockValidatorsForNextEpoch()
      ).revertedWithCustomError(
        stakingContract,
        'NotEnoughValidatorsInNextEpoch'
      );
      // reset it back to 7
      await updateMinimumValidatorCount(
        stakingViewsFacet,
        stakingAdminFacet,
        7n
      );

      // lock new validators
      await stakingContract.lockValidatorsForNextEpoch();

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      // validators should be unchanged
      const validators = await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validators.length).equal(10);

      // validators in next epoch should include stakingAccount1
      const validatorsInNextEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpoch.length).equal(11);
      expect(validatorsInNextEpoch[10]).equal(await stakingAccount1.address);

      // signal that we are ready to advance epoch

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const { nodeAddress } = stakingAccount;
        stakingContract = stakingContract.connect(nodeAddress);
        await stakingContract.signalReadyForNextEpoch(2);
      }

      // try advancing before validators all signalled
      await expect(stakingContract.advanceEpoch())
        .revertedWithCustomError(
          stakingContract,
          'MustBeInReadyForNextEpochState'
        )
        .withArgs(1);

      stakingContract = stakingContract.connect(nodeAccount1);
      await stakingContract.signalReadyForNextEpoch(2);

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingContract = stakingContract.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingContract.advanceEpoch();

      const epochAfterAdvancingEpoch = await stakingViewsFacet.epoch();

      // advancing the epoch should reset validatorsForNextEpochLocked
      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      expect(epochAfterAdvancingEpoch.number).to.equal(
        epochBeforeAdvancingEpoch.number + 1n
      );

      // validators should include stakingAccount1
      let validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validatorsAfterAdvancingEpoch.length).equal(11);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).is.true;

      const validatorsInNextEpochBefore =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochBefore.length).equal(11);
      expect(
        validatorsInNextEpochBefore.includes(await stakingAccount1.address)
      ).is.true;

      // attempt to leave with the node account
      await stakingContract.connect(nodeAccount1).requestToLeaveAsNode();

      const validatorsInNextEpochAfter =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochAfter.length).equal(10);
      expect(validatorsInNextEpochAfter.includes(await stakingAccount1.address))
        .is.false;

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      // create the new validator set
      await stakingContract.lockValidatorsForNextEpoch();

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      for (let i = 0; i < stakingAccounts.length; i++) {
        const stakingAccount = stakingAccounts[i];
        const nodeAddress = stakingAccount.nodeAddress;
        stakingContract = stakingContract.connect(nodeAddress);
        await stakingContract.signalReadyForNextEpoch(3);
      }

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.ReadyForNextEpoch);

      stakingContract = stakingContract.connect(stakingAccount1);

      // advance the epoch.  this sets the validators to be the new set
      await stakingContract.advanceEpoch();

      currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.Active);

      validatorsAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInCurrentEpoch();
      expect(validatorsAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsAfterAdvancingEpoch.includes(await stakingAccount1.address)
      ).to.be.false;

      const validatorsInNextEpochAfterAdvancingEpoch =
        await stakingViewsFacet.getValidatorsInNextEpoch();
      expect(validatorsInNextEpochAfterAdvancingEpoch.length).equal(10);
      expect(
        validatorsInNextEpochAfterAdvancingEpoch.includes(
          await stakingAccount1.address
        )
      ).to.be.false;
    });

    it('kicks and slashes validator', async () => {
      const toBeKicked = stakingAccounts[stakingAccounts.length - 1];

      await makeAssertions(stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: 10,
        expectedValidatorsInNextEpoch: 10,
        expectedKickedValidatorsInNextEpoch: 0,
      });

      const kickedValidatorStakeBefore = await stakingBalances.balanceOf(
        toBeKicked.stakingAddress.address
      );
      const totalStakedBefore = await stakingBalances.totalStaked();

      // get epoch number
      const epoch = await stakingViewsFacet.epoch();

      // vote to kick the last stakingAccount
      for (let i = 0; i < stakingAccounts.length - 1; i++) {
        const stakingAccountToVoteFrom = stakingAccounts[i];
        await voteToKick(stakingContract, stakingViewsFacet, {
          stakingAccountToVoteFrom,
          stakingAccountToBeKicked: toBeKicked,
          epoch,
          expectedExistingNumVotes: i,
        });
      }

      await makeAssertions(stakingViewsFacet, {
        expectedValidatorsInNextEpoch: 9,
        expectedValidatorsInNextEpochExcludes: [
          await toBeKicked.stakingAddress.address,
        ],
        expectedKickedValidatorsInNextEpoch: 1,
      });

      // check that they were slashed
      const kickedValidatorStakeAfter = await stakingBalances.balanceOf(
        toBeKicked.stakingAddress.address
      );
      const kickPenaltyPercent =
        await stakingViewsFacet.kickPenaltyPercentByReason(1);
      const amountBurned =
        (kickedValidatorStakeBefore * kickPenaltyPercent) / 100n;

      expect(kickedValidatorStakeAfter.toString()).to.equal(
        (kickedValidatorStakeBefore - amountBurned).toString()
      );

      const totalStakedAfter = await stakingBalances.totalStaked();
      expect(totalStakedAfter.toString()).to.equal(
        (totalStakedBefore - amountBurned).toString()
      );

      let currentState = await stakingViewsFacet.state();
      expect(currentState).to.equal(StakingState.NextValidatorSetLocked);

      // Signal and advance epoch.
      await signalAndAdvanceEpoch(stakingContract, stakingViewsFacet, {
        stakingAccounts: stakingAccounts.slice(0, stakingAccounts.length - 1),
      });

      // Make assertions after advancing epoch
      await makeAssertions(stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: 9,
        expectedValidatorsInCurrentEpochExcludes: [
          await toBeKicked.stakingAddress.address,
        ],
        expectedValidatorsInNextEpoch: 9,
        expectedKickedValidatorsInNextEpoch: 0,
      });
    });

    it('cannot kick such that less than a threshold of validators from current set remain', async () => {
      let stakingAccountsCopy = [...stakingAccounts];

      // Get threshold count
      const thresholdCount = Number(
        await stakingViewsFacet.currentValidatorCountForConsensus()
      );

      // Get number of validator votes needed.
      const numVotesNeeded = thresholdCount;

      // Calculate number of successful kicks
      const numSuccessfulKicks = stakingAccountsCopy.length - thresholdCount;

      // get epoch number
      const epoch = await stakingViewsFacet.epoch();

      // Track the kicked validators
      let kickedValidators = [];

      for (let i = 0; i < numSuccessfulKicks; i++) {
        // Choose a random index to kick from
        const indexToKickFrom = Math.floor(
          Math.random() * stakingAccountsCopy.length
        );
        const stakingAccountToBeKicked = stakingAccountsCopy[indexToKickFrom];

        // Make assertions before kicking
        await makeAssertions(stakingViewsFacet, {
          expectedValidatorsInCurrentEpoch: 10,
          expectedValidatorsInNextEpoch: 10 - i,
        });

        // Vote to kick the stakingAccount
        let numVotes = 0;
        for (let j = 0; j < stakingAccountsCopy.length; j++) {
          if (numVotes === numVotesNeeded) {
            break;
          }

          const stakingAccountToVoteFrom = stakingAccountsCopy[j];

          if (stakingAccountToVoteFrom === stakingAccountToBeKicked) {
            // If the staking account to vote from is the same as the one to be kicked, then skip
            continue;
          }

          await voteToKick(stakingContract, stakingViewsFacet, {
            stakingAccountToVoteFrom,
            stakingAccountToBeKicked,
            epoch,
            expectedExistingNumVotes: numVotes,
          });

          numVotes++;
        }

        // Make assertions after kicking
        await makeAssertions(stakingViewsFacet, {
          expectedValidatorsInNextEpoch: 10 - i - 1,
          expectedValidatorsInNextEpochExcludes: [
            await stakingAccountToBeKicked.stakingAddress.address,
          ],
          expectedKickedValidatorsInNextEpoch: i + 1,
        });

        // Update stakingAccountsCopy with the kicked validator removed
        stakingAccountsCopy = stakingAccountsCopy.filter(
          (account) => account !== stakingAccountToBeKicked
        );

        kickedValidators.push(stakingAccountToBeKicked);
      }

      // After all the successful kicks, the next kick should result in a revert with custom error.

      // Choose to kick the last staker remaining
      const indexToKickFrom = stakingAccountsCopy.length - 1;
      const stakingAccountToBeKicked = stakingAccountsCopy[indexToKickFrom];

      // Vote to kick the stakingAccount
      const stakingAccountToVoteFrom = stakingAccountsCopy[0];
      const nodeAddress = stakingAccountToVoteFrom.nodeAddress;
      expect(
        stakingContract
          .connect(nodeAddress)
          .kickValidatorInNextEpoch(
            stakingAccountToBeKicked.stakingAddress.address,
            1,
            '0x'
          )
      ).revertedWithCustomError(
        stakingContract,
        'CannotKickBelowCurrentValidatorThreshold'
      );

      await makeAssertions(stakingViewsFacet, {
        expectedValidatorsInNextEpoch: thresholdCount,
      });

      // Now, try locking, signalling ready for next epoch and advancing epoch
      await lockSignalAndAdvanceEpoch(stakingContract, stakingViewsFacet, {
        stakingAccounts: stakingAccountsCopy,
      });

      // Make assertions after advancing epoch
      await makeAssertions(stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: thresholdCount,
        expectedValidatorsInCurrentEpochExcludes: kickedValidators,
        expectedValidatorsInNextEpochExcludes: kickedValidators,
        expectedValidatorsInNextEpoch: thresholdCount,
        expectedKickedValidatorsInNextEpoch: 0,
      });
    });
  });

  describe('Validators joining and leaving simultaneously', () => {
    it('2 leaves and 4 joins', async () => {
      const nodesRequestingToLeave = [stakingAccounts[0], stakingAccounts[1]];

      let nodesRequestingToJoin = [];
      for (let i = 0; i < 4; i++) {
        nodesRequestingToJoin.push(
          await createValidatorAndStake(
            ethers,
            stakingContract,
            stakingBalances,
            token,
            deployer,
            {
              ipAddress: stakingAccount1IpAddress,
              port: stakingAccount1Port,
            }
          )
        );
      }

      // nodes request to leave
      for (let i = 0; i < nodesRequestingToLeave.length; i++) {
        const node = nodesRequestingToLeave[i];
        await stakingContract.connect(node.stakingAddress).requestToLeave();
      }

      // nodes request to join
      for (let i = 0; i < nodesRequestingToJoin.length; i++) {
        const node = nodesRequestingToJoin[i];
        await stakingContract
          .connect(node.stakingAddress)
          .requestToJoin(
            ip2int(node.ip),
            0,
            node.port,
            node.nodeAddress.address,
            node.commsKeys.sender,
            node.commsKeys.receiver
          );
      }

      await makeAssertions(stakingViewsFacet, {
        expectedValidatorsInCurrentEpoch: 10,
        expectedValidatorsInNextEpoch: 12,
      });
    });
  });

  describe('setting new resolver contract address', () => {
    it('sets the new contract address', async () => {
      stakingAdminFacet = stakingAdminFacet.connect(deployer);
      const existingResolverContractAddress =
        await stakingViewsFacet.contractResolver();
      const newResolverContractAddress =
        '0xea1762E80ED1C54baCa25C7aF4E435FA1427C99E';
      expect(existingResolverContractAddress).not.equal(
        newResolverContractAddress
      );
      await stakingAdminFacet.setContractResolver(newResolverContractAddress);
      expect(await stakingViewsFacet.contractResolver()).equal(
        newResolverContractAddress
      );

      // revert this change
      await stakingAdminFacet.setContractResolver(
        existingResolverContractAddress
      );
    });
  });

  describe('Alias tests', () => {
    it('Can use an alias', async () => {
      // Create new stakers
      const newStakingAddress = ethers.Wallet.createRandom().connect(
        deployer.provider
      );
      const newNodeAddress = ethers.Wallet.createRandom().connect(
        deployer.provider
      );

      // Send them some gas
      const ethForGas = ethers.parseEther('1.0');
      await deployer.sendTransaction({
        to: newStakingAddress.address,
        value: ethForGas,
      });
      await deployer.sendTransaction({
        to: newNodeAddress.address,
        value: ethForGas,
      });

      // Send them some tokens
      const totalToStake = minStake * 3n; // 3 times the minimum stake
      await token
        .connect(deployer)
        .transfer(newStakingAddress.address, totalToStake);
      await token
        .connect(newStakingAddress)
        .approve(await stakingBalances.getAddress(), totalToStake);

      // Prepare joining parameters
      const ipAddress = ip2int(stakingAccount1IpAddress);
      const port = 1337;
      const nodeAddress = newNodeAddress.address;
      const comsKeys = toBigInt(randomBytes(32));

      // Stake and join
      await stakingContract
        .connect(newStakingAddress)
        .stakeAndJoin(
          minStake,
          ipAddress,
          0,
          port,
          nodeAddress,
          comsKeys,
          comsKeys
        );

      // create an alias
      const aliasStakingAddress = ethers.Wallet.createRandom().connect(
        deployer.provider
      );
      await deployer.sendTransaction({
        to: aliasStakingAddress.address,
        value: ethForGas,
      });

      await stakingBalances
        .connect(newStakingAddress)
        .addAlias(aliasStakingAddress.address);

      // show that the node can join with the alias address
      const aliasCommsKeys = toBigInt(randomBytes(32));
      await stakingContract
        .connect(aliasStakingAddress)
        .requestToJoin(
          ipAddress,
          0,
          port,
          nodeAddress,
          aliasCommsKeys,
          aliasCommsKeys
        );

      const anotherRandomAddress = ethers.Wallet.createRandom().connect(
        deployer.provider
      );
      await deployer.sendTransaction({
        to: anotherRandomAddress.address,
        value: ethForGas,
      });
      // show that only the newStakingAddress can remove
      await expect(
        stakingBalances
          .connect(anotherRandomAddress)
          .removeAlias(aliasStakingAddress.address)
      ).revertedWithCustomError(stakingBalances, 'AliasNotOwnedBySender');

      // test balance and reward of alias
    });
  });

  describe('Version tests', () => {
    it('Can get min and max version', async () => {
      let minVersionString = await stakingVersionFacet.getMinVersionString();
      expect(minVersionString).to.equal('0.0.0');

      let maxVersionString = await stakingVersionFacet.getMaxVersionString();
      expect(maxVersionString).to.equal('10000.0.0');

      let minVersion = await stakingVersionFacet.getMinVersion();
      expect(minVersion.major).to.equal(0);
      expect(minVersion.minor).to.equal(0);
      expect(minVersion.patch).to.equal(0);

      let maxVersion = await stakingVersionFacet.getMaxVersion();
      expect(maxVersion.major).to.equal(10000);
      expect(maxVersion.minor).to.equal(0);
      expect(maxVersion.patch).to.equal(0);
    });

    it('Can set min and max version', async () => {
      let isAllowedVersion = await stakingVersionFacet.checkVersion([
        20000, 0, 0,
      ]);
      expect(isAllowedVersion).to.equal(false);
      isAllowedVersion = await stakingVersionFacet.checkVersion([1, 1, 200]);
      expect(isAllowedVersion).to.equal(true);

      let minVersionBefore = await stakingVersionFacet.getMinVersion();
      expect(minVersionBefore.major).to.equal(0);
      expect(minVersionBefore.minor).to.equal(0);
      expect(minVersionBefore.patch).to.equal(0);

      let maxVersionBefore = await stakingVersionFacet.getMaxVersion();
      expect(maxVersionBefore.major).to.equal(10000);
      expect(maxVersionBefore.minor).to.equal(0);
      expect(maxVersionBefore.patch).to.equal(0);

      await stakingVersionFacet.setMinVersion([1, 2, 3]);
      let minVersionAfter = await stakingVersionFacet.getMinVersion();
      expect(minVersionAfter.major).to.equal(1);
      expect(minVersionAfter.minor).to.equal(2);
      expect(minVersionAfter.patch).to.equal(3);

      await stakingVersionFacet.setMaxVersion([4, 5, 6]);
      let maxVersionAfter = await stakingVersionFacet.getMaxVersion();
      expect(maxVersionAfter.major).to.equal(4);
      expect(maxVersionAfter.minor).to.equal(5);
      expect(maxVersionAfter.patch).to.equal(6);

      isAllowedVersion = await stakingVersionFacet.checkVersion([1, 2, 3]);
      expect(isAllowedVersion).to.equal(true);

      isAllowedVersion = await stakingVersionFacet.checkVersion([4, 5, 6]);
      expect(isAllowedVersion).to.equal(true);

      isAllowedVersion = await stakingVersionFacet.checkVersion([1, 1, 200]);
      expect(isAllowedVersion).to.equal(false);
    });
  });

  describe('only the admin can call admin functions', () => {
    it('tries to call the admin functions as a non admin and fails', async () => {
      stakingContract = stakingContract.connect(nodeAccount1);
      stakingBalances = stakingBalances.connect(nodeAccount1);
      const stakingUtilsLib = await ethers.getContractAt(
        'StakingUtilsLib',
        await stakingContract.getAddress()
      );

      expect(stakingAdminFacet.setEpochLength(25)).revertedWithCustomError(
        stakingUtilsLib,
        'CallerNotOwner()'
      );

      expect(stakingAdminFacet.setEpochTimeout(25)).revertedWithCustomError(
        stakingUtilsLib,
        'CallerNotOwner()'
      );

      expect(
        stakingAdminFacet.setConfig([
          25,
          1,
          10,
          [1, 2, 3],
          6,
          1000,
          10,
          25,
          10,
          10,
          true,
          {
            timeout_ms: 1000,
            memory_limit_mb: 100,
            max_code_length: 1000,
            max_response_length: 1000,
            max_fetch_count: 1000,
            max_sign_count: 1000,
            max_contract_call_count: 1000,
            max_broadcast_and_collect_count: 1000,
            max_call_depth: 1000,
            max_retries: 1000,
          },
        ])
      ).revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner()');

      expect(
        stakingAdminFacet.setKickPenaltyPercent(1, 5)
      ).revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner()');

      expect(
        stakingBalances.setContractResolver(routerContract.address)
      ).revertedWithCustomError(stakingBalances, 'CallerNotOwner()');

      expect(
        stakingAdminFacet.setContractResolver(routerContract.address)
      ).revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner()');

      expect(
        stakingAdminFacet.setEpochState(StakingState.NextValidatorSetLocked)
      ).revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner()');

      expect(
        stakingAdminFacet.setEpochState(StakingState.Paused)
      ).revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner()');

      expect(
        stakingAdminFacet.adminKickValidatorInNextEpoch(stakingAccount1.address)
      ).revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner()');

      expect(
        stakingAdminFacet.adminSlashValidator(stakingAccount1.address, 100)
      ).revertedWithCustomError(stakingUtilsLib, 'CallerNotOwner()');
    });
  });

  describe('the admin can pause', () => {
    it('tries to pause then unpause as admin', async () => {
      stakingAdminFacet = stakingAdminFacet.connect(deployer);

      const currentState = await stakingViewsFacet.state();
      await stakingAdminFacet.setEpochState(StakingState.Paused);
      expect(await stakingViewsFacet.state()).to.equal(StakingState.Paused);

      // move the state back
      await stakingAdminFacet.setEpochState(currentState);
      expect(await stakingViewsFacet.state()).to.equal(currentState);
    });
  });

  describe('when paused', () => {
    let stateBeforePause;

    beforeEach(async () => {
      stateBeforePause = await stakingViewsFacet.state();
      await stakingAdminFacet
        .connect(deployer)
        .setEpochState(StakingState.Paused);
    });

    afterEach(async () => {
      await stakingAdminFacet.connect(deployer).setEpochState(stateBeforePause);
    });

    describe('can call mutative functions', function () {
      it('cannot lock validators for next epoch', async () => {
        await expect(
          stakingContract
            .connect(stakingAccounts[0].stakingAddress)
            .lockValidatorsForNextEpoch()
        ).revertedWithCustomError(
          stakingContract,
          'MustBeInActiveOrUnlockedState'
        );
      });

      it('cannot signal ready for next epoch', async () => {
        await expect(
          stakingContract
            .connect(stakingAccounts[0].stakingAddress)
            .signalReadyForNextEpoch(5)
        )
          .revertedWithCustomError(
            stakingContract,
            'SignaledReadyForWrongEpochNumber'
          )
          .withArgs(2, 5);
      });

      it('cannot advance epoch', async () => {
        // Get current block time
        const blockTimestamp = (
          await deployer.provider.getBlock(
            await deployer.provider.getBlockNumber()
          )
        ).timestamp;

        // Get current epoch end time and timeout
        const epoch = await stakingViewsFacet.epoch();
        const epochEndTime = epoch.endTime;

        if (epochEndTime > BigInt(blockTimestamp)) {
          // Fast forward time to when next epoch can be reached.
          await deployer.provider.send('evm_setNextBlockTimestamp', [
            Number(epochEndTime),
          ]);
          await deployer.provider.send('evm_mine');
        }

        await expect(
          stakingContract
            .connect(stakingAccounts[0].stakingAddress)
            .advanceEpoch()
        ).revertedWithCustomError(
          stakingContract,
          'MustBeInReadyForNextEpochState'
        );
      });

      it('new stakers can stake and join', async () => {
        // Create new stakers
        const newStakingAddress = ethers.Wallet.createRandom().connect(
          deployer.provider
        );
        const newNodeAddress = ethers.Wallet.createRandom().connect(
          deployer.provider
        );

        // Send them some gas
        const ethForGas = ethers.parseEther('1.0');
        await deployer.sendTransaction({
          to: newStakingAddress.address,
          value: ethForGas,
        });
        await deployer.sendTransaction({
          to: newNodeAddress.address,
          value: ethForGas,
        });

        // Send them some tokens
        const totalToStake = minStake * 3n; // 3 times the minimum stake
        await token
          .connect(deployer)
          .transfer(newStakingAddress.address, totalToStake);
        await token
          .connect(newStakingAddress)
          .approve(await stakingBalances.getAddress(), totalToStake);

        // Prepare joining parameters
        const ipAddress = ip2int(stakingAccount1IpAddress);
        const port = 1337;
        const nodeAddress = newNodeAddress.address;
        const comsKeys = toBigInt(randomBytes(32));

        // Stake and join
        await stakingContract
          .connect(newStakingAddress)
          .stakeAndJoin(
            minStake,
            ipAddress,
            0,
            port,
            nodeAddress,
            comsKeys,
            comsKeys
          );
      });

      it('cannot exit for a reason unrelated to paused state', async () => {
        await expect(
          stakingContract.connect(stakingAccounts[0].stakingAddress).exit()
        ).revertedWithCustomError(
          stakingContract,
          'ActiveValidatorsCannotLeave'
        );
      });

      it('can request to leave', async () => {
        await stakingContract
          .connect(stakingAccounts[0].stakingAddress)
          .requestToLeave();
      });

      it('can kick validator in next epoch', async () => {
        await stakingContract
          .connect(stakingAccounts[2].nodeAddress)
          .kickValidatorInNextEpoch(
            stakingAccounts[1].stakingAddress.address,
            1,
            '0x'
          );
      });
    });
  });
});

async function updateMinimumValidatorCount(
  stakingViewsFacet,
  stakingAdminFacet,
  newMinimumValidatorCount
) {
  // Get currrent config
  const currentConfig = await stakingViewsFacet.config();

  // Update config
  await stakingAdminFacet.setConfig({
    tokenRewardPerTokenPerEpoch: currentConfig.tokenRewardPerTokenPerEpoch,
    DEPRECATED_complaintTolerance: 0,
    DEPRECATED_complaintIntervalSecs: 0,
    keyTypes: [...(await stakingViewsFacet.getKeyTypes())],
    minimumValidatorCount: newMinimumValidatorCount,
    maxConcurrentRequests: currentConfig.maxConcurrentRequests,
    maxTripleCount: currentConfig.maxTripleCount,
    minTripleCount: currentConfig.minTripleCount,
    peerCheckingIntervalSecs: currentConfig.peerCheckingIntervalSecs,
    maxTripleConcurrency: currentConfig.maxTripleConcurrency,
    rpcHealthcheckEnabled: true,
    litActionConfig: Object.values(currentConfig.litActionConfig),
    heliosEnabled: true,
  });
}

async function voteToKick(
  stakingContract,
  stakingViewsFacet,
  {
    stakingAccountToVoteFrom,
    stakingAccountToBeKicked,
    epoch,
    expectedExistingNumVotes,
  }
) {
  const nodeAddress = stakingAccountToVoteFrom.nodeAddress;
  stakingContract = stakingContract.connect(nodeAddress);
  await stakingContract.kickValidatorInNextEpoch(
    stakingAccountToBeKicked.stakingAddress.address,
    1,
    '0x'
  );

  // assert votesToKickValidatorsInNextEpoch state
  const [numVotes, didStakerVote] =
    await stakingViewsFacet.getVotingStatusToKickValidator(
      epoch.number,
      stakingAccountToBeKicked.stakingAddress.address,
      stakingAccountToVoteFrom.stakingAddress.address
    );
  expect(numVotes).equal(expectedExistingNumVotes + 1);
  expect(didStakerVote).is.true;
}

async function makeAssertions(
  stakingViewsFacet,
  {
    expectedValidatorsInCurrentEpoch = undefined,
    expectedValidatorsInNextEpoch = undefined,
    expectedValidatorsInCurrentEpochExcludes = [], // array of addresses
    expectedValidatorsInNextEpochExcludes = [], // array of addresses
    expectedKickedValidatorsInNextEpoch = undefined,
  }
) {
  if (!!expectedValidatorsInCurrentEpoch) {
    expect(
      (await stakingViewsFacet.getValidatorsInCurrentEpoch()).length
    ).equal(expectedValidatorsInCurrentEpoch);
  }

  if (!!expectedValidatorsInNextEpoch) {
    expect((await stakingViewsFacet.getValidatorsInNextEpoch()).length).equal(
      expectedValidatorsInNextEpoch
    );
  }

  if (expectedValidatorsInCurrentEpochExcludes.length > 0) {
    for (const excludedValidatorAddress of expectedValidatorsInCurrentEpochExcludes) {
      expect(
        (await stakingViewsFacet.getValidatorsInCurrentEpoch()).includes(
          excludedValidatorAddress
        )
      ).is.false;
    }
  }

  if (expectedValidatorsInNextEpochExcludes.length > 0) {
    for (const excludedValidatorAddress of expectedValidatorsInNextEpochExcludes) {
      expect(
        (await stakingViewsFacet.getValidatorsInNextEpoch()).includes(
          excludedValidatorAddress
        )
      ).is.false;
    }
  }

  if (!!expectedKickedValidatorsInNextEpoch) {
    expect((await stakingViewsFacet.getKickedValidators()).length).equal(
      expectedKickedValidatorsInNextEpoch
    );
  }
}

async function lockSignalAndAdvanceEpoch(
  stakingContract,
  stakingViewsFacet,
  { stakingAccounts }
) {
  // lock the validator set
  await stakingContract.lockValidatorsForNextEpoch();
  await signalAndAdvanceEpoch(stakingContract, stakingViewsFacet, {
    stakingAccounts,
  });
}

async function signalAndAdvanceEpoch(
  stakingContract,
  stakingViewsFacet,
  { stakingAccounts }
) {
  // check that the validator set is already locked
  expect(await stakingViewsFacet.state()).to.equal(
    StakingState.NextValidatorSetLocked
  );

  // signal that we are ready to advance epoch
  const existingEpochNumber = (await stakingViewsFacet.epoch()).number;

  for (let i = 0; i < stakingAccounts.length; i++) {
    const stakingAccount = stakingAccounts[i];
    const nodeAddress = stakingAccount.nodeAddress;
    stakingContract = stakingContract.connect(nodeAddress);
    await stakingContract
      .connect(nodeAddress)
      .signalReadyForNextEpoch(existingEpochNumber);
  }
  expect(await stakingViewsFacet.state()).to.equal(
    StakingState.ReadyForNextEpoch
  );

  // advance the epoch.  this sets the validators to be the new set
  await stakingContract.advanceEpoch();
  expect(await stakingViewsFacet.state()).to.equal(StakingState.Active);
}
