// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

import "hardhat/console.sol";

contract ReleaseRegister is AccessControl {
    /* ========== DEPS ========== */

    using EnumerableSet for EnumerableSet.Bytes32Set;

    /* ========== TYPE DEFINITIONS ========== */

    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN");
    bytes32 public constant CREATOR_ROLE = keccak256("CREATOR");
    bytes32 public constant ACTIVATOR_ROLE = keccak256("ACTIVATOR");
    bytes32 public constant DEACTIVATOR_ROLE = keccak256("DEACTIVATOR");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER");

    // Also update contracts/release/mod.rs
    uint public constant RELEASE_OPTION_RO = 1 << 1;
    uint public constant RELEASE_OPTION_USERS = 1 << 2;
    uint public constant RELEASE_OPTION_SSH = 1 << 3;

    enum Status {
        Null,
        Pending,
        Active,
        Disabled
    }

    enum Env {
        Dev,
        Staging,
        Prod
    }

    enum Type {
        Node,
        Prov,
        Build,
        Custom
    }

    enum Platform {
        MetalAmdSev
    }

    struct Release {
        Status status;
        Env env;
        Type typ;
        bytes kind;
        uint date;
        Platform platform;
        uint options;
        bytes id_key_digest;
        bytes public_key;
        bytes cid;
    }

    /* ========== ERRORS ========== */

    /// A release with this ID does not exist.
    error ReleaseNotFound();

    /// The env provided is not valid for this contract..
    error InvalidEnv();

    /// The status provided is not valid in this context.
    error InvalidStatus();

    /// The ADMIN role is required to use this function
    error AdminRoleRequired();

    /// The CREATOR role is required to use this function
    error CreatorRoleRequired();

    /// The ACTIVATOR role is required when setting status to Active
    error ActivatorRoleRequired();

    /// The DEACTIVATOR role is required when setting status to Disabled
    error DeactivatorRoleRequired();

    /// The BURNER role is required when burning a release
    error BurnerRoleRequired();

    /* ========== EVENTS ========== */

    event InitCreator(bytes domain, bytes authorKeyDigest);

    event AllowedEnvAdded(Env env);

    event AllowedEnvRemoved(Env env);

    event AllowedSubnetAdded(address subnet);

    event AllowedSubnetRemoved(address subnet);

    event AllowedAdminSigningPublicKeyAdded(bytes pubKey);

    event AllowedAdminSigningPublicKeyRemoved(bytes pubKey);

    event AllowedAuthorKeyDigestAdded(bytes digest);

    event AllowedAuthorKeyDigestRemoved(bytes digest);

    event ReleaseCreated(
        bytes32 releaseId,
        Status status,
        Env env,
        Type typ,
        bytes kind,
        uint date,
        Platform platform,
        uint options,
        bytes id_key_digest,
        bytes public_key,
        bytes cid
    );

    event ReleaseStatusChange(bytes32 releaseId, Status status);

    event ReleaseBurned(bytes32 releaseId);

    /* ========== STATE VARIABLES ========== */

    bool creatorInit;
    bytes creatorDomain;
    mapping(Env => bool) allowedEnvs;
    mapping(address => bool) allowedSubnets;
    mapping(bytes => bool) allowedAdminSigningPublicKeys;
    mapping(bytes => bool) allowedAuthorKeyDigests;
    mapping(bytes32 => Release) releases;
    mapping(bytes32 => bytes32) activeReleases;
    EnumerableSet.Bytes32Set activeReleaseSet;

    /* ========== CONSTRUCTOR ========== */

    constructor(Env env) {
        _setupRole(ADMIN_ROLE, msg.sender);
        _setRoleAdmin(ADMIN_ROLE, ADMIN_ROLE);
        _setRoleAdmin(CREATOR_ROLE, ADMIN_ROLE);
        _setRoleAdmin(ACTIVATOR_ROLE, ADMIN_ROLE);
        _setRoleAdmin(DEACTIVATOR_ROLE, ADMIN_ROLE);
        _setRoleAdmin(BURNER_ROLE, ADMIN_ROLE);

        allowedEnvs[env] = true;

        emit AllowedEnvAdded(env);
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    /// initialise the creator portion of the contract (provisioning).
    function initCreator(
        Env env,
        address subnetId,
        bytes memory domain,
        bytes memory authorKeyDigest
    ) public {
        // Check roles
        if (!hasRole(CREATOR_ROLE, msg.sender)) {
            revert CreatorRoleRequired();
        }

        // Ensure the env is allowed
        if (allowedEnvs[env] != true) {
            revert InvalidEnv();
        }

        if (env != Env.Dev && env != Env.Staging) {
            require(!creatorInit, "initCreator() may only be called once");
        }

        creatorInit = true;
        creatorDomain = domain;

        allowedSubnets[subnetId] = true;
        allowedAuthorKeyDigests[authorKeyDigest] = true;

        emit AllowedAuthorKeyDigestAdded(authorKeyDigest);
        emit AllowedSubnetAdded(subnetId);
        emit InitCreator(domain, authorKeyDigest);
    }

    /// add an allowed env
    function addAllowedEnv(Env env) public {
        // Check roles
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert AdminRoleRequired();
        }

        allowedEnvs[env] = true;

        emit AllowedEnvAdded(env);
    }

    /// remove an allowed env
    function removeAllowedEnv(Env env) public {
        // Check roles
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert AdminRoleRequired();
        }

        delete allowedEnvs[env];

        emit AllowedEnvRemoved(env);
    }

    /// add an allowed subnet
    function addAllowedSubnet(address subnet) public {
        // Check roles
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert AdminRoleRequired();
        }

        allowedSubnets[subnet] = true;

        emit AllowedSubnetAdded(subnet);
    }

    /// remove an allowed subnet
    function removeAllowedSubnet(address subnet) public {
        // Check roles
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert AdminRoleRequired();
        }

        delete allowedSubnets[subnet];

        emit AllowedSubnetRemoved(subnet);
    }

    /// add an admin signing public key
    function addAllowedAdminSigningPublicKey(bytes memory pubKey) public {
        // Check roles
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert AdminRoleRequired();
        }

        allowedAdminSigningPublicKeys[pubKey] = true;

        emit AllowedAdminSigningPublicKeyAdded(pubKey);
    }

    /// remove an admin signing public key
    function removeAllowedAdminSigningPublicKey(bytes memory pubKey) public {
        // Check roles
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert AdminRoleRequired();
        }

        delete allowedAdminSigningPublicKeys[pubKey];

        emit AllowedAdminSigningPublicKeyRemoved(pubKey);
    }

    /// create a release
    function updateActiveRelease(bytes32 releaseId) internal virtual {
        // Ensure it exists
        if (releases[releaseId].status == Status.Null) {
            revert ReleaseNotFound();
        }

        Release memory release = releases[releaseId];

        // Ensure it's active.
        if (release.status != Status.Active) {
            revert InvalidStatus();
        }

        bytes32 activeHashKey = keccak256(
            abi.encodePacked(
                release.env,
                release.typ,
                release.kind,
                release.platform
            )
        );
        bytes32 currentActiveId = activeReleases[activeHashKey];

        if (currentActiveId != 0) {
            if (release.date > releases[currentActiveId].date) {
                activeReleases[activeHashKey] = releaseId;
            }
        } else {
            activeReleases[activeHashKey] = releaseId;
        }

        // Add to set
        activeReleaseSet.add(releaseId);
    }

    /// create a release
    function createRelease(
        bytes32 releaseId,
        Status status,
        Env env,
        Type typ,
        bytes memory kind,
        Platform platform,
        uint options,
        bytes memory id_key_digest,
        bytes memory public_key,
        bytes memory cid,
        uint date
    ) public {
        // Check roles
        if (!hasRole(CREATOR_ROLE, msg.sender)) {
            revert CreatorRoleRequired();
        }

        if (status == Status.Active) {
            if (!hasRole(ACTIVATOR_ROLE, msg.sender)) {
                revert ActivatorRoleRequired();
            }
        } else if (status != Status.Pending && status != Status.Disabled) {
            revert InvalidStatus();
        }

        // Ensure the env is allowed
        if (allowedEnvs[env] != true) {
            revert InvalidEnv();
        }

        // Ensure the subnet is allowed
        require(
            allowedSubnets[_extractAddressFromBytes32(releaseId, 4)] == true,
            "The provided subnet (within the release id) is not valid for this contract"
        );

        if (env != Env.Dev && env != Env.Staging) {
            // Ensure does not already exist
            require(
                releases[releaseId].status == Status.Null,
                "A release with this ID already exists"
            );
        }

        // Check options
        if (env == Env.Prod) {
            require(
                (options & RELEASE_OPTION_RO) != 0,
                "The RO option is required for prod releases"
            );
        }

        // Add the new release
        if (date == 0) {
            date = block.timestamp;
        }

        releases[releaseId] = Release(
            status,
            env,
            typ,
            kind,
            date,
            platform,
            options,
            id_key_digest,
            public_key,
            cid
        );

        // Emit events
        emit ReleaseCreated(
            releaseId,
            status,
            env,
            typ,
            kind,
            date,
            platform,
            options,
            id_key_digest,
            public_key,
            cid
        );

        if (status == Status.Active) {
            updateActiveRelease(releaseId);

            emit ReleaseStatusChange(releaseId, status);
        }
    }

    /// set the release status
    function setReleaseStatus(bytes32 releaseId, Status status) public {
        // Check roles
        if (status == Status.Active) {
            if (!hasRole(ACTIVATOR_ROLE, msg.sender)) {
                revert ActivatorRoleRequired();
            }
        } else if (status == Status.Disabled) {
            if (!hasRole(DEACTIVATOR_ROLE, msg.sender)) {
                revert DeactivatorRoleRequired();
            }
        } else {
            revert InvalidStatus();
        }

        // Ensure it exists
        if (releases[releaseId].status == Status.Null) {
            revert ReleaseNotFound();
        }

        // Ensure status can be changed
        Release memory release = releases[releaseId];

        if (status != Status.Active) {
            require(
                getActiveRelease(
                    release.env,
                    release.typ,
                    release.kind,
                    release.platform
                ) != releaseId,
                "Must replace active release before changing status from Active"
            );
        }

        // Update the status
        releases[releaseId].status = status;

        // Emit events
        emit ReleaseStatusChange(releaseId, status);

        // Update active release
        if (status == Status.Active) {
            updateActiveRelease(releaseId);
        } else {
            activeReleaseSet.remove(releaseId);
        }
    }

    /// burn a release
    function burnRelease(bytes32 releaseId) public {
        // Check roles

        if (!hasRole(BURNER_ROLE, msg.sender)) {
            revert BurnerRoleRequired();
        }

        // Ensure it exists
        if (releases[releaseId].status == Status.Null) {
            revert ReleaseNotFound();
        }

        // Delete
        delete releases[releaseId];
        activeReleaseSet.remove(releaseId);

        // Emit events
        emit ReleaseBurned(releaseId);
    }

    /* ========== VIEWS ========== */

    /// Check if the creator init method has been called.
    function hasCreatorInit() external view returns (bool) {
        return (creatorInit);
    }

    /// Get the creator (provisioning) domain.
    function getCreatorDomain() external view returns (bytes memory) {
        return (creatorDomain);
    }

    /// Check if has allowed env.
    function hasAllowedEnv(Env env) external view returns (bool) {
        return (allowedEnvs[env]);
    }

    /// Check if has allowed subnet.
    function hasAllowedSubnet(address subnet) external view returns (bool) {
        return (allowedSubnets[subnet]);
    }

    /// Check if has allowed author key digest.
    function hasAllowedAuthorKeyDigest(
        bytes memory digest
    ) external view returns (bool) {
        return (allowedAuthorKeyDigests[digest]);
    }

    /// Check if has allowed admin signing public key.
    function hasAllowedAdminSigningPublicKey(
        bytes memory pubKey
    ) external view returns (bool) {
        return (allowedAdminSigningPublicKeys[pubKey]);
    }

    /// Returns the release for a given id.
    function getRelease(
        bytes32 releaseId
    ) external view returns (Release memory) {
        return releases[releaseId];
    }

    /// Returns the active release for a given criteria.
    function getActiveRelease(
        Env env,
        Type typ,
        bytes memory kind,
        Platform platform
    ) public view returns (bytes32) {
        bytes32 activeHashKey = keccak256(
            abi.encodePacked(env, typ, kind, platform)
        );

        return (activeReleases[activeHashKey]);
    }

    /// Returns the active releases.
    function getActiveReleases() public view returns (bytes32[] memory) {
        return (activeReleaseSet.values());
    }

    /* ========== UTILS ========== */

    function _extractAddressFromBytes32(
        bytes32 input,
        uint offset
    ) private pure returns (address) {
        bytes32 out;

        for (uint i = 0; i < 20; i++) {
            out |= bytes32(input[offset + i] & 0xFF) >> (i * 8);
        }

        return address(uint160(bytes20(out)));
    }
}
