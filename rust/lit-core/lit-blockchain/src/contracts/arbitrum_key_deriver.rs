pub use arbitrum_key_deriver::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod arbitrum_key_deriver {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/ArbitrumKeyDeriver.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_resolver"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_env"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("enum ContractResolver.Env"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HD_KDF_K256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HD_KDF_K256"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HD_KDF_P256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HD_KDF_P256"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeHDPubKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeHDPubKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("derivedKeyId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootHDKeys"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPubkeyRouter.RootKey[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("contractResolver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contractResolver"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ContractResolver",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("env"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("env"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setContractResolver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setContractResolver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "contractResolverAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ARBITRUMKEYDERIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0 \xC98\x03\x80b\0 \xC9\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x03\x9EV[b\0\0i\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3b\0\x01\x11` \x1B` \x1CV[b\0\0\x9B\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x80b\0\x01'` \x1B` \x1CV[\x81`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x02\x81\x11\x15b\0\x01\x04Wb\0\x01\x03b\0\x03\xE5V[[\x02\x17\x90UPPPb\0\x04\x14V[b\0\x01#\x82\x82b\0\x01\x8A` \x1B` \x1CV[PPV[`\0b\0\x01:\x83b\0\x02{` \x1B` \x1CV[\x90P\x81`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01\x81\x90UP\x81\x81\x84\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[b\0\x01\x9C\x82\x82b\0\x02\x9A` \x1B` \x1CV[b\0\x02wW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPb\0\x02\x1Cb\0\x03\x04` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x03>\x82b\0\x03\x11V[\x90P\x91\x90PV[b\0\x03P\x81b\0\x031V[\x81\x14b\0\x03\\W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03p\x81b\0\x03EV[\x92\x91PPV[`\x03\x81\x10b\0\x03\x84W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\x98\x81b\0\x03vV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x03\xB8Wb\0\x03\xB7b\0\x03\x0CV[[`\0b\0\x03\xC8\x85\x82\x86\x01b\0\x03_V[\x92PP` b\0\x03\xDB\x85\x82\x86\x01b\0\x03\x87V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[a\x1C\xA5\x80b\0\x04$`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x9D\xCA\x002\x11a\0\x8CW\x80c\xB2N\xD3\x08\x11a\0fW\x80c\xB2N\xD3\x08\x14a\x02`W\x80c\xD5Gt\x1F\x14a\x02~W\x80c\xF9]q\xB1\x14a\x02\x9AW\x80c\xFE\x89\xC9p\x14a\x02\xB6Wa\0\xEAV[\x80c\x9D\xCA\x002\x14a\x01\xF3W\x80c\xA2\x17\xFD\xDF\x14a\x02\x11W\x80c\xA3,+\x99\x14a\x02/Wa\0\xEAV[\x80c6V\x8A\xBE\x11a\0\xC8W\x80c6V\x8A\xBE\x14a\x01kW\x80cP\xD1{^\x14a\x01\x87W\x80cu\xB28\xFC\x14a\x01\xA5W\x80c\x91\xD1HT\x14a\x01\xC3Wa\0\xEAV[\x80c\x01\xFF\xC9\xA7\x14a\0\xEFW\x80c$\x8A\x9C\xA3\x14a\x01\x1FW\x80c//\xF1]\x14a\x01OW[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x0E\xD5V[a\x02\xD4V[`@Qa\x01\x16\x91\x90a\x0F\x1DV[`@Q\x80\x91\x03\x90\xF3[a\x019`\x04\x806\x03\x81\x01\x90a\x014\x91\x90a\x0FnV[a\x03NV[`@Qa\x01F\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x01i`\x04\x806\x03\x81\x01\x90a\x01d\x91\x90a\x10#V[a\x03mV[\0[a\x01\x85`\x04\x806\x03\x81\x01\x90a\x01\x80\x91\x90a\x10#V[a\x03\x8EV[\0[a\x01\x8Fa\x04\x11V[`@Qa\x01\x9C\x91\x90a\x10\xC2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xADa\x047V[`@Qa\x01\xBA\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDD`\x04\x806\x03\x81\x01\x90a\x01\xD8\x91\x90a\x10#V[a\x04[V[`@Qa\x01\xEA\x91\x90a\x0F\x1DV[`@Q\x80\x91\x03\x90\xF3[a\x01\xFBa\x04\xC5V[`@Qa\x02\x08\x91\x90a\x11TV[`@Q\x80\x91\x03\x90\xF3[a\x02\x19a\x04\xD8V[`@Qa\x02&\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x02I`\x04\x806\x03\x81\x01\x90a\x02D\x91\x90a\x14GV[a\x04\xDFV[`@Qa\x02W\x92\x91\x90a\x155V[`@Q\x80\x91\x03\x90\xF3[a\x02ha\x06\xD0V[`@Qa\x02u\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x02\x98`\x04\x806\x03\x81\x01\x90a\x02\x93\x91\x90a\x10#V[a\x06\xF4V[\0[a\x02\xB4`\x04\x806\x03\x81\x01\x90a\x02\xAF\x91\x90a\x15eV[a\x07\x15V[\0[a\x02\xBEa\x07\x84V[`@Qa\x02\xCB\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03GWPa\x03F\x82a\x07\xA7V[[\x90P\x91\x90PV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x03v\x82a\x03NV[a\x03\x7F\x81a\x08\x11V[a\x03\x89\x83\x83a\x08%V[PPPV[a\x03\x96a\t\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xFA\x90a\x16\x15V[`@Q\x80\x91\x03\x90\xFD[a\x04\r\x82\x82a\t\rV[PPV[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80\x1B\x81V[`\0```\0a\x04\xF0\x86\x86\x86a\t\xEEV[\x90P`\0\x80`\xF8\x1B\x82`\0\x81Q\x81\x10a\x05\x0CWa\x05\x0Ba\x165V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x03a\x05fW\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9\x90Pa\x05\x89V[~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd\x90P[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16\x83`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xF7\x92\x91\x90a\x16dV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x068\x91\x90a\x16\xA2V[\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xECr3g\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06u\x91\x90a\x16\xCFV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xBB\x91\x90a\x17aV[\x90P`\x01\x81\x95P\x95PPPPP\x93P\x93\x91PPV[\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9\x81V[a\x06\xFD\x82a\x03NV[a\x07\x06\x81a\x08\x11V[a\x07\x10\x83\x83a\t\rV[PPPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x07?\x81a\x08\x11V[\x81`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a\x08\"\x81a\x08\x1Da\t\x05V[a\x0B{V[PV[a\x08/\x82\x82a\x04[V[a\t\x01W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x08\xA6a\t\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\t\x17\x82\x82a\x04[V[\x15a\t\xEAW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\t\x8Fa\t\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[```\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x0CWa\n\x0Ba\x11\x85V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\n>W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0\x80[\x85Q\x81\x10\x15a\n\xD7W\x84\x86\x82\x81Q\x81\x10a\nbWa\naa\x165V[[` \x02` \x01\x01Q` \x01Q\x03a\n\xC4W\x82\x86\x82\x81Q\x81\x10a\n\x87Wa\n\x86a\x165V[[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\n\xA4\x92\x91\x90a\x17\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\n\xC0\x90a\x18IV[\x92PP[\x80\x80a\n\xCF\x90a\x18uV[\x91PPa\nEV[P`\x02\x84\x03a\n\xE9W`\x01\x93Pa\n\xF7V[`\x03\x84\x03a\n\xF6W`\0\x93P[[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\x1CE`+\x919\x90P`\0\x85`\xF8\x1B\x90P`\0` `\xFF\x16`\xE0\x1B\x90P`\0\x83Q`\xE0\x1B\x90P`\0\x85`\xE0\x1B\x90P`\0\x84\x84\x8D\x85\x89\x86\x8D`@Q` \x01a\x0BX\x97\x96\x95\x94\x93\x92\x91\x90a\x19LV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x98PPPPPPPPP\x93\x92PPPV[a\x0B\x85\x82\x82a\x04[V[a\x0B\xFCWa\x0B\x92\x81a\x0C\0V[a\x0B\xA0\x83`\0\x1C` a\x0C-V[`@Q` \x01a\x0B\xB1\x92\x91\x90a\x1A\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xF3\x91\x90a\x1B\x17V[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x0C&\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x0C-V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a\x0C@\x91\x90a\x1B9V[a\x0CJ\x91\x90a\x1B{V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CcWa\x0Cba\x11\x85V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0C\x95W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x0C\xCDWa\x0C\xCCa\x165V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\r1Wa\r0a\x165V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\rq\x91\x90a\x1B9V[a\r{\x91\x90a\x1B{V[\x90P[`\x01\x81\x11\x15a\x0E\x1BW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\r\xBDWa\r\xBCa\x165V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\r\xD4Wa\r\xD3a\x165V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x0E\x14\x90a\x1B\xAFV[\x90Pa\r~V[P`\0\x84\x14a\x0E_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EV\x90a\x1C$V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x0E\xB2\x81a\x0E}V[\x81\x14a\x0E\xBDW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xCF\x81a\x0E\xA9V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0E\xEBWa\x0E\xEAa\x0EsV[[`\0a\x0E\xF9\x84\x82\x85\x01a\x0E\xC0V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0F\x17\x81a\x0F\x02V[\x82RPPV[`\0` \x82\x01\x90Pa\x0F2`\0\x83\x01\x84a\x0F\x0EV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0FK\x81a\x0F8V[\x81\x14a\x0FVW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0Fh\x81a\x0FBV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\x84Wa\x0F\x83a\x0EsV[[`\0a\x0F\x92\x84\x82\x85\x01a\x0FYV[\x91PP\x92\x91PPV[a\x0F\xA4\x81a\x0F8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0F\xBF`\0\x83\x01\x84a\x0F\x9BV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0F\xF0\x82a\x0F\xC5V[\x90P\x91\x90PV[a\x10\0\x81a\x0F\xE5V[\x81\x14a\x10\x0BW`\0\x80\xFD[PV[`\0\x815\x90Pa\x10\x1D\x81a\x0F\xF7V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10:Wa\x109a\x0EsV[[`\0a\x10H\x85\x82\x86\x01a\x0FYV[\x92PP` a\x10Y\x85\x82\x86\x01a\x10\x0EV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x10\x88a\x10\x83a\x10~\x84a\x0F\xC5V[a\x10cV[a\x0F\xC5V[\x90P\x91\x90PV[`\0a\x10\x9A\x82a\x10mV[\x90P\x91\x90PV[`\0a\x10\xAC\x82a\x10\x8FV[\x90P\x91\x90PV[a\x10\xBC\x81a\x10\xA1V[\x82RPPV[`\0` \x82\x01\x90Pa\x10\xD7`\0\x83\x01\x84a\x10\xB3V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x11\x1DWa\x11\x1Ca\x10\xDDV[[PV[`\0\x81\x90Pa\x11.\x82a\x11\x0CV[\x91\x90PV[`\0a\x11>\x82a\x11 V[\x90P\x91\x90PV[a\x11N\x81a\x113V[\x82RPPV[`\0` \x82\x01\x90Pa\x11i`\0\x83\x01\x84a\x11EV[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x11\xBD\x82a\x11tV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x11\xDCWa\x11\xDBa\x11\x85V[[\x80`@RPPPV[`\0a\x11\xEFa\x0EiV[\x90Pa\x11\xFB\x82\x82a\x11\xB4V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12\x1BWa\x12\x1Aa\x11\x85V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12[Wa\x12Za\x11\x85V[[a\x12d\x82a\x11tV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x12\x93a\x12\x8E\x84a\x12@V[a\x11\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x12\xAFWa\x12\xAEa\x12;V[[a\x12\xBA\x84\x82\x85a\x12qV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x12\xD7Wa\x12\xD6a\x11oV[[\x815a\x12\xE7\x84\x82` \x86\x01a\x12\x80V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x13\x03\x81a\x12\xF0V[\x81\x14a\x13\x0EW`\0\x80\xFD[PV[`\0\x815\x90Pa\x13 \x81a\x12\xFAV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x13<Wa\x13;a\x121V[[a\x13F`@a\x11\xE5V[\x90P`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13fWa\x13ea\x126V[[a\x13r\x84\x82\x85\x01a\x12\xC2V[`\0\x83\x01RP` a\x13\x86\x84\x82\x85\x01a\x13\x11V[` \x83\x01RP\x92\x91PPV[`\0a\x13\xA5a\x13\xA0\x84a\x12\0V[a\x11\xE5V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x13\xC8Wa\x13\xC7a\x12,V[[\x83[\x81\x81\x10\x15a\x14\x0FW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xEDWa\x13\xECa\x11oV[[\x80\x86\x01a\x13\xFA\x89\x82a\x13&V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x13\xCAV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14.Wa\x14-a\x11oV[[\x815a\x14>\x84\x82` \x86\x01a\x13\x92V[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14`Wa\x14_a\x0EsV[[`\0a\x14n\x86\x82\x87\x01a\x0FYV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x8FWa\x14\x8Ea\x0ExV[[a\x14\x9B\x86\x82\x87\x01a\x14\x19V[\x92PP`@a\x14\xAC\x86\x82\x87\x01a\x13\x11V[\x91PP\x92P\x92P\x92V[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x14\xF0W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x14\xD5V[`\0\x84\x84\x01RPPPPV[`\0a\x15\x07\x82a\x14\xB6V[a\x15\x11\x81\x85a\x14\xC1V[\x93Pa\x15!\x81\x85` \x86\x01a\x14\xD2V[a\x15*\x81a\x11tV[\x84\x01\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa\x15J`\0\x83\x01\x85a\x0F\x0EV[\x81\x81\x03` \x83\x01Ra\x15\\\x81\x84a\x14\xFCV[\x90P\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x15{Wa\x15za\x0EsV[[`\0a\x15\x89\x84\x82\x85\x01a\x10\x0EV[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x15\xFF`/\x83a\x15\x92V[\x91Pa\x16\n\x82a\x15\xA3V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x16.\x81a\x15\xF2V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x01\x90Pa\x16y`\0\x83\x01\x85a\x0F\x9BV[a\x16\x86` \x83\x01\x84a\x11EV[\x93\x92PPPV[`\0\x81Q\x90Pa\x16\x9C\x81a\x0F\xF7V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xB8Wa\x16\xB7a\x0EsV[[`\0a\x16\xC6\x84\x82\x85\x01a\x16\x8DV[\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x16\xE9\x81\x84a\x14\xFCV[\x90P\x92\x91PPV[`\0a\x17\x04a\x16\xFF\x84a\x12@V[a\x11\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x17 Wa\x17\x1Fa\x12;V[[a\x17+\x84\x82\x85a\x14\xD2V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x17HWa\x17Ga\x11oV[[\x81Qa\x17X\x84\x82` \x86\x01a\x16\xF1V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17wWa\x17va\x0EsV[[`\0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x95Wa\x17\x94a\x0ExV[[a\x17\xA1\x84\x82\x85\x01a\x173V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[`\0a\x17\xC0\x82a\x14\xB6V[a\x17\xCA\x81\x85a\x17\xAAV[\x93Pa\x17\xDA\x81\x85` \x86\x01a\x14\xD2V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x17\xF2\x82\x85a\x17\xB5V[\x91Pa\x17\xFE\x82\x84a\x17\xB5V[\x91P\x81\x90P\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x18T\x82a\x189V[\x91Pc\xFF\xFF\xFF\xFF\x82\x03a\x18jWa\x18ia\x18\nV[[`\x01\x82\x01\x90P\x91\x90PV[`\0a\x18\x80\x82a\x12\xF0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x18\xB2Wa\x18\xB1a\x18\nV[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x19\x04a\x18\xFF\x82a\x18\xBDV[a\x18\xE9V[\x82RPPV[`\0\x81\x90P\x91\x90PV[a\x19%a\x19 \x82a\x0E}V[a\x19\nV[\x82RPPV[`\0\x81\x90P\x91\x90PV[a\x19Fa\x19A\x82a\x0F8V[a\x19+V[\x82RPPV[`\0a\x19X\x82\x8Aa\x18\xF3V[`\x01\x82\x01\x91Pa\x19h\x82\x89a\x19\x14V[`\x04\x82\x01\x91Pa\x19x\x82\x88a\x195V[` \x82\x01\x91Pa\x19\x88\x82\x87a\x19\x14V[`\x04\x82\x01\x91Pa\x19\x98\x82\x86a\x17\xB5V[\x91Pa\x19\xA4\x82\x85a\x19\x14V[`\x04\x82\x01\x91Pa\x19\xB4\x82\x84a\x17\xB5V[\x91P\x81\x90P\x98\x97PPPPPPPPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A\x06`\x17\x83a\x19\xC5V[\x91Pa\x1A\x11\x82a\x19\xD0V[`\x17\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0a\x1A2\x82a\x1A\x1CV[a\x1A<\x81\x85a\x19\xC5V[\x93Pa\x1AL\x81\x85` \x86\x01a\x14\xD2V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A\x8E`\x11\x83a\x19\xC5V[\x91Pa\x1A\x99\x82a\x1AXV[`\x11\x82\x01\x90P\x91\x90PV[`\0a\x1A\xAF\x82a\x19\xF9V[\x91Pa\x1A\xBB\x82\x85a\x1A'V[\x91Pa\x1A\xC6\x82a\x1A\x81V[\x91Pa\x1A\xD2\x82\x84a\x1A'V[\x91P\x81\x90P\x93\x92PPPV[`\0a\x1A\xE9\x82a\x1A\x1CV[a\x1A\xF3\x81\x85a\x15\x92V[\x93Pa\x1B\x03\x81\x85` \x86\x01a\x14\xD2V[a\x1B\x0C\x81a\x11tV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1B1\x81\x84a\x1A\xDEV[\x90P\x92\x91PPV[`\0a\x1BD\x82a\x12\xF0V[\x91Pa\x1BO\x83a\x12\xF0V[\x92P\x82\x82\x02a\x1B]\x81a\x12\xF0V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1BtWa\x1Bsa\x18\nV[[P\x92\x91PPV[`\0a\x1B\x86\x82a\x12\xF0V[\x91Pa\x1B\x91\x83a\x12\xF0V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1B\xA9Wa\x1B\xA8a\x18\nV[[\x92\x91PPV[`\0a\x1B\xBA\x82a\x12\xF0V[\x91P`\0\x82\x03a\x1B\xCDWa\x1B\xCCa\x18\nV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a\x1C\x0E` \x83a\x15\x92V[\x91Pa\x1C\x19\x82a\x1B\xD8V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C=\x81a\x1C\x01V[\x90P\x91\x90PV\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 p\xF3\x19\"4\x8D\x94\xF9_\xF2m\xF5\x0C?P0\xB1\x8F\xE6{2\xCA\xE6\xE3\xC7\xA3\xFFZ\xA1\x93K\x11dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static ARBITRUMKEYDERIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x9D\xCA\x002\x11a\0\x8CW\x80c\xB2N\xD3\x08\x11a\0fW\x80c\xB2N\xD3\x08\x14a\x02`W\x80c\xD5Gt\x1F\x14a\x02~W\x80c\xF9]q\xB1\x14a\x02\x9AW\x80c\xFE\x89\xC9p\x14a\x02\xB6Wa\0\xEAV[\x80c\x9D\xCA\x002\x14a\x01\xF3W\x80c\xA2\x17\xFD\xDF\x14a\x02\x11W\x80c\xA3,+\x99\x14a\x02/Wa\0\xEAV[\x80c6V\x8A\xBE\x11a\0\xC8W\x80c6V\x8A\xBE\x14a\x01kW\x80cP\xD1{^\x14a\x01\x87W\x80cu\xB28\xFC\x14a\x01\xA5W\x80c\x91\xD1HT\x14a\x01\xC3Wa\0\xEAV[\x80c\x01\xFF\xC9\xA7\x14a\0\xEFW\x80c$\x8A\x9C\xA3\x14a\x01\x1FW\x80c//\xF1]\x14a\x01OW[`\0\x80\xFD[a\x01\t`\x04\x806\x03\x81\x01\x90a\x01\x04\x91\x90a\x0E\xD5V[a\x02\xD4V[`@Qa\x01\x16\x91\x90a\x0F\x1DV[`@Q\x80\x91\x03\x90\xF3[a\x019`\x04\x806\x03\x81\x01\x90a\x014\x91\x90a\x0FnV[a\x03NV[`@Qa\x01F\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x01i`\x04\x806\x03\x81\x01\x90a\x01d\x91\x90a\x10#V[a\x03mV[\0[a\x01\x85`\x04\x806\x03\x81\x01\x90a\x01\x80\x91\x90a\x10#V[a\x03\x8EV[\0[a\x01\x8Fa\x04\x11V[`@Qa\x01\x9C\x91\x90a\x10\xC2V[`@Q\x80\x91\x03\x90\xF3[a\x01\xADa\x047V[`@Qa\x01\xBA\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDD`\x04\x806\x03\x81\x01\x90a\x01\xD8\x91\x90a\x10#V[a\x04[V[`@Qa\x01\xEA\x91\x90a\x0F\x1DV[`@Q\x80\x91\x03\x90\xF3[a\x01\xFBa\x04\xC5V[`@Qa\x02\x08\x91\x90a\x11TV[`@Q\x80\x91\x03\x90\xF3[a\x02\x19a\x04\xD8V[`@Qa\x02&\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x02I`\x04\x806\x03\x81\x01\x90a\x02D\x91\x90a\x14GV[a\x04\xDFV[`@Qa\x02W\x92\x91\x90a\x155V[`@Q\x80\x91\x03\x90\xF3[a\x02ha\x06\xD0V[`@Qa\x02u\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[a\x02\x98`\x04\x806\x03\x81\x01\x90a\x02\x93\x91\x90a\x10#V[a\x06\xF4V[\0[a\x02\xB4`\x04\x806\x03\x81\x01\x90a\x02\xAF\x91\x90a\x15eV[a\x07\x15V[\0[a\x02\xBEa\x07\x84V[`@Qa\x02\xCB\x91\x90a\x0F\xAAV[`@Q\x80\x91\x03\x90\xF3[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03GWPa\x03F\x82a\x07\xA7V[[\x90P\x91\x90PV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x03v\x82a\x03NV[a\x03\x7F\x81a\x08\x11V[a\x03\x89\x83\x83a\x08%V[PPPV[a\x03\x96a\t\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x03W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xFA\x90a\x16\x15V[`@Q\x80\x91\x03\x90\xFD[a\x04\r\x82\x82a\t\rV[PPV[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80\x1B\x81V[`\0```\0a\x04\xF0\x86\x86\x86a\t\xEEV[\x90P`\0\x80`\xF8\x1B\x82`\0\x81Q\x81\x10a\x05\x0CWa\x05\x0Ba\x165V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x03a\x05fW\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9\x90Pa\x05\x89V[~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd\x90P[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16\x83`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xF7\x92\x91\x90a\x16dV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x068\x91\x90a\x16\xA2V[\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xECr3g\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06u\x91\x90a\x16\xCFV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xBB\x91\x90a\x17aV[\x90P`\x01\x81\x95P\x95PPPPP\x93P\x93\x91PPV[\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9\x81V[a\x06\xFD\x82a\x03NV[a\x07\x06\x81a\x08\x11V[a\x07\x10\x83\x83a\t\rV[PPPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x07?\x81a\x08\x11V[\x81`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a\x08\"\x81a\x08\x1Da\t\x05V[a\x0B{V[PV[a\x08/\x82\x82a\x04[V[a\t\x01W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x08\xA6a\t\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\t\x17\x82\x82a\x04[V[\x15a\t\xEAW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\t\x8Fa\t\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[```\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x0CWa\n\x0Ba\x11\x85V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\n>W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0\x80[\x85Q\x81\x10\x15a\n\xD7W\x84\x86\x82\x81Q\x81\x10a\nbWa\naa\x165V[[` \x02` \x01\x01Q` \x01Q\x03a\n\xC4W\x82\x86\x82\x81Q\x81\x10a\n\x87Wa\n\x86a\x165V[[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\n\xA4\x92\x91\x90a\x17\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\n\xC0\x90a\x18IV[\x92PP[\x80\x80a\n\xCF\x90a\x18uV[\x91PPa\nEV[P`\x02\x84\x03a\n\xE9W`\x01\x93Pa\n\xF7V[`\x03\x84\x03a\n\xF6W`\0\x93P[[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\x1CE`+\x919\x90P`\0\x85`\xF8\x1B\x90P`\0` `\xFF\x16`\xE0\x1B\x90P`\0\x83Q`\xE0\x1B\x90P`\0\x85`\xE0\x1B\x90P`\0\x84\x84\x8D\x85\x89\x86\x8D`@Q` \x01a\x0BX\x97\x96\x95\x94\x93\x92\x91\x90a\x19LV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x98PPPPPPPPP\x93\x92PPPV[a\x0B\x85\x82\x82a\x04[V[a\x0B\xFCWa\x0B\x92\x81a\x0C\0V[a\x0B\xA0\x83`\0\x1C` a\x0C-V[`@Q` \x01a\x0B\xB1\x92\x91\x90a\x1A\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xF3\x91\x90a\x1B\x17V[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x0C&\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x0C-V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a\x0C@\x91\x90a\x1B9V[a\x0CJ\x91\x90a\x1B{V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CcWa\x0Cba\x11\x85V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0C\x95W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x0C\xCDWa\x0C\xCCa\x165V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\r1Wa\r0a\x165V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\rq\x91\x90a\x1B9V[a\r{\x91\x90a\x1B{V[\x90P[`\x01\x81\x11\x15a\x0E\x1BW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\r\xBDWa\r\xBCa\x165V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\r\xD4Wa\r\xD3a\x165V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x0E\x14\x90a\x1B\xAFV[\x90Pa\r~V[P`\0\x84\x14a\x0E_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EV\x90a\x1C$V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x0E\xB2\x81a\x0E}V[\x81\x14a\x0E\xBDW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xCF\x81a\x0E\xA9V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0E\xEBWa\x0E\xEAa\x0EsV[[`\0a\x0E\xF9\x84\x82\x85\x01a\x0E\xC0V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x0F\x17\x81a\x0F\x02V[\x82RPPV[`\0` \x82\x01\x90Pa\x0F2`\0\x83\x01\x84a\x0F\x0EV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0FK\x81a\x0F8V[\x81\x14a\x0FVW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0Fh\x81a\x0FBV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\x84Wa\x0F\x83a\x0EsV[[`\0a\x0F\x92\x84\x82\x85\x01a\x0FYV[\x91PP\x92\x91PPV[a\x0F\xA4\x81a\x0F8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0F\xBF`\0\x83\x01\x84a\x0F\x9BV[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0F\xF0\x82a\x0F\xC5V[\x90P\x91\x90PV[a\x10\0\x81a\x0F\xE5V[\x81\x14a\x10\x0BW`\0\x80\xFD[PV[`\0\x815\x90Pa\x10\x1D\x81a\x0F\xF7V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x10:Wa\x109a\x0EsV[[`\0a\x10H\x85\x82\x86\x01a\x0FYV[\x92PP` a\x10Y\x85\x82\x86\x01a\x10\x0EV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x10\x88a\x10\x83a\x10~\x84a\x0F\xC5V[a\x10cV[a\x0F\xC5V[\x90P\x91\x90PV[`\0a\x10\x9A\x82a\x10mV[\x90P\x91\x90PV[`\0a\x10\xAC\x82a\x10\x8FV[\x90P\x91\x90PV[a\x10\xBC\x81a\x10\xA1V[\x82RPPV[`\0` \x82\x01\x90Pa\x10\xD7`\0\x83\x01\x84a\x10\xB3V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x11\x1DWa\x11\x1Ca\x10\xDDV[[PV[`\0\x81\x90Pa\x11.\x82a\x11\x0CV[\x91\x90PV[`\0a\x11>\x82a\x11 V[\x90P\x91\x90PV[a\x11N\x81a\x113V[\x82RPPV[`\0` \x82\x01\x90Pa\x11i`\0\x83\x01\x84a\x11EV[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x11\xBD\x82a\x11tV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x11\xDCWa\x11\xDBa\x11\x85V[[\x80`@RPPPV[`\0a\x11\xEFa\x0EiV[\x90Pa\x11\xFB\x82\x82a\x11\xB4V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12\x1BWa\x12\x1Aa\x11\x85V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12[Wa\x12Za\x11\x85V[[a\x12d\x82a\x11tV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x12\x93a\x12\x8E\x84a\x12@V[a\x11\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x12\xAFWa\x12\xAEa\x12;V[[a\x12\xBA\x84\x82\x85a\x12qV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x12\xD7Wa\x12\xD6a\x11oV[[\x815a\x12\xE7\x84\x82` \x86\x01a\x12\x80V[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x13\x03\x81a\x12\xF0V[\x81\x14a\x13\x0EW`\0\x80\xFD[PV[`\0\x815\x90Pa\x13 \x81a\x12\xFAV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x13<Wa\x13;a\x121V[[a\x13F`@a\x11\xE5V[\x90P`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13fWa\x13ea\x126V[[a\x13r\x84\x82\x85\x01a\x12\xC2V[`\0\x83\x01RP` a\x13\x86\x84\x82\x85\x01a\x13\x11V[` \x83\x01RP\x92\x91PPV[`\0a\x13\xA5a\x13\xA0\x84a\x12\0V[a\x11\xE5V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x13\xC8Wa\x13\xC7a\x12,V[[\x83[\x81\x81\x10\x15a\x14\x0FW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xEDWa\x13\xECa\x11oV[[\x80\x86\x01a\x13\xFA\x89\x82a\x13&V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x13\xCAV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14.Wa\x14-a\x11oV[[\x815a\x14>\x84\x82` \x86\x01a\x13\x92V[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14`Wa\x14_a\x0EsV[[`\0a\x14n\x86\x82\x87\x01a\x0FYV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x8FWa\x14\x8Ea\x0ExV[[a\x14\x9B\x86\x82\x87\x01a\x14\x19V[\x92PP`@a\x14\xAC\x86\x82\x87\x01a\x13\x11V[\x91PP\x92P\x92P\x92V[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x14\xF0W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x14\xD5V[`\0\x84\x84\x01RPPPPV[`\0a\x15\x07\x82a\x14\xB6V[a\x15\x11\x81\x85a\x14\xC1V[\x93Pa\x15!\x81\x85` \x86\x01a\x14\xD2V[a\x15*\x81a\x11tV[\x84\x01\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa\x15J`\0\x83\x01\x85a\x0F\x0EV[\x81\x81\x03` \x83\x01Ra\x15\\\x81\x84a\x14\xFCV[\x90P\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x15{Wa\x15za\x0EsV[[`\0a\x15\x89\x84\x82\x85\x01a\x10\x0EV[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x15\xFF`/\x83a\x15\x92V[\x91Pa\x16\n\x82a\x15\xA3V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x16.\x81a\x15\xF2V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x01\x90Pa\x16y`\0\x83\x01\x85a\x0F\x9BV[a\x16\x86` \x83\x01\x84a\x11EV[\x93\x92PPPV[`\0\x81Q\x90Pa\x16\x9C\x81a\x0F\xF7V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xB8Wa\x16\xB7a\x0EsV[[`\0a\x16\xC6\x84\x82\x85\x01a\x16\x8DV[\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x16\xE9\x81\x84a\x14\xFCV[\x90P\x92\x91PPV[`\0a\x17\x04a\x16\xFF\x84a\x12@V[a\x11\xE5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x17 Wa\x17\x1Fa\x12;V[[a\x17+\x84\x82\x85a\x14\xD2V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x17HWa\x17Ga\x11oV[[\x81Qa\x17X\x84\x82` \x86\x01a\x16\xF1V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17wWa\x17va\x0EsV[[`\0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x95Wa\x17\x94a\x0ExV[[a\x17\xA1\x84\x82\x85\x01a\x173V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[`\0a\x17\xC0\x82a\x14\xB6V[a\x17\xCA\x81\x85a\x17\xAAV[\x93Pa\x17\xDA\x81\x85` \x86\x01a\x14\xD2V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x17\xF2\x82\x85a\x17\xB5V[\x91Pa\x17\xFE\x82\x84a\x17\xB5V[\x91P\x81\x90P\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x18T\x82a\x189V[\x91Pc\xFF\xFF\xFF\xFF\x82\x03a\x18jWa\x18ia\x18\nV[[`\x01\x82\x01\x90P\x91\x90PV[`\0a\x18\x80\x82a\x12\xF0V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x18\xB2Wa\x18\xB1a\x18\nV[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x19\x04a\x18\xFF\x82a\x18\xBDV[a\x18\xE9V[\x82RPPV[`\0\x81\x90P\x91\x90PV[a\x19%a\x19 \x82a\x0E}V[a\x19\nV[\x82RPPV[`\0\x81\x90P\x91\x90PV[a\x19Fa\x19A\x82a\x0F8V[a\x19+V[\x82RPPV[`\0a\x19X\x82\x8Aa\x18\xF3V[`\x01\x82\x01\x91Pa\x19h\x82\x89a\x19\x14V[`\x04\x82\x01\x91Pa\x19x\x82\x88a\x195V[` \x82\x01\x91Pa\x19\x88\x82\x87a\x19\x14V[`\x04\x82\x01\x91Pa\x19\x98\x82\x86a\x17\xB5V[\x91Pa\x19\xA4\x82\x85a\x19\x14V[`\x04\x82\x01\x91Pa\x19\xB4\x82\x84a\x17\xB5V[\x91P\x81\x90P\x98\x97PPPPPPPPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A\x06`\x17\x83a\x19\xC5V[\x91Pa\x1A\x11\x82a\x19\xD0V[`\x17\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0a\x1A2\x82a\x1A\x1CV[a\x1A<\x81\x85a\x19\xC5V[\x93Pa\x1AL\x81\x85` \x86\x01a\x14\xD2V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1A\x8E`\x11\x83a\x19\xC5V[\x91Pa\x1A\x99\x82a\x1AXV[`\x11\x82\x01\x90P\x91\x90PV[`\0a\x1A\xAF\x82a\x19\xF9V[\x91Pa\x1A\xBB\x82\x85a\x1A'V[\x91Pa\x1A\xC6\x82a\x1A\x81V[\x91Pa\x1A\xD2\x82\x84a\x1A'V[\x91P\x81\x90P\x93\x92PPPV[`\0a\x1A\xE9\x82a\x1A\x1CV[a\x1A\xF3\x81\x85a\x15\x92V[\x93Pa\x1B\x03\x81\x85` \x86\x01a\x14\xD2V[a\x1B\x0C\x81a\x11tV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1B1\x81\x84a\x1A\xDEV[\x90P\x92\x91PPV[`\0a\x1BD\x82a\x12\xF0V[\x91Pa\x1BO\x83a\x12\xF0V[\x92P\x82\x82\x02a\x1B]\x81a\x12\xF0V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1BtWa\x1Bsa\x18\nV[[P\x92\x91PPV[`\0a\x1B\x86\x82a\x12\xF0V[\x91Pa\x1B\x91\x83a\x12\xF0V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1B\xA9Wa\x1B\xA8a\x18\nV[[\x92\x91PPV[`\0a\x1B\xBA\x82a\x12\xF0V[\x91P`\0\x82\x03a\x1B\xCDWa\x1B\xCCa\x18\nV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a\x1C\x0E` \x83a\x15\x92V[\x91Pa\x1C\x19\x82a\x1B\xD8V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C=\x81a\x1C\x01V[\x90P\x91\x90PV\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 p\xF3\x19\"4\x8D\x94\xF9_\xF2m\xF5\x0C?P0\xB1\x8F\xE6{2\xCA\xE6\xE3\xC7\xA3\xFFZ\xA1\x93K\x11dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static ARBITRUMKEYDERIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ArbitrumKeyDeriver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ArbitrumKeyDeriver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ArbitrumKeyDeriver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ArbitrumKeyDeriver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ArbitrumKeyDeriver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ArbitrumKeyDeriver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ArbitrumKeyDeriver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ARBITRUMKEYDERIVER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ARBITRUMKEYDERIVER_ABI.clone(),
                ARBITRUMKEYDERIVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ADMIN_ROLE` (0x75b238fc) function
        pub fn admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([117, 178, 56, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HD_KDF_K256` (0xfe89c970) function
        pub fn hd_kdf_k256(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([254, 137, 201, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HD_KDF_P256` (0xb24ed308) function
        pub fn hd_kdf_p256(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([178, 78, 211, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeHDPubKey` (0xa32c2b99) function
        pub fn compute_hd_pub_key(
            &self,
            derived_key_id: [u8; 32],
            root_hd_keys: ::std::vec::Vec<RootKey>,
            key_type: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash(
                    [163, 44, 43, 153],
                    (derived_key_id, root_hd_keys, key_type),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractResolver` (0x50d17b5e) function
        pub fn contract_resolver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 209, 123, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `env` (0x9dca0032) function
        pub fn env(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([157, 202, 0, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setContractResolver` (0xf95d71b1) function
        pub fn set_contract_resolver(
            &self,
            contract_resolver_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 93, 113, 177], contract_resolver_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ArbitrumKeyDeriverEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ArbitrumKeyDeriver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ArbitrumKeyDeriverEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ArbitrumKeyDeriverEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ArbitrumKeyDeriverEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ArbitrumKeyDeriverEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ArbitrumKeyDeriverEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ArbitrumKeyDeriverEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ArbitrumKeyDeriverEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ArbitrumKeyDeriverEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ArbitrumKeyDeriverEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ADMIN_ROLE` function with signature `ADMIN_ROLE()` and selector `0x75b238fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ADMIN_ROLE", abi = "ADMIN_ROLE()")]
    pub struct AdminRoleCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `HD_KDF_K256` function with signature `HD_KDF_K256()` and selector `0xfe89c970`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "HD_KDF_K256", abi = "HD_KDF_K256()")]
    pub struct HdKdfK256Call;
    ///Container type for all input parameters for the `HD_KDF_P256` function with signature `HD_KDF_P256()` and selector `0xb24ed308`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "HD_KDF_P256", abi = "HD_KDF_P256()")]
    pub struct HdKdfP256Call;
    ///Container type for all input parameters for the `computeHDPubKey` function with signature `computeHDPubKey(bytes32,(bytes,uint256)[],uint256)` and selector `0xa32c2b99`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "computeHDPubKey",
        abi = "computeHDPubKey(bytes32,(bytes,uint256)[],uint256)"
    )]
    pub struct ComputeHDPubKeyCall {
        pub derived_key_id: [u8; 32],
        pub root_hd_keys: ::std::vec::Vec<RootKey>,
        pub key_type: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `contractResolver` function with signature `contractResolver()` and selector `0x50d17b5e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "contractResolver", abi = "contractResolver()")]
    pub struct ContractResolverCall;
    ///Container type for all input parameters for the `env` function with signature `env()` and selector `0x9dca0032`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "env", abi = "env()")]
    pub struct EnvCall;
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setContractResolver` function with signature `setContractResolver(address)` and selector `0xf95d71b1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setContractResolver", abi = "setContractResolver(address)")]
    pub struct SetContractResolverCall {
        pub contract_resolver_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum ArbitrumKeyDeriverCalls {
        AdminRole(AdminRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        HdKdfK256(HdKdfK256Call),
        HdKdfP256(HdKdfP256Call),
        ComputeHDPubKey(ComputeHDPubKeyCall),
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetContractResolver(SetContractResolverCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ArbitrumKeyDeriverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <HdKdfK256Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HdKdfK256(decoded));
            }
            if let Ok(decoded) = <HdKdfP256Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HdKdfP256(decoded));
            }
            if let Ok(decoded) = <ComputeHDPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeHDPubKey(decoded));
            }
            if let Ok(decoded) = <ContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContractResolver(decoded));
            }
            if let Ok(decoded) = <EnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Env(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SetContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetContractResolver(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ArbitrumKeyDeriverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HdKdfK256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HdKdfP256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeHDPubKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Env(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ArbitrumKeyDeriverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HdKdfK256(element) => ::core::fmt::Display::fmt(element, f),
                Self::HdKdfP256(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHDPubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Env(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractResolver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<HdKdfK256Call> for ArbitrumKeyDeriverCalls {
        fn from(value: HdKdfK256Call) -> Self {
            Self::HdKdfK256(value)
        }
    }
    impl ::core::convert::From<HdKdfP256Call> for ArbitrumKeyDeriverCalls {
        fn from(value: HdKdfP256Call) -> Self {
            Self::HdKdfP256(value)
        }
    }
    impl ::core::convert::From<ComputeHDPubKeyCall> for ArbitrumKeyDeriverCalls {
        fn from(value: ComputeHDPubKeyCall) -> Self {
            Self::ComputeHDPubKey(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for ArbitrumKeyDeriverCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for ArbitrumKeyDeriverCalls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ArbitrumKeyDeriverCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetContractResolverCall> for ArbitrumKeyDeriverCalls {
        fn from(value: SetContractResolverCall) -> Self {
            Self::SetContractResolver(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ArbitrumKeyDeriverCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `ADMIN_ROLE` function with signature `ADMIN_ROLE()` and selector `0x75b238fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HD_KDF_K256` function with signature `HD_KDF_K256()` and selector `0xfe89c970`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HdKdfK256Return(pub [u8; 32]);
    ///Container type for all return fields from the `HD_KDF_P256` function with signature `HD_KDF_P256()` and selector `0xb24ed308`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HdKdfP256Return(pub [u8; 32]);
    ///Container type for all return fields from the `computeHDPubKey` function with signature `computeHDPubKey(bytes32,(bytes,uint256)[],uint256)` and selector `0xa32c2b99`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ComputeHDPubKeyReturn(pub bool, pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `contractResolver` function with signature `contractResolver()` and selector `0x50d17b5e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ContractResolverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `env` function with signature `env()` and selector `0x9dca0032`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvReturn(pub u8);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///`RootKey(bytes,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RootKey {
        pub pubkey: ::ethers::core::types::Bytes,
        pub key_type: ::ethers::core::types::U256,
    }
}
