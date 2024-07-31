const { expect } = require('chai');

describe('LITToken', function () {
  let deployer;
  let signers;
  let token;

  beforeEach(async () => {
    [deployer, ...signers] = await ethers.getSigners();
  });

  beforeEach(async () => {
    token = await ethers.deployContract('LITToken', [
      ethers.parseUnits('1000000000', 18), // 1 billion
    ]);
  });

  it('grants the admin role to the deployer', async () => {
    expect(
      await token.hasRole(await token.ADMIN_ROLE(), await deployer.getAddress())
    ).is.true;
  });

  it('grants the minter role to the deployer', async () => {
    expect(
      await token.hasRole(
        await token.MINTER_ROLE(),
        await deployer.getAddress()
      )
    ).is.true;
  });

  describe('mint', async () => {
    context('when unauthorized', async () => {
      let unauthorizedMinter;
      let recipient;

      beforeEach(
        async () => ([unauthorizedMinter, recipient, ...signers] = signers)
      );

      beforeEach(async () => (token = token.connect(unauthorizedMinter)));

      it('reverts', async () => {
        expect(token.mint(await recipient.getAddress(), 1)).revertedWith(
          'LITToken: only minter'
        );
      });
    });

    context('when authorized', async () => {
      let minter;
      let recipient;
      const amount = 1000;

      beforeEach(async () => ([minter, recipient, ...signers] = signers));

      beforeEach(async () => {
        await token.grantRole(
          await token.MINTER_ROLE(),
          await minter.getAddress()
        );

        return await token.grantRole(
          await token.PAUSER_ROLE(),
          await minter.getAddress()
        );
      });

      beforeEach(async () => (token = token.connect(minter)));

      it('mints and burns tokens', async () => {
        await token.mint(await recipient.getAddress(), amount);
        expect(await token.balanceOf(await recipient.getAddress())).equal(
          amount
        );

        token = token.connect(recipient);

        await token.burn(amount);
        expect(await token.balanceOf(await recipient.getAddress())).equal(0);
      });

      it('wont mint past the cap', async () => {
        // 10 billion but the cap is 1b
        expect(
          token.mint(
            await recipient.getAddress(),
            ethers.parseUnits('10000000000', 18)
          )
        ).revertedWith('ERC20Capped: cap exceeded');
      });

      it('wont transfer when paused', async () => {
        await token.mint(await recipient.getAddress(), amount);
        expect(await token.balanceOf(await recipient.getAddress())).equal(
          amount
        );

        await token.pause();

        token = token.connect(recipient);

        expect(token.transfer(await minter.getAddress(), amount)).revertedWith(
          'ERC20Pausable: token transfer while paused'
        );

        // confirm the balance is still the same
        expect(await token.balanceOf(await recipient.getAddress())).equal(
          amount
        );

        token = token.connect(minter);
        await token.unpause();

        token = token.connect(recipient);
        await token.transfer(await minter.getAddress(), amount);

        expect(await token.balanceOf(await recipient.getAddress())).equal(0);
      });
    });
  });
});
