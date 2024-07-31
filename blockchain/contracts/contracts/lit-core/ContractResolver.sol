// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/AccessControl.sol";

import "hardhat/console.sol";

contract ContractResolver is AccessControl {
    /* ========== TYPE DEFINITIONS ========== */

    // the comments following each one of these are the keccak256 hashes of the string values
    // this is very useful if you have to manually set any of these, so that you
    // don't have to calculate the hahes yourself.

    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN"); // 0xdf8b4c520ffe197c5343c6f5aec59570151ef9a492f2c624fd45ddde6135ec42

    bytes32 public constant RELEASE_REGISTER_CONTRACT =
        keccak256("RELEASE_REGISTER"); // 0x3a68dbfd8bbb64015c42bc131c388dea7965e28c1004d09b39f59500c3a763ec
    bytes32 public constant STAKING_CONTRACT = keccak256("STAKING"); // 0x080909c18c958ce5a2d36481697824e477319323d03154ceba3b78f28a61887b
    bytes32 public constant STAKING_BALANCES_CONTRACT =
        keccak256("STAKING_BALANCES"); // 0xaa06d108dbd7bf976b16b7bf5adb29d2d0ef2c385ca8b9d833cc802f33942d72
    bytes32 public constant MULTI_SENDER_CONTRACT = keccak256("MULTI_SENDER"); // 0xdd5b9b8a5e8e01f2962ed7e983d58fe32e1f66aa88dd7ab30770fa9b77da7243
    bytes32 public constant LIT_TOKEN_CONTRACT = keccak256("LIT_TOKEN");
    bytes32 public constant PUB_KEY_ROUTER_CONTRACT =
        keccak256("PUB_KEY_ROUTER"); // 0xb1f79813bc7630a52ae948bc99781397e409d0dd3521953bf7d8d7a2db6147f7
    bytes32 public constant PKP_NFT_CONTRACT = keccak256("PKP_NFT"); // 0xb7b4fde9944d3c13e9a78835431c33a5084d90a7f0c73def76d7886315fe87b0
    bytes32 public constant RATE_LIMIT_NFT_CONTRACT =
        keccak256("RATE_LIMIT_NFT"); // 0xb931b2719aeb2a65a5035fa0a190bfdc4c8622ce8cbff7a3d1ab42531fb1a918
    bytes32 public constant PKP_HELPER_CONTRACT = keccak256("PKP_HELPER"); // 0x27d764ea2a4a3865434bbf4a391110149644be31448f3479fd15b44388755765
    bytes32 public constant PKP_HELPER_V2_CONTRACT = keccak256("PKP_HELPER_V2"); // 0x58a0044e0ecd81025e398bf1815075d1234cbac3749614b0b33a404c2ee2babf
    bytes32 public constant PKP_PERMISSIONS_CONTRACT =
        keccak256("PKP_PERMISSIONS"); // 0x54953c23068b8fc4c0736301b50f10027d6b469327de1fd42841a5072b1bcebe
    bytes32 public constant PKP_NFT_METADATA_CONTRACT =
        keccak256("PKP_NFT_METADATA"); // 0xf14f431dadc82e7dbc5e379f71234e5735c9187e4327a7c6ac014d55d1b7727a
    bytes32 public constant ALLOWLIST_CONTRACT = keccak256("ALLOWLIST"); // 0x74845de37cfabd357633214b47fa91ccd19b05b7c5a08ac22c187f811fb62bca
    bytes32 public constant DOMAIN_WALLET_REGISTRY =
        keccak256("DOMAIN_WALLET_REGISTRY");
    bytes32 public constant HD_KEY_DERIVER_CONTRACT =
        keccak256("HD_KEY_DERIVER");
    bytes32 public constant BACKUP_RECOVERY_CONTRACT =
        keccak256("BACKUP_RECOVERY");
    bytes32 public constant PAYMENT_DELEGATION_CONTRACT =
        keccak256("PAYMENT_DELEGATION"); // 0xc6674f98ba35c01c130e08195dd26c70466037473a068c5aaa470a783d99c16c

    enum Env {
        Dev,
        Staging,
        Prod
    }

    /* ========== ERRORS ========== */

    /// The ADMIN role is required to use this function
    error AdminRoleRequired();

    /* ========== EVENTS ========== */

    event AllowedEnvAdded(Env env);

    event AllowedEnvRemoved(Env env);

    event SetContract(bytes32 typ, Env env, address addr);

    /* ========== STATE VARIABLES ========== */

    mapping(Env => bool) allowedEnvs;
    mapping(bytes32 => mapping(Env => address)) public typeAddresses;

    /* ========== CONSTRUCTOR ========== */

    constructor(Env env) {
        _setupRole(ADMIN_ROLE, msg.sender);
        _setRoleAdmin(ADMIN_ROLE, ADMIN_ROLE);

        allowedEnvs[env] = true;

        emit AllowedEnvAdded(env);
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

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

    /// set the active address for a deployed contract
    function setContract(bytes32 typ, Env env, address addr) public {
        // Check roles
        if (!hasRole(ADMIN_ROLE, msg.sender)) {
            revert AdminRoleRequired();
        }

        // Ensure the env is available
        require(
            allowedEnvs[env] == true,
            "The provided Env is not valid for this contract"
        );

        // Set the contract address
        typeAddresses[typ][env] = addr;

        // Emit events
        emit SetContract(typ, env, addr);
    }

    function addAdmin(address newAdmin) public onlyRole(ADMIN_ROLE) {
        _grantRole(ADMIN_ROLE, newAdmin);
    }

    function removeAdmin(
        address adminBeingRemoved
    ) public onlyRole(ADMIN_ROLE) {
        require(
            adminBeingRemoved != msg.sender,
            "Cannot remove self as admin.  Have the new admin do it."
        );
        _revokeRole(ADMIN_ROLE, adminBeingRemoved);
    }

    /* ========== VIEWS ========== */

    /// Returns the matching contract address for a given type and env
    function getContract(bytes32 typ, Env env) public view returns (address) {
        return (typeAddresses[typ][env]);
    }
}
