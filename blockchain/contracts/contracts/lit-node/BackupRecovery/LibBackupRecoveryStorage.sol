// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { IPubkeyRouter } from "../../lit-node/PubkeyRouter/LibPubkeyRouterStorage.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

library LibBackupRecoveryStorage {
    using EnumerableSet for EnumerableSet.AddressSet;
    bytes32 constant BACKUP_RECOVERY_POSITION =
        keccak256("backuprecovery.storage");

    /**
     * @dev
     * Simply maps a backup party member to the node that generated their backup share
     */
    struct RecoveryPair {
        address backupPartyAddress;
        address nodeAddress;
        bool isRegistered;
    }

    /**
     * @dev
     *  Holds the current backup party state
     */
    struct BackupRecoveryState {
        bytes sessionId;
        bytes bls12381G1EncKey;
        bytes secp256K1EcdsaPubKey;
        uint256 partyThreshold;
        address[] partyMembers;
    }

    struct RecoveryKey {
        bytes pubkey;
        uint256 keyType; // 1 = BLS, 2 = ECDSA.  Not doing this in an enum so we can add more keytypes in the future without redeploying.
    }

    /**
     * @dev
     * Holds the votes per Recovery pubKey
     */
    struct VotesToRegisterRecoveryKey {
        uint256 votes;
        mapping(address => bool) voted;
    }

    /**
     * @dev
     * Holds the next state of the backup party
     */
    struct NextState {
        address[] partyMembers;
        mapping(address => address) peerToBackupMemberMapping;
        uint nodesAssignedCount;
        mapping(address => address) backupMemberPeerMapping;
        mapping(address => bool) keysRecieved;
        mapping(address => K256Proof) proofsRecieved;
        // pubkey -> votes
        mapping(bytes => VotesToRegisterRecoveryKey) votesToRegisterRecoveryKeys;
        RecoveryKey[] registeredRecoveryKeys;
        bytes sessionId;
        bytes encryptedBlsKey;
        uint256 partyThreshold;
        bytes pubkey;
        bytes curve;
    }

    struct K256Proof {
        bytes1 version;
        uint256 timestamp;
        bytes1 particpantId;
        bytes schnorrProof;
        bytes schnorrVerification;
        bytes ecdsaProof;
        bytes ecdsaVerificationKey;
    }

    struct NextStateDownloadable {
        address[] partyMembers;
        RecoveryKey[] registeredRecoveryKeys;
    }

    struct BackupRecoveryStorage {
        bytes1 verificationVersion;
        // current state of the backup recovery
        mapping(uint256 => BackupRecoveryState) recoveryState;
        // holds the next party state until promoted to active
        mapping(uint256 => NextState) nextState;
        // holds past states after a new backup party is registered
        mapping(bytes => BackupRecoveryState) pastBackupStates;
        // proof submission mapping
        mapping(address => uint256[]) submittedProofs;
        // instance of the deployed contract resolver
        ContractResolver resolver;
        // env context
        ContractResolver.Env env;
        // Status of the recovering nodes
        mapping(uint256 => NodeRecoveryStatusMap[]) nodeStatusMap;
    }

    function getStorage()
        internal
        pure
        returns (BackupRecoveryStorage storage storageStruct)
    {
        bytes32 position = BACKUP_RECOVERY_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }

    enum NodeRecoveryStatus {
        Null,
        StartedInRestoreState,
        BackupsAreLoaded,
        AllKeysAreRestored,
        AbandonedRecoveryDueToNetworkState
    }

    struct NodeRecoveryStatusMap {
        address node_address;
        NodeRecoveryStatus status;
    }
}
