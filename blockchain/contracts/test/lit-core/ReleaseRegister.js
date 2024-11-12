const { expect } = require('chai');
const { getBytesFromMultihash, getBytesFromString } = require('../../utils');

// Release public key
const TEST_RELEASE_PUBLIC_KEY =
  '0x047822917c9faccf83219eafa79866e37c56d5873a5bc11b5eb8b6747e328b6800d9b51749f9e15f7c0effc8a9f899dcf17d71e1ebe1ad3d6047b215636ff9b4e1';
const TEST_RELEASE_ID_KEY_DIGEST =
  '0xcd0cd6e576bf70fe725a3327da7db4435f5f8720ebd645dcd824e9922661d6169780566cac067c15aca9802d78f7bc4c';

// Release CID
const TEST_RELEASE_CID = getBytesFromMultihash(
  'QmbWqxBEKC3P8tqsKc98xmWNzrzDtRLMiMPL8wBuTGsMnR'
);

// Enum Constants
const STATUS_NULL = 0;
const STATUS_PENDING = 1;
const STATUS_ACTIVE = 2;
const STATUS_DISABLED = 3;

const ENV_DEV = 0;
const ENV_STAGING = 1;
const ENV_PROD = 2;

const TYPE_NODE = 0;
const TYPE_PROV = 1;
const TYPE_BUILD = 2;
const TYPE_CUSTOM = 3;

const KIND_NULL = '0x';
const KIND_SALT_MASTER = getBytesFromString('salt-master');

const PLATFORM_METAL_AMD_SEV = 0;

describe('ReleaseRegister', function () {
  let deployer;
  let signers;
  let registerContract;

  before(async () => {
    [deployer, ...signers] = await ethers.getSigners();
  });

  describe('init creator', async () => {
    context('having a valid role and called the first time', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_STAGING,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
      });

      it('will succeed', async () => {
        await registerContract.initCreator(
          ENV_STAGING,
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C',
          '0x6465762e6765746c69742e7368',
          '0xd5a9b78e23b91ea9cba1623781a35de5ff9b0c0ddb3442d1a3ffa48364e625c9452ac4bfe97f5335a4748b0bf76bcc1e'
        );

        // creatorInit is disabled for now
        // expect(await registerContract.hasCreatorInit()).equals(true);
        expect(await registerContract.getCreatorDomain()).equals(
          '0x6465762e6765746c69742e7368'
        );
        expect(
          await registerContract.hasAllowedSubnet(
            '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
          )
        ).equals(true);
        expect(
          await registerContract.hasAllowedAuthorKeyDigest(
            '0xd5a9b78e23b91ea9cba1623781a35de5ff9b0c0ddb3442d1a3ffa48364e625c9452ac4bfe97f5335a4748b0bf76bcc1e'
          )
        ).equals(true);
      });
    });

    context('without a valid role', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_STAGING,
        ]);
      });

      it('will fail', async () => {
        await expect(
          registerContract.initCreator(
            ENV_STAGING,
            '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C',
            '0x6465762e6765746c69742e7368',
            '0xd5a9b78e23b91ea9cba1623781a35de5ff9b0c0ddb3442d1a3ffa48364e625c9452ac4bfe97f5335a4748b0bf76bcc1e'
          )
        ).to.be.revertedWithCustomError(
          registerContract,
          'CreatorRoleRequired()'
        );
      });
    });

    context('when called more than once', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_PROD,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
      });

      it('will fail', async () => {
        await registerContract.initCreator(
          ENV_PROD,
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C',
          '0x6465762e6765746c69742e7368',
          '0xd5a9b78e23b91ea9cba1623781a35de5ff9b0c0ddb3442d1a3ffa48364e625c9452ac4bfe97f5335a4748b0bf76bcc1e'
        );

        // creatorInit is disabled for now
        // expect(await registerContract.hasCreatorInit()).equals(true);

        // await expect(
        //   registerContract.initCreator(
        //     ENV_PROD,
        //     '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C',
        //     '0x6465762e6765746c69742e7368',
        //     '0xd5a9b78e23b91ea9cba1623781a35de5ff9b0c0ddb3442d1a3ffa48364e625c9452ac4bfe97f5335a4748b0bf76bcc1e'
        //   )
        // ).to.be.revertedWith('initCreator() may only be called once');
      });
    });
  });

  describe('create release', async () => {
    context('when no existing release', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('creates a release', async () => {
        const RELEASE_OPTION_USERS =
          await registerContract.RELEASE_OPTION_USERS();
        const RELEASE_OPTION_SSH = await registerContract.RELEASE_OPTION_SSH();

        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';
        let options = RELEASE_OPTION_USERS | RELEASE_OPTION_SSH;

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          options,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );

        let release = await registerContract.getRelease(releaseId);

        expect(release.status).equals(STATUS_PENDING);
        expect(release.env).equals(ENV_DEV);
        expect(release.typ).equals(TYPE_NODE);
        expect(release.kind).equals(KIND_NULL);
        expect(release.date).to.be.not.null;
        expect(Number(release.date)).to.be.greaterThan(0);
        expect(release.platform).equals(PLATFORM_METAL_AMD_SEV);
        expect(release.options).equals(options);
        expect((release.options & RELEASE_OPTION_USERS) !== 0).equals(true);
        expect((release.options & RELEASE_OPTION_SSH) !== 0).equals(true);
        expect(release.id_key_digest).equals(TEST_RELEASE_ID_KEY_DIGEST);
        expect(release.public_key).equals(TEST_RELEASE_PUBLIC_KEY);
        expect(release.cid).equals(TEST_RELEASE_CID);
      });

      it('creates a release with specific date', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';
        let options = await registerContract.RELEASE_OPTION_RO();

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_PROV,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          options,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          1668710525
        );

        let release = await registerContract.getRelease(releaseId);

        expect(release.status).equals(STATUS_PENDING);
        expect(release.env).equals(ENV_DEV);
        expect(release.typ).equals(TYPE_PROV);
        expect(release.kind).equals(KIND_NULL);
        expect(release.date).to.be.not.null;
        expect(Number(release.date)).to.be.equals(1668710525);
        expect(release.platform).equals(PLATFORM_METAL_AMD_SEV);
        expect(release.options).equals(options);
        expect(
          (release.options & (await registerContract.RELEASE_OPTION_RO())) !== 0
        ).equals(true);
        expect(release.id_key_digest).equals(TEST_RELEASE_ID_KEY_DIGEST);
        expect(release.public_key).equals(TEST_RELEASE_PUBLIC_KEY);
        expect(release.cid).equals(TEST_RELEASE_CID);
      });

      it('creates a custom release with a kind', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';
        let options = await registerContract.RELEASE_OPTION_RO();

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_CUSTOM,
          KIND_SALT_MASTER,
          PLATFORM_METAL_AMD_SEV,
          options,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );

        let release = await registerContract.getRelease(releaseId);

        expect(release.status).equals(STATUS_PENDING);
        expect(release.env).equals(ENV_DEV);
        expect(release.typ).equals(TYPE_CUSTOM);
        expect(release.kind).equals(KIND_SALT_MASTER);
        expect(release.date).to.be.not.null;
        expect(Number(release.date)).to.be.greaterThan(0);
        expect(release.platform).equals(PLATFORM_METAL_AMD_SEV);
        expect(release.options).equals(options);
        expect(
          (release.options & (await registerContract.RELEASE_OPTION_RO())) !== 0
        ).equals(true);
        expect(release.id_key_digest).equals(TEST_RELEASE_ID_KEY_DIGEST);
        expect(release.public_key).equals(TEST_RELEASE_PUBLIC_KEY);
        expect(release.cid).equals(TEST_RELEASE_CID);
      });
    });

    context('with an existing release', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_PROD,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';
        let options = await registerContract.RELEASE_OPTION_RO();

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_PROD,
          TYPE_BUILD,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          options,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );

        await expect(
          registerContract.createRelease(
            releaseId,
            STATUS_PENDING,
            ENV_PROD,
            TYPE_BUILD,
            KIND_NULL,
            PLATFORM_METAL_AMD_SEV,
            options,
            TEST_RELEASE_ID_KEY_DIGEST,
            TEST_RELEASE_PUBLIC_KEY,
            TEST_RELEASE_CID,
            0
          )
        ).to.be.revertedWith('A release with this ID already exists');
      });
    });

    context('with an invalid status', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.createRelease(
            releaseId,
            STATUS_NULL,
            ENV_DEV,
            TYPE_NODE,
            KIND_NULL,
            PLATFORM_METAL_AMD_SEV,
            100,
            TEST_RELEASE_ID_KEY_DIGEST,
            TEST_RELEASE_PUBLIC_KEY,
            TEST_RELEASE_CID,
            0
          )
        ).to.be.revertedWithCustomError(registerContract, 'InvalidStatus()');
      });
    });

    context('with an invalid env', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.createRelease(
            releaseId,
            STATUS_PENDING,
            ENV_STAGING,
            TYPE_BUILD,
            KIND_NULL,
            PLATFORM_METAL_AMD_SEV,
            100,
            TEST_RELEASE_ID_KEY_DIGEST,
            TEST_RELEASE_PUBLIC_KEY,
            TEST_RELEASE_CID,
            0
          )
        ).to.be.revertedWithCustomError(registerContract, 'InvalidEnv()');
      });
    });

    context('without role', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.createRelease(
            releaseId,
            STATUS_PENDING,
            ENV_DEV,
            TYPE_BUILD,
            KIND_NULL,
            PLATFORM_METAL_AMD_SEV,
            100,
            TEST_RELEASE_ID_KEY_DIGEST,
            TEST_RELEASE_PUBLIC_KEY,
            TEST_RELEASE_CID,
            0
          )
        ).to.be.revertedWithCustomError(
          registerContract,
          'CreatorRoleRequired()'
        );
      });
    });
  });

  describe('set release status', async () => {
    context('with Active status and release', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.grantRole(
          await registerContract.ACTIVATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('will set the status', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );

        await registerContract.setReleaseStatus(releaseId, STATUS_ACTIVE);
      });
    });

    context('with Disabled status and release', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.grantRole(
          await registerContract.DEACTIVATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('will set the status', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );

        await registerContract.setReleaseStatus(releaseId, STATUS_DISABLED);
      });
    });

    context("when release doesn't exist", async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.ACTIVATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.setReleaseStatus(releaseId, STATUS_ACTIVE)
        ).to.be.revertedWithCustomError(registerContract, 'ReleaseNotFound()');
      });
    });

    context('with an invalid status', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.ACTIVATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.setReleaseStatus(releaseId, STATUS_NULL)
        ).to.be.revertedWithCustomError(registerContract, 'InvalidStatus()');
      });
    });

    context('without ACTIVATOR role', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.setReleaseStatus(releaseId, STATUS_ACTIVE)
        ).to.be.revertedWithCustomError(
          registerContract,
          'ActivatorRoleRequired()'
        );
      });
    });

    context('without DEACTIVATOR role', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.setReleaseStatus(releaseId, STATUS_DISABLED)
        ).to.be.revertedWithCustomError(
          registerContract,
          'DeactivatorRoleRequired()'
        );
      });
    });
  });

  describe('burn release', async () => {
    context('with valid release', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.grantRole(
          await registerContract.BURNER_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('will burn the release', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );

        await registerContract.burnRelease(releaseId);
      });
    });

    context("when release doesn't exist", async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.BURNER_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.burnRelease(releaseId)
        ).to.be.revertedWithCustomError(registerContract, 'ReleaseNotFound()');
      });
    });

    context('without role', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);
      });

      it('produces an error', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await expect(
          registerContract.burnRelease(releaseId)
        ).to.be.revertedWithCustomError(
          registerContract,
          'BurnerRoleRequired()'
        );
      });
    });
  });

  describe('get active release', async () => {
    context('with a set of releases', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.grantRole(
          await registerContract.ACTIVATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedEnv(ENV_STAGING);
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );

        let releaseId1 =
          '0xbbc93621cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';
        let releaseId2 =
          '0xbbc93622cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';
        let releaseId3 =
          '0xbbc93623cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';
        let releaseId4 =
          '0xbbc93624cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';
        let releaseId5 =
          '0xbbc93625cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';
        let releaseId6 =
          '0xbbc93626cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';
        let releaseId7 =
          '0xbbc93627cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';
        let releaseId8 =
          '0xbbc93628cD63735A432B55B73d50c9FB140cc12e3d100d7c0000000000000000';

        await registerContract.createRelease(
          releaseId1,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
        await registerContract.createRelease(
          releaseId2,
          STATUS_ACTIVE,
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
        await registerContract.createRelease(
          releaseId3,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
        await registerContract.createRelease(
          releaseId4,
          STATUS_ACTIVE,
          ENV_DEV,
          TYPE_BUILD,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
        await registerContract.createRelease(
          releaseId5,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_BUILD,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
        await registerContract.createRelease(
          releaseId6,
          STATUS_ACTIVE,
          ENV_STAGING,
          TYPE_PROV,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
        await registerContract.createRelease(
          releaseId7,
          STATUS_PENDING,
          ENV_STAGING,
          TYPE_PROV,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
        await registerContract.createRelease(
          releaseId8,
          STATUS_ACTIVE,
          ENV_STAGING,
          TYPE_CUSTOM,
          KIND_SALT_MASTER,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
      });

      it('will find an active node release', async () => {
        let releaseId = await registerContract.getActiveRelease(
          ENV_DEV,
          TYPE_NODE,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV
        );

        expect(releaseId).equals(
          '0xbbc93622cd63735a432b55b73d50c9fb140cc12e3d100d7c0000000000000000'
        );
      });

      it('will find an active prov release', async () => {
        let releaseId = await registerContract.getActiveRelease(
          ENV_STAGING,
          TYPE_PROV,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV
        );

        expect(releaseId).equals(
          '0xbbc93626cd63735a432b55b73d50c9fb140cc12e3d100d7c0000000000000000'
        );
      });

      it('will find an active build release', async () => {
        let releaseId = await registerContract.getActiveRelease(
          ENV_DEV,
          TYPE_BUILD,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV
        );

        expect(releaseId).equals(
          '0xbbc93624cd63735a432b55b73d50c9fb140cc12e3d100d7c0000000000000000'
        );
      });

      it('will find an active custom salt-master release', async () => {
        let releaseId = await registerContract.getActiveRelease(
          ENV_STAGING,
          TYPE_CUSTOM,
          KIND_SALT_MASTER,
          PLATFORM_METAL_AMD_SEV
        );

        expect(releaseId).equals(
          '0xbbc93628cd63735a432b55b73d50c9fb140cc12e3d100d7c0000000000000000'
        );
      });

      it('will return a list of active release ids', async () => {
        let releaseIds = await registerContract.getActiveReleases();

        expect(releaseIds).to.be.not.null;
        expect(releaseIds.length).equals(4);
      });
    });
  });

  describe('add allowed env', async () => {
    context('with valid env', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('will allow release for env', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';
        let options = registerContract.RELEASE_OPTION_RO();

        await registerContract.addAllowedEnv(ENV_PROD);

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_PROD,
          TYPE_BUILD,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          options,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
      });
    });
  });

  describe('remove allowed env', async () => {
    context('with valid env', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('will prevent release for env', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await registerContract.removeAllowedEnv(ENV_DEV);

        await expect(
          registerContract.createRelease(
            releaseId,
            STATUS_PENDING,
            ENV_DEV,
            TYPE_BUILD,
            KIND_NULL,
            PLATFORM_METAL_AMD_SEV,
            100,
            TEST_RELEASE_ID_KEY_DIGEST,
            TEST_RELEASE_PUBLIC_KEY,
            TEST_RELEASE_CID,
            0
          )
        ).to.be.revertedWithCustomError(registerContract, 'InvalidEnv()');
      });
    });
  });

  describe('add allowed subnet', async () => {
    context('with valid subnet', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
      });

      it('will allow release for env', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );

        expect(
          await registerContract.hasAllowedSubnet(
            '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
          )
        ).equals(true);

        await registerContract.createRelease(
          releaseId,
          STATUS_PENDING,
          ENV_DEV,
          TYPE_BUILD,
          KIND_NULL,
          PLATFORM_METAL_AMD_SEV,
          100,
          TEST_RELEASE_ID_KEY_DIGEST,
          TEST_RELEASE_PUBLIC_KEY,
          TEST_RELEASE_CID,
          0
        );
      });
    });
  });

  describe('remove allowed subnet', async () => {
    context('with valid subnet', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);

        await registerContract.grantRole(
          await registerContract.CREATOR_ROLE(),
          deployer.address
        );
        await registerContract.addAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );
      });

      it('will prevent release for env', async () => {
        let releaseId =
          '0xbbc9362ecD63735A432B55B73d50c9FB140cc12e3d100d7C0000000000000000';

        await registerContract.removeAllowedSubnet(
          '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
        );

        expect(
          await registerContract.hasAllowedSubnet(
            '0xcD63735A432B55B73d50c9FB140cc12e3d100d7C'
          )
        ).equals(false);

        await expect(
          registerContract.createRelease(
            releaseId,
            STATUS_PENDING,
            ENV_DEV,
            TYPE_BUILD,
            KIND_NULL,
            PLATFORM_METAL_AMD_SEV,
            100,
            TEST_RELEASE_ID_KEY_DIGEST,
            TEST_RELEASE_PUBLIC_KEY,
            TEST_RELEASE_CID,
            0
          )
        ).to.be.revertedWith(
          'The provided subnet (within the release id) is not valid for this contract'
        );
      });
    });
  });

  describe('add allowed admin signing public key', async () => {
    context('with valid public key', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);
      });

      it('will add allowed public key', async () => {
        await registerContract.addAllowedAdminSigningPublicKey(
          '0x041e8c01d7ea24b6a91a4baa0abcac369cec8258881975417922f524b38fe6513f39069cb76254846902a5ccec999ca7f92feb00cd57ae4b87bf60bd1412006d5d'
        );

        let has_addr = await registerContract.hasAllowedAdminSigningPublicKey(
          '0x041e8c01d7ea24b6a91a4baa0abcac369cec8258881975417922f524b38fe6513f39069cb76254846902a5ccec999ca7f92feb00cd57ae4b87bf60bd1412006d5d'
        );

        expect(has_addr).equals(true);
      });
    });
  });

  describe('remove allowed admin signing public key', async () => {
    context('with valid public key', async () => {
      beforeEach(async () => {
        registerContract = await ethers.deployContract('ReleaseRegister', [
          ENV_DEV,
        ]);
      });

      it('will remove allowed address', async () => {
        await registerContract.addAllowedAdminSigningPublicKey(
          '0x041e8c01d7ea24b6a91a4baa0abcac369cec8258881975417922f524b38fe6513f39069cb76254846902a5ccec999ca7f92feb00cd57ae4b87bf60bd1412006d5d'
        );

        expect(
          await registerContract.hasAllowedAdminSigningPublicKey(
            '0x041e8c01d7ea24b6a91a4baa0abcac369cec8258881975417922f524b38fe6513f39069cb76254846902a5ccec999ca7f92feb00cd57ae4b87bf60bd1412006d5d'
          )
        ).equals(true);

        await registerContract.removeAllowedAdminSigningPublicKey(
          '0x041e8c01d7ea24b6a91a4baa0abcac369cec8258881975417922f524b38fe6513f39069cb76254846902a5ccec999ca7f92feb00cd57ae4b87bf60bd1412006d5d'
        );

        expect(
          await registerContract.hasAllowedAdminSigningPublicKey(
            '0x041e8c01d7ea24b6a91a4baa0abcac369cec8258881975417922f524b38fe6513f39069cb76254846902a5ccec999ca7f92feb00cd57ae4b87bf60bd1412006d5d'
          )
        ).equals(false);
      });
    });
  });
});
