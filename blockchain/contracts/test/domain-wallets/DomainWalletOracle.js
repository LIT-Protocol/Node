const { expect } = require("chai");
const { ethers } = require("hardhat");
const { Environment, setContractResolver } = require("../../utils/contract");

describe("DomainWalletOracle", function () {
    let oracleContract;
    let registryContract;
    let contractResolver;
    let pkpTokenId = parseInt(
        "04e4df1c02a7b9bd8f0668c344bbbae3a66b72273a50890ed8e653f513a247b8d2a39ddd5a6b6bb8ef1270fd105f73d0133886c9a4890777f2f4f8127cdf5cc1fc"
    );
    before(async () => {
        [deployer, ...signers] = await ethers.getSigners();
        contractResolver = await ethers.deployContract("ContractResolver", [
            Environment.DEV,
        ]);
        oracleContract = await ethers.deployContract("DomainWalletOracle", [
            await contractResolver.getAddress(),
            Environment.DEV,
        ]);
        registryContract = await ethers.deployContract("DomainWalletRegistry", [
            await contractResolver.getAddress(),
            Environment.DEV,
        ]);
        await setContractResolver(contractResolver, Environment.DEV, {
            domainWalletRegistryContract: registryContract,
        });
        await oracleContract.addAdmin(deployer.address);
    });

    it("should register uri", async () => {
        let uri = `0x${Buffer.from("foobar.lit.id").toString("hex")}`;
        let userId = `0x${Buffer.from("foo@bar.baz").toString("hex")}`;

        let ttl = Date.now();
        let tx = await oracleContract.registerDomain(
            userId,
            uri,
            pkpTokenId,
            ttl
        );

        let res = await tx.wait();
        let tokenId = await oracleContract.getDomainUri(pkpTokenId);

        expect(tokenId).to.equal(uri);
    });
});
