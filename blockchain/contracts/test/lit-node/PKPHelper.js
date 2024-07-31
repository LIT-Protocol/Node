const { expect } = require('chai');
const { ethers } = require('hardhat');
const { getBytesFromMultihash, getParamsFromPKPMint } = require('../../utils');
const { deployDiamond } = require('../../scripts/deployDiamond');
const {
  Environment,
  setContractResolver,
  allNodesVoteForRootKeys,
  setupStakingWithValidatorsAndAdvance,
} = require('../../utils/contract');
const { AbiCoder, toBeHex } = require('ethers');

describe('PKPHelper', function () {
  let deployer;
  let signers;
  let pkpContract;
  let router;
  let pkpHelper;
  let pkpPermissionsDiamond;
  let pkpPermissions;
  let contractResolver;
  let tokenContract;
  const totalTokens = BigInt('1000000000') * BigInt('10') ** BigInt('18'); // create 1,000,000,000 total tokens with 18 decimals

  beforeEach(async () => {
    [deployer, ...signers] = await ethers.getSigners();
    contractResolver = await ethers.deployContract('ContractResolver', [
      Environment.DEV,
    ]);
    const { diamond: pkpDiamond } = await deployDiamond(
      'PKPNFT',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PKPNFTFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    pkpContract = await ethers.getContractAt(
      'PKPNFTFacet',
      await pkpDiamond.getAddress()
    );
    pkpHelper = await ethers.deployContract('PKPHelper', [
      await contractResolver.getAddress(),
      Environment.DEV,
    ]);
    keyDeriver = await ethers.deployContract('KeyDeriver');

    let deployResult = await deployDiamond(
      'PubkeyRouter',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PubkeyRouterFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    routerDiamond = deployResult.diamond;
    router = await ethers.getContractAt(
      'PubkeyRouterFacet',
      await routerDiamond.getAddress()
    );

    deployResult = await deployDiamond(
      'PKPPermissions',
      await contractResolver.getAddress(),
      Environment.DEV,
      {
        additionalFacets: ['PKPPermissionsFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    pkpPermissionsDiamond = deployResult.diamond;
    pkpPermissions = await ethers.getContractAt(
      'PKPPermissionsFacet',
      await pkpPermissionsDiamond.getAddress()
    );

    tokenContract = await ethers.deployContract('LITToken', [
      ethers.parseUnits('1000000000', 18), // 1b tokens
    ]);

    const { diamond: stakingDiamond } = await deployDiamond(
      'Staking',
      await contractResolver.getAddress(),
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
    const stakingContract = await ethers.getContractAt(
      'StakingFacet',
      await stakingDiamond.getAddress()
    );
    const { diamond: stakingBalancesDiamond } = await deployDiamond(
      'StakingBalances',
      await contractResolver.getAddress(),
      0,
      {
        additionalFacets: ['StakingBalancesFacet'],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    const stakingBalancesContract = await ethers.getContractAt(
      'StakingBalancesFacet',
      await stakingBalancesDiamond.getAddress()
    );

    await setContractResolver(contractResolver, Environment.DEV, {
      tokenContract,
      stakingContract,
      stakingBalancesContract,
      pkpContract,
      pkpHelperContract: pkpHelper,
      pkpPermissionsContract: pkpPermissions,
      hdKeyDeriverContract: keyDeriver,
      pubkeyRouterContract: router,
    });

    // Mint enough tokens for the deployer
    await tokenContract.mint(deployer.address, totalTokens);
    const stakingAccounts = await setupStakingWithValidatorsAndAdvance(
      ethers,
      stakingContract,
      stakingBalancesContract,
      tokenContract,
      deployer,
      {
        numValidators: 3,
        startingPort: 7777,
        ipAddress: '192.168.1.1',
      }
    );
    await allNodesVoteForRootKeys(
      ethers,
      router,
      stakingContract,
      stakingAccounts
    );
  });

  describe('Attempt to Mint PKP NFT via PKPHelper', async () => {
    let minter;

    beforeEach(async () => {
      [minter, recipient, ...signers] = signers;
      pkpContract = pkpContract.connect(minter);
      pkpHelper = pkpHelper.connect(minter);
    });

    it('mints successfully with permitted auth methods', async () => {
      const addressesToPermit = [
        '0x75EdCdfb5A678290A8654979703bdb75C683B3dD',
        '0xeb250b8DA8021fE09Ea2D0121e20eDa65D523aA6',
      ];
      const ipfsIdsToPermit = [
        'QmPRjq7medLpjnFSZaiJ3xUudKteVFQDmaMZuhr644MQ4Z',
        'QmSX1eaPhZjxb8rJtejunop8Sq41FMSUVv9HfqtPNtVi7j',
      ];
      const ipfsIdsBytes = ipfsIdsToPermit.map((f) => getBytesFromMultihash(f));
      // const ipfsIdHash = ipfsIdToIpfsIdHash(ipfsIdToPermit);
      // const multihashStruct = getBytes32FromMultihash(ipfsIdToPermit);
      const authMethodTypes = [4, 5];
      const authMethodUserIds = [
        '0xdeadbeef',
        '0x7ce7b7b6766949f0bf8552a0db7117de4e5628321ae8c589e67e5839ee3c1912402dfd0ed9be127812d0d2c16df2ac2c319ebed0927b0de98a3b946767577ad7',
      ];
      const authMethodPubkeys = [
        '0xacbe9af83570da302d072984c4938bd7d9dd86186ebedf53d693171d48dbf5e60e2ae9dc9f72ee9592b054ec0a9de5d3bac6a35b9f658b5183c40990e588ffea',
        '0x00',
      ];
      const authMethodIdHashes = authMethodUserIds.map((f, idx) =>
        ethers.keccak256(
          AbiCoder.defaultAbiCoder().encode(
            ['uint256', 'bytes'],
            [authMethodTypes[idx], f]
          )
        )
      );

      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const tx = await pkpHelper.mintNextAndAddAuthMethodsWithTypes(
        2,
        ipfsIdsBytes,
        [[], []],
        addressesToPermit,
        [[], []],
        authMethodTypes,
        authMethodUserIds,
        authMethodPubkeys,
        [[], []],
        true, //addPkpEthAddressAsPermittedAddress,
        false, // sendPkpToItself
        transaction
      );
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);
      expect(tokenId.toString().length).to.be.greaterThan(0);
      expect(pubkey.length).to.be.equal(132);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      const pkpEthAddress = await pkpPermissions.getEthAddress(tokenId);

      // check the auth methods
      for (let i = 0; i < addressesToPermit.length; i++) {
        const actionIsPermitted = await pkpPermissions.isPermittedAction(
          tokenId,
          ipfsIdsBytes[i]
        );
        expect(actionIsPermitted).to.equal(true);
      }

      for (let i = 0; i < addressesToPermit.length; i++) {
        const addressIsPermitted = await pkpPermissions.isPermittedAddress(
          tokenId,
          addressesToPermit[i]
        );
        expect(addressIsPermitted).to.equal(true);
      }

      // confirm that the owner is also permitted
      const ownerIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        minter.address
      );
      expect(ownerIsPermitted).to.equal(true);

      // confirm that the pkp eth address is permitted
      const pkpEthAddressIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        pkpEthAddress
      );
      expect(pkpEthAddressIsPermitted).to.equal(true);

      for (let i = 0; i < authMethodTypes.length; i++) {
        const authMethodIsPermitted =
          await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodTypes[i],
            authMethodUserIds[i]
          );
        expect(authMethodIsPermitted).to.equal(true);
      }

      // check the reverse mapping of the auth method
      for (let i = 0; i < authMethodTypes.length; i++) {
        const authedTokenIds = (
          await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodTypes[i],
            authMethodUserIds[i]
          )
        ).map((f) => toBeHex(f));
        expect(authedTokenIds).to.deep.equal([tokenId]);
      }

      // check all the getters
      const permittedActions = await pkpPermissions.getPermittedActions(
        tokenId
      );
      // console.log("permittedActions: ", permittedActions);
      expect(permittedActions).to.deep.equal(ipfsIdsBytes);

      const permittedAddresses = await pkpPermissions.getPermittedAddresses(
        tokenId
      );
      expect(permittedAddresses).to.deep.equal([
        minter.address,
        ...addressesToPermit,
        pkpEthAddress,
      ]);

      const permittedAuthMethods = await pkpPermissions.getPermittedAuthMethods(
        tokenId
      );
      expect(permittedAuthMethods.length).to.equal(7);
      let authMethodIndex = 0;
      for (let i = 0; i < permittedAuthMethods.length; i++) {
        if (
          Number(permittedAuthMethods[i][0]) !== 1 &&
          Number(permittedAuthMethods[i][0]) !== 2
        ) {
          expect([
            Number(permittedAuthMethods[i][0]),
            permittedAuthMethods[i][1],
            permittedAuthMethods[i][2],
          ]).to.deep.equal([
            authMethodTypes[authMethodIndex],
            authMethodUserIds[authMethodIndex],
            authMethodPubkeys[authMethodIndex],
          ]);
          authMethodIndex++;
        }
      }
    });

    it('mints successfully with permitted auth methods using the simple non-typed function', async () => {
      const addressesToPermit = [
        '0x75EdCdfb5A678290A8654979703bdb75C683B3dD',
        '0xeb250b8DA8021fE09Ea2D0121e20eDa65D523aA6',
      ];
      const ipfsIdsToPermit = [
        'QmPRjq7medLpjnFSZaiJ3xUudKteVFQDmaMZuhr644MQ4Z',
        'QmSX1eaPhZjxb8rJtejunop8Sq41FMSUVv9HfqtPNtVi7j',
      ];
      const ipfsIdsBytes = ipfsIdsToPermit.map((f) => getBytesFromMultihash(f));

      const authMethodTypes = [4, 5, 2, 2, 1, 1];
      const authMethodUserIds = [
        '0xdeadbeef',
        '0x7ce7b7b6766949f0bf8552a0db7117de4e5628321ae8c589e67e5839ee3c1912402dfd0ed9be127812d0d2c16df2ac2c319ebed0927b0de98a3b946767577ad7',
        ...ipfsIdsBytes,
        ...addressesToPermit,
      ];
      const authMethodPubkeys = [
        '0xacbe9af83570da302d072984c4938bd7d9dd86186ebedf53d693171d48dbf5e60e2ae9dc9f72ee9592b054ec0a9de5d3bac6a35b9f658b5183c40990e588ffea',
        '0x00',
        ...ipfsIdsBytes.map((r) => '0x00'),
        ...addressesToPermit.map((r) => '0x00'),
      ];

      const authMethodScopes = authMethodUserIds.map((r) => []);

      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const tx = await pkpHelper.mintNextAndAddAuthMethods(
        2,
        authMethodTypes,
        authMethodUserIds,
        authMethodPubkeys,
        authMethodScopes,
        true, //addPkpEthAddressAsPermittedAddress,
        false, // sendPkpToItself
        transaction
      );
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);
      expect(tokenId.toString().length).to.be.greaterThan(0);
      expect(pubkey.length).to.be.equal(132);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      const pkpEthAddress = await pkpPermissions.getEthAddress(tokenId);

      // check the auth methods
      for (let i = 0; i < addressesToPermit.length; i++) {
        const actionIsPermitted = await pkpPermissions.isPermittedAction(
          tokenId,
          ipfsIdsBytes[i]
        );
        expect(actionIsPermitted).to.equal(true);
      }

      for (let i = 0; i < addressesToPermit.length; i++) {
        const addressIsPermitted = await pkpPermissions.isPermittedAddress(
          tokenId,
          addressesToPermit[i]
        );
        expect(addressIsPermitted).to.equal(true);
      }

      // confirm that the owner is also permitted
      const ownerIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        minter.address
      );
      expect(ownerIsPermitted).to.equal(true);

      // confirm that the pkp eth address is permitted
      const pkpEthAddressIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        pkpEthAddress
      );
      expect(pkpEthAddressIsPermitted).to.equal(true);

      for (let i = 0; i < authMethodTypes.length; i++) {
        const authMethodIsPermitted =
          await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodTypes[i],
            authMethodUserIds[i]
          );
        expect(authMethodIsPermitted).to.equal(true);
      }

      // check the reverse mapping of the auth method
      for (let i = 0; i < authMethodTypes.length; i++) {
        const authedTokenIds = (
          await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodTypes[i],
            authMethodUserIds[i]
          )
        ).map((f) => toBeHex(f));
        expect(authedTokenIds).to.deep.equal([tokenId]);
      }

      // check all the getters
      const permittedActions = await pkpPermissions.getPermittedActions(
        tokenId
      );
      // console.log("permittedActions: ", permittedActions);
      expect(permittedActions).to.deep.equal(ipfsIdsBytes);

      const permittedAddresses = await pkpPermissions.getPermittedAddresses(
        tokenId
      );
      expect(permittedAddresses).to.deep.equal([
        minter.address,
        ...addressesToPermit,
        pkpEthAddress,
      ]);

      const permittedAuthMethods = await pkpPermissions.getPermittedAuthMethods(
        tokenId
      );
      expect(permittedAuthMethods.length).to.equal(7);
      let authMethodIndex = 0;
      for (let i = 0; i < permittedAuthMethods.length; i++) {
        if (
          Number(permittedAuthMethods[i][0]) !== 1 &&
          Number(permittedAuthMethods[i][0]) !== 2
        ) {
          expect([
            Number(permittedAuthMethods[i][0]),
            permittedAuthMethods[i][1],
            permittedAuthMethods[i][2],
          ]).to.deep.equal([
            authMethodTypes[authMethodIndex],
            authMethodUserIds[authMethodIndex],
            authMethodPubkeys[authMethodIndex],
          ]);
          authMethodIndex++;
        }
      }
    });

    it('mints successfully with permitted auth methods and sends the PKP to itself', async () => {
      const addressesToPermit = [
        '0x75EdCdfb5A678290A8654979703bdb75C683B3dD',
        '0xeb250b8DA8021fE09Ea2D0121e20eDa65D523aA6',
      ];
      const ipfsIdsToPermit = [
        'QmPRjq7medLpjnFSZaiJ3xUudKteVFQDmaMZuhr644MQ4Z',
        'QmSX1eaPhZjxb8rJtejunop8Sq41FMSUVv9HfqtPNtVi7j',
      ];
      const ipfsIdsBytes = ipfsIdsToPermit.map((f) => getBytesFromMultihash(f));
      const authMethodTypes = [4, 5];
      const authMethodUserIds = [
        '0xdeadbeef',
        '0x7ce7b7b6766949f0bf8552a0db7117de4e5628321ae8c589e67e5839ee3c1912402dfd0ed9be127812d0d2c16df2ac2c319ebed0927b0de98a3b946767577ad7',
      ];
      const authMethodPubkeys = [
        '0xacbe9af83570da302d072984c4938bd7d9dd86186ebedf53d693171d48dbf5e60e2ae9dc9f72ee9592b054ec0a9de5d3bac6a35b9f658b5183c40990e588ffea',
        '0x00',
      ];
      const authMethodIdHashes = authMethodUserIds.map((f, idx) =>
        ethers.keccak256(
          AbiCoder.defaultAbiCoder().encode(
            ['uint256', 'bytes'],
            [authMethodTypes[idx], f]
          )
        )
      );

      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const tx = await pkpHelper.mintNextAndAddAuthMethodsWithTypes(
        2,
        ipfsIdsBytes,
        [[], []],
        addressesToPermit,
        [[], []],
        authMethodTypes,
        authMethodUserIds,
        authMethodPubkeys,
        [[], []],
        true, //addPkpEthAddressAsPermittedAddress,
        true, // sendPkpToItself
        transaction
      );
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);
      expect(tokenId.toString().length).to.be.greaterThan(0);
      expect(pubkey.length).to.be.equal(132);

      const pkpEthAddress = await pkpPermissions.getEthAddress(tokenId);

      // check the token was minted and is owned by the PKP itself
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(pkpEthAddress);

      // check the auth methods
      for (let i = 0; i < addressesToPermit.length; i++) {
        const actionIsPermitted = await pkpPermissions.isPermittedAction(
          tokenId,
          ipfsIdsBytes[i]
        );
        expect(actionIsPermitted).to.equal(true);
      }

      for (let i = 0; i < addressesToPermit.length; i++) {
        const addressIsPermitted = await pkpPermissions.isPermittedAddress(
          tokenId,
          addressesToPermit[i]
        );
        expect(addressIsPermitted).to.equal(true);
      }

      // confirm that the owner is also permitted
      const ownerIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        pkpEthAddress
      );
      expect(ownerIsPermitted).to.equal(true);

      // confirm that the pkp eth address is permitted
      const pkpEthAddressIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        pkpEthAddress
      );
      expect(pkpEthAddressIsPermitted).to.equal(true);

      for (let i = 0; i < authMethodTypes.length; i++) {
        const authMethodIsPermitted =
          await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodTypes[i],
            authMethodUserIds[i]
          );
        expect(authMethodIsPermitted).to.equal(true);
      }

      // check the reverse mapping of the auth method
      for (let i = 0; i < authMethodTypes.length; i++) {
        const authedTokenIds = (
          await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodTypes[i],
            authMethodUserIds[i]
          )
        ).map((f) => toBeHex(f));
        expect(authedTokenIds).to.deep.equal([tokenId]);
      }

      // check all the getters
      const permittedActions = await pkpPermissions.getPermittedActions(
        tokenId
      );
      // console.log("permittedActions: ", permittedActions);
      expect(permittedActions).to.deep.equal(ipfsIdsBytes);

      const permittedAddresses = await pkpPermissions.getPermittedAddresses(
        tokenId
      );
      expect(permittedAddresses).to.deep.equal([
        pkpEthAddress,
        ...addressesToPermit,
        pkpEthAddress,
      ]);

      const permittedAuthMethods = await pkpPermissions.getPermittedAuthMethods(
        tokenId
      );
      expect(permittedAuthMethods.length).to.equal(7);
      let authMethodIndex = 0;
      for (let i = 0; i < permittedAuthMethods.length; i++) {
        if (
          Number(permittedAuthMethods[i][0]) !== 1 &&
          Number(permittedAuthMethods[i][0]) !== 2
        ) {
          expect([
            Number(permittedAuthMethods[i][0]),
            permittedAuthMethods[i][1],
            permittedAuthMethods[i][2],
          ]).to.deep.equal([
            authMethodTypes[authMethodIndex],
            authMethodUserIds[authMethodIndex],
            authMethodPubkeys[authMethodIndex],
          ]);
          authMethodIndex++;
        }
      }
    });

    it('mints without setting the pkp nft address as permitted', async () => {
      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const tx = await pkpHelper.mintNextAndAddAuthMethodsWithTypes(
        2,
        [],
        [],
        [],
        [],
        [],
        [],
        [],
        [],
        false, //addPkpEthAddressAsPermittedAddress,
        false, //sendPkpToItself
        transaction
      );

      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);
      expect(tokenId.toString().length).to.be.greaterThan(0);
      expect(pubkey.length).to.be.equal(132);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      // check all the getters
      const permittedActions = await pkpPermissions.getPermittedActions(
        tokenId
      );
      // console.log("permittedActions: ", permittedActions);
      expect(permittedActions).to.deep.equal([]);

      const permittedAddresses = await pkpPermissions.getPermittedAddresses(
        tokenId
      );
      expect(permittedAddresses).to.deep.equal([minter.address]);

      const permittedAuthMethods = await pkpPermissions.getPermittedAuthMethods(
        tokenId
      );
      expect(permittedAuthMethods.length).to.equal(0);
    });

    it('mints successfully with empty auth methods', async () => {
      const addressesToPermit = [];
      const ipfsIdsToPermit = [];
      const ipfsIdsBytes = ipfsIdsToPermit.map((f) => getBytesFromMultihash(f));
      const authMethodTypes = [];
      const authMethodUserIds = [];
      const authMethodPubkeys = [];

      // send eth with the txn
      const mintCost = await pkpContract.mintCost();
      const transaction = {
        value: mintCost,
      };

      const tx = await pkpHelper.mintNextAndAddAuthMethodsWithTypes(
        2,
        ipfsIdsBytes,
        [],
        addressesToPermit,
        [],
        authMethodTypes,
        authMethodUserIds,
        authMethodPubkeys,
        [],
        true, //addPkpEthAddressAsPermittedAddress,
        false, // sendPkpToItself
        transaction
      );
      expect(tx).to.emit(pkpContract, 'PKPMinted');
      const { tokenId, pubkey } = await getParamsFromPKPMint(tx, pkpContract);
      expect(tokenId.toString().length).to.be.greaterThan(0);
      expect(pubkey.length).to.be.equal(132);

      // check the token was minted
      const owner = await pkpContract.ownerOf(tokenId);
      expect(owner).to.equal(minter.address);

      const pkpEthAddress = await pkpPermissions.getEthAddress(tokenId);

      // confirm that the owner is permitted
      const ownerIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        minter.address
      );
      expect(ownerIsPermitted).to.equal(true);

      // confirm that the pkp eth address is permitted
      const pkpEthAddressIsPermitted = await pkpPermissions.isPermittedAddress(
        tokenId,
        pkpEthAddress
      );
      expect(pkpEthAddressIsPermitted).to.equal(true);

      // check all the getters
      const permittedActions = await pkpPermissions.getPermittedActions(
        tokenId
      );
      expect(permittedActions.length).to.equal(0);

      const permittedAddresses = await pkpPermissions.getPermittedAddresses(
        tokenId
      );
      expect(permittedAddresses).to.deep.equal([minter.address, pkpEthAddress]);

      const permittedAuthMethods = await pkpPermissions.getPermittedAuthMethods(
        tokenId
      );
      expect(permittedAuthMethods.length).to.equal(1);
    });
  });
});
