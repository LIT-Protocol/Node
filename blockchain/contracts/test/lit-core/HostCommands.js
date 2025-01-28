const { expect } = require('chai');
const { ethers } = require('hardhat');
const { getBytesFromMultihash, getParamsFromPKPMint } = require('../../utils');
const { deployDiamond } = require('../../scripts/deployDiamond');
const { Environment, setContractResolver } = require('../../utils/contract');

describe('HostCommands', function () {
  let deployer;
  let authorizedCommandSender;
  let signers;
  let contractResolver;
  let hostCommandsContract;
  let expirationTime;

  before(async () => {
    [deployer, authorizedCommandSender, ...signers] = await ethers.getSigners();

    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);

    const { diamond: hostCommandsDiamond } = await deployDiamond(
      'HostCommands',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['HostCommandsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    hostCommandsContract = await ethers.getContractAt(
      'HostCommandsFacet',
      await hostCommandsDiamond.getAddress()
    );

    await hostCommandsContract.setAuthorizedCommandSender(
      authorizedCommandSender.address
    );

    await setContractResolver(contractResolver, Environment.DEV, {
      hostCommandsContract,
    });

    // 6 hours in the future
    expirationTime = Math.floor(Date.now() / 1000) + 6 * 60 * 60;
  });

  describe('Attempt to send some commands', async () => {
    it('attempts to restart a node as an unauthorized caller', async () => {
      await expect(
        hostCommandsContract
          .connect(signers[0])
          .restart(
            '0x0000000000000000000000000000000000000000',
            expirationTime,
            true
          )
      ).to.be.reverted;
    });

    it('restarts a node as an authorized caller', async () => {
      const tx = await hostCommandsContract
        .connect(authorizedCommandSender)
        .restart(
          '0x0000000000000000000000000000000000000000',
          expirationTime,
          true
        );
      const receipt = await tx.wait();
      expect(receipt)
        .to.emit(hostCommandsContract, 'Restart')
        .withArgs(
          '0x0000000000000000000000000000000000000000',
          expirationTime,
          true
        );
    });

    it('attempts to upgrade a node as an unauthorized caller', async () => {
      await expect(
        hostCommandsContract
          .connect(signers[0])
          .upgrade(
            '0x0000000000000000000000000000000000000000',
            expirationTime,
            'main',
            '1',
            true
          )
      ).to.be.reverted;
    });

    it('upgrades a node as an authorized caller', async () => {
      const tx = await hostCommandsContract
        .connect(authorizedCommandSender)
        .upgrade(
          '0x0000000000000000000000000000000000000000',
          expirationTime,
          'main',
          '1',
          true
        );
      const receipt = await tx.wait();
      expect(receipt)
        .to.emit(hostCommandsContract, 'Upgrade')
        .withArgs(
          '0x0000000000000000000000000000000000000000',
          expirationTime,
          'main',
          '1',
          true
        );
    });
  });
  describe('Attempt to set the authorized command sender', async () => {
    it('attempts to set the authorized command sender as an unauthorized caller', async () => {
      await expect(
        hostCommandsContract
          .connect(signers[0])
          .setAuthorizedCommandSender(authorizedCommandSender.address)
      ).to.be.reverted;
    });

    it('sets the authorized command sender as an authorized caller', async () => {
      const newAuthorizedCommandSender = signers[1];
      const tx = await hostCommandsContract
        .connect(deployer)
        .setAuthorizedCommandSender(newAuthorizedCommandSender.address);
      const receipt = await tx.wait();
      expect(receipt)
        .to.emit(hostCommandsContract, 'AuthorizedCommandSenderUpdated')
        .withArgs(newAuthorizedCommandSender.address);

      // try using it
      await expect(
        hostCommandsContract
          .connect(newAuthorizedCommandSender)
          .restart(
            '0x0000000000000000000000000000000000000000',
            expirationTime,
            true
          )
      ).to.not.be.reverted;

      // set it back
      await hostCommandsContract
        .connect(deployer)
        .setAuthorizedCommandSender(authorizedCommandSender.address);
    });
  });
});
