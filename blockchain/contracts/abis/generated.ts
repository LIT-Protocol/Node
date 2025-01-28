//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// AccessControl
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const accessControlAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'previousAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
      {
        name: 'newAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
    ],
    name: 'RoleAdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleGranted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleRevoked',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DEFAULT_ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'role', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRoleAdmin',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'grantRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'hasRole',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'renounceRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'revokeRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Allowlist
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const allowlistAbi = [
  { type: 'constructor', inputs: [], stateMutability: 'nonpayable' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newAdmin',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'AdminAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newAdmin',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'AdminRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'bytes32', type: 'bytes32', indexed: true },
    ],
    name: 'ItemAllowed',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'key', internalType: 'bytes32', type: 'bytes32', indexed: true },
    ],
    name: 'ItemNotAllowed',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [{ name: 'newAdmin', internalType: 'address', type: 'address' }],
    name: 'addAdmin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'allowAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'allowedItems',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'key', internalType: 'bytes32', type: 'bytes32' }],
    name: 'isAllowed',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'newAdmin', internalType: 'address', type: 'address' }],
    name: 'removeAdmin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_allowAll', internalType: 'bool', type: 'bool' }],
    name: 'setAllowAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'key', internalType: 'bytes32', type: 'bytes32' }],
    name: 'setAllowed',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'key', internalType: 'bytes32', type: 'bytes32' }],
    name: 'setNotAllowed',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ArbitrumKeyDeriver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const arbitrumKeyDeriverAbi = [
  {
    type: 'constructor',
    inputs: [
      { name: '_resolver', internalType: 'address', type: 'address' },
      {
        name: '_env',
        internalType: 'enum ContractResolver.Env',
        type: 'uint8',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'previousAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
      {
        name: 'newAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
    ],
    name: 'RoleAdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleGranted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleRevoked',
  },
  {
    type: 'function',
    inputs: [],
    name: 'ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DEFAULT_ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'HD_KDF_K256',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'HD_KDF_P256',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'rootHDKeys',
        internalType: 'struct IPubkeyRouter.RootKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'computeHDPubKey',
    outputs: [
      { name: '', internalType: 'bool', type: 'bool' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [
      { name: '', internalType: 'contract ContractResolver', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'env',
    outputs: [
      { name: '', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'role', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRoleAdmin',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'grantRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'hasRole',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'renounceRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'revokeRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'contractResolverAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// BackupRecovery
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const backupRecoveryAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct BackupRecoveryArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'payable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// BackupRecoveryDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const backupRecoveryDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'error',
    inputs: [
      { name: 'publicKey', internalType: 'bytes', type: 'bytes' },
      { name: 'senderPublicKey', internalType: 'bytes', type: 'bytes' },
      { name: 'blsKey', internalType: 'bytes', type: 'bytes' },
      { name: 'senderBlsKey', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'BackupKeysMismatch',
  },
  {
    type: 'error',
    inputs: [{ name: 'peer', internalType: 'address', type: 'address' }],
    name: 'BackupMemberNotMappedToNode',
  },
  {
    type: 'error',
    inputs: [{ name: 'members', internalType: 'address[]', type: 'address[]' }],
    name: 'BackupSetIncomplete',
  },
  {
    type: 'error',
    inputs: [{ name: 'pubkey', internalType: 'bytes', type: 'bytes' }],
    name: 'BackupStateAlreadyRegistered',
  },
  { type: 'error', inputs: [], name: 'BackupStateNotRegistered' },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [{ name: 'addr', internalType: 'address', type: 'address' }],
    name: 'InvalidCaller',
  },
  {
    type: 'error',
    inputs: [{ name: 'addr', internalType: 'address', type: 'address' }],
    name: 'NodesAllMappedToBackupMembers',
  },
  { type: 'error', inputs: [], name: 'ProofExpired' },
  { type: 'error', inputs: [], name: 'WrongVerificationVersion' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'state',
        internalType: 'struct LibBackupRecoveryStorage.BackupRecoveryState',
        type: 'tuple',
        components: [
          { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
          { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
          {
            name: 'secp256K1EcdsaPubKey',
            internalType: 'bytes',
            type: 'bytes',
          },
          { name: 'partyThreshold', internalType: 'uint256', type: 'uint256' },
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
        ],
        indexed: false,
      },
    ],
    name: 'BackupKeysRegistered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'partyTheshold',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'BackupPartyRegistered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'backupMemberAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'NodeAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'NodeAssignedToBackupMember',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'recoveryKey',
        internalType: 'struct LibBackupRecoveryStorage.RecoveryKey',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'RecoveryKeySet',
  },
  {
    type: 'function',
    inputs: [],
    name: 'BASE_EC_OP_ADDRESS',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: '_calculatePartyThreshold',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: '_checkValidatorSetForAddress',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: '_getStakingViewFacet',
    outputs: [
      { name: '', internalType: 'contract StakingViewsFacet', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'allBackupMembersMapped',
    outputs: [{ name: 'mapped', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getBackupPartyState',
    outputs: [
      {
        name: '',
        internalType: 'struct LibBackupRecoveryStorage.BackupRecoveryState',
        type: 'tuple',
        components: [
          { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
          { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
          {
            name: 'secp256K1EcdsaPubKey',
            internalType: 'bytes',
            type: 'bytes',
          },
          { name: 'partyThreshold', internalType: 'uint256', type: 'uint256' },
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDecryptionThreshold',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMemberForNodeDkg',
    outputs: [{ name: 'bp', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNextBackupPartyMembers',
    outputs: [
      { name: 'backupMembers', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNextBackupState',
    outputs: [
      {
        name: 'nextState',
        internalType: 'struct LibBackupRecoveryStorage.NextStateDownloadable',
        type: 'tuple',
        components: [
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
          {
            name: 'registeredRecoveryKeys',
            internalType: 'struct LibBackupRecoveryStorage.RecoveryKey[]',
            type: 'tuple[]',
            components: [
              { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
              { name: 'keyType', internalType: 'uint256', type: 'uint256' },
            ],
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNodeAddressesForDkg',
    outputs: [{ name: 'nodes', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNodeForBackupMember',
    outputs: [{ name: 'peer', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'sessionId', internalType: 'bytes', type: 'bytes' }],
    name: 'getPastBackupState',
    outputs: [
      {
        name: 'partyState',
        internalType: 'struct LibBackupRecoveryStorage.BackupRecoveryState',
        type: 'tuple',
        components: [
          { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
          { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
          {
            name: 'secp256K1EcdsaPubKey',
            internalType: 'bytes',
            type: 'bytes',
          },
          { name: 'partyThreshold', internalType: 'uint256', type: 'uint256' },
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getProofSubmissionForBackupPartyMember',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakerAddressesForDkg',
    outputs: [{ name: 'nodes', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isNodeForDkg',
    outputs: [{ name: 'inSet', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isRecoveryDkgCompleted',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'publicKey', internalType: 'bytes', type: 'bytes' },
      { name: 'encryptedKey', internalType: 'bytes', type: 'bytes' },
      { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'recieveNewKeySet',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'proof', internalType: 'bytes', type: 'bytes' }],
    name: 'recieveProofBls12381G1',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'proof', internalType: 'bytes', type: 'bytes' }],
    name: 'recieveProofsK256',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'partyMembers', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'registerNewBackupParty',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'recoveryKeys',
        internalType: 'struct LibBackupRecoveryStorage.RecoveryKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'registerRecoveryKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'setMemberForDkg',
    outputs: [{ name: 'bp', internalType: 'address', type: 'address' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'CURRENT',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'clearNodeRecoveryStatus',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNodeRecoveryStatus',
    outputs: [
      {
        name: '',
        internalType: 'struct LibBackupRecoveryStorage.NodeRecoveryStatusMap[]',
        type: 'tuple[]',
        components: [
          { name: 'node_address', internalType: 'address', type: 'address' },
          {
            name: 'status',
            internalType: 'enum LibBackupRecoveryStorage.NodeRecoveryStatus',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'status',
        internalType: 'enum LibBackupRecoveryStorage.NodeRecoveryStatus',
        type: 'uint8',
      },
    ],
    name: 'setNodeRecoveryStatus',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
      { name: 'secp256K1EcdsaPubKey', internalType: 'bytes', type: 'bytes' },
      { name: 'partyMembers', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'setBackupPartyState',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNonSubmitingBackupMembersInNextState',
    outputs: [
      {
        name: 'missingRecoveryMembers',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// BackupRecoveryFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const backupRecoveryFacetAbi = [
  {
    type: 'error',
    inputs: [
      { name: 'publicKey', internalType: 'bytes', type: 'bytes' },
      { name: 'senderPublicKey', internalType: 'bytes', type: 'bytes' },
      { name: 'blsKey', internalType: 'bytes', type: 'bytes' },
      { name: 'senderBlsKey', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'BackupKeysMismatch',
  },
  {
    type: 'error',
    inputs: [{ name: 'peer', internalType: 'address', type: 'address' }],
    name: 'BackupMemberNotMappedToNode',
  },
  {
    type: 'error',
    inputs: [{ name: 'members', internalType: 'address[]', type: 'address[]' }],
    name: 'BackupSetIncomplete',
  },
  {
    type: 'error',
    inputs: [{ name: 'pubkey', internalType: 'bytes', type: 'bytes' }],
    name: 'BackupStateAlreadyRegistered',
  },
  { type: 'error', inputs: [], name: 'BackupStateNotRegistered' },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [{ name: 'addr', internalType: 'address', type: 'address' }],
    name: 'InvalidCaller',
  },
  {
    type: 'error',
    inputs: [{ name: 'addr', internalType: 'address', type: 'address' }],
    name: 'NodesAllMappedToBackupMembers',
  },
  { type: 'error', inputs: [], name: 'ProofExpired' },
  { type: 'error', inputs: [], name: 'WrongVerificationVersion' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'state',
        internalType: 'struct LibBackupRecoveryStorage.BackupRecoveryState',
        type: 'tuple',
        components: [
          { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
          { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
          {
            name: 'secp256K1EcdsaPubKey',
            internalType: 'bytes',
            type: 'bytes',
          },
          { name: 'partyThreshold', internalType: 'uint256', type: 'uint256' },
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
        ],
        indexed: false,
      },
    ],
    name: 'BackupKeysRegistered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'partyTheshold',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'BackupPartyRegistered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'backupMemberAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'NodeAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'NodeAssignedToBackupMember',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'recoveryKey',
        internalType: 'struct LibBackupRecoveryStorage.RecoveryKey',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'RecoveryKeySet',
  },
  {
    type: 'function',
    inputs: [],
    name: 'BASE_EC_OP_ADDRESS',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: '_calculatePartyThreshold',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: '_checkValidatorSetForAddress',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: '_getStakingViewFacet',
    outputs: [
      { name: '', internalType: 'contract StakingViewsFacet', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'allBackupMembersMapped',
    outputs: [{ name: 'mapped', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getBackupPartyState',
    outputs: [
      {
        name: '',
        internalType: 'struct LibBackupRecoveryStorage.BackupRecoveryState',
        type: 'tuple',
        components: [
          { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
          { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
          {
            name: 'secp256K1EcdsaPubKey',
            internalType: 'bytes',
            type: 'bytes',
          },
          { name: 'partyThreshold', internalType: 'uint256', type: 'uint256' },
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDecryptionThreshold',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMemberForNodeDkg',
    outputs: [{ name: 'bp', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNextBackupPartyMembers',
    outputs: [
      { name: 'backupMembers', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNextBackupState',
    outputs: [
      {
        name: 'nextState',
        internalType: 'struct LibBackupRecoveryStorage.NextStateDownloadable',
        type: 'tuple',
        components: [
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
          {
            name: 'registeredRecoveryKeys',
            internalType: 'struct LibBackupRecoveryStorage.RecoveryKey[]',
            type: 'tuple[]',
            components: [
              { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
              { name: 'keyType', internalType: 'uint256', type: 'uint256' },
            ],
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNodeAddressesForDkg',
    outputs: [{ name: 'nodes', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNodeForBackupMember',
    outputs: [{ name: 'peer', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'sessionId', internalType: 'bytes', type: 'bytes' }],
    name: 'getPastBackupState',
    outputs: [
      {
        name: 'partyState',
        internalType: 'struct LibBackupRecoveryStorage.BackupRecoveryState',
        type: 'tuple',
        components: [
          { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
          { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
          {
            name: 'secp256K1EcdsaPubKey',
            internalType: 'bytes',
            type: 'bytes',
          },
          { name: 'partyThreshold', internalType: 'uint256', type: 'uint256' },
          {
            name: 'partyMembers',
            internalType: 'address[]',
            type: 'address[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getProofSubmissionForBackupPartyMember',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakerAddressesForDkg',
    outputs: [{ name: 'nodes', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isNodeForDkg',
    outputs: [{ name: 'inSet', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isRecoveryDkgCompleted',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'publicKey', internalType: 'bytes', type: 'bytes' },
      { name: 'encryptedKey', internalType: 'bytes', type: 'bytes' },
      { name: 'sessionId', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'recieveNewKeySet',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'proof', internalType: 'bytes', type: 'bytes' }],
    name: 'recieveProofBls12381G1',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'proof', internalType: 'bytes', type: 'bytes' }],
    name: 'recieveProofsK256',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'partyMembers', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'registerNewBackupParty',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'recoveryKeys',
        internalType: 'struct LibBackupRecoveryStorage.RecoveryKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'registerRecoveryKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'setMemberForDkg',
    outputs: [{ name: 'bp', internalType: 'address', type: 'address' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// BackupRecoveryNodeStatusFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const backupRecoveryNodeStatusFacetAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'CURRENT',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'clearNodeRecoveryStatus',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNodeRecoveryStatus',
    outputs: [
      {
        name: '',
        internalType: 'struct LibBackupRecoveryStorage.NodeRecoveryStatusMap[]',
        type: 'tuple[]',
        components: [
          { name: 'node_address', internalType: 'address', type: 'address' },
          {
            name: 'status',
            internalType: 'enum LibBackupRecoveryStorage.NodeRecoveryStatus',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'status',
        internalType: 'enum LibBackupRecoveryStorage.NodeRecoveryStatus',
        type: 'uint8',
      },
    ],
    name: 'setNodeRecoveryStatus',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// BackupRecoveryTestFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const backupRecoveryTestFacetAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'bls12381G1EncKey', internalType: 'bytes', type: 'bytes' },
      { name: 'secp256K1EcdsaPubKey', internalType: 'bytes', type: 'bytes' },
      { name: 'partyMembers', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'setBackupPartyState',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// BackupRecoveryViewFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const backupRecoveryViewFacetAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'getNonSubmitingBackupMembersInNextState',
    outputs: [
      {
        name: 'missingRecoveryMembers',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// CloneNet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const cloneNetAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct CloneNetArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'nonpayable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// CloneNetDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const cloneNetDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'function',
    inputs: [
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'adminAddActiveStakingContract',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'adminRemoveActiveStakingContract',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveStakingContracts',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getAllActiveUnkickedValidatorStructsAndCounts',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.KeyedStakingAggregateDetails[]',
        type: 'tuple[]',
        components: [
          {
            name: 'stakingContractAddress',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'details',
            internalType: 'struct LibStakingStorage.StakingAggregateDetails',
            type: 'tuple',
            components: [
              {
                name: 'epoch',
                internalType: 'struct LibStakingStorage.Epoch',
                type: 'tuple',
                components: [
                  {
                    name: 'epochLength',
                    internalType: 'uint256',
                    type: 'uint256',
                  },
                  { name: 'number', internalType: 'uint256', type: 'uint256' },
                  { name: 'endTime', internalType: 'uint256', type: 'uint256' },
                  { name: 'retries', internalType: 'uint256', type: 'uint256' },
                  { name: 'timeout', internalType: 'uint256', type: 'uint256' },
                ],
              },
              {
                name: 'currentValidatorCountForConsensus',
                internalType: 'uint256',
                type: 'uint256',
              },
              {
                name: 'activeUnkickedValidators',
                internalType: 'struct LibStakingStorage.Validator[]',
                type: 'tuple[]',
                components: [
                  { name: 'ip', internalType: 'uint32', type: 'uint32' },
                  { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
                  { name: 'port', internalType: 'uint32', type: 'uint32' },
                  {
                    name: 'nodeAddress',
                    internalType: 'address',
                    type: 'address',
                  },
                  { name: 'reward', internalType: 'uint256', type: 'uint256' },
                  {
                    name: 'senderPubKey',
                    internalType: 'uint256',
                    type: 'uint256',
                  },
                  {
                    name: 'receiverPubKey',
                    internalType: 'uint256',
                    type: 'uint256',
                  },
                ],
              },
            ],
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'numActiveStakingContracts',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// CloneNetFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const cloneNetFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'function',
    inputs: [
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'adminAddActiveStakingContract',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'adminRemoveActiveStakingContract',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveStakingContracts',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getAllActiveUnkickedValidatorStructsAndCounts',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.KeyedStakingAggregateDetails[]',
        type: 'tuple[]',
        components: [
          {
            name: 'stakingContractAddress',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'details',
            internalType: 'struct LibStakingStorage.StakingAggregateDetails',
            type: 'tuple',
            components: [
              {
                name: 'epoch',
                internalType: 'struct LibStakingStorage.Epoch',
                type: 'tuple',
                components: [
                  {
                    name: 'epochLength',
                    internalType: 'uint256',
                    type: 'uint256',
                  },
                  { name: 'number', internalType: 'uint256', type: 'uint256' },
                  { name: 'endTime', internalType: 'uint256', type: 'uint256' },
                  { name: 'retries', internalType: 'uint256', type: 'uint256' },
                  { name: 'timeout', internalType: 'uint256', type: 'uint256' },
                ],
              },
              {
                name: 'currentValidatorCountForConsensus',
                internalType: 'uint256',
                type: 'uint256',
              },
              {
                name: 'activeUnkickedValidators',
                internalType: 'struct LibStakingStorage.Validator[]',
                type: 'tuple[]',
                components: [
                  { name: 'ip', internalType: 'uint32', type: 'uint32' },
                  { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
                  { name: 'port', internalType: 'uint32', type: 'uint32' },
                  {
                    name: 'nodeAddress',
                    internalType: 'address',
                    type: 'address',
                  },
                  { name: 'reward', internalType: 'uint256', type: 'uint256' },
                  {
                    name: 'senderPubKey',
                    internalType: 'uint256',
                    type: 'uint256',
                  },
                  {
                    name: 'receiverPubKey',
                    internalType: 'uint256',
                    type: 'uint256',
                  },
                ],
              },
            ],
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'numActiveStakingContracts',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ContextUpgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const contextUpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ContractResolver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const contractResolverAbi = [
  {
    type: 'constructor',
    inputs: [
      { name: 'env', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'AdminRoleRequired' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'env',
        internalType: 'enum ContractResolver.Env',
        type: 'uint8',
        indexed: false,
      },
    ],
    name: 'AllowedEnvAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'env',
        internalType: 'enum ContractResolver.Env',
        type: 'uint8',
        indexed: false,
      },
    ],
    name: 'AllowedEnvRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'previousAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
      {
        name: 'newAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
    ],
    name: 'RoleAdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleGranted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleRevoked',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'typ', internalType: 'bytes32', type: 'bytes32', indexed: false },
      {
        name: 'env',
        internalType: 'enum ContractResolver.Env',
        type: 'uint8',
        indexed: false,
      },
      {
        name: 'addr',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'SetContract',
  },
  {
    type: 'function',
    inputs: [],
    name: 'ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'ALLOWLIST_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'BACKUP_RECOVERY_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'CLONE_NET_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DEFAULT_ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DOMAIN_WALLET_REGISTRY',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'HD_KEY_DERIVER_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'HOST_COMMANDS_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'LIT_TOKEN_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'MULTI_SENDER_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PAYMENT_DELEGATION_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PKP_HELPER_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PKP_HELPER_V2_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PKP_NFT_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PKP_NFT_METADATA_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PKP_PERMISSIONS_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PUB_KEY_ROUTER_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'RATE_LIMIT_NFT_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'RELEASE_REGISTER_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'STAKING_BALANCES_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'STAKING_CONTRACT',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'newAdmin', internalType: 'address', type: 'address' }],
    name: 'addAdmin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'env', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    name: 'addAllowedEnv',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'typ', internalType: 'bytes32', type: 'bytes32' },
      { name: 'env', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    name: 'getContract',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'role', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRoleAdmin',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'grantRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'hasRole',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'adminBeingRemoved', internalType: 'address', type: 'address' },
    ],
    name: 'removeAdmin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'env', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    name: 'removeAllowedEnv',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'renounceRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'revokeRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'typ', internalType: 'bytes32', type: 'bytes32' },
      { name: 'env', internalType: 'enum ContractResolver.Env', type: 'uint8' },
      { name: 'addr', internalType: 'address', type: 'address' },
    ],
    name: 'setContract',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'bytes32', type: 'bytes32' },
      { name: '', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    name: 'typeAddresses',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DiamondCutFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const diamondCutFacetAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DiamondInit
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const diamondInitAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'init',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DiamondLoupeFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const diamondLoupeFacetAbi = [
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DiamondLoupeFacetNoERC165
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const diamondLoupeFacetNoErc165Abi = [
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DiamondMultiInit
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const diamondMultiInitAbi = [
  {
    type: 'error',
    inputs: [
      { name: '_addressesLength', internalType: 'uint256', type: 'uint256' },
      { name: '_calldataLength', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'AddressAndCalldataLengthDoNotMatch',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'function',
    inputs: [
      { name: '_addresses', internalType: 'address[]', type: 'address[]' },
      { name: '_calldata', internalType: 'bytes[]', type: 'bytes[]' },
    ],
    name: 'multiInit',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DomainWalletRegistry
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const domainWalletRegistryAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct ConstructorArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'payable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DomainWalletRegistryDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const domainWalletRegistryDiamondAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [
      { name: 'metadataCount', internalType: 'uint256', type: 'uint256' },
      { name: 'validMetadataCount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'InvalidNftMetadataCollectionLength',
  },
  {
    type: 'error',
    inputs: [
      { name: 'length', internalType: 'uint256', type: 'uint256' },
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'MaximumCharacterLimitExceeded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'ttl', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Expired',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'id', internalType: 'uint64', type: 'uint64', indexed: false },
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      { name: 'ttl', internalType: 'uint256', type: 'uint256', indexed: false },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Registered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'Removed',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'Revoked',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'hasExpired',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'userId', internalType: 'bytes', type: 'bytes' },
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
      { name: 'ttl', internalType: 'uint256', type: 'uint256' },
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
    ],
    name: 'registerDomain',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'userId', internalType: 'bytes', type: 'bytes' },
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
      { name: 'ttl', internalType: 'uint256', type: 'uint256' },
      {
        name: 'permittedAuthMethodTypes',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIds',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodPubkeys',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodScopes',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
    ],
    name: 'registerDomainAndMintNext',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'id', internalType: 'uint64', type: 'uint64' },
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'registerPKP',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'removeDomain',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'revokeDomain',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
    ],
    name: 'setPKPMetadata',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'record', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'updateDomainRecord',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'error',
    inputs: [
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'DomainAlreadyRegistered',
  },
  {
    type: 'function',
    inputs: [{ name: 'uri', internalType: 'bytes', type: 'bytes' }],
    name: 'checkRegistration',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDomainCharacterLimit',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getDomainIdByTokenId',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'uri', internalType: 'bytes', type: 'bytes' }],
    name: 'getDomainTokenIdByUri',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getDomainUri',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDomainWalletRegistryAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getExpiration',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpHelperAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'id', internalType: 'uint64', type: 'uint64' }],
    name: 'getPkpTokenId',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getRecord',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'hasOwner',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isOwner',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isRouted',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DomainWalletRegistryFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const domainWalletRegistryFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [
      { name: 'metadataCount', internalType: 'uint256', type: 'uint256' },
      { name: 'validMetadataCount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'InvalidNftMetadataCollectionLength',
  },
  {
    type: 'error',
    inputs: [
      { name: 'length', internalType: 'uint256', type: 'uint256' },
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'MaximumCharacterLimitExceeded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'ttl', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Expired',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'id', internalType: 'uint64', type: 'uint64', indexed: false },
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      { name: 'ttl', internalType: 'uint256', type: 'uint256', indexed: false },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Registered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'Removed',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'subDomain',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'Revoked',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'hasExpired',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'userId', internalType: 'bytes', type: 'bytes' },
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
      { name: 'ttl', internalType: 'uint256', type: 'uint256' },
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
    ],
    name: 'registerDomain',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'userId', internalType: 'bytes', type: 'bytes' },
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
      { name: 'ttl', internalType: 'uint256', type: 'uint256' },
      {
        name: 'permittedAuthMethodTypes',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIds',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodPubkeys',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodScopes',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
    ],
    name: 'registerDomainAndMintNext',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'id', internalType: 'uint64', type: 'uint64' },
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'registerPKP',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'removeDomain',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'revokeDomain',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
    ],
    name: 'setPKPMetadata',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'record', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'updateDomainRecord',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// DomainWalletRegistryViewsFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const domainWalletRegistryViewsFacetAbi = [
  {
    type: 'error',
    inputs: [
      { name: 'uri', internalType: 'bytes', type: 'bytes' },
      { name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'DomainAlreadyRegistered',
  },
  {
    type: 'function',
    inputs: [{ name: 'uri', internalType: 'bytes', type: 'bytes' }],
    name: 'checkRegistration',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDomainCharacterLimit',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getDomainIdByTokenId',
    outputs: [{ name: '', internalType: 'uint64', type: 'uint64' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'uri', internalType: 'bytes', type: 'bytes' }],
    name: 'getDomainTokenIdByUri',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getDomainUri',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDomainWalletRegistryAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getExpiration',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpHelperAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'id', internalType: 'uint64', type: 'uint64' }],
    name: 'getPkpTokenId',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getRecord',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'hasOwner',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isOwner',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pkpTokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isRouted',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// EIP712
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const eip712Abi = [
  { type: 'error', inputs: [], name: 'InvalidShortString' },
  {
    type: 'error',
    inputs: [{ name: 'str', internalType: 'string', type: 'string' }],
    name: 'StringTooLong',
  },
  { type: 'event', anonymous: false, inputs: [], name: 'EIP712DomainChanged' },
  {
    type: 'function',
    inputs: [],
    name: 'eip712Domain',
    outputs: [
      { name: 'fields', internalType: 'bytes1', type: 'bytes1' },
      { name: 'name', internalType: 'string', type: 'string' },
      { name: 'version', internalType: 'string', type: 'string' },
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'verifyingContract', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensions', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC165
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc165Abi = [
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC165Upgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc165UpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20Abi = [
  {
    type: 'constructor',
    inputs: [
      { name: 'name_', internalType: 'string', type: 'string' },
      { name: 'symbol_', internalType: 'string', type: 'string' },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20Burnable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20BurnableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'amount', internalType: 'uint256', type: 'uint256' }],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'burnFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20Capped
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20CappedAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'cap',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20Pausable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20PausableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'Paused',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'Unpaused',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'paused',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20Permit
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20PermitAbi = [
  { type: 'error', inputs: [], name: 'InvalidShortString' },
  {
    type: 'error',
    inputs: [{ name: 'str', internalType: 'string', type: 'string' }],
    name: 'StringTooLong',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  { type: 'event', anonymous: false, inputs: [], name: 'EIP712DomainChanged' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DOMAIN_SEPARATOR',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'eip712Domain',
    outputs: [
      { name: 'fields', internalType: 'bytes1', type: 'bytes1' },
      { name: 'name', internalType: 'string', type: 'string' },
      { name: 'version', internalType: 'string', type: 'string' },
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'verifyingContract', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensions', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'nonces',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'deadline', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'permit',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20Votes
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20VotesAbi = [
  { type: 'error', inputs: [], name: 'InvalidShortString' },
  {
    type: 'error',
    inputs: [{ name: 'str', internalType: 'string', type: 'string' }],
    name: 'StringTooLong',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'fromDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'toDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'DelegateChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'previousBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'DelegateVotesChanged',
  },
  { type: 'event', anonymous: false, inputs: [], name: 'EIP712DomainChanged' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [],
    name: 'CLOCK_MODE',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DOMAIN_SEPARATOR',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'pos', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'checkpoints',
    outputs: [
      {
        name: '',
        internalType: 'struct ERC20Votes.Checkpoint',
        type: 'tuple',
        components: [
          { name: 'fromBlock', internalType: 'uint32', type: 'uint32' },
          { name: 'votes', internalType: 'uint224', type: 'uint224' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'clock',
    outputs: [{ name: '', internalType: 'uint48', type: 'uint48' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'delegatee', internalType: 'address', type: 'address' }],
    name: 'delegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'delegatee', internalType: 'address', type: 'address' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'delegateBySig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'delegates',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'eip712Domain',
    outputs: [
      { name: 'fields', internalType: 'bytes1', type: 'bytes1' },
      { name: 'name', internalType: 'string', type: 'string' },
      { name: 'version', internalType: 'string', type: 'string' },
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'verifyingContract', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensions', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'timepoint', internalType: 'uint256', type: 'uint256' }],
    name: 'getPastTotalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'timepoint', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'getPastVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'getVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'nonces',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'numCheckpoints',
    outputs: [{ name: '', internalType: 'uint32', type: 'uint32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'deadline', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'permit',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC721BurnableUpgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc721BurnableUpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC721EnumerableUpgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc721EnumerableUpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'index', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'tokenOfOwnerByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC721Upgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc721UpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// HostCommands
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const hostCommandsAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct HostCommandArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'payable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// HostCommandsFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const hostCommandsFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotAuthorizedCommandSender' },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  { type: 'error', inputs: [], name: 'ExpirationTimeInPast' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newAuthorizedCommandSender',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'AuthorizedCommandSenderUpdated',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'stakeAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'expirationTime',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'force', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'Restart',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'stakeAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'expirationTime',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'releaseBranchName',
        internalType: 'string',
        type: 'string',
        indexed: false,
      },
      {
        name: 'releaseId',
        internalType: 'string',
        type: 'string',
        indexed: false,
      },
      { name: 'force', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'Upgrade',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakeAddress', internalType: 'address', type: 'address' },
      { name: 'expirationTime', internalType: 'uint256', type: 'uint256' },
      { name: 'force', internalType: 'bool', type: 'bool' },
    ],
    name: 'restart',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_newAuthorizedCommandSender',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'setAuthorizedCommandSender',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakeAddress', internalType: 'address', type: 'address' },
      { name: 'expirationTime', internalType: 'uint256', type: 'uint256' },
      { name: 'releaseBranchName', internalType: 'string', type: 'string' },
      { name: 'releaseId', internalType: 'string', type: 'string' },
      { name: 'force', internalType: 'bool', type: 'bool' },
    ],
    name: 'upgrade',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IAccessControl
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iAccessControlAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'previousAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
      {
        name: 'newAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
    ],
    name: 'RoleAdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleGranted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleRevoked',
  },
  {
    type: 'function',
    inputs: [{ name: 'role', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRoleAdmin',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'grantRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'hasRole',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'renounceRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'revokeRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iDiamondAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IDiamondCut
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iDiamondCutAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IDiamondLoupe
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iDiamondLoupeAbi = [
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: 'facetFunctionSelectors_',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC173
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc173Abi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC20
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc20Abi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC20Metadata
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc20MetadataAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC20Permit
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc20PermitAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'DOMAIN_SEPARATOR',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'nonces',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'deadline', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'permit',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC5267
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc5267Abi = [
  { type: 'event', anonymous: false, inputs: [], name: 'EIP712DomainChanged' },
  {
    type: 'function',
    inputs: [],
    name: 'eip712Domain',
    outputs: [
      { name: 'fields', internalType: 'bytes1', type: 'bytes1' },
      { name: 'name', internalType: 'string', type: 'string' },
      { name: 'version', internalType: 'string', type: 'string' },
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'verifyingContract', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensions', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC5805
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc5805Abi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'fromDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'toDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'DelegateChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'previousBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'DelegateVotesChanged',
  },
  {
    type: 'function',
    inputs: [],
    name: 'CLOCK_MODE',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'clock',
    outputs: [{ name: '', internalType: 'uint48', type: 'uint48' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'delegatee', internalType: 'address', type: 'address' }],
    name: 'delegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'delegatee', internalType: 'address', type: 'address' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'delegateBySig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'delegates',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'timepoint', internalType: 'uint256', type: 'uint256' }],
    name: 'getPastTotalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'timepoint', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'getPastVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'getVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC6372
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc6372Abi = [
  {
    type: 'function',
    inputs: [],
    name: 'CLOCK_MODE',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'clock',
    outputs: [{ name: '', internalType: 'uint48', type: 'uint48' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721Abi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721Enumerable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721EnumerableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'index', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'tokenOfOwnerByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721EnumerableUpgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721EnumerableUpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'index', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'tokenOfOwnerByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721Metadata
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721MetadataAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721MetadataUpgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721MetadataUpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721Receiver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721ReceiverAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC721Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721ReceiverUpgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721ReceiverUpgradeableAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC721Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IERC721Upgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ierc721UpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: 'operator', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IHDKeyDeriver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ihdKeyDeriverAbi = [
  {
    type: 'function',
    inputs: [{ name: 'data', internalType: 'bytes', type: 'bytes' }],
    name: 'hdKeyDerive',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IKeyDeriver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iKeyDeriverAbi = [
  {
    type: 'function',
    inputs: [
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'rootHDKeys',
        internalType: 'struct IPubkeyRouter.RootKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'computeHDPubKey',
    outputs: [
      { name: '', internalType: 'bool', type: 'bool' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IVotes
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const iVotesAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'fromDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'toDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'DelegateChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'previousBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'DelegateVotesChanged',
  },
  {
    type: 'function',
    inputs: [{ name: 'delegatee', internalType: 'address', type: 'address' }],
    name: 'delegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'delegatee', internalType: 'address', type: 'address' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'delegateBySig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'delegates',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'timepoint', internalType: 'uint256', type: 'uint256' }],
    name: 'getPastTotalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'timepoint', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'getPastVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'getVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Initializable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const initializableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// KeyDeriver
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const keyDeriverAbi = [
  { type: 'constructor', inputs: [], stateMutability: 'nonpayable' },
  {
    type: 'function',
    inputs: [],
    name: 'HD_KDF',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'rootHDKeys',
        internalType: 'struct IPubkeyRouter.RootKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'computeHDPubKey',
    outputs: [
      { name: '', internalType: 'bool', type: 'bool' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// LITToken
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const litTokenAbi = [
  {
    type: 'constructor',
    inputs: [{ name: 'cap', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'InvalidShortString' },
  {
    type: 'error',
    inputs: [{ name: 'str', internalType: 'string', type: 'string' }],
    name: 'StringTooLong',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'spender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'fromDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'toDelegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'DelegateChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'delegate',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'previousBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newBalance',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'DelegateVotesChanged',
  },
  { type: 'event', anonymous: false, inputs: [], name: 'EIP712DomainChanged' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'Paused',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'previousAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
      {
        name: 'newAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
    ],
    name: 'RoleAdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleGranted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleRevoked',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'value',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'Unpaused',
  },
  {
    type: 'function',
    inputs: [],
    name: 'ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'CLOCK_MODE',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DEFAULT_ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DOMAIN_SEPARATOR',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'MINTER_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'PAUSER_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'amount', internalType: 'uint256', type: 'uint256' }],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'burnFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'cap',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'pos', internalType: 'uint32', type: 'uint32' },
    ],
    name: 'checkpoints',
    outputs: [
      {
        name: '',
        internalType: 'struct ERC20Votes.Checkpoint',
        type: 'tuple',
        components: [
          { name: 'fromBlock', internalType: 'uint32', type: 'uint32' },
          { name: 'votes', internalType: 'uint224', type: 'uint224' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'clock',
    outputs: [{ name: '', internalType: 'uint48', type: 'uint48' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'delegatee', internalType: 'address', type: 'address' }],
    name: 'delegate',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'delegatee', internalType: 'address', type: 'address' },
      { name: 'nonce', internalType: 'uint256', type: 'uint256' },
      { name: 'expiry', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'delegateBySig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'delegates',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'eip712Domain',
    outputs: [
      { name: 'fields', internalType: 'bytes1', type: 'bytes1' },
      { name: 'name', internalType: 'string', type: 'string' },
      { name: 'version', internalType: 'string', type: 'string' },
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'verifyingContract', internalType: 'address', type: 'address' },
      { name: 'salt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensions', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'timepoint', internalType: 'uint256', type: 'uint256' }],
    name: 'getPastTotalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'timepoint', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'getPastVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'role', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRoleAdmin',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'getVotes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'grantRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'hasRole',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_recipient', internalType: 'address', type: 'address' },
      { name: '_amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'mint',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'nonces',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'numCheckpoints',
    outputs: [{ name: '', internalType: 'uint32', type: 'uint32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'pause',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'paused',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'value', internalType: 'uint256', type: 'uint256' },
      { name: 'deadline', internalType: 'uint256', type: 'uint256' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 's', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'permit',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'renounceRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'revokeRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'unpause',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// LibDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const libDiamondAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Multisender
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const multisenderAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_recipients', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'sendEth',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_recipients', internalType: 'address[]', type: 'address[]' },
      { name: 'tokenContract', internalType: 'address', type: 'address' },
    ],
    name: 'sendTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenContract', internalType: 'address', type: 'address' },
    ],
    name: 'withdrawTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Ownable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ownableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// OwnershipFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const ownershipFacetAbi = [
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPHelper
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpHelperAbi = [
  {
    type: 'constructor',
    inputs: [
      { name: '_resolver', internalType: 'address', type: 'address' },
      {
        name: '_env',
        internalType: 'enum ContractResolver.Env',
        type: 'uint8',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'previousAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
      {
        name: 'newAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
    ],
    name: 'RoleAdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleGranted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleRevoked',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DEFAULT_ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'claimMaterial',
        internalType: 'struct LibPKPNFTStorage.ClaimMaterial',
        type: 'tuple',
        components: [
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
          {
            name: 'signatures',
            internalType: 'struct IPubkeyRouter.Signature[]',
            type: 'tuple[]',
            components: [
              { name: 'r', internalType: 'bytes32', type: 'bytes32' },
              { name: 's', internalType: 'bytes32', type: 'bytes32' },
              { name: 'v', internalType: 'uint8', type: 'uint8' },
            ],
          },
          {
            name: 'stakingContractAddress',
            internalType: 'address',
            type: 'address',
          },
        ],
      },
      {
        name: 'authMethodData',
        internalType: 'struct PKPHelper.AuthMethodData',
        type: 'tuple',
        components: [
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          {
            name: 'permittedIpfsCIDs',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedIpfsCIDScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'permittedAddresses',
            internalType: 'address[]',
            type: 'address[]',
          },
          {
            name: 'permittedAddressScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'permittedAuthMethodTypes',
            internalType: 'uint256[]',
            type: 'uint256[]',
          },
          {
            name: 'permittedAuthMethodIds',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodPubkeys',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'addPkpEthAddressAsPermittedAddress',
            internalType: 'bool',
            type: 'bool',
          },
          { name: 'sendPkpToItself', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    name: 'claimAndMintNextAndAddAuthMethods',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'claimMaterial',
        internalType: 'struct LibPKPNFTStorage.ClaimMaterial',
        type: 'tuple',
        components: [
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
          {
            name: 'signatures',
            internalType: 'struct IPubkeyRouter.Signature[]',
            type: 'tuple[]',
            components: [
              { name: 'r', internalType: 'bytes32', type: 'bytes32' },
              { name: 's', internalType: 'bytes32', type: 'bytes32' },
              { name: 'v', internalType: 'uint8', type: 'uint8' },
            ],
          },
          {
            name: 'stakingContractAddress',
            internalType: 'address',
            type: 'address',
          },
        ],
      },
      {
        name: 'authMethodData',
        internalType: 'struct PKPHelper.AuthMethodData',
        type: 'tuple',
        components: [
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          {
            name: 'permittedIpfsCIDs',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedIpfsCIDScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'permittedAddresses',
            internalType: 'address[]',
            type: 'address[]',
          },
          {
            name: 'permittedAddressScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'permittedAuthMethodTypes',
            internalType: 'uint256[]',
            type: 'uint256[]',
          },
          {
            name: 'permittedAuthMethodIds',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodPubkeys',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'addPkpEthAddressAsPermittedAddress',
            internalType: 'bool',
            type: 'bool',
          },
          { name: 'sendPkpToItself', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    name: 'claimAndMintNextAndAddAuthMethodsWithTypes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [
      { name: '', internalType: 'contract ContractResolver', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'env',
    outputs: [
      { name: '', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDomainWalletRegistry',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPKPNftMetdataAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpPermissionsAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'role', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRoleAdmin',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'grantRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'hasRole',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      {
        name: 'permittedAuthMethodTypes',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIds',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodPubkeys',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodScopes',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      {
        name: 'addPkpEthAddressAsPermittedAddress',
        internalType: 'bool',
        type: 'bool',
      },
      { name: 'sendPkpToItself', internalType: 'bool', type: 'bool' },
    ],
    name: 'mintNextAndAddAuthMethods',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'permittedIpfsCIDs', internalType: 'bytes[]', type: 'bytes[]' },
      {
        name: 'permittedIpfsCIDScopes',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      {
        name: 'permittedAddresses',
        internalType: 'address[]',
        type: 'address[]',
      },
      {
        name: 'permittedAddressScopes',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      {
        name: 'permittedAuthMethodTypes',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIds',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodPubkeys',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodScopes',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      {
        name: 'addPkpEthAddressAsPermittedAddress',
        internalType: 'bool',
        type: 'bool',
      },
      { name: 'sendPkpToItself', internalType: 'bool', type: 'bool' },
    ],
    name: 'mintNextAndAddAuthMethodsWithTypes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      {
        name: 'permittedAuthMethodTypes',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIds',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodPubkeys',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodScopes',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
      {
        name: 'addPkpEthAddressAsPermittedAddress',
        internalType: 'bool',
        type: 'bool',
      },
      { name: 'sendPkpToItself', internalType: 'bool', type: 'bool' },
    ],
    name: 'mintNextAndAddDomainWalletMetadata',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC721Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'removePkpMetadata',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'renounceRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'revokeRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'nftMetadata', internalType: 'string[]', type: 'string[]' },
    ],
    name: 'setPkpMetadata',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPHelperV2
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpHelperV2Abi = [
  {
    type: 'constructor',
    inputs: [
      { name: '_resolver', internalType: 'address', type: 'address' },
      {
        name: '_env',
        internalType: 'enum ContractResolver.Env',
        type: 'uint8',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [
      { name: '', internalType: 'contract ContractResolver', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'env',
    outputs: [
      { name: '', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getDomainWalletRegistry',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPKPNftMetdataAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpPermissionsAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'params',
        internalType: 'struct PKPHelperV2.NewPKPParams',
        type: 'tuple',
        components: [
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          {
            name: 'permittedAuthMethodTypes',
            internalType: 'uint256[]',
            type: 'uint256[]',
          },
          {
            name: 'permittedAuthMethodIds',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodPubkeys',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'addPkpEthAddressAsPermittedAddress',
            internalType: 'bool',
            type: 'bool',
          },
          {
            name: 'pkpEthAddressScopes',
            internalType: 'uint256[]',
            type: 'uint256[]',
          },
          { name: 'sendPkpToItself', internalType: 'bool', type: 'bool' },
          { name: 'burnPkp', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    name: 'mintNextAndAddAuthMethods',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'params',
        internalType: 'struct PKPHelperV2.NewPKPParams',
        type: 'tuple',
        components: [
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          {
            name: 'permittedAuthMethodTypes',
            internalType: 'uint256[]',
            type: 'uint256[]',
          },
          {
            name: 'permittedAuthMethodIds',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodPubkeys',
            internalType: 'bytes[]',
            type: 'bytes[]',
          },
          {
            name: 'permittedAuthMethodScopes',
            internalType: 'uint256[][]',
            type: 'uint256[][]',
          },
          {
            name: 'addPkpEthAddressAsPermittedAddress',
            internalType: 'bool',
            type: 'bool',
          },
          {
            name: 'pkpEthAddressScopes',
            internalType: 'uint256[]',
            type: 'uint256[]',
          },
          { name: 'sendPkpToItself', internalType: 'bool', type: 'bool' },
          { name: 'burnPkp', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    name: 'mintNextAndAddAuthMethodsWithTypes',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC721Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPNFT
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpnftAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct StakingArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'payable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPNFTDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpnftDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newFreeMintSigner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'FreeMintSignerSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMintCost',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MintCostSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'PKPMinted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Withdrew',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'signatures',
        internalType: 'struct IPubkeyRouter.Signature[]',
        type: 'tuple[]',
        components: [
          { name: 'r', internalType: 'bytes32', type: 'bytes32' },
          { name: 's', internalType: 'bytes32', type: 'bytes32' },
          { name: 'v', internalType: 'uint8', type: 'uint8' },
        ],
      },
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'claimAndMint',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'exists',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'freeMintSigner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getEthAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNextDerivedKeyId',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftMetadataAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpPermissionsAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getRouterAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakingAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'initialize',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'mintCost',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'mintGrantAndBurnNext',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [{ name: 'keyType', internalType: 'uint256', type: 'uint256' }],
    name: 'mintNext',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'hash', internalType: 'bytes32', type: 'bytes32' }],
    name: 'prefixed',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'redeemedFreeMintIds',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newFreeMintSigner', internalType: 'address', type: 'address' },
    ],
    name: 'setFreeMintSigner',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newMintCost', internalType: 'uint256', type: 'uint256' }],
    name: 'setMintCost',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'index', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'tokenOfOwnerByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPNFTFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpnftFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newFreeMintSigner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'FreeMintSignerSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMintCost',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MintCostSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'PKPMinted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Withdrew',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'signatures',
        internalType: 'struct IPubkeyRouter.Signature[]',
        type: 'tuple[]',
        components: [
          { name: 'r', internalType: 'bytes32', type: 'bytes32' },
          { name: 's', internalType: 'bytes32', type: 'bytes32' },
          { name: 'v', internalType: 'uint8', type: 'uint8' },
        ],
      },
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'claimAndMint',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'exists',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'freeMintSigner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getEthAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getNextDerivedKeyId',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftMetadataAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpPermissionsAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getRouterAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakingAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'initialize',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'mintCost',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'mintGrantAndBurnNext',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [{ name: 'keyType', internalType: 'uint256', type: 'uint256' }],
    name: 'mintNext',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'hash', internalType: 'bytes32', type: 'bytes32' }],
    name: 'prefixed',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'redeemedFreeMintIds',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newFreeMintSigner', internalType: 'address', type: 'address' },
    ],
    name: 'setFreeMintSigner',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'newMintCost', internalType: 'uint256', type: 'uint256' }],
    name: 'setMintCost',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'index', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'tokenOfOwnerByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPNFTMetadata
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpnftMetadataAbi = [
  {
    type: 'constructor',
    inputs: [
      { name: '_resolver', internalType: 'address', type: 'address' },
      {
        name: '_env',
        internalType: 'enum ContractResolver.Env',
        type: 'uint8',
      },
    ],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'buffer', internalType: 'bytes', type: 'bytes' }],
    name: 'bytesToHex',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [
      { name: '', internalType: 'contract ContractResolver', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'env',
    outputs: [
      { name: '', internalType: 'enum ContractResolver.Env', type: 'uint8' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'removeProfileForPkp',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'removeUrlForPKP',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'imgUrl', internalType: 'string', type: 'string' },
    ],
    name: 'setProfileForPKP',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'url', internalType: 'string', type: 'string' },
    ],
    name: 'setUrlForPKP',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'pubKey', internalType: 'bytes', type: 'bytes' },
      { name: 'ethAddress', internalType: 'address', type: 'address' },
    ],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPPermissions
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpPermissionsAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct PKPPermissionsArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'nonpayable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPPermissionsDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpPermissionsDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'userPubkey',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'PermittedAuthMethodAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'PermittedAuthMethodRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'scopeId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'PermittedAuthMethodScopeAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'scopeId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'PermittedAuthMethodScopeRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'group',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'root',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'RootHashUpdated',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
      { name: 'scopes', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    name: 'addPermittedAction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'user', internalType: 'address', type: 'address' },
      { name: 'scopes', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    name: 'addPermittedAddress',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      {
        name: 'authMethod',
        internalType: 'struct LibPKPPermissionsStorage.AuthMethod',
        type: 'tuple',
        components: [
          { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
          { name: 'id', internalType: 'bytes', type: 'bytes' },
          { name: 'userPubkey', internalType: 'bytes', type: 'bytes' },
        ],
      },
      { name: 'scopes', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    name: 'addPermittedAuthMethod',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'scopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'addPermittedAuthMethodScope',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      {
        name: 'permittedAuthMethodTypesToAdd',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIdsToAdd',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodPubkeysToAdd',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodScopesToAdd',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      {
        name: 'permittedAuthMethodTypesToRemove',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIdsToRemove',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
    ],
    name: 'batchAddRemoveAuthMethods',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getAuthMethodId',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getEthAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getPKPPubKeysByAuthMethod',
    outputs: [{ name: '', internalType: 'bytes[]', type: 'bytes[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPermittedActions',
    outputs: [{ name: '', internalType: 'bytes[]', type: 'bytes[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPermittedAddresses',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'maxScopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'getPermittedAuthMethodScopes',
    outputs: [{ name: '', internalType: 'bool[]', type: 'bool[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPermittedAuthMethods',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPKPPermissionsStorage.AuthMethod[]',
        type: 'tuple[]',
        components: [
          { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
          { name: 'id', internalType: 'bytes', type: 'bytes' },
          { name: 'userPubkey', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getRouterAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getTokenIdsForAuthMethod',
    outputs: [{ name: '', internalType: 'uint256[]', type: 'uint256[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getUserPubkeyForAuthMethod',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'isPermittedAction',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'user', internalType: 'address', type: 'address' },
    ],
    name: 'isPermittedAddress',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'isPermittedAuthMethod',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'scopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'isPermittedAuthMethodScopePresent',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'removePermittedAction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'user', internalType: 'address', type: 'address' },
    ],
    name: 'removePermittedAddress',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'removePermittedAuthMethod',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'scopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'removePermittedAuthMethodScope',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'group', internalType: 'uint256', type: 'uint256' },
      { name: 'root', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setRootHash',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'group', internalType: 'uint256', type: 'uint256' },
      { name: 'proof', internalType: 'bytes32[]', type: 'bytes32[]' },
      { name: 'leaf', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'verifyState',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'group', internalType: 'uint256', type: 'uint256' },
      { name: 'proof', internalType: 'bytes32[]', type: 'bytes32[]' },
      { name: 'proofFlags', internalType: 'bool[]', type: 'bool[]' },
      { name: 'leaves', internalType: 'bytes32[]', type: 'bytes32[]' },
    ],
    name: 'verifyStates',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PKPPermissionsFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pkpPermissionsFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'userPubkey',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'PermittedAuthMethodAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'PermittedAuthMethodRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'scopeId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'PermittedAuthMethodScopeAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'authMethodType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      { name: 'id', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'scopeId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'PermittedAuthMethodScopeRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'group',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      {
        name: 'root',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'RootHashUpdated',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
      { name: 'scopes', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    name: 'addPermittedAction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'user', internalType: 'address', type: 'address' },
      { name: 'scopes', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    name: 'addPermittedAddress',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      {
        name: 'authMethod',
        internalType: 'struct LibPKPPermissionsStorage.AuthMethod',
        type: 'tuple',
        components: [
          { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
          { name: 'id', internalType: 'bytes', type: 'bytes' },
          { name: 'userPubkey', internalType: 'bytes', type: 'bytes' },
        ],
      },
      { name: 'scopes', internalType: 'uint256[]', type: 'uint256[]' },
    ],
    name: 'addPermittedAuthMethod',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'scopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'addPermittedAuthMethodScope',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      {
        name: 'permittedAuthMethodTypesToAdd',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIdsToAdd',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodPubkeysToAdd',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
      {
        name: 'permittedAuthMethodScopesToAdd',
        internalType: 'uint256[][]',
        type: 'uint256[][]',
      },
      {
        name: 'permittedAuthMethodTypesToRemove',
        internalType: 'uint256[]',
        type: 'uint256[]',
      },
      {
        name: 'permittedAuthMethodIdsToRemove',
        internalType: 'bytes[]',
        type: 'bytes[]',
      },
    ],
    name: 'batchAddRemoveAuthMethods',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getAuthMethodId',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getEthAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getPKPPubKeysByAuthMethod',
    outputs: [{ name: '', internalType: 'bytes[]', type: 'bytes[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPermittedActions',
    outputs: [{ name: '', internalType: 'bytes[]', type: 'bytes[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPermittedAddresses',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'maxScopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'getPermittedAuthMethodScopes',
    outputs: [{ name: '', internalType: 'bool[]', type: 'bool[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPermittedAuthMethods',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPKPPermissionsStorage.AuthMethod[]',
        type: 'tuple[]',
        components: [
          { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
          { name: 'id', internalType: 'bytes', type: 'bytes' },
          { name: 'userPubkey', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getRouterAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getTokenIdsForAuthMethod',
    outputs: [{ name: '', internalType: 'uint256[]', type: 'uint256[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'getUserPubkeyForAuthMethod',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'isPermittedAction',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'user', internalType: 'address', type: 'address' },
    ],
    name: 'isPermittedAddress',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'isPermittedAuthMethod',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'scopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'isPermittedAuthMethodScopePresent',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'ipfsCID', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'removePermittedAction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'user', internalType: 'address', type: 'address' },
    ],
    name: 'removePermittedAddress',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'removePermittedAuthMethod',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'authMethodType', internalType: 'uint256', type: 'uint256' },
      { name: 'id', internalType: 'bytes', type: 'bytes' },
      { name: 'scopeId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'removePermittedAuthMethodScope',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'group', internalType: 'uint256', type: 'uint256' },
      { name: 'root', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setRootHash',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'group', internalType: 'uint256', type: 'uint256' },
      { name: 'proof', internalType: 'bytes32[]', type: 'bytes32[]' },
      { name: 'leaf', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'verifyState',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'group', internalType: 'uint256', type: 'uint256' },
      { name: 'proof', internalType: 'bytes32[]', type: 'bytes32[]' },
      { name: 'proofFlags', internalType: 'bool[]', type: 'bool[]' },
      { name: 'leaves', internalType: 'bytes32[]', type: 'bytes32[]' },
    ],
    name: 'verifyStates',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Pausable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pausableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'Paused',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'Unpaused',
  },
  {
    type: 'function',
    inputs: [],
    name: 'paused',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PaymentDelegation
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const paymentDelegationAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct PaymentDelegationArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'nonpayable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PaymentDelegationDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const paymentDelegationDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'payer',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'restriction',
        internalType: 'struct LibPaymentDelegationStorage.Restriction',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'RestrictionSet',
  },
  {
    type: 'function',
    inputs: [{ name: 'user', internalType: 'address', type: 'address' }],
    name: 'delegatePayments',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'users', internalType: 'address[]', type: 'address[]' }],
    name: 'delegatePaymentsBatch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'user', internalType: 'address', type: 'address' }],
    name: 'getPayers',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'users', internalType: 'address[]', type: 'address[]' }],
    name: 'getPayersAndRestrictions',
    outputs: [
      { name: '', internalType: 'address[][]', type: 'address[][]' },
      {
        name: '',
        internalType: 'struct LibPaymentDelegationStorage.Restriction[][]',
        type: 'tuple[][]',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'payer', internalType: 'address', type: 'address' }],
    name: 'getRestriction',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPaymentDelegationStorage.Restriction',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'payer', internalType: 'address', type: 'address' }],
    name: 'getUsers',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'setDefaultRestriction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'r',
        internalType: 'struct LibPaymentDelegationStorage.Restriction',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'setRestriction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'user', internalType: 'address', type: 'address' }],
    name: 'undelegatePayments',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'users', internalType: 'address[]', type: 'address[]' }],
    name: 'undelegatePaymentsBatch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PaymentDelegationFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const paymentDelegationFacetAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'payer',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'restriction',
        internalType: 'struct LibPaymentDelegationStorage.Restriction',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'RestrictionSet',
  },
  {
    type: 'function',
    inputs: [{ name: 'user', internalType: 'address', type: 'address' }],
    name: 'delegatePayments',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'users', internalType: 'address[]', type: 'address[]' }],
    name: 'delegatePaymentsBatch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'user', internalType: 'address', type: 'address' }],
    name: 'getPayers',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'users', internalType: 'address[]', type: 'address[]' }],
    name: 'getPayersAndRestrictions',
    outputs: [
      { name: '', internalType: 'address[][]', type: 'address[][]' },
      {
        name: '',
        internalType: 'struct LibPaymentDelegationStorage.Restriction[][]',
        type: 'tuple[][]',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'payer', internalType: 'address', type: 'address' }],
    name: 'getRestriction',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPaymentDelegationStorage.Restriction',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'payer', internalType: 'address', type: 'address' }],
    name: 'getUsers',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'setDefaultRestriction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'r',
        internalType: 'struct LibPaymentDelegationStorage.Restriction',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerPeriod',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'periodSeconds', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'setRestriction',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'user', internalType: 'address', type: 'address' }],
    name: 'undelegatePayments',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'users', internalType: 'address[]', type: 'address[]' }],
    name: 'undelegatePaymentsBatch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PubkeyRouter
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pubkeyRouterAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct PubkeyRouterArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'nonpayable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PubkeyRouterDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pubkeyRouterDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'stakingContract',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'keyType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'derivedKeyId',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PubkeyRoutingDataSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'stakingContract',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'rootKey',
        internalType: 'struct IPubkeyRouter.RootKey',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'RootKeySet',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakingContract', internalType: 'address', type: 'address' },
    ],
    name: 'adminResetRootKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'signatures',
        internalType: 'struct IPubkeyRouter.Signature[]',
        type: 'tuple[]',
        components: [
          { name: 'r', internalType: 'bytes32', type: 'bytes32' },
          { name: 's', internalType: 'bytes32', type: 'bytes32' },
          { name: 'v', internalType: 'uint8', type: 'uint8' },
        ],
      },
      { name: 'signedMessage', internalType: 'bytes', type: 'bytes' },
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'checkNodeSignatures',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pubkey', internalType: 'bytes', type: 'bytes' }],
    name: 'deriveEthAddressFromPubkey',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'ethAddress', internalType: 'address', type: 'address' }],
    name: 'ethAddressToPkpId',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakingContract', internalType: 'address', type: 'address' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'getDerivedPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getEthAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakingContract', internalType: 'address', type: 'address' },
    ],
    name: 'getRootKeys',
    outputs: [
      {
        name: '',
        internalType: 'struct IPubkeyRouter.RootKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getRoutingData',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPubkeyRouterStorage.PubkeyRoutingData',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isRouted',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'pubkeys',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPubkeyRouterStorage.PubkeyRoutingData',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setRoutingData',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
      { name: 'stakingContract', internalType: 'address', type: 'address' },
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setRoutingDataAsAdmin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
      {
        name: 'newRootKeys',
        internalType: 'struct IPubkeyRouter.RootKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'voteForRootKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// PubkeyRouterFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const pubkeyRouterFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ContractResolverAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'stakingContract',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'keyType',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'derivedKeyId',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'PubkeyRoutingDataSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'stakingContract',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'rootKey',
        internalType: 'struct IPubkeyRouter.RootKey',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'RootKeySet',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakingContract', internalType: 'address', type: 'address' },
    ],
    name: 'adminResetRootKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'signatures',
        internalType: 'struct IPubkeyRouter.Signature[]',
        type: 'tuple[]',
        components: [
          { name: 'r', internalType: 'bytes32', type: 'bytes32' },
          { name: 's', internalType: 'bytes32', type: 'bytes32' },
          { name: 'v', internalType: 'uint8', type: 'uint8' },
        ],
      },
      { name: 'signedMessage', internalType: 'bytes', type: 'bytes' },
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'checkNodeSignatures',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pubkey', internalType: 'bytes', type: 'bytes' }],
    name: 'deriveEthAddressFromPubkey',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'ethAddress', internalType: 'address', type: 'address' }],
    name: 'ethAddressToPkpId',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakingContract', internalType: 'address', type: 'address' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'getDerivedPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getEthAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getPkpNftAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getPubkey',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakingContract', internalType: 'address', type: 'address' },
    ],
    name: 'getRootKeys',
    outputs: [
      {
        name: '',
        internalType: 'struct IPubkeyRouter.RootKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getRoutingData',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPubkeyRouterStorage.PubkeyRoutingData',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isRouted',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'pubkeys',
    outputs: [
      {
        name: '',
        internalType: 'struct LibPubkeyRouterStorage.PubkeyRoutingData',
        type: 'tuple',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
          { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setRoutingData',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
      { name: 'stakingContract', internalType: 'address', type: 'address' },
      { name: 'keyType', internalType: 'uint256', type: 'uint256' },
      { name: 'derivedKeyId', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'setRoutingDataAsAdmin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'stakingContractAddress',
        internalType: 'address',
        type: 'address',
      },
      {
        name: 'newRootKeys',
        internalType: 'struct IPubkeyRouter.RootKey[]',
        type: 'tuple[]',
        components: [
          { name: 'pubkey', internalType: 'bytes', type: 'bytes' },
          { name: 'keyType', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'voteForRootKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// RateLimitNFT
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const rateLimitNftAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct StakingArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'payable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// RateLimitNFTDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const rateLimitNftDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newAdditionalRequestsPerKilosecondCost',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'AdditionalRequestsPerKilosecondCostSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newFreeMintSigner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'FreeMintSignerSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newFreeRequestsPerRateLimitWindow',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'FreeRequestsPerRateLimitWindowSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newRLIHolderRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RLIHolderRateLimitWindowSecondsSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RateLimitWindowSecondsSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Withdrew',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
      { name: 'msgHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 'sVal', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'freeMint',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'initialize',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'expiresAt', internalType: 'uint256', type: 'uint256' }],
    name: 'mint',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'pruneExpired',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newAdditionalRequestsPerKilosecondCost',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setAdditionalRequestsPerKilosecondCost',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newFreeMintSigner', internalType: 'address', type: 'address' },
    ],
    name: 'setFreeMintSigner',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newFreeRequestsPerRateLimitWindow',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setFreeRequestsPerRateLimitWindow',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newMaxExpirationSeconds',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setMaxExpirationSeconds',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newMaxRequestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setMaxRequestsPerKilosecond',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newRLIHolderRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setRLIHolderRateLimitWindowSeconds',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setRateLimitWindowSeconds',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'index', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'tokenOfOwnerByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'RLIHolderRateLimitWindowSeconds',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'additionalRequestsPerKilosecondCost',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'requestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'calculateCost',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'payingAmount', internalType: 'uint256', type: 'uint256' },
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'calculateRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'capacity',
    outputs: [
      {
        name: '',
        internalType: 'struct LibRateLimitNFTStorage.RateLimit',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerKilosecond',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'requestedRequestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'checkBelowMaxRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'currentSoldRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'defaultRateLimitWindowSeconds',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
      { name: 'msgHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 'sVal', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'freeMintSigTest',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'freeMintSigner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'freeRequestsPerRateLimitWindow',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isExpired',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'maxExpirationSeconds',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'maxRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'hash', internalType: 'bytes32', type: 'bytes32' }],
    name: 'prefixed',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'msgHash', internalType: 'bytes32', type: 'bytes32' }],
    name: 'redeemedFreeMints',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'tokenIdCounter',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenSVG',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'expiresAt', internalType: 'uint256', type: 'uint256' }],
    name: 'totalSoldRequestsPerKilosecondByExpirationTime',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// RateLimitNFTFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const rateLimitNftFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newAdditionalRequestsPerKilosecondCost',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'AdditionalRequestsPerKilosecondCostSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'approved',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'owner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'operator',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      { name: 'approved', internalType: 'bool', type: 'bool', indexed: false },
    ],
    name: 'ApprovalForAll',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newFreeMintSigner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'FreeMintSignerSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newFreeRequestsPerRateLimitWindow',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'FreeRequestsPerRateLimitWindowSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newRLIHolderRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RLIHolderRateLimitWindowSecondsSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RateLimitWindowSecondsSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      {
        name: 'tokenId',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Withdrew',
  },
  {
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'burn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
      { name: 'msgHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 'sVal', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'freeMint',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'getApproved',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'initialize',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'operator', internalType: 'address', type: 'address' },
    ],
    name: 'isApprovedForAll',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'expiresAt', internalType: 'uint256', type: 'uint256' }],
    name: 'mint',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'ownerOf',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'owner', internalType: 'address', type: 'address' }],
    name: 'pruneExpired',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'safeTransferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newAdditionalRequestsPerKilosecondCost',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setAdditionalRequestsPerKilosecondCost',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'operator', internalType: 'address', type: 'address' },
      { name: 'approved', internalType: 'bool', type: 'bool' },
    ],
    name: 'setApprovalForAll',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newFreeMintSigner', internalType: 'address', type: 'address' },
    ],
    name: 'setFreeMintSigner',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newFreeRequestsPerRateLimitWindow',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setFreeRequestsPerRateLimitWindow',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newMaxExpirationSeconds',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setMaxExpirationSeconds',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newMaxRequestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setMaxRequestsPerKilosecond',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newRLIHolderRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setRLIHolderRateLimitWindowSeconds',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newRateLimitWindowSeconds',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setRateLimitWindowSeconds',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'index', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'index', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'tokenOfOwnerByIndex',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenURI',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'tokenId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// RateLimitNFTViewsFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const rateLimitNftViewsFacetAbi = [
  {
    type: 'function',
    inputs: [],
    name: 'RLIHolderRateLimitWindowSeconds',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'additionalRequestsPerKilosecondCost',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'requestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'calculateCost',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'payingAmount', internalType: 'uint256', type: 'uint256' },
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'calculateRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'capacity',
    outputs: [
      {
        name: '',
        internalType: 'struct LibRateLimitNFTStorage.RateLimit',
        type: 'tuple',
        components: [
          {
            name: 'requestsPerKilosecond',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'requestedRequestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'checkBelowMaxRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'currentSoldRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'defaultRateLimitWindowSeconds',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'expiresAt', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestsPerKilosecond',
        internalType: 'uint256',
        type: 'uint256',
      },
      { name: 'msgHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'v', internalType: 'uint8', type: 'uint8' },
      { name: 'r', internalType: 'bytes32', type: 'bytes32' },
      { name: 'sVal', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'freeMintSigTest',
    outputs: [],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'freeMintSigner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'freeRequestsPerRateLimitWindow',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'isExpired',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'maxExpirationSeconds',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'maxRequestsPerKilosecond',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'hash', internalType: 'bytes32', type: 'bytes32' }],
    name: 'prefixed',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'pure',
  },
  {
    type: 'function',
    inputs: [{ name: 'msgHash', internalType: 'bytes32', type: 'bytes32' }],
    name: 'redeemedFreeMints',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'tokenIdCounter',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'tokenId', internalType: 'uint256', type: 'uint256' }],
    name: 'tokenSVG',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'expiresAt', internalType: 'uint256', type: 'uint256' }],
    name: 'totalSoldRequestsPerKilosecondByExpirationTime',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ReentrancyGuardUpgradeable
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const reentrancyGuardUpgradeableAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'version', internalType: 'uint8', type: 'uint8', indexed: false },
    ],
    name: 'Initialized',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ReleaseRegister
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const releaseRegisterAbi = [
  {
    type: 'constructor',
    inputs: [
      { name: 'env', internalType: 'enum ReleaseRegister.Env', type: 'uint8' },
    ],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'ActivatorRoleRequired' },
  { type: 'error', inputs: [], name: 'AdminRoleRequired' },
  { type: 'error', inputs: [], name: 'BurnerRoleRequired' },
  { type: 'error', inputs: [], name: 'CreatorRoleRequired' },
  { type: 'error', inputs: [], name: 'DeactivatorRoleRequired' },
  { type: 'error', inputs: [], name: 'InvalidEnv' },
  { type: 'error', inputs: [], name: 'InvalidStatus' },
  { type: 'error', inputs: [], name: 'ReleaseNotFound' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'pubKey', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'AllowedAdminSigningPublicKeyAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'pubKey', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'AllowedAdminSigningPublicKeyRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'digest', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'AllowedAuthorKeyDigestAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'digest', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'AllowedAuthorKeyDigestRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'env',
        internalType: 'enum ReleaseRegister.Env',
        type: 'uint8',
        indexed: false,
      },
    ],
    name: 'AllowedEnvAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'env',
        internalType: 'enum ReleaseRegister.Env',
        type: 'uint8',
        indexed: false,
      },
    ],
    name: 'AllowedEnvRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'subnet',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'AllowedSubnetAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'subnet',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'AllowedSubnetRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'domain', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'authorKeyDigest',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'InitCreator',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'releaseId',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
    ],
    name: 'ReleaseBurned',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'releaseId',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'status',
        internalType: 'enum ReleaseRegister.Status',
        type: 'uint8',
        indexed: false,
      },
      {
        name: 'env',
        internalType: 'enum ReleaseRegister.Env',
        type: 'uint8',
        indexed: false,
      },
      {
        name: 'typ',
        internalType: 'enum ReleaseRegister.Type',
        type: 'uint8',
        indexed: false,
      },
      { name: 'kind', internalType: 'bytes', type: 'bytes', indexed: false },
      {
        name: 'date',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'platform',
        internalType: 'enum ReleaseRegister.Platform',
        type: 'uint8',
        indexed: false,
      },
      {
        name: 'options',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'id_key_digest',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      {
        name: 'public_key',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
      { name: 'cid', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'ReleaseCreated',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'releaseId',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: false,
      },
      {
        name: 'status',
        internalType: 'enum ReleaseRegister.Status',
        type: 'uint8',
        indexed: false,
      },
    ],
    name: 'ReleaseStatusChange',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'previousAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
      {
        name: 'newAdminRole',
        internalType: 'bytes32',
        type: 'bytes32',
        indexed: true,
      },
    ],
    name: 'RoleAdminChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleGranted',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32', indexed: true },
      {
        name: 'account',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'sender',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RoleRevoked',
  },
  {
    type: 'function',
    inputs: [],
    name: 'ACTIVATOR_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'BURNER_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'CREATOR_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DEACTIVATOR_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'DEFAULT_ADMIN_ROLE',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'RELEASE_OPTION_RO',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'RELEASE_OPTION_SSH',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'RELEASE_OPTION_USERS',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'pubKey', internalType: 'bytes', type: 'bytes' }],
    name: 'addAllowedAdminSigningPublicKey',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'env', internalType: 'enum ReleaseRegister.Env', type: 'uint8' },
    ],
    name: 'addAllowedEnv',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'subnet', internalType: 'address', type: 'address' }],
    name: 'addAllowedSubnet',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'releaseId', internalType: 'bytes32', type: 'bytes32' }],
    name: 'burnRelease',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'releaseId', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'status',
        internalType: 'enum ReleaseRegister.Status',
        type: 'uint8',
      },
      { name: 'env', internalType: 'enum ReleaseRegister.Env', type: 'uint8' },
      { name: 'typ', internalType: 'enum ReleaseRegister.Type', type: 'uint8' },
      { name: 'kind', internalType: 'bytes', type: 'bytes' },
      {
        name: 'platform',
        internalType: 'enum ReleaseRegister.Platform',
        type: 'uint8',
      },
      { name: 'options', internalType: 'uint256', type: 'uint256' },
      { name: 'id_key_digest', internalType: 'bytes', type: 'bytes' },
      { name: 'public_key', internalType: 'bytes', type: 'bytes' },
      { name: 'cid', internalType: 'bytes', type: 'bytes' },
      { name: 'date', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'createRelease',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'env', internalType: 'enum ReleaseRegister.Env', type: 'uint8' },
      { name: 'typ', internalType: 'enum ReleaseRegister.Type', type: 'uint8' },
      { name: 'kind', internalType: 'bytes', type: 'bytes' },
      {
        name: 'platform',
        internalType: 'enum ReleaseRegister.Platform',
        type: 'uint8',
      },
    ],
    name: 'getActiveRelease',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveReleases',
    outputs: [{ name: '', internalType: 'bytes32[]', type: 'bytes32[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getCreatorDomain',
    outputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'releaseId', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRelease',
    outputs: [
      {
        name: '',
        internalType: 'struct ReleaseRegister.Release',
        type: 'tuple',
        components: [
          {
            name: 'status',
            internalType: 'enum ReleaseRegister.Status',
            type: 'uint8',
          },
          {
            name: 'env',
            internalType: 'enum ReleaseRegister.Env',
            type: 'uint8',
          },
          {
            name: 'typ',
            internalType: 'enum ReleaseRegister.Type',
            type: 'uint8',
          },
          { name: 'kind', internalType: 'bytes', type: 'bytes' },
          { name: 'date', internalType: 'uint256', type: 'uint256' },
          {
            name: 'platform',
            internalType: 'enum ReleaseRegister.Platform',
            type: 'uint8',
          },
          { name: 'options', internalType: 'uint256', type: 'uint256' },
          { name: 'id_key_digest', internalType: 'bytes', type: 'bytes' },
          { name: 'public_key', internalType: 'bytes', type: 'bytes' },
          { name: 'cid', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'role', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getRoleAdmin',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'grantRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'pubKey', internalType: 'bytes', type: 'bytes' }],
    name: 'hasAllowedAdminSigningPublicKey',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'digest', internalType: 'bytes', type: 'bytes' }],
    name: 'hasAllowedAuthorKeyDigest',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'env', internalType: 'enum ReleaseRegister.Env', type: 'uint8' },
    ],
    name: 'hasAllowedEnv',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'subnet', internalType: 'address', type: 'address' }],
    name: 'hasAllowedSubnet',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'hasCreatorInit',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'hasRole',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'env', internalType: 'enum ReleaseRegister.Env', type: 'uint8' },
      { name: 'subnetId', internalType: 'address', type: 'address' },
      { name: 'domain', internalType: 'bytes', type: 'bytes' },
      { name: 'authorKeyDigest', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'initCreator',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'pubKey', internalType: 'bytes', type: 'bytes' }],
    name: 'removeAllowedAdminSigningPublicKey',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'env', internalType: 'enum ReleaseRegister.Env', type: 'uint8' },
    ],
    name: 'removeAllowedEnv',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'subnet', internalType: 'address', type: 'address' }],
    name: 'removeAllowedSubnet',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'renounceRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'role', internalType: 'bytes32', type: 'bytes32' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'revokeRole',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'releaseId', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'status',
        internalType: 'enum ReleaseRegister.Status',
        type: 'uint8',
      },
    ],
    name: 'setReleaseStatus',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ShortStrings
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const shortStringsAbi = [
  { type: 'error', inputs: [], name: 'InvalidShortString' },
  {
    type: 'error',
    inputs: [{ name: 'str', internalType: 'string', type: 'string' }],
    name: 'StringTooLong',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Staking
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct StakingArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'nonpayable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// StakingBalances
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingBalancesAbi = [
  {
    type: 'constructor',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      {
        name: '_args',
        internalType: 'struct StakingArgs',
        type: 'tuple',
        components: [
          { name: 'owner', internalType: 'address', type: 'address' },
          { name: 'init', internalType: 'address', type: 'address' },
          { name: 'initCalldata', internalType: 'bytes', type: 'bytes' },
          {
            name: 'contractResolver',
            internalType: 'address',
            type: 'address',
          },
          {
            name: 'env',
            internalType: 'enum ContractResolver.Env',
            type: 'uint8',
          },
        ],
      },
    ],
    stateMutability: 'payable',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'FunctionNotFound',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  { type: 'fallback', stateMutability: 'payable' },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// StakingBalancesDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingBalancesDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'ActiveValidatorsCannotLeave' },
  {
    type: 'error',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'AliasNotOwnedBySender',
  },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
    ],
    name: 'CannotRemoveAliasOfActiveValidator',
  },
  { type: 'error', inputs: [], name: 'CannotStakeZero' },
  { type: 'error', inputs: [], name: 'CannotWithdrawZero' },
  {
    type: 'error',
    inputs: [{ name: 'aliasCount', internalType: 'uint256', type: 'uint256' }],
    name: 'MaxAliasCountReached',
  },
  {
    type: 'error',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: 'OnlyStakingContract',
  },
  {
    type: 'error',
    inputs: [
      { name: 'amountStaked', internalType: 'uint256', type: 'uint256' },
      { name: 'minimumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'StakeMustBeGreaterThanMinimumStake',
  },
  {
    type: 'error',
    inputs: [
      { name: 'amountStaked', internalType: 'uint256', type: 'uint256' },
      { name: 'maximumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'StakeMustBeLessThanMaximumStake',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'StakerNotPermitted',
  },
  {
    type: 'error',
    inputs: [
      { name: 'yourBalance', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestedWithdrawlAmount',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'TryingToWithdrawMoreThanStaked',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'aliasAccount',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'AliasAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'aliasAccount',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'AliasRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMaxAliasCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MaxAliasCountSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMaximumStake',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MaximumStakeSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMinimumStake',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MinimumStakeSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'PermittedStakerAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'PermittedStakerRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'permittedStakersOn',
        internalType: 'bool',
        type: 'bool',
        indexed: false,
      },
    ],
    name: 'PermittedStakersOnChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ResolverContractAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'reward',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RewardPaid',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Staked',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newTokenRewardPerTokenPerEpoch',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'TokenRewardPerTokenPerEpochSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'aliasAccount',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ValidatorNotRewardedBecauseAlias',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ValidatorRewarded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ValidatorTokensPenalized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Withdrawn',
  },
  {
    type: 'function',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
    ],
    name: 'addAlias',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'addPermittedStaker',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'stakers', internalType: 'address[]', type: 'address[]' }],
    name: 'addPermittedStakers',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'checkStakingAmounts',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'getReward',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakingAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getTokenAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'isPermittedStaker',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'maximumStake',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'minimumStake',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'penalizeTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'permittedStakersOn',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
    ],
    name: 'removeAlias',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'removePermittedStaker',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'staker', internalType: 'address', type: 'address' },
      { name: 'balance', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'restakePenaltyTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'rewardOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'rewardValidator',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newMaxAliasCount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setMaxAliasCount',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newMaximumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setMaximumStake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newMinimumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setMinimumStake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'permitted', internalType: 'bool', type: 'bool' }],
    name: 'setPermittedStakersOn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'stake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalStaked',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'balance', internalType: 'uint256', type: 'uint256' },
      { name: 'recipient', internalType: 'address', type: 'address' },
    ],
    name: 'transferPenaltyTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    name: 'withdrawPenaltyTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// StakingBalancesFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingBalancesFacetAbi = [
  { type: 'error', inputs: [], name: 'ActiveValidatorsCannotLeave' },
  {
    type: 'error',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'AliasNotOwnedBySender',
  },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
    ],
    name: 'CannotRemoveAliasOfActiveValidator',
  },
  { type: 'error', inputs: [], name: 'CannotStakeZero' },
  { type: 'error', inputs: [], name: 'CannotWithdrawZero' },
  {
    type: 'error',
    inputs: [{ name: 'aliasCount', internalType: 'uint256', type: 'uint256' }],
    name: 'MaxAliasCountReached',
  },
  {
    type: 'error',
    inputs: [{ name: 'sender', internalType: 'address', type: 'address' }],
    name: 'OnlyStakingContract',
  },
  {
    type: 'error',
    inputs: [
      { name: 'amountStaked', internalType: 'uint256', type: 'uint256' },
      { name: 'minimumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'StakeMustBeGreaterThanMinimumStake',
  },
  {
    type: 'error',
    inputs: [
      { name: 'amountStaked', internalType: 'uint256', type: 'uint256' },
      { name: 'maximumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'StakeMustBeLessThanMaximumStake',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'StakerNotPermitted',
  },
  {
    type: 'error',
    inputs: [
      { name: 'yourBalance', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestedWithdrawlAmount',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'TryingToWithdrawMoreThanStaked',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'aliasAccount',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'AliasAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'aliasAccount',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'AliasRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMaxAliasCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MaxAliasCountSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMaximumStake',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MaximumStakeSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newMinimumStake',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'MinimumStakeSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'PermittedStakerAdded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'PermittedStakerRemoved',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'permittedStakersOn',
        internalType: 'bool',
        type: 'bool',
        indexed: false,
      },
    ],
    name: 'PermittedStakersOnChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ResolverContractAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'reward',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RewardPaid',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Staked',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newTokenRewardPerTokenPerEpoch',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'TokenRewardPerTokenPerEpochSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'aliasAccount',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ValidatorNotRewardedBecauseAlias',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ValidatorRewarded',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ValidatorTokensPenalized',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Withdrawn',
  },
  {
    type: 'function',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
    ],
    name: 'addAlias',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'addPermittedStaker',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'stakers', internalType: 'address[]', type: 'address[]' }],
    name: 'addPermittedStakers',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'checkStakingAmounts',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'getReward',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakingAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getTokenAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'isPermittedStaker',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'maximumStake',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'minimumStake',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'penalizeTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'permittedStakersOn',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'aliasAccount', internalType: 'address', type: 'address' },
    ],
    name: 'removeAlias',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'removePermittedStaker',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'staker', internalType: 'address', type: 'address' },
      { name: 'balance', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'restakePenaltyTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'rewardOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'rewardValidator',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newMaxAliasCount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setMaxAliasCount',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newMaximumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setMaximumStake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newMinimumStake', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setMinimumStake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'permitted', internalType: 'bool', type: 'bool' }],
    name: 'setPermittedStakersOn',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'stake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalStaked',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'balance', internalType: 'uint256', type: 'uint256' },
      { name: 'recipient', internalType: 'address', type: 'address' },
    ],
    name: 'transferPenaltyTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'account', internalType: 'address', type: 'address' },
    ],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'balance', internalType: 'uint256', type: 'uint256' }],
    name: 'withdrawPenaltyTokens',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// StakingDiamond
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingDiamondAbi = [
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotAddFunctionToDiamondThatAlreadyExists',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotAddSelectorsToZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveFunctionThatDoesNotExist',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotRemoveImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionThatDoesNotExists',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet',
  },
  {
    type: 'error',
    inputs: [
      { name: '_selectors', internalType: 'bytes4[]', type: 'bytes4[]' },
    ],
    name: 'CannotReplaceFunctionsFromFacetWithZeroAddress',
  },
  {
    type: 'error',
    inputs: [{ name: '_selector', internalType: 'bytes4', type: 'bytes4' }],
    name: 'CannotReplaceImmutableFunction',
  },
  {
    type: 'error',
    inputs: [{ name: '_action', internalType: 'uint8', type: 'uint8' }],
    name: 'IncorrectFacetCutAction',
  },
  {
    type: 'error',
    inputs: [
      {
        name: '_initializationContractAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'InitializationFunctionReverted',
  },
  {
    type: 'error',
    inputs: [
      { name: '_contractAddress', internalType: 'address', type: 'address' },
      { name: '_message', internalType: 'string', type: 'string' },
    ],
    name: 'NoBytecodeAtAddress',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'NoSelectorsProvidedForFacetForCut',
  },
  {
    type: 'error',
    inputs: [
      { name: '_user', internalType: 'address', type: 'address' },
      { name: '_contractOwner', internalType: 'address', type: 'address' },
    ],
    name: 'NotContractOwner',
  },
  {
    type: 'error',
    inputs: [
      { name: '_facetAddress', internalType: 'address', type: 'address' },
    ],
    name: 'RemoveFacetAddressMustBeZeroAddress',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
        indexed: false,
      },
      {
        name: '_init',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: '_calldata',
        internalType: 'bytes',
        type: 'bytes',
        indexed: false,
      },
    ],
    name: 'DiamondCut',
  },
  {
    type: 'function',
    inputs: [
      {
        name: '_diamondCut',
        internalType: 'struct IDiamond.FacetCut[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'action',
            internalType: 'enum IDiamond.FacetCutAction',
            type: 'uint8',
          },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
      { name: '_init', internalType: 'address', type: 'address' },
      { name: '_calldata', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'diamondCut',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: '_functionSelector', internalType: 'bytes4', type: 'bytes4' },
    ],
    name: 'facetAddress',
    outputs: [
      { name: 'facetAddress_', internalType: 'address', type: 'address' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facetAddresses',
    outputs: [
      { name: 'facetAddresses_', internalType: 'address[]', type: 'address[]' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_facet', internalType: 'address', type: 'address' }],
    name: 'facetFunctionSelectors',
    outputs: [
      {
        name: '_facetFunctionSelectors',
        internalType: 'bytes4[]',
        type: 'bytes4[]',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'facets',
    outputs: [
      {
        name: 'facets_',
        internalType: 'struct IDiamondLoupe.Facet[]',
        type: 'tuple[]',
        components: [
          { name: 'facetAddress', internalType: 'address', type: 'address' },
          {
            name: 'functionSelectors',
            internalType: 'bytes4[]',
            type: 'bytes4[]',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'previousOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'newOwner',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: 'owner_', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: '_newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'error', inputs: [], name: 'ActiveValidatorsCannotLeave' },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [],
    name: 'CannotKickBelowCurrentValidatorThreshold',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakingAddress', internalType: 'address', type: 'address' },
    ],
    name: 'CannotRejoinUntilNextEpochBecauseKicked',
  },
  {
    type: 'error',
    inputs: [
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'CannotReuseCommsKeys',
  },
  { type: 'error', inputs: [], name: 'CannotStakeZero' },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'CannotVoteTwice',
  },
  { type: 'error', inputs: [], name: 'CannotWithdrawZero' },
  {
    type: 'error',
    inputs: [{ name: 'nodeAddress', internalType: 'address', type: 'address' }],
    name: 'CouldNotMapNodeAddressToStakerAddress',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInActiveOrUnlockedOrPausedState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInActiveOrUnlockedState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInNextValidatorSetLockedOrReadyForNextEpochState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInNextValidatorSetLockedState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInReadyForNextEpochState',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'MustBeValidatorInNextEpochToKick',
  },
  {
    type: 'error',
    inputs: [
      { name: 'currentTimestamp', internalType: 'uint256', type: 'uint256' },
      { name: 'epochEndTime', internalType: 'uint256', type: 'uint256' },
      { name: 'timeout', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'NotEnoughTimeElapsedForTimeoutSinceLastEpoch',
  },
  {
    type: 'error',
    inputs: [
      { name: 'currentTimestamp', internalType: 'uint256', type: 'uint256' },
      { name: 'epochEndTime', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'NotEnoughTimeElapsedSinceLastEpoch',
  },
  {
    type: 'error',
    inputs: [
      { name: 'validatorCount', internalType: 'uint256', type: 'uint256' },
      {
        name: 'minimumValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'NotEnoughValidatorsInNextEpoch',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'currentReadyValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
      },
      {
        name: 'nextReadyValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
      },
      {
        name: 'minimumValidatorCountToBeReady',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'NotEnoughValidatorsReadyForNextEpoch',
  },
  {
    type: 'error',
    inputs: [
      { name: 'currentEpochNumber', internalType: 'uint256', type: 'uint256' },
      { name: 'receivedEpochNumber', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'SignaledReadyForWrongEpochNumber',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'StakerNotPermitted',
  },
  {
    type: 'error',
    inputs: [
      { name: 'yourBalance', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestedWithdrawlAmount',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'TryingToWithdrawMoreThanStaked',
  },
  {
    type: 'error',
    inputs: [
      { name: 'validator', internalType: 'address', type: 'address' },
      {
        name: 'validatorsInNextEpoch',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    name: 'ValidatorIsNotInNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'reason',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'config',
        internalType: 'struct LibStakingStorage.ComplaintConfig',
        type: 'tuple',
        components: [
          { name: 'tolerance', internalType: 'uint256', type: 'uint256' },
          { name: 'intervalSecs', internalType: 'uint256', type: 'uint256' },
          {
            name: 'kickPenaltyPercent',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
        indexed: false,
      },
    ],
    name: 'ComplaintConfigSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newTokenRewardPerTokenPerEpoch',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newKeyTypes',
        internalType: 'uint256[]',
        type: 'uint256[]',
        indexed: false,
      },
      {
        name: 'newMinimumValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMaxConcurrentRequests',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMaxTripleCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMinTripleCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newPeerCheckingIntervalSecs',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMaxTripleConcurrency',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newRpcHealthcheckEnabled',
        internalType: 'bool',
        type: 'bool',
        indexed: false,
      },
    ],
    name: 'ConfigSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newEpochEndTime',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'EpochEndTimeSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newEpochLength',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'EpochLengthSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newEpochTimeout',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'EpochTimeoutSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'reason',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newKickPenaltyPercent',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'KickPenaltyPercentSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'epochNumber',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ReadyForNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Recovered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RequestToJoin',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RequestToLeave',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverContractAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ResolverContractAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newDuration',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RewardsDurationUpdated',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newStakingTokenAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'StakingTokenSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newState',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
        indexed: false,
      },
    ],
    name: 'StateChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amountBurned',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ValidatorKickedFromNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ValidatorRejoinedNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'reporter',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'reason',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      { name: 'data', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'VotedToKickValidatorInNextEpoch',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'adminKickValidatorInNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'adminRejoinValidator',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'adminResetEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'amountToPenalize', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'adminSlashValidator',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'advanceEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'exit',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getReward',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'reason', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'kickValidatorInNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'lockValidatorsForNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'ip', internalType: 'uint32', type: 'uint32' },
      { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
      { name: 'port', internalType: 'uint32', type: 'uint32' },
      { name: 'nodeAddress', internalType: 'address', type: 'address' },
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'requestToJoin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'requestToLeave',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'requestToLeaveAsNode',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'reason', internalType: 'uint256', type: 'uint256' },
      {
        name: 'config',
        internalType: 'struct LibStakingStorage.ComplaintConfig',
        type: 'tuple',
        components: [
          { name: 'tolerance', internalType: 'uint256', type: 'uint256' },
          { name: 'intervalSecs', internalType: 'uint256', type: 'uint256' },
          {
            name: 'kickPenaltyPercent',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    name: 'setComplaintConfig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newConfig',
        internalType: 'struct LibStakingStorage.Config',
        type: 'tuple',
        components: [
          {
            name: 'tokenRewardPerTokenPerEpoch',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintTolerance',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'keyTypes', internalType: 'uint256[]', type: 'uint256[]' },
          {
            name: 'minimumValidatorCount',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxConcurrentRequests',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'maxTripleCount', internalType: 'uint256', type: 'uint256' },
          { name: 'minTripleCount', internalType: 'uint256', type: 'uint256' },
          {
            name: 'peerCheckingIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxTripleConcurrency',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'rpcHealthcheckEnabled', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    name: 'setConfig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newEpochEndTime', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setEpochEndTime',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newEpochLength', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setEpochLength',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newState',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'setEpochState',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newEpochTimeout', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setEpochTimeout',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'ip', internalType: 'uint32', type: 'uint32' },
      { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
      { name: 'port', internalType: 'uint32', type: 'uint32' },
      { name: 'nodeAddress', internalType: 'address', type: 'address' },
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setIpPortNodeAddressAndCommunicationPubKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'reason', internalType: 'uint256', type: 'uint256' },
      {
        name: 'newKickPenaltyPercent',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setKickPenaltyPercent',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'epochNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'signalReadyForNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'amount', internalType: 'uint256', type: 'uint256' }],
    name: 'stake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'ip', internalType: 'uint32', type: 'uint32' },
      { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
      { name: 'port', internalType: 'uint32', type: 'uint32' },
      { name: 'nodeAddress', internalType: 'address', type: 'address' },
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'stakeAndJoin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'amount', internalType: 'uint256', type: 'uint256' }],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'index',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'VersionRequirementsUpdated',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'checkVersion',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMaxVersion',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMaxVersionString',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMinVersion',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMinVersionString',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'setMaxVersion',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'setMinVersion',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'reason', internalType: 'uint256', type: 'uint256' }],
    name: 'complaintConfig',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.ComplaintConfig',
        type: 'tuple',
        components: [
          { name: 'tolerance', internalType: 'uint256', type: 'uint256' },
          { name: 'intervalSecs', internalType: 'uint256', type: 'uint256' },
          {
            name: 'kickPenaltyPercent',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'config',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Config',
        type: 'tuple',
        components: [
          {
            name: 'tokenRewardPerTokenPerEpoch',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintTolerance',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'keyTypes', internalType: 'uint256[]', type: 'uint256[]' },
          {
            name: 'minimumValidatorCount',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxConcurrentRequests',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'maxTripleCount', internalType: 'uint256', type: 'uint256' },
          { name: 'minTripleCount', internalType: 'uint256', type: 'uint256' },
          {
            name: 'peerCheckingIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxTripleConcurrency',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'rpcHealthcheckEnabled', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'countOfCurrentValidatorsReadyForNextEpoch',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'countOfNextValidatorsReadyForNextEpoch',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'currentValidatorCountForConsensus',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'epoch',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Epoch',
        type: 'tuple',
        components: [
          { name: 'epochLength', internalType: 'uint256', type: 'uint256' },
          { name: 'number', internalType: 'uint256', type: 'uint256' },
          { name: 'endTime', internalType: 'uint256', type: 'uint256' },
          { name: 'retries', internalType: 'uint256', type: 'uint256' },
          { name: 'timeout', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveUnkickedValidatorStructs',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveUnkickedValidatorStructsAndCounts',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Epoch',
        type: 'tuple',
        components: [
          { name: 'epochLength', internalType: 'uint256', type: 'uint256' },
          { name: 'number', internalType: 'uint256', type: 'uint256' },
          { name: 'endTime', internalType: 'uint256', type: 'uint256' },
          { name: 'retries', internalType: 'uint256', type: 'uint256' },
          { name: 'timeout', internalType: 'uint256', type: 'uint256' },
        ],
      },
      { name: '', internalType: 'uint256', type: 'uint256' },
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveUnkickedValidators',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getKeyTypes',
    outputs: [{ name: '', internalType: 'uint256[]', type: 'uint256[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getKickedValidators',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'addresses', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'getNodeStakerAddressMappings',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.AddressMapping[]',
        type: 'tuple[]',
        components: [
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'stakerAddress', internalType: 'address', type: 'address' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakingBalancesAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getTokenAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsInCurrentEpoch',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsInCurrentEpochLength',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsInNextEpoch',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'addresses', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'getValidatorsStructs',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsStructsInCurrentEpoch',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsStructsInNextEpoch',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'epochNumber', internalType: 'uint256', type: 'uint256' },
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'voterStakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'getVotingStatusToKickValidator',
    outputs: [
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bool', type: 'bool' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'isActiveValidator',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'isActiveValidatorByNodeAddress',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isReadyForNextEpoch',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'reason', internalType: 'uint256', type: 'uint256' }],
    name: 'kickPenaltyPercentByReason',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'nextValidatorCountForConsensus',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'nodeAddress', internalType: 'address', type: 'address' }],
    name: 'nodeAddressToStakerAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'readyForNextEpoch',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'shouldKickValidator',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'state',
    outputs: [
      {
        name: '',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'validators',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator',
        type: 'tuple',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// StakingFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingFacetAbi = [
  { type: 'error', inputs: [], name: 'ActiveValidatorsCannotLeave' },
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'error',
    inputs: [],
    name: 'CannotKickBelowCurrentValidatorThreshold',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakingAddress', internalType: 'address', type: 'address' },
    ],
    name: 'CannotRejoinUntilNextEpochBecauseKicked',
  },
  {
    type: 'error',
    inputs: [
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'CannotReuseCommsKeys',
  },
  { type: 'error', inputs: [], name: 'CannotStakeZero' },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'CannotVoteTwice',
  },
  { type: 'error', inputs: [], name: 'CannotWithdrawZero' },
  {
    type: 'error',
    inputs: [{ name: 'nodeAddress', internalType: 'address', type: 'address' }],
    name: 'CouldNotMapNodeAddressToStakerAddress',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInActiveOrUnlockedOrPausedState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInActiveOrUnlockedState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInNextValidatorSetLockedOrReadyForNextEpochState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInNextValidatorSetLockedState',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'state',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'MustBeInReadyForNextEpochState',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'MustBeValidatorInNextEpochToKick',
  },
  {
    type: 'error',
    inputs: [
      { name: 'currentTimestamp', internalType: 'uint256', type: 'uint256' },
      { name: 'epochEndTime', internalType: 'uint256', type: 'uint256' },
      { name: 'timeout', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'NotEnoughTimeElapsedForTimeoutSinceLastEpoch',
  },
  {
    type: 'error',
    inputs: [
      { name: 'currentTimestamp', internalType: 'uint256', type: 'uint256' },
      { name: 'epochEndTime', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'NotEnoughTimeElapsedSinceLastEpoch',
  },
  {
    type: 'error',
    inputs: [
      { name: 'validatorCount', internalType: 'uint256', type: 'uint256' },
      {
        name: 'minimumValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'NotEnoughValidatorsInNextEpoch',
  },
  {
    type: 'error',
    inputs: [
      {
        name: 'currentReadyValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
      },
      {
        name: 'nextReadyValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
      },
      {
        name: 'minimumValidatorCountToBeReady',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'NotEnoughValidatorsReadyForNextEpoch',
  },
  {
    type: 'error',
    inputs: [
      { name: 'currentEpochNumber', internalType: 'uint256', type: 'uint256' },
      { name: 'receivedEpochNumber', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'SignaledReadyForWrongEpochNumber',
  },
  {
    type: 'error',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'StakerNotPermitted',
  },
  {
    type: 'error',
    inputs: [
      { name: 'yourBalance', internalType: 'uint256', type: 'uint256' },
      {
        name: 'requestedWithdrawlAmount',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'TryingToWithdrawMoreThanStaked',
  },
  {
    type: 'error',
    inputs: [
      { name: 'validator', internalType: 'address', type: 'address' },
      {
        name: 'validatorsInNextEpoch',
        internalType: 'address[]',
        type: 'address[]',
      },
    ],
    name: 'ValidatorIsNotInNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'reason',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'config',
        internalType: 'struct LibStakingStorage.ComplaintConfig',
        type: 'tuple',
        components: [
          { name: 'tolerance', internalType: 'uint256', type: 'uint256' },
          { name: 'intervalSecs', internalType: 'uint256', type: 'uint256' },
          {
            name: 'kickPenaltyPercent',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
        indexed: false,
      },
    ],
    name: 'ComplaintConfigSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newTokenRewardPerTokenPerEpoch',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newKeyTypes',
        internalType: 'uint256[]',
        type: 'uint256[]',
        indexed: false,
      },
      {
        name: 'newMinimumValidatorCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMaxConcurrentRequests',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMaxTripleCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMinTripleCount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newPeerCheckingIntervalSecs',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newMaxTripleConcurrency',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newRpcHealthcheckEnabled',
        internalType: 'bool',
        type: 'bool',
        indexed: false,
      },
    ],
    name: 'ConfigSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newEpochEndTime',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'EpochEndTimeSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newEpochLength',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'EpochLengthSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newEpochTimeout',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'EpochTimeoutSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'reason',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'newKickPenaltyPercent',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'KickPenaltyPercentSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'epochNumber',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ReadyForNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'token',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
      {
        name: 'amount',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'Recovered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RequestToJoin',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
    ],
    name: 'RequestToLeave',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newResolverContractAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ResolverContractAddressSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newDuration',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'RewardsDurationUpdated',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newStakingTokenAddress',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'StakingTokenSet',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'newState',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
        indexed: false,
      },
    ],
    name: 'StateChanged',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'amountBurned',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
    ],
    name: 'ValidatorKickedFromNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'staker',
        internalType: 'address',
        type: 'address',
        indexed: false,
      },
    ],
    name: 'ValidatorRejoinedNextEpoch',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'reporter',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
        indexed: true,
      },
      {
        name: 'reason',
        internalType: 'uint256',
        type: 'uint256',
        indexed: true,
      },
      { name: 'data', internalType: 'bytes', type: 'bytes', indexed: false },
    ],
    name: 'VotedToKickValidatorInNextEpoch',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
    ],
    name: 'adminKickValidatorInNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'staker', internalType: 'address', type: 'address' }],
    name: 'adminRejoinValidator',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'adminResetEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'amountToPenalize', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'adminSlashValidator',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'advanceEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'exit',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getReward',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'reason', internalType: 'uint256', type: 'uint256' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'kickValidatorInNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'lockValidatorsForNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'ip', internalType: 'uint32', type: 'uint32' },
      { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
      { name: 'port', internalType: 'uint32', type: 'uint32' },
      { name: 'nodeAddress', internalType: 'address', type: 'address' },
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'requestToJoin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'requestToLeave',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'requestToLeaveAsNode',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'reason', internalType: 'uint256', type: 'uint256' },
      {
        name: 'config',
        internalType: 'struct LibStakingStorage.ComplaintConfig',
        type: 'tuple',
        components: [
          { name: 'tolerance', internalType: 'uint256', type: 'uint256' },
          { name: 'intervalSecs', internalType: 'uint256', type: 'uint256' },
          {
            name: 'kickPenaltyPercent',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    name: 'setComplaintConfig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newConfig',
        internalType: 'struct LibStakingStorage.Config',
        type: 'tuple',
        components: [
          {
            name: 'tokenRewardPerTokenPerEpoch',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintTolerance',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'keyTypes', internalType: 'uint256[]', type: 'uint256[]' },
          {
            name: 'minimumValidatorCount',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxConcurrentRequests',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'maxTripleCount', internalType: 'uint256', type: 'uint256' },
          { name: 'minTripleCount', internalType: 'uint256', type: 'uint256' },
          {
            name: 'peerCheckingIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxTripleConcurrency',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'rpcHealthcheckEnabled', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    name: 'setConfig',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newResolverAddress', internalType: 'address', type: 'address' },
    ],
    name: 'setContractResolver',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newEpochEndTime', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setEpochEndTime',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newEpochLength', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setEpochLength',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'newState',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    name: 'setEpochState',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'newEpochTimeout', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setEpochTimeout',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'ip', internalType: 'uint32', type: 'uint32' },
      { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
      { name: 'port', internalType: 'uint32', type: 'uint32' },
      { name: 'nodeAddress', internalType: 'address', type: 'address' },
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setIpPortNodeAddressAndCommunicationPubKeys',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'reason', internalType: 'uint256', type: 'uint256' },
      {
        name: 'newKickPenaltyPercent',
        internalType: 'uint256',
        type: 'uint256',
      },
    ],
    name: 'setKickPenaltyPercent',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'epochNumber', internalType: 'uint256', type: 'uint256' }],
    name: 'signalReadyForNextEpoch',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'amount', internalType: 'uint256', type: 'uint256' }],
    name: 'stake',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'ip', internalType: 'uint32', type: 'uint32' },
      { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
      { name: 'port', internalType: 'uint32', type: 'uint32' },
      { name: 'nodeAddress', internalType: 'address', type: 'address' },
      { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
      { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'stakeAndJoin',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'amount', internalType: 'uint256', type: 'uint256' }],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// StakingVersionFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingVersionFacetAbi = [
  { type: 'error', inputs: [], name: 'CallerNotOwner' },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      {
        name: 'index',
        internalType: 'uint256',
        type: 'uint256',
        indexed: false,
      },
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
        indexed: false,
      },
    ],
    name: 'VersionRequirementsUpdated',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'checkVersion',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMaxVersion',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMaxVersionString',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMinVersion',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getMinVersionString',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'setMaxVersion',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      {
        name: 'version',
        internalType: 'struct LibStakingStorage.Version',
        type: 'tuple',
        components: [
          { name: 'major', internalType: 'uint256', type: 'uint256' },
          { name: 'minor', internalType: 'uint256', type: 'uint256' },
          { name: 'patch', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'setMinVersion',
    outputs: [],
    stateMutability: 'nonpayable',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// StakingViewsFacet
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const stakingViewsFacetAbi = [
  {
    type: 'function',
    inputs: [{ name: 'reason', internalType: 'uint256', type: 'uint256' }],
    name: 'complaintConfig',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.ComplaintConfig',
        type: 'tuple',
        components: [
          { name: 'tolerance', internalType: 'uint256', type: 'uint256' },
          { name: 'intervalSecs', internalType: 'uint256', type: 'uint256' },
          {
            name: 'kickPenaltyPercent',
            internalType: 'uint256',
            type: 'uint256',
          },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'config',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Config',
        type: 'tuple',
        components: [
          {
            name: 'tokenRewardPerTokenPerEpoch',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintTolerance',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'DEPRECATED_complaintIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'keyTypes', internalType: 'uint256[]', type: 'uint256[]' },
          {
            name: 'minimumValidatorCount',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxConcurrentRequests',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'maxTripleCount', internalType: 'uint256', type: 'uint256' },
          { name: 'minTripleCount', internalType: 'uint256', type: 'uint256' },
          {
            name: 'peerCheckingIntervalSecs',
            internalType: 'uint256',
            type: 'uint256',
          },
          {
            name: 'maxTripleConcurrency',
            internalType: 'uint256',
            type: 'uint256',
          },
          { name: 'rpcHealthcheckEnabled', internalType: 'bool', type: 'bool' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'contractResolver',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'countOfCurrentValidatorsReadyForNextEpoch',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'countOfNextValidatorsReadyForNextEpoch',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'currentValidatorCountForConsensus',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'epoch',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Epoch',
        type: 'tuple',
        components: [
          { name: 'epochLength', internalType: 'uint256', type: 'uint256' },
          { name: 'number', internalType: 'uint256', type: 'uint256' },
          { name: 'endTime', internalType: 'uint256', type: 'uint256' },
          { name: 'retries', internalType: 'uint256', type: 'uint256' },
          { name: 'timeout', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveUnkickedValidatorStructs',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveUnkickedValidatorStructsAndCounts',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Epoch',
        type: 'tuple',
        components: [
          { name: 'epochLength', internalType: 'uint256', type: 'uint256' },
          { name: 'number', internalType: 'uint256', type: 'uint256' },
          { name: 'endTime', internalType: 'uint256', type: 'uint256' },
          { name: 'retries', internalType: 'uint256', type: 'uint256' },
          { name: 'timeout', internalType: 'uint256', type: 'uint256' },
        ],
      },
      { name: '', internalType: 'uint256', type: 'uint256' },
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getActiveUnkickedValidators',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getKeyTypes',
    outputs: [{ name: '', internalType: 'uint256[]', type: 'uint256[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getKickedValidators',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'addresses', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'getNodeStakerAddressMappings',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.AddressMapping[]',
        type: 'tuple[]',
        components: [
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'stakerAddress', internalType: 'address', type: 'address' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getStakingBalancesAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getTokenAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsInCurrentEpoch',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsInCurrentEpochLength',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsInNextEpoch',
    outputs: [{ name: '', internalType: 'address[]', type: 'address[]' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'addresses', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'getValidatorsStructs',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsStructsInCurrentEpoch',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'getValidatorsStructsInNextEpoch',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator[]',
        type: 'tuple[]',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'epochNumber', internalType: 'uint256', type: 'uint256' },
      {
        name: 'validatorStakerAddress',
        internalType: 'address',
        type: 'address',
      },
      { name: 'voterStakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'getVotingStatusToKickValidator',
    outputs: [
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bool', type: 'bool' },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'isActiveValidator',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'isActiveValidatorByNodeAddress',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'isReadyForNextEpoch',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'reason', internalType: 'uint256', type: 'uint256' }],
    name: 'kickPenaltyPercentByReason',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'nextValidatorCountForConsensus',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [{ name: 'nodeAddress', internalType: 'address', type: 'address' }],
    name: 'nodeAddressToStakerAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'readyForNextEpoch',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'shouldKickValidator',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'state',
    outputs: [
      {
        name: '',
        internalType: 'enum LibStakingStorage.States',
        type: 'uint8',
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'stakerAddress', internalType: 'address', type: 'address' },
    ],
    name: 'validators',
    outputs: [
      {
        name: '',
        internalType: 'struct LibStakingStorage.Validator',
        type: 'tuple',
        components: [
          { name: 'ip', internalType: 'uint32', type: 'uint32' },
          { name: 'ipv6', internalType: 'uint128', type: 'uint128' },
          { name: 'port', internalType: 'uint32', type: 'uint32' },
          { name: 'nodeAddress', internalType: 'address', type: 'address' },
          { name: 'reward', internalType: 'uint256', type: 'uint256' },
          { name: 'senderPubKey', internalType: 'uint256', type: 'uint256' },
          { name: 'receiverPubKey', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// WLIT
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const wlitAbi = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'src', internalType: 'address', type: 'address', indexed: true },
      { name: 'guy', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'dst', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Deposit',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'src', internalType: 'address', type: 'address', indexed: true },
      { name: 'dst', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'src', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Withdrawal',
  },
  { type: 'fallback', stateMutability: 'payable' },
  {
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'guy', internalType: 'address', type: 'address' },
      { name: 'wad', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'account', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'burnFrom',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'deposit',
    outputs: [],
    stateMutability: 'payable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    inputs: [
      { name: 'dst', internalType: 'address', type: 'address' },
      { name: 'wad', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [
      { name: 'src', internalType: 'address', type: 'address' },
      { name: 'dst', internalType: 'address', type: 'address' },
      { name: 'wad', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: 'wad', internalType: 'uint256', type: 'uint256' }],
    name: 'withdraw',
    outputs: [],
    stateMutability: 'nonpayable',
  },
  { type: 'receive', stateMutability: 'payable' },
] as const;
