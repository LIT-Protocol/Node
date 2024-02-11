pub use domain_wallet_oracle::*;
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
pub mod domain_wallet_oracle {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/DomainWalletOracle.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_contractResolver"),
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
                    ::std::borrow::ToOwned::to_owned("checkRegistration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkRegistration"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uri"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getDomainIdByTokenId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDomainIdByTokenId",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDomainTokenIdByUri"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDomainTokenIdByUri",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uri"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDomainUri"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDomainUri"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                    ::std::borrow::ToOwned::to_owned("getExpiration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getExpiration"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPkpTokenId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPkpTokenId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRecord"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRecord"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                    ::std::borrow::ToOwned::to_owned("hasExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasExpired"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("isOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isRouted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isRouted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerDomain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("userId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uri"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ttl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("registerPKP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerPKP"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("removeDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeDomain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                            ],
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
                    ::std::borrow::ToOwned::to_owned("revokeDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeDomain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
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
                            ],
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
                    ::std::borrow::ToOwned::to_owned("updateDomainRecord"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateDomainRecord"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("record"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Expired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Expired"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ttl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Registered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Registered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ttl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Removed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Removed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Revoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Revoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DomainAlreadyRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DomainAlreadyRegistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uri"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pkpTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NonRegistryCaller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NonRegistryCaller"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("registryAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DOMAINWALLETORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0`\x04U`\0`\x05U`\0`\x06`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP4\x80\x15b\0\0EW`\0\x80\xFD[P`@Qb\0<\x898\x03\x80b\0<\x89\x839\x81\x81\x01`@R\x81\x01\x90b\0\0k\x91\x90b\0\x03\x08V[\x81`\x06`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x06`\x1Ca\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x02\x81\x11\x15b\0\0\xD4Wb\0\0\xD3b\0\x03OV[[\x02\x17\x90UPb\0\x01\x0B\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3b\0\x01\x13` \x1B` \x1CV[PPb\0\x03~V[b\0\x01%\x82\x82b\0\x02\x04` \x1B` \x1CV[b\0\x02\0W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPb\0\x01\xA5b\0\x02n` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x02\xA8\x82b\0\x02{V[\x90P\x91\x90PV[b\0\x02\xBA\x81b\0\x02\x9BV[\x81\x14b\0\x02\xC6W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02\xDA\x81b\0\x02\xAFV[\x92\x91PPV[`\x03\x81\x10b\0\x02\xEEW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\x02\x81b\0\x02\xE0V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x03\"Wb\0\x03!b\0\x02vV[[`\0b\0\x032\x85\x82\x86\x01b\0\x02\xC9V[\x92PP` b\0\x03E\x85\x82\x86\x01b\0\x02\xF1V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[a8\xFB\x80b\0\x03\x8E`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80coP\xCD\x97\x11a\0\xF9W\x80c\xA0[w_\x11a\0\x97W\x80c\xC9\xB2\x87I\x11a\0qW\x80c\xC9\xB2\x87I\x14a\x05\x89W\x80c\xD5Gt\x1F\x14a\x05\xB9W\x80c\xD9d\xA3u\x14a\x05\xD5W\x80c\xF4\x8D`\xCA\x14a\x06\x05Wa\x01\xC4V[\x80c\xA0[w_\x14a\x05\x0BW\x80c\xA2\x17\xFD\xDF\x14a\x05;W\x80c\xA2\xF4!\x04\x14a\x05YWa\x01\xC4V[\x80c\x88\x0B\xC4\xE6\x11a\0\xD3W\x80c\x88\x0B\xC4\xE6\x14a\x04qW\x80c\x91\xD1HT\x14a\x04\xA1W\x80c\x9A V\xF4\x14a\x04\xD1W\x80c\x9D\xCA\x002\x14a\x04\xEDWa\x01\xC4V[\x80coP\xCD\x97\x14a\x04\x07W\x80cpH\x02u\x14a\x047W\x80cu\xB28\xFC\x14a\x04SWa\x01\xC4V[\x80c'\xBD\x06\x9E\x11a\x01fW\x80c9\xC7c\x9C\x11a\x01@W\x80c9\xC7c\x9C\x14a\x03YW\x80cI\xFC\xBCT\x14a\x03\x89W\x80cP\xD1{^\x14a\x03\xB9W\x80cj0\0\xC4\x14a\x03\xD7Wa\x01\xC4V[\x80c'\xBD\x06\x9E\x14a\x03\x05W\x80c//\xF1]\x14a\x03!W\x80c6V\x8A\xBE\x14a\x03=Wa\x01\xC4V[\x80c\x06\x98_\xB1\x11a\x01\xA2W\x80c\x06\x98_\xB1\x14a\x02YW\x80c\x15\xD4dt\x14a\x02\x89W\x80c\x17\x85\xF5<\x14a\x02\xB9W\x80c$\x8A\x9C\xA3\x14a\x02\xD5Wa\x01\xC4V[\x80c\x01\xC6\xD05\x14a\x01\xC9W\x80c\x01\xFF\xC9\xA7\x14a\x01\xF9W\x80c\x03\xE9\xE6\t\x14a\x02)W[`\0\x80\xFD[a\x01\xE3`\x04\x806\x03\x81\x01\x90a\x01\xDE\x91\x90a#\xA7V[a\x065V[`@Qa\x01\xF0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02\x13`\x04\x806\x03\x81\x01\x90a\x02\x0E\x91\x90a$bV[a\x06bV[`@Qa\x02 \x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02C`\x04\x806\x03\x81\x01\x90a\x02>\x91\x90a#\xA7V[a\x06\xDCV[`@Qa\x02P\x91\x90a%\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x02s`\x04\x806\x03\x81\x01\x90a\x02n\x91\x90a&vV[a\x07\x84V[`@Qa\x02\x80\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xA3`\x04\x806\x03\x81\x01\x90a\x02\x9E\x91\x90a#\xA7V[a\t\x19V[`@Qa\x02\xB0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xD3`\x04\x806\x03\x81\x01\x90a\x02\xCE\x91\x90a'0V[a\x0C\xDAV[\0[a\x02\xEF`\x04\x806\x03\x81\x01\x90a\x02\xEA\x91\x90a'\x93V[a\r\xA0V[`@Qa\x02\xFC\x91\x90a'\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x03\x1F`\x04\x806\x03\x81\x01\x90a\x03\x1A\x91\x90a'\xEAV[a\r\xBFV[\0[a\x03;`\x04\x806\x03\x81\x01\x90a\x036\x91\x90a(3V[a\x0EEV[\0[a\x03W`\x04\x806\x03\x81\x01\x90a\x03R\x91\x90a(3V[a\x0EfV[\0[a\x03s`\x04\x806\x03\x81\x01\x90a\x03n\x91\x90a#\xA7V[a\x0E\xE9V[`@Qa\x03\x80\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x03\xA3`\x04\x806\x03\x81\x01\x90a\x03\x9E\x91\x90a#\xA7V[a\x0F\x16V[`@Qa\x03\xB0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x03\xC1a\x106V[`@Qa\x03\xCE\x91\x90a(\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x03\xF1`\x04\x806\x03\x81\x01\x90a\x03\xEC\x91\x90a#\xA7V[a\x10\\V[`@Qa\x03\xFE\x91\x90a%\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04!`\x04\x806\x03\x81\x01\x90a\x04\x1C\x91\x90a'\xEAV[a\x11\x04V[`@Qa\x04.\x91\x90a(\xFCV[`@Q\x80\x91\x03\x90\xF3[a\x04Q`\x04\x806\x03\x81\x01\x90a\x04L\x91\x90a'0V[a\x11,V[\0[a\x04[a\x11\x84V[`@Qa\x04h\x91\x90a'\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x04\x8B`\x04\x806\x03\x81\x01\x90a\x04\x86\x91\x90a#\xA7V[a\x11\xA8V[`@Qa\x04\x98\x91\x90a):V[`@Q\x80\x91\x03\x90\xF3[a\x04\xBB`\x04\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a(3V[a\x11\xDCV[`@Qa\x04\xC8\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x04\xEB`\x04\x806\x03\x81\x01\x90a\x04\xE6\x91\x90a)UV[a\x12FV[\0[a\x04\xF5a\x17\x1BV[`@Qa\x05\x02\x91\x90a*kV[`@Q\x80\x91\x03\x90\xF3[a\x05%`\x04\x806\x03\x81\x01\x90a\x05 \x91\x90a#\xA7V[a\x17.V[`@Qa\x052\x91\x90a(\xFCV[`@Q\x80\x91\x03\x90\xF3[a\x05Ca\x17NV[`@Qa\x05P\x91\x90a'\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x05s`\x04\x806\x03\x81\x01\x90a\x05n\x91\x90a*\xB2V[a\x17UV[`@Qa\x05\x80\x91\x90a(\xFCV[`@Q\x80\x91\x03\x90\xF3[a\x05\xA3`\x04\x806\x03\x81\x01\x90a\x05\x9E\x91\x90a*\xDFV[a\x17\x9FV[`@Qa\x05\xB0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x05\xD3`\x04\x806\x03\x81\x01\x90a\x05\xCE\x91\x90a(3V[a\x18\xDCV[\0[a\x05\xEF`\x04\x806\x03\x81\x01\x90a\x05\xEA\x91\x90a#\xA7V[a\x18\xFDV[`@Qa\x05\xFC\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x06\x1F`\x04\x806\x03\x81\x01\x90a\x06\x1A\x91\x90a#\xA7V[a\x1CZV[`@Qa\x06,\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xD5WPa\x06\xD4\x82a\x1C\x87V[[\x90P\x91\x90PV[```\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x80Ta\x06\xFF\x90a+NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07+\x90a+NV[\x80\x15a\x07xW\x80`\x1F\x10a\x07MWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07xV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07[W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0a\x07\x8Ea\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x07\xF0WPa\x07\xEE\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x08;Wa\x07\xFDa\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x082\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\x08sW`\0\x90Pa\t\x13V[\x81`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\x08\x96\x91\x90a-YV[P`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01`\x01`\0`\x02`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\t\r\x91\x90a.VV[P`\x01\x90P[\x92\x91PPV[`\0a\t#a\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\t\x85WPa\t\x83\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\t\xD0Wa\t\x92a\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xC7\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\n\x08W`\0\x90Pa\x0C\xD5V[`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x80\x82\x01`\0a\n[\x91\x90a#\0V[`\x01\x82\x01`\0a\x01\0\n\x81T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01`\0\x90U`\x03\x82\x01`\0a\n\x8E\x91\x90a#\0V[`\x04\x82\x01`\0\x90U`\x05\x82\x01`\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x05\x82\x01`\x01a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x06\x82\x01`\0a\n\xCE\x91\x90a#\0V[PP`\x03`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01`@Qa\x0B\t\x91\x90a/\xCCV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x90U`\0`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x90\x81a\x0C\x1E\x91\x90a/\xEEV[P`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\x0Cl\x91\x90a/\xEEV[P\x7F\xD4\xA9^(6z<\xE6i@B\xB0\x1D4\xE7\xBA\x01\x98\xBC\xDA\xCA\x1F\xFB8\xEA\xEC\x98\x98\xC7H\xB0\xAD\x81`\x01`\0\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Qa\x0C\xC7\x92\x91\x90a1uV[`@Q\x80\x91\x03\x90\xA1`\x01\x91PP[\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\r\x04\x81a\x1E5V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\rrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\ri\x90a2(V[`@Q\x80\x91\x03\x90\xFD[a\r\x9C\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1EIV[PPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[`\0`\x03\x82`@Qa\r\xD1\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14a\x0EBW\x80`\x03\x82`@Qa\r\xF6\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`@Q\x7F;\xCB\xF6\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E9\x92\x91\x90a2\x90V[`@Q\x80\x91\x03\x90\xFD[PV[a\x0EN\x82a\r\xA0V[a\x0EW\x81a\x1E5V[a\x0Ea\x83\x83a\x1EIV[PPPV[a\x0Ena\x1F)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0E\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xD2\x90a32V[`@Q\x80\x91\x03\x90\xFD[a\x0E\xE5\x82\x82a\x1F1V[PPV[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x80`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01T\x90P`\0`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01\x80Ta\x0FU\x90a+NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x81\x90a+NV[\x80\x15a\x0F\xCEW\x80`\x1F\x10a\x0F\xA3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xCEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0B\x83\x10\x90P\x80\x15a\x10)W\x7F\xB26?h\x8F{2U|d\xBF\xF9\0\x81\x95\x9F\x0B\xB6\xC2O\t\xCAFT$\xC5\xAF\xC5\x85M\xC5\x8D\x82\x86\x85`@Qa\x10\x15\x93\x92\x91\x90a3RV[`@Q\x80\x91\x03\x90\xA1`\x01\x93PPPPa\x101V[`\0\x93PPPP[\x91\x90PV[`\x06`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[```\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01\x80Ta\x10\x7F\x90a+NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xAB\x90a+NV[\x80\x15a\x10\xF8W\x80`\x1F\x10a\x10\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xF8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0`\x03\x82`@Qa\x11\x16\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x11V\x81a\x1E5V[a\x11\x80\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1EIV[PPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[a\x12Na\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x12\xB0WPa\x12\xAE\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x12\xFBWa\x12\xBDa\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xF2\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x84`@Qa\x13\r\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14a\x13~W\x82`\x03\x84`@Qa\x132\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`@Q\x7F;\xCB\xF6\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13u\x92\x91\x90a2\x90V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x06`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13\xA1\x91\x90a3\xBFV[`\x06`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0`\x06`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x84`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x90\x81a\x14\x1B\x91\x90a-YV[P\x82`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP\x83`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01\x90\x81a\x14\x82\x91\x90a-YV[P\x81`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01\x81\x90UP`\x01\x80`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x80`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01\x81`\0\x01\x90\x81a\x15\xD5\x91\x90a4\x11V[P`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02\x82\x01T\x81`\x02\x01U`\x03\x82\x01\x81`\x03\x01\x90\x81a\x167\x91\x90a4\x11V[P`\x04\x82\x01T\x81`\x04\x01U`\x05\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x05\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x06\x82\x01\x81`\x06\x01\x90\x81a\x16\xB0\x91\x90a4\x11V[P\x90PP\x82`\x03\x85`@Qa\x16\xC5\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x81\x90UP\x7F\xC2v\xB2\x07U\x8B\x88-n\x0EI\xFC>\"\x1E\x9F\xD1@\xCC\xEDy\xDD\x16\xFA\xE8\x8E\r\xE9\x7FJ/\r\x81\x86\x84\x86`@Qa\x17\x0C\x94\x93\x92\x91\x90a4\xF9V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x06`\x1C\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01T\x90P\x91\x90PV[`\0\x80\x1B\x81V[`\0`\x02`\0`\x01`\0\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01T\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01T\x90P\x91\x90PV[`\0a\x17\xA9a\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x18\x0BWPa\x18\t\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x18VWa\x18\x18a\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18M\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x01`\0\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\x18\xA2W`\0\x90Pa\x18\xD6V[\x81`\x01`\0\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP`\x01\x90P[\x92\x91PPV[a\x18\xE5\x82a\r\xA0V[a\x18\xEE\x81a\x1E5V[a\x18\xF8\x83\x83a\x1F1V[PPPV[`\0a\x19\x07a\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x19iWPa\x19g\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x19\xB4Wa\x19va\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xAB\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\x19\xECW`\0\x90Pa\x1CUV[`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x90\x81a\x1Ap\x91\x90a/\xEEV[P`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\x1A\xAA\x91\x90a/\xEEV[P`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01\x81`\0\x01\x90\x81a\x1B#\x91\x90a4\x11V[P`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02\x82\x01T\x81`\x02\x01U`\x03\x82\x01\x81`\x03\x01\x90\x81a\x1B\x85\x91\x90a4\x11V[P`\x04\x82\x01T\x81`\x04\x01U`\x05\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x05\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x06\x82\x01\x81`\x06\x01\x90\x81a\x1B\xFE\x91\x90a4\x11V[P\x90PP\x7F\xAE\x17\x95\x13\xBE\xB9\xB1\x9A\"\xA2\xC8/yM\xFDM7\x9C\xBB\xD2\xE7\xE9\\tJ\x1E\xA8\x9A\xD2\x002\xF7\x82`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01`@Qa\x1CH\x92\x91\x90a5EV[`@Q\x80\x91\x03\x90\xA1`\x01\x90P[\x91\x90PV[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[`\0`\x06`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x06`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x16\xE7:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xC2\x91\x90a5\x8AV[`\x06`\x1C\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xEF\x92\x91\x90a5\xB7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E0\x91\x90a5\xF5V[\x90P\x90V[a\x1EF\x81a\x1EAa\x1F)V[a \x12V[PV[a\x1ES\x82\x82a\x11\xDCV[a\x1F%W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1E\xCAa\x1F)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\x1F;\x82\x82a\x11\xDCV[\x15a \x0EW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1F\xB3a\x1F)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a \x1C\x82\x82a\x11\xDCV[a \x93Wa )\x81a \x97V[a 7\x83`\0\x1C` a \xC4V[`@Q` \x01a H\x92\x91\x90a6\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \x8A\x91\x90a7iV[`@Q\x80\x91\x03\x90\xFD[PPV[``a \xBD\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a \xC4V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a \xD7\x91\x90a7\x8BV[a \xE1\x91\x90a7\xCDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xFAWa \xF9a%KV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!,W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a!dWa!ca8\x01V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a!\xC8Wa!\xC7a8\x01V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\"\x08\x91\x90a7\x8BV[a\"\x12\x91\x90a7\xCDV[\x90P[`\x01\x81\x11\x15a\"\xB2W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\"TWa\"Sa8\x01V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\"kWa\"ja8\x01V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\"\xAB\x90a80V[\x90Pa\"\x15V[P`\0\x84\x14a\"\xF6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xED\x90a8\xA5V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[P\x80Ta#\x0C\x90a+NV[`\0\x82U\x80`\x1F\x10a#\x1EWPa#=V[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a#<\x91\x90a#@V[[PV[[\x80\x82\x11\x15a#YW`\0\x81`\0\x90UP`\x01\x01a#AV[P\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a#\x84\x81a#qV[\x81\x14a#\x8FW`\0\x80\xFD[PV[`\0\x815\x90Pa#\xA1\x81a#{V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\xBDWa#\xBCa#gV[[`\0a#\xCB\x84\x82\x85\x01a#\x92V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a#\xE9\x81a#\xD4V[\x82RPPV[`\0` \x82\x01\x90Pa$\x04`\0\x83\x01\x84a#\xE0V[\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a$?\x81a$\nV[\x81\x14a$JW`\0\x80\xFD[PV[`\0\x815\x90Pa$\\\x81a$6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a$xWa$wa#gV[[`\0a$\x86\x84\x82\x85\x01a$MV[\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a$\xC9W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa$\xAEV[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a$\xF1\x82a$\x8FV[a$\xFB\x81\x85a$\x9AV[\x93Pa%\x0B\x81\x85` \x86\x01a$\xABV[a%\x14\x81a$\xD5V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra%9\x81\x84a$\xE6V[\x90P\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a%\x83\x82a$\xD5V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a%\xA2Wa%\xA1a%KV[[\x80`@RPPPV[`\0a%\xB5a#]V[\x90Pa%\xC1\x82\x82a%zV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xE1Wa%\xE0a%KV[[a%\xEA\x82a$\xD5V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a&\x19a&\x14\x84a%\xC6V[a%\xABV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a&5Wa&4a%FV[[a&@\x84\x82\x85a%\xF7V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a&]Wa&\\a%AV[[\x815a&m\x84\x82` \x86\x01a&\x06V[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a&\x8DWa&\x8Ca#gV[[`\0a&\x9B\x85\x82\x86\x01a#\x92V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xBCWa&\xBBa#lV[[a&\xC8\x85\x82\x86\x01a&HV[\x91PP\x92P\x92\x90PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a&\xFD\x82a&\xD2V[\x90P\x91\x90PV[a'\r\x81a&\xF2V[\x81\x14a'\x18W`\0\x80\xFD[PV[`\0\x815\x90Pa'*\x81a'\x04V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'FWa'Ea#gV[[`\0a'T\x84\x82\x85\x01a'\x1BV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a'p\x81a']V[\x81\x14a'{W`\0\x80\xFD[PV[`\0\x815\x90Pa'\x8D\x81a'gV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'\xA9Wa'\xA8a#gV[[`\0a'\xB7\x84\x82\x85\x01a'~V[\x91PP\x92\x91PPV[a'\xC9\x81a']V[\x82RPPV[`\0` \x82\x01\x90Pa'\xE4`\0\x83\x01\x84a'\xC0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\0Wa'\xFFa#gV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1EWa(\x1Da#lV[[a(*\x84\x82\x85\x01a&HV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a(JWa(Ia#gV[[`\0a(X\x85\x82\x86\x01a'~V[\x92PP` a(i\x85\x82\x86\x01a'\x1BV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a(\x98a(\x93a(\x8E\x84a&\xD2V[a(sV[a&\xD2V[\x90P\x91\x90PV[`\0a(\xAA\x82a(}V[\x90P\x91\x90PV[`\0a(\xBC\x82a(\x9FV[\x90P\x91\x90PV[a(\xCC\x81a(\xB1V[\x82RPPV[`\0` \x82\x01\x90Pa(\xE7`\0\x83\x01\x84a(\xC3V[\x92\x91PPV[a(\xF6\x81a#qV[\x82RPPV[`\0` \x82\x01\x90Pa)\x11`\0\x83\x01\x84a(\xEDV[\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a)4\x81a)\x17V[\x82RPPV[`\0` \x82\x01\x90Pa)O`\0\x83\x01\x84a)+V[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)oWa)na#gV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x8DWa)\x8Ca#lV[[a)\x99\x87\x82\x88\x01a&HV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xBAWa)\xB9a#lV[[a)\xC6\x87\x82\x88\x01a&HV[\x93PP`@a)\xD7\x87\x82\x88\x01a#\x92V[\x92PP``a)\xE8\x87\x82\x88\x01a#\x92V[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a*4Wa*3a)\xF4V[[PV[`\0\x81\x90Pa*E\x82a*#V[\x91\x90PV[`\0a*U\x82a*7V[\x90P\x91\x90PV[a*e\x81a*JV[\x82RPPV[`\0` \x82\x01\x90Pa*\x80`\0\x83\x01\x84a*\\V[\x92\x91PPV[a*\x8F\x81a)\x17V[\x81\x14a*\x9AW`\0\x80\xFD[PV[`\0\x815\x90Pa*\xAC\x81a*\x86V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*\xC8Wa*\xC7a#gV[[`\0a*\xD6\x84\x82\x85\x01a*\x9DV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a*\xF6Wa*\xF5a#gV[[`\0a+\x04\x85\x82\x86\x01a*\x9DV[\x92PP` a+\x15\x85\x82\x86\x01a#\x92V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a+fW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+yWa+xa+\x1FV[[P\x91\x90PV[a+\x88\x81a&\xF2V[\x82RPPV[`\0`@\x82\x01\x90Pa+\xA3`\0\x83\x01\x85a+\x7FV[a+\xB0` \x83\x01\x84a+\x7FV[\x93\x92PPPV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a,\x19\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a+\xDCV[a,#\x86\x83a+\xDCV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0a,Va,Qa,L\x84a#qV[a(sV[a#qV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a,p\x83a,;V[a,\x84a,|\x82a,]V[\x84\x84Ta+\xE9V[\x82UPPPPV[`\0\x90V[a,\x99a,\x8CV[a,\xA4\x81\x84\x84a,gV[PPPV[[\x81\x81\x10\x15a,\xC8Wa,\xBD`\0\x82a,\x91V[`\x01\x81\x01\x90Pa,\xAAV[PPV[`\x1F\x82\x11\x15a-\rWa,\xDE\x81a+\xB7V[a,\xE7\x84a+\xCCV[\x81\x01` \x85\x10\x15a,\xF6W\x81\x90P[a-\na-\x02\x85a+\xCCV[\x83\x01\x82a,\xA9V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a-0`\0\x19\x84`\x08\x02a-\x12V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a-I\x83\x83a-\x1FV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a-b\x82a$\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-{Wa-za%KV[[a-\x85\x82Ta+NV[a-\x90\x82\x82\x85a,\xCCV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a-\xC3W`\0\x84\x15a-\xB1W\x82\x87\x01Q\x90P[a-\xBB\x85\x82a-=V[\x86UPa.#V[`\x1F\x19\x84\x16a-\xD1\x86a+\xB7V[`\0[\x82\x81\x10\x15a-\xF9W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa-\xD4V[\x86\x83\x10\x15a.\x16W\x84\x89\x01Qa.\x12`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x81T\x90Pa.:\x81a+NV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[\x81\x81\x03a.dWPPa/<V[a.m\x82a.+V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\x86Wa.\x85a%KV[[a.\x90\x82Ta+NV[a.\x9B\x82\x82\x85a,\xCCV[`\0`\x1F\x83\x11`\x01\x81\x14a.\xCAW`\0\x84\x15a.\xB8W\x82\x87\x01T\x90P[a.\xC2\x85\x82a-=V[\x86UPa/5V[`\x1F\x19\x84\x16a.\xD8\x87a.AV[\x96Pa.\xE3\x86a+\xB7V[`\0[\x82\x81\x10\x15a/\x0BW\x84\x89\x01T\x82U`\x01\x82\x01\x91P`\x01\x85\x01\x94P` \x81\x01\x90Pa.\xE6V[\x86\x83\x10\x15a/(W\x84\x89\x01Ta/$`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPP[V[`\0\x81\x90P\x92\x91PPV[`\0\x81Ta/V\x81a+NV[a/`\x81\x86a/>V[\x94P`\x01\x82\x16`\0\x81\x14a/{W`\x01\x81\x14a/\x90Wa/\xC3V[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa/\xC3V[a/\x99\x85a+\xB7V[`\0[\x83\x81\x10\x15a/\xBBW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa/\x9CV[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[`\0a/\xD8\x82\x84a/IV[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[a/\xF7\x82a/\xE3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\x10Wa0\x0Fa%KV[[a0\x1A\x82Ta+NV[a0%\x82\x82\x85a,\xCCV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a0XW`\0\x84\x15a0FW\x82\x87\x01Q\x90P[a0P\x85\x82a-=V[\x86UPa0\xB8V[`\x1F\x19\x84\x16a0f\x86a+\xB7V[`\0[\x82\x81\x10\x15a0\x8EW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa0iV[\x86\x83\x10\x15a0\xABW\x84\x89\x01Qa0\xA7`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0a0\xDBa0\xD6a0\xD1\x84a)\x17V[a(sV[a#qV[\x90P\x91\x90PV[a0\xEB\x81a0\xC0V[\x82RPPV[`\0\x81Ta0\xFE\x81a+NV[a1\x08\x81\x86a$\x9AV[\x94P`\x01\x82\x16`\0\x81\x14a1#W`\x01\x81\x14a19Wa1lV[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pa1lV[a1B\x85a+\xB7V[`\0[\x83\x81\x10\x15a1dW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa1EV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[`\0`@\x82\x01\x90Pa1\x8A`\0\x83\x01\x85a0\xE2V[\x81\x81\x03` \x83\x01Ra1\x9C\x81\x84a0\xF1V[\x90P\x93\x92PPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCannot remove self as admin.  Ha`\0\x82\x01R\x7Fve the new admin do it.\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a2\x12`7\x83a1\xA5V[\x91Pa2\x1D\x82a1\xB6V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra2A\x81a2\x05V[\x90P\x91\x90PV[`\0a2S\x82a$\x8FV[a2]\x81\x85a/>V[\x93Pa2m\x81\x85` \x86\x01a$\xABV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a2\x85\x82\x84a2HV[\x91P\x81\x90P\x92\x91PPV[`\0`@\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra2\xAA\x81\x85a$\xE6V[\x90Pa2\xB9` \x83\x01\x84a(\xEDV[\x93\x92PPPV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a3\x1C`/\x83a1\xA5V[\x91Pa3'\x82a2\xC0V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra3K\x81a3\x0FV[\x90P\x91\x90PV[`\0``\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra3l\x81\x86a$\xE6V[\x90Pa3{` \x83\x01\x85a(\xEDV[a3\x88`@\x83\x01\x84a(\xEDV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a3\xCA\x82a)\x17V[\x91Pa3\xD5\x83a)\x17V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\xF5Wa3\xF4a3\x90V[[\x92\x91PPV[`\0\x81T\x90Pa4\n\x81a+NV[\x90P\x91\x90PV[\x81\x81\x03a4\x1FWPPa4\xF7V[a4(\x82a3\xFBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4AWa4@a%KV[[a4K\x82Ta+NV[a4V\x82\x82\x85a,\xCCV[`\0`\x1F\x83\x11`\x01\x81\x14a4\x85W`\0\x84\x15a4sW\x82\x87\x01T\x90P[a4}\x85\x82a-=V[\x86UPa4\xF0V[`\x1F\x19\x84\x16a4\x93\x87a+\xB7V[\x96Pa4\x9E\x86a+\xB7V[`\0[\x82\x81\x10\x15a4\xC6W\x84\x89\x01T\x82U`\x01\x82\x01\x91P`\x01\x85\x01\x94P` \x81\x01\x90Pa4\xA1V[\x86\x83\x10\x15a4\xE3W\x84\x89\x01Ta4\xDF`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPP[V[`\0`\x80\x82\x01\x90Pa5\x0E`\0\x83\x01\x87a)+V[\x81\x81\x03` \x83\x01Ra5 \x81\x86a$\xE6V[\x90Pa5/`@\x83\x01\x85a(\xEDV[a5<``\x83\x01\x84a(\xEDV[\x95\x94PPPPPV[`\0`@\x82\x01\x90Pa5Z`\0\x83\x01\x85a(\xEDV[\x81\x81\x03` \x83\x01Ra5l\x81\x84a0\xF1V[\x90P\x93\x92PPPV[`\0\x81Q\x90Pa5\x84\x81a'gV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a5\xA0Wa5\x9Fa#gV[[`\0a5\xAE\x84\x82\x85\x01a5uV[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa5\xCC`\0\x83\x01\x85a'\xC0V[a5\xD9` \x83\x01\x84a*\\V[\x93\x92PPPV[`\0\x81Q\x90Pa5\xEF\x81a'\x04V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a6\x0BWa6\na#gV[[`\0a6\x19\x84\x82\x85\x01a5\xE0V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a6c`\x17\x83a6\"V[\x91Pa6n\x82a6-V[`\x17\x82\x01\x90P\x91\x90PV[`\0a6\x84\x82a/\xE3V[a6\x8E\x81\x85a6\"V[\x93Pa6\x9E\x81\x85` \x86\x01a$\xABV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a6\xE0`\x11\x83a6\"V[\x91Pa6\xEB\x82a6\xAAV[`\x11\x82\x01\x90P\x91\x90PV[`\0a7\x01\x82a6VV[\x91Pa7\r\x82\x85a6yV[\x91Pa7\x18\x82a6\xD3V[\x91Pa7$\x82\x84a6yV[\x91P\x81\x90P\x93\x92PPPV[`\0a7;\x82a/\xE3V[a7E\x81\x85a1\xA5V[\x93Pa7U\x81\x85` \x86\x01a$\xABV[a7^\x81a$\xD5V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra7\x83\x81\x84a70V[\x90P\x92\x91PPV[`\0a7\x96\x82a#qV[\x91Pa7\xA1\x83a#qV[\x92P\x82\x82\x02a7\xAF\x81a#qV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a7\xC6Wa7\xC5a3\x90V[[P\x92\x91PPV[`\0a7\xD8\x82a#qV[\x91Pa7\xE3\x83a#qV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a7\xFBWa7\xFAa3\x90V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a8;\x82a#qV[\x91P`\0\x82\x03a8NWa8Ma3\x90V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a8\x8F` \x83a1\xA5V[\x91Pa8\x9A\x82a8YV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra8\xBE\x81a8\x82V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 3p\xEE\x08-\x8D\x9Eh\xC3\n\x8D\x89_*ItUj\xED\xD9\xB4\x01\xA9WoL&\xD12\xA7\xC9hdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static DOMAINWALLETORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80coP\xCD\x97\x11a\0\xF9W\x80c\xA0[w_\x11a\0\x97W\x80c\xC9\xB2\x87I\x11a\0qW\x80c\xC9\xB2\x87I\x14a\x05\x89W\x80c\xD5Gt\x1F\x14a\x05\xB9W\x80c\xD9d\xA3u\x14a\x05\xD5W\x80c\xF4\x8D`\xCA\x14a\x06\x05Wa\x01\xC4V[\x80c\xA0[w_\x14a\x05\x0BW\x80c\xA2\x17\xFD\xDF\x14a\x05;W\x80c\xA2\xF4!\x04\x14a\x05YWa\x01\xC4V[\x80c\x88\x0B\xC4\xE6\x11a\0\xD3W\x80c\x88\x0B\xC4\xE6\x14a\x04qW\x80c\x91\xD1HT\x14a\x04\xA1W\x80c\x9A V\xF4\x14a\x04\xD1W\x80c\x9D\xCA\x002\x14a\x04\xEDWa\x01\xC4V[\x80coP\xCD\x97\x14a\x04\x07W\x80cpH\x02u\x14a\x047W\x80cu\xB28\xFC\x14a\x04SWa\x01\xC4V[\x80c'\xBD\x06\x9E\x11a\x01fW\x80c9\xC7c\x9C\x11a\x01@W\x80c9\xC7c\x9C\x14a\x03YW\x80cI\xFC\xBCT\x14a\x03\x89W\x80cP\xD1{^\x14a\x03\xB9W\x80cj0\0\xC4\x14a\x03\xD7Wa\x01\xC4V[\x80c'\xBD\x06\x9E\x14a\x03\x05W\x80c//\xF1]\x14a\x03!W\x80c6V\x8A\xBE\x14a\x03=Wa\x01\xC4V[\x80c\x06\x98_\xB1\x11a\x01\xA2W\x80c\x06\x98_\xB1\x14a\x02YW\x80c\x15\xD4dt\x14a\x02\x89W\x80c\x17\x85\xF5<\x14a\x02\xB9W\x80c$\x8A\x9C\xA3\x14a\x02\xD5Wa\x01\xC4V[\x80c\x01\xC6\xD05\x14a\x01\xC9W\x80c\x01\xFF\xC9\xA7\x14a\x01\xF9W\x80c\x03\xE9\xE6\t\x14a\x02)W[`\0\x80\xFD[a\x01\xE3`\x04\x806\x03\x81\x01\x90a\x01\xDE\x91\x90a#\xA7V[a\x065V[`@Qa\x01\xF0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02\x13`\x04\x806\x03\x81\x01\x90a\x02\x0E\x91\x90a$bV[a\x06bV[`@Qa\x02 \x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02C`\x04\x806\x03\x81\x01\x90a\x02>\x91\x90a#\xA7V[a\x06\xDCV[`@Qa\x02P\x91\x90a%\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x02s`\x04\x806\x03\x81\x01\x90a\x02n\x91\x90a&vV[a\x07\x84V[`@Qa\x02\x80\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xA3`\x04\x806\x03\x81\x01\x90a\x02\x9E\x91\x90a#\xA7V[a\t\x19V[`@Qa\x02\xB0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xD3`\x04\x806\x03\x81\x01\x90a\x02\xCE\x91\x90a'0V[a\x0C\xDAV[\0[a\x02\xEF`\x04\x806\x03\x81\x01\x90a\x02\xEA\x91\x90a'\x93V[a\r\xA0V[`@Qa\x02\xFC\x91\x90a'\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x03\x1F`\x04\x806\x03\x81\x01\x90a\x03\x1A\x91\x90a'\xEAV[a\r\xBFV[\0[a\x03;`\x04\x806\x03\x81\x01\x90a\x036\x91\x90a(3V[a\x0EEV[\0[a\x03W`\x04\x806\x03\x81\x01\x90a\x03R\x91\x90a(3V[a\x0EfV[\0[a\x03s`\x04\x806\x03\x81\x01\x90a\x03n\x91\x90a#\xA7V[a\x0E\xE9V[`@Qa\x03\x80\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x03\xA3`\x04\x806\x03\x81\x01\x90a\x03\x9E\x91\x90a#\xA7V[a\x0F\x16V[`@Qa\x03\xB0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x03\xC1a\x106V[`@Qa\x03\xCE\x91\x90a(\xD2V[`@Q\x80\x91\x03\x90\xF3[a\x03\xF1`\x04\x806\x03\x81\x01\x90a\x03\xEC\x91\x90a#\xA7V[a\x10\\V[`@Qa\x03\xFE\x91\x90a%\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04!`\x04\x806\x03\x81\x01\x90a\x04\x1C\x91\x90a'\xEAV[a\x11\x04V[`@Qa\x04.\x91\x90a(\xFCV[`@Q\x80\x91\x03\x90\xF3[a\x04Q`\x04\x806\x03\x81\x01\x90a\x04L\x91\x90a'0V[a\x11,V[\0[a\x04[a\x11\x84V[`@Qa\x04h\x91\x90a'\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x04\x8B`\x04\x806\x03\x81\x01\x90a\x04\x86\x91\x90a#\xA7V[a\x11\xA8V[`@Qa\x04\x98\x91\x90a):V[`@Q\x80\x91\x03\x90\xF3[a\x04\xBB`\x04\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a(3V[a\x11\xDCV[`@Qa\x04\xC8\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x04\xEB`\x04\x806\x03\x81\x01\x90a\x04\xE6\x91\x90a)UV[a\x12FV[\0[a\x04\xF5a\x17\x1BV[`@Qa\x05\x02\x91\x90a*kV[`@Q\x80\x91\x03\x90\xF3[a\x05%`\x04\x806\x03\x81\x01\x90a\x05 \x91\x90a#\xA7V[a\x17.V[`@Qa\x052\x91\x90a(\xFCV[`@Q\x80\x91\x03\x90\xF3[a\x05Ca\x17NV[`@Qa\x05P\x91\x90a'\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x05s`\x04\x806\x03\x81\x01\x90a\x05n\x91\x90a*\xB2V[a\x17UV[`@Qa\x05\x80\x91\x90a(\xFCV[`@Q\x80\x91\x03\x90\xF3[a\x05\xA3`\x04\x806\x03\x81\x01\x90a\x05\x9E\x91\x90a*\xDFV[a\x17\x9FV[`@Qa\x05\xB0\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x05\xD3`\x04\x806\x03\x81\x01\x90a\x05\xCE\x91\x90a(3V[a\x18\xDCV[\0[a\x05\xEF`\x04\x806\x03\x81\x01\x90a\x05\xEA\x91\x90a#\xA7V[a\x18\xFDV[`@Qa\x05\xFC\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[a\x06\x1F`\x04\x806\x03\x81\x01\x90a\x06\x1A\x91\x90a#\xA7V[a\x1CZV[`@Qa\x06,\x91\x90a#\xEFV[`@Q\x80\x91\x03\x90\xF3[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xD5WPa\x06\xD4\x82a\x1C\x87V[[\x90P\x91\x90PV[```\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x80Ta\x06\xFF\x90a+NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07+\x90a+NV[\x80\x15a\x07xW\x80`\x1F\x10a\x07MWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07xV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07[W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0a\x07\x8Ea\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x07\xF0WPa\x07\xEE\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x08;Wa\x07\xFDa\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x082\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\x08sW`\0\x90Pa\t\x13V[\x81`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\x08\x96\x91\x90a-YV[P`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01`\x01`\0`\x02`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\t\r\x91\x90a.VV[P`\x01\x90P[\x92\x91PPV[`\0a\t#a\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\t\x85WPa\t\x83\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\t\xD0Wa\t\x92a\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xC7\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\n\x08W`\0\x90Pa\x0C\xD5V[`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x80\x82\x01`\0a\n[\x91\x90a#\0V[`\x01\x82\x01`\0a\x01\0\n\x81T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01`\0\x90U`\x03\x82\x01`\0a\n\x8E\x91\x90a#\0V[`\x04\x82\x01`\0\x90U`\x05\x82\x01`\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x05\x82\x01`\x01a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x06\x82\x01`\0a\n\xCE\x91\x90a#\0V[PP`\x03`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01`@Qa\x0B\t\x91\x90a/\xCCV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x90U`\0`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x90\x81a\x0C\x1E\x91\x90a/\xEEV[P`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\x0Cl\x91\x90a/\xEEV[P\x7F\xD4\xA9^(6z<\xE6i@B\xB0\x1D4\xE7\xBA\x01\x98\xBC\xDA\xCA\x1F\xFB8\xEA\xEC\x98\x98\xC7H\xB0\xAD\x81`\x01`\0\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Qa\x0C\xC7\x92\x91\x90a1uV[`@Q\x80\x91\x03\x90\xA1`\x01\x91PP[\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\r\x04\x81a\x1E5V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\rrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\ri\x90a2(V[`@Q\x80\x91\x03\x90\xFD[a\r\x9C\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1EIV[PPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[`\0`\x03\x82`@Qa\r\xD1\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14a\x0EBW\x80`\x03\x82`@Qa\r\xF6\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`@Q\x7F;\xCB\xF6\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E9\x92\x91\x90a2\x90V[`@Q\x80\x91\x03\x90\xFD[PV[a\x0EN\x82a\r\xA0V[a\x0EW\x81a\x1E5V[a\x0Ea\x83\x83a\x1EIV[PPPV[a\x0Ena\x1F)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0E\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xD2\x90a32V[`@Q\x80\x91\x03\x90\xFD[a\x0E\xE5\x82\x82a\x1F1V[PPV[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x80`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01T\x90P`\0`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01\x80Ta\x0FU\x90a+NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x81\x90a+NV[\x80\x15a\x0F\xCEW\x80`\x1F\x10a\x0F\xA3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xCEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0B\x83\x10\x90P\x80\x15a\x10)W\x7F\xB26?h\x8F{2U|d\xBF\xF9\0\x81\x95\x9F\x0B\xB6\xC2O\t\xCAFT$\xC5\xAF\xC5\x85M\xC5\x8D\x82\x86\x85`@Qa\x10\x15\x93\x92\x91\x90a3RV[`@Q\x80\x91\x03\x90\xA1`\x01\x93PPPPa\x101V[`\0\x93PPPP[\x91\x90PV[`\x06`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[```\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01\x80Ta\x10\x7F\x90a+NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xAB\x90a+NV[\x80\x15a\x10\xF8W\x80`\x1F\x10a\x10\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xF8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0`\x03\x82`@Qa\x11\x16\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x90P\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x11V\x81a\x1E5V[a\x11\x80\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1EIV[PPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[a\x12Na\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x12\xB0WPa\x12\xAE\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x12\xFBWa\x12\xBDa\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xF2\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x84`@Qa\x13\r\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T\x14a\x13~W\x82`\x03\x84`@Qa\x132\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 T`@Q\x7F;\xCB\xF6\xC4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13u\x92\x91\x90a2\x90V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x06`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x13\xA1\x91\x90a3\xBFV[`\x06`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0`\x06`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x84`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x90\x81a\x14\x1B\x91\x90a-YV[P\x82`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP\x83`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01\x90\x81a\x14\x82\x91\x90a-YV[P\x81`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01\x81\x90UP`\x01\x80`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x80`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80`\x01`\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01\x81`\0\x01\x90\x81a\x15\xD5\x91\x90a4\x11V[P`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02\x82\x01T\x81`\x02\x01U`\x03\x82\x01\x81`\x03\x01\x90\x81a\x167\x91\x90a4\x11V[P`\x04\x82\x01T\x81`\x04\x01U`\x05\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x05\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x06\x82\x01\x81`\x06\x01\x90\x81a\x16\xB0\x91\x90a4\x11V[P\x90PP\x82`\x03\x85`@Qa\x16\xC5\x91\x90a2yV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 \x81\x90UP\x7F\xC2v\xB2\x07U\x8B\x88-n\x0EI\xFC>\"\x1E\x9F\xD1@\xCC\xEDy\xDD\x16\xFA\xE8\x8E\r\xE9\x7FJ/\r\x81\x86\x84\x86`@Qa\x17\x0C\x94\x93\x92\x91\x90a4\xF9V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x06`\x1C\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01T\x90P\x91\x90PV[`\0\x80\x1B\x81V[`\0`\x02`\0`\x01`\0\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01T\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01T\x90P\x91\x90PV[`\0a\x17\xA9a\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x18\x0BWPa\x18\t\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x18VWa\x18\x18a\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18M\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x01`\0\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\x18\xA2W`\0\x90Pa\x18\xD6V[\x81`\x01`\0\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP`\x01\x90P[\x92\x91PPV[a\x18\xE5\x82a\r\xA0V[a\x18\xEE\x81a\x1E5V[a\x18\xF8\x83\x83a\x1F1V[PPPV[`\0a\x19\x07a\x1C\xF1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80\x15a\x19iWPa\x19g\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x11\xDCV[\x15[\x15a\x19\xB4Wa\x19va\x1C\xF1V[3`@Q\x7F2t\x93\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xAB\x92\x91\x90a+\x8EV[`@Q\x80\x91\x03\x90\xFD[`\0\x15\x15`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x03a\x19\xECW`\0\x90Pa\x1CUV[`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01\x81\x90UP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x90\x81a\x1Ap\x91\x90a/\xEEV[P`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\0\x81RP`\x02`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\x06\x01\x90\x81a\x1A\xAA\x91\x90a/\xEEV[P`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01\x81`\0\x01\x90\x81a\x1B#\x91\x90a4\x11V[P`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02\x82\x01T\x81`\x02\x01U`\x03\x82\x01\x81`\x03\x01\x90\x81a\x1B\x85\x91\x90a4\x11V[P`\x04\x82\x01T\x81`\x04\x01U`\x05\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x05\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81`\x05\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x06\x82\x01\x81`\x06\x01\x90\x81a\x1B\xFE\x91\x90a4\x11V[P\x90PP\x7F\xAE\x17\x95\x13\xBE\xB9\xB1\x9A\"\xA2\xC8/yM\xFDM7\x9C\xBB\xD2\xE7\xE9\\tJ\x1E\xA8\x9A\xD2\x002\xF7\x82`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\x03\x01`@Qa\x1CH\x92\x91\x90a5EV[`@Q\x80\x91\x03\x90\xA1`\x01\x90P[\x91\x90PV[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x05\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[`\0`\x06`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x06`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x16\xE7:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xC2\x91\x90a5\x8AV[`\x06`\x1C\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xEF\x92\x91\x90a5\xB7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E0\x91\x90a5\xF5V[\x90P\x90V[a\x1EF\x81a\x1EAa\x1F)V[a \x12V[PV[a\x1ES\x82\x82a\x11\xDCV[a\x1F%W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1E\xCAa\x1F)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\x1F;\x82\x82a\x11\xDCV[\x15a \x0EW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1F\xB3a\x1F)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a \x1C\x82\x82a\x11\xDCV[a \x93Wa )\x81a \x97V[a 7\x83`\0\x1C` a \xC4V[`@Q` \x01a H\x92\x91\x90a6\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \x8A\x91\x90a7iV[`@Q\x80\x91\x03\x90\xFD[PPV[``a \xBD\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a \xC4V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a \xD7\x91\x90a7\x8BV[a \xE1\x91\x90a7\xCDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xFAWa \xF9a%KV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!,W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a!dWa!ca8\x01V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a!\xC8Wa!\xC7a8\x01V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\"\x08\x91\x90a7\x8BV[a\"\x12\x91\x90a7\xCDV[\x90P[`\x01\x81\x11\x15a\"\xB2W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\"TWa\"Sa8\x01V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\"kWa\"ja8\x01V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\"\xAB\x90a80V[\x90Pa\"\x15V[P`\0\x84\x14a\"\xF6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\xED\x90a8\xA5V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[P\x80Ta#\x0C\x90a+NV[`\0\x82U\x80`\x1F\x10a#\x1EWPa#=V[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a#<\x91\x90a#@V[[PV[[\x80\x82\x11\x15a#YW`\0\x81`\0\x90UP`\x01\x01a#AV[P\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a#\x84\x81a#qV[\x81\x14a#\x8FW`\0\x80\xFD[PV[`\0\x815\x90Pa#\xA1\x81a#{V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\xBDWa#\xBCa#gV[[`\0a#\xCB\x84\x82\x85\x01a#\x92V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a#\xE9\x81a#\xD4V[\x82RPPV[`\0` \x82\x01\x90Pa$\x04`\0\x83\x01\x84a#\xE0V[\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a$?\x81a$\nV[\x81\x14a$JW`\0\x80\xFD[PV[`\0\x815\x90Pa$\\\x81a$6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a$xWa$wa#gV[[`\0a$\x86\x84\x82\x85\x01a$MV[\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a$\xC9W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa$\xAEV[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a$\xF1\x82a$\x8FV[a$\xFB\x81\x85a$\x9AV[\x93Pa%\x0B\x81\x85` \x86\x01a$\xABV[a%\x14\x81a$\xD5V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra%9\x81\x84a$\xE6V[\x90P\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a%\x83\x82a$\xD5V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a%\xA2Wa%\xA1a%KV[[\x80`@RPPPV[`\0a%\xB5a#]V[\x90Pa%\xC1\x82\x82a%zV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xE1Wa%\xE0a%KV[[a%\xEA\x82a$\xD5V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a&\x19a&\x14\x84a%\xC6V[a%\xABV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a&5Wa&4a%FV[[a&@\x84\x82\x85a%\xF7V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a&]Wa&\\a%AV[[\x815a&m\x84\x82` \x86\x01a&\x06V[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a&\x8DWa&\x8Ca#gV[[`\0a&\x9B\x85\x82\x86\x01a#\x92V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xBCWa&\xBBa#lV[[a&\xC8\x85\x82\x86\x01a&HV[\x91PP\x92P\x92\x90PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a&\xFD\x82a&\xD2V[\x90P\x91\x90PV[a'\r\x81a&\xF2V[\x81\x14a'\x18W`\0\x80\xFD[PV[`\0\x815\x90Pa'*\x81a'\x04V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'FWa'Ea#gV[[`\0a'T\x84\x82\x85\x01a'\x1BV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a'p\x81a']V[\x81\x14a'{W`\0\x80\xFD[PV[`\0\x815\x90Pa'\x8D\x81a'gV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'\xA9Wa'\xA8a#gV[[`\0a'\xB7\x84\x82\x85\x01a'~V[\x91PP\x92\x91PPV[a'\xC9\x81a']V[\x82RPPV[`\0` \x82\x01\x90Pa'\xE4`\0\x83\x01\x84a'\xC0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\0Wa'\xFFa#gV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1EWa(\x1Da#lV[[a(*\x84\x82\x85\x01a&HV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a(JWa(Ia#gV[[`\0a(X\x85\x82\x86\x01a'~V[\x92PP` a(i\x85\x82\x86\x01a'\x1BV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a(\x98a(\x93a(\x8E\x84a&\xD2V[a(sV[a&\xD2V[\x90P\x91\x90PV[`\0a(\xAA\x82a(}V[\x90P\x91\x90PV[`\0a(\xBC\x82a(\x9FV[\x90P\x91\x90PV[a(\xCC\x81a(\xB1V[\x82RPPV[`\0` \x82\x01\x90Pa(\xE7`\0\x83\x01\x84a(\xC3V[\x92\x91PPV[a(\xF6\x81a#qV[\x82RPPV[`\0` \x82\x01\x90Pa)\x11`\0\x83\x01\x84a(\xEDV[\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a)4\x81a)\x17V[\x82RPPV[`\0` \x82\x01\x90Pa)O`\0\x83\x01\x84a)+V[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)oWa)na#gV[[`\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x8DWa)\x8Ca#lV[[a)\x99\x87\x82\x88\x01a&HV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xBAWa)\xB9a#lV[[a)\xC6\x87\x82\x88\x01a&HV[\x93PP`@a)\xD7\x87\x82\x88\x01a#\x92V[\x92PP``a)\xE8\x87\x82\x88\x01a#\x92V[\x91PP\x92\x95\x91\x94P\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a*4Wa*3a)\xF4V[[PV[`\0\x81\x90Pa*E\x82a*#V[\x91\x90PV[`\0a*U\x82a*7V[\x90P\x91\x90PV[a*e\x81a*JV[\x82RPPV[`\0` \x82\x01\x90Pa*\x80`\0\x83\x01\x84a*\\V[\x92\x91PPV[a*\x8F\x81a)\x17V[\x81\x14a*\x9AW`\0\x80\xFD[PV[`\0\x815\x90Pa*\xAC\x81a*\x86V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*\xC8Wa*\xC7a#gV[[`\0a*\xD6\x84\x82\x85\x01a*\x9DV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a*\xF6Wa*\xF5a#gV[[`\0a+\x04\x85\x82\x86\x01a*\x9DV[\x92PP` a+\x15\x85\x82\x86\x01a#\x92V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a+fW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+yWa+xa+\x1FV[[P\x91\x90PV[a+\x88\x81a&\xF2V[\x82RPPV[`\0`@\x82\x01\x90Pa+\xA3`\0\x83\x01\x85a+\x7FV[a+\xB0` \x83\x01\x84a+\x7FV[\x93\x92PPPV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a,\x19\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a+\xDCV[a,#\x86\x83a+\xDCV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0a,Va,Qa,L\x84a#qV[a(sV[a#qV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a,p\x83a,;V[a,\x84a,|\x82a,]V[\x84\x84Ta+\xE9V[\x82UPPPPV[`\0\x90V[a,\x99a,\x8CV[a,\xA4\x81\x84\x84a,gV[PPPV[[\x81\x81\x10\x15a,\xC8Wa,\xBD`\0\x82a,\x91V[`\x01\x81\x01\x90Pa,\xAAV[PPV[`\x1F\x82\x11\x15a-\rWa,\xDE\x81a+\xB7V[a,\xE7\x84a+\xCCV[\x81\x01` \x85\x10\x15a,\xF6W\x81\x90P[a-\na-\x02\x85a+\xCCV[\x83\x01\x82a,\xA9V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a-0`\0\x19\x84`\x08\x02a-\x12V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a-I\x83\x83a-\x1FV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a-b\x82a$\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-{Wa-za%KV[[a-\x85\x82Ta+NV[a-\x90\x82\x82\x85a,\xCCV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a-\xC3W`\0\x84\x15a-\xB1W\x82\x87\x01Q\x90P[a-\xBB\x85\x82a-=V[\x86UPa.#V[`\x1F\x19\x84\x16a-\xD1\x86a+\xB7V[`\0[\x82\x81\x10\x15a-\xF9W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa-\xD4V[\x86\x83\x10\x15a.\x16W\x84\x89\x01Qa.\x12`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x81T\x90Pa.:\x81a+NV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[\x81\x81\x03a.dWPPa/<V[a.m\x82a.+V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\x86Wa.\x85a%KV[[a.\x90\x82Ta+NV[a.\x9B\x82\x82\x85a,\xCCV[`\0`\x1F\x83\x11`\x01\x81\x14a.\xCAW`\0\x84\x15a.\xB8W\x82\x87\x01T\x90P[a.\xC2\x85\x82a-=V[\x86UPa/5V[`\x1F\x19\x84\x16a.\xD8\x87a.AV[\x96Pa.\xE3\x86a+\xB7V[`\0[\x82\x81\x10\x15a/\x0BW\x84\x89\x01T\x82U`\x01\x82\x01\x91P`\x01\x85\x01\x94P` \x81\x01\x90Pa.\xE6V[\x86\x83\x10\x15a/(W\x84\x89\x01Ta/$`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPP[V[`\0\x81\x90P\x92\x91PPV[`\0\x81Ta/V\x81a+NV[a/`\x81\x86a/>V[\x94P`\x01\x82\x16`\0\x81\x14a/{W`\x01\x81\x14a/\x90Wa/\xC3V[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa/\xC3V[a/\x99\x85a+\xB7V[`\0[\x83\x81\x10\x15a/\xBBW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa/\x9CV[\x83\x88\x01\x95PPP[PPP\x92\x91PPV[`\0a/\xD8\x82\x84a/IV[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[a/\xF7\x82a/\xE3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\x10Wa0\x0Fa%KV[[a0\x1A\x82Ta+NV[a0%\x82\x82\x85a,\xCCV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a0XW`\0\x84\x15a0FW\x82\x87\x01Q\x90P[a0P\x85\x82a-=V[\x86UPa0\xB8V[`\x1F\x19\x84\x16a0f\x86a+\xB7V[`\0[\x82\x81\x10\x15a0\x8EW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa0iV[\x86\x83\x10\x15a0\xABW\x84\x89\x01Qa0\xA7`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0a0\xDBa0\xD6a0\xD1\x84a)\x17V[a(sV[a#qV[\x90P\x91\x90PV[a0\xEB\x81a0\xC0V[\x82RPPV[`\0\x81Ta0\xFE\x81a+NV[a1\x08\x81\x86a$\x9AV[\x94P`\x01\x82\x16`\0\x81\x14a1#W`\x01\x81\x14a19Wa1lV[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pa1lV[a1B\x85a+\xB7V[`\0[\x83\x81\x10\x15a1dW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa1EV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[`\0`@\x82\x01\x90Pa1\x8A`\0\x83\x01\x85a0\xE2V[\x81\x81\x03` \x83\x01Ra1\x9C\x81\x84a0\xF1V[\x90P\x93\x92PPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCannot remove self as admin.  Ha`\0\x82\x01R\x7Fve the new admin do it.\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a2\x12`7\x83a1\xA5V[\x91Pa2\x1D\x82a1\xB6V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra2A\x81a2\x05V[\x90P\x91\x90PV[`\0a2S\x82a$\x8FV[a2]\x81\x85a/>V[\x93Pa2m\x81\x85` \x86\x01a$\xABV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a2\x85\x82\x84a2HV[\x91P\x81\x90P\x92\x91PPV[`\0`@\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra2\xAA\x81\x85a$\xE6V[\x90Pa2\xB9` \x83\x01\x84a(\xEDV[\x93\x92PPPV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a3\x1C`/\x83a1\xA5V[\x91Pa3'\x82a2\xC0V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra3K\x81a3\x0FV[\x90P\x91\x90PV[`\0``\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra3l\x81\x86a$\xE6V[\x90Pa3{` \x83\x01\x85a(\xEDV[a3\x88`@\x83\x01\x84a(\xEDV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a3\xCA\x82a)\x17V[\x91Pa3\xD5\x83a)\x17V[\x92P\x82\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\xF5Wa3\xF4a3\x90V[[\x92\x91PPV[`\0\x81T\x90Pa4\n\x81a+NV[\x90P\x91\x90PV[\x81\x81\x03a4\x1FWPPa4\xF7V[a4(\x82a3\xFBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4AWa4@a%KV[[a4K\x82Ta+NV[a4V\x82\x82\x85a,\xCCV[`\0`\x1F\x83\x11`\x01\x81\x14a4\x85W`\0\x84\x15a4sW\x82\x87\x01T\x90P[a4}\x85\x82a-=V[\x86UPa4\xF0V[`\x1F\x19\x84\x16a4\x93\x87a+\xB7V[\x96Pa4\x9E\x86a+\xB7V[`\0[\x82\x81\x10\x15a4\xC6W\x84\x89\x01T\x82U`\x01\x82\x01\x91P`\x01\x85\x01\x94P` \x81\x01\x90Pa4\xA1V[\x86\x83\x10\x15a4\xE3W\x84\x89\x01Ta4\xDF`\x1F\x89\x16\x82a-\x1FV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPP[V[`\0`\x80\x82\x01\x90Pa5\x0E`\0\x83\x01\x87a)+V[\x81\x81\x03` \x83\x01Ra5 \x81\x86a$\xE6V[\x90Pa5/`@\x83\x01\x85a(\xEDV[a5<``\x83\x01\x84a(\xEDV[\x95\x94PPPPPV[`\0`@\x82\x01\x90Pa5Z`\0\x83\x01\x85a(\xEDV[\x81\x81\x03` \x83\x01Ra5l\x81\x84a0\xF1V[\x90P\x93\x92PPPV[`\0\x81Q\x90Pa5\x84\x81a'gV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a5\xA0Wa5\x9Fa#gV[[`\0a5\xAE\x84\x82\x85\x01a5uV[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa5\xCC`\0\x83\x01\x85a'\xC0V[a5\xD9` \x83\x01\x84a*\\V[\x93\x92PPPV[`\0\x81Q\x90Pa5\xEF\x81a'\x04V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a6\x0BWa6\na#gV[[`\0a6\x19\x84\x82\x85\x01a5\xE0V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a6c`\x17\x83a6\"V[\x91Pa6n\x82a6-V[`\x17\x82\x01\x90P\x91\x90PV[`\0a6\x84\x82a/\xE3V[a6\x8E\x81\x85a6\"V[\x93Pa6\x9E\x81\x85` \x86\x01a$\xABV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a6\xE0`\x11\x83a6\"V[\x91Pa6\xEB\x82a6\xAAV[`\x11\x82\x01\x90P\x91\x90PV[`\0a7\x01\x82a6VV[\x91Pa7\r\x82\x85a6yV[\x91Pa7\x18\x82a6\xD3V[\x91Pa7$\x82\x84a6yV[\x91P\x81\x90P\x93\x92PPPV[`\0a7;\x82a/\xE3V[a7E\x81\x85a1\xA5V[\x93Pa7U\x81\x85` \x86\x01a$\xABV[a7^\x81a$\xD5V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra7\x83\x81\x84a70V[\x90P\x92\x91PPV[`\0a7\x96\x82a#qV[\x91Pa7\xA1\x83a#qV[\x92P\x82\x82\x02a7\xAF\x81a#qV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a7\xC6Wa7\xC5a3\x90V[[P\x92\x91PPV[`\0a7\xD8\x82a#qV[\x91Pa7\xE3\x83a#qV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a7\xFBWa7\xFAa3\x90V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a8;\x82a#qV[\x91P`\0\x82\x03a8NWa8Ma3\x90V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a8\x8F` \x83a1\xA5V[\x91Pa8\x9A\x82a8YV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra8\xBE\x81a8\x82V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 3p\xEE\x08-\x8D\x9Eh\xC3\n\x8D\x89_*ItUj\xED\xD9\xB4\x01\xA9WoL&\xD12\xA7\xC9hdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static DOMAINWALLETORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DomainWalletOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DomainWalletOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DomainWalletOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DomainWalletOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DomainWalletOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DomainWalletOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DomainWalletOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DOMAINWALLETORACLE_ABI.clone(),
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
                DOMAINWALLETORACLE_ABI.clone(),
                DOMAINWALLETORACLE_BYTECODE.clone().into(),
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
        ///Calls the contract's `addAdmin` (0x70480275) function
        pub fn add_admin(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 72, 2, 117], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkRegistration` (0x27bd069e) function
        pub fn check_registration(
            &self,
            uri: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 189, 6, 158], uri)
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
        ///Calls the contract's `getDomainIdByTokenId` (0x880bc4e6) function
        pub fn get_domain_id_by_token_id(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([136, 11, 196, 230], pkp_token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDomainTokenIdByUri` (0x6f50cd97) function
        pub fn get_domain_token_id_by_uri(
            &self,
            uri: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([111, 80, 205, 151], uri)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDomainUri` (0x6a3000c4) function
        pub fn get_domain_uri(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([106, 48, 0, 196], pkp_token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getExpiration` (0xa05b775f) function
        pub fn get_expiration(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 91, 119, 95], pkp_token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPkpTokenId` (0xa2f42104) function
        pub fn get_pkp_token_id(
            &self,
            id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([162, 244, 33, 4], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRecord` (0x03e9e609) function
        pub fn get_record(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([3, 233, 230, 9], pkp_token_id)
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
        ///Calls the contract's `hasExpired` (0x49fcbc54) function
        pub fn has_expired(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 252, 188, 84], pkp_token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasOwner` (0xf48d60ca) function
        pub fn has_owner(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([244, 141, 96, 202], pkp_token_id)
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
        ///Calls the contract's `isOwner` (0x39c7639c) function
        pub fn is_owner(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 199, 99, 156], pkp_token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isRouted` (0x01c6d035) function
        pub fn is_routed(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 198, 208, 53], pkp_token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerDomain` (0x9a2056f4) function
        pub fn register_domain(
            &self,
            user_id: ::ethers::core::types::Bytes,
            uri: ::ethers::core::types::Bytes,
            pkp_token_id: ::ethers::core::types::U256,
            ttl: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 32, 86, 244], (user_id, uri, pkp_token_id, ttl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerPKP` (0xc9b28749) function
        pub fn register_pkp(
            &self,
            id: u64,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([201, 178, 135, 73], (id, pkp_token_id))
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
        ///Calls the contract's `removeDomain` (0x15d46474) function
        pub fn remove_domain(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 212, 100, 116], pkp_token_id)
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
        ///Calls the contract's `revokeDomain` (0xd964a375) function
        pub fn revoke_domain(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 100, 163, 117], pkp_token_id)
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDomainRecord` (0x06985fb1) function
        pub fn update_domain_record(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
            record: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([6, 152, 95, 177], (pkp_token_id, record))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Expired` event
        pub fn expired_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ExpiredFilter> {
            self.0.event()
        }
        ///Gets the contract's `Registered` event
        pub fn registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Removed` event
        pub fn removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemovedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Revoked` event
        pub fn revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RevokedFilter> {
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DomainWalletOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DomainWalletOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DomainAlreadyRegistered` with signature `DomainAlreadyRegistered(bytes,uint256)` and selector `0x3bcbf6c4`
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
    #[etherror(
        name = "DomainAlreadyRegistered",
        abi = "DomainAlreadyRegistered(bytes,uint256)"
    )]
    pub struct DomainAlreadyRegistered {
        pub uri: ::ethers::core::types::Bytes,
        pub pkp_token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `NonRegistryCaller` with signature `NonRegistryCaller(address,address)` and selector `0x3274939c`
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
    #[etherror(name = "NonRegistryCaller", abi = "NonRegistryCaller(address,address)")]
    pub struct NonRegistryCaller {
        pub registry_address: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
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
    pub enum DomainWalletOracleErrors {
        DomainAlreadyRegistered(DomainAlreadyRegistered),
        NonRegistryCaller(NonRegistryCaller),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DomainWalletOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <DomainAlreadyRegistered as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DomainAlreadyRegistered(decoded));
            }
            if let Ok(decoded)
                = <NonRegistryCaller as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonRegistryCaller(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DomainWalletOracleErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DomainAlreadyRegistered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonRegistryCaller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DomainWalletOracleErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DomainAlreadyRegistered as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NonRegistryCaller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DomainWalletOracleErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainAlreadyRegistered(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NonRegistryCaller(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DomainWalletOracleErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DomainAlreadyRegistered> for DomainWalletOracleErrors {
        fn from(value: DomainAlreadyRegistered) -> Self {
            Self::DomainAlreadyRegistered(value)
        }
    }
    impl ::core::convert::From<NonRegistryCaller> for DomainWalletOracleErrors {
        fn from(value: NonRegistryCaller) -> Self {
            Self::NonRegistryCaller(value)
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
    #[ethevent(name = "Expired", abi = "Expired(bytes,uint256,uint256)")]
    pub struct ExpiredFilter {
        pub sub_domain: ::ethers::core::types::Bytes,
        pub token_id: ::ethers::core::types::U256,
        pub ttl: ::ethers::core::types::U256,
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
    #[ethevent(name = "Registered", abi = "Registered(uint64,bytes,uint256,uint256)")]
    pub struct RegisteredFilter {
        pub id: u64,
        pub sub_domain: ::ethers::core::types::Bytes,
        pub ttl: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Removed", abi = "Removed(uint256,bytes)")]
    pub struct RemovedFilter {
        pub token_id: ::ethers::core::types::U256,
        pub sub_domain: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "Revoked", abi = "Revoked(uint256,bytes)")]
    pub struct RevokedFilter {
        pub token_id: ::ethers::core::types::U256,
        pub sub_domain: ::ethers::core::types::Bytes,
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
    pub enum DomainWalletOracleEvents {
        ExpiredFilter(ExpiredFilter),
        RegisteredFilter(RegisteredFilter),
        RemovedFilter(RemovedFilter),
        RevokedFilter(RevokedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DomainWalletOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ExpiredFilter::decode_log(log) {
                return Ok(DomainWalletOracleEvents::ExpiredFilter(decoded));
            }
            if let Ok(decoded) = RegisteredFilter::decode_log(log) {
                return Ok(DomainWalletOracleEvents::RegisteredFilter(decoded));
            }
            if let Ok(decoded) = RemovedFilter::decode_log(log) {
                return Ok(DomainWalletOracleEvents::RemovedFilter(decoded));
            }
            if let Ok(decoded) = RevokedFilter::decode_log(log) {
                return Ok(DomainWalletOracleEvents::RevokedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(DomainWalletOracleEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(DomainWalletOracleEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(DomainWalletOracleEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DomainWalletOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExpiredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExpiredFilter> for DomainWalletOracleEvents {
        fn from(value: ExpiredFilter) -> Self {
            Self::ExpiredFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredFilter> for DomainWalletOracleEvents {
        fn from(value: RegisteredFilter) -> Self {
            Self::RegisteredFilter(value)
        }
    }
    impl ::core::convert::From<RemovedFilter> for DomainWalletOracleEvents {
        fn from(value: RemovedFilter) -> Self {
            Self::RemovedFilter(value)
        }
    }
    impl ::core::convert::From<RevokedFilter> for DomainWalletOracleEvents {
        fn from(value: RevokedFilter) -> Self {
            Self::RevokedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for DomainWalletOracleEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for DomainWalletOracleEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for DomainWalletOracleEvents {
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
    ///Container type for all input parameters for the `checkRegistration` function with signature `checkRegistration(bytes)` and selector `0x27bd069e`
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
    #[ethcall(name = "checkRegistration", abi = "checkRegistration(bytes)")]
    pub struct CheckRegistrationCall {
        pub uri: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `getDomainIdByTokenId` function with signature `getDomainIdByTokenId(uint256)` and selector `0x880bc4e6`
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
    #[ethcall(name = "getDomainIdByTokenId", abi = "getDomainIdByTokenId(uint256)")]
    pub struct GetDomainIdByTokenIdCall {
        pub pkp_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getDomainTokenIdByUri` function with signature `getDomainTokenIdByUri(bytes)` and selector `0x6f50cd97`
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
    #[ethcall(name = "getDomainTokenIdByUri", abi = "getDomainTokenIdByUri(bytes)")]
    pub struct GetDomainTokenIdByUriCall {
        pub uri: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getDomainUri` function with signature `getDomainUri(uint256)` and selector `0x6a3000c4`
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
    #[ethcall(name = "getDomainUri", abi = "getDomainUri(uint256)")]
    pub struct GetDomainUriCall {
        pub pkp_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getExpiration` function with signature `getExpiration(uint256)` and selector `0xa05b775f`
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
    #[ethcall(name = "getExpiration", abi = "getExpiration(uint256)")]
    pub struct GetExpirationCall {
        pub pkp_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPkpTokenId` function with signature `getPkpTokenId(uint64)` and selector `0xa2f42104`
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
    #[ethcall(name = "getPkpTokenId", abi = "getPkpTokenId(uint64)")]
    pub struct GetPkpTokenIdCall {
        pub id: u64,
    }
    ///Container type for all input parameters for the `getRecord` function with signature `getRecord(uint256)` and selector `0x03e9e609`
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
    #[ethcall(name = "getRecord", abi = "getRecord(uint256)")]
    pub struct GetRecordCall {
        pub pkp_token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `hasExpired` function with signature `hasExpired(uint256)` and selector `0x49fcbc54`
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
    #[ethcall(name = "hasExpired", abi = "hasExpired(uint256)")]
    pub struct HasExpiredCall {
        pub pkp_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `hasOwner` function with signature `hasOwner(uint256)` and selector `0xf48d60ca`
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
    #[ethcall(name = "hasOwner", abi = "hasOwner(uint256)")]
    pub struct HasOwnerCall {
        pub pkp_token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `isOwner` function with signature `isOwner(uint256)` and selector `0x39c7639c`
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
    #[ethcall(name = "isOwner", abi = "isOwner(uint256)")]
    pub struct IsOwnerCall {
        pub pkp_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isRouted` function with signature `isRouted(uint256)` and selector `0x01c6d035`
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
    #[ethcall(name = "isRouted", abi = "isRouted(uint256)")]
    pub struct IsRoutedCall {
        pub pkp_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerDomain` function with signature `registerDomain(bytes,bytes,uint256,uint256)` and selector `0x9a2056f4`
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
        name = "registerDomain",
        abi = "registerDomain(bytes,bytes,uint256,uint256)"
    )]
    pub struct RegisterDomainCall {
        pub user_id: ::ethers::core::types::Bytes,
        pub uri: ::ethers::core::types::Bytes,
        pub pkp_token_id: ::ethers::core::types::U256,
        pub ttl: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerPKP` function with signature `registerPKP(uint64,uint256)` and selector `0xc9b28749`
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
    #[ethcall(name = "registerPKP", abi = "registerPKP(uint64,uint256)")]
    pub struct RegisterPKPCall {
        pub id: u64,
        pub pkp_token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `removeDomain` function with signature `removeDomain(uint256)` and selector `0x15d46474`
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
    #[ethcall(name = "removeDomain", abi = "removeDomain(uint256)")]
    pub struct RemoveDomainCall {
        pub pkp_token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `revokeDomain` function with signature `revokeDomain(uint256)` and selector `0xd964a375`
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
    #[ethcall(name = "revokeDomain", abi = "revokeDomain(uint256)")]
    pub struct RevokeDomainCall {
        pub pkp_token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `updateDomainRecord` function with signature `updateDomainRecord(uint256,bytes)` and selector `0x06985fb1`
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
    #[ethcall(name = "updateDomainRecord", abi = "updateDomainRecord(uint256,bytes)")]
    pub struct UpdateDomainRecordCall {
        pub pkp_token_id: ::ethers::core::types::U256,
        pub record: ::ethers::core::types::Bytes,
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
    pub enum DomainWalletOracleCalls {
        AdminRole(AdminRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        AddAdmin(AddAdminCall),
        CheckRegistration(CheckRegistrationCall),
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        GetDomainIdByTokenId(GetDomainIdByTokenIdCall),
        GetDomainTokenIdByUri(GetDomainTokenIdByUriCall),
        GetDomainUri(GetDomainUriCall),
        GetExpiration(GetExpirationCall),
        GetPkpTokenId(GetPkpTokenIdCall),
        GetRecord(GetRecordCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasExpired(HasExpiredCall),
        HasOwner(HasOwnerCall),
        HasRole(HasRoleCall),
        IsOwner(IsOwnerCall),
        IsRouted(IsRoutedCall),
        RegisterDomain(RegisterDomainCall),
        RegisterPKP(RegisterPKPCall),
        RemoveAdmin(RemoveAdminCall),
        RemoveDomain(RemoveDomainCall),
        RenounceRole(RenounceRoleCall),
        RevokeDomain(RevokeDomainCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdateDomainRecord(UpdateDomainRecordCall),
    }
    impl ::ethers::core::abi::AbiDecode for DomainWalletOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <AddAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddAdmin(decoded));
            }
            if let Ok(decoded)
                = <CheckRegistrationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckRegistration(decoded));
            }
            if let Ok(decoded)
                = <ContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ContractResolver(decoded));
            }
            if let Ok(decoded)
                = <EnvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Env(decoded));
            }
            if let Ok(decoded)
                = <GetDomainIdByTokenIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetDomainIdByTokenId(decoded));
            }
            if let Ok(decoded)
                = <GetDomainTokenIdByUriCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetDomainTokenIdByUri(decoded));
            }
            if let Ok(decoded)
                = <GetDomainUriCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDomainUri(decoded));
            }
            if let Ok(decoded)
                = <GetExpirationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetExpiration(decoded));
            }
            if let Ok(decoded)
                = <GetPkpTokenIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPkpTokenId(decoded));
            }
            if let Ok(decoded)
                = <GetRecordCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRecord(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasExpiredCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasExpired(decoded));
            }
            if let Ok(decoded)
                = <HasOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasOwner(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded)
                = <IsRoutedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsRouted(decoded));
            }
            if let Ok(decoded)
                = <RegisterDomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RegisterDomain(decoded));
            }
            if let Ok(decoded)
                = <RegisterPKPCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RegisterPKP(decoded));
            }
            if let Ok(decoded)
                = <RemoveAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveAdmin(decoded));
            }
            if let Ok(decoded)
                = <RemoveDomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveDomain(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeDomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeDomain(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <UpdateDomainRecordCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateDomainRecord(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DomainWalletOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckRegistration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Env(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDomainIdByTokenId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDomainTokenIdByUri(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDomainUri(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetExpiration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPkpTokenId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRecord(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsRouted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterPKP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDomainRecord(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DomainWalletOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckRegistration(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Env(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDomainIdByTokenId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDomainTokenIdByUri(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDomainUri(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetExpiration(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPkpTokenId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRecord(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsRouted(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDomainRecord(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for DomainWalletOracleCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for DomainWalletOracleCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AddAdminCall> for DomainWalletOracleCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<CheckRegistrationCall> for DomainWalletOracleCalls {
        fn from(value: CheckRegistrationCall) -> Self {
            Self::CheckRegistration(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for DomainWalletOracleCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for DomainWalletOracleCalls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<GetDomainIdByTokenIdCall> for DomainWalletOracleCalls {
        fn from(value: GetDomainIdByTokenIdCall) -> Self {
            Self::GetDomainIdByTokenId(value)
        }
    }
    impl ::core::convert::From<GetDomainTokenIdByUriCall> for DomainWalletOracleCalls {
        fn from(value: GetDomainTokenIdByUriCall) -> Self {
            Self::GetDomainTokenIdByUri(value)
        }
    }
    impl ::core::convert::From<GetDomainUriCall> for DomainWalletOracleCalls {
        fn from(value: GetDomainUriCall) -> Self {
            Self::GetDomainUri(value)
        }
    }
    impl ::core::convert::From<GetExpirationCall> for DomainWalletOracleCalls {
        fn from(value: GetExpirationCall) -> Self {
            Self::GetExpiration(value)
        }
    }
    impl ::core::convert::From<GetPkpTokenIdCall> for DomainWalletOracleCalls {
        fn from(value: GetPkpTokenIdCall) -> Self {
            Self::GetPkpTokenId(value)
        }
    }
    impl ::core::convert::From<GetRecordCall> for DomainWalletOracleCalls {
        fn from(value: GetRecordCall) -> Self {
            Self::GetRecord(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for DomainWalletOracleCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for DomainWalletOracleCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasExpiredCall> for DomainWalletOracleCalls {
        fn from(value: HasExpiredCall) -> Self {
            Self::HasExpired(value)
        }
    }
    impl ::core::convert::From<HasOwnerCall> for DomainWalletOracleCalls {
        fn from(value: HasOwnerCall) -> Self {
            Self::HasOwner(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for DomainWalletOracleCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for DomainWalletOracleCalls {
        fn from(value: IsOwnerCall) -> Self {
            Self::IsOwner(value)
        }
    }
    impl ::core::convert::From<IsRoutedCall> for DomainWalletOracleCalls {
        fn from(value: IsRoutedCall) -> Self {
            Self::IsRouted(value)
        }
    }
    impl ::core::convert::From<RegisterDomainCall> for DomainWalletOracleCalls {
        fn from(value: RegisterDomainCall) -> Self {
            Self::RegisterDomain(value)
        }
    }
    impl ::core::convert::From<RegisterPKPCall> for DomainWalletOracleCalls {
        fn from(value: RegisterPKPCall) -> Self {
            Self::RegisterPKP(value)
        }
    }
    impl ::core::convert::From<RemoveAdminCall> for DomainWalletOracleCalls {
        fn from(value: RemoveAdminCall) -> Self {
            Self::RemoveAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveDomainCall> for DomainWalletOracleCalls {
        fn from(value: RemoveDomainCall) -> Self {
            Self::RemoveDomain(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for DomainWalletOracleCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeDomainCall> for DomainWalletOracleCalls {
        fn from(value: RevokeDomainCall) -> Self {
            Self::RevokeDomain(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for DomainWalletOracleCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for DomainWalletOracleCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpdateDomainRecordCall> for DomainWalletOracleCalls {
        fn from(value: UpdateDomainRecordCall) -> Self {
            Self::UpdateDomainRecord(value)
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
    ///Container type for all return fields from the `getDomainIdByTokenId` function with signature `getDomainIdByTokenId(uint256)` and selector `0x880bc4e6`
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
    pub struct GetDomainIdByTokenIdReturn(pub u64);
    ///Container type for all return fields from the `getDomainTokenIdByUri` function with signature `getDomainTokenIdByUri(bytes)` and selector `0x6f50cd97`
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
    pub struct GetDomainTokenIdByUriReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getDomainUri` function with signature `getDomainUri(uint256)` and selector `0x6a3000c4`
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
    pub struct GetDomainUriReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getExpiration` function with signature `getExpiration(uint256)` and selector `0xa05b775f`
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
    pub struct GetExpirationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPkpTokenId` function with signature `getPkpTokenId(uint64)` and selector `0xa2f42104`
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
    pub struct GetPkpTokenIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRecord` function with signature `getRecord(uint256)` and selector `0x03e9e609`
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
    pub struct GetRecordReturn(pub ::ethers::core::types::Bytes);
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
    ///Container type for all return fields from the `hasExpired` function with signature `hasExpired(uint256)` and selector `0x49fcbc54`
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
    pub struct HasExpiredReturn(pub bool);
    ///Container type for all return fields from the `hasOwner` function with signature `hasOwner(uint256)` and selector `0xf48d60ca`
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
    pub struct HasOwnerReturn(pub bool);
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
    ///Container type for all return fields from the `isOwner` function with signature `isOwner(uint256)` and selector `0x39c7639c`
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
    pub struct IsOwnerReturn(pub bool);
    ///Container type for all return fields from the `isRouted` function with signature `isRouted(uint256)` and selector `0x01c6d035`
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
    pub struct IsRoutedReturn(pub bool);
    ///Container type for all return fields from the `registerPKP` function with signature `registerPKP(uint64,uint256)` and selector `0xc9b28749`
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
    pub struct RegisterPKPReturn(pub bool);
    ///Container type for all return fields from the `removeDomain` function with signature `removeDomain(uint256)` and selector `0x15d46474`
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
    pub struct RemoveDomainReturn(pub bool);
    ///Container type for all return fields from the `revokeDomain` function with signature `revokeDomain(uint256)` and selector `0xd964a375`
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
    pub struct RevokeDomainReturn(pub bool);
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
    ///Container type for all return fields from the `updateDomainRecord` function with signature `updateDomainRecord(uint256,bytes)` and selector `0x06985fb1`
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
    pub struct UpdateDomainRecordReturn(pub bool);
}
