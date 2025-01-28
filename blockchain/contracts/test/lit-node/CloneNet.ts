import { expect } from 'chai';
import { Contract } from 'ethers';
import hre from 'hardhat';
import { deployDiamond } from '../../scripts/deployDiamond';
import {
  CloneNetFacet,
  LITToken,
  StakingBalancesFacet,
} from '../../typechain-types';
import {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  StakingAccount,
} from '../../utils/contract';
const { ethers } = hre;

describe('CloneNet', function () {
  let contractResolver1;
  let contractResolver2;
  let stakingDiamond1: Contract;
  let stakingDiamond2: Contract;
  let stakingBalances1: StakingBalancesFacet;
  let cloneNetContract: CloneNetFacet;
  let stakingAccounts1: StakingAccount[];
  let token: LITToken;
  const stakingAccountCount = 10;
  const stakingAccount1Port = 7777;
  const stakingAccount1IpAddress = '192.168.1.1';
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals

  beforeEach(async () => {
    const [deployer, ...signers] = await ethers.getSigners();
    token = await ethers.deployContract(
      'LITToken',
      [ethers.parseUnits('1000000000', 18)] // 1b tokens
    );
    token = token.connect(deployer);

    contractResolver1 = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);
    contractResolver2 = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);
    const deployStakingRes1 = await deployDiamond(
      'Staking',
      await contractResolver1.getAddress(),
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
    stakingDiamond1 = deployStakingRes1.diamond;

    const { diamond: stakingBalancesDiamond } = await deployDiamond(
      'StakingBalances',
      await contractResolver1.getAddress(),
      0,
      {
        additionalFacets: ['StakingBalancesFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    stakingBalances1 = await ethers.getContractAt(
      'StakingBalancesFacet',
      await stakingBalancesDiamond.getAddress()
    );

    const deployStakingRes2 = await deployDiamond(
      'Staking',
      await contractResolver2.getAddress(),
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
    stakingDiamond2 = deployStakingRes2.diamond;
    const stakingFacet1 = await ethers.getContractAt(
      'StakingFacet',
      await stakingDiamond1.getAddress()
    );
    const stakingAdminFacet1 = await ethers.getContractAt(
      'StakingAdminFacet',
      await stakingDiamond1.getAddress()
    );

    const deployCloneNetRes = await deployDiamond('CloneNet', null, null, {
      additionalFacets: ['CloneNetFacet'],
      verifyContracts: false,
      waitForDeployment: false,
    });
    cloneNetContract = await ethers.getContractAt(
      'CloneNetFacet',
      await deployCloneNetRes.diamond.getAddress()
    );

    await setContractResolver(contractResolver1, Environment.DEV, {
      tokenContract: token,
      stakingContract: stakingFacet1,
      stakingBalancesContract: stakingBalances1,
    });
    await token.mint(deployer.address, totalTokens);

    // Only set up validators for stakingDiamond1
    stakingAccounts1 = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingFacet1,
      stakingAdminFacet1,
      stakingBalances1,
      token,
      deployer,
      {
        numValidators: stakingAccountCount,
        startingPort: stakingAccount1Port,
        ipAddress: stakingAccount1IpAddress,
      }
    );
  });

  describe('active staking contracts', async () => {
    it('should be able to be updated', async () => {
      // First assert that the number of active staking contracts is 0
      const numActiveStakingContracts =
        await cloneNetContract.numActiveStakingContracts();
      expect(numActiveStakingContracts).to.equal(0);

      // Add the staking contract to the clone net
      const addStakingContractTx =
        await cloneNetContract.adminAddActiveStakingContract(
          await stakingDiamond1.getAddress()
        );
      await addStakingContractTx.wait();

      // Assert that the number of active staking contracts is 1
      const numActiveStakingContractsAfter =
        await cloneNetContract.numActiveStakingContracts();
      expect(numActiveStakingContractsAfter).to.equal(1);

      // Add the second staking contract to the clone net
      const addStakingContractTx2 =
        await cloneNetContract.adminAddActiveStakingContract(
          await stakingDiamond2.getAddress()
        );
      await addStakingContractTx2.wait();

      // Assert that the number of active staking contracts is 2
      const numActiveStakingContractsAfter2 =
        await cloneNetContract.numActiveStakingContracts();
      expect(numActiveStakingContractsAfter2).to.equal(2);

      // Assert that the active staking contracts are correct
      const activeStakingContracts =
        await cloneNetContract.getActiveStakingContracts();
      expect(activeStakingContracts).to.deep.equal([
        await stakingDiamond1.getAddress(),
        await stakingDiamond2.getAddress(),
      ]);

      // Remove the first staking contract from the clone net
      const removeStakingContractTx =
        await cloneNetContract.adminRemoveActiveStakingContract(
          await stakingDiamond1.getAddress()
        );
      await removeStakingContractTx.wait();

      // Assert that the number of active staking contracts is 1
      const numActiveStakingContractsAfterRemove =
        await cloneNetContract.numActiveStakingContracts();
      expect(numActiveStakingContractsAfterRemove).to.equal(1);
    });
  });

  describe('querying', async () => {
    it('should be able to query all active staking contracts', async () => {
      // Add both staking contracts to the clone net
      await cloneNetContract.adminAddActiveStakingContract(
        await stakingDiamond1.getAddress()
      );
      await cloneNetContract.adminAddActiveStakingContract(
        await stakingDiamond2.getAddress()
      );

      const stakingAggregateDetails =
        await cloneNetContract.getAllActiveUnkickedValidatorStructsAndCounts();

      // Assertions on the first returned object
      const firstStakingContractAggregateDetails = stakingAggregateDetails[0];
      expect(
        firstStakingContractAggregateDetails.stakingContractAddress
      ).to.equal(await stakingDiamond1.getAddress());
      expect(
        firstStakingContractAggregateDetails.details.epoch.number
      ).to.equal(2);
      expect(
        firstStakingContractAggregateDetails.details
          .currentValidatorCountForConsensus
      ).to.equal(Math.floor((stakingAccountCount * 2) / 3));
      for (let i = 0; i < stakingAccountCount; i++) {
        expect(
          firstStakingContractAggregateDetails.details.activeUnkickedValidators[
            i
          ].nodeAddress
        ).to.equal(stakingAccounts1[i].nodeAddress.address);
      }

      // Assertions on the second returned object
      const secondStakingContractAggregateDetails = stakingAggregateDetails[1];
      expect(
        secondStakingContractAggregateDetails.stakingContractAddress
      ).to.equal(await stakingDiamond2.getAddress());
      expect(
        secondStakingContractAggregateDetails.details.epoch.number
      ).to.equal(1);
      expect(
        secondStakingContractAggregateDetails.details
          .currentValidatorCountForConsensus
      ).to.equal(0);
      expect(
        secondStakingContractAggregateDetails.details.activeUnkickedValidators
      ).to.deep.equal([]);
    });
  });
});
