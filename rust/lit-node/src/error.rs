use derive_more::Display;
use lit_core::error::*;
pub use lit_core::error::{Error, Result, Unexpected};
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};
use std::fmt::Debug;
pub const PKG_NAME: &str = "lit_node";

// constructors

#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code)]
pub(crate) enum EC {
    /// A general system fault has occurred in the node
    #[code(kind = Unexpected, http_status = 500)]
    NodeSystemFault,
    /// Lit nodes failed to check the condition possibly due to RPC servers being down or because the condition is making an incorrect smart contract call that reverts
    #[code(kind = Validation, http_status = 502)]
    NodeAccessControlConditionsCheckFailed,
    /// The access control condition check returned that you are not permitted to access this content.  Are you sure you meet the conditions?  Check the auth_sig and the other conditions
    #[code(kind = Validation, http_status = 401)]
    NodeAccessControlConditionsReturnedNotAuthorized,
    /// Failed to find the passed encrypted symmetric key
    #[code(kind = Validation, http_status = 404)]
    NodeEncryptedSymmetricKeyNotFound,
    /// While signing JWT, EXP too large or wrong
    #[code(kind = Validation, http_status = 403)]
    NodeExpWrongOrTooLarge,
    /// While signing JWT, IAT outside grace period
    #[code(kind = Validation, http_status = 403)]
    NodeIatOutsideGracePeriod,
    /// ACC passed doesn't match the ones set by the condition creator
    #[code(kind = Validation, http_status = 403)]
    NodeIncorrectAccessControlConditions,
    /// Authorization has failed
    #[code(kind = Validation, http_status = 401)]
    NodeAuthFailed,
    /// auth_sig passed is invalid or couldn't be verified
    #[code(kind = Validation, http_status = 401)]
    NodeInvalidAuthSig,
    /// auth_sig passed is not supported
    #[code(kind = Validation, http_status = 401)]
    NodeAuthSigNotSupported,
    /// auth_sig scope that is passed does not support the requested operation
    #[code(kind = Validation, http_status = 401)]
    NodeAuthSigScopeTooLimited,
    /// Auth sig is not valid against any recognized chain
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidMultipleAuthSigs,
    /// Invalid ed25519 auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidED25519AuthSig,
    /// Invalid auth_sig signing algorithm
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidAuthSigSigningAlgo,
    /// Invalid Unified auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidUnifiedAuthSig,
    /// Invalid Ethereum auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidEthereumAuthSig,
    /// Invalid Cosmos auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidCosmosAuthSig,
    /// Invalid Cosmos SDK signature
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidCosmosSDKSignature,
    /// Invalid Kyve auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidKyveAuthSig,
    /// Invalid Cheqd auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidCheqdAuthSig,
    /// Invalid Juno auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidJunoAuthSig,
    /// Invalid Solana auth_sig
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidSolanaAuthSig,
    /// auth_sig address couldn't be converted to Ethereum address
    #[code(kind = Validation, http_status = 400)]
    NodeAuthSigAddressConversionError,
    /// condition address couldn't be converted to Ethereum address
    #[code(kind = Validation, http_status = 400)]
    NodeConditionAddressConversionError,
    /// Error parsing token_id from condition
    #[code(kind = Parser, http_status = 400)]
    NodeConditionTokenIdParsingError,
    /// Invalid IPFS Id
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidIPFSID,
    /// auth_sig sig couldn't be converted to Signature object
    #[code(kind = Validation, http_status = 400)]
    NodeAuthSigSignatureConversionError,

    #[code(kind = Validation, http_status = 500)]
    NodeNoKeyGenError,

    /// auth_sig address couldn't be converted to ed25519_dalek PublicKey
    #[code(kind = Validation, http_status = 400)]
    NodeAuthSigSessionKeyConversionError,
    /// auth_sig signedMessage couldn't be converted to SessionKeySignedMessage
    #[code(kind = Validation, http_status = 400)]
    NodeAuthSigSignedMessageConversionError,
    /// auth_sig Session key signature is not valid
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidAuthSigSessionKeySignature,
    /// Invalid conditionType is Unified ACC
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidUnifiedConditionType,
    /// Invalid boolean conditionType
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidBooleanConditionType,
    /// Lit node client isn't connected to the Lit network
    #[code(kind = Config, http_status = 400)]
    NodeLitNodeClientNotReady,
    /// Must pass either access_control_conditions or evm_contract_conditions or sol_rpc_conditions
    #[code(kind = Validation, http_status = 400)]
    NodeMissingAccessControlConditions,
    /// User isn't authorized to access to decrypt or receive a JWT for this item
    #[code(kind = Validation, http_status = 403)]
    NodeNotAuthorized,
    /// Could not find the passed resource ID
    #[code(kind = Validation, http_status = 404)]
    NodeResourceIdNotFound,
    /// Lit nodes failed to complete the RPC call possibly due to RPC servers being down or because the RPC call is making an incorrect smart contract call that reverts
    #[code(kind = Validation, http_status = 502)]
    NodeRpcError,
    /// Error updating the condition possibly because the condition is permanent or the user isn't the creator of the condition
    #[code(kind = Unexpected, http_status = 403)]
    NodeStorageError,
    /// Mismatched condition & the wallet networks
    #[code(kind = Validation, http_status = 400)]
    NodeWrongNetwork,
    /// Unable to convert input chain url to HTTP transport
    #[code(kind = Conversion, http_status = 400)]
    NodeHTTPConversionError,
    /// Unknown error occured
    #[code(kind = Unknown, http_status = 400)]
    NodeUnknownError,
    /// Unable to parse session key signed message
    #[code(kind = Parser, http_status = 400)]
    NodeParserError,
    /// Invalid Capability object in SIWE resource ReCap
    #[code(kind = Validation, http_status = 400)]
    NodeSIWECapabilityInvalid,
    /// Invalid target action of Capability object in SIWE resource ReCap
    #[code(kind = Parser, http_status = 400)]
    NodeSIWECapabilityActionInvalid,
    /// Unable to convert SIWE sig to array
    #[code(kind = Validation, http_status = 400)]
    NodeSIWESigConversionError,
    /// Invalid session key signature of SIWE
    #[code(kind = Validation, http_status = 400)]
    NodeSIWESessionKeySignatureInvalid,
    /// Error related to a blockchain or its contract
    #[code(kind = Blockchain, http_status = 502)]
    NodeBlockchainError,
    /// Unknown blockchain chain for operation
    #[code(kind = Blockchain, http_status = 502)]
    NodeBlockchainChainUnknown,
    /// Wallet Signature not in JSON format
    #[code(kind = Serializer, http_status = 502)]
    NodeWalletSignatureJSONError,
    /// POAP not in JSON format
    #[code(kind = Serializer, http_status = 502)]
    NodePOAPJSONError,
    /// Cosmos Response not in JSON format
    #[code(kind = Serializer, http_status = 502)]
    NodeCosmosJSONError,
    /// Error parsing SIWE message
    #[code(kind = Parser, http_status = 400)]
    NodeSIWEMessageError,
    /// Invalid SIWE resource
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidSIWEResource,
    /// Invalid SIWE special param
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidSIWESpecialParam,
    /// Error converting SIWE special param address
    #[code(kind = Validation, http_status = 400)]
    NodeSIWESpecialParamAddressConversionError,
    /// Invalid Access Control Condition returnValueTest
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidACCReturnValueTest,
    /// Unable to parse Recovery Id
    #[code(kind = Parser, http_status = 502)]
    NodeRecoveryIdError,
    /// Unable to parse Authcontext from sessionSig
    #[code(kind = Parser, http_status = 502)]
    NodeAuthContextFromSessionSigError,
    /// Cosmos response body error
    #[code(kind = Unexpected, http_status = 502)]
    NodeCosmosResponseBodyError,
    /// Cosmos block height request error
    #[code(kind = Unexpected, http_status = 502)]
    NodeCosmosBlockHeightRequestError,
    /// Cosmos block height response body error
    #[code(kind = Unexpected, http_status = 502)]
    NodeCosmosBlockHeightParseError,
    /// Cosmos url encoding error
    #[code(kind = Unexpected, http_status = 502)]
    NodeCosmosUrlEncodingError,
    /// Cosmos invalid condition
    #[code(kind = Validation, http_status = 400)]
    NodeCosmosInvalidCondition,
    /// Mismatched number of parameters in Params & Condition
    #[code(kind = Validation, http_status = 400)]
    NodeMismatchParameters,
    /// Error tokenizing condition params
    #[code(kind = Validation, http_status = 400)]
    NodeConditionTokenizingError,
    /// Invalid condition token type
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidConditionTokenType,
    /// Token encoding/decoding error
    #[code(kind = Validation, http_status = 400)]
    NodeTokenEncodingDecodingError,
    /// Solana RPC method unsupported
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidSolanaRpcMethod,
    /// Invalid Metaplex Collection Address
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidMetaplexCollectionAddress,
    /// Failed to retrieve Metadata for Solana NFT
    #[code(kind = Unexpected, http_status = 502)]
    NodeSolanaNFTMetadataError,
    /// Failed to convert timestamp from u256 to u64
    #[code(kind = Conversion, http_status = 400)]
    NodeTimestampConversionError,
    /// Failed to convert offline PublicKey to bytes
    #[code(kind = Conversion, http_status = 502)]
    NodeOfflinePublicKeyConversionError,
    /// Can't update permanent encryption condition for key
    #[code(kind = Generic, http_status = 400)]
    NodeUpdatePermanentCondition,
    /// User doesn't match the creator for the encryption condition
    #[code(kind = Generic, http_status = 400)]
    NodeInvalidUpdatingUser,
    /// Too many conditions have been provided
    #[code(kind = Validation, http_status = 400)]
    NodeTooManyConditions,
    /// The action you attempted is not allowed
    #[code(kind = Validation, http_status = 400)]
    NodeActionNotAllowed,
    /// There was an error executing the Javascript for this action
    #[code(kind = Validation, http_status = 502)]
    NodeJsExecutionError,
    /// There was a timeout error executing the Javascript for this action
    #[code(kind = Validation, http_status = 502)]
    NodeJsTimeoutError,
    /// The memory limit was exceeded when executing the Javascript for this action
    #[code(kind = Validation, http_status = 502)]
    NodeJsMemoryLimitError,
    /// Failed to communicate with the lit_actions server
    #[code(kind = Connect, http_status = 502)]
    NodeJsConnectionError,
    /// Invalid Peer Id for Gennaro
    #[code(kind = Validation, http_status = 502)]
    NodeInvalidPeerId,
    /// Invalid Block Hash for sign session key
    #[code(kind = Validation, http_status = 502)]
    NodeInvalidBlockhash,
    /// The node has no Eeid
    #[code(kind = Unexpected, http_status = 502)]
    NodeBlsNoEeidError,
    /// Eeid and Dkg round message has mismatched epoch
    #[code(kind = Unexpected, http_status = 502)]
    NodeBlsWrongEpochError,
    /// Timeout waiting for DKG messages from other nodes
    #[code(kind = Unexpected, http_status = 500)]
    NodeDkgRoundTimeoutError,
    /// Participant not Initialized yet
    #[code(kind = Unexpected, http_status = 502)]
    NodeBlsParticipantUninitialized,
    /// Invalid smart contract function parameters
    #[code(kind = Parser, http_status = 400)]
    NodeContractFunctionParamsEncodingError,
    /// Access control failed for Smart contract
    #[code(kind = Validation, http_status = 401)]
    NodeContractAuthsigUnauthorized,
    /// Blockchain currently not supported by Lit
    #[code(kind = Blockchain, http_status = 501)]
    NodeChainNotSupported,
    /// This usually means your JSON body is wrong. Please check that you have sent over every required JSON parameter and that the types are correct according to the docs here https://lit-protocol.github.io/lit-js-sdk/api_docs_html/index.html#litnodeclient
    #[code(kind = Blockchain, http_status = 422)]
    NodeBadInput,
    /// The peer cannot be found.
    #[code(kind = Unexpected, http_status = 500)]
    NodePeerNotFound,
    /// The node could not perform an encryption / decryption operation.
    #[code(kind = Unexpected, http_status = 500)]
    NodeEncryptionError,
    /// The node could not perform a serialization / deserialization operation.
    #[code(kind = Conversion, http_status = 500)]
    NodeSerializationError,
    /// Your request as a node admin is unauthorized
    #[code(kind = Validation, http_status = 401)]
    NodeAdminUnauthorized,
    /// Could not find PKP token ID.
    #[code(kind = Validation, http_status = 400)]
    NodePKPTokenIdNotFound,
    // Invalid challenge is used for WebAuthn.
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidWebAuthnChallenge,

    // Requested a key type that is not supported by the node.
    #[code(kind = Validation, http_status = 401)]
    NodePKPKeyTypeRequestNotSupported,
    // Failed to perform the conversion from Lit Config to Contract Resolver
    #[code(kind = Unexpected, http_status = 500)]
    NodeContractResolverConversionFailed,
    // Failed to complete DKG round
    #[code(kind = Unexpected, http_status = 500)]
    NodeDkgRoundFailed,
    // Failed to complete DKG round
    #[code(kind = Unexpected, http_status = 500)]
    NodeDKGInvalidValue,
    // Lit Actions feature flag is not enabled
    #[code(kind = Unexpected, http_status = 400)]
    NodeLitActionsNotEnabled,
    // Lit Actions returned false for sessionSig signing authentication
    #[code(kind = Validation, http_status = 401)]
    NodeLitActionsSessionSigAuthenticationFailed,
    // The PKP permissions check failed
    #[code(kind = Validation, http_status = 401)]
    NodePKPNotAuthorized,
    // The PKP isn't the signer for the SIWE message
    #[code(kind = Validation, http_status = 401)]
    NodeInvalidPKPAddress,
    // The user sent an unsupported curve type when requesting to sign something
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidCurveType,
    // The user sent an auth_sig for signing a session_key
    #[code(kind = Validation, http_status = 400)]
    NodeInvalidAuthSigForSessionKey,
    // The user sent a auth_methods for endpoint
    #[code(kind = Validation, http_status = 400)]
    NodeCannotProvideAuthMethodForEndpoint,
    // The network root BLS key was not found
    #[code(kind = Unexpected, http_status = 500)]
    NodeBLSRootKeyNotFound,
    // Concurrency limit reached
    #[code(kind = Unexpected, http_status = 429)]
    NodeConcurrencyOverload,
    // Invalid Signature Requested
    #[code(kind = Validation, http_status = 401)]
    NodeSignatureNotSupported,
    // Invalid Signature Requested
    #[code(kind = Validation, http_status = 400)]
    NodeCannotProvideAuthSigForEndpoint,
    // Can't define AuthContext resources in user capability
    #[code(kind = Validation, http_status = 401)]
    NodeInvalidAuthContextResource,
    // Siwe message doesn't contain expiration time
    #[code(kind = Validation, http_status = 401)]
    NodeUndefinedSiweExpiration,
}

generate_pkg_constructors!(PKG_NAME, pub(crate), EC);
