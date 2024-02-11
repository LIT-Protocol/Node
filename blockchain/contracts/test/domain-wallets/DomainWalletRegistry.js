const { expect } = require("chai");
const { deployDiamond } = require("../../scripts/deployDiamond");
const {
    Environment,
    setContractResolver,
    setupStakingWithValidatorsAndAdvance,
    allNodesVoteForRootKeys,
} = require("../../utils/contract");

describe("DomainWalletRegistry", function () {
    let oracleContract;
    let registryContract;
    let cachedTokenId;
    const totalTokens = BigInt("1000000000") * BigInt("10") ** BigInt("18"); // create 1,000,000,000 total tokens with 18 decimals

    beforeEach(async () => {
        const contractResolver = await ethers.deployContract(
            "ContractResolver",
            [Environment.DEV]
        );

        [deployer, ...signers] = await ethers.getSigners();
        let [minter] = signers;

        token = await ethers.deployContract(
            "LITToken",
            [ethers.parseUnits("1000000000", 18)] // 1b tokens
        );

        registryContract = await ethers.deployContract("DomainWalletRegistry", [
            await contractResolver.getAddress(),
            Environment.DEV,
        ]);

        await registryContract.addAdmin(deployer.address);

        oracleContract = await ethers.deployContract("DomainWalletOracle", [
            await contractResolver.getAddress(),
            Environment.DEV,
        ]);

        pkpNftMetadata = await ethers.deployContract("PKPNFTMetadata", [
            await contractResolver.getAddress(),
            Environment.DEV,
        ]);
        const { diamond: pkpDiamond } = await deployDiamond(
            "PKPNFT",
            await contractResolver.getAddress(),
            Environment.DEV,
            ["PKPNFTFacet"],
            false
        );
        pkpContract = await ethers.getContractAt(
            "PKPNFTFacet",
            await pkpDiamond.getAddress()
        );

        const { diamond: routerDiamond } = await deployDiamond(
            "PubkeyRouter",
            await contractResolver.getAddress(),
            Environment.DEV,
            ["PubkeyRouterFacet"]
        );
        router = await ethers.getContractAt(
            "PubkeyRouterFacet",
            await routerDiamond.getAddress()
        );
        let deployResult = await deployDiamond(
            "PKPPermissions",
            await contractResolver.getAddress(),
            Environment.DEV,
            ["PKPPermissionsFacet"]
        );
        pkpPermissionsDiamond = deployResult.diamond;
        pkpPermissions = await ethers.getContractAt(
            "PKPPermissionsFacet",
            await pkpPermissionsDiamond.getAddress()
        );
        pkpHelper = await ethers.deployContract("PKPHelper", [
            await contractResolver.getAddress(),
            Environment.DEV,
        ]);
        const keyDeriver = await ethers.deployContract("KeyDeriver");
        const { diamond: stakingDiamond } = await deployDiamond(
            "Staking",
            await contractResolver.getAddress(),
            0,
            ["StakingFacet", "StakingViewsFacet", "StakingVersionFacet"]
        );
        const stakingContract = await ethers.getContractAt(
            "StakingFacet",
            await stakingDiamond.getAddress()
        );
        const { diamond: stakingBalancesDiamond } = await deployDiamond(
            "StakingBalances",
            await contractResolver.getAddress(),
            0,
            ["StakingBalancesFacet"]
        );
        const stakingBalancesContract = await ethers.getContractAt(
            "StakingBalancesFacet",
            await stakingBalancesDiamond.getAddress()
        );

        pkpContract = pkpContract.connect(minter);
        pkpHelper = pkpHelper.connect(minter);

        await setContractResolver(contractResolver, Environment.DEV, {
            tokenContract: token,
            stakingContract,
            stakingBalancesContract,
            pkpContract,
            pkpPermissionsContract: pkpPermissions,
            pkpHelperContract: pkpHelper,
            pkpNftMetadataContract: pkpNftMetadata,
            domainWalletRegistryContract: registryContract,
            domainWalletOracleContract: oracleContract,
            hdKeyDeriverContract: keyDeriver,
            pubkeyRouterContract: router,
        });

        // Mint enough tokens for the deployer
        await token.mint(deployer.address, totalTokens);
        stakingAccounts = await setupStakingWithValidatorsAndAdvance(
            ethers,
            stakingContract,
            stakingBalancesContract,
            token,
            deployer,
            {
                numValidators: 3,
                startingPort: 7777,
                ipAddress: "192.168.1.1",
            }
        );
        await allNodesVoteForRootKeys(
            ethers,
            router,
            stakingContract,
            stakingAccounts
        );
    });

    it("should register uri", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        let ttl = Date.now();

        const mintCost = await pkpContract.mintCost();

        const transaction = {
            value: mintCost,
        };
        let tx = await registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );
        let res = await tx.wait();
        let tokenId = res.logs[0].topics[1];
        cachedTokenId = tokenId;
        let domainUri = await registryContract.getDomainUri(tokenId);

        expect(domainUri).to.equal(uri);
    });

    it("converted expirery should match registration expirery time", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        let ttl = Date.now();

        const mintCost = await pkpContract.mintCost();
        const transaction = {
            value: mintCost,
        };
        let tx = await registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );
        let res = await tx.wait();
        let tokenId = res.logs[0].topics[1];

        let exp = await registryContract.getExpiration(tokenId);
        expect(exp).to.equal(BigInt(ttl));
    });

    it("revoked domain should not be routed", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        let ttl = Date.now();

        const mintCost = await pkpContract.mintCost();
        const transaction = {
            value: mintCost,
        };
        let tx = await registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );
        let res = await tx.wait();
        let tokenId = res.logs[0].topics[1];

        tx = await registryContract.removeDomain(tokenId);
        tx = await tx.wait();

        const isRouted = await registryContract.isRouted(tokenId);
        expect(isRouted).to.equal(false);
    });

    it("revoked domain should not be have owner", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        const id = 1;
        let ttl = Date.now();

        const mintCost = await pkpContract.mintCost();
        const transaction = {
            value: mintCost,
        };
        let tx = await registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );
        let res = await tx.wait();
        let tokenId = res.logs[0].topics[1];

        tx = await registryContract.removeDomain(tokenId);
        tx = await tx.wait();

        const isRouted = await registryContract.isRouted(tokenId);
        const isOwner = await registryContract.hasOwner(tokenId);
        expect(isRouted).to.equal(false);
        expect(isOwner).to.equal(false);
        const resolvedDomain = await registryContract.getDomainUri(tokenId);
        expect(resolvedDomain).to.equal("0x");
        const resolvedTokenId = await registryContract.getPkpTokenId(id);
        expect(resolvedTokenId).to.equal(0);
    });

    it("registered domain should have pkp owner", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        let ttl = Date.now();

        const mintCost = await pkpContract.mintCost();
        const transaction = {
            value: mintCost,
        };
        let tx = await registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );
        let res = await tx.wait();
        let tokenId = res.logs[0].topics[1];

        let isOwner = await registryContract.hasOwner(tokenId);
        expect(isOwner).to.equal(true);
    });

    it("Should not mint if domain is registered", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        let ttl = Date.now();

        const mintCost = await pkpContract.mintCost();
        const transaction = {
            value: mintCost,
        };
        let tx = await registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );

        let failureTx = registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );
        expect(failureTx).to.be.revertedWith("DomainAlreadyRegistered");
    });

    it("registered domain expirey should be true", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        let ttl = Date.now();
        ttl = (ttl / 1000).toFixed(0);
        const mintCost = await pkpContract.mintCost();
        const transaction = {
            value: mintCost,
        };
        let tx = await registryContract.registerDomainAndMintNext(
            userId,
            uri,
            ttl,
            [],
            [],
            [],
            [],
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ],
            transaction
        );
        let res = await tx.wait();
        let tokenId = res.logs[0].topics[1];
        await new Promise((resolve, reject) => {
            setTimeout(() => {
                resolve();
            }, 2000);
        });
        let expTx = await registryContract.hasExpired(tokenId);
        expTx = await expTx.wait();
        const decodedEvent = oracleContract.interface.decodeEventLog(
            "Expired",
            expTx.logs[0].data,
            expTx.logs[0].topics
        );
        let logTTL = Number(decodedEvent.ttl);
        expect(parseInt(logTTL, 10)).to.equal(parseInt(ttl, 10));
    }, 10_000);

    it("Should not register domain already registered", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;
        let ttl = Date.now();

        let tx = await registryContract.registerDomain(
            userId,
            uri,
            ttl,
            cachedTokenId,
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ]
        );
        tx = await tx.wait();

        let failureTx = registryContract.registerDomain(
            userId,
            uri,
            ttl,
            cachedTokenId,
            [
                "foobar.lit.id",
                "https://yt3.googleusercontent.com/ytc/AGIKgqOnBN9Fze9naSk9bKiMEMwBqFyrxeVhYoVSM1rl=s176-c-k-c0x00ffffff-no-rj-mo",
            ]
        );

        expect(failureTx).to.be.revertedWith("DomainAlreadyRegistered");
    });
});
