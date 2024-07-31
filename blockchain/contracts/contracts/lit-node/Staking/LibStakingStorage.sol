//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import "solidity-bytes-utils/contracts/BytesLib.sol";
import { ContractResolver } from "../../lit-core/ContractResolver.sol";

interface IPubkeyRouter {
    struct RootKey {
        bytes pubkey;
        uint256 keyType; // 1 = BLS, 2 = ECDSA.  Not doing this in an enum so we can add more keytypes in the future without redeploying.
    }

    struct Signature {
        bytes32 r;
        bytes32 s;
        uint8 v;
    }
}

library LibStakingStorage {
    using EnumerableSet for EnumerableSet.AddressSet;

    bytes32 constant STAKING_POSITION = keccak256("staking.storage");

    enum States {
        Active,
        NextValidatorSetLocked,
        ReadyForNextEpoch,
        Unlocked,
        Paused,
        Restore
    }

    struct Validator {
        uint32 ip;
        uint128 ipv6;
        uint32 port;
        address nodeAddress;
        uint256 reward;
        uint256 senderPubKey;
        uint256 receiverPubKey;
    }

    struct AddressMapping {
        address nodeAddress;
        address stakerAddress;
    }

    struct VoteToKickValidatorInNextEpoch {
        uint256 votes;
        mapping(address => bool) voted;
    }

    struct Epoch {
        uint256 epochLength; // in seconds
        uint256 number; // the current epoch number
        uint256 endTime; // the end timestamp where the next epoch can be kicked off
        uint256 retries; // incremented upon failure to advance and subsequent unlock
        uint256 timeout; // timeout in seconds, where the nodes can be unlocked.
    }

    struct Config {
        uint256 tokenRewardPerTokenPerEpoch;
        /// @notice Use complaintReasonToConfig instead.
        uint256 DEPRECATED_complaintTolerance; // cycles after which to escalate peer complaints to chain
        /// @notice Use complaintReasonToConfig instead.
        uint256 DEPRECATED_complaintIntervalSecs;
        // the key type of the node.  // 1 = BLS, 2 = ECDSA.  Not doing this in an enum so we can add more keytypes in the future without redeploying.
        uint256[] keyTypes;
        // don't start the DKG or let nodes leave the validator set
        // if there are less than this many nodes
        uint256 minimumValidatorCount;
        uint256 maxConcurrentRequests;
        uint256 maxTripleCount;
        uint256 minTripleCount;
        uint256 peerCheckingIntervalSecs;
        uint256 maxTripleConcurrency;
        bool rpcHealthcheckEnabled;
    }

    struct ComplaintConfig {
        uint256 tolerance;
        uint256 intervalSecs;
        uint256 kickPenaltyPercent;
    }

    struct Version {
        uint256 major;
        uint256 minor;
        uint256 patch;
    }

    struct StakingStorage {
        ContractResolver contractResolver;
        ContractResolver.Env env;
        States state;
        EnumerableSet.AddressSet validatorsInCurrentEpoch;
        EnumerableSet.AddressSet validatorsInNextEpoch;
        EnumerableSet.AddressSet validatorsKickedFromNextEpoch;
        uint256 totalStaked;
        // versionRequirements[0] is the min version
        // versionRequirements[1] is the max version
        mapping(uint256 => Version) versionRequirements;
        // storing this in a mapping so that it can be changed in the future
        // always use epochs[0]
        mapping(uint256 => Epoch) epochs;
        // storing this in a mapping so that it can be changed in the future.
        // always use configs[0]
        mapping(uint256 => Config) configs;
        // list of all validators, even ones that are not in the current or next epoch
        // maps STAKER address to Validator struct
        mapping(address => Validator) validators;
        // stakers join by staking, but nodes need to be able to vote to kick.
        // to avoid node operators having to run a hotwallet with their staking private key,
        // the node gets it's own private key that it can use to vote to kick,
        // or signal that the next epoch is ready.
        // this mapping lets you go from the nodeAddressto the stakingAddress.
        mapping(address => address) nodeAddressToStakerAddress;
        // after the validator set is locked, nodes vote that they have successfully completed the PSS
        // operation.  Once a threshold of nodes have voted that they are ready, then the epoch can advance
        mapping(address => bool) readyForNextEpoch;
        // nodes can vote to kick another node.  If a threshold of nodes vote to kick someone, they
        // are removed from the next validator set
        mapping(uint256 => mapping(address => VoteToKickValidatorInNextEpoch)) votesToKickValidatorsInNextEpoch;
        /// Maps kick reason to amount to slash
        /// @notice Use complaintReasonToConfig instead.
        mapping(uint256 => uint256) DEPRECATED_kickPenaltyPercentByReason;
        // maps hash(comms_sender_pubkey,comms_receiver_pubkey) to a boolean to show if
        // the set of comms keys has been used or not
        mapping(bytes32 => bool) usedCommsKeys;
        // This is the set of validators that are in the current validator set that are also kicked
        // from the next validator set.
        EnumerableSet.AddressSet currentValidatorsKickedFromNextEpoch;
        // Mapping of the complaint reason code to the config for that reason
        mapping(uint256 => ComplaintConfig) complaintReasonToConfig;
    }

    // Return ERC721 storage struct for reading and writing
    function getStorage()
        internal
        pure
        returns (StakingStorage storage storageStruct)
    {
        bytes32 position = STAKING_POSITION;
        assembly {
            storageStruct.slot := position
        }
    }
}
