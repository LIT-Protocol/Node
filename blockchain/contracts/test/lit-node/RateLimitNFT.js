const { expect } = require('chai');
const { ethers } = require('hardhat');
const { deployDiamond } = require('../../scripts/deployDiamond');
const { Environment } = require('../../utils/contract');

describe('RateLimitNFT', function () {
  let deployer;
  let signers;
  let rateLimitNFTContract;
  let rateLimitNFTViewsContract;
  let contractResolver;

  beforeEach(async () => {
    [deployer, ...signers] = await ethers.getSigners();
    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);
    const { diamond: rateLimitNFTDiamond } = await deployDiamond(
      'RateLimitNFT',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['RateLimitNFTFacet', 'RateLimitNFTViewsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    rateLimitNFTContract = await ethers.getContractAt(
      'RateLimitNFTFacet',
      await rateLimitNFTDiamond.getAddress()
    );
    rateLimitNFTViewsContract = await ethers.getContractAt(
      'RateLimitNFTViewsFacet',
      await rateLimitNFTDiamond.getAddress()
    );
  });

  describe('Test free minting of Rate Limit NFT', async () => {
    let minter;
    let admin;

    beforeEach(async () => ([minter, admin, ...signers] = signers));

    it('checks the signature for a free mint', async () => {
      let now = Date.now();
      // tomorrow
      let expiresAt = BigInt(now + 1000 * 60 * 60 * 24);
      let requestsPerKilosecond = BigInt(1000);

      rateLimitNFTContract = rateLimitNFTContract.connect(deployer);
      await rateLimitNFTContract.setFreeMintSigner(admin.address);

      // test with empty sig
      expect(
        rateLimitNFTContract.freeMint(
          expiresAt,
          requestsPerKilosecond,
          '0x0000000000000000000000000000000000000000000000000000000000000000',
          0,
          '0x0000000000000000000000000000000000000000000000000000000000000000',
          '0x0000000000000000000000000000000000000000000000000000000000000000'
        )
      ).revertedWith(
        'The msgHash is not a hash of the expiresAt + requestsPerKilosecond.  Explain yourself!'
      );
    });
  });

  describe('Test minting costs and params of Rate Limit NFT', async () => {
    let minter;
    let admin;

    beforeEach(async () => ([minter, admin, ...signers] = signers));

    it('mints a rate limit increase nft and checks the params', async () => {
      // we would like 10 requests per kilosecond, which is 10 request per 1000 seconds.
      const requestsPerKilosecond = BigInt(10);
      // this should be set to 1,000,000 wei
      const additionalRequestsPerKilosecondCost =
        await rateLimitNFTViewsContract.additionalRequestsPerKilosecondCost();
      expect(additionalRequestsPerKilosecondCost).to.equal(1000000);

      // get block timestamp
      let block = await ethers.provider.getBlock('latest');
      let timestamp = block.timestamp;
      // this calculates the next midnight time
      const expiresAt = Math.ceil(timestamp / 86400 + 1) * 86400;
      // calculate the cost manually
      // const manualCost =
      //     (additionalRequestsPerKilosecondCost *
      //         requestsPerKilosecond *
      //         expirationTimeInSecondsFromNow) /
      //     1000n;

      const cost = await rateLimitNFTViewsContract.calculateCost(
        requestsPerKilosecond,
        expiresAt
      );

      const secondsBought = expiresAt - timestamp;

      // console.log("cost: ", cost.toString());
      // each additional kilosecond costs 1,000,000 wei, aka 1000 for each additional second
      // we're asking for 10 request per kilosecond, aka 0.01 requests per second
      // for secondsBought seconds, so we need to pay secondsBought seconds * 1000 * 10 for each second
      expect(cost).to.equal(
        BigInt(secondsBought) * 1000n * requestsPerKilosecond
      );

      // let's sanity check the opposite calculation
      const requestsPerKilosecondFromContract =
        await rateLimitNFTViewsContract.calculateRequestsPerKilosecond(
          cost,
          expiresAt
        );

      expect(requestsPerKilosecondFromContract).to.equal(requestsPerKilosecond);

      block = await ethers.provider.getBlock('latest');
      timestamp = BigInt(block.timestamp);
      // send eth with the txn
      let res = await rateLimitNFTContract.mint(expiresAt, {
        value: cost,
      });
      let receipt = await res.wait();

      // get the tokenId from the event
      let tokenId = receipt.logs[0].topics[3];
      // console.log("tokenId", tokenId.toString());

      // check the params
      let capacity = await rateLimitNFTViewsContract.capacity(tokenId);
      expect(capacity[0]).to.equal(requestsPerKilosecond);
      expect(capacity[1]).to.equal(expiresAt);
    });

    it('tries to mint with some bad params', async () => {
      // we would like 10 request per kilosecond, which is 10 request per 1000 seconds.
      const requestsPerKilosecond = BigInt(10);
      // this should be set to 1,000,000 wei
      const additionalRequestsPerKilosecondCost =
        await rateLimitNFTViewsContract.additionalRequestsPerKilosecondCost();
      expect(additionalRequestsPerKilosecondCost).to.equal(1000000);

      // get block timestamp
      let block = await ethers.provider.getBlock('latest');
      let timestamp = block.timestamp;
      // this calculates the next midnight time
      const expiresAt = Math.ceil(timestamp / 86400 + 1) * 86400;
      const cost = await rateLimitNFTViewsContract.calculateCost(
        requestsPerKilosecond,
        expiresAt
      );

      const belowCost = cost - 20000000n;

      // now let's try to mint with too little.  minting should work
      // but the rate limit increase nft should have a smaller rate limit increase
      let res = await rateLimitNFTContract.mint(expiresAt, {
        value: belowCost,
      });
      let receipt = await res.wait();
      // get the tokenId from the event
      let tokenId = receipt.logs[0].topics[3];

      // check the params
      let capacity = await rateLimitNFTViewsContract.capacity(tokenId);
      expect(capacity[0]).to.be.below(requestsPerKilosecond);
      expect(capacity[1]).to.equal(expiresAt);

      // try to trick it with tiny numbers
      expect(
        rateLimitNFTContract.mint(timestamp + 10, {
          value: BigInt(1),
        })
      ).revertedWith('The requestsPerKilosecond must be greater than 0');
    });

    it('tries to mint past the expiration', async () => {
      // we would like 10 request per kilosecond, which is 10 request per 1000 seconds.
      const requestsPerKilosecond = BigInt(10);
      // this should be set to 1,000,000 wei
      const additionalRequestsPerKilosecondCost =
        await rateLimitNFTViewsContract.additionalRequestsPerKilosecondCost();
      expect(additionalRequestsPerKilosecondCost).to.equal(1000000);

      // get max expiration
      const maxExpiration =
        await rateLimitNFTViewsContract.maxExpirationSeconds();

      // 100 seconds from now
      // get block timestamp
      let block = await ethers.provider.getBlock('latest');
      let timestamp = BigInt(block.timestamp);
      const expirationTimeInSecondsFromNow = maxExpiration + 1000n; // add 1000 seconds to max expiration
      let expiresAt = timestamp + BigInt(expirationTimeInSecondsFromNow);
      expect(
        rateLimitNFTViewsContract.calculateCost(
          requestsPerKilosecond,
          expiresAt
        )
      ).revertedWith(
        'You cannot purchase an expiration time that is more than the maxExpirationSeconds in the future'
      );

      // test the calculateRequestsPerKilosecond function too
      expect(
        rateLimitNFTViewsContract.calculateRequestsPerKilosecond(
          ethers.parseEther('1'),
          expiresAt
        )
      ).revertedWith(
        'You cannot purchase an expiration time that is more than the maxExpirationSeconds in the future'
      );

      // calculate cost manually and try to mint
      const durationInSeconds = expiresAt - timestamp;
      const cost =
        (additionalRequestsPerKilosecondCost *
          requestsPerKilosecond *
          durationInSeconds) /
        1000n;
      expect(
        rateLimitNFTContract.mint(expiresAt, {
          value: cost,
        })
      ).revertedWith(
        'You cannot purchase an expiration time that is more than the maxExpirationSeconds in the future'
      );
    });

    it("tries to mint with an expiration that isn't midnight", async () => {
      // we would like 10 request per kilosecond, which is 10 request per 1000 seconds.
      const requestsPerKilosecond = BigInt(10);
      // this should be set to 1,000,000 wei
      const additionalRequestsPerKilosecondCost =
        await rateLimitNFTViewsContract.additionalRequestsPerKilosecondCost();
      expect(additionalRequestsPerKilosecondCost).to.equal(1000000);

      // 100 seconds from now
      // get block timestamp
      let block = await ethers.provider.getBlock('latest');
      let timestamp = block.timestamp;
      // this calculates the next midnight time
      let expiresAt = timestamp + 1000;
      expect(
        rateLimitNFTViewsContract.calculateCost(
          requestsPerKilosecond,
          expiresAt
        )
      ).revertedWith(
        'Expiration time must be set to midnight on any given day'
      );

      // try to mint with a non-midnight epiration
      expect(
        rateLimitNFTContract.mint(expiresAt, {
          value: 20000000n,
        })
      ).revertedWith(
        'Expiration time must be set to midnight on any given day'
      );
    });
  });

  describe('Test minting past max capacity', async () => {
    it('tries to mint past the max capacity in a single txn', async () => {
      const maxRequestsPerKilosecond =
        await rateLimitNFTViewsContract.maxRequestsPerKilosecond();
      const requestsPerKilosecond = maxRequestsPerKilosecond + 1n;
      // this should be set to 1,000,000 wei
      const additionalRequestsPerKilosecondCost =
        await rateLimitNFTViewsContract.additionalRequestsPerKilosecondCost();
      expect(additionalRequestsPerKilosecondCost).to.equal(1000000);

      // get block timestamp
      let block = await ethers.provider.getBlock('latest');
      let timestamp = block.timestamp;
      // this calculates the next midnight time
      const expiresAt = Math.ceil(timestamp / 86400 + 1) * 86400;
      // calculate the cost manually
      // const manualCost =
      //     (additionalRequestsPerKilosecondCost *
      //         requestsPerKilosecond *
      //         expirationTimeInSecondsFromNow) /
      //     1000n;

      const cost = await rateLimitNFTViewsContract.calculateCost(
        requestsPerKilosecond,
        expiresAt
      );

      const secondsBought = expiresAt - timestamp;

      // console.log("cost: ", cost.toString());
      // each additional kilosecond costs 1,000,000 wei, aka 1000 for each additional second
      // we're asking for 10001 request per kilosecond, aka 0.001 requests per second
      // for secondsBought seconds, so we need to pay secondsBought seconds * 1000 * 10001 for each second
      expect(cost).to.equal(
        BigInt(secondsBought) * 1000n * requestsPerKilosecond
      );

      // let's sanity check the opposite calculation
      const requestsPerKilosecondFromContract =
        await rateLimitNFTViewsContract.calculateRequestsPerKilosecond(
          cost,
          expiresAt
        );

      expect(requestsPerKilosecondFromContract).to.equal(requestsPerKilosecond);

      block = await ethers.provider.getBlock('latest');
      timestamp = BigInt(block.timestamp);
      // send eth with the txn
      expect(
        rateLimitNFTContract.mint(expiresAt, {
          value: cost,
        })
      ).revertedWith(
        "Can't allocate capacity beyond the global max requests per kilosecond"
      );
    });

    it('tries to mint past the max capacity with many NFTs', async () => {
      const maxRequestsPerKilosecond = 70n;
      // set max requests globally
      await rateLimitNFTContract.setMaxRequestsPerKilosecond(
        maxRequestsPerKilosecond
      );

      const requestsPerKilosecond = 10n;
      // this should be set to 1,000,000 wei
      const additionalRequestsPerKilosecondCost =
        await rateLimitNFTViewsContract.additionalRequestsPerKilosecondCost();
      expect(additionalRequestsPerKilosecondCost).to.equal(1000000);

      // get block timestamp
      let block = await ethers.provider.getBlock('latest');
      let timestamp = block.timestamp;
      // this calculates the next midnight time
      const expiresAt = Math.ceil(timestamp / 86400 + 1) * 86400;

      let toMint = 3;
      let expirations = [];
      let nfts = [];
      let totalSoldCapacity = 0n;
      for (let i = 0; i < toMint; i++) {
        let expirationTimeForThisNft = expiresAt + i * 86400;
        // console.log("expirationTimeForThisNft", expirationTimeForThisNft);
        const cost = await rateLimitNFTViewsContract.calculateCost(
          requestsPerKilosecond,
          expirationTimeForThisNft
        );
        const txn = await rateLimitNFTContract.mint(expirationTimeForThisNft, {
          value: cost,
        });
        nfts.push(txn);
        expirations.push(expirationTimeForThisNft);
        totalSoldCapacity += requestsPerKilosecond;
        // console.log("total sold capacity", totalSoldCapacity);
        const currentSoldRequestsPerKilosecond =
          await rateLimitNFTViewsContract.currentSoldRequestsPerKilosecond();
        expect(totalSoldCapacity).to.equal(currentSoldRequestsPerKilosecond);
        // console.log(
        //     "currentSoldRequestsPerKilosecond",
        //     currentSoldRequestsPerKilosecond
        // );
        const totalSoldRequestsPerKilosecondByExpirationTime =
          await rateLimitNFTViewsContract.totalSoldRequestsPerKilosecondByExpirationTime(
            expirationTimeForThisNft
          );
        expect(totalSoldRequestsPerKilosecondByExpirationTime).to.equal(10n);
        // console.log(
        //     "totalSoldRequestsPerKilosecondByExpirationTime",
        //     totalSoldRequestsPerKilosecondByExpirationTime
        // );
      }

      // run this loop again, so that we are testing putting multiple nfts in the same expiration time bucket
      for (let i = 0; i < toMint; i++) {
        let expirationTimeForThisNft = expiresAt + i * 86400;
        // console.log("expirationTimeForThisNft", expirationTimeForThisNft);
        const cost = await rateLimitNFTViewsContract.calculateCost(
          requestsPerKilosecond,
          expirationTimeForThisNft
        );
        const txn = await rateLimitNFTContract.mint(expirationTimeForThisNft, {
          value: cost,
        });
        nfts.push(txn);
        expirations.push(expirationTimeForThisNft);
        totalSoldCapacity += requestsPerKilosecond;
        // console.log("total sold capacity", totalSoldCapacity);
        const currentSoldRequestsPerKilosecond =
          await rateLimitNFTViewsContract.currentSoldRequestsPerKilosecond();
        expect(totalSoldCapacity).to.equal(currentSoldRequestsPerKilosecond);
        // console.log(
        //     "currentSoldRequestsPerKilosecond",
        //     currentSoldRequestsPerKilosecond
        // );
        const totalSoldRequestsPerKilosecondByExpirationTime =
          await rateLimitNFTViewsContract.totalSoldRequestsPerKilosecondByExpirationTime(
            expirationTimeForThisNft
          );
        expect(totalSoldRequestsPerKilosecondByExpirationTime).to.equal(20n);
        // console.log(
        //     "totalSoldRequestsPerKilosecondByExpirationTime",
        //     totalSoldRequestsPerKilosecondByExpirationTime
        // );
      }

      let currentSoldRequestsPerKilosecond =
        await rateLimitNFTViewsContract.currentSoldRequestsPerKilosecond();
      expect(currentSoldRequestsPerKilosecond).to.equal(60n);

      let cost = await rateLimitNFTViewsContract.calculateCost(11n, expiresAt);
      // at this point we've sold 290 of 300 total requests per kilosecond.  let's try minting 11 more requests per kilosecond and it should fail
      expect(
        rateLimitNFTContract.mint(expiresAt, {
          value: cost,
        })
      ).revertedWith(
        "Can't allocate capacity beyond the global max requests per kilosecond"
      );

      // advance the clock to tomorrow and try minting again.  it should succeed.
      const tomorrow = expiresAt + 1; // 1 second past midnight tonight
      await ethers.provider.send('evm_setNextBlockTimestamp', [tomorrow]);
      await ethers.provider.send('evm_mine', []);
      currentSoldRequestsPerKilosecond =
        await rateLimitNFTViewsContract.currentSoldRequestsPerKilosecond();
      expect(currentSoldRequestsPerKilosecond).to.equal(40n); // we advanced 1 day, but had 20 requests for today, so we should have 60 - 20 = 40 now.

      const expiresAtForTomorrow = expiresAt + 86400;
      cost = await rateLimitNFTViewsContract.calculateCost(
        11n,
        expiresAtForTomorrow
      );
      // let's mint
      await rateLimitNFTContract.mint(expiresAtForTomorrow, {
        value: cost,
      });
    });
  });

  describe('Test pruning expired NFTs', async () => {
    let minter;
    let admin;

    beforeEach(async () => ([minter, admin, ...signers] = signers));

    it('mints a rate limit increase nft, advances the clock to tomorrow, and prunes the expired NFT', async () => {
      // we would like 10 requests per kilosecond, which is 10 request per 1000 seconds.
      const requestsPerKilosecond = BigInt(10);
      // this should be set to 1,000,000 wei
      const additionalRequestsPerKilosecondCost =
        await rateLimitNFTViewsContract.additionalRequestsPerKilosecondCost();
      expect(additionalRequestsPerKilosecondCost).to.equal(1000000);

      // get block timestamp
      let block = await ethers.provider.getBlock('latest');
      let timestamp = block.timestamp;
      // this calculates the next midnight time
      const expiresAt = Math.ceil(timestamp / 86400 + 1) * 86400;
      // calculate the cost manually
      // const manualCost =
      //     (additionalRequestsPerKilosecondCost *
      //         requestsPerKilosecond *
      //         expirationTimeInSecondsFromNow) /
      //     1000n;

      const cost = await rateLimitNFTViewsContract.calculateCost(
        requestsPerKilosecond,
        expiresAt
      );

      const secondsBought = expiresAt - timestamp;

      // console.log("cost: ", cost.toString());
      // each additional kilosecond costs 1,000,000 wei, aka 1000 for each additional second
      // we're asking for 10 request per kilosecond, aka 0.01 requests per second
      // for secondsBought seconds, so we need to pay secondsBought seconds * 1000 * 10 for each second
      expect(cost).to.equal(
        BigInt(secondsBought) * 1000n * requestsPerKilosecond
      );

      // let's sanity check the opposite calculation
      const requestsPerKilosecondFromContract =
        await rateLimitNFTViewsContract.calculateRequestsPerKilosecond(
          cost,
          expiresAt
        );

      expect(requestsPerKilosecondFromContract).to.equal(requestsPerKilosecond);

      block = await ethers.provider.getBlock('latest');
      timestamp = BigInt(block.timestamp);
      // send eth with the txn
      let rateLimitNFTContractAsMinter = rateLimitNFTContract.connect(minter);
      let res = await rateLimitNFTContractAsMinter.mint(expiresAt, {
        value: cost,
      });
      let receipt = await res.wait();

      // get the tokenId from the event
      let tokenId = receipt.logs[0].topics[3];
      // console.log("tokenId", tokenId.toString());

      // check the params
      let capacity = await rateLimitNFTViewsContract.capacity(tokenId);
      expect(capacity[0]).to.equal(requestsPerKilosecond);
      expect(capacity[1]).to.equal(expiresAt);

      let tokenBalance = await rateLimitNFTContract.balanceOf(minter.address);
      expect(tokenBalance).to.equal(1);

      // mint 1 more, but set the expiration to 3 days from now.  this will show that the
      // pruning process only removes NFTs that have expired.
      const expiresAtThreeDaysFromNow =
        Math.ceil(expiresAt / 86400 + 3) * 86400; // 1 second past midnight in 3 days
      const costThreeDaysFromNow =
        await rateLimitNFTViewsContract.calculateCost(
          requestsPerKilosecond,
          expiresAtThreeDaysFromNow
        );
      res = await rateLimitNFTContractAsMinter.mint(expiresAtThreeDaysFromNow, {
        value: costThreeDaysFromNow,
      });
      receipt = await res.wait();

      tokenBalance = await rateLimitNFTContract.balanceOf(minter.address);
      expect(tokenBalance).to.equal(2);

      // advance the clock to tomorrow and try minting again.  it should succeed.
      // console.log('expiresAt', expiresAt);
      const tomorrow = Math.ceil(expiresAt / 86400 + 1) * 86400; // 1 second past midnight tonight
      // console.log('tomorrow', tomorrow);
      await ethers.provider.send('evm_setNextBlockTimestamp', [tomorrow]);
      await ethers.provider.send('evm_mine', []);

      // prune the expired NFT
      await rateLimitNFTContract.pruneExpired(minter.address);
      tokenBalance = await rateLimitNFTContract.balanceOf(minter.address);
      expect(tokenBalance).to.equal(1);
    });
  });
});
