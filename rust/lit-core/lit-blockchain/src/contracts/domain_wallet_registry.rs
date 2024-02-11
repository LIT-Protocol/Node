pub use domain_wallet_registry::*;
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
pub mod domain_wallet_registry {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/DomainWalletRegistry.json",
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
                                    name: ::std::borrow::ToOwned::to_owned("ttl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("nftMetadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerDomainAndMintNext"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerDomainAndMintNext",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("ttl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodTypes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodIds",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodPubkeys",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodScopes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[][]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nftMetadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("setPKPMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPKPMetadata"),
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
                                    name: ::std::borrow::ToOwned::to_owned("nftMetadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "InvalidNftMetadataCollectionLength",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidNftMetadataCollectionLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("metadataCount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validMetadataCount",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("NonAdminCaller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NonAdminCaller"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adminAddress"),
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
    pub static DOMAINWALLETREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0@\xFB8\x03\x80b\0@\xFB\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x03\xEBV[3`\x02`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x02\x81\x11\x15b\0\0\xE1Wb\0\0\xE0b\0\x042V[[\x02\x17\x90UPb\0\x01:\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\x01t` \x1B` \x1CV[b\0\x01l\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x80b\0\x02e` \x1B` \x1CV[PPb\0\x04aV[b\0\x01\x86\x82\x82b\0\x02\xC8` \x1B` \x1CV[b\0\x02aW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPb\0\x02\x06b\0\x032` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\0b\0\x02x\x83b\0\x03:` \x1B` \x1CV[\x90P\x81`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01\x81\x90UP\x81\x81\x84\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x003\x90P\x90V[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x03\x8B\x82b\0\x03^V[\x90P\x91\x90PV[b\0\x03\x9D\x81b\0\x03~V[\x81\x14b\0\x03\xA9W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\xBD\x81b\0\x03\x92V[\x92\x91PPV[`\x03\x81\x10b\0\x03\xD1W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x03\xE5\x81b\0\x03\xC3V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x04\x05Wb\0\x04\x04b\0\x03YV[[`\0b\0\x04\x15\x85\x82\x86\x01b\0\x03\xACV[\x92PP` b\0\x04(\x85\x82\x86\x01b\0\x03\xD4V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[a<\x8A\x80b\0\x04q`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x9CW`\x005`\xE0\x1C\x80coP\xCD\x97\x11a\0\xECW\x80c\xA2\x17\xFD\xDF\x11a\0\x8AW\x80c\xC9\xB2\x87I\x11a\0dW\x80c\xC9\xB2\x87I\x14a\x06cW\x80c\xD5Gt\x1F\x14a\x06\xA0W\x80c\xD9d\xA3u\x14a\x06\xC9W\x80c\xF4\x8D`\xCA\x14a\x07\x06Wa\x01\x9CV[\x80c\xA2\x17\xFD\xDF\x14a\x05\xD2W\x80c\xA2\xF4!\x04\x14a\x05\xFDW\x80c\xB6\xE7\xF4\xD2\x14a\x06:Wa\x01\x9CV[\x80c\x88\x0B\xC4\xE6\x11a\0\xC6W\x80c\x88\x0B\xC4\xE6\x14a\x04\xF0W\x80c\x91\xD1HT\x14a\x05-W\x80c\x9D\xCA\x002\x14a\x05jW\x80c\xA0[w_\x14a\x05\x95Wa\x01\x9CV[\x80coP\xCD\x97\x14a\x04_W\x80cpH\x02u\x14a\x04\x9CW\x80cu\xB28\xFC\x14a\x04\xC5Wa\x01\x9CV[\x80c6V\x8A\xBE\x11a\x01YW\x80cP\xD1{^\x11a\x013W\x80cP\xD1{^\x14a\x03\x8AW\x80cU0\xAD\xF8\x14a\x03\xB5W\x80c[\x04W$\x14a\x03\xE5W\x80cj0\0\xC4\x14a\x04\"Wa\x01\x9CV[\x80c6V\x8A\xBE\x14a\x02\xE7W\x80c9\xC7c\x9C\x14a\x03\x10W\x80cI\xFC\xBCT\x14a\x03MWa\x01\x9CV[\x80c\x01\xC6\xD05\x14a\x01\xA1W\x80c\x01\xFF\xC9\xA7\x14a\x01\xDEW\x80c\x15\xD4dt\x14a\x02\x1BW\x80c\x17\x85\xF5<\x14a\x02XW\x80c$\x8A\x9C\xA3\x14a\x02\x81W\x80c//\xF1]\x14a\x02\xBEW[`\0\x80\xFD[4\x80\x15a\x01\xADW`\0\x80\xFD[Pa\x01\xC8`\x04\x806\x03\x81\x01\x90a\x01\xC3\x91\x90a\")V[a\x07CV[`@Qa\x01\xD5\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xEAW`\0\x80\xFD[Pa\x02\x05`\x04\x806\x03\x81\x01\x90a\x02\0\x91\x90a\"\xE4V[a\x07\xD2V[`@Qa\x02\x12\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02'W`\0\x80\xFD[Pa\x02B`\x04\x806\x03\x81\x01\x90a\x02=\x91\x90a\")V[a\x08LV[`@Qa\x02O\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02dW`\0\x80\xFD[Pa\x02\x7F`\x04\x806\x03\x81\x01\x90a\x02z\x91\x90a#oV[a\t\xE5V[\0[4\x80\x15a\x02\x8DW`\0\x80\xFD[Pa\x02\xA8`\x04\x806\x03\x81\x01\x90a\x02\xA3\x91\x90a#\xD2V[a\n\xABV[`@Qa\x02\xB5\x91\x90a$\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCAW`\0\x80\xFD[Pa\x02\xE5`\x04\x806\x03\x81\x01\x90a\x02\xE0\x91\x90a$)V[a\n\xCAV[\0[4\x80\x15a\x02\xF3W`\0\x80\xFD[Pa\x03\x0E`\x04\x806\x03\x81\x01\x90a\x03\t\x91\x90a$)V[a\n\xEBV[\0[4\x80\x15a\x03\x1CW`\0\x80\xFD[Pa\x037`\x04\x806\x03\x81\x01\x90a\x032\x91\x90a\")V[a\x0BnV[`@Qa\x03D\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03YW`\0\x80\xFD[Pa\x03t`\x04\x806\x03\x81\x01\x90a\x03o\x91\x90a\")V[a\x0B\xFDV[`@Qa\x03\x81\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x03\x9Fa\r\x1EV[`@Qa\x03\xAC\x91\x90a$\xC8V[`@Q\x80\x91\x03\x90\xF3[a\x03\xCF`\x04\x806\x03\x81\x01\x90a\x03\xCA\x91\x90a*5V[a\rDV[`@Qa\x03\xDC\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xF1W`\0\x80\xFD[Pa\x04\x0C`\x04\x806\x03\x81\x01\x90a\x04\x07\x91\x90a+\xD9V[a\x0F\xB4V[`@Qa\x04\x19\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04.W`\0\x80\xFD[Pa\x04I`\x04\x806\x03\x81\x01\x90a\x04D\x91\x90a\")V[a\x11\xD7V[`@Qa\x04V\x91\x90a-'V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04kW`\0\x80\xFD[Pa\x04\x86`\x04\x806\x03\x81\x01\x90a\x04\x81\x91\x90a-IV[a\x12lV[`@Qa\x04\x93\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xA8W`\0\x80\xFD[Pa\x04\xC3`\x04\x806\x03\x81\x01\x90a\x04\xBE\x91\x90a#oV[a\x12\xFBV[\0[4\x80\x15a\x04\xD1W`\0\x80\xFD[Pa\x04\xDAa\x13SV[`@Qa\x04\xE7\x91\x90a$\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xFCW`\0\x80\xFD[Pa\x05\x17`\x04\x806\x03\x81\x01\x90a\x05\x12\x91\x90a\")V[a\x13wV[`@Qa\x05$\x91\x90a-\xB5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x059W`\0\x80\xFD[Pa\x05T`\x04\x806\x03\x81\x01\x90a\x05O\x91\x90a$)V[a\x14\x06V[`@Qa\x05a\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05vW`\0\x80\xFD[Pa\x05\x7Fa\x14pV[`@Qa\x05\x8C\x91\x90a.GV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xA1W`\0\x80\xFD[Pa\x05\xBC`\x04\x806\x03\x81\x01\x90a\x05\xB7\x91\x90a\")V[a\x14\x83V[`@Qa\x05\xC9\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xDEW`\0\x80\xFD[Pa\x05\xE7a\x15\x12V[`@Qa\x05\xF4\x91\x90a$\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\tW`\0\x80\xFD[Pa\x06$`\x04\x806\x03\x81\x01\x90a\x06\x1F\x91\x90a.\x8EV[a\x15\x19V[`@Qa\x061\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06FW`\0\x80\xFD[Pa\x06a`\x04\x806\x03\x81\x01\x90a\x06\\\x91\x90a.\xBBV[a\x15\xA8V[\0[4\x80\x15a\x06oW`\0\x80\xFD[Pa\x06\x8A`\x04\x806\x03\x81\x01\x90a\x06\x85\x91\x90a/\x17V[a\x16\xB5V[`@Qa\x06\x97\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xACW`\0\x80\xFD[Pa\x06\xC7`\x04\x806\x03\x81\x01\x90a\x06\xC2\x91\x90a$)V[a\x17\xD9V[\0[4\x80\x15a\x06\xD5W`\0\x80\xFD[Pa\x06\xF0`\x04\x806\x03\x81\x01\x90a\x06\xEB\x91\x90a\")V[a\x17\xFAV[`@Qa\x06\xFD\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x12W`\0\x80\xFD[Pa\x07-`\x04\x806\x03\x81\x01\x90a\x07(\x91\x90a\")V[a\x19\x93V[`@Qa\x07:\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[`\0\x80a\x07Na\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xC6\xD05\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x89\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xCA\x91\x90a/\x83V[\x91PP\x91\x90PV[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x08EWPa\x08D\x82a\x1BfV[[\x90P\x91\x90PV[`\0a\x08x\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x08\xDDW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD4\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x08\xE7a\x1A\"V[\x90P`\0a\x08\xF3a\x1B\xD0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+U5Q\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t.\x91\x90a+\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tHW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\\W=`\0\x80>=`\0\xFD[PPPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x15\xD4dt\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x99\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDC\x91\x90a/\x83V[\x92PPP\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\n\x0F\x81a\x1D\x14V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\n}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nt\x90a0kV[`@Q\x80\x91\x03\x90\xFD[a\n\xA7\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1D(V[PPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\n\xD3\x82a\n\xABV[a\n\xDC\x81a\x1D\x14V[a\n\xE6\x83\x83a\x1D(V[PPPV[a\n\xF3a\x1E\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BW\x90a0\xFDV[`@Q\x80\x91\x03\x90\xFD[a\x0Bj\x82\x82a\x1E\x10V[PPV[`\0\x80a\x0Bya\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c9\xC7c\x9C\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xB4\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF5\x91\x90a/\x83V[\x91PP\x91\x90PV[`\0a\x0C)\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x0C\x8EW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x85\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x0C\x98a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cI\xFC\xBCT\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD3\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x16\x91\x90a/\x83V[\x91PP\x91\x90PV[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0a\rp\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\r\xD5W`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xCC\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\r\xDFa\x1A\"V[\x90P`\0a\r\xEBa\x1B\xD0V[\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c'\xBD\x06\x9E\x8B`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E&\x91\x90a-'V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E>W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0ERW=`\0\x80>=`\0\xFD[PPPP`\x02\x84Q\x14a\x0E\xA0W\x83Q`\x02`@Q\x7F!j\xEB?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x97\x92\x91\x90a1XV[`@Q\x80\x91\x03\x90\xFD[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xC83%4`\x02\x8C\x8C\x8C\x8C\x8C`\x01\x80`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xEC\x98\x97\x96\x95\x94\x93\x92\x91\x90a5\x93V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x0F\nW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F/\x91\x90a6IV[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A V\xF4\x8D\x8D\x84\x8E`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Fp\x94\x93\x92\x91\x90a6vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x8AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x9EW=`\0\x80>=`\0\xFD[PPPP\x80\x93PPPP\x98\x97PPPPPPPPV[`\0a\x0F\xE0\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x10EW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10<\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\x02\x82Q\x14a\x10\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x80\x90a7aV[`@Q\x80\x91\x03\x90\xFD[`\x02\x82Q\x14a\x10\xD3W\x81Q`\x02`@Q\x7F!j\xEB?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xCA\x92\x91\x90a1XV[`@Q\x80\x91\x03\x90\xFD[`\0a\x10\xDDa\x1A\"V[\x90P`\0a\x10\xE9a\x1B\xD0V[\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A V\xF4\x89\x89\x88\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11*\x94\x93\x92\x91\x90a6vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11XW=`\0\x80>=`\0\xFD[PPPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cx..\xA5\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x97\x92\x91\x90a7\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xC5W=`\0\x80>=`\0\xFD[PPPP\x84\x92PPP\x95\x94PPPPPV[```\0a\x11\xE3a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cj0\0\xC4\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x1E\x91\x90a+\xBEV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12d\x91\x90a8!V[\x91PP\x91\x90PV[`\0\x80a\x12wa\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16coP\xCD\x97\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB2\x91\x90a-'V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF3\x91\x90a6IV[\x91PP\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x13%\x81a\x1D\x14V[a\x13O\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1D(V[PPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\0\x80a\x13\x82a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x88\x0B\xC4\xE6\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xBD\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFE\x91\x90a8\x7FV[\x91PP\x91\x90PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80a\x14\x8Ea\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0[w_\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xC9\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\n\x91\x90a6IV[\x91PP\x91\x90PV[`\0\x80\x1B\x81V[`\0\x80a\x15$a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA2\xF4!\x04\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15_\x91\x90a-\xB5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA0\x91\x90a6IV[\x91PP\x91\x90PV[a\x15\xD2\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x167W`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16.\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x16Aa\x1B\xD0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cx..\xA5\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16~\x92\x91\x90a7\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xACW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0a\x16\xE1\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x17FW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17=\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x17Pa\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC9\xB2\x87I\x85\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x8D\x92\x91\x90a8\xACV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xD0\x91\x90a/\x83V[\x91PP\x92\x91PPV[a\x17\xE2\x82a\n\xABV[a\x17\xEB\x81a\x1D\x14V[a\x17\xF5\x83\x83a\x1E\x10V[PPPV[`\0a\x18&\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x18\x8BW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\x82\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x18\x95a\x1A\"V[\x90P`\0a\x18\xA1a\x1B\xD0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+U5Q\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xDC\x91\x90a+\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xF6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\nW=`\0\x80>=`\0\xFD[PPPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD9d\xA3u\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19G\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x8A\x91\x90a/\x83V[\x92PPP\x91\x90PV[`\0\x80a\x19\x9Ea\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF4\x8D`\xCA\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xD9\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x1A\x91\x90a/\x83V[\x91PP\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cr\x82?\xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xF3\x91\x90a8\xEAV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B \x92\x91\x90a9\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ba\x91\x90a9UV[\x90P\x90V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xA1\x91\x90a8\xEAV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xCE\x92\x91\x90a9\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x0F\x91\x90a9UV[\x90P\x90V[a\x1D%\x81a\x1D a\x1E\x08V[a\x1E\xF1V[PV[a\x1D2\x82\x82a\x14\x06V[a\x1E\x04W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1D\xA9a\x1E\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\x1E\x1A\x82\x82a\x14\x06V[\x15a\x1E\xEDW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1E\x92a\x1E\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a\x1E\xFB\x82\x82a\x14\x06V[a\x1FrWa\x1F\x08\x81a\x1FvV[a\x1F\x16\x83`\0\x1C` a\x1F\xA3V[`@Q` \x01a\x1F'\x92\x91\x90a:VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Fi\x91\x90a:\xC9V[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x1F\x9C\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x1F\xA3V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a\x1F\xB6\x91\x90a;\x1AV[a\x1F\xC0\x91\x90a;\\V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xD9Wa\x1F\xD8a$\xFEV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a \x0BW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a CWa Ba;\x90V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a \xA7Wa \xA6a;\x90V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a \xE7\x91\x90a;\x1AV[a \xF1\x91\x90a;\\V[\x90P[`\x01\x81\x11\x15a!\x91W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a!3Wa!2a;\x90V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a!JWa!Ia;\x90V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a!\x8A\x90a;\xBFV[\x90Pa \xF4V[P`\0\x84\x14a!\xD5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\xCC\x90a<4V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\"\x06\x81a!\xF3V[\x81\x14a\"\x11W`\0\x80\xFD[PV[`\0\x815\x90Pa\"#\x81a!\xFDV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\"?Wa\">a!\xE9V[[`\0a\"M\x84\x82\x85\x01a\"\x14V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\"k\x81a\"VV[\x82RPPV[`\0` \x82\x01\x90Pa\"\x86`\0\x83\x01\x84a\"bV[\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\"\xC1\x81a\"\x8CV[\x81\x14a\"\xCCW`\0\x80\xFD[PV[`\0\x815\x90Pa\"\xDE\x81a\"\xB8V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\"\xFAWa\"\xF9a!\xE9V[[`\0a#\x08\x84\x82\x85\x01a\"\xCFV[\x91PP\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a#<\x82a#\x11V[\x90P\x91\x90PV[a#L\x81a#1V[\x81\x14a#WW`\0\x80\xFD[PV[`\0\x815\x90Pa#i\x81a#CV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\x85Wa#\x84a!\xE9V[[`\0a#\x93\x84\x82\x85\x01a#ZV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a#\xAF\x81a#\x9CV[\x81\x14a#\xBAW`\0\x80\xFD[PV[`\0\x815\x90Pa#\xCC\x81a#\xA6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\xE8Wa#\xE7a!\xE9V[[`\0a#\xF6\x84\x82\x85\x01a#\xBDV[\x91PP\x92\x91PPV[a$\x08\x81a#\x9CV[\x82RPPV[`\0` \x82\x01\x90Pa$#`\0\x83\x01\x84a#\xFFV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a$@Wa$?a!\xE9V[[`\0a$N\x85\x82\x86\x01a#\xBDV[\x92PP` a$_\x85\x82\x86\x01a#ZV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a$\x8Ea$\x89a$\x84\x84a#\x11V[a$iV[a#\x11V[\x90P\x91\x90PV[`\0a$\xA0\x82a$sV[\x90P\x91\x90PV[`\0a$\xB2\x82a$\x95V[\x90P\x91\x90PV[a$\xC2\x81a$\xA7V[\x82RPPV[`\0` \x82\x01\x90Pa$\xDD`\0\x83\x01\x84a$\xB9V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a%6\x82a$\xEDV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a%UWa%Ta$\xFEV[[\x80`@RPPPV[`\0a%ha!\xDFV[\x90Pa%t\x82\x82a%-V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x94Wa%\x93a$\xFEV[[a%\x9D\x82a$\xEDV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a%\xCCa%\xC7\x84a%yV[a%^V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a%\xE8Wa%\xE7a$\xE8V[[a%\xF3\x84\x82\x85a%\xAAV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a&\x10Wa&\x0Fa$\xE3V[[\x815a& \x84\x82` \x86\x01a%\xB9V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&DWa&Ca$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a&ma&h\x84a&)V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a&\x90Wa&\x8Fa&UV[[\x83[\x81\x81\x10\x15a&\xB9W\x80a&\xA5\x88\x82a\"\x14V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa&\x92V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a&\xD8Wa&\xD7a$\xE3V[[\x815a&\xE8\x84\x82` \x86\x01a&ZV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\x0CWa'\x0Ba$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a'0a'+\x84a&\xF1V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a'SWa'Ra&UV[[\x83[\x81\x81\x10\x15a'\x9AW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'xWa'wa$\xE3V[[\x80\x86\x01a'\x85\x89\x82a%\xFBV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa'UV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a'\xB9Wa'\xB8a$\xE3V[[\x815a'\xC9\x84\x82` \x86\x01a'\x1DV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\xEDWa'\xECa$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a(\x11a(\x0C\x84a'\xD2V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a(4Wa(3a&UV[[\x83[\x81\x81\x10\x15a({W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(YWa(Xa$\xE3V[[\x80\x86\x01a(f\x89\x82a&\xC3V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa(6V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a(\x9AWa(\x99a$\xE3V[[\x815a(\xAA\x84\x82` \x86\x01a'\xFEV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xCEWa(\xCDa$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xFAWa(\xF9a$\xFEV[[a)\x03\x82a$\xEDV[\x90P` \x81\x01\x90P\x91\x90PV[`\0a)#a)\x1E\x84a(\xDFV[a%^V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a)?Wa)>a$\xE8V[[a)J\x84\x82\x85a%\xAAV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a)gWa)fa$\xE3V[[\x815a)w\x84\x82` \x86\x01a)\x10V[\x91PP\x92\x91PPV[`\0a)\x93a)\x8E\x84a(\xB3V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a)\xB6Wa)\xB5a&UV[[\x83[\x81\x81\x10\x15a)\xFDW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xDBWa)\xDAa$\xE3V[[\x80\x86\x01a)\xE8\x89\x82a)RV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa)\xB8V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a*\x1CWa*\x1Ba$\xE3V[[\x815a*,\x84\x82` \x86\x01a)\x80V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a*VWa*Ua!\xE9V[[`\0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*tWa*sa!\xEEV[[a*\x80\x8B\x82\x8C\x01a%\xFBV[\x98PP` \x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xA1Wa*\xA0a!\xEEV[[a*\xAD\x8B\x82\x8C\x01a%\xFBV[\x97PP`@a*\xBE\x8B\x82\x8C\x01a\"\x14V[\x96PP``\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xDFWa*\xDEa!\xEEV[[a*\xEB\x8B\x82\x8C\x01a&\xC3V[\x95PP`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x0CWa+\x0Ba!\xEEV[[a+\x18\x8B\x82\x8C\x01a'\xA4V[\x94PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+9Wa+8a!\xEEV[[a+E\x8B\x82\x8C\x01a'\xA4V[\x93PP`\xC0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+fWa+ea!\xEEV[[a+r\x8B\x82\x8C\x01a(\x85V[\x92PP`\xE0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x93Wa+\x92a!\xEEV[[a+\x9F\x8B\x82\x8C\x01a*\x07V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[a+\xB8\x81a!\xF3V[\x82RPPV[`\0` \x82\x01\x90Pa+\xD3`\0\x83\x01\x84a+\xAFV[\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a+\xF5Wa+\xF4a!\xE9V[[`\0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x13Wa,\x12a!\xEEV[[a,\x1F\x88\x82\x89\x01a%\xFBV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,@Wa,?a!\xEEV[[a,L\x88\x82\x89\x01a%\xFBV[\x94PP`@a,]\x88\x82\x89\x01a\"\x14V[\x93PP``a,n\x88\x82\x89\x01a\"\x14V[\x92PP`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x8FWa,\x8Ea!\xEEV[[a,\x9B\x88\x82\x89\x01a*\x07V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a,\xE2W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa,\xC7V[`\0\x84\x84\x01RPPPPV[`\0a,\xF9\x82a,\xA8V[a-\x03\x81\x85a,\xB3V[\x93Pa-\x13\x81\x85` \x86\x01a,\xC4V[a-\x1C\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra-A\x81\x84a,\xEEV[\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a-_Wa-^a!\xE9V[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-}Wa-|a!\xEEV[[a-\x89\x84\x82\x85\x01a%\xFBV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a-\xAF\x81a-\x92V[\x82RPPV[`\0` \x82\x01\x90Pa-\xCA`\0\x83\x01\x84a-\xA6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a.\x10Wa.\x0Fa-\xD0V[[PV[`\0\x81\x90Pa.!\x82a-\xFFV[\x91\x90PV[`\0a.1\x82a.\x13V[\x90P\x91\x90PV[a.A\x81a.&V[\x82RPPV[`\0` \x82\x01\x90Pa.\\`\0\x83\x01\x84a.8V[\x92\x91PPV[a.k\x81a-\x92V[\x81\x14a.vW`\0\x80\xFD[PV[`\0\x815\x90Pa.\x88\x81a.bV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a.\xA4Wa.\xA3a!\xE9V[[`\0a.\xB2\x84\x82\x85\x01a.yV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a.\xD2Wa.\xD1a!\xE9V[[`\0a.\xE0\x85\x82\x86\x01a\"\x14V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x01Wa/\0a!\xEEV[[a/\r\x85\x82\x86\x01a*\x07V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/.Wa/-a!\xE9V[[`\0a/<\x85\x82\x86\x01a.yV[\x92PP` a/M\x85\x82\x86\x01a\"\x14V[\x91PP\x92P\x92\x90PV[a/`\x81a\"VV[\x81\x14a/kW`\0\x80\xFD[PV[`\0\x81Q\x90Pa/}\x81a/WV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a/\x99Wa/\x98a!\xE9V[[`\0a/\xA7\x84\x82\x85\x01a/nV[\x91PP\x92\x91PPV[a/\xB9\x81a#1V[\x82RPPV[`\0`@\x82\x01\x90Pa/\xD4`\0\x83\x01\x85a/\xB0V[a/\xE1` \x83\x01\x84a/\xB0V[\x93\x92PPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCannot remove self as admin.  Ha`\0\x82\x01R\x7Fve the new admin do it.\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a0U`7\x83a/\xE8V[\x91Pa0`\x82a/\xF9V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra0\x84\x81a0HV[\x90P\x91\x90PV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a0\xE7`/\x83a/\xE8V[\x91Pa0\xF2\x82a0\x8BV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra1\x16\x81a0\xDAV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a1Ba1=a18\x84a1\x1DV[a$iV[a!\xF3V[\x90P\x91\x90PV[a1R\x81a1'V[\x82RPPV[`\0`@\x82\x01\x90Pa1m`\0\x83\x01\x85a+\xAFV[a1z` \x83\x01\x84a1IV[\x93\x92PPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a1\xB6\x81a!\xF3V[\x82RPPV[`\0a1\xC8\x83\x83a1\xADV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a1\xEC\x82a1\x81V[a1\xF6\x81\x85a1\x8CV[\x93Pa2\x01\x83a1\x9DV[\x80`\0[\x83\x81\x10\x15a22W\x81Qa2\x19\x88\x82a1\xBCV[\x97Pa2$\x83a1\xD4V[\x92PP`\x01\x81\x01\x90Pa2\x05V[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a2\x87\x82a,\xA8V[a2\x91\x81\x85a2kV[\x93Pa2\xA1\x81\x85` \x86\x01a,\xC4V[a2\xAA\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0a2\xC1\x83\x83a2|V[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a2\xE1\x82a2?V[a2\xEB\x81\x85a2JV[\x93P\x83` \x82\x02\x85\x01a2\xFD\x85a2[V[\x80`\0[\x85\x81\x10\x15a39W\x84\x84\x03\x89R\x81Qa3\x1A\x85\x82a2\xB5V[\x94Pa3%\x83a2\xC9V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa3\x01V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a3\x93\x82a1\x81V[a3\x9D\x81\x85a3wV[\x93Pa3\xA8\x83a1\x9DV[\x80`\0[\x83\x81\x10\x15a3\xD9W\x81Qa3\xC0\x88\x82a1\xBCV[\x97Pa3\xCB\x83a1\xD4V[\x92PP`\x01\x81\x01\x90Pa3\xACV[P\x85\x93PPPP\x92\x91PPV[`\0a3\xF2\x83\x83a3\x88V[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a4\x12\x82a3KV[a4\x1C\x81\x85a3VV[\x93P\x83` \x82\x02\x85\x01a4.\x85a3gV[\x80`\0[\x85\x81\x10\x15a4jW\x84\x84\x03\x89R\x81Qa4K\x85\x82a3\xE6V[\x94Pa4V\x83a3\xFAV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa42V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a4\xCF\x82a4\xA8V[a4\xD9\x81\x85a4\xB3V[\x93Pa4\xE9\x81\x85` \x86\x01a,\xC4V[a4\xF2\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0a5\t\x83\x83a4\xC4V[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a5)\x82a4|V[a53\x81\x85a4\x87V[\x93P\x83` \x82\x02\x85\x01a5E\x85a4\x98V[\x80`\0[\x85\x81\x10\x15a5\x81W\x84\x84\x03\x89R\x81Qa5b\x85\x82a4\xFDV[\x94Pa5m\x83a5\x11V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa5IV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0a\x01\0\x82\x01\x90Pa5\xA9`\0\x83\x01\x8Ba1IV[\x81\x81\x03` \x83\x01Ra5\xBB\x81\x8Aa1\xE1V[\x90P\x81\x81\x03`@\x83\x01Ra5\xCF\x81\x89a2\xD6V[\x90P\x81\x81\x03``\x83\x01Ra5\xE3\x81\x88a2\xD6V[\x90P\x81\x81\x03`\x80\x83\x01Ra5\xF7\x81\x87a4\x07V[\x90P\x81\x81\x03`\xA0\x83\x01Ra6\x0B\x81\x86a5\x1EV[\x90Pa6\x1A`\xC0\x83\x01\x85a\"bV[a6'`\xE0\x83\x01\x84a\"bV[\x99\x98PPPPPPPPPV[`\0\x81Q\x90Pa6C\x81a!\xFDV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a6_Wa6^a!\xE9V[[`\0a6m\x84\x82\x85\x01a64V[\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra6\x90\x81\x87a,\xEEV[\x90P\x81\x81\x03` \x83\x01Ra6\xA4\x81\x86a,\xEEV[\x90Pa6\xB3`@\x83\x01\x85a+\xAFV[a6\xC0``\x83\x01\x84a+\xAFV[\x95\x94PPPPPV[\x7FDomainWalletRegistry: metadata n`\0\x82\x01R\x7Fame and url must be set in metad` \x82\x01R\x7Fata\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0a7K`C\x83a/\xE8V[\x91Pa7V\x82a6\xC9V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra7z\x81a7>V[\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa7\x96`\0\x83\x01\x85a+\xAFV[\x81\x81\x03` \x83\x01Ra7\xA8\x81\x84a5\x1EV[\x90P\x93\x92PPPV[`\0a7\xC4a7\xBF\x84a%yV[a%^V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a7\xE0Wa7\xDFa$\xE8V[[a7\xEB\x84\x82\x85a,\xC4V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a8\x08Wa8\x07a$\xE3V[[\x81Qa8\x18\x84\x82` \x86\x01a7\xB1V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a87Wa86a!\xE9V[[`\0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8UWa8Ta!\xEEV[[a8a\x84\x82\x85\x01a7\xF3V[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa8y\x81a.bV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8\x95Wa8\x94a!\xE9V[[`\0a8\xA3\x84\x82\x85\x01a8jV[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa8\xC1`\0\x83\x01\x85a-\xA6V[a8\xCE` \x83\x01\x84a+\xAFV[\x93\x92PPPV[`\0\x81Q\x90Pa8\xE4\x81a#\xA6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a9\0Wa8\xFFa!\xE9V[[`\0a9\x0E\x84\x82\x85\x01a8\xD5V[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa9,`\0\x83\x01\x85a#\xFFV[a99` \x83\x01\x84a.8V[\x93\x92PPPV[`\0\x81Q\x90Pa9O\x81a#CV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a9kWa9ja!\xE9V[[`\0a9y\x84\x82\x85\x01a9@V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a9\xC3`\x17\x83a9\x82V[\x91Pa9\xCE\x82a9\x8DV[`\x17\x82\x01\x90P\x91\x90PV[`\0a9\xE4\x82a4\xA8V[a9\xEE\x81\x85a9\x82V[\x93Pa9\xFE\x81\x85` \x86\x01a,\xC4V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a:@`\x11\x83a9\x82V[\x91Pa:K\x82a:\nV[`\x11\x82\x01\x90P\x91\x90PV[`\0a:a\x82a9\xB6V[\x91Pa:m\x82\x85a9\xD9V[\x91Pa:x\x82a:3V[\x91Pa:\x84\x82\x84a9\xD9V[\x91P\x81\x90P\x93\x92PPPV[`\0a:\x9B\x82a4\xA8V[a:\xA5\x81\x85a/\xE8V[\x93Pa:\xB5\x81\x85` \x86\x01a,\xC4V[a:\xBE\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra:\xE3\x81\x84a:\x90V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a;%\x82a!\xF3V[\x91Pa;0\x83a!\xF3V[\x92P\x82\x82\x02a;>\x81a!\xF3V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a;UWa;Ta:\xEBV[[P\x92\x91PPV[`\0a;g\x82a!\xF3V[\x91Pa;r\x83a!\xF3V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a;\x8AWa;\x89a:\xEBV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a;\xCA\x82a!\xF3V[\x91P`\0\x82\x03a;\xDDWa;\xDCa:\xEBV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a<\x1E` \x83a/\xE8V[\x91Pa<)\x82a;\xE8V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra<M\x81a<\x11V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 e\xBC\xD6\x15P{K\xB4SE\xED`L\x0E.C\xD4\xF8\xD1\xC8G\xD4\xE6Tk<\x86\x8D\xE4\xB9\xE0\x97dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static DOMAINWALLETREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x9CW`\x005`\xE0\x1C\x80coP\xCD\x97\x11a\0\xECW\x80c\xA2\x17\xFD\xDF\x11a\0\x8AW\x80c\xC9\xB2\x87I\x11a\0dW\x80c\xC9\xB2\x87I\x14a\x06cW\x80c\xD5Gt\x1F\x14a\x06\xA0W\x80c\xD9d\xA3u\x14a\x06\xC9W\x80c\xF4\x8D`\xCA\x14a\x07\x06Wa\x01\x9CV[\x80c\xA2\x17\xFD\xDF\x14a\x05\xD2W\x80c\xA2\xF4!\x04\x14a\x05\xFDW\x80c\xB6\xE7\xF4\xD2\x14a\x06:Wa\x01\x9CV[\x80c\x88\x0B\xC4\xE6\x11a\0\xC6W\x80c\x88\x0B\xC4\xE6\x14a\x04\xF0W\x80c\x91\xD1HT\x14a\x05-W\x80c\x9D\xCA\x002\x14a\x05jW\x80c\xA0[w_\x14a\x05\x95Wa\x01\x9CV[\x80coP\xCD\x97\x14a\x04_W\x80cpH\x02u\x14a\x04\x9CW\x80cu\xB28\xFC\x14a\x04\xC5Wa\x01\x9CV[\x80c6V\x8A\xBE\x11a\x01YW\x80cP\xD1{^\x11a\x013W\x80cP\xD1{^\x14a\x03\x8AW\x80cU0\xAD\xF8\x14a\x03\xB5W\x80c[\x04W$\x14a\x03\xE5W\x80cj0\0\xC4\x14a\x04\"Wa\x01\x9CV[\x80c6V\x8A\xBE\x14a\x02\xE7W\x80c9\xC7c\x9C\x14a\x03\x10W\x80cI\xFC\xBCT\x14a\x03MWa\x01\x9CV[\x80c\x01\xC6\xD05\x14a\x01\xA1W\x80c\x01\xFF\xC9\xA7\x14a\x01\xDEW\x80c\x15\xD4dt\x14a\x02\x1BW\x80c\x17\x85\xF5<\x14a\x02XW\x80c$\x8A\x9C\xA3\x14a\x02\x81W\x80c//\xF1]\x14a\x02\xBEW[`\0\x80\xFD[4\x80\x15a\x01\xADW`\0\x80\xFD[Pa\x01\xC8`\x04\x806\x03\x81\x01\x90a\x01\xC3\x91\x90a\")V[a\x07CV[`@Qa\x01\xD5\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xEAW`\0\x80\xFD[Pa\x02\x05`\x04\x806\x03\x81\x01\x90a\x02\0\x91\x90a\"\xE4V[a\x07\xD2V[`@Qa\x02\x12\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02'W`\0\x80\xFD[Pa\x02B`\x04\x806\x03\x81\x01\x90a\x02=\x91\x90a\")V[a\x08LV[`@Qa\x02O\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02dW`\0\x80\xFD[Pa\x02\x7F`\x04\x806\x03\x81\x01\x90a\x02z\x91\x90a#oV[a\t\xE5V[\0[4\x80\x15a\x02\x8DW`\0\x80\xFD[Pa\x02\xA8`\x04\x806\x03\x81\x01\x90a\x02\xA3\x91\x90a#\xD2V[a\n\xABV[`@Qa\x02\xB5\x91\x90a$\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCAW`\0\x80\xFD[Pa\x02\xE5`\x04\x806\x03\x81\x01\x90a\x02\xE0\x91\x90a$)V[a\n\xCAV[\0[4\x80\x15a\x02\xF3W`\0\x80\xFD[Pa\x03\x0E`\x04\x806\x03\x81\x01\x90a\x03\t\x91\x90a$)V[a\n\xEBV[\0[4\x80\x15a\x03\x1CW`\0\x80\xFD[Pa\x037`\x04\x806\x03\x81\x01\x90a\x032\x91\x90a\")V[a\x0BnV[`@Qa\x03D\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03YW`\0\x80\xFD[Pa\x03t`\x04\x806\x03\x81\x01\x90a\x03o\x91\x90a\")V[a\x0B\xFDV[`@Qa\x03\x81\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x03\x9Fa\r\x1EV[`@Qa\x03\xAC\x91\x90a$\xC8V[`@Q\x80\x91\x03\x90\xF3[a\x03\xCF`\x04\x806\x03\x81\x01\x90a\x03\xCA\x91\x90a*5V[a\rDV[`@Qa\x03\xDC\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xF1W`\0\x80\xFD[Pa\x04\x0C`\x04\x806\x03\x81\x01\x90a\x04\x07\x91\x90a+\xD9V[a\x0F\xB4V[`@Qa\x04\x19\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04.W`\0\x80\xFD[Pa\x04I`\x04\x806\x03\x81\x01\x90a\x04D\x91\x90a\")V[a\x11\xD7V[`@Qa\x04V\x91\x90a-'V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04kW`\0\x80\xFD[Pa\x04\x86`\x04\x806\x03\x81\x01\x90a\x04\x81\x91\x90a-IV[a\x12lV[`@Qa\x04\x93\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xA8W`\0\x80\xFD[Pa\x04\xC3`\x04\x806\x03\x81\x01\x90a\x04\xBE\x91\x90a#oV[a\x12\xFBV[\0[4\x80\x15a\x04\xD1W`\0\x80\xFD[Pa\x04\xDAa\x13SV[`@Qa\x04\xE7\x91\x90a$\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xFCW`\0\x80\xFD[Pa\x05\x17`\x04\x806\x03\x81\x01\x90a\x05\x12\x91\x90a\")V[a\x13wV[`@Qa\x05$\x91\x90a-\xB5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x059W`\0\x80\xFD[Pa\x05T`\x04\x806\x03\x81\x01\x90a\x05O\x91\x90a$)V[a\x14\x06V[`@Qa\x05a\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05vW`\0\x80\xFD[Pa\x05\x7Fa\x14pV[`@Qa\x05\x8C\x91\x90a.GV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xA1W`\0\x80\xFD[Pa\x05\xBC`\x04\x806\x03\x81\x01\x90a\x05\xB7\x91\x90a\")V[a\x14\x83V[`@Qa\x05\xC9\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xDEW`\0\x80\xFD[Pa\x05\xE7a\x15\x12V[`@Qa\x05\xF4\x91\x90a$\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\tW`\0\x80\xFD[Pa\x06$`\x04\x806\x03\x81\x01\x90a\x06\x1F\x91\x90a.\x8EV[a\x15\x19V[`@Qa\x061\x91\x90a+\xBEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06FW`\0\x80\xFD[Pa\x06a`\x04\x806\x03\x81\x01\x90a\x06\\\x91\x90a.\xBBV[a\x15\xA8V[\0[4\x80\x15a\x06oW`\0\x80\xFD[Pa\x06\x8A`\x04\x806\x03\x81\x01\x90a\x06\x85\x91\x90a/\x17V[a\x16\xB5V[`@Qa\x06\x97\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xACW`\0\x80\xFD[Pa\x06\xC7`\x04\x806\x03\x81\x01\x90a\x06\xC2\x91\x90a$)V[a\x17\xD9V[\0[4\x80\x15a\x06\xD5W`\0\x80\xFD[Pa\x06\xF0`\x04\x806\x03\x81\x01\x90a\x06\xEB\x91\x90a\")V[a\x17\xFAV[`@Qa\x06\xFD\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x12W`\0\x80\xFD[Pa\x07-`\x04\x806\x03\x81\x01\x90a\x07(\x91\x90a\")V[a\x19\x93V[`@Qa\x07:\x91\x90a\"qV[`@Q\x80\x91\x03\x90\xF3[`\0\x80a\x07Na\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xC6\xD05\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x89\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xCA\x91\x90a/\x83V[\x91PP\x91\x90PV[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x08EWPa\x08D\x82a\x1BfV[[\x90P\x91\x90PV[`\0a\x08x\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x08\xDDW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD4\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x08\xE7a\x1A\"V[\x90P`\0a\x08\xF3a\x1B\xD0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+U5Q\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t.\x91\x90a+\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tHW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\\W=`\0\x80>=`\0\xFD[PPPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x15\xD4dt\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x99\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDC\x91\x90a/\x83V[\x92PPP\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\n\x0F\x81a\x1D\x14V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\n}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nt\x90a0kV[`@Q\x80\x91\x03\x90\xFD[a\n\xA7\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1D(V[PPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\n\xD3\x82a\n\xABV[a\n\xDC\x81a\x1D\x14V[a\n\xE6\x83\x83a\x1D(V[PPPV[a\n\xF3a\x1E\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BW\x90a0\xFDV[`@Q\x80\x91\x03\x90\xFD[a\x0Bj\x82\x82a\x1E\x10V[PPV[`\0\x80a\x0Bya\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c9\xC7c\x9C\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xB4\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF5\x91\x90a/\x83V[\x91PP\x91\x90PV[`\0a\x0C)\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x0C\x8EW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x85\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x0C\x98a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cI\xFC\xBCT\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD3\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x16\x91\x90a/\x83V[\x91PP\x91\x90PV[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\0a\rp\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\r\xD5W`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xCC\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\r\xDFa\x1A\"V[\x90P`\0a\r\xEBa\x1B\xD0V[\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c'\xBD\x06\x9E\x8B`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E&\x91\x90a-'V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0E>W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0ERW=`\0\x80>=`\0\xFD[PPPP`\x02\x84Q\x14a\x0E\xA0W\x83Q`\x02`@Q\x7F!j\xEB?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x97\x92\x91\x90a1XV[`@Q\x80\x91\x03\x90\xFD[`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xC83%4`\x02\x8C\x8C\x8C\x8C\x8C`\x01\x80`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xEC\x98\x97\x96\x95\x94\x93\x92\x91\x90a5\x93V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x0F\nW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F/\x91\x90a6IV[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A V\xF4\x8D\x8D\x84\x8E`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Fp\x94\x93\x92\x91\x90a6vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x8AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x9EW=`\0\x80>=`\0\xFD[PPPP\x80\x93PPPP\x98\x97PPPPPPPPV[`\0a\x0F\xE0\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x10EW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10<\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\x02\x82Q\x14a\x10\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x80\x90a7aV[`@Q\x80\x91\x03\x90\xFD[`\x02\x82Q\x14a\x10\xD3W\x81Q`\x02`@Q\x7F!j\xEB?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xCA\x92\x91\x90a1XV[`@Q\x80\x91\x03\x90\xFD[`\0a\x10\xDDa\x1A\"V[\x90P`\0a\x10\xE9a\x1B\xD0V[\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9A V\xF4\x89\x89\x88\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11*\x94\x93\x92\x91\x90a6vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11XW=`\0\x80>=`\0\xFD[PPPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cx..\xA5\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x97\x92\x91\x90a7\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xC5W=`\0\x80>=`\0\xFD[PPPP\x84\x92PPP\x95\x94PPPPPV[```\0a\x11\xE3a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cj0\0\xC4\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x1E\x91\x90a+\xBEV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12d\x91\x90a8!V[\x91PP\x91\x90PV[`\0\x80a\x12wa\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16coP\xCD\x97\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB2\x91\x90a-'V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF3\x91\x90a6IV[\x91PP\x91\x90PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x13%\x81a\x1D\x14V[a\x13O\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x83a\x1D(V[PPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\0\x80a\x13\x82a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x88\x0B\xC4\xE6\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xBD\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFE\x91\x90a8\x7FV[\x91PP\x91\x90PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80a\x14\x8Ea\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA0[w_\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xC9\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\n\x91\x90a6IV[\x91PP\x91\x90PV[`\0\x80\x1B\x81V[`\0\x80a\x15$a\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA2\xF4!\x04\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15_\x91\x90a-\xB5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA0\x91\x90a6IV[\x91PP\x91\x90PV[a\x15\xD2\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x167W`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16.\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x16Aa\x1B\xD0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cx..\xA5\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16~\x92\x91\x90a7\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xACW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0a\x16\xE1\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x17FW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17=\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x17Pa\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC9\xB2\x87I\x85\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x8D\x92\x91\x90a8\xACV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xD0\x91\x90a/\x83V[\x91PP\x92\x91PPV[a\x17\xE2\x82a\n\xABV[a\x17\xEB\x81a\x1D\x14V[a\x17\xF5\x83\x83a\x1E\x10V[PPPV[`\0a\x18&\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x14\x06V[a\x18\x8BW`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163`@Q\x7F4\xA5B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\x82\x92\x91\x90a/\xBFV[`@Q\x80\x91\x03\x90\xFD[`\0a\x18\x95a\x1A\"V[\x90P`\0a\x18\xA1a\x1B\xD0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+U5Q\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xDC\x91\x90a+\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xF6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\nW=`\0\x80>=`\0\xFD[PPPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD9d\xA3u\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19G\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x8A\x91\x90a/\x83V[\x92PPP\x91\x90PV[`\0\x80a\x19\x9Ea\x1A\"V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF4\x8D`\xCA\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xD9\x91\x90a+\xBEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x1A\x91\x90a/\x83V[\x91PP\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cr\x82?\xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xF3\x91\x90a8\xEAV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B \x92\x91\x90a9\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ba\x91\x90a9UV[\x90P\x90V[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xA1\x91\x90a8\xEAV[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xCE\x92\x91\x90a9\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x0F\x91\x90a9UV[\x90P\x90V[a\x1D%\x81a\x1D a\x1E\x08V[a\x1E\xF1V[PV[a\x1D2\x82\x82a\x14\x06V[a\x1E\x04W`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1D\xA9a\x1E\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a\x1E\x1A\x82\x82a\x14\x06V[\x15a\x1E\xEDW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x1E\x92a\x1E\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a\x1E\xFB\x82\x82a\x14\x06V[a\x1FrWa\x1F\x08\x81a\x1FvV[a\x1F\x16\x83`\0\x1C` a\x1F\xA3V[`@Q` \x01a\x1F'\x92\x91\x90a:VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Fi\x91\x90a:\xC9V[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x1F\x9C\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x1F\xA3V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a\x1F\xB6\x91\x90a;\x1AV[a\x1F\xC0\x91\x90a;\\V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xD9Wa\x1F\xD8a$\xFEV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a \x0BW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a CWa Ba;\x90V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a \xA7Wa \xA6a;\x90V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a \xE7\x91\x90a;\x1AV[a \xF1\x91\x90a;\\V[\x90P[`\x01\x81\x11\x15a!\x91W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a!3Wa!2a;\x90V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a!JWa!Ia;\x90V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a!\x8A\x90a;\xBFV[\x90Pa \xF4V[P`\0\x84\x14a!\xD5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\xCC\x90a<4V[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\"\x06\x81a!\xF3V[\x81\x14a\"\x11W`\0\x80\xFD[PV[`\0\x815\x90Pa\"#\x81a!\xFDV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\"?Wa\">a!\xE9V[[`\0a\"M\x84\x82\x85\x01a\"\x14V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\"k\x81a\"VV[\x82RPPV[`\0` \x82\x01\x90Pa\"\x86`\0\x83\x01\x84a\"bV[\x92\x91PPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\"\xC1\x81a\"\x8CV[\x81\x14a\"\xCCW`\0\x80\xFD[PV[`\0\x815\x90Pa\"\xDE\x81a\"\xB8V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\"\xFAWa\"\xF9a!\xE9V[[`\0a#\x08\x84\x82\x85\x01a\"\xCFV[\x91PP\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a#<\x82a#\x11V[\x90P\x91\x90PV[a#L\x81a#1V[\x81\x14a#WW`\0\x80\xFD[PV[`\0\x815\x90Pa#i\x81a#CV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\x85Wa#\x84a!\xE9V[[`\0a#\x93\x84\x82\x85\x01a#ZV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a#\xAF\x81a#\x9CV[\x81\x14a#\xBAW`\0\x80\xFD[PV[`\0\x815\x90Pa#\xCC\x81a#\xA6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a#\xE8Wa#\xE7a!\xE9V[[`\0a#\xF6\x84\x82\x85\x01a#\xBDV[\x91PP\x92\x91PPV[a$\x08\x81a#\x9CV[\x82RPPV[`\0` \x82\x01\x90Pa$#`\0\x83\x01\x84a#\xFFV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a$@Wa$?a!\xE9V[[`\0a$N\x85\x82\x86\x01a#\xBDV[\x92PP` a$_\x85\x82\x86\x01a#ZV[\x91PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[`\0a$\x8Ea$\x89a$\x84\x84a#\x11V[a$iV[a#\x11V[\x90P\x91\x90PV[`\0a$\xA0\x82a$sV[\x90P\x91\x90PV[`\0a$\xB2\x82a$\x95V[\x90P\x91\x90PV[a$\xC2\x81a$\xA7V[\x82RPPV[`\0` \x82\x01\x90Pa$\xDD`\0\x83\x01\x84a$\xB9V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a%6\x82a$\xEDV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a%UWa%Ta$\xFEV[[\x80`@RPPPV[`\0a%ha!\xDFV[\x90Pa%t\x82\x82a%-V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\x94Wa%\x93a$\xFEV[[a%\x9D\x82a$\xEDV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a%\xCCa%\xC7\x84a%yV[a%^V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a%\xE8Wa%\xE7a$\xE8V[[a%\xF3\x84\x82\x85a%\xAAV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a&\x10Wa&\x0Fa$\xE3V[[\x815a& \x84\x82` \x86\x01a%\xB9V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&DWa&Ca$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0a&ma&h\x84a&)V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a&\x90Wa&\x8Fa&UV[[\x83[\x81\x81\x10\x15a&\xB9W\x80a&\xA5\x88\x82a\"\x14V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa&\x92V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a&\xD8Wa&\xD7a$\xE3V[[\x815a&\xE8\x84\x82` \x86\x01a&ZV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\x0CWa'\x0Ba$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a'0a'+\x84a&\xF1V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a'SWa'Ra&UV[[\x83[\x81\x81\x10\x15a'\x9AW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'xWa'wa$\xE3V[[\x80\x86\x01a'\x85\x89\x82a%\xFBV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa'UV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a'\xB9Wa'\xB8a$\xE3V[[\x815a'\xC9\x84\x82` \x86\x01a'\x1DV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\xEDWa'\xECa$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a(\x11a(\x0C\x84a'\xD2V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a(4Wa(3a&UV[[\x83[\x81\x81\x10\x15a({W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(YWa(Xa$\xE3V[[\x80\x86\x01a(f\x89\x82a&\xC3V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa(6V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a(\x9AWa(\x99a$\xE3V[[\x815a(\xAA\x84\x82` \x86\x01a'\xFEV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xCEWa(\xCDa$\xFEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xFAWa(\xF9a$\xFEV[[a)\x03\x82a$\xEDV[\x90P` \x81\x01\x90P\x91\x90PV[`\0a)#a)\x1E\x84a(\xDFV[a%^V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a)?Wa)>a$\xE8V[[a)J\x84\x82\x85a%\xAAV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a)gWa)fa$\xE3V[[\x815a)w\x84\x82` \x86\x01a)\x10V[\x91PP\x92\x91PPV[`\0a)\x93a)\x8E\x84a(\xB3V[a%^V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a)\xB6Wa)\xB5a&UV[[\x83[\x81\x81\x10\x15a)\xFDW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xDBWa)\xDAa$\xE3V[[\x80\x86\x01a)\xE8\x89\x82a)RV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa)\xB8V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a*\x1CWa*\x1Ba$\xE3V[[\x815a*,\x84\x82` \x86\x01a)\x80V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a*VWa*Ua!\xE9V[[`\0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*tWa*sa!\xEEV[[a*\x80\x8B\x82\x8C\x01a%\xFBV[\x98PP` \x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xA1Wa*\xA0a!\xEEV[[a*\xAD\x8B\x82\x8C\x01a%\xFBV[\x97PP`@a*\xBE\x8B\x82\x8C\x01a\"\x14V[\x96PP``\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xDFWa*\xDEa!\xEEV[[a*\xEB\x8B\x82\x8C\x01a&\xC3V[\x95PP`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x0CWa+\x0Ba!\xEEV[[a+\x18\x8B\x82\x8C\x01a'\xA4V[\x94PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+9Wa+8a!\xEEV[[a+E\x8B\x82\x8C\x01a'\xA4V[\x93PP`\xC0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+fWa+ea!\xEEV[[a+r\x8B\x82\x8C\x01a(\x85V[\x92PP`\xE0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x93Wa+\x92a!\xEEV[[a+\x9F\x8B\x82\x8C\x01a*\x07V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[a+\xB8\x81a!\xF3V[\x82RPPV[`\0` \x82\x01\x90Pa+\xD3`\0\x83\x01\x84a+\xAFV[\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a+\xF5Wa+\xF4a!\xE9V[[`\0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x13Wa,\x12a!\xEEV[[a,\x1F\x88\x82\x89\x01a%\xFBV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,@Wa,?a!\xEEV[[a,L\x88\x82\x89\x01a%\xFBV[\x94PP`@a,]\x88\x82\x89\x01a\"\x14V[\x93PP``a,n\x88\x82\x89\x01a\"\x14V[\x92PP`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x8FWa,\x8Ea!\xEEV[[a,\x9B\x88\x82\x89\x01a*\x07V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a,\xE2W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa,\xC7V[`\0\x84\x84\x01RPPPPV[`\0a,\xF9\x82a,\xA8V[a-\x03\x81\x85a,\xB3V[\x93Pa-\x13\x81\x85` \x86\x01a,\xC4V[a-\x1C\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra-A\x81\x84a,\xEEV[\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a-_Wa-^a!\xE9V[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-}Wa-|a!\xEEV[[a-\x89\x84\x82\x85\x01a%\xFBV[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a-\xAF\x81a-\x92V[\x82RPPV[`\0` \x82\x01\x90Pa-\xCA`\0\x83\x01\x84a-\xA6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a.\x10Wa.\x0Fa-\xD0V[[PV[`\0\x81\x90Pa.!\x82a-\xFFV[\x91\x90PV[`\0a.1\x82a.\x13V[\x90P\x91\x90PV[a.A\x81a.&V[\x82RPPV[`\0` \x82\x01\x90Pa.\\`\0\x83\x01\x84a.8V[\x92\x91PPV[a.k\x81a-\x92V[\x81\x14a.vW`\0\x80\xFD[PV[`\0\x815\x90Pa.\x88\x81a.bV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a.\xA4Wa.\xA3a!\xE9V[[`\0a.\xB2\x84\x82\x85\x01a.yV[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a.\xD2Wa.\xD1a!\xE9V[[`\0a.\xE0\x85\x82\x86\x01a\"\x14V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x01Wa/\0a!\xEEV[[a/\r\x85\x82\x86\x01a*\x07V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/.Wa/-a!\xE9V[[`\0a/<\x85\x82\x86\x01a.yV[\x92PP` a/M\x85\x82\x86\x01a\"\x14V[\x91PP\x92P\x92\x90PV[a/`\x81a\"VV[\x81\x14a/kW`\0\x80\xFD[PV[`\0\x81Q\x90Pa/}\x81a/WV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a/\x99Wa/\x98a!\xE9V[[`\0a/\xA7\x84\x82\x85\x01a/nV[\x91PP\x92\x91PPV[a/\xB9\x81a#1V[\x82RPPV[`\0`@\x82\x01\x90Pa/\xD4`\0\x83\x01\x85a/\xB0V[a/\xE1` \x83\x01\x84a/\xB0V[\x93\x92PPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCannot remove self as admin.  Ha`\0\x82\x01R\x7Fve the new admin do it.\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a0U`7\x83a/\xE8V[\x91Pa0`\x82a/\xF9V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra0\x84\x81a0HV[\x90P\x91\x90PV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a0\xE7`/\x83a/\xE8V[\x91Pa0\xF2\x82a0\x8BV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra1\x16\x81a0\xDAV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a1Ba1=a18\x84a1\x1DV[a$iV[a!\xF3V[\x90P\x91\x90PV[a1R\x81a1'V[\x82RPPV[`\0`@\x82\x01\x90Pa1m`\0\x83\x01\x85a+\xAFV[a1z` \x83\x01\x84a1IV[\x93\x92PPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a1\xB6\x81a!\xF3V[\x82RPPV[`\0a1\xC8\x83\x83a1\xADV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a1\xEC\x82a1\x81V[a1\xF6\x81\x85a1\x8CV[\x93Pa2\x01\x83a1\x9DV[\x80`\0[\x83\x81\x10\x15a22W\x81Qa2\x19\x88\x82a1\xBCV[\x97Pa2$\x83a1\xD4V[\x92PP`\x01\x81\x01\x90Pa2\x05V[P\x85\x93PPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a2\x87\x82a,\xA8V[a2\x91\x81\x85a2kV[\x93Pa2\xA1\x81\x85` \x86\x01a,\xC4V[a2\xAA\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0a2\xC1\x83\x83a2|V[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a2\xE1\x82a2?V[a2\xEB\x81\x85a2JV[\x93P\x83` \x82\x02\x85\x01a2\xFD\x85a2[V[\x80`\0[\x85\x81\x10\x15a39W\x84\x84\x03\x89R\x81Qa3\x1A\x85\x82a2\xB5V[\x94Pa3%\x83a2\xC9V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa3\x01V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a3\x93\x82a1\x81V[a3\x9D\x81\x85a3wV[\x93Pa3\xA8\x83a1\x9DV[\x80`\0[\x83\x81\x10\x15a3\xD9W\x81Qa3\xC0\x88\x82a1\xBCV[\x97Pa3\xCB\x83a1\xD4V[\x92PP`\x01\x81\x01\x90Pa3\xACV[P\x85\x93PPPP\x92\x91PPV[`\0a3\xF2\x83\x83a3\x88V[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a4\x12\x82a3KV[a4\x1C\x81\x85a3VV[\x93P\x83` \x82\x02\x85\x01a4.\x85a3gV[\x80`\0[\x85\x81\x10\x15a4jW\x84\x84\x03\x89R\x81Qa4K\x85\x82a3\xE6V[\x94Pa4V\x83a3\xFAV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa42V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a4\xCF\x82a4\xA8V[a4\xD9\x81\x85a4\xB3V[\x93Pa4\xE9\x81\x85` \x86\x01a,\xC4V[a4\xF2\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0a5\t\x83\x83a4\xC4V[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a5)\x82a4|V[a53\x81\x85a4\x87V[\x93P\x83` \x82\x02\x85\x01a5E\x85a4\x98V[\x80`\0[\x85\x81\x10\x15a5\x81W\x84\x84\x03\x89R\x81Qa5b\x85\x82a4\xFDV[\x94Pa5m\x83a5\x11V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa5IV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0a\x01\0\x82\x01\x90Pa5\xA9`\0\x83\x01\x8Ba1IV[\x81\x81\x03` \x83\x01Ra5\xBB\x81\x8Aa1\xE1V[\x90P\x81\x81\x03`@\x83\x01Ra5\xCF\x81\x89a2\xD6V[\x90P\x81\x81\x03``\x83\x01Ra5\xE3\x81\x88a2\xD6V[\x90P\x81\x81\x03`\x80\x83\x01Ra5\xF7\x81\x87a4\x07V[\x90P\x81\x81\x03`\xA0\x83\x01Ra6\x0B\x81\x86a5\x1EV[\x90Pa6\x1A`\xC0\x83\x01\x85a\"bV[a6'`\xE0\x83\x01\x84a\"bV[\x99\x98PPPPPPPPPV[`\0\x81Q\x90Pa6C\x81a!\xFDV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a6_Wa6^a!\xE9V[[`\0a6m\x84\x82\x85\x01a64V[\x91PP\x92\x91PPV[`\0`\x80\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra6\x90\x81\x87a,\xEEV[\x90P\x81\x81\x03` \x83\x01Ra6\xA4\x81\x86a,\xEEV[\x90Pa6\xB3`@\x83\x01\x85a+\xAFV[a6\xC0``\x83\x01\x84a+\xAFV[\x95\x94PPPPPV[\x7FDomainWalletRegistry: metadata n`\0\x82\x01R\x7Fame and url must be set in metad` \x82\x01R\x7Fata\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0a7K`C\x83a/\xE8V[\x91Pa7V\x82a6\xC9V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra7z\x81a7>V[\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa7\x96`\0\x83\x01\x85a+\xAFV[\x81\x81\x03` \x83\x01Ra7\xA8\x81\x84a5\x1EV[\x90P\x93\x92PPPV[`\0a7\xC4a7\xBF\x84a%yV[a%^V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a7\xE0Wa7\xDFa$\xE8V[[a7\xEB\x84\x82\x85a,\xC4V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a8\x08Wa8\x07a$\xE3V[[\x81Qa8\x18\x84\x82` \x86\x01a7\xB1V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a87Wa86a!\xE9V[[`\0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8UWa8Ta!\xEEV[[a8a\x84\x82\x85\x01a7\xF3V[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa8y\x81a.bV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8\x95Wa8\x94a!\xE9V[[`\0a8\xA3\x84\x82\x85\x01a8jV[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa8\xC1`\0\x83\x01\x85a-\xA6V[a8\xCE` \x83\x01\x84a+\xAFV[\x93\x92PPPV[`\0\x81Q\x90Pa8\xE4\x81a#\xA6V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a9\0Wa8\xFFa!\xE9V[[`\0a9\x0E\x84\x82\x85\x01a8\xD5V[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa9,`\0\x83\x01\x85a#\xFFV[a99` \x83\x01\x84a.8V[\x93\x92PPPV[`\0\x81Q\x90Pa9O\x81a#CV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a9kWa9ja!\xE9V[[`\0a9y\x84\x82\x85\x01a9@V[\x91PP\x92\x91PPV[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a9\xC3`\x17\x83a9\x82V[\x91Pa9\xCE\x82a9\x8DV[`\x17\x82\x01\x90P\x91\x90PV[`\0a9\xE4\x82a4\xA8V[a9\xEE\x81\x85a9\x82V[\x93Pa9\xFE\x81\x85` \x86\x01a,\xC4V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a:@`\x11\x83a9\x82V[\x91Pa:K\x82a:\nV[`\x11\x82\x01\x90P\x91\x90PV[`\0a:a\x82a9\xB6V[\x91Pa:m\x82\x85a9\xD9V[\x91Pa:x\x82a:3V[\x91Pa:\x84\x82\x84a9\xD9V[\x91P\x81\x90P\x93\x92PPPV[`\0a:\x9B\x82a4\xA8V[a:\xA5\x81\x85a/\xE8V[\x93Pa:\xB5\x81\x85` \x86\x01a,\xC4V[a:\xBE\x81a$\xEDV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra:\xE3\x81\x84a:\x90V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a;%\x82a!\xF3V[\x91Pa;0\x83a!\xF3V[\x92P\x82\x82\x02a;>\x81a!\xF3V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a;UWa;Ta:\xEBV[[P\x92\x91PPV[`\0a;g\x82a!\xF3V[\x91Pa;r\x83a!\xF3V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a;\x8AWa;\x89a:\xEBV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a;\xCA\x82a!\xF3V[\x91P`\0\x82\x03a;\xDDWa;\xDCa:\xEBV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a<\x1E` \x83a/\xE8V[\x91Pa<)\x82a;\xE8V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra<M\x81a<\x11V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 e\xBC\xD6\x15P{K\xB4SE\xED`L\x0E.C\xD4\xF8\xD1\xC8G\xD4\xE6Tk<\x86\x8D\xE4\xB9\xE0\x97dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static DOMAINWALLETREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DomainWalletRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DomainWalletRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DomainWalletRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DomainWalletRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DomainWalletRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DomainWalletRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DomainWalletRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DOMAINWALLETREGISTRY_ABI.clone(),
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
                DOMAINWALLETREGISTRY_ABI.clone(),
                DOMAINWALLETREGISTRY_BYTECODE.clone().into(),
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
        ///Calls the contract's `registerDomain` (0x5b045724) function
        pub fn register_domain(
            &self,
            user_id: ::ethers::core::types::Bytes,
            uri: ::ethers::core::types::Bytes,
            ttl: ::ethers::core::types::U256,
            pkp_token_id: ::ethers::core::types::U256,
            nft_metadata: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [91, 4, 87, 36],
                    (user_id, uri, ttl, pkp_token_id, nft_metadata),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerDomainAndMintNext` (0x5530adf8) function
        pub fn register_domain_and_mint_next(
            &self,
            user_id: ::ethers::core::types::Bytes,
            uri: ::ethers::core::types::Bytes,
            ttl: ::ethers::core::types::U256,
            permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
            permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
            permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
            permitted_auth_method_scopes: ::std::vec::Vec<
                ::std::vec::Vec<::ethers::core::types::U256>,
            >,
            nft_metadata: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [85, 48, 173, 248],
                    (
                        user_id,
                        uri,
                        ttl,
                        permitted_auth_method_types,
                        permitted_auth_method_ids,
                        permitted_auth_method_pubkeys,
                        permitted_auth_method_scopes,
                        nft_metadata,
                    ),
                )
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
        ///Calls the contract's `setPKPMetadata` (0xb6e7f4d2) function
        pub fn set_pkp_metadata(
            &self,
            pkp_token_id: ::ethers::core::types::U256,
            nft_metadata: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 231, 244, 210], (pkp_token_id, nft_metadata))
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
            DomainWalletRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DomainWalletRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidNftMetadataCollectionLength` with signature `InvalidNftMetadataCollectionLength(uint256,uint256)` and selector `0x216aeb3f`
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
        name = "InvalidNftMetadataCollectionLength",
        abi = "InvalidNftMetadataCollectionLength(uint256,uint256)"
    )]
    pub struct InvalidNftMetadataCollectionLength {
        pub metadata_count: ::ethers::core::types::U256,
        pub valid_metadata_count: ::ethers::core::types::U256,
    }
    ///Custom Error type `NonAdminCaller` with signature `NonAdminCaller(address,address)` and selector `0x34a54232`
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
    #[etherror(name = "NonAdminCaller", abi = "NonAdminCaller(address,address)")]
    pub struct NonAdminCaller {
        pub admin_address: ::ethers::core::types::Address,
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
    pub enum DomainWalletRegistryErrors {
        InvalidNftMetadataCollectionLength(InvalidNftMetadataCollectionLength),
        NonAdminCaller(NonAdminCaller),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DomainWalletRegistryErrors {
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
                = <InvalidNftMetadataCollectionLength as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidNftMetadataCollectionLength(decoded));
            }
            if let Ok(decoded)
                = <NonAdminCaller as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonAdminCaller(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DomainWalletRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidNftMetadataCollectionLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonAdminCaller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DomainWalletRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidNftMetadataCollectionLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NonAdminCaller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DomainWalletRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidNftMetadataCollectionLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NonAdminCaller(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DomainWalletRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidNftMetadataCollectionLength>
    for DomainWalletRegistryErrors {
        fn from(value: InvalidNftMetadataCollectionLength) -> Self {
            Self::InvalidNftMetadataCollectionLength(value)
        }
    }
    impl ::core::convert::From<NonAdminCaller> for DomainWalletRegistryErrors {
        fn from(value: NonAdminCaller) -> Self {
            Self::NonAdminCaller(value)
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
    pub enum DomainWalletRegistryEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DomainWalletRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(DomainWalletRegistryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(DomainWalletRegistryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(DomainWalletRegistryEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DomainWalletRegistryEvents {
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
    impl ::core::convert::From<RoleAdminChangedFilter> for DomainWalletRegistryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for DomainWalletRegistryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for DomainWalletRegistryEvents {
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
    ///Container type for all input parameters for the `registerDomain` function with signature `registerDomain(bytes,bytes,uint256,uint256,string[])` and selector `0x5b045724`
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
        abi = "registerDomain(bytes,bytes,uint256,uint256,string[])"
    )]
    pub struct RegisterDomainCall {
        pub user_id: ::ethers::core::types::Bytes,
        pub uri: ::ethers::core::types::Bytes,
        pub ttl: ::ethers::core::types::U256,
        pub pkp_token_id: ::ethers::core::types::U256,
        pub nft_metadata: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `registerDomainAndMintNext` function with signature `registerDomainAndMintNext(bytes,bytes,uint256,uint256[],bytes[],bytes[],uint256[][],string[])` and selector `0x5530adf8`
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
        name = "registerDomainAndMintNext",
        abi = "registerDomainAndMintNext(bytes,bytes,uint256,uint256[],bytes[],bytes[],uint256[][],string[])"
    )]
    pub struct RegisterDomainAndMintNextCall {
        pub user_id: ::ethers::core::types::Bytes,
        pub uri: ::ethers::core::types::Bytes,
        pub ttl: ::ethers::core::types::U256,
        pub permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub nft_metadata: ::std::vec::Vec<::std::string::String>,
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
    ///Container type for all input parameters for the `setPKPMetadata` function with signature `setPKPMetadata(uint256,string[])` and selector `0xb6e7f4d2`
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
    #[ethcall(name = "setPKPMetadata", abi = "setPKPMetadata(uint256,string[])")]
    pub struct SetPKPMetadataCall {
        pub pkp_token_id: ::ethers::core::types::U256,
        pub nft_metadata: ::std::vec::Vec<::std::string::String>,
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
    pub enum DomainWalletRegistryCalls {
        AdminRole(AdminRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        AddAdmin(AddAdminCall),
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        GetDomainIdByTokenId(GetDomainIdByTokenIdCall),
        GetDomainTokenIdByUri(GetDomainTokenIdByUriCall),
        GetDomainUri(GetDomainUriCall),
        GetExpiration(GetExpirationCall),
        GetPkpTokenId(GetPkpTokenIdCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasExpired(HasExpiredCall),
        HasOwner(HasOwnerCall),
        HasRole(HasRoleCall),
        IsOwner(IsOwnerCall),
        IsRouted(IsRoutedCall),
        RegisterDomain(RegisterDomainCall),
        RegisterDomainAndMintNext(RegisterDomainAndMintNextCall),
        RegisterPKP(RegisterPKPCall),
        RemoveAdmin(RemoveAdminCall),
        RemoveDomain(RemoveDomainCall),
        RenounceRole(RenounceRoleCall),
        RevokeDomain(RevokeDomainCall),
        RevokeRole(RevokeRoleCall),
        SetPKPMetadata(SetPKPMetadataCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for DomainWalletRegistryCalls {
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
                = <RegisterDomainAndMintNextCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RegisterDomainAndMintNext(decoded));
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
                = <SetPKPMetadataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPKPMetadata(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DomainWalletRegistryCalls {
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
                Self::RegisterDomainAndMintNext(element) => {
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
                Self::SetPKPMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DomainWalletRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsRouted(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterDomainAndMintNext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPKPMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for DomainWalletRegistryCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for DomainWalletRegistryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AddAdminCall> for DomainWalletRegistryCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for DomainWalletRegistryCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for DomainWalletRegistryCalls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<GetDomainIdByTokenIdCall> for DomainWalletRegistryCalls {
        fn from(value: GetDomainIdByTokenIdCall) -> Self {
            Self::GetDomainIdByTokenId(value)
        }
    }
    impl ::core::convert::From<GetDomainTokenIdByUriCall> for DomainWalletRegistryCalls {
        fn from(value: GetDomainTokenIdByUriCall) -> Self {
            Self::GetDomainTokenIdByUri(value)
        }
    }
    impl ::core::convert::From<GetDomainUriCall> for DomainWalletRegistryCalls {
        fn from(value: GetDomainUriCall) -> Self {
            Self::GetDomainUri(value)
        }
    }
    impl ::core::convert::From<GetExpirationCall> for DomainWalletRegistryCalls {
        fn from(value: GetExpirationCall) -> Self {
            Self::GetExpiration(value)
        }
    }
    impl ::core::convert::From<GetPkpTokenIdCall> for DomainWalletRegistryCalls {
        fn from(value: GetPkpTokenIdCall) -> Self {
            Self::GetPkpTokenId(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for DomainWalletRegistryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for DomainWalletRegistryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasExpiredCall> for DomainWalletRegistryCalls {
        fn from(value: HasExpiredCall) -> Self {
            Self::HasExpired(value)
        }
    }
    impl ::core::convert::From<HasOwnerCall> for DomainWalletRegistryCalls {
        fn from(value: HasOwnerCall) -> Self {
            Self::HasOwner(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for DomainWalletRegistryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for DomainWalletRegistryCalls {
        fn from(value: IsOwnerCall) -> Self {
            Self::IsOwner(value)
        }
    }
    impl ::core::convert::From<IsRoutedCall> for DomainWalletRegistryCalls {
        fn from(value: IsRoutedCall) -> Self {
            Self::IsRouted(value)
        }
    }
    impl ::core::convert::From<RegisterDomainCall> for DomainWalletRegistryCalls {
        fn from(value: RegisterDomainCall) -> Self {
            Self::RegisterDomain(value)
        }
    }
    impl ::core::convert::From<RegisterDomainAndMintNextCall>
    for DomainWalletRegistryCalls {
        fn from(value: RegisterDomainAndMintNextCall) -> Self {
            Self::RegisterDomainAndMintNext(value)
        }
    }
    impl ::core::convert::From<RegisterPKPCall> for DomainWalletRegistryCalls {
        fn from(value: RegisterPKPCall) -> Self {
            Self::RegisterPKP(value)
        }
    }
    impl ::core::convert::From<RemoveAdminCall> for DomainWalletRegistryCalls {
        fn from(value: RemoveAdminCall) -> Self {
            Self::RemoveAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveDomainCall> for DomainWalletRegistryCalls {
        fn from(value: RemoveDomainCall) -> Self {
            Self::RemoveDomain(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for DomainWalletRegistryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeDomainCall> for DomainWalletRegistryCalls {
        fn from(value: RevokeDomainCall) -> Self {
            Self::RevokeDomain(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for DomainWalletRegistryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetPKPMetadataCall> for DomainWalletRegistryCalls {
        fn from(value: SetPKPMetadataCall) -> Self {
            Self::SetPKPMetadata(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for DomainWalletRegistryCalls {
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
    ///Container type for all return fields from the `registerDomain` function with signature `registerDomain(bytes,bytes,uint256,uint256,string[])` and selector `0x5b045724`
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
    pub struct RegisterDomainReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `registerDomainAndMintNext` function with signature `registerDomainAndMintNext(bytes,bytes,uint256,uint256[],bytes[],bytes[],uint256[][],string[])` and selector `0x5530adf8`
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
    pub struct RegisterDomainAndMintNextReturn(pub ::ethers::core::types::U256);
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
}
