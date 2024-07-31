const { expect } = require('chai');
const { deployDiamond } = require('../../scripts/deployDiamond');

describe('PaymentDelegation', function () {
  let signers;
  let aPayer;
  let user;
  let anotherUser;
  let paymentDelegationFacet;

  before(async () => {
    const { diamond: paymentDelegationDiamond } = await deployDiamond(
      'PaymentDelegation',
      null,
      null,
      {
        additionalFacets: ['PaymentDelegationFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    paymentDelegationFacet = await ethers.getContractAt(
      'PaymentDelegationFacet',
      await paymentDelegationDiamond.getAddress()
    );

    signers = await ethers.getSigners();

    aPayer = signers[1];
    user = signers[2];
    anotherUser = signers[3];
  });

  it('allows delegating and undelegating payments', async () => {
    // Delegate payment from aPayer to user
    await paymentDelegationFacet.connect(aPayer).delegatePayments(user.address);
    let payersAndRestrictions =
      await paymentDelegationFacet.getPayersAndRestrictions([user.address]);
    expect(payersAndRestrictions.length).to.equal(2);

    let payers = payersAndRestrictions[0];
    expect(payers.length).to.equal(1);
    let restrictions = payersAndRestrictions[1];
    expect(restrictions.length).to.equal(1);
    expect(restrictions[0].length).to.equal(1);
    let [requestsPerPeriod, periodSeconds] = restrictions[0][0];
    expect(requestsPerPeriod).to.equal(0);
    expect(periodSeconds).to.equal(0);
    expect(payers[0].length).to.equal(1);
    expect(payers[0][0]).to.equal(aPayer.address);

    let justPayers = await paymentDelegationFacet.getPayers(user.address);
    expect(justPayers.length).to.equal(1);
    expect(justPayers[0]).to.equal(aPayer.address);

    // check the reverse index
    let users = await paymentDelegationFacet.getUsers(aPayer.address);
    expect(users.length).to.equal(1);
    expect(users[0]).to.equal(user.address);

    // Then, undelegate payment
    await paymentDelegationFacet
      .connect(aPayer)
      .undelegatePayments(user.address);
    payersAndRestrictions =
      await paymentDelegationFacet.getPayersAndRestrictions([user.address]);

    payers = payersAndRestrictions[0];
    expect(payers.length).to.equal(1);
    restrictions = payersAndRestrictions[1];
    expect(restrictions.length).to.equal(1);
    expect(restrictions[0].length).to.equal(0);
    expect(payers[0].length).to.equal(0);

    justPayers = await paymentDelegationFacet.getPayers(user.address);
    expect(justPayers.length).to.equal(0);

    // check the reverse index
    users = await paymentDelegationFacet.getUsers(aPayer.address);
    expect(users.length).to.equal(0);
  });

  it('allows setting restrictions', async () => {
    const restriction = {
      requestsPerPeriod: 100,
      periodSeconds: 3600,
    };
    await paymentDelegationFacet.connect(aPayer).setRestriction(restriction);
    const deployerRestriction = await paymentDelegationFacet.getRestriction(
      aPayer.address
    );
    expect(deployerRestriction.requestsPerPeriod).to.equal(
      restriction.requestsPerPeriod
    );
    expect(deployerRestriction.periodSeconds).to.equal(
      restriction.periodSeconds
    );
  });

  it('allows batch delegating and undelegating payments', async () => {
    // Batch delegate payments to user and anotherUser
    await paymentDelegationFacet
      .connect(aPayer)
      .delegatePaymentsBatch([user.address, anotherUser.address]);
    let payersAndRestrictions =
      await paymentDelegationFacet.getPayersAndRestrictions([
        user.address,
        anotherUser.address,
      ]);
    let payers = payersAndRestrictions[0];
    let restrictions = payersAndRestrictions[1];
    expect(payers.length).to.equal(2);
    expect(restrictions.length).to.equal(2);

    let userPayers = payers[0];
    let anotherUserPayers = payers[1];
    expect(userPayers.length).to.equal(1);
    expect(userPayers[0]).to.equal(aPayer.address);
    expect(anotherUserPayers.length).to.equal(1);
    expect(anotherUserPayers[0]).to.equal(aPayer.address);

    let userRestrictions = restrictions[0];
    let anotherUserRestrictions = restrictions[1];

    expect(userRestrictions.length).to.equal(1);
    let [requestsPerPeriod, periodSeconds] = userRestrictions[0];
    expect(requestsPerPeriod).to.equal(100);
    expect(periodSeconds).to.equal(3600);
    expect(anotherUserRestrictions.length).to.equal(1);
    [requestsPerPeriod, periodSeconds] = anotherUserRestrictions[0];
    expect(requestsPerPeriod).to.equal(100);
    expect(periodSeconds).to.equal(3600);

    // check the reverse index
    let users = await paymentDelegationFacet.getUsers(aPayer.address);
    expect(users.length).to.equal(2);
    expect(users[0]).to.equal(user.address);
    expect(users[1]).to.equal(anotherUser.address);

    // Then, batch undelegate payments
    await paymentDelegationFacet
      .connect(aPayer)
      .undelegatePaymentsBatch([user.address, anotherUser.address]);
    payersAndRestrictions =
      await paymentDelegationFacet.getPayersAndRestrictions([
        user.address,
        anotherUser.address,
      ]);
    payers = payersAndRestrictions[0];
    restrictions = payersAndRestrictions[1];
    expect(payers.length).to.equal(2);
    expect(restrictions.length).to.equal(2);

    userPayers = payers[0];
    anotherUserPayers = payers[1];
    expect(userPayers.length).to.equal(0);
    expect(anotherUserPayers.length).to.equal(0);

    userRestrictions = restrictions[0];
    anotherUserRestrictions = restrictions[1];
    expect(userRestrictions.length).to.equal(0);
    expect(anotherUserRestrictions.length).to.equal(0);
  });
});
