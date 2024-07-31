// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ContractResolver } from "../../lit-core/ContractResolver.sol";
import { LibBackupRecoveryStorage } from "./LibBackupRecoveryStorage.sol";
import { StakingViewsFacet } from "../Staking/StakingViewsFacet.sol";
import { LibDiamond } from "../../libraries/LibDiamond.sol";
import "hardhat/console.sol";
import { EnumerableMap } from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";

contract BackupRecoveryFacet {
    address public constant BASE_EC_OP_ADDRESS =
        0x000000000000000000000000000000000000012D;
    bytes32 HASH_SHA2_256 = sha256("sha2-256");

    // errors
    error CallerNotOwner();
    error BackupStateAlreadyRegistered(bytes pubkey);
    error NodesAllMappedToBackupMembers(address addr);
    error BackupStateNotRegistered();
    error InvalidCaller(address addr);
    error BackupMemberNotMappedToNode(address peer);
    error BackupKeysMismatch(
        bytes publicKey,
        bytes senderPublicKey,
        bytes blsKey,
        bytes senderBlsKey
    );
    error BackupSetIncomplete(address[] members);
    error WrongVerificationVersion();
    error ProofExpired();

    modifier onlyOwner() {
        if (msg.sender != LibDiamond.contractOwner()) revert CallerNotOwner();
        _;
    }

    function s()
        internal
        pure
        returns (LibBackupRecoveryStorage.BackupRecoveryStorage storage)
    {
        return LibBackupRecoveryStorage.getStorage();
    }

    function _calculatePartyThreshold() public view returns (uint256) {
        if (s().nextState[0].partyMembers.length == 2) {
            return 2;
        }
        return (s().nextState[0].partyMembers.length * 2) / 3;
    }

    function _getStakingViewFacet() public view returns (StakingViewsFacet) {
        address stakingAddress = s().resolver.getContract(
            s().resolver.STAKING_CONTRACT(),
            s().env
        );
        return StakingViewsFacet(stakingAddress);
    }

    function _checkValidatorSetForAddress(
        address sender
    ) public view returns (bool) {
        address[] memory validators = _getStakingViewFacet()
            .getValidatorsInCurrentEpoch();
        address stakerAddressOfSender = _getStakingViewFacet()
            .nodeAddressToStakerAddress(sender);

        for (uint256 i = 0; i < validators.length; i++) {
            if (validators[i] == stakerAddressOfSender) {
                return true;
            }
        }

        return false;
    }

    function _checkBackupSetForSender(
        address sender
    ) internal view returns (bool) {
        for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
            if (sender == s().nextState[0].partyMembers[i]) {
                return true;
            }
        }

        return false;
    }

    function _updateRecoveryState() internal {
        s().recoveryState[0].secp256K1EcdsaPubKey = s().nextState[0].pubkey;
        s().recoveryState[0].bls12381G1EncKey = s()
            .nextState[0]
            .encryptedBlsKey;
        s().recoveryState[0].partyThreshold = _calculatePartyThreshold();
        s().recoveryState[0].sessionId = s().nextState[0].sessionId;
        s().recoveryState[0].partyMembers = s().nextState[0].partyMembers;

        _deleteNextStateMappings();
        delete s().nextState[0];
    }

    function _schnorr_verify_1(
        bytes memory input
    ) internal view returns (bool, bytes memory) {
        (bool success, bytes memory res) = BASE_EC_OP_ADDRESS.staticcall(input);

        return (success, res);
    }

    /**
     * @dev
     * Returns the currrent recovery party state
     * If not all backup party members register their state the struct will be null
     */
    function getBackupPartyState()
        public
        view
        returns (LibBackupRecoveryStorage.BackupRecoveryState memory)
    {
        return s().recoveryState[0];
    }

    /**
     * @dev
     * Get the next set of backup party members.
     * if looking for the current set the `getBackupPartyState`
     * contains the current backup party members
     */
    function getNextBackupPartyMembers()
        public
        view
        returns (address[] memory backupMembers)
    {
        return s().nextState[0].partyMembers;
    }

    /**
     * @dev
     * get the state of a backup from a previous set of backup party members
     * sessionId is used for looking up the historic state.
     * If called before the state is made the non active state it will return a null struct
     * to get the most recent state use `getBackupPartyState`
     */
    function getPastBackupState(
        bytes memory sessionId
    )
        public
        view
        returns (LibBackupRecoveryStorage.BackupRecoveryState memory partyState)
    {
        return s().pastBackupStates[sessionId];
    }

    function getNextBackupState()
        public
        view
        returns (
            LibBackupRecoveryStorage.NextStateDownloadable memory nextState
        )
    {
        return
            LibBackupRecoveryStorage.NextStateDownloadable(
                s().nextState[0].partyMembers,
                s().nextState[0].registeredRecoveryKeys
            );
    }

    /**
     * @dev
     * Returns the count of party members needed to perform recovery
     */
    function getDecryptionThreshold() public view returns (uint256) {
        return _calculatePartyThreshold();
    }

    function recieveProofsK256(
        bytes calldata proof
    ) public view returns (bool) {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        (bool ecdsaSuccess, bytes memory ecdsaRes) = _schnorr_verify_1(proof);

        return ecdsaSuccess;
    }

    /**
     * @dev
     * Verifies the encoded schnorr proof for BLS 12381G1 curve type
     * utilizes a precompile for bls schnorr verification
     * if the proof is valid we map the proofs recieved to the back up party member which submitted it
     */
    function recieveProofBls12381G1(
        bytes calldata proof
    ) public returns (bool) {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        bytes1 version = bytes1(proof[0]);
        if (version != s().verificationVersion) {
            revert WrongVerificationVersion();
        }

        // encoding format
        // Version || Timestamp || ParticipantId || Curve || SchnorrProof || SchnorrVerificationKey
        uint64 timestamp = uint64(bytes8(proof[1:9]));
        bytes32 crv = bytes32(proof[10:42]);
        bytes memory sProof = bytes(proof[42:170]);
        bytes memory sKey = bytes(proof[170:266]);
        bytes32 hName = HASH_SHA2_256;
        bytes32 y = sha256(proof[0:42]);
        bytes1 op = 0x51; // bls schnorr_verify_1

        bytes memory pcProof = abi.encodePacked(
            op,
            crv,
            hName,
            y,
            sKey,
            sProof
        );

        (bool blsSuccess, bytes memory blsRes) = _schnorr_verify_1(pcProof);

        // If the precompile status is true and the byte returned is 1 we have verification of the proof
        // If the byte retuirned is 0 then we know the proof verification has failed
        // if the success boolean is false then we know that the precompile had an error returned and was not successful
        if (blsSuccess && bytes1(blsRes) == 0x01) {
            if (s().submittedProofs[sender].length < 1) {
                s().submittedProofs[sender] = new uint256[](2);
                s().submittedProofs[sender][0] = timestamp;
            } else if (s().submittedProofs[sender].length < 3) {
                s().submittedProofs[sender][1] = timestamp;
            }

            return true;
        }

        return false;
    }

    function getProofSubmissionForBackupPartyMember()
        public
        view
        returns (uint256)
    {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );
        return s().submittedProofs[sender].length;
    }

    /**
     * @dev
     * Registers a set of bls and ecdsa keys for a given party member based on the sender address
     * Creates a backup recovery snapshot with the two respective keys
     * which is incomplete until all recovery signatures are registered to chain
     */
    function recieveNewKeySet(
        bytes memory publicKey,
        bytes memory encryptedKey,
        bytes memory sessionId
    ) public {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        // condition checks if the current set of public keys matches what is held in
        // the next state for the backup. If they do not match we set the next state
        // public keys to what is submitted by the backup party member
        // byte comparison is not supported between memory and storage
        // so we encode and hash to compare 32 byte hashes
        if (
            keccak256(abi.encodePacked(publicKey)) ==
            keccak256(abi.encodePacked(s().nextState[0].pubkey))
        ) {
            s().nextState[0].keysRecieved[msg.sender] = true;
            for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
                if (
                    s().nextState[0].keysRecieved[
                        s().nextState[0].partyMembers[i]
                    ] != true
                ) {
                    return;
                }
            }

            // make nextState backupState
            _updateRecoveryState();
            emit BackupKeysRegistered(s().recoveryState[0]);
        } else {
            // length check for null if null set the state
            // otherwise we reject the transaction as there is a lack of consensus
            if (s().nextState[0].pubkey.length < 1) {
                s().nextState[0].pubkey = publicKey;
                s().nextState[0].encryptedBlsKey = encryptedKey;
                s().nextState[0].sessionId = sessionId;
                s().nextState[0].keysRecieved[msg.sender] = true;
            } else {
                // revert as the there is a key mismatch and not the first sender
                revert BackupKeysMismatch(
                    s().nextState[0].pubkey,
                    publicKey,
                    s().nextState[0].encryptedBlsKey,
                    encryptedKey
                );
            }
        }
    }

    /**
     * @dev
     * Resets the contract state by registering a new set of backup members
     * emits `BackupPartyRegistered`
     */
    function registerNewBackupParty(
        address[] memory partyMembers
    ) public onlyOwner {
        require(
            partyMembers.length > 1,
            "BackupRecovery: cannot vote for empty party set"
        );

        // record the past state for history
        s().pastBackupStates[s().recoveryState[0].sessionId] = s()
            .recoveryState[0];

        // null the recoveryState and public key state
        delete s().recoveryState[0];

        // clear the next state to start over
        _deleteNextStateMappings();
        delete s().nextState[0];

        s().nextState[0].partyMembers = partyMembers;
        s().nextState[0].nodesAssignedCount = 0;

        emit BackupPartyRegistered(_calculatePartyThreshold());
    }

    function _deleteNextStateMappings() internal {
        address[] memory oldPartyMembers = s().nextState[0].partyMembers;
        for (uint256 i = 0; i < oldPartyMembers.length; i++) {
            address oldPartyMember = oldPartyMembers[i];
            address oldPeer = s().nextState[0].backupMemberPeerMapping[
                oldPartyMember
            ];

            delete s().nextState[0].backupMemberPeerMapping[oldPartyMember];
            delete s().nextState[0].peerToBackupMemberMapping[oldPeer];
            delete s().nextState[0].keysRecieved[oldPartyMember];
            delete s().submittedProofs[oldPartyMember];
            // No need to delete `votesToRegisterRecoveryKeys` as the pubKey will be different for all the DKGs
        }
    }

    /**
     * @dev
     * @helper
     * Returns whether all the Backup members have been mapped to a node operator
     */
    function allBackupMembersMapped() public view returns (bool mapped) {
        return
            s().nextState[0].partyMembers.length ==
            s().nextState[0].nodesAssignedCount;
    }

    /**
     * @dev
     * @helper
     * Helper function to check whether the current Recovery DKG has completed successfully
     */
    function isRecoveryDkgCompleted() public view returns (bool) {
        return s().nextState[0].registeredRecoveryKeys.length > 0;
    }

    /**
     * @dev
     * Vote to register Recovery pubKeys
     * msg.sender is the nodeAddress
     */
    function registerRecoveryKeys(
        LibBackupRecoveryStorage.RecoveryKey[] memory recoveryKeys
    ) public {
        require(
            s().nextState[0].partyMembers.length > 1,
            "BackupRecovery: cannot do dkg for empty party set"
        );

        require(
            s().nextState[0].peerToBackupMemberMapping[msg.sender] !=
                address(0),
            "BackupRecovery: not a member of the Recovery DKG peer group"
        );

        require(
            s().nextState[0].registeredRecoveryKeys.length == 0,
            "BackupRecovery: recovery keys already set for this Recovery DKG"
        );

        // record the votes
        for (uint i = 0; i < recoveryKeys.length; i++) {
            LibBackupRecoveryStorage.RecoveryKey
                memory recoveryKey = recoveryKeys[i];

            require(
                s()
                    .nextState[0]
                    .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                    .voted[msg.sender] == false,
                "BackupRecovery: validator has already voted for this recovery key"
            );

            s()
                .nextState[0]
                .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                .votes += 1;
            s()
                .nextState[0]
                .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                .voted[msg.sender] = true;

            // If all the Recovery peers have voted, register it
            if (
                s()
                    .nextState[0]
                    .votesToRegisterRecoveryKeys[recoveryKey.pubkey]
                    .votes == s().nextState[0].partyMembers.length
            ) {
                s().nextState[0].registeredRecoveryKeys.push(recoveryKey);

                emit RecoveryKeySet(recoveryKey);
            }
        }
    }

    /**
     * @dev
     * Maps a backup party member to a validator node. returns the address of the mapped validator
     * Validator refers to a node operator in this contenxt.
     * msg.sender is the nodeAddress
     */
    function setMemberForDkg() public returns (address bp) {
        require(
            _checkValidatorSetForAddress(msg.sender),
            "BackupRecovery: not a member of the current peer group"
        );
        if (allBackupMembersMapped()) {
            // Also checks whether backup party exists
            revert NodesAllMappedToBackupMembers(msg.sender);
        }
        if (
            s().nextState[0].peerToBackupMemberMapping[msg.sender] != address(0)
        ) {
            return s().nextState[0].peerToBackupMemberMapping[msg.sender];
        }

        s().nextState[0].peerToBackupMemberMapping[msg.sender] = s()
            .nextState[0]
            .partyMembers[s().nextState[0].nodesAssignedCount];
        s().nextState[0].backupMemberPeerMapping[
            s().nextState[0].partyMembers[s().nextState[0].nodesAssignedCount]
        ] = msg.sender;
        s().nextState[0].nodesAssignedCount += 1;
        emit NodeAssignedToBackupMember(
            s().nextState[0].peerToBackupMemberMapping[msg.sender],
            msg.sender
        );

        return s().nextState[0].peerToBackupMemberMapping[msg.sender];
    }

    function setContractResolver(address newResolverAddress) public onlyOwner {
        s().resolver = ContractResolver(newResolverAddress);
        emit ContractResolverAddressSet(newResolverAddress);
    }

    /**
     * @dev
     * returns the mapped backup party member to a member of the peer set
     * this peer will contain the backup party members key share from dkg
     * msg.sender is the nodeAddress
     */
    function getMemberForNodeDkg() public view returns (address bp) {
        return s().nextState[0].peerToBackupMemberMapping[msg.sender];
    }

    /**
     * @dev
     * get a peer address for a given backup party members key share.
     * the returned address will contain it's backup share
     * you can look up the wallet address in the peer state.
     */
    function getNodeForBackupMember() public view returns (address peer) {
        address sender = msg.sender;
        require(
            _checkBackupSetForSender(sender),
            "BackupRecovery: Sender is not a party member"
        );

        return s().nextState[0].backupMemberPeerMapping[msg.sender];
    }

    /**
     * @dev
     * @helper
     * returns a boolean value relating to if a node is part of the dkg set
     * msg.sender is the nodeAddress
     */
    function isNodeForDkg() public view returns (bool inSet) {
        if (
            s().nextState[0].peerToBackupMemberMapping[msg.sender] != address(0)
        ) {
            return true;
        }

        return false;
    }

    /**
     * @dev
     * @helper
     * after mapping of nodes for dkg this can be used as a helper method to get all node addresses for the dkg operation
     * these then can be used to look up the `validatorStruct` in the staking contract
     * If called before all party members have been mapped to a node for dkg the set will be incomplete.
     * Correct set size can be validated by comparing with the backup recovery party state.
     */
    function getNodeAddressesForDkg()
        public
        view
        returns (address[] memory nodes)
    {
        require(
            _checkValidatorSetForAddress(msg.sender),
            "BackupRecovery: not a member of the current peer group"
        );

        address[] memory nodeAddresses = new address[](
            s().nextState[0].partyMembers.length
        );

        for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
            nodeAddresses[i] = s().nextState[0].backupMemberPeerMapping[
                s().nextState[0].partyMembers[i]
            ];

            if (nodeAddresses[i] == address(0)) {
                revert BackupSetIncomplete(nodeAddresses);
            }
        }

        return nodeAddresses;
    }

    /**
     * @dev
     * @helper
     * after mapping of nodes for dkg this can be used as a helper method to get all node addresses for the dkg operation
     * these then can be used to look up the `validatorStruct` in the staking contract
     * If called before all party members have been mapped to a node for dkg the set will be incomplete.
     * Correct set size can be validated by comparing with the backup recovery party state.
     */
    function getStakerAddressesForDkg()
        public
        view
        returns (address[] memory nodes)
    {
        address[] memory stakerAddresses = new address[](
            s().nextState[0].partyMembers.length
        );

        for (uint256 i = 0; i < s().nextState[0].partyMembers.length; i++) {
            address nodeAddress = s().nextState[0].backupMemberPeerMapping[
                s().nextState[0].partyMembers[i]
            ];

            if (nodeAddress == address(0)) {
                revert BackupSetIncomplete(stakerAddresses);
            }

            stakerAddresses[i] = _getStakingViewFacet()
                .nodeAddressToStakerAddress(nodeAddress);
        }

        return stakerAddresses;
    }

    /* ========== EVENTS ========== */

    /**
     *
     * Emits when a new backup party is registered, after the inital
     */
    event BackupPartyRegistered(uint256 partyTheshold);

    /**
     * Emits when keys are reigstered from all parties in the backup set
     */
    event BackupKeysRegistered(
        LibBackupRecoveryStorage.BackupRecoveryState state
    );

    /**
     * Emits when a node is assigned to a backup member for dkg
     */
    event NodeAssignedToBackupMember(
        address backupMemberAddress,
        address NodeAddress
    );

    /**
     * Emits when a recovery key has been registered for the current Recovery DKG
     */
    event RecoveryKeySet(LibBackupRecoveryStorage.RecoveryKey recoveryKey);

    /**
     * Emits when the contract resolver address is set
     */
    event ContractResolverAddressSet(address newResolverAddress);
}
