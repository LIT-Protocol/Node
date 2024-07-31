//SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.17;
import { ContractResolver } from "../lit-core/ContractResolver.sol";
import { IPubkeyRouter } from "./PubkeyRouter/LibPubkeyRouterStorage.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

abstract contract IKeyDeriver {
    function computeHDPubKey(
        bytes32 derivedKeyId,
        IPubkeyRouter.RootKey[] memory rootHDKeys,
        uint256 keyType
    ) public view virtual returns (bool, bytes memory);
}

contract KeyDeriver is IKeyDeriver {
    // address for HD public KDF
    address public constant HD_KDF = 0x00000000000000000000000000000000000000F5;
    // hd kdf ctx
    string constant HD_KDF_CTX = "LIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_";

    constructor() {}

    function computeHDPubKey(
        bytes32 derivedKeyId,
        IPubkeyRouter.RootKey[] memory rootHDKeys,
        uint256 keyType
    ) public view override returns (bool, bytes memory) {
        bytes memory args = _buildArgs(derivedKeyId, rootHDKeys, keyType);
        (bool success, bytes memory data) = HD_KDF.staticcall(args);
        return (success, data);
    }

    function _buildArgs(
        bytes32 derivedKeyId,
        IPubkeyRouter.RootKey[] memory rootHDKeys,
        uint256 keyType
    ) internal pure returns (bytes memory) {
        // empty array for concating pubkeys
        bytes memory rootPubkeys = new bytes(0);
        uint32 numRootPubkeys = 0;
        for (uint256 i = 0; i < rootHDKeys.length; i++) {
            if (rootHDKeys[i].keyType == keyType) {
                rootPubkeys = abi.encodePacked(
                    rootPubkeys,
                    rootHDKeys[i].pubkey
                );
                numRootPubkeys++;
            }
        }

        // so in Lit land, we use 2 for ECDSA secp256k1 keyType.
        // but the precompile expects 1 for secp256k1
        // someday, we may add keyType 3 for ECDSA p256, which the precompile expects as "0"
        if (keyType == 2) {
            keyType = 1;
            // assuming p256 curve type will be value '3'
        } else if (keyType == 3) {
            keyType = 0;
        }

        bytes memory CTX = bytes(HD_KDF_CTX);
        bytes1 kt = bytes1(uint8(keyType));
        bytes4 id_len = bytes4(uint32(derivedKeyId.length));
        bytes4 ctx_len = bytes4(uint32(CTX.length));
        bytes4 pubkey_len = bytes4(numRootPubkeys);

        bytes memory args_bytes = abi.encodePacked(
            kt, // 1st arg is a byte for the curve type, 0 is Nist Prime256, 1 is secp256k1
            id_len, // 2nd arg is a 4 byte big-endian integer for the number of bytes in id
            derivedKeyId, // 3rd arg is the byte sequence for id
            ctx_len, // 4th arg is a 4 byte big-endian integer for the number of bytes in cxt
            CTX, // 5th arg is the byte sequence for cxt
            pubkey_len, // 6th arg is a 4 byte big-endian integer for the number of root keys
            rootPubkeys // 7th arg is a variable number of root keys each 33 bytes (compressed) or 65 bytes (uncompressed) in length
        );

        return args_bytes;
    }
}

// Define the interface for interacting with the Arbitrum Stylus HD KDF Contract Entrypoint
interface IHDKeyDeriver {
    function hdKeyDerive(
        bytes memory data
    ) external view returns (bytes memory);
}

contract ArbitrumKeyDeriver is IKeyDeriver, AccessControl {
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN"); // 0xdf8b4c520ffe197c5343c6f5aec59570151ef9a492f2c624fd45ddde6135ec42

    // hd kdf ctx
    string constant HD_KDF_CTX = "LIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_";

    // typs for use in the ContractResolver
    bytes32 public constant HD_KDF_P256 =
        keccak256("HD_KEY_DERIVER_CONTRACT_P256");
    bytes32 public constant HD_KDF_K256 =
        keccak256("HD_KEY_DERIVER_CONTRACT_K256");

    ContractResolver public contractResolver;
    ContractResolver.Env public env;

    constructor(address _resolver, ContractResolver.Env _env) {
        _setupRole(ADMIN_ROLE, msg.sender);
        _setRoleAdmin(ADMIN_ROLE, ADMIN_ROLE);

        contractResolver = ContractResolver(_resolver);
        env = _env;
    }

    function setContractResolver(
        address contractResolverAddress
    ) public onlyRole(ADMIN_ROLE) {
        contractResolver = ContractResolver(contractResolverAddress);
    }

    function computeHDPubKey(
        bytes32 derivedKeyId,
        IPubkeyRouter.RootKey[] memory rootHDKeys,
        uint256 keyType
    ) public view override returns (bool, bytes memory) {
        bytes memory args = _buildArgs(derivedKeyId, rootHDKeys, keyType);

        // Get the corresponding Stylus contract address depending on the curve type.
        bytes32 typToLookup;
        if (args[0] == 0) {
            typToLookup = HD_KDF_P256;
        } else {
            typToLookup = HD_KDF_K256;
        }

        IHDKeyDeriver HD_KDF = IHDKeyDeriver(
            contractResolver.getContract(typToLookup, env)
        );

        bytes memory data = HD_KDF.hdKeyDerive(args);
        return (true, data);
    }

    function _buildArgs(
        bytes32 derivedKeyId,
        IPubkeyRouter.RootKey[] memory rootHDKeys,
        uint256 keyType
    ) internal pure returns (bytes memory) {
        // empty array for concating pubkeys
        bytes memory rootPubkeys = new bytes(0);
        uint32 numRootPubkeys = 0;
        for (uint256 i = 0; i < rootHDKeys.length; i++) {
            if (rootHDKeys[i].keyType == keyType) {
                rootPubkeys = abi.encodePacked(
                    rootPubkeys,
                    rootHDKeys[i].pubkey
                );
                numRootPubkeys++;
            }
        }

        // so in Lit land, we use 2 for ECDSA secp256k1 keyType.
        // but the precompile expects 1 for secp256k1
        // someday, we may add keyType 3 for ECDSA p256, which the precompile expects as "0"
        if (keyType == 2) {
            keyType = 1;
            // assuming p256 curve type will be value '3'
        } else if (keyType == 3) {
            keyType = 0;
        }

        bytes memory CTX = bytes(HD_KDF_CTX);
        bytes1 kt = bytes1(uint8(keyType));
        bytes4 id_len = bytes4(uint32(derivedKeyId.length));
        bytes4 ctx_len = bytes4(uint32(CTX.length));
        bytes4 pubkey_len = bytes4(numRootPubkeys);

        bytes memory args_bytes = abi.encodePacked(
            kt, // 1st arg is a byte for the curve type, 0 is Nist Prime256, 1 is secp256k1
            id_len, // 2nd arg is a 4 byte big-endian integer for the number of bytes in id
            derivedKeyId, // 3rd arg is the byte sequence for id
            ctx_len, // 4th arg is a 4 byte big-endian integer for the number of bytes in cxt
            CTX, // 5th arg is the byte sequence for cxt
            pubkey_len, // 6th arg is a 4 byte big-endian integer for the number of root keys
            rootPubkeys // 7th arg is a variable number of root keys each 33 bytes (compressed) or 65 bytes (uncompressed) in length
        );

        return args_bytes;
    }
}
