pub use contract_resolver::*;
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
pub mod contract_resolver {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/ContractResolver.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("env"),
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
                    ::std::borrow::ToOwned::to_owned("ALLOWLIST_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ALLOWLIST_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("BACKUP_RECOVERY_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BACKUP_RECOVERY_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("CLONE_NET_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CLONE_NET_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_WALLET_REGISTRY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DOMAIN_WALLET_REGISTRY",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("HD_KEY_DERIVER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HD_KEY_DERIVER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("HOST_COMMANDS_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HOST_COMMANDS_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("LIT_TOKEN_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LIT_TOKEN_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("MULTI_SENDER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MULTI_SENDER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PAYMENT_DELEGATION_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PAYMENT_DELEGATION_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_HELPER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_HELPER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_HELPER_V2_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_HELPER_V2_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_NFT_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PKP_NFT_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("PKP_NFT_METADATA_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_NFT_METADATA_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_PERMISSIONS_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_PERMISSIONS_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PUB_KEY_ROUTER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PUB_KEY_ROUTER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("RATE_LIMIT_NFT_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RATE_LIMIT_NFT_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("RELEASE_REGISTER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RELEASE_REGISTER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("STAKING_BALANCES_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STAKING_BALANCES_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("STAKING_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("STAKING_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("addAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("addAllowedEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAllowedEnv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("getContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("removeAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adminBeingRemoved"),
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
                    ::std::borrow::ToOwned::to_owned("removeAllowedEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAllowedEnv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("setContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
                (
                    ::std::borrow::ToOwned::to_owned("typeAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("typeAddresses"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("AllowedEnvAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllowedEnvAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedEnvRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllowedEnvRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                (
                    ::std::borrow::ToOwned::to_owned("SetContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONTRACTRESOLVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\"\xE98\x03\x80b\0\"\xE9\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x03XV[b\0\0i\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3b\0\x010` \x1B` \x1CV[b\0\0\x9B\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x80b\0\x01F` \x1B` \x1CV[`\x01\x80`\0\x83`\x02\x81\x11\x15b\0\0\xB6Wb\0\0\xB5b\0\x03\x8AV[[`\x02\x81\x11\x15b\0\0\xCBWb\0\0\xCAb\0\x03\x8AV[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qb\0\x01!\x91\x90b\0\x04\nV[`@Q\x80\x91\x03\x90\xA1Pb\0\x04'V[b\0\x01B\x82\x82b\0\x01\xA9` \x1B` \x1CV[PPV[`\0b\0\x01Y\x83b\0\x02\x9A` \x1B` \x1CV[\x90P\x81`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01\x81\x90UP\x81\x81\x84\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[b\0\x01\xBB\x82\x82b\0\x02\xB9` \x1B` \x1CV[b\0\x02\x96W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPb\0\x02;b\0\x03#` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\x03\x81\x10b\0\x03>W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03R\x81b\0\x030V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x03qWb\0\x03pb\0\x03+V[[`\0b\0\x03\x81\x84\x82\x85\x01b\0\x03AV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10b\0\x03\xCDWb\0\x03\xCCb\0\x03\x8AV[[PV[`\0\x81\x90Pb\0\x03\xE0\x82b\0\x03\xB9V[\x91\x90PV[`\0b\0\x03\xF2\x82b\0\x03\xD0V[\x90P\x91\x90PV[b\0\x04\x04\x81b\0\x03\xE5V[\x82RPPV[`\0` \x82\x01\x90Pb\0\x04!`\0\x83\x01\x84b\0\x03\xF9V[\x92\x91PPV[a\x1E\xB2\x80b\0\x047`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80cu\xB28\xFC\x11a\x01\x1AW\x80c\x90r\xF88\x11a\0\xADW\x80c\xAD\x1C\x8A\x86\x11a\0|W\x80c\xAD\x1C\x8A\x86\x14a\x05\xBDW\x80c\xD5Gt\x1F\x14a\x05\xDBW\x80c\xDA\x19\xDD\xFB\x14a\x05\xF7W\x80c\xDF8\x06\x93\x14a\x06\x15W\x80c\xF8\xAE\x93\xB4\x14a\x063Wa\x02\x06V[\x80c\x90r\xF88\x14a\x053W\x80c\x91\xD1HT\x14a\x05QW\x80c\x97z\x80p\x14a\x05\x81W\x80c\xA2\x17\xFD\xDF\x14a\x05\x9FWa\x02\x06V[\x80c\x85\xCB\x11\x91\x11a\0\xE9W\x80c\x85\xCB\x11\x91\x14a\x04\xABW\x80c\x8C\x156\xDF\x14a\x04\xC9W\x80c\x8D\xEB8\x93\x14a\x04\xE7W\x80c\x8E\x8D\xFD\x16\x14a\x05\x03Wa\x02\x06V[\x80cu\xB28\xFC\x14a\x043W\x80c|\xAD\xF6\x9F\x14a\x04QW\x80c}J\x03\xBD\x14a\x04oW\x80c\x7F\x90 \x9F\x14a\x04\x8DWa\x02\x06V[\x80c.H\x85\xE8\x11a\x01\x9DW\x80cB\x16\xE7:\x11a\x01lW\x80cB\x16\xE7:\x14a\x03\xA3W\x80cQ\xAD\n\x80\x14a\x03\xC1W\x80cZ\xF2\x7Fy\x14a\x03\xDDW\x80cpH\x02u\x14a\x03\xFBW\x80ct\xBC\x819\x14a\x04\x17Wa\x02\x06V[\x80c.H\x85\xE8\x14a\x03\x1DW\x80c//\xF1]\x14a\x03;W\x80c6V\x8A\xBE\x14a\x03WW\x80c>\xBFy\x85\x14a\x03sWa\x02\x06V[\x80c\x19;\x98=\x11a\x01\xD9W\x80c\x19;\x98=\x14a\x02\x93W\x80c$\x8A\x9C\xA3\x14a\x02\xB1W\x80c&h\xF3\x05\x14a\x02\xE1W\x80c,\x0B\x8B\xF7\x14a\x02\xFFWa\x02\x06V[\x80c\x01\xFF\xC9\xA7\x14a\x02\x0BW\x80c\x11\xEE\x8F\xF7\x14a\x02;W\x80c\x16\xF7k\xBF\x14a\x02YW\x80c\x17\x85\xF5<\x14a\x02wW[`\0\x80\xFD[a\x02%`\x04\x806\x03\x81\x01\x90a\x02 \x91\x90a\x15\xD1V[a\x06QV[`@Qa\x022\x91\x90a\x16\x19V[`@Q\x80\x91\x03\x90\xF3[a\x02Ca\x06\xCBV[`@Qa\x02P\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02aa\x06\xEFV[`@Qa\x02n\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a\x16\xC6V[a\x07\x13V[\0[a\x02\x9Ba\x07\xD9V[`@Qa\x02\xA8\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02\xCB`\x04\x806\x03\x81\x01\x90a\x02\xC6\x91\x90a\x17\x1FV[a\x07\xFDV[`@Qa\x02\xD8\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE9a\x08\x1CV[`@Qa\x02\xF6\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03\x07a\x08@V[`@Qa\x03\x14\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03%a\x08dV[`@Qa\x032\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03U`\x04\x806\x03\x81\x01\x90a\x03P\x91\x90a\x17LV[a\x08\x88V[\0[a\x03q`\x04\x806\x03\x81\x01\x90a\x03l\x91\x90a\x17LV[a\x08\xA9V[\0[a\x03\x8D`\x04\x806\x03\x81\x01\x90a\x03\x88\x91\x90a\x17\xB1V[a\t,V[`@Qa\x03\x9A\x91\x90a\x18\0V[`@Q\x80\x91\x03\x90\xF3[a\x03\xABa\tnV[`@Qa\x03\xB8\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03\xDB`\x04\x806\x03\x81\x01\x90a\x03\xD6\x91\x90a\x18\x1BV[a\t\x92V[\0[a\x03\xE5a\x0BDV[`@Qa\x03\xF2\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\x15`\x04\x806\x03\x81\x01\x90a\x04\x10\x91\x90a\x16\xC6V[a\x0BhV[\0[a\x041`\x04\x806\x03\x81\x01\x90a\x04,\x91\x90a\x18nV[a\x0B\xC0V[\0[a\x04;a\x0C\xA9V[`@Qa\x04H\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04Ya\x0C\xCDV[`@Qa\x04f\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04wa\x0C\xF1V[`@Qa\x04\x84\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\x95a\r\x15V[`@Qa\x04\xA2\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\xB3a\r9V[`@Qa\x04\xC0\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\xD1a\r]V[`@Qa\x04\xDE\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\x01`\x04\x806\x03\x81\x01\x90a\x04\xFC\x91\x90a\x18nV[a\r\x81V[\0[a\x05\x1D`\x04\x806\x03\x81\x01\x90a\x05\x18\x91\x90a\x17\xB1V[a\x0EbV[`@Qa\x05*\x91\x90a\x18\0V[`@Q\x80\x91\x03\x90\xF3[a\x05;a\x0E\xD5V[`@Qa\x05H\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05k`\x04\x806\x03\x81\x01\x90a\x05f\x91\x90a\x17LV[a\x0E\xF9V[`@Qa\x05x\x91\x90a\x16\x19V[`@Q\x80\x91\x03\x90\xF3[a\x05\x89a\x0FcV[`@Qa\x05\x96\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\xA7a\x0F\x87V[`@Qa\x05\xB4\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\xC5a\x0F\x8EV[`@Qa\x05\xD2\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\xF5`\x04\x806\x03\x81\x01\x90a\x05\xF0\x91\x90a\x17LV[a\x0F\xB2V[\0[a\x05\xFFa\x0F\xD3V[`@Qa\x06\x0C\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x06\x1Da\x0F\xF7V[`@Qa\x06*\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x06;a\x10\x1BV[`@Qa\x06H\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xC4WPa\x06\xC3\x82a\x10?V[[\x90P\x91\x90PV[\x7FX\xA0\x04N\x0E\xCD\x81\x02^9\x8B\xF1\x81Pu\xD1#L\xBA\xC3t\x96\x14\xB0\xB3:@L.\xE2\xBA\xBF\x81V[\x7F\xF1OC\x1D\xAD\xC8.}\xBC^7\x9Fq#NW5\xC9\x18~C'\xA7\xC6\xAC\x01MU\xD1\xB7rz\x81V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x07=\x81a\x10\xA9V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x07\xABW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xA2\x90a\x19\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x07\xD5\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x10\xBDV[PPV[\x7Ft\xEB\x03\xCFc\x07P\x04\xAB\xBF\x06s\xB7\x96\xDEr<\x9E\xB4\xE7\xCA2\xA8\x05\x85y\x0E\x13K\xB9\x9F\x8C\x81V[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[\x7F\xB1\xF7\x98\x13\xBCv0\xA5*\xE9H\xBC\x99x\x13\x97\xE4\t\xD0\xDD5!\x95;\xF7\xD8\xD7\xA2\xDBaG\xF7\x81V[\x7F\xB7\xB4\xFD\xE9\x94M<\x13\xE9\xA7\x885C\x1C3\xA5\x08M\x90\xA7\xF0\xC7=\xEFv\xD7\x88c\x15\xFE\x87\xB0\x81V[\x7F\xB91\xB2q\x9A\xEB*e\xA5\x03_\xA0\xA1\x90\xBF\xDCL\x86\"\xCE\x8C\xBF\xF7\xA3\xD1\xABBS\x1F\xB1\xA9\x18\x81V[a\x08\x91\x82a\x07\xFDV[a\x08\x9A\x81a\x10\xA9V[a\x08\xA4\x83\x83a\x11\x9EV[PPPV[a\x08\xB1a\x12~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\x1EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x15\x90a\x19\xB0V[`@Q\x80\x91\x03\x90\xFD[a\t(\x82\x82a\x10\xBDV[PPV[`\x02` R\x81`\0R`@`\0 ` R\x80`\0R`@`\0 `\0\x91P\x91P\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7FLA\xAEEK\xEBk\xBB\xE9\xBEP\xAC\xCC\x95z;\x156\xE4\x8B\x83Z\x86\x91\x9A\xF9\x81\xB5$M\xB7U\x81V[a\t\xBC\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x0E\xF9V[a\t\xF2W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x15\x15`\x01`\0\x84`\x02\x81\x11\x15a\n\rWa\n\x0Ca\x19\xD0V[[`\x02\x81\x11\x15a\n\x1FWa\n\x1Ea\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a\n}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nt\x90a\x1AqV[`@Q\x80\x91\x03\x90\xFD[\x80`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x84`\x02\x81\x11\x15a\n\xA6Wa\n\xA5a\x19\xD0V[[`\x02\x81\x11\x15a\n\xB8Wa\n\xB7a\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F3\xF0\x14\x89\x0F\x10\x92)\xBB\xCF\x8D\xD4r\x04\xC1S\xA2\xC0\xFF\x1CW*a\xDE\"\r\x103e0\xF5=\x83\x83\x83`@Qa\x0B7\x93\x92\x91\x90a\x1A\xD9V[`@Q\x80\x91\x03\x90\xA1PPPV[\x7F\xA2\xC772\xDEez\xD0\xF3n\r\xDB\xB2q\x0FK\x13\xE8\xDD\xE4d!8k\xB9-\x1E\x17\x9D\xAEMM\x81V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x0B\x92\x81a\x10\xA9V[a\x0B\xBC\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x11\x9EV[PPV[a\x0B\xEA\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x0E\xF9V[a\x0C W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80`\0\x83`\x02\x81\x11\x15a\x0C8Wa\x0C7a\x19\xD0V[[`\x02\x81\x11\x15a\x0CJWa\x0CIa\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\x0C\x9E\x91\x90a\x1B\x10V[`@Q\x80\x91\x03\x90\xA1PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[\x7Ft\x84]\xE3|\xFA\xBD5v3!KG\xFA\x91\xCC\xD1\x9B\x05\xB7\xC5\xA0\x8A\xC2,\x18\x7F\x81\x1F\xB6+\xCA\x81V[\x7F\x9F5\xEF>\x0C&R\xA8\xBB\x87G\xD9/@\x7F\xCD9\xA7v\x8D\xAC\xC7\xF1e\x81\xC7\xA7\x1F\x10>Ub\x81V[\x7F\xC6gO\x98\xBA5\xC0\x1C\x13\x0E\x08\x19]\xD2lpF`7G:\x06\x8CZ\xAAG\nx=\x99\xC1l\x81V[\x7F\xAEy\xA95sp\x12\xD0f\xE7\x1802i.R\x1F\xFE\x1A\xDE+\xED\xA2g\xE2>\x02\xB1\xD6\xE9\x11\x87\x81V[\x7F\xAA\x06\xD1\x08\xDB\xD7\xBF\x97k\x16\xB7\xBFZ\xDB)\xD2\xD0\xEF,8\\\xA8\xB9\xD83\xCC\x80/3\x94-r\x81V[a\r\xAB\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x0E\xF9V[a\r\xE1W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x82`\x02\x81\x11\x15a\r\xF8Wa\r\xF7a\x19\xD0V[[`\x02\x81\x11\x15a\x0E\nWa\x0E\ta\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x81`@Qa\x0EW\x91\x90a\x1B\x10V[`@Q\x80\x91\x03\x90\xA1PV[`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x83`\x02\x81\x11\x15a\x0E\x8CWa\x0E\x8Ba\x19\xD0V[[`\x02\x81\x11\x15a\x0E\x9EWa\x0E\x9Da\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x92\x91PPV[\x7FT\x95<#\x06\x8B\x8F\xC4\xC0sc\x01\xB5\x0F\x10\x02}kF\x93'\xDE\x1F\xD4(A\xA5\x07+\x1B\xCE\xBE\x81V[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[\x7F'\xD7d\xEA*J8eCK\xBFJ9\x11\x10\x14\x96D\xBE1D\x8F4y\xFD\x15\xB4C\x88uWe\x81V[`\0\x80\x1B\x81V[\x7F:h\xDB\xFD\x8B\xBBd\x01\\B\xBC\x13\x1C8\x8D\xEAye\xE2\x8C\x10\x04\xD0\x9B9\xF5\x95\0\xC3\xA7c\xEC\x81V[a\x0F\xBB\x82a\x07\xFDV[a\x0F\xC4\x81a\x10\xA9V[a\x0F\xCE\x83\x83a\x10\xBDV[PPPV[\x7F\x08\t\t\xC1\x8C\x95\x8C\xE5\xA2\xD3d\x81ix$\xE4w1\x93#\xD01T\xCE\xBA;x\xF2\x8Aa\x88{\x81V[\x7F\xB4\xBF\x99\x9Bh\xD8\x08]\xBB\xF7\xA0\xEC/Z-f\x08s\x93[\xDF\x1E\xD0\x8E\xB4!\xACm\xCB\xC0\x03b\x81V[\x7F\xDD[\x9B\x8A^\x8E\x01\xF2\x96.\xD7\xE9\x83\xD5\x8F\xE3.\x1Ff\xAA\x88\xDDz\xB3\x07p\xFA\x9Bw\xDArC\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a\x10\xBA\x81a\x10\xB5a\x12~V[a\x12\x86V[PV[a\x10\xC7\x82\x82a\x0E\xF9V[\x15a\x11\x9AW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x11?a\x12~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a\x11\xA8\x82\x82a\x0E\xF9V[a\x12zW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x12\x1Fa\x12~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\x12\x90\x82\x82a\x0E\xF9V[a\x13\x07Wa\x12\x9D\x81a\x13\x0BV[a\x12\xAB\x83`\0\x1C` a\x138V[`@Q` \x01a\x12\xBC\x92\x91\x90a\x1C4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xFE\x91\x90a\x1C\xB8V[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x131\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x138V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a\x13K\x91\x90a\x1D\x13V[a\x13U\x91\x90a\x1DUV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13nWa\x13ma\x1D\x89V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13\xA0W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x13\xD8Wa\x13\xD7a\x1D\xB8V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x14<Wa\x14;a\x1D\xB8V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\x14|\x91\x90a\x1D\x13V[a\x14\x86\x91\x90a\x1DUV[\x90P[`\x01\x81\x11\x15a\x15&W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\x14\xC8Wa\x14\xC7a\x1D\xB8V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x14\xDFWa\x14\xDEa\x1D\xB8V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x15\x1F\x90a\x1D\xE7V[\x90Pa\x14\x89V[P`\0\x84\x14a\x15jW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15a\x90a\x1E\\V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x15\xAE\x81a\x15yV[\x81\x14a\x15\xB9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x15\xCB\x81a\x15\xA5V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xE7Wa\x15\xE6a\x15tV[[`\0a\x15\xF5\x84\x82\x85\x01a\x15\xBCV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x16\x13\x81a\x15\xFEV[\x82RPPV[`\0` \x82\x01\x90Pa\x16.`\0\x83\x01\x84a\x16\nV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x16G\x81a\x164V[\x82RPPV[`\0` \x82\x01\x90Pa\x16b`\0\x83\x01\x84a\x16>V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x16\x93\x82a\x16hV[\x90P\x91\x90PV[a\x16\xA3\x81a\x16\x88V[\x81\x14a\x16\xAEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x16\xC0\x81a\x16\x9AV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xDCWa\x16\xDBa\x15tV[[`\0a\x16\xEA\x84\x82\x85\x01a\x16\xB1V[\x91PP\x92\x91PPV[a\x16\xFC\x81a\x164V[\x81\x14a\x17\x07W`\0\x80\xFD[PV[`\0\x815\x90Pa\x17\x19\x81a\x16\xF3V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x175Wa\x174a\x15tV[[`\0a\x17C\x84\x82\x85\x01a\x17\nV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17cWa\x17ba\x15tV[[`\0a\x17q\x85\x82\x86\x01a\x17\nV[\x92PP` a\x17\x82\x85\x82\x86\x01a\x16\xB1V[\x91PP\x92P\x92\x90PV[`\x03\x81\x10a\x17\x99W`\0\x80\xFD[PV[`\0\x815\x90Pa\x17\xAB\x81a\x17\x8CV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xC8Wa\x17\xC7a\x15tV[[`\0a\x17\xD6\x85\x82\x86\x01a\x17\nV[\x92PP` a\x17\xE7\x85\x82\x86\x01a\x17\x9CV[\x91PP\x92P\x92\x90PV[a\x17\xFA\x81a\x16\x88V[\x82RPPV[`\0` \x82\x01\x90Pa\x18\x15`\0\x83\x01\x84a\x17\xF1V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x184Wa\x183a\x15tV[[`\0a\x18B\x86\x82\x87\x01a\x17\nV[\x93PP` a\x18S\x86\x82\x87\x01a\x17\x9CV[\x92PP`@a\x18d\x86\x82\x87\x01a\x16\xB1V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x18\x84Wa\x18\x83a\x15tV[[`\0a\x18\x92\x84\x82\x85\x01a\x17\x9CV[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCannot remove self as admin.  Ha`\0\x82\x01R\x7Fve the new admin do it.\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x19\x08`7\x83a\x18\x9BV[\x91Pa\x19\x13\x82a\x18\xACV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x197\x81a\x18\xFBV[\x90P\x91\x90PV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x19\x9A`/\x83a\x18\x9BV[\x91Pa\x19\xA5\x82a\x19>V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x19\xC9\x81a\x19\x8DV[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x7FThe provided Env is not valid fo`\0\x82\x01R\x7Fr this contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1A[`/\x83a\x18\x9BV[\x91Pa\x1Af\x82a\x19\xFFV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\x8A\x81a\x1ANV[\x90P\x91\x90PV[`\x03\x81\x10a\x1A\xA2Wa\x1A\xA1a\x19\xD0V[[PV[`\0\x81\x90Pa\x1A\xB3\x82a\x1A\x91V[\x91\x90PV[`\0a\x1A\xC3\x82a\x1A\xA5V[\x90P\x91\x90PV[a\x1A\xD3\x81a\x1A\xB8V[\x82RPPV[`\0``\x82\x01\x90Pa\x1A\xEE`\0\x83\x01\x86a\x16>V[a\x1A\xFB` \x83\x01\x85a\x1A\xCAV[a\x1B\x08`@\x83\x01\x84a\x17\xF1V[\x94\x93PPPPV[`\0` \x82\x01\x90Pa\x1B%`\0\x83\x01\x84a\x1A\xCAV[\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1Bl`\x17\x83a\x1B+V[\x91Pa\x1Bw\x82a\x1B6V[`\x17\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0[\x83\x81\x10\x15a\x1B\xABW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1B\x90V[`\0\x84\x84\x01RPPPPV[`\0a\x1B\xC2\x82a\x1B\x82V[a\x1B\xCC\x81\x85a\x1B+V[\x93Pa\x1B\xDC\x81\x85` \x86\x01a\x1B\x8DV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1C\x1E`\x11\x83a\x1B+V[\x91Pa\x1C)\x82a\x1B\xE8V[`\x11\x82\x01\x90P\x91\x90PV[`\0a\x1C?\x82a\x1B_V[\x91Pa\x1CK\x82\x85a\x1B\xB7V[\x91Pa\x1CV\x82a\x1C\x11V[\x91Pa\x1Cb\x82\x84a\x1B\xB7V[\x91P\x81\x90P\x93\x92PPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x1C\x8A\x82a\x1B\x82V[a\x1C\x94\x81\x85a\x18\x9BV[\x93Pa\x1C\xA4\x81\x85` \x86\x01a\x1B\x8DV[a\x1C\xAD\x81a\x1CnV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\xD2\x81\x84a\x1C\x7FV[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x1D\x1E\x82a\x1C\xDAV[\x91Pa\x1D)\x83a\x1C\xDAV[\x92P\x82\x82\x02a\x1D7\x81a\x1C\xDAV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1DNWa\x1DMa\x1C\xE4V[[P\x92\x91PPV[`\0a\x1D`\x82a\x1C\xDAV[\x91Pa\x1Dk\x83a\x1C\xDAV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1D\x83Wa\x1D\x82a\x1C\xE4V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a\x1D\xF2\x82a\x1C\xDAV[\x91P`\0\x82\x03a\x1E\x05Wa\x1E\x04a\x1C\xE4V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a\x1EF` \x83a\x18\x9BV[\x91Pa\x1EQ\x82a\x1E\x10V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1Eu\x81a\x1E9V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xC0-S\\B\x14\x0C\xBB\xEE.\x86\xCC\xAA\xEE:\t\xDE\x07\0T-R\xEDR\\\xFC\xF8\xFB+S\x85tdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static CONTRACTRESOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80cu\xB28\xFC\x11a\x01\x1AW\x80c\x90r\xF88\x11a\0\xADW\x80c\xAD\x1C\x8A\x86\x11a\0|W\x80c\xAD\x1C\x8A\x86\x14a\x05\xBDW\x80c\xD5Gt\x1F\x14a\x05\xDBW\x80c\xDA\x19\xDD\xFB\x14a\x05\xF7W\x80c\xDF8\x06\x93\x14a\x06\x15W\x80c\xF8\xAE\x93\xB4\x14a\x063Wa\x02\x06V[\x80c\x90r\xF88\x14a\x053W\x80c\x91\xD1HT\x14a\x05QW\x80c\x97z\x80p\x14a\x05\x81W\x80c\xA2\x17\xFD\xDF\x14a\x05\x9FWa\x02\x06V[\x80c\x85\xCB\x11\x91\x11a\0\xE9W\x80c\x85\xCB\x11\x91\x14a\x04\xABW\x80c\x8C\x156\xDF\x14a\x04\xC9W\x80c\x8D\xEB8\x93\x14a\x04\xE7W\x80c\x8E\x8D\xFD\x16\x14a\x05\x03Wa\x02\x06V[\x80cu\xB28\xFC\x14a\x043W\x80c|\xAD\xF6\x9F\x14a\x04QW\x80c}J\x03\xBD\x14a\x04oW\x80c\x7F\x90 \x9F\x14a\x04\x8DWa\x02\x06V[\x80c.H\x85\xE8\x11a\x01\x9DW\x80cB\x16\xE7:\x11a\x01lW\x80cB\x16\xE7:\x14a\x03\xA3W\x80cQ\xAD\n\x80\x14a\x03\xC1W\x80cZ\xF2\x7Fy\x14a\x03\xDDW\x80cpH\x02u\x14a\x03\xFBW\x80ct\xBC\x819\x14a\x04\x17Wa\x02\x06V[\x80c.H\x85\xE8\x14a\x03\x1DW\x80c//\xF1]\x14a\x03;W\x80c6V\x8A\xBE\x14a\x03WW\x80c>\xBFy\x85\x14a\x03sWa\x02\x06V[\x80c\x19;\x98=\x11a\x01\xD9W\x80c\x19;\x98=\x14a\x02\x93W\x80c$\x8A\x9C\xA3\x14a\x02\xB1W\x80c&h\xF3\x05\x14a\x02\xE1W\x80c,\x0B\x8B\xF7\x14a\x02\xFFWa\x02\x06V[\x80c\x01\xFF\xC9\xA7\x14a\x02\x0BW\x80c\x11\xEE\x8F\xF7\x14a\x02;W\x80c\x16\xF7k\xBF\x14a\x02YW\x80c\x17\x85\xF5<\x14a\x02wW[`\0\x80\xFD[a\x02%`\x04\x806\x03\x81\x01\x90a\x02 \x91\x90a\x15\xD1V[a\x06QV[`@Qa\x022\x91\x90a\x16\x19V[`@Q\x80\x91\x03\x90\xF3[a\x02Ca\x06\xCBV[`@Qa\x02P\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02aa\x06\xEFV[`@Qa\x02n\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a\x16\xC6V[a\x07\x13V[\0[a\x02\x9Ba\x07\xD9V[`@Qa\x02\xA8\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02\xCB`\x04\x806\x03\x81\x01\x90a\x02\xC6\x91\x90a\x17\x1FV[a\x07\xFDV[`@Qa\x02\xD8\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE9a\x08\x1CV[`@Qa\x02\xF6\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03\x07a\x08@V[`@Qa\x03\x14\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03%a\x08dV[`@Qa\x032\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03U`\x04\x806\x03\x81\x01\x90a\x03P\x91\x90a\x17LV[a\x08\x88V[\0[a\x03q`\x04\x806\x03\x81\x01\x90a\x03l\x91\x90a\x17LV[a\x08\xA9V[\0[a\x03\x8D`\x04\x806\x03\x81\x01\x90a\x03\x88\x91\x90a\x17\xB1V[a\t,V[`@Qa\x03\x9A\x91\x90a\x18\0V[`@Q\x80\x91\x03\x90\xF3[a\x03\xABa\tnV[`@Qa\x03\xB8\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x03\xDB`\x04\x806\x03\x81\x01\x90a\x03\xD6\x91\x90a\x18\x1BV[a\t\x92V[\0[a\x03\xE5a\x0BDV[`@Qa\x03\xF2\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\x15`\x04\x806\x03\x81\x01\x90a\x04\x10\x91\x90a\x16\xC6V[a\x0BhV[\0[a\x041`\x04\x806\x03\x81\x01\x90a\x04,\x91\x90a\x18nV[a\x0B\xC0V[\0[a\x04;a\x0C\xA9V[`@Qa\x04H\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04Ya\x0C\xCDV[`@Qa\x04f\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04wa\x0C\xF1V[`@Qa\x04\x84\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\x95a\r\x15V[`@Qa\x04\xA2\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\xB3a\r9V[`@Qa\x04\xC0\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x04\xD1a\r]V[`@Qa\x04\xDE\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\x01`\x04\x806\x03\x81\x01\x90a\x04\xFC\x91\x90a\x18nV[a\r\x81V[\0[a\x05\x1D`\x04\x806\x03\x81\x01\x90a\x05\x18\x91\x90a\x17\xB1V[a\x0EbV[`@Qa\x05*\x91\x90a\x18\0V[`@Q\x80\x91\x03\x90\xF3[a\x05;a\x0E\xD5V[`@Qa\x05H\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05k`\x04\x806\x03\x81\x01\x90a\x05f\x91\x90a\x17LV[a\x0E\xF9V[`@Qa\x05x\x91\x90a\x16\x19V[`@Q\x80\x91\x03\x90\xF3[a\x05\x89a\x0FcV[`@Qa\x05\x96\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\xA7a\x0F\x87V[`@Qa\x05\xB4\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\xC5a\x0F\x8EV[`@Qa\x05\xD2\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x05\xF5`\x04\x806\x03\x81\x01\x90a\x05\xF0\x91\x90a\x17LV[a\x0F\xB2V[\0[a\x05\xFFa\x0F\xD3V[`@Qa\x06\x0C\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x06\x1Da\x0F\xF7V[`@Qa\x06*\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[a\x06;a\x10\x1BV[`@Qa\x06H\x91\x90a\x16MV[`@Q\x80\x91\x03\x90\xF3[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xC4WPa\x06\xC3\x82a\x10?V[[\x90P\x91\x90PV[\x7FX\xA0\x04N\x0E\xCD\x81\x02^9\x8B\xF1\x81Pu\xD1#L\xBA\xC3t\x96\x14\xB0\xB3:@L.\xE2\xBA\xBF\x81V[\x7F\xF1OC\x1D\xAD\xC8.}\xBC^7\x9Fq#NW5\xC9\x18~C'\xA7\xC6\xAC\x01MU\xD1\xB7rz\x81V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x07=\x81a\x10\xA9V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x07\xABW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xA2\x90a\x19\x1EV[`@Q\x80\x91\x03\x90\xFD[a\x07\xD5\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x10\xBDV[PPV[\x7Ft\xEB\x03\xCFc\x07P\x04\xAB\xBF\x06s\xB7\x96\xDEr<\x9E\xB4\xE7\xCA2\xA8\x05\x85y\x0E\x13K\xB9\x9F\x8C\x81V[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[\x7F\xB1\xF7\x98\x13\xBCv0\xA5*\xE9H\xBC\x99x\x13\x97\xE4\t\xD0\xDD5!\x95;\xF7\xD8\xD7\xA2\xDBaG\xF7\x81V[\x7F\xB7\xB4\xFD\xE9\x94M<\x13\xE9\xA7\x885C\x1C3\xA5\x08M\x90\xA7\xF0\xC7=\xEFv\xD7\x88c\x15\xFE\x87\xB0\x81V[\x7F\xB91\xB2q\x9A\xEB*e\xA5\x03_\xA0\xA1\x90\xBF\xDCL\x86\"\xCE\x8C\xBF\xF7\xA3\xD1\xABBS\x1F\xB1\xA9\x18\x81V[a\x08\x91\x82a\x07\xFDV[a\x08\x9A\x81a\x10\xA9V[a\x08\xA4\x83\x83a\x11\x9EV[PPPV[a\x08\xB1a\x12~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\x1EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x15\x90a\x19\xB0V[`@Q\x80\x91\x03\x90\xFD[a\t(\x82\x82a\x10\xBDV[PPV[`\x02` R\x81`\0R`@`\0 ` R\x80`\0R`@`\0 `\0\x91P\x91P\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[\x7FLA\xAEEK\xEBk\xBB\xE9\xBEP\xAC\xCC\x95z;\x156\xE4\x8B\x83Z\x86\x91\x9A\xF9\x81\xB5$M\xB7U\x81V[a\t\xBC\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x0E\xF9V[a\t\xF2W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x15\x15`\x01`\0\x84`\x02\x81\x11\x15a\n\rWa\n\x0Ca\x19\xD0V[[`\x02\x81\x11\x15a\n\x1FWa\n\x1Ea\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a\n}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nt\x90a\x1AqV[`@Q\x80\x91\x03\x90\xFD[\x80`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x84`\x02\x81\x11\x15a\n\xA6Wa\n\xA5a\x19\xD0V[[`\x02\x81\x11\x15a\n\xB8Wa\n\xB7a\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F3\xF0\x14\x89\x0F\x10\x92)\xBB\xCF\x8D\xD4r\x04\xC1S\xA2\xC0\xFF\x1CW*a\xDE\"\r\x103e0\xF5=\x83\x83\x83`@Qa\x0B7\x93\x92\x91\x90a\x1A\xD9V[`@Q\x80\x91\x03\x90\xA1PPPV[\x7F\xA2\xC772\xDEez\xD0\xF3n\r\xDB\xB2q\x0FK\x13\xE8\xDD\xE4d!8k\xB9-\x1E\x17\x9D\xAEMM\x81V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x0B\x92\x81a\x10\xA9V[a\x0B\xBC\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x11\x9EV[PPV[a\x0B\xEA\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x0E\xF9V[a\x0C W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80`\0\x83`\x02\x81\x11\x15a\x0C8Wa\x0C7a\x19\xD0V[[`\x02\x81\x11\x15a\x0CJWa\x0CIa\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\x0C\x9E\x91\x90a\x1B\x10V[`@Q\x80\x91\x03\x90\xA1PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[\x7Ft\x84]\xE3|\xFA\xBD5v3!KG\xFA\x91\xCC\xD1\x9B\x05\xB7\xC5\xA0\x8A\xC2,\x18\x7F\x81\x1F\xB6+\xCA\x81V[\x7F\x9F5\xEF>\x0C&R\xA8\xBB\x87G\xD9/@\x7F\xCD9\xA7v\x8D\xAC\xC7\xF1e\x81\xC7\xA7\x1F\x10>Ub\x81V[\x7F\xC6gO\x98\xBA5\xC0\x1C\x13\x0E\x08\x19]\xD2lpF`7G:\x06\x8CZ\xAAG\nx=\x99\xC1l\x81V[\x7F\xAEy\xA95sp\x12\xD0f\xE7\x1802i.R\x1F\xFE\x1A\xDE+\xED\xA2g\xE2>\x02\xB1\xD6\xE9\x11\x87\x81V[\x7F\xAA\x06\xD1\x08\xDB\xD7\xBF\x97k\x16\xB7\xBFZ\xDB)\xD2\xD0\xEF,8\\\xA8\xB9\xD83\xCC\x80/3\x94-r\x81V[a\r\xAB\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x0E\xF9V[a\r\xE1W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x82`\x02\x81\x11\x15a\r\xF8Wa\r\xF7a\x19\xD0V[[`\x02\x81\x11\x15a\x0E\nWa\x0E\ta\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x81`@Qa\x0EW\x91\x90a\x1B\x10V[`@Q\x80\x91\x03\x90\xA1PV[`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x83`\x02\x81\x11\x15a\x0E\x8CWa\x0E\x8Ba\x19\xD0V[[`\x02\x81\x11\x15a\x0E\x9EWa\x0E\x9Da\x19\xD0V[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x92\x91PPV[\x7FT\x95<#\x06\x8B\x8F\xC4\xC0sc\x01\xB5\x0F\x10\x02}kF\x93'\xDE\x1F\xD4(A\xA5\x07+\x1B\xCE\xBE\x81V[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[\x7F'\xD7d\xEA*J8eCK\xBFJ9\x11\x10\x14\x96D\xBE1D\x8F4y\xFD\x15\xB4C\x88uWe\x81V[`\0\x80\x1B\x81V[\x7F:h\xDB\xFD\x8B\xBBd\x01\\B\xBC\x13\x1C8\x8D\xEAye\xE2\x8C\x10\x04\xD0\x9B9\xF5\x95\0\xC3\xA7c\xEC\x81V[a\x0F\xBB\x82a\x07\xFDV[a\x0F\xC4\x81a\x10\xA9V[a\x0F\xCE\x83\x83a\x10\xBDV[PPPV[\x7F\x08\t\t\xC1\x8C\x95\x8C\xE5\xA2\xD3d\x81ix$\xE4w1\x93#\xD01T\xCE\xBA;x\xF2\x8Aa\x88{\x81V[\x7F\xB4\xBF\x99\x9Bh\xD8\x08]\xBB\xF7\xA0\xEC/Z-f\x08s\x93[\xDF\x1E\xD0\x8E\xB4!\xACm\xCB\xC0\x03b\x81V[\x7F\xDD[\x9B\x8A^\x8E\x01\xF2\x96.\xD7\xE9\x83\xD5\x8F\xE3.\x1Ff\xAA\x88\xDDz\xB3\x07p\xFA\x9Bw\xDArC\x81V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a\x10\xBA\x81a\x10\xB5a\x12~V[a\x12\x86V[PV[a\x10\xC7\x82\x82a\x0E\xF9V[\x15a\x11\x9AW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x11?a\x12~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a\x11\xA8\x82\x82a\x0E\xF9V[a\x12zW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x12\x1Fa\x12~V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\x12\x90\x82\x82a\x0E\xF9V[a\x13\x07Wa\x12\x9D\x81a\x13\x0BV[a\x12\xAB\x83`\0\x1C` a\x138V[`@Q` \x01a\x12\xBC\x92\x91\x90a\x1C4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xFE\x91\x90a\x1C\xB8V[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x131\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x138V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a\x13K\x91\x90a\x1D\x13V[a\x13U\x91\x90a\x1DUV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13nWa\x13ma\x1D\x89V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13\xA0W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x13\xD8Wa\x13\xD7a\x1D\xB8V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x14<Wa\x14;a\x1D\xB8V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\x14|\x91\x90a\x1D\x13V[a\x14\x86\x91\x90a\x1DUV[\x90P[`\x01\x81\x11\x15a\x15&W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\x14\xC8Wa\x14\xC7a\x1D\xB8V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x14\xDFWa\x14\xDEa\x1D\xB8V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x15\x1F\x90a\x1D\xE7V[\x90Pa\x14\x89V[P`\0\x84\x14a\x15jW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15a\x90a\x1E\\V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x15\xAE\x81a\x15yV[\x81\x14a\x15\xB9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x15\xCB\x81a\x15\xA5V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xE7Wa\x15\xE6a\x15tV[[`\0a\x15\xF5\x84\x82\x85\x01a\x15\xBCV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x16\x13\x81a\x15\xFEV[\x82RPPV[`\0` \x82\x01\x90Pa\x16.`\0\x83\x01\x84a\x16\nV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x16G\x81a\x164V[\x82RPPV[`\0` \x82\x01\x90Pa\x16b`\0\x83\x01\x84a\x16>V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x16\x93\x82a\x16hV[\x90P\x91\x90PV[a\x16\xA3\x81a\x16\x88V[\x81\x14a\x16\xAEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x16\xC0\x81a\x16\x9AV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xDCWa\x16\xDBa\x15tV[[`\0a\x16\xEA\x84\x82\x85\x01a\x16\xB1V[\x91PP\x92\x91PPV[a\x16\xFC\x81a\x164V[\x81\x14a\x17\x07W`\0\x80\xFD[PV[`\0\x815\x90Pa\x17\x19\x81a\x16\xF3V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x175Wa\x174a\x15tV[[`\0a\x17C\x84\x82\x85\x01a\x17\nV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17cWa\x17ba\x15tV[[`\0a\x17q\x85\x82\x86\x01a\x17\nV[\x92PP` a\x17\x82\x85\x82\x86\x01a\x16\xB1V[\x91PP\x92P\x92\x90PV[`\x03\x81\x10a\x17\x99W`\0\x80\xFD[PV[`\0\x815\x90Pa\x17\xAB\x81a\x17\x8CV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xC8Wa\x17\xC7a\x15tV[[`\0a\x17\xD6\x85\x82\x86\x01a\x17\nV[\x92PP` a\x17\xE7\x85\x82\x86\x01a\x17\x9CV[\x91PP\x92P\x92\x90PV[a\x17\xFA\x81a\x16\x88V[\x82RPPV[`\0` \x82\x01\x90Pa\x18\x15`\0\x83\x01\x84a\x17\xF1V[\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x184Wa\x183a\x15tV[[`\0a\x18B\x86\x82\x87\x01a\x17\nV[\x93PP` a\x18S\x86\x82\x87\x01a\x17\x9CV[\x92PP`@a\x18d\x86\x82\x87\x01a\x16\xB1V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x18\x84Wa\x18\x83a\x15tV[[`\0a\x18\x92\x84\x82\x85\x01a\x17\x9CV[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCannot remove self as admin.  Ha`\0\x82\x01R\x7Fve the new admin do it.\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x19\x08`7\x83a\x18\x9BV[\x91Pa\x19\x13\x82a\x18\xACV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x197\x81a\x18\xFBV[\x90P\x91\x90PV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x19\x9A`/\x83a\x18\x9BV[\x91Pa\x19\xA5\x82a\x19>V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x19\xC9\x81a\x19\x8DV[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x7FThe provided Env is not valid fo`\0\x82\x01R\x7Fr this contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1A[`/\x83a\x18\x9BV[\x91Pa\x1Af\x82a\x19\xFFV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1A\x8A\x81a\x1ANV[\x90P\x91\x90PV[`\x03\x81\x10a\x1A\xA2Wa\x1A\xA1a\x19\xD0V[[PV[`\0\x81\x90Pa\x1A\xB3\x82a\x1A\x91V[\x91\x90PV[`\0a\x1A\xC3\x82a\x1A\xA5V[\x90P\x91\x90PV[a\x1A\xD3\x81a\x1A\xB8V[\x82RPPV[`\0``\x82\x01\x90Pa\x1A\xEE`\0\x83\x01\x86a\x16>V[a\x1A\xFB` \x83\x01\x85a\x1A\xCAV[a\x1B\x08`@\x83\x01\x84a\x17\xF1V[\x94\x93PPPPV[`\0` \x82\x01\x90Pa\x1B%`\0\x83\x01\x84a\x1A\xCAV[\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1Bl`\x17\x83a\x1B+V[\x91Pa\x1Bw\x82a\x1B6V[`\x17\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0[\x83\x81\x10\x15a\x1B\xABW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1B\x90V[`\0\x84\x84\x01RPPPPV[`\0a\x1B\xC2\x82a\x1B\x82V[a\x1B\xCC\x81\x85a\x1B+V[\x93Pa\x1B\xDC\x81\x85` \x86\x01a\x1B\x8DV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1C\x1E`\x11\x83a\x1B+V[\x91Pa\x1C)\x82a\x1B\xE8V[`\x11\x82\x01\x90P\x91\x90PV[`\0a\x1C?\x82a\x1B_V[\x91Pa\x1CK\x82\x85a\x1B\xB7V[\x91Pa\x1CV\x82a\x1C\x11V[\x91Pa\x1Cb\x82\x84a\x1B\xB7V[\x91P\x81\x90P\x93\x92PPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x1C\x8A\x82a\x1B\x82V[a\x1C\x94\x81\x85a\x18\x9BV[\x93Pa\x1C\xA4\x81\x85` \x86\x01a\x1B\x8DV[a\x1C\xAD\x81a\x1CnV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\xD2\x81\x84a\x1C\x7FV[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x1D\x1E\x82a\x1C\xDAV[\x91Pa\x1D)\x83a\x1C\xDAV[\x92P\x82\x82\x02a\x1D7\x81a\x1C\xDAV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1DNWa\x1DMa\x1C\xE4V[[P\x92\x91PPV[`\0a\x1D`\x82a\x1C\xDAV[\x91Pa\x1Dk\x83a\x1C\xDAV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1D\x83Wa\x1D\x82a\x1C\xE4V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a\x1D\xF2\x82a\x1C\xDAV[\x91P`\0\x82\x03a\x1E\x05Wa\x1E\x04a\x1C\xE4V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a\x1EF` \x83a\x18\x9BV[\x91Pa\x1EQ\x82a\x1E\x10V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1Eu\x81a\x1E9V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xC0-S\\B\x14\x0C\xBB\xEE.\x86\xCC\xAA\xEE:\t\xDE\x07\0T-R\xEDR\\\xFC\xF8\xFB+S\x85tdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static CONTRACTRESOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ContractResolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ContractResolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ContractResolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ContractResolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ContractResolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ContractResolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ContractResolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONTRACTRESOLVER_ABI.clone(),
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
                CONTRACTRESOLVER_ABI.clone(),
                CONTRACTRESOLVER_BYTECODE.clone().into(),
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
        ///Calls the contract's `ALLOWLIST_CONTRACT` (0x7cadf69f) function
        pub fn allowlist_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([124, 173, 246, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BACKUP_RECOVERY_CONTRACT` (0x7d4a03bd) function
        pub fn backup_recovery_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([125, 74, 3, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CLONE_NET_CONTRACT` (0x193b983d) function
        pub fn clone_net_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([25, 59, 152, 61], ())
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
        ///Calls the contract's `DOMAIN_WALLET_REGISTRY` (0x4216e73a) function
        pub fn domain_wallet_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([66, 22, 231, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HD_KEY_DERIVER_CONTRACT` (0x85cb1191) function
        pub fn hd_key_deriver_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([133, 203, 17, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HOST_COMMANDS_CONTRACT` (0x5af27f79) function
        pub fn host_commands_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([90, 242, 127, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LIT_TOKEN_CONTRACT` (0xdf380693) function
        pub fn lit_token_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([223, 56, 6, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MULTI_SENDER_CONTRACT` (0xf8ae93b4) function
        pub fn multi_sender_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([248, 174, 147, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PAYMENT_DELEGATION_CONTRACT` (0x7f90209f) function
        pub fn payment_delegation_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([127, 144, 32, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_HELPER_CONTRACT` (0x977a8070) function
        pub fn pkp_helper_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([151, 122, 128, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_HELPER_V2_CONTRACT` (0x11ee8ff7) function
        pub fn pkp_helper_v2_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([17, 238, 143, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_NFT_CONTRACT` (0x2c0b8bf7) function
        pub fn pkp_nft_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([44, 11, 139, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_NFT_METADATA_CONTRACT` (0x16f76bbf) function
        pub fn pkp_nft_metadata_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([22, 247, 107, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_PERMISSIONS_CONTRACT` (0x9072f838) function
        pub fn pkp_permissions_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([144, 114, 248, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PUB_KEY_ROUTER_CONTRACT` (0x2668f305) function
        pub fn pub_key_router_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([38, 104, 243, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RATE_LIMIT_NFT_CONTRACT` (0x2e4885e8) function
        pub fn rate_limit_nft_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([46, 72, 133, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELEASE_REGISTER_CONTRACT` (0xad1c8a86) function
        pub fn release_register_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([173, 28, 138, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKING_BALANCES_CONTRACT` (0x8c1536df) function
        pub fn staking_balances_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([140, 21, 54, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKING_CONTRACT` (0xda19ddfb) function
        pub fn staking_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([218, 25, 221, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAdmin` (0x70480275) function
        pub fn add_admin(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 72, 2, 117], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAllowedEnv` (0x74bc8139) function
        pub fn add_allowed_env(
            &self,
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 188, 129, 57], env)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getContract` (0x8e8dfd16) function
        pub fn get_contract(
            &self,
            typ: [u8; 32],
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([142, 141, 253, 22], (typ, env))
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
        ///Calls the contract's `removeAdmin` (0x1785f53c) function
        pub fn remove_admin(
            &self,
            admin_being_removed: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 133, 245, 60], admin_being_removed)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAllowedEnv` (0x8deb3893) function
        pub fn remove_allowed_env(
            &self,
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 235, 56, 147], env)
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
        ///Calls the contract's `setContract` (0x51ad0a80) function
        pub fn set_contract(
            &self,
            typ: [u8; 32],
            env: u8,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 173, 10, 128], (typ, env, addr))
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
        ///Calls the contract's `typeAddresses` (0x3ebf7985) function
        pub fn type_addresses(
            &self,
            p0: [u8; 32],
            p1: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([62, 191, 121, 133], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AllowedEnvAdded` event
        pub fn allowed_env_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedEnvAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedEnvRemoved` event
        pub fn allowed_env_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedEnvRemovedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `SetContract` event
        pub fn set_contract_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetContractFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractResolverEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ContractResolver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AdminRoleRequired` with signature `AdminRoleRequired()` and selector `0xc890f84a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AdminRoleRequired", abi = "AdminRoleRequired()")]
    pub struct AdminRoleRequired;
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
    #[ethevent(name = "AllowedEnvAdded", abi = "AllowedEnvAdded(uint8)")]
    pub struct AllowedEnvAddedFilter {
        pub env: u8,
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
    #[ethevent(name = "AllowedEnvRemoved", abi = "AllowedEnvRemoved(uint8)")]
    pub struct AllowedEnvRemovedFilter {
        pub env: u8,
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
    #[ethevent(name = "SetContract", abi = "SetContract(bytes32,uint8,address)")]
    pub struct SetContractFilter {
        pub typ: [u8; 32],
        pub env: u8,
        pub addr: ::ethers::core::types::Address,
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
    pub enum ContractResolverEvents {
        AllowedEnvAddedFilter(AllowedEnvAddedFilter),
        AllowedEnvRemovedFilter(AllowedEnvRemovedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        SetContractFilter(SetContractFilter),
    }
    impl ::ethers::contract::EthLogDecode for ContractResolverEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllowedEnvAddedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::AllowedEnvAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowedEnvRemovedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::AllowedEnvRemovedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SetContractFilter::decode_log(log) {
                return Ok(ContractResolverEvents::SetContractFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ContractResolverEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowedEnvAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedEnvRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowedEnvAddedFilter> for ContractResolverEvents {
        fn from(value: AllowedEnvAddedFilter) -> Self {
            Self::AllowedEnvAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedEnvRemovedFilter> for ContractResolverEvents {
        fn from(value: AllowedEnvRemovedFilter) -> Self {
            Self::AllowedEnvRemovedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ContractResolverEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ContractResolverEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ContractResolverEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<SetContractFilter> for ContractResolverEvents {
        fn from(value: SetContractFilter) -> Self {
            Self::SetContractFilter(value)
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
    ///Container type for all input parameters for the `ALLOWLIST_CONTRACT` function with signature `ALLOWLIST_CONTRACT()` and selector `0x7cadf69f`
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
    #[ethcall(name = "ALLOWLIST_CONTRACT", abi = "ALLOWLIST_CONTRACT()")]
    pub struct AllowlistContractCall;
    ///Container type for all input parameters for the `BACKUP_RECOVERY_CONTRACT` function with signature `BACKUP_RECOVERY_CONTRACT()` and selector `0x7d4a03bd`
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
    #[ethcall(name = "BACKUP_RECOVERY_CONTRACT", abi = "BACKUP_RECOVERY_CONTRACT()")]
    pub struct BackupRecoveryContractCall;
    ///Container type for all input parameters for the `CLONE_NET_CONTRACT` function with signature `CLONE_NET_CONTRACT()` and selector `0x193b983d`
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
    #[ethcall(name = "CLONE_NET_CONTRACT", abi = "CLONE_NET_CONTRACT()")]
    pub struct CloneNetContractCall;
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
    ///Container type for all input parameters for the `DOMAIN_WALLET_REGISTRY` function with signature `DOMAIN_WALLET_REGISTRY()` and selector `0x4216e73a`
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
    #[ethcall(name = "DOMAIN_WALLET_REGISTRY", abi = "DOMAIN_WALLET_REGISTRY()")]
    pub struct DomainWalletRegistryCall;
    ///Container type for all input parameters for the `HD_KEY_DERIVER_CONTRACT` function with signature `HD_KEY_DERIVER_CONTRACT()` and selector `0x85cb1191`
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
    #[ethcall(name = "HD_KEY_DERIVER_CONTRACT", abi = "HD_KEY_DERIVER_CONTRACT()")]
    pub struct HdKeyDeriverContractCall;
    ///Container type for all input parameters for the `HOST_COMMANDS_CONTRACT` function with signature `HOST_COMMANDS_CONTRACT()` and selector `0x5af27f79`
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
    #[ethcall(name = "HOST_COMMANDS_CONTRACT", abi = "HOST_COMMANDS_CONTRACT()")]
    pub struct HostCommandsContractCall;
    ///Container type for all input parameters for the `LIT_TOKEN_CONTRACT` function with signature `LIT_TOKEN_CONTRACT()` and selector `0xdf380693`
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
    #[ethcall(name = "LIT_TOKEN_CONTRACT", abi = "LIT_TOKEN_CONTRACT()")]
    pub struct LitTokenContractCall;
    ///Container type for all input parameters for the `MULTI_SENDER_CONTRACT` function with signature `MULTI_SENDER_CONTRACT()` and selector `0xf8ae93b4`
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
    #[ethcall(name = "MULTI_SENDER_CONTRACT", abi = "MULTI_SENDER_CONTRACT()")]
    pub struct MultiSenderContractCall;
    ///Container type for all input parameters for the `PAYMENT_DELEGATION_CONTRACT` function with signature `PAYMENT_DELEGATION_CONTRACT()` and selector `0x7f90209f`
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
        name = "PAYMENT_DELEGATION_CONTRACT",
        abi = "PAYMENT_DELEGATION_CONTRACT()"
    )]
    pub struct PaymentDelegationContractCall;
    ///Container type for all input parameters for the `PKP_HELPER_CONTRACT` function with signature `PKP_HELPER_CONTRACT()` and selector `0x977a8070`
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
    #[ethcall(name = "PKP_HELPER_CONTRACT", abi = "PKP_HELPER_CONTRACT()")]
    pub struct PkpHelperContractCall;
    ///Container type for all input parameters for the `PKP_HELPER_V2_CONTRACT` function with signature `PKP_HELPER_V2_CONTRACT()` and selector `0x11ee8ff7`
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
    #[ethcall(name = "PKP_HELPER_V2_CONTRACT", abi = "PKP_HELPER_V2_CONTRACT()")]
    pub struct PkpHelperV2ContractCall;
    ///Container type for all input parameters for the `PKP_NFT_CONTRACT` function with signature `PKP_NFT_CONTRACT()` and selector `0x2c0b8bf7`
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
    #[ethcall(name = "PKP_NFT_CONTRACT", abi = "PKP_NFT_CONTRACT()")]
    pub struct PkpNftContractCall;
    ///Container type for all input parameters for the `PKP_NFT_METADATA_CONTRACT` function with signature `PKP_NFT_METADATA_CONTRACT()` and selector `0x16f76bbf`
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
    #[ethcall(name = "PKP_NFT_METADATA_CONTRACT", abi = "PKP_NFT_METADATA_CONTRACT()")]
    pub struct PkpNftMetadataContractCall;
    ///Container type for all input parameters for the `PKP_PERMISSIONS_CONTRACT` function with signature `PKP_PERMISSIONS_CONTRACT()` and selector `0x9072f838`
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
    #[ethcall(name = "PKP_PERMISSIONS_CONTRACT", abi = "PKP_PERMISSIONS_CONTRACT()")]
    pub struct PkpPermissionsContractCall;
    ///Container type for all input parameters for the `PUB_KEY_ROUTER_CONTRACT` function with signature `PUB_KEY_ROUTER_CONTRACT()` and selector `0x2668f305`
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
    #[ethcall(name = "PUB_KEY_ROUTER_CONTRACT", abi = "PUB_KEY_ROUTER_CONTRACT()")]
    pub struct PubKeyRouterContractCall;
    ///Container type for all input parameters for the `RATE_LIMIT_NFT_CONTRACT` function with signature `RATE_LIMIT_NFT_CONTRACT()` and selector `0x2e4885e8`
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
    #[ethcall(name = "RATE_LIMIT_NFT_CONTRACT", abi = "RATE_LIMIT_NFT_CONTRACT()")]
    pub struct RateLimitNftContractCall;
    ///Container type for all input parameters for the `RELEASE_REGISTER_CONTRACT` function with signature `RELEASE_REGISTER_CONTRACT()` and selector `0xad1c8a86`
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
    #[ethcall(name = "RELEASE_REGISTER_CONTRACT", abi = "RELEASE_REGISTER_CONTRACT()")]
    pub struct ReleaseRegisterContractCall;
    ///Container type for all input parameters for the `STAKING_BALANCES_CONTRACT` function with signature `STAKING_BALANCES_CONTRACT()` and selector `0x8c1536df`
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
    #[ethcall(name = "STAKING_BALANCES_CONTRACT", abi = "STAKING_BALANCES_CONTRACT()")]
    pub struct StakingBalancesContractCall;
    ///Container type for all input parameters for the `STAKING_CONTRACT` function with signature `STAKING_CONTRACT()` and selector `0xda19ddfb`
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
    #[ethcall(name = "STAKING_CONTRACT", abi = "STAKING_CONTRACT()")]
    pub struct StakingContractCall;
    ///Container type for all input parameters for the `addAdmin` function with signature `addAdmin(address)` and selector `0x70480275`
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
    #[ethcall(name = "addAdmin", abi = "addAdmin(address)")]
    pub struct AddAdminCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addAllowedEnv` function with signature `addAllowedEnv(uint8)` and selector `0x74bc8139`
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
    #[ethcall(name = "addAllowedEnv", abi = "addAllowedEnv(uint8)")]
    pub struct AddAllowedEnvCall {
        pub env: u8,
    }
    ///Container type for all input parameters for the `getContract` function with signature `getContract(bytes32,uint8)` and selector `0x8e8dfd16`
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
    #[ethcall(name = "getContract", abi = "getContract(bytes32,uint8)")]
    pub struct GetContractCall {
        pub typ: [u8; 32],
        pub env: u8,
    }
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
    ///Container type for all input parameters for the `removeAdmin` function with signature `removeAdmin(address)` and selector `0x1785f53c`
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
    #[ethcall(name = "removeAdmin", abi = "removeAdmin(address)")]
    pub struct RemoveAdminCall {
        pub admin_being_removed: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeAllowedEnv` function with signature `removeAllowedEnv(uint8)` and selector `0x8deb3893`
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
    #[ethcall(name = "removeAllowedEnv", abi = "removeAllowedEnv(uint8)")]
    pub struct RemoveAllowedEnvCall {
        pub env: u8,
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
    ///Container type for all input parameters for the `setContract` function with signature `setContract(bytes32,uint8,address)` and selector `0x51ad0a80`
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
    #[ethcall(name = "setContract", abi = "setContract(bytes32,uint8,address)")]
    pub struct SetContractCall {
        pub typ: [u8; 32],
        pub env: u8,
        pub addr: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `typeAddresses` function with signature `typeAddresses(bytes32,uint8)` and selector `0x3ebf7985`
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
    #[ethcall(name = "typeAddresses", abi = "typeAddresses(bytes32,uint8)")]
    pub struct TypeAddressesCall(pub [u8; 32], pub u8);
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
    pub enum ContractResolverCalls {
        AdminRole(AdminRoleCall),
        AllowlistContract(AllowlistContractCall),
        BackupRecoveryContract(BackupRecoveryContractCall),
        CloneNetContract(CloneNetContractCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        DomainWalletRegistry(DomainWalletRegistryCall),
        HdKeyDeriverContract(HdKeyDeriverContractCall),
        HostCommandsContract(HostCommandsContractCall),
        LitTokenContract(LitTokenContractCall),
        MultiSenderContract(MultiSenderContractCall),
        PaymentDelegationContract(PaymentDelegationContractCall),
        PkpHelperContract(PkpHelperContractCall),
        PkpHelperV2Contract(PkpHelperV2ContractCall),
        PkpNftContract(PkpNftContractCall),
        PkpNftMetadataContract(PkpNftMetadataContractCall),
        PkpPermissionsContract(PkpPermissionsContractCall),
        PubKeyRouterContract(PubKeyRouterContractCall),
        RateLimitNftContract(RateLimitNftContractCall),
        ReleaseRegisterContract(ReleaseRegisterContractCall),
        StakingBalancesContract(StakingBalancesContractCall),
        StakingContract(StakingContractCall),
        AddAdmin(AddAdminCall),
        AddAllowedEnv(AddAllowedEnvCall),
        GetContract(GetContractCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RemoveAdmin(RemoveAdminCall),
        RemoveAllowedEnv(RemoveAllowedEnvCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetContract(SetContractCall),
        SupportsInterface(SupportsInterfaceCall),
        TypeAddresses(TypeAddressesCall),
    }
    impl ::ethers::core::abi::AbiDecode for ContractResolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded) = <AllowlistContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowlistContract(decoded));
            }
            if let Ok(decoded) = <BackupRecoveryContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BackupRecoveryContract(decoded));
            }
            if let Ok(decoded) = <CloneNetContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CloneNetContract(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <DomainWalletRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainWalletRegistry(decoded));
            }
            if let Ok(decoded) = <HdKeyDeriverContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HdKeyDeriverContract(decoded));
            }
            if let Ok(decoded) = <HostCommandsContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HostCommandsContract(decoded));
            }
            if let Ok(decoded) = <LitTokenContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LitTokenContract(decoded));
            }
            if let Ok(decoded) = <MultiSenderContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MultiSenderContract(decoded));
            }
            if let Ok(decoded) = <PaymentDelegationContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PaymentDelegationContract(decoded));
            }
            if let Ok(decoded) = <PkpHelperContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpHelperContract(decoded));
            }
            if let Ok(decoded) = <PkpHelperV2ContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpHelperV2Contract(decoded));
            }
            if let Ok(decoded) = <PkpNftContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpNftContract(decoded));
            }
            if let Ok(decoded) = <PkpNftMetadataContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpNftMetadataContract(decoded));
            }
            if let Ok(decoded) = <PkpPermissionsContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpPermissionsContract(decoded));
            }
            if let Ok(decoded) = <PubKeyRouterContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubKeyRouterContract(decoded));
            }
            if let Ok(decoded) = <RateLimitNftContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RateLimitNftContract(decoded));
            }
            if let Ok(decoded) = <ReleaseRegisterContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReleaseRegisterContract(decoded));
            }
            if let Ok(decoded) = <StakingBalancesContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakingBalancesContract(decoded));
            }
            if let Ok(decoded) = <StakingContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakingContract(decoded));
            }
            if let Ok(decoded) = <AddAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAdmin(decoded));
            }
            if let Ok(decoded) = <AddAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAllowedEnv(decoded));
            }
            if let Ok(decoded) = <GetContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetContract(decoded));
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
            if let Ok(decoded) = <RemoveAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAdmin(decoded));
            }
            if let Ok(decoded) = <RemoveAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAllowedEnv(decoded));
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
            if let Ok(decoded) = <SetContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetContract(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <TypeAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TypeAddresses(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ContractResolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowlistContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BackupRecoveryContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CloneNetContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainWalletRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HdKeyDeriverContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HostCommandsContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LitTokenContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiSenderContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PaymentDelegationContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpHelperContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpHelperV2Contract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpNftContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpNftMetadataContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpPermissionsContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubKeyRouterContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RateLimitNftContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseRegisterContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingBalancesContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TypeAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ContractResolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowlistContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::BackupRecoveryContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CloneNetContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainWalletRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HdKeyDeriverContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HostCommandsContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LitTokenContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiSenderContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PaymentDelegationContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PkpHelperContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::PkpHelperV2Contract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PkpNftContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::PkpNftMetadataContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PkpPermissionsContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PubKeyRouterContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RateLimitNftContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReleaseRegisterContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingBalancesContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TypeAddresses(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for ContractResolverCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<AllowlistContractCall> for ContractResolverCalls {
        fn from(value: AllowlistContractCall) -> Self {
            Self::AllowlistContract(value)
        }
    }
    impl ::core::convert::From<BackupRecoveryContractCall> for ContractResolverCalls {
        fn from(value: BackupRecoveryContractCall) -> Self {
            Self::BackupRecoveryContract(value)
        }
    }
    impl ::core::convert::From<CloneNetContractCall> for ContractResolverCalls {
        fn from(value: CloneNetContractCall) -> Self {
            Self::CloneNetContract(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ContractResolverCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<DomainWalletRegistryCall> for ContractResolverCalls {
        fn from(value: DomainWalletRegistryCall) -> Self {
            Self::DomainWalletRegistry(value)
        }
    }
    impl ::core::convert::From<HdKeyDeriverContractCall> for ContractResolverCalls {
        fn from(value: HdKeyDeriverContractCall) -> Self {
            Self::HdKeyDeriverContract(value)
        }
    }
    impl ::core::convert::From<HostCommandsContractCall> for ContractResolverCalls {
        fn from(value: HostCommandsContractCall) -> Self {
            Self::HostCommandsContract(value)
        }
    }
    impl ::core::convert::From<LitTokenContractCall> for ContractResolverCalls {
        fn from(value: LitTokenContractCall) -> Self {
            Self::LitTokenContract(value)
        }
    }
    impl ::core::convert::From<MultiSenderContractCall> for ContractResolverCalls {
        fn from(value: MultiSenderContractCall) -> Self {
            Self::MultiSenderContract(value)
        }
    }
    impl ::core::convert::From<PaymentDelegationContractCall> for ContractResolverCalls {
        fn from(value: PaymentDelegationContractCall) -> Self {
            Self::PaymentDelegationContract(value)
        }
    }
    impl ::core::convert::From<PkpHelperContractCall> for ContractResolverCalls {
        fn from(value: PkpHelperContractCall) -> Self {
            Self::PkpHelperContract(value)
        }
    }
    impl ::core::convert::From<PkpHelperV2ContractCall> for ContractResolverCalls {
        fn from(value: PkpHelperV2ContractCall) -> Self {
            Self::PkpHelperV2Contract(value)
        }
    }
    impl ::core::convert::From<PkpNftContractCall> for ContractResolverCalls {
        fn from(value: PkpNftContractCall) -> Self {
            Self::PkpNftContract(value)
        }
    }
    impl ::core::convert::From<PkpNftMetadataContractCall> for ContractResolverCalls {
        fn from(value: PkpNftMetadataContractCall) -> Self {
            Self::PkpNftMetadataContract(value)
        }
    }
    impl ::core::convert::From<PkpPermissionsContractCall> for ContractResolverCalls {
        fn from(value: PkpPermissionsContractCall) -> Self {
            Self::PkpPermissionsContract(value)
        }
    }
    impl ::core::convert::From<PubKeyRouterContractCall> for ContractResolverCalls {
        fn from(value: PubKeyRouterContractCall) -> Self {
            Self::PubKeyRouterContract(value)
        }
    }
    impl ::core::convert::From<RateLimitNftContractCall> for ContractResolverCalls {
        fn from(value: RateLimitNftContractCall) -> Self {
            Self::RateLimitNftContract(value)
        }
    }
    impl ::core::convert::From<ReleaseRegisterContractCall> for ContractResolverCalls {
        fn from(value: ReleaseRegisterContractCall) -> Self {
            Self::ReleaseRegisterContract(value)
        }
    }
    impl ::core::convert::From<StakingBalancesContractCall> for ContractResolverCalls {
        fn from(value: StakingBalancesContractCall) -> Self {
            Self::StakingBalancesContract(value)
        }
    }
    impl ::core::convert::From<StakingContractCall> for ContractResolverCalls {
        fn from(value: StakingContractCall) -> Self {
            Self::StakingContract(value)
        }
    }
    impl ::core::convert::From<AddAdminCall> for ContractResolverCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddAllowedEnvCall> for ContractResolverCalls {
        fn from(value: AddAllowedEnvCall) -> Self {
            Self::AddAllowedEnv(value)
        }
    }
    impl ::core::convert::From<GetContractCall> for ContractResolverCalls {
        fn from(value: GetContractCall) -> Self {
            Self::GetContract(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ContractResolverCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ContractResolverCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ContractResolverCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<RemoveAdminCall> for ContractResolverCalls {
        fn from(value: RemoveAdminCall) -> Self {
            Self::RemoveAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedEnvCall> for ContractResolverCalls {
        fn from(value: RemoveAllowedEnvCall) -> Self {
            Self::RemoveAllowedEnv(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ContractResolverCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ContractResolverCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetContractCall> for ContractResolverCalls {
        fn from(value: SetContractCall) -> Self {
            Self::SetContract(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ContractResolverCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TypeAddressesCall> for ContractResolverCalls {
        fn from(value: TypeAddressesCall) -> Self {
            Self::TypeAddresses(value)
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
    ///Container type for all return fields from the `ALLOWLIST_CONTRACT` function with signature `ALLOWLIST_CONTRACT()` and selector `0x7cadf69f`
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
    pub struct AllowlistContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `BACKUP_RECOVERY_CONTRACT` function with signature `BACKUP_RECOVERY_CONTRACT()` and selector `0x7d4a03bd`
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
    pub struct BackupRecoveryContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CLONE_NET_CONTRACT` function with signature `CLONE_NET_CONTRACT()` and selector `0x193b983d`
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
    pub struct CloneNetContractReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `DOMAIN_WALLET_REGISTRY` function with signature `DOMAIN_WALLET_REGISTRY()` and selector `0x4216e73a`
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
    pub struct DomainWalletRegistryReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HD_KEY_DERIVER_CONTRACT` function with signature `HD_KEY_DERIVER_CONTRACT()` and selector `0x85cb1191`
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
    pub struct HdKeyDeriverContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HOST_COMMANDS_CONTRACT` function with signature `HOST_COMMANDS_CONTRACT()` and selector `0x5af27f79`
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
    pub struct HostCommandsContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `LIT_TOKEN_CONTRACT` function with signature `LIT_TOKEN_CONTRACT()` and selector `0xdf380693`
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
    pub struct LitTokenContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MULTI_SENDER_CONTRACT` function with signature `MULTI_SENDER_CONTRACT()` and selector `0xf8ae93b4`
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
    pub struct MultiSenderContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PAYMENT_DELEGATION_CONTRACT` function with signature `PAYMENT_DELEGATION_CONTRACT()` and selector `0x7f90209f`
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
    pub struct PaymentDelegationContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_HELPER_CONTRACT` function with signature `PKP_HELPER_CONTRACT()` and selector `0x977a8070`
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
    pub struct PkpHelperContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_HELPER_V2_CONTRACT` function with signature `PKP_HELPER_V2_CONTRACT()` and selector `0x11ee8ff7`
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
    pub struct PkpHelperV2ContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_NFT_CONTRACT` function with signature `PKP_NFT_CONTRACT()` and selector `0x2c0b8bf7`
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
    pub struct PkpNftContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_NFT_METADATA_CONTRACT` function with signature `PKP_NFT_METADATA_CONTRACT()` and selector `0x16f76bbf`
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
    pub struct PkpNftMetadataContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_PERMISSIONS_CONTRACT` function with signature `PKP_PERMISSIONS_CONTRACT()` and selector `0x9072f838`
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
    pub struct PkpPermissionsContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PUB_KEY_ROUTER_CONTRACT` function with signature `PUB_KEY_ROUTER_CONTRACT()` and selector `0x2668f305`
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
    pub struct PubKeyRouterContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `RATE_LIMIT_NFT_CONTRACT` function with signature `RATE_LIMIT_NFT_CONTRACT()` and selector `0x2e4885e8`
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
    pub struct RateLimitNftContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `RELEASE_REGISTER_CONTRACT` function with signature `RELEASE_REGISTER_CONTRACT()` and selector `0xad1c8a86`
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
    pub struct ReleaseRegisterContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKING_BALANCES_CONTRACT` function with signature `STAKING_BALANCES_CONTRACT()` and selector `0x8c1536df`
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
    pub struct StakingBalancesContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKING_CONTRACT` function with signature `STAKING_CONTRACT()` and selector `0xda19ddfb`
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
    pub struct StakingContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getContract` function with signature `getContract(bytes32,uint8)` and selector `0x8e8dfd16`
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
    pub struct GetContractReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `typeAddresses` function with signature `typeAddresses(bytes32,uint8)` and selector `0x3ebf7985`
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
    pub struct TypeAddressesReturn(pub ::ethers::core::types::Address);
}
