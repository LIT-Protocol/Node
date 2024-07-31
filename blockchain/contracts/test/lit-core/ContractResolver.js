const { expect } = require('chai');

/// @dev The identifier of the role which maintains other roles.
const ADMIN_ROLE = ethers.keccak256(ethers.toUtf8Bytes('ADMIN'));

// Enum Constants
const ENV_DEV = 0;
const ENV_STAGING = 1;
const ENV_PROD = 2;

describe('ContractResolver', function () {
  let deployer;
  let signers;
  let resolverContract;

  before(async () => {
    [deployer, ...signers] = await ethers.getSigners();
  });

  describe('set contract', async () => {
    context('with a valid environment', async () => {
      beforeEach(async () => {
        resolverContract = await ethers.deployContract('ContractResolver', [
          ENV_DEV,
        ]);
      });

      it('creates an address mapping', async () => {
        await resolverContract.setContract(
          await resolverContract.RELEASE_REGISTER_CONTRACT(),
          ENV_DEV,
          '0x6392A007A19801e15700B6d5Ce3CED63CCebd795'
        );

        let exp_addr = await resolverContract.getContract(
          await resolverContract.RELEASE_REGISTER_CONTRACT(),
          ENV_DEV
        );

        expect(exp_addr).equals('0x6392A007A19801e15700B6d5Ce3CED63CCebd795');
      });
    });

    context('without role', async () => {
      beforeEach(async () => {
        resolverContract = await ethers.deployContract('ContractResolver', [
          ENV_DEV,
        ]);

        await resolverContract.renounceRole(ADMIN_ROLE, deployer.address);
      });

      it('produces an error', async () => {
        const test_typ = ethers.keccak256(ethers.toUtf8Bytes('test'));

        await expect(
          resolverContract.setContract(
            test_typ,
            ENV_DEV,
            '0x6392A007A19801e15700B6d5Ce3CED63CCebd795'
          )
        ).to.be.revertedWithCustomError(resolverContract, 'AdminRoleRequired');
      });
    });
  });

  describe('get contract', async () => {
    context('with an existing contract', async () => {
      beforeEach(async () => {
        resolverContract = await ethers.deployContract('ContractResolver', [
          ENV_DEV,
        ]);

        const test_typ = ethers.keccak256(ethers.toUtf8Bytes('test'));

        await resolverContract.setContract(
          test_typ,
          ENV_DEV,
          '0x6392A007A19801e15700B6d5Ce3CED63CCebd795'
        );
      });

      it('returns a contract address', async () => {
        const test_typ = ethers.keccak256(ethers.toUtf8Bytes('test'));

        let exp_addr = await resolverContract.getContract(test_typ, ENV_DEV);

        expect(exp_addr).equals(
          BigInt('0x6392A007A19801e15700B6d5Ce3CED63CCebd795')
        );
      });
    });

    context('when no contract exists', async () => {
      beforeEach(async () => {
        resolverContract = await ethers.deployContract('ContractResolver', [
          ENV_DEV,
        ]);
      });

      it('returns an empty address', async () => {
        const test_typ = ethers.keccak256(ethers.toUtf8Bytes('test'));

        let exp_addr = await resolverContract.getContract(test_typ, ENV_DEV);

        expect(exp_addr).equals('0x0000000000000000000000000000000000000000');
      });
    });
  });

  describe('add allowed env', async () => {
    context('with valid env', async () => {
      beforeEach(async () => {
        resolverContract = await ethers.deployContract('ContractResolver', [
          ENV_DEV,
        ]);
      });

      it('will allow release for env', async () => {
        const test_typ = ethers.keccak256(ethers.toUtf8Bytes('test'));

        await resolverContract.setContract(
          test_typ,
          ENV_DEV,
          '0x6392A007A19801e15700B6d5Ce3CED63CCebd795'
        );
      });
    });
  });

  describe('remove allowed env', async () => {
    context('with valid env', async () => {
      beforeEach(async () => {
        resolverContract = await ethers.deployContract('ContractResolver', [
          ENV_DEV,
        ]);
      });

      it('will prevent release for env', async () => {
        const test_typ = ethers.keccak256(ethers.toUtf8Bytes('test'));

        await resolverContract.removeAllowedEnv(ENV_DEV);

        await expect(
          resolverContract.setContract(
            test_typ,
            ENV_DEV,
            '0x6392A007A19801e15700B6d5Ce3CED63CCebd795'
          )
        ).to.be.revertedWith('The provided Env is not valid for this contract');
      });
    });
  });
});
