const { expect } = require('chai');
const { StandardMerkleTree } = require('@openzeppelin/merkle-tree');
const { getBytesFromMultihash, getParamsFromPKPMint } = require('../../utils');

const { deployDiamond } = require('../../scripts/deployDiamond');
const {
  Environment,
  setContractResolver,
  setupStakingWithValidatorsAndAdvance,
  allNodesVoteForRootKeys,
} = require('../../utils/contract');

describe('PKPPermissions', function () {
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

  let tokenId;

  before(async () => {
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
          'StakingAdminFacet',
        ],
        verifyContracts: false,
        waitForDeployment: false,
      }
    );
    const stakingContract = await ethers.getContractAt(
      'StakingFacet',
      await stakingDiamond.getAddress()
    );
    const stakingAdminFacet = await ethers.getContractAt(
      'StakingAdminFacet',
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
      stakingAdminFacet,
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

  describe('register a PKP and set routing permissions', async function () {
    context(
      'when the PKP grants permission to an ETH address',
      async function () {
        let tester;
        let creator;

        before(async () => {
          [creator, tester, ...signers] = signers;

          router = await router.connect(deployer);

          // mint the PKP to the tester account
          pkpContract = await pkpContract.connect(tester);
          // send eth with the txn
          const mintCost = await pkpContract.mintCost();
          const transaction = {
            value: mintCost,
          };
          const tx = await pkpContract.mintNext(2, transaction);
          expect(tx).to.emit(pkpContract, 'PKPMinted');
          let params = await getParamsFromPKPMint(tx, pkpContract);
          tokenId = params.tokenId;
          pubkey = params.pubkey;
          expect(tokenId.toString().length).to.be.greaterThan(0);
          expect(pubkey.length).to.be.equal(132);

          // validate that it was set
          const [pubkeyAfter, keyTypeAfter, _derivedKeyIdAfter] =
            await router.getRoutingData(tokenId);
          expect(pubkeyAfter).equal(pubkey);

          expect(keyTypeAfter).equal(2);

          const owner = await pkpContract.ownerOf(tokenId);
          expect(owner).equal(tester.address);
        });

        it('grants permission to an eth address and then revokes it', async () => {
          const addressToPermit = '0x75EdCdfb5A678290A8654979703bdb75C683B3dD';

          pkpContract = await pkpContract.connect(tester);

          // validate that the address is not permitted
          let permitted = await pkpPermissions.isPermittedAddress(
            tokenId,
            addressToPermit
          );
          expect(permitted).equal(false);

          // permit it
          pkpPermissions = await pkpPermissions.connect(tester);
          await pkpPermissions.addPermittedAddress(
            tokenId,
            addressToPermit,
            []
          );
          permitted = await pkpPermissions.isPermittedAddress(
            tokenId,
            addressToPermit
          );
          expect(permitted).equal(true);

          // revoke
          await pkpPermissions.removePermittedAddress(tokenId, addressToPermit);
          permitted = await pkpPermissions.isPermittedAddress(
            tokenId,
            addressToPermit
          );
          expect(permitted).equal(false);
        });

        it('grants permission to an IPFS id and then revokes it', async () => {
          const ipfsIdToPermit =
            'QmNc6gpdFBq1dF1imq5xhHQPmbWuL7ScGXChr2rjPgfkbZ';
          const ipfsIdBytes = getBytesFromMultihash(ipfsIdToPermit);

          pkpContract = await pkpContract.connect(tester);

          // validate that the ipfs ID is not permitted
          let permitted = await pkpPermissions.isPermittedAction(
            tokenId,
            ipfsIdBytes
          );
          expect(permitted).equal(false);

          await pkpPermissions.addPermittedAction(tokenId, ipfsIdBytes, []);

          permitted = await pkpPermissions.isPermittedAction(
            tokenId,
            ipfsIdBytes
          );
          expect(permitted).equal(true);

          // revoke
          await pkpPermissions.removePermittedAction(tokenId, ipfsIdBytes);
          permitted = await pkpPermissions.isPermittedAction(
            tokenId,
            ipfsIdBytes
          );
          expect(permitted).equal(false);

          await pkpPermissions.addPermittedAction(tokenId, ipfsIdBytes, []);

          permitted = await pkpPermissions.isPermittedAction(
            tokenId,
            ipfsIdBytes
          );
          expect(permitted).equal(true);

          // try to add it again.  this should either work or trigger the revert issue
          // of "Cannot add a different pubkey for the same auth method type and id"
          await expect(
            pkpPermissions.addPermittedAction(tokenId, ipfsIdBytes, [])
          ).to.be.ok;

          permitted = await pkpPermissions.isPermittedAction(
            tokenId,
            ipfsIdBytes
          );
          expect(permitted).equal(true);
        });

        it('registers and grants permission to a webauthn AuthMethod', async () => {
          const authMethodType = 3;
          const userId = '0xdeadbeef1234';
          const userPubkey = '0x9876543210';

          pkpContract = await pkpContract.connect(tester);

          // validate that the auth method is not permitted
          let permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(false);

          // attempt to permit it
          pkpPermissions = await pkpPermissions.connect(tester);
          await pkpPermissions.addPermittedAuthMethod(
            tokenId,
            [authMethodType, userId, userPubkey],
            []
          );

          // lookup the pubkey by the user id
          let pubkey = await pkpPermissions.getUserPubkeyForAuthMethod(
            authMethodType,
            userId
          );
          // console.log("pubkey stored in contract", pubkey);
          // console.log("userPubkey", userPubkey);
          expect(pubkey).equal(userPubkey);

          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(true);

          // do a reverse lookup
          let pkpIds = await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodType,
            userId
          );
          expect(pkpIds.length).equal(1);
          expect(pkpIds[0]).equal(tokenId);

          // Confirm that the reverse lookup and public key derivation is correct
          let pkpPublicKeys = await pkpPermissions.getPKPPubKeysByAuthMethod(
            authMethodType,
            userId
          );
          expect(pkpPublicKeys.length).greaterThan(0);
          // Check the pub keys match when looked up with a token id
          let pkpPublicKey = await pkpPermissions.getPubkey(tokenId);
          expect(pkpPublicKey).equal(pkpPublicKeys[0]);

          // try changing the pubkey
          expect(
            // attempt to permit it
            pkpPermissions.addPermittedAuthMethod(
              tokenId,
              [authMethodType, userId, '0x55'], // a new pubkey
              []
            )
          ).revertedWith(
            'Cannot add a different pubkey for the same auth method type and id'
          );

          // lookup the pubkey by the user id and make sure it's still correct
          pubkey = await pkpPermissions.getUserPubkeyForAuthMethod(
            authMethodType,
            userId
          );
          // console.log("pubkey stored in contract", pubkey);
          // console.log("userPubkey", userPubkey);
          expect(pubkey).equal(userPubkey);

          // revoke
          await pkpPermissions.removePermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(false);

          // try changing the pubkey again now that we revoked the auth method
          // it should still fail
          expect(
            // attempt to permit it
            pkpPermissions.addPermittedAuthMethod(
              tokenId,
              [authMethodType, userId, '0x66'], // a new pubkey
              []
            )
          ).revertedWith(
            'Cannot add a different pubkey for the same auth method type and id'
          );

          // lookup the pubkey by the user id and make sure it's still correct
          pubkey = await pkpPermissions.getUserPubkeyForAuthMethod(
            authMethodType,
            userId
          );
          // console.log("pubkey stored in contract", pubkey);
          // console.log("userPubkey", userPubkey);
          expect(pubkey).equal(userPubkey);

          // confirm that it's still not permitted
          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(false);

          // confirm that the reverse lookup is now empty
          pkpIds = await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodType,
            userId
          );
          expect(pkpIds.length).equal(0);
        });

        it('registers and grants permission to a generic AuthMethod with scopes', async () => {
          const authMethodType = 5;
          const userId = '0xdead1234beef';
          const userPubkey = '0x98765432101234';
          const scopes = [10, 20];

          pkpContract = await pkpContract.connect(tester);

          // validate that the auth method is not permitted
          let permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(false);

          // make sure the scopes aren't set
          let storedScopes = await pkpPermissions.getPermittedAuthMethodScopes(
            tokenId,
            authMethodType,
            userId,
            256
          );
          expect(storedScopes.length).equal(256);

          for (let i = 0; i < storedScopes.length; i++) {
            expect(storedScopes[i]).equal(false);
          }

          // check the scopes one by one
          for (let i = 0; i < scopes.length; i++) {
            const scopePresent =
              await pkpPermissions.isPermittedAuthMethodScopePresent(
                tokenId,
                authMethodType,
                userId,
                scopes[i]
              );
            expect(scopePresent).equal(false);
          }

          // attempt to permit it
          pkpPermissions = await pkpPermissions.connect(tester);
          await pkpPermissions.addPermittedAuthMethod(
            tokenId,
            [authMethodType, userId, userPubkey],
            scopes
          );

          // lookup the pubkey by the user id
          let pubkey = await pkpPermissions.getUserPubkeyForAuthMethod(
            authMethodType,
            userId
          );
          // console.log("pubkey stored in contract", pubkey);
          // console.log("userPubkey", userPubkey);
          expect(pubkey).equal(userPubkey);

          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(true);

          // do a reverse lookup
          let pkpIds = await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodType,
            userId
          );
          expect(pkpIds.length).equal(1);
          expect(pkpIds[0]).equal(tokenId);

          // check the scopes
          storedScopes = await pkpPermissions.getPermittedAuthMethodScopes(
            tokenId,
            authMethodType,
            userId,
            256
          );
          expect(storedScopes.length).equal(256);
          for (let i = 0; i < scopes.length; i++) {
            expect(storedScopes[scopes[i]]).equal(true);
          }

          // check the scopes one by one
          for (let i = 0; i < scopes.length; i++) {
            const scopePresent =
              await pkpPermissions.isPermittedAuthMethodScopePresent(
                tokenId,
                authMethodType,
                userId,
                scopes[i]
              );
            expect(scopePresent).equal(true);
          }

          // remove a scope
          let scopePresent =
            await pkpPermissions.isPermittedAuthMethodScopePresent(
              tokenId,
              authMethodType,
              userId,
              scopes[0]
            );
          expect(scopePresent).equal(true);
          await pkpPermissions.removePermittedAuthMethodScope(
            tokenId,
            authMethodType,
            userId,
            scopes[0]
          );
          scopePresent = await pkpPermissions.isPermittedAuthMethodScopePresent(
            tokenId,
            authMethodType,
            userId,
            scopes[0]
          );
          expect(scopePresent).equal(false);

          // add a new scope
          const newScope = 40;
          scopePresent = await pkpPermissions.isPermittedAuthMethodScopePresent(
            tokenId,
            authMethodType,
            userId,
            newScope
          );
          expect(scopePresent).equal(false);
          await pkpPermissions.addPermittedAuthMethodScope(
            tokenId,
            authMethodType,
            userId,
            newScope
          );
          scopePresent = await pkpPermissions.isPermittedAuthMethodScopePresent(
            tokenId,
            authMethodType,
            userId,
            newScope
          );
          expect(scopePresent).equal(true);

          // revoke
          await pkpPermissions.removePermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(false);

          // confirm that the reverse lookup is now empty
          pkpIds = await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodType,
            userId
          );
          expect(pkpIds.length).equal(0);
        });

        it('registers and grants permission to a generic AuthMethod using batch methods', async () => {
          const authMethodType = 5;
          const userId = '0xdead1234beefbeefdead';
          const userPubkey = '0x98765432101234';
          const scopes = [10, 20];

          pkpContract = await pkpContract.connect(tester);

          // validate that the auth method is not permitted
          let permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(false);

          // make sure the scopes aren't set
          let storedScopes = await pkpPermissions.getPermittedAuthMethodScopes(
            tokenId,
            authMethodType,
            userId,
            256
          );
          expect(storedScopes.length).equal(256);

          for (let i = 0; i < storedScopes.length; i++) {
            expect(storedScopes[i]).equal(false);
          }

          // check the scopes one by one
          for (let i = 0; i < scopes.length; i++) {
            const scopePresent =
              await pkpPermissions.isPermittedAuthMethodScopePresent(
                tokenId,
                authMethodType,
                userId,
                scopes[i]
              );
            expect(scopePresent).equal(false);
          }

          // attempt to permit it
          pkpPermissions = await pkpPermissions.connect(tester);
          await pkpPermissions.batchAddRemoveAuthMethods(
            tokenId,
            [authMethodType],
            [userId],
            [userPubkey],
            [scopes],
            [],
            []
          );

          // lookup the pubkey by the user id
          let pubkey = await pkpPermissions.getUserPubkeyForAuthMethod(
            authMethodType,
            userId
          );
          // console.log("pubkey stored in contract", pubkey);
          // console.log("userPubkey", userPubkey);
          expect(pubkey).equal(userPubkey);

          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(true);

          // do a reverse lookup
          let pkpIds = await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodType,
            userId
          );
          expect(pkpIds.length).equal(1);
          expect(pkpIds[0]).equal(tokenId);

          // check the scopes
          storedScopes = await pkpPermissions.getPermittedAuthMethodScopes(
            tokenId,
            authMethodType,
            userId,
            256
          );
          expect(storedScopes.length).equal(256);
          for (let i = 0; i < scopes.length; i++) {
            expect(storedScopes[scopes[i]]).equal(true);
          }

          // check the scopes one by one
          for (let i = 0; i < scopes.length; i++) {
            const scopePresent =
              await pkpPermissions.isPermittedAuthMethodScopePresent(
                tokenId,
                authMethodType,
                userId,
                scopes[i]
              );
            expect(scopePresent).equal(true);
          }

          // remove a scope
          let scopePresent =
            await pkpPermissions.isPermittedAuthMethodScopePresent(
              tokenId,
              authMethodType,
              userId,
              scopes[0]
            );
          expect(scopePresent).equal(true);
          await pkpPermissions.removePermittedAuthMethodScope(
            tokenId,
            authMethodType,
            userId,
            scopes[0]
          );
          scopePresent = await pkpPermissions.isPermittedAuthMethodScopePresent(
            tokenId,
            authMethodType,
            userId,
            scopes[0]
          );
          expect(scopePresent).equal(false);

          // add a new scope
          const newScope = 40;
          scopePresent = await pkpPermissions.isPermittedAuthMethodScopePresent(
            tokenId,
            authMethodType,
            userId,
            newScope
          );
          expect(scopePresent).equal(false);
          await pkpPermissions.addPermittedAuthMethodScope(
            tokenId,
            authMethodType,
            userId,
            newScope
          );
          scopePresent = await pkpPermissions.isPermittedAuthMethodScopePresent(
            tokenId,
            authMethodType,
            userId,
            newScope
          );
          expect(scopePresent).equal(true);

          // now add a new one and revoke this old one atomically

          let userId2 = '0xdead12345678beefdeadbeefdead';
          let userPubkey2 = '0x1234567890abcdefbeefbeefbeef';
          await pkpPermissions.batchAddRemoveAuthMethods(
            tokenId,
            [authMethodType],
            [userId2],
            [userPubkey2],
            [scopes],
            [authMethodType],
            [userId]
          );

          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId
          );
          expect(permitted).equal(false);

          permitted = await pkpPermissions.isPermittedAuthMethod(
            tokenId,
            authMethodType,
            userId2
          );
          expect(permitted).equal(true);

          // confirm that the reverse lookup is now empty
          pkpIds = await pkpPermissions.getTokenIdsForAuthMethod(
            authMethodType,
            userId
          );
          expect(pkpIds.length).equal(0);
        });

        it('updates root hash and verify state', async () => {
          // permit it
          pkpPermissions = await pkpPermissions.connect(tester);
          const authMethodType = 5;
          const userId1 = 'dead1234beef';
          const userId2 = '1234deadbeef';
          const userId3 = 'deadbeef1234';
          const group = 1;
          const leaves = [
            [authMethodType, Buffer.from(userId1, 'hex')],
            [authMethodType, Buffer.from(userId2, 'hex')],
            [authMethodType, Buffer.from(userId3, 'hex')],
          ];
          let tree = StandardMerkleTree.of(leaves, ['uint256', 'bytes']);
          await pkpPermissions.setRootHash(tokenId, group, tree.root);

          const proof = tree.getProof(0);
          const leafHash = tree.leafHash(leaves[0]);
          let verified = await pkpPermissions.verifyState(
            tokenId,
            group,
            proof,
            leafHash
          );
          expect(verified).equal(true);

          const multiProof = tree.getMultiProof([0, 1]);
          verified = await pkpPermissions.verifyStates(
            tokenId,
            group,
            multiProof.proof,
            multiProof.proofFlags,
            multiProof.leaves.map((l) => tree.leafHash(l))
          );
          expect(verified).equal(true);

          const updatedLeafs = [[authMethodType, Buffer.from(userId2, 'hex')]];
          tree = StandardMerkleTree.of(updatedLeafs, ['uint256', 'bytes']);
          await pkpPermissions.setRootHash(tokenId, group, tree.root);
          verified = await pkpPermissions.verifyState(
            tokenId,
            group,
            proof,
            leafHash
          );
          expect(verified).equal(false);
          verified = await pkpPermissions.verifyState(
            tokenId,
            2,
            proof,
            leafHash
          );
          expect(verified).equal(false);
        });

        it('grants permission to an eth address and then revokes it and then burns it', async () => {
          const addressToPermit = '0x75EdCdfb5A678290A8654979703bdb75C683B3dD';

          pkpContract = await pkpContract.connect(tester);

          // validate that the address is not permitted
          let permitted = await pkpPermissions.isPermittedAddress(
            tokenId,
            addressToPermit
          );
          expect(permitted).equal(false);

          // permit it
          pkpPermissions = await pkpPermissions.connect(tester);
          await pkpPermissions.addPermittedAddress(
            tokenId,
            addressToPermit,
            []
          );
          permitted = await pkpPermissions.isPermittedAddress(
            tokenId,
            addressToPermit
          );
          expect(permitted).equal(true);

          // revoke
          await pkpPermissions.removePermittedAddress(tokenId, addressToPermit);
          permitted = await pkpPermissions.isPermittedAddress(
            tokenId,
            addressToPermit
          );
          expect(permitted).equal(false);

          let exists = await pkpContract.exists(tokenId);
          expect(exists).equal(true);

          // try burning the PKP and make sure everything still works
          await pkpContract.burn(tokenId);

          let permittedAddresses =
            pkpPermissions.getPermittedAddresses(tokenId);
          expect(permittedAddresses).to.be.empty;

          permitted = await pkpPermissions.isPermittedAddress(
            tokenId,
            addressToPermit
          );
          expect(permitted).equal(false);

          exists = await pkpContract.exists(tokenId);
          expect(exists).equal(false);
        });
      }
    );
  });
});
