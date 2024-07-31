const { expect } = require('chai');

describe('Allowlist', function () {
  let signers;
  let contract;

  beforeEach(async () => {
    signers = await ethers.getSigners();
    contract = await ethers.deployContract('Allowlist');
  });

  describe('Test the Allowlist', async () => {
    context('unallowed by default', async () => {
      let deployer;
      let tester;

      beforeEach(async () => ([deployer, tester, ...signers] = signers));

      beforeEach(async () => (contract = contract.connect(tester)));

      it('is unallowed', async () => {
        const allowed = await contract.isAllowed(ethers.keccak256('0x123456'));
        expect(allowed).equal(false);
      });
    });

    context('when the owner sets things', async () => {
      let deployer;
      let tester;

      const key = '0x123456789f';

      beforeEach(async () => {
        [deployer, tester, ...signers] = signers;
      });

      beforeEach(async () => (contract = contract.connect(tester)));

      it('can allow and unallow things', async () => {
        // unallowed by default
        let allowed = await contract.isAllowed(ethers.keccak256(key));
        expect(allowed).equal(false);

        // attempt to allow it with the wrong address.  it should revert.
        expect(contract.setAllowed(ethers.keccak256(key))).revertedWith(
          'Not an admin'
        );

        // attempt to unallow it with the wrong address.  it should revert.
        expect(contract.setNotAllowed(ethers.keccak256(key))).revertedWith(
          'Not an admin'
        );

        // attempt to add an admin with the wrong address.  it should revert.
        expect(contract.addAdmin(tester.address)).revertedWith(
          'Ownable: caller is not the owner'
        );

        // connect an admin and try allowing
        contract = contract.connect(deployer);
        await contract.setAllowed(ethers.keccak256(key));

        allowed = await contract.isAllowed(ethers.keccak256(key));
        expect(allowed).equal(true);

        // allow the tester to be an admin
        await contract.addAdmin(tester.address);

        contract = contract.connect(tester);
        // attempt to unallow it as a new admin
        await contract.setNotAllowed(ethers.keccak256(key));

        // show that it's not allowed anymore
        allowed = await contract.isAllowed(ethers.keccak256(key));
        expect(allowed).equal(false);
      });
    });
  });
});
