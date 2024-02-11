pub use pkpnft_metadata::*;
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
pub mod pkpnft_metadata {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/PKPNFTMetadata.json",
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
                    ::std::borrow::ToOwned::to_owned("WRITER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WRITER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("bytesToHex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bytesToHex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("buffer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("removeProfileForPkp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeProfileForPkp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("removeUrlForPKP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeUrlForPKP"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("setPKPHelperWriterAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPKPHelperWriterAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "pkpHelperWriterAddress",
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
                    ::std::borrow::ToOwned::to_owned("setProfileForPKP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setProfileForPKP"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imgUrl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("setUrlForPKP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUrlForPKP"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("url"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ethAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
    pub static PKPNFTMETADATA_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\x006|8\x03\x80b\x006|\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x01?V[\x81`\x02`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x02`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x02\x81\x11\x15b\0\0\xA0Wb\0\0\x9Fb\0\x01\x86V[[\x02\x17\x90UPPPb\0\x01\xB5V[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\0\xDF\x82b\0\0\xB2V[\x90P\x91\x90PV[b\0\0\xF1\x81b\0\0\xD2V[\x81\x14b\0\0\xFDW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x01\x11\x81b\0\0\xE6V[\x92\x91PPV[`\x03\x81\x10b\0\x01%W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x019\x81b\0\x01\x17V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01YWb\0\x01Xb\0\0\xADV[[`\0b\0\x01i\x85\x82\x86\x01b\0\x01\0V[\x92PP` b\0\x01|\x85\x82\x86\x01b\0\x01(V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[a4\xB7\x80b\0\x01\xC5`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80c\x85^\xEC\"\x11a\0\xA2W\x80c\x9B\xEA\xAB{\x11a\0qW\x80c\x9B\xEA\xAB{\x14a\x02\xEFW\x80c\x9D\xCA\x002\x14a\x03\rW\x80c\xA2\x17\xFD\xDF\x14a\x03+W\x80c\xB6:vw\x14a\x03IW\x80c\xD5Gt\x1F\x14a\x03eWa\x01\x16V[\x80c\x85^\xEC\"\x14a\x02WW\x80c\x90\0\xFE\xE1\x14a\x02sW\x80c\x91\xD1HT\x14a\x02\x8FW\x80c\x95\x04b\xEE\x14a\x02\xBFWa\x01\x16V[\x80c6V\x8A\xBE\x11a\0\xE9W\x80c6V\x8A\xBE\x14a\x01\xB3W\x80cE\x1D\x89\xFA\x14a\x01\xCFW\x80cP\xD1{^\x14a\x01\xFFW\x80cQ\x9A!\x8E\x14a\x02\x1DW\x80cu\xB28\xFC\x14a\x029Wa\x01\x16V[\x80c\x01\xFF\xC9\xA7\x14a\x01\x1BW\x80c\x0F\xA8\xAE/\x14a\x01KW\x80c$\x8A\x9C\xA3\x14a\x01gW\x80c//\xF1]\x14a\x01\x97W[`\0\x80\xFD[a\x015`\x04\x806\x03\x81\x01\x90a\x010\x91\x90a\x1B\xAAV[a\x03\x81V[`@Qa\x01B\x91\x90a\x1B\xF2V[`@Q\x80\x91\x03\x90\xF3[a\x01e`\x04\x806\x03\x81\x01\x90a\x01`\x91\x90a\x1CkV[a\x03\xFBV[\0[a\x01\x81`\x04\x806\x03\x81\x01\x90a\x01|\x91\x90a\x1C\xCEV[a\x04\x91V[`@Qa\x01\x8E\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x01\xB1`\x04\x806\x03\x81\x01\x90a\x01\xAC\x91\x90a\x1D%V[a\x04\xB0V[\0[a\x01\xCD`\x04\x806\x03\x81\x01\x90a\x01\xC8\x91\x90a\x1D%V[a\x04\xD1V[\0[a\x01\xE9`\x04\x806\x03\x81\x01\x90a\x01\xE4\x91\x90a\x1E\xABV[a\x05TV[`@Qa\x01\xF6\x91\x90a\x1FsV[`@Q\x80\x91\x03\x90\xF3[a\x02\x07a\x07yV[`@Qa\x02\x14\x91\x90a\x1F\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x027`\x04\x806\x03\x81\x01\x90a\x022\x91\x90a EV[a\x07\x9FV[\0[a\x02Aa\t|V[`@Qa\x02N\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x02q`\x04\x806\x03\x81\x01\x90a\x02l\x91\x90a!\x13V[a\t\xA0V[\0[a\x02\x8D`\x04\x806\x03\x81\x01\x90a\x02\x88\x91\x90a!\x13V[a\x0BoV[\0[a\x02\xA9`\x04\x806\x03\x81\x01\x90a\x02\xA4\x91\x90a\x1D%V[a\r>V[`@Qa\x02\xB6\x91\x90a\x1B\xF2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD9`\x04\x806\x03\x81\x01\x90a\x02\xD4\x91\x90a!oV[a\r\xA8V[`@Qa\x02\xE6\x91\x90a\x1FsV[`@Q\x80\x91\x03\x90\xF3[a\x02\xF7a\r\xE3V[`@Qa\x03\x04\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x03\x15a\x0E\x07V[`@Qa\x03\"\x91\x90a\"UV[`@Q\x80\x91\x03\x90\xF3[a\x033a\x0E\x1AV[`@Qa\x03@\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x03c`\x04\x806\x03\x81\x01\x90a\x03^\x91\x90a EV[a\x0E!V[\0[a\x03\x7F`\x04\x806\x03\x81\x01\x90a\x03z\x91\x90a\x1D%V[a\x0F\xFEV[\0[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03\xF4WPa\x03\xF3\x82a\x10\x1FV[[\x90P\x91\x90PV[a\x04%\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\r>V[a\x04dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04[\x90a\"\xE2V[`@Q\x80\x91\x03\x90\xFD[a\x04\x8E\x7Fs\xA9\x98S\x16\xCDL\xBF\xD1=\xAD\xCA\xA0\xE6\xF7s\xC8^\x93:\r\x88\xEF\xBE`\xE4\xDCI\xDA\x91v\xA0\x82a\x10\x89V[PV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x04\xB9\x82a\x04\x91V[a\x04\xC2\x81a\x11iV[a\x04\xCC\x83\x83a\x10\x89V[PPPV[a\x04\xD9a\x11}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05=\x90a#tV[`@Q\x80\x91\x03\x90\xFD[a\x05P\x82\x82a\x11\x85V[PPV[```\0`\x02\x83Qa\x05f\x91\x90a#\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x7FWa\x05~a\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05\xB1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x90P`\0[\x84Q\x81\x10\x15a\x07OW\x81\x82Q\x86\x83\x81Q\x81\x10a\x06\x10Wa\x06\x0Fa$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16a\x06+\x91\x90a$cV[\x81Q\x81\x10a\x06<Wa\x06;a$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x02\x83a\x06U\x91\x90a#\xC3V[\x81Q\x81\x10a\x06fWa\x06ea$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x81\x82Q\x86\x83\x81Q\x81\x10a\x06\xABWa\x06\xAAa$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16a\x06\xC6\x91\x90a$\x94V[\x81Q\x81\x10a\x06\xD7Wa\x06\xD6a$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x01`\x02\x84a\x06\xF2\x91\x90a#\xC3V[a\x06\xFC\x91\x90a$\xC5V[\x81Q\x81\x10a\x07\rWa\x07\x0Ca$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80\x80a\x07G\x90a$\xF9V[\x91PPa\x05\xF1V[P\x81`@Q` \x01a\x07a\x91\x90a%\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08n\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x9B\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDC\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t@\x90a'FV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80` \x01`@R\x80`\0\x81RP`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\tx\x91\x90a)hV[PPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\no\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x9C\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDD\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BJW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BA\x90a'FV[`@Q\x80\x91\x03\x90\xFD[\x80`\x03`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\x0Bj\x91\x90a)hV[PPPV[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C>\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ck\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAC\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x10\x90a'FV[`@Q\x80\x91\x03\x90\xFD[\x80`\x04`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\r9\x91\x90a)hV[PPPV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[```\0a\r\xB7\x85\x85\x85a\x12fV[\x90P\x80`@Q` \x01a\r\xCA\x91\x90a*\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[\x7Fs\xA9\x98S\x16\xCDL\xBF\xD1=\xAD\xCA\xA0\xE6\xF7s\xC8^\x93:\r\x88\xEF\xBE`\xE4\xDCI\xDA\x91v\xA0\x81V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80\x1B\x81V[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF0\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x1D\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F^\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xC2\x90a'FV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80` \x01`@R\x80`\0\x81RP`\x04`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\x0F\xFA\x91\x90a)hV[PPV[a\x10\x07\x82a\x04\x91V[a\x10\x10\x81a\x11iV[a\x10\x1A\x83\x83a\x11\x85V[PPPV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a\x10\x93\x82\x82a\r>V[a\x11eW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x11\na\x11}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a\x11z\x81a\x11ua\x11}V[a\x14\xCCV[PV[`\x003\x90P\x90V[a\x11\x8F\x82\x82a\r>V[\x15a\x12bW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x12\x07a\x11}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[```\0`@Q\x80a\x04\x80\x01`@R\x80a\x04V\x81R` \x01a/\xECa\x04V\x919\x90P`\0a\x12\x93\x85a\x05TV[\x90P`\0a\x12\xA0\x85a\x15QV[\x90P`\0a\x12\xAD\x88a\x15~V[\x90P`\0`\x03`\0\x8A\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\x12\xCF\x90a'\x95V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xFB\x90a'\x95V[\x80\x15a\x13HW\x80`\x1F\x10a\x13\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13HV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13+W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0`\x04`\0\x8B\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\x13o\x90a'\x95V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x9B\x90a'\x95V[\x80\x15a\x13\xE8W\x80`\x1F\x10a\x13\xBDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xE8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xCBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0\x82Q\x14\x80\x15a\x14\x02WP`\0\x81Q\x14\x15[\x15a\x14.W\x82`@Q` \x01a\x14\x18\x91\x90a*\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa\x14\x8DV[`\0\x82Q\x14\x15\x80\x15a\x14AWP`\0\x81Q\x14[\x15a\x14NW\x85\x90Pa\x14\x8CV[`\0\x82Q\x14\x80\x15a\x14`WP`\0\x81Q\x14[\x15a\x14\x8BW\x82`@Q` \x01a\x14v\x91\x90a*\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x85\x90P[[[a\x14\xBD\x82\x82\x87\x87\x87`@Q` \x01a\x14\xA9\x95\x94\x93\x92\x91\x90a-\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x16LV[\x96PPPPPPP\x93\x92PPPV[a\x14\xD6\x82\x82a\r>V[a\x15MWa\x14\xE3\x81a\x15QV[a\x14\xF1\x83`\0\x1C` a\x17\xAFV[`@Q` \x01a\x15\x02\x92\x91\x90a/\x1CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15D\x91\x90a\x1FsV[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x15w\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x17\xAFV[\x90P\x91\x90PV[```\0`\x01a\x15\x8D\x84a\x19\xEBV[\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xACWa\x15\xABa\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15\xDEW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0\x82` \x01\x82\x01\x90P[`\x01\x15a\x16AW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x165Wa\x164a$4V[[\x04\x94P`\0\x85\x03a\x15\xECW[\x81\x93PPPP\x91\x90PV[```\0\x82Q\x03a\x16nW`@Q\x80` \x01`@R\x80`\0\x81RP\x90Pa\x17\xAAV[`\0`@Q\x80``\x01`@R\x80`@\x81R` \x01a4B`@\x919\x90P`\0`\x03`\x02\x85Qa\x16\x9D\x91\x90a$\xC5V[a\x16\xA7\x91\x90a$cV[`\x04a\x16\xB3\x91\x90a#\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xCCWa\x16\xCBa\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x16\xFEW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\x01\x82\x01` \x82\x01\x85\x86Q\x87\x01[\x80\x82\x10\x15a\x17jW`\x03\x82\x01\x91P\x81Q`?\x81`\x12\x1C\x16\x85\x01Q\x84S`\x01\x84\x01\x93P`?\x81`\x0C\x1C\x16\x85\x01Q\x84S`\x01\x84\x01\x93P`?\x81`\x06\x1C\x16\x85\x01Q\x84S`\x01\x84\x01\x93P`?\x81\x16\x85\x01Q\x84S`\x01\x84\x01\x93PPa\x17\x0FV[PP`\x03\x86Q\x06`\x01\x81\x14a\x17\x86W`\x02\x81\x14a\x17\x99Wa\x17\xA1V[`=`\x01\x83\x03S`=`\x02\x83\x03Sa\x17\xA1V[`=`\x01\x83\x03S[PPP\x80\x92PPP[\x91\x90PV[```\0`\x02\x83`\x02a\x17\xC2\x91\x90a#\xC3V[a\x17\xCC\x91\x90a$\xC5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xE5Wa\x17\xE4a\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\x17W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x18OWa\x18Na$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x18\xB3Wa\x18\xB2a$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\x18\xF3\x91\x90a#\xC3V[a\x18\xFD\x91\x90a$\xC5V[\x90P[`\x01\x81\x11\x15a\x19\x9DW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\x19?Wa\x19>a$\x05V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x19VWa\x19Ua$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x19\x96\x90a/VV[\x90Pa\x19\0V[P`\0\x84\x14a\x19\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xD8\x90a/\xCBV[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0\x80`\0\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x1AIWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a\x1A?Wa\x1A>a$4V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1A\x86Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a\x1A|Wa\x1A{a$4V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a\x1A\xB5Wf#\x86\xF2o\xC1\0\0\x83\x81a\x1A\xABWa\x1A\xAAa$4V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a\x1A\xDEWc\x05\xF5\xE1\0\x83\x81a\x1A\xD4Wa\x1A\xD3a$4V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a\x1B\x03Wa'\x10\x83\x81a\x1A\xF9Wa\x1A\xF8a$4V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a\x1B&W`d\x83\x81a\x1B\x1CWa\x1B\x1Ba$4V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a\x1B5W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x1B\x87\x81a\x1BRV[\x81\x14a\x1B\x92W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\xA4\x81a\x1B~V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1B\xC0Wa\x1B\xBFa\x1BHV[[`\0a\x1B\xCE\x84\x82\x85\x01a\x1B\x95V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x1B\xEC\x81a\x1B\xD7V[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\x07`\0\x83\x01\x84a\x1B\xE3V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1C8\x82a\x1C\rV[\x90P\x91\x90PV[a\x1CH\x81a\x1C-V[\x81\x14a\x1CSW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1Ce\x81a\x1C?V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\x81Wa\x1C\x80a\x1BHV[[`\0a\x1C\x8F\x84\x82\x85\x01a\x1CVV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x1C\xAB\x81a\x1C\x98V[\x81\x14a\x1C\xB6W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1C\xC8\x81a\x1C\xA2V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\xE4Wa\x1C\xE3a\x1BHV[[`\0a\x1C\xF2\x84\x82\x85\x01a\x1C\xB9V[\x91PP\x92\x91PPV[a\x1D\x04\x81a\x1C\x98V[\x82RPPV[`\0` \x82\x01\x90Pa\x1D\x1F`\0\x83\x01\x84a\x1C\xFBV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D<Wa\x1D;a\x1BHV[[`\0a\x1DJ\x85\x82\x86\x01a\x1C\xB9V[\x92PP` a\x1D[\x85\x82\x86\x01a\x1CVV[\x91PP\x92P\x92\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x1D\xB8\x82a\x1DoV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\xD7Wa\x1D\xD6a\x1D\x80V[[\x80`@RPPPV[`\0a\x1D\xEAa\x1B>V[\x90Pa\x1D\xF6\x82\x82a\x1D\xAFV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1E\x16Wa\x1E\x15a\x1D\x80V[[a\x1E\x1F\x82a\x1DoV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x1ENa\x1EI\x84a\x1D\xFBV[a\x1D\xE0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x1EjWa\x1Eia\x1DjV[[a\x1Eu\x84\x82\x85a\x1E,V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x1E\x92Wa\x1E\x91a\x1DeV[[\x815a\x1E\xA2\x84\x82` \x86\x01a\x1E;V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1E\xC1Wa\x1E\xC0a\x1BHV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xDFWa\x1E\xDEa\x1BMV[[a\x1E\xEB\x84\x82\x85\x01a\x1E}V[\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1F.W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1F\x13V[`\0\x84\x84\x01RPPPPV[`\0a\x1FE\x82a\x1E\xF4V[a\x1FO\x81\x85a\x1E\xFFV[\x93Pa\x1F_\x81\x85` \x86\x01a\x1F\x10V[a\x1Fh\x81a\x1DoV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1F\x8D\x81\x84a\x1F:V[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x1F\xBAa\x1F\xB5a\x1F\xB0\x84a\x1C\rV[a\x1F\x95V[a\x1C\rV[\x90P\x91\x90PV[`\0a\x1F\xCC\x82a\x1F\x9FV[\x90P\x91\x90PV[`\0a\x1F\xDE\x82a\x1F\xC1V[\x90P\x91\x90PV[a\x1F\xEE\x81a\x1F\xD3V[\x82RPPV[`\0` \x82\x01\x90Pa \t`\0\x83\x01\x84a\x1F\xE5V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a \"\x81a \x0FV[\x81\x14a -W`\0\x80\xFD[PV[`\0\x815\x90Pa ?\x81a \x19V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a [Wa Za\x1BHV[[`\0a i\x84\x82\x85\x01a 0V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a \x8DWa \x8Ca\x1D\x80V[[a \x96\x82a\x1DoV[\x90P` \x81\x01\x90P\x91\x90PV[`\0a \xB6a \xB1\x84a rV[a\x1D\xE0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a \xD2Wa \xD1a\x1DjV[[a \xDD\x84\x82\x85a\x1E,V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a \xFAWa \xF9a\x1DeV[[\x815a!\n\x84\x82` \x86\x01a \xA3V[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a!*Wa!)a\x1BHV[[`\0a!8\x85\x82\x86\x01a 0V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!YWa!Xa\x1BMV[[a!e\x85\x82\x86\x01a \xE5V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x88Wa!\x87a\x1BHV[[`\0a!\x96\x86\x82\x87\x01a 0V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xB7Wa!\xB6a\x1BMV[[a!\xC3\x86\x82\x87\x01a\x1E}V[\x92PP`@a!\xD4\x86\x82\x87\x01a\x1CVV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\"\x1EWa\"\x1Da!\xDEV[[PV[`\0\x81\x90Pa\"/\x82a\"\rV[\x91\x90PV[`\0a\"?\x82a\"!V[\x90P\x91\x90PV[a\"O\x81a\"4V[\x82RPPV[`\0` \x82\x01\x90Pa\"j`\0\x83\x01\x84a\"FV[\x92\x91PPV[\x7FPKPNFTMetadata: must had admin r`\0\x82\x01R\x7Fole\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\"\xCC`#\x83a\x1E\xFFV[\x91Pa\"\xD7\x82a\"pV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\"\xFB\x81a\"\xBFV[\x90P\x91\x90PV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a#^`/\x83a\x1E\xFFV[\x91Pa#i\x82a#\x02V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra#\x8D\x81a#QV[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a#\xCE\x82a \x0FV[\x91Pa#\xD9\x83a \x0FV[\x92P\x82\x82\x02a#\xE7\x81a \x0FV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a#\xFEWa#\xFDa#\x94V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a$n\x82a \x0FV[\x91Pa$y\x83a \x0FV[\x92P\x82a$\x89Wa$\x88a$4V[[\x82\x82\x04\x90P\x92\x91PPV[`\0a$\x9F\x82a \x0FV[\x91Pa$\xAA\x83a \x0FV[\x92P\x82a$\xBAWa$\xB9a$4V[[\x82\x82\x06\x90P\x92\x91PPV[`\0a$\xD0\x82a \x0FV[\x91Pa$\xDB\x83a \x0FV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a$\xF3Wa$\xF2a#\x94V[[\x92\x91PPV[`\0a%\x04\x82a \x0FV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a%6Wa%5a#\x94V[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[\x7F0x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a%\x82`\x02\x83a%AV[\x91Pa%\x8D\x82a%LV[`\x02\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0a%\xB9\x82a%\x98V[a%\xC3\x81\x85a%\xA3V[\x93Pa%\xD3\x81\x85` \x86\x01a\x1F\x10V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a%\xEA\x82a%uV[\x91Pa%\xF6\x82\x84a%\xAEV[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90Pa&\x10\x81a\x1C\xA2V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a&,Wa&+a\x1BHV[[`\0a&:\x84\x82\x85\x01a&\x01V[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa&X`\0\x83\x01\x85a\x1C\xFBV[a&e` \x83\x01\x84a\"FV[\x93\x92PPPV[`\0\x81Q\x90Pa&{\x81a\x1C?V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a&\x97Wa&\x96a\x1BHV[[`\0a&\xA5\x84\x82\x85\x01a&lV[\x91PP\x92\x91PPV[\x7FPKPHelper: only the Domain Walle`\0\x82\x01R\x7Ft registry is allowed to mint do` \x82\x01R\x7Fmain wallets, who are you?\0\0\0\0\0\0`@\x82\x01RPV[`\0a'0`Z\x83a\x1E\xFFV[\x91Pa';\x82a&\xAEV[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra'_\x81a'#V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a'\xADW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\xC0Wa'\xBFa'fV[[P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a((\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a'\xEBV[a(2\x86\x83a'\xEBV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0a(ea(`a([\x84a \x0FV[a\x1F\x95V[a \x0FV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a(\x7F\x83a(JV[a(\x93a(\x8B\x82a(lV[\x84\x84Ta'\xF8V[\x82UPPPPV[`\0\x90V[a(\xA8a(\x9BV[a(\xB3\x81\x84\x84a(vV[PPPV[[\x81\x81\x10\x15a(\xD7Wa(\xCC`\0\x82a(\xA0V[`\x01\x81\x01\x90Pa(\xB9V[PPV[`\x1F\x82\x11\x15a)\x1CWa(\xED\x81a'\xC6V[a(\xF6\x84a'\xDBV[\x81\x01` \x85\x10\x15a)\x05W\x81\x90P[a)\x19a)\x11\x85a'\xDBV[\x83\x01\x82a(\xB8V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a)?`\0\x19\x84`\x08\x02a)!V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a)X\x83\x83a).V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a)q\x82a\x1E\xF4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x8AWa)\x89a\x1D\x80V[[a)\x94\x82Ta'\x95V[a)\x9F\x82\x82\x85a(\xDBV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a)\xD2W`\0\x84\x15a)\xC0W\x82\x87\x01Q\x90P[a)\xCA\x85\x82a)LV[\x86UPa*2V[`\x1F\x19\x84\x16a)\xE0\x86a'\xC6V[`\0[\x82\x81\x10\x15a*\x08W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa)\xE3V[\x86\x83\x10\x15a*%W\x84\x89\x01Qa*!`\x1F\x89\x16\x82a).V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7Fdata:application/json;base64,\0\0\0`\0\x82\x01RPV[`\0a*p`\x1D\x83a%AV[\x91Pa*{\x82a*:V[`\x1D\x82\x01\x90P\x91\x90PV[`\0a*\x91\x82a\x1E\xF4V[a*\x9B\x81\x85a%AV[\x93Pa*\xAB\x81\x85` \x86\x01a\x1F\x10V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a*\xC2\x82a*cV[\x91Pa*\xCE\x82\x84a*\x86V[\x91P\x81\x90P\x92\x91PPV[\x7FLit PKP #\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPV[`\0a+\n\x82a*\xD9V[`\t\x82\x01\x91Pa+\x1A\x82\x84a*\x86V[\x91P\x81\x90P\x92\x91PPV[\x7F{\"name\":\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a+[`\t\x83a%AV[\x91Pa+f\x82a+%V[`\t\x82\x01\x90P\x91\x90PV[\x7F\", \"description\": \"This NFT enti`\0\x82\x01R\x7Ftles the holder to use a Lit Pro` \x82\x01R\x7Ftocol PKP, and to grant access t`@\x82\x01R\x7Fo other users and Lit Actions to``\x82\x01R\x7F use this PKP\",\"image_data\": \"\0\0`\x80\x82\x01RPV[`\0a,?`\x9E\x83a%AV[\x91Pa,J\x82a+qV[`\x9E\x82\x01\x90P\x91\x90PV[\x7F\",\"attributes\": [{\"trait_type\": `\0\x82\x01R\x7F\"Public Key\", \"value\": \"\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a,\xB1`8\x83a%AV[\x91Pa,\xBC\x82a,UV[`8\x82\x01\x90P\x91\x90PV[\x7F\"}, {\"trait_type\": \"ETH Wallet A`\0\x82\x01R\x7Fddress\", \"value\": \"\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a-#`3\x83a%AV[\x91Pa-.\x82a,\xC7V[`3\x82\x01\x90P\x91\x90PV[\x7F\"}, {\"trait_type\": \"Token ID\", \"`\0\x82\x01R\x7Fvalue\": \"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a-\x95`)\x83a%AV[\x91Pa-\xA0\x82a-9V[`)\x82\x01\x90P\x91\x90PV[\x7F\"}]}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a-\xE1`\x04\x83a%AV[\x91Pa-\xEC\x82a-\xABV[`\x04\x82\x01\x90P\x91\x90PV[`\0a.\x02\x82a+NV[\x91Pa.\x0E\x82\x88a*\x86V[\x91Pa.\x19\x82a,2V[\x91Pa.%\x82\x87a%\xAEV[\x91Pa.0\x82a,\xA4V[\x91Pa.<\x82\x86a*\x86V[\x91Pa.G\x82a-\x16V[\x91Pa.S\x82\x85a*\x86V[\x91Pa.^\x82a-\x88V[\x91Pa.j\x82\x84a*\x86V[\x91Pa.u\x82a-\xD4V[\x91P\x81\x90P\x96\x95PPPPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a.\xBA`\x17\x83a%AV[\x91Pa.\xC5\x82a.\x84V[`\x17\x82\x01\x90P\x91\x90PV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a/\x06`\x11\x83a%AV[\x91Pa/\x11\x82a.\xD0V[`\x11\x82\x01\x90P\x91\x90PV[`\0a/'\x82a.\xADV[\x91Pa/3\x82\x85a*\x86V[\x91Pa/>\x82a.\xF9V[\x91Pa/J\x82\x84a*\x86V[\x91P\x81\x90P\x93\x92PPPV[`\0a/a\x82a \x0FV[\x91P`\0\x82\x03a/tWa/sa#\x94V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a/\xB5` \x83a\x1E\xFFV[\x91Pa/\xC0\x82a/\x7FV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra/\xE4\x81a/\xA8V[\x90P\x91\x90PV\xFE<svg xmlns='http://www.w3.org/2000/svg' width='1080' height='1080' fill='none' xmlns:v='https://vecta.io/nano'><path d='M363.076 392.227s-.977 18.524-36.874 78.947c-41.576 70.018-45.481 151.978-3.017 220.4 89.521 144.245 332.481 141.52 422.556.089 34.832-54.707 44.816-117.479 32.924-181.248 0 0-28.819-133.144-127.237-217.099 1.553 1.308 5.369 19.122 6.101 26.722 2.241 23.354.045 47.838-7.787 70.062-5.746 16.33-13.711 30.467-27.178 41.368 0-3.811-.954-10.635-.976-12.918-.644-46.508-18.659-89.582-48.011-125.743-25.647-31.552-60.812-53.089-97.84-68.932.931 3.191 2.662 16.419 2.906 19.033 1.908 21.958 2.263 52.713-.621 74.649s-7.832 33.878-14.554 54.441c-10.184 31.175-24.05 54.285-41.621 82.004-3.24 5.096-12.913 19.078-18.082 26.146 0 0-8.897-56.191-40.667-87.921h-.022z' fill='#000'/><path d='M562.5 27.28l410.279 236.874c13.923 8.039 22.5 22.895 22.5 38.971v473.75c0 16.076-8.577 30.932-22.5 38.971L562.5 1052.72c-13.923 8.04-31.077 8.04-45 0L107.221 815.846c-13.923-8.039-22.5-22.895-22.5-38.971v-473.75a45 45 0 0 1 22.5-38.971L517.5 27.28a45 45 0 0 1 45 0z' stroke='#000' stroke-width='24.75'/></svg>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\xA2dipfsX\"\x12 \x0C\xEB,\x1A{y|\xD2\x1A\xE5\x91\x01gvQF'Z(\xFD(\xF1{[D4`\x8B\x05gG\x08dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static PKPNFTMETADATA_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80c\x85^\xEC\"\x11a\0\xA2W\x80c\x9B\xEA\xAB{\x11a\0qW\x80c\x9B\xEA\xAB{\x14a\x02\xEFW\x80c\x9D\xCA\x002\x14a\x03\rW\x80c\xA2\x17\xFD\xDF\x14a\x03+W\x80c\xB6:vw\x14a\x03IW\x80c\xD5Gt\x1F\x14a\x03eWa\x01\x16V[\x80c\x85^\xEC\"\x14a\x02WW\x80c\x90\0\xFE\xE1\x14a\x02sW\x80c\x91\xD1HT\x14a\x02\x8FW\x80c\x95\x04b\xEE\x14a\x02\xBFWa\x01\x16V[\x80c6V\x8A\xBE\x11a\0\xE9W\x80c6V\x8A\xBE\x14a\x01\xB3W\x80cE\x1D\x89\xFA\x14a\x01\xCFW\x80cP\xD1{^\x14a\x01\xFFW\x80cQ\x9A!\x8E\x14a\x02\x1DW\x80cu\xB28\xFC\x14a\x029Wa\x01\x16V[\x80c\x01\xFF\xC9\xA7\x14a\x01\x1BW\x80c\x0F\xA8\xAE/\x14a\x01KW\x80c$\x8A\x9C\xA3\x14a\x01gW\x80c//\xF1]\x14a\x01\x97W[`\0\x80\xFD[a\x015`\x04\x806\x03\x81\x01\x90a\x010\x91\x90a\x1B\xAAV[a\x03\x81V[`@Qa\x01B\x91\x90a\x1B\xF2V[`@Q\x80\x91\x03\x90\xF3[a\x01e`\x04\x806\x03\x81\x01\x90a\x01`\x91\x90a\x1CkV[a\x03\xFBV[\0[a\x01\x81`\x04\x806\x03\x81\x01\x90a\x01|\x91\x90a\x1C\xCEV[a\x04\x91V[`@Qa\x01\x8E\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x01\xB1`\x04\x806\x03\x81\x01\x90a\x01\xAC\x91\x90a\x1D%V[a\x04\xB0V[\0[a\x01\xCD`\x04\x806\x03\x81\x01\x90a\x01\xC8\x91\x90a\x1D%V[a\x04\xD1V[\0[a\x01\xE9`\x04\x806\x03\x81\x01\x90a\x01\xE4\x91\x90a\x1E\xABV[a\x05TV[`@Qa\x01\xF6\x91\x90a\x1FsV[`@Q\x80\x91\x03\x90\xF3[a\x02\x07a\x07yV[`@Qa\x02\x14\x91\x90a\x1F\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x027`\x04\x806\x03\x81\x01\x90a\x022\x91\x90a EV[a\x07\x9FV[\0[a\x02Aa\t|V[`@Qa\x02N\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x02q`\x04\x806\x03\x81\x01\x90a\x02l\x91\x90a!\x13V[a\t\xA0V[\0[a\x02\x8D`\x04\x806\x03\x81\x01\x90a\x02\x88\x91\x90a!\x13V[a\x0BoV[\0[a\x02\xA9`\x04\x806\x03\x81\x01\x90a\x02\xA4\x91\x90a\x1D%V[a\r>V[`@Qa\x02\xB6\x91\x90a\x1B\xF2V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD9`\x04\x806\x03\x81\x01\x90a\x02\xD4\x91\x90a!oV[a\r\xA8V[`@Qa\x02\xE6\x91\x90a\x1FsV[`@Q\x80\x91\x03\x90\xF3[a\x02\xF7a\r\xE3V[`@Qa\x03\x04\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x03\x15a\x0E\x07V[`@Qa\x03\"\x91\x90a\"UV[`@Q\x80\x91\x03\x90\xF3[a\x033a\x0E\x1AV[`@Qa\x03@\x91\x90a\x1D\nV[`@Q\x80\x91\x03\x90\xF3[a\x03c`\x04\x806\x03\x81\x01\x90a\x03^\x91\x90a EV[a\x0E!V[\0[a\x03\x7F`\x04\x806\x03\x81\x01\x90a\x03z\x91\x90a\x1D%V[a\x0F\xFEV[\0[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03\xF4WPa\x03\xF3\x82a\x10\x1FV[[\x90P\x91\x90PV[a\x04%\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\r>V[a\x04dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04[\x90a\"\xE2V[`@Q\x80\x91\x03\x90\xFD[a\x04\x8E\x7Fs\xA9\x98S\x16\xCDL\xBF\xD1=\xAD\xCA\xA0\xE6\xF7s\xC8^\x93:\r\x88\xEF\xBE`\xE4\xDCI\xDA\x91v\xA0\x82a\x10\x89V[PV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x04\xB9\x82a\x04\x91V[a\x04\xC2\x81a\x11iV[a\x04\xCC\x83\x83a\x10\x89V[PPPV[a\x04\xD9a\x11}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05=\x90a#tV[`@Q\x80\x91\x03\x90\xFD[a\x05P\x82\x82a\x11\x85V[PPV[```\0`\x02\x83Qa\x05f\x91\x90a#\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x7FWa\x05~a\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x05\xB1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x90P`\0[\x84Q\x81\x10\x15a\x07OW\x81\x82Q\x86\x83\x81Q\x81\x10a\x06\x10Wa\x06\x0Fa$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16a\x06+\x91\x90a$cV[\x81Q\x81\x10a\x06<Wa\x06;a$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x02\x83a\x06U\x91\x90a#\xC3V[\x81Q\x81\x10a\x06fWa\x06ea$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x81\x82Q\x86\x83\x81Q\x81\x10a\x06\xABWa\x06\xAAa$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16a\x06\xC6\x91\x90a$\x94V[\x81Q\x81\x10a\x06\xD7Wa\x06\xD6a$\x05V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x01`\x02\x84a\x06\xF2\x91\x90a#\xC3V[a\x06\xFC\x91\x90a$\xC5V[\x81Q\x81\x10a\x07\rWa\x07\x0Ca$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x80\x80a\x07G\x90a$\xF9V[\x91PPa\x05\xF1V[P\x81`@Q` \x01a\x07a\x91\x90a%\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08n\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x9B\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDC\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tIW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t@\x90a'FV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80` \x01`@R\x80`\0\x81RP`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\tx\x91\x90a)hV[PPV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\no\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x9C\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDD\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0BJW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BA\x90a'FV[`@Q\x80\x91\x03\x90\xFD[\x80`\x03`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\x0Bj\x91\x90a)hV[PPPV[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C>\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ck\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAC\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x10\x90a'FV[`@Q\x80\x91\x03\x90\xFD[\x80`\x04`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\r9\x91\x90a)hV[PPPV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[```\0a\r\xB7\x85\x85\x85a\x12fV[\x90P\x80`@Q` \x01a\r\xCA\x91\x90a*\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[\x7Fs\xA9\x98S\x16\xCDL\xBF\xD1=\xAD\xCA\xA0\xE6\xF7s\xC8^\x93:\r\x88\xEF\xBE`\xE4\xDCI\xDA\x91v\xA0\x81V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x80\x1B\x81V[`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x02`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97z\x80p`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF0\x91\x90a&\x16V[`\x02`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x1D\x92\x91\x90a&CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F^\x91\x90a&\x81V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F\xCBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xC2\x90a'FV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80` \x01`@R\x80`\0\x81RP`\x04`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x90\x81a\x0F\xFA\x91\x90a)hV[PPV[a\x10\x07\x82a\x04\x91V[a\x10\x10\x81a\x11iV[a\x10\x1A\x83\x83a\x11\x85V[PPPV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a\x10\x93\x82\x82a\r>V[a\x11eW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x11\na\x11}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[a\x11z\x81a\x11ua\x11}V[a\x14\xCCV[PV[`\x003\x90P\x90V[a\x11\x8F\x82\x82a\r>V[\x15a\x12bW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x12\x07a\x11}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[```\0`@Q\x80a\x04\x80\x01`@R\x80a\x04V\x81R` \x01a/\xECa\x04V\x919\x90P`\0a\x12\x93\x85a\x05TV[\x90P`\0a\x12\xA0\x85a\x15QV[\x90P`\0a\x12\xAD\x88a\x15~V[\x90P`\0`\x03`\0\x8A\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\x12\xCF\x90a'\x95V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xFB\x90a'\x95V[\x80\x15a\x13HW\x80`\x1F\x10a\x13\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13HV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13+W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0`\x04`\0\x8B\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\x13o\x90a'\x95V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x9B\x90a'\x95V[\x80\x15a\x13\xE8W\x80`\x1F\x10a\x13\xBDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xE8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xCBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0\x82Q\x14\x80\x15a\x14\x02WP`\0\x81Q\x14\x15[\x15a\x14.W\x82`@Q` \x01a\x14\x18\x91\x90a*\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa\x14\x8DV[`\0\x82Q\x14\x15\x80\x15a\x14AWP`\0\x81Q\x14[\x15a\x14NW\x85\x90Pa\x14\x8CV[`\0\x82Q\x14\x80\x15a\x14`WP`\0\x81Q\x14[\x15a\x14\x8BW\x82`@Q` \x01a\x14v\x91\x90a*\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x85\x90P[[[a\x14\xBD\x82\x82\x87\x87\x87`@Q` \x01a\x14\xA9\x95\x94\x93\x92\x91\x90a-\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x16LV[\x96PPPPPPP\x93\x92PPPV[a\x14\xD6\x82\x82a\r>V[a\x15MWa\x14\xE3\x81a\x15QV[a\x14\xF1\x83`\0\x1C` a\x17\xAFV[`@Q` \x01a\x15\x02\x92\x91\x90a/\x1CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15D\x91\x90a\x1FsV[`@Q\x80\x91\x03\x90\xFD[PPV[``a\x15w\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a\x17\xAFV[\x90P\x91\x90PV[```\0`\x01a\x15\x8D\x84a\x19\xEBV[\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xACWa\x15\xABa\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15\xDEW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0\x82` \x01\x82\x01\x90P[`\x01\x15a\x16AW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x165Wa\x164a$4V[[\x04\x94P`\0\x85\x03a\x15\xECW[\x81\x93PPPP\x91\x90PV[```\0\x82Q\x03a\x16nW`@Q\x80` \x01`@R\x80`\0\x81RP\x90Pa\x17\xAAV[`\0`@Q\x80``\x01`@R\x80`@\x81R` \x01a4B`@\x919\x90P`\0`\x03`\x02\x85Qa\x16\x9D\x91\x90a$\xC5V[a\x16\xA7\x91\x90a$cV[`\x04a\x16\xB3\x91\x90a#\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xCCWa\x16\xCBa\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x16\xFEW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\x01\x82\x01` \x82\x01\x85\x86Q\x87\x01[\x80\x82\x10\x15a\x17jW`\x03\x82\x01\x91P\x81Q`?\x81`\x12\x1C\x16\x85\x01Q\x84S`\x01\x84\x01\x93P`?\x81`\x0C\x1C\x16\x85\x01Q\x84S`\x01\x84\x01\x93P`?\x81`\x06\x1C\x16\x85\x01Q\x84S`\x01\x84\x01\x93P`?\x81\x16\x85\x01Q\x84S`\x01\x84\x01\x93PPa\x17\x0FV[PP`\x03\x86Q\x06`\x01\x81\x14a\x17\x86W`\x02\x81\x14a\x17\x99Wa\x17\xA1V[`=`\x01\x83\x03S`=`\x02\x83\x03Sa\x17\xA1V[`=`\x01\x83\x03S[PPP\x80\x92PPP[\x91\x90PV[```\0`\x02\x83`\x02a\x17\xC2\x91\x90a#\xC3V[a\x17\xCC\x91\x90a$\xC5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xE5Wa\x17\xE4a\x1D\x80V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\x17W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x18OWa\x18Na$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x18\xB3Wa\x18\xB2a$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a\x18\xF3\x91\x90a#\xC3V[a\x18\xFD\x91\x90a$\xC5V[\x90P[`\x01\x81\x11\x15a\x19\x9DW\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a\x19?Wa\x19>a$\x05V[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x19VWa\x19Ua$\x05V[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a\x19\x96\x90a/VV[\x90Pa\x19\0V[P`\0\x84\x14a\x19\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xD8\x90a/\xCBV[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0\x80`\0\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\x1AIWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a\x1A?Wa\x1A>a$4V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1A\x86Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a\x1A|Wa\x1A{a$4V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a\x1A\xB5Wf#\x86\xF2o\xC1\0\0\x83\x81a\x1A\xABWa\x1A\xAAa$4V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a\x1A\xDEWc\x05\xF5\xE1\0\x83\x81a\x1A\xD4Wa\x1A\xD3a$4V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a\x1B\x03Wa'\x10\x83\x81a\x1A\xF9Wa\x1A\xF8a$4V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a\x1B&W`d\x83\x81a\x1B\x1CWa\x1B\x1Ba$4V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a\x1B5W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x1B\x87\x81a\x1BRV[\x81\x14a\x1B\x92W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1B\xA4\x81a\x1B~V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1B\xC0Wa\x1B\xBFa\x1BHV[[`\0a\x1B\xCE\x84\x82\x85\x01a\x1B\x95V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x1B\xEC\x81a\x1B\xD7V[\x82RPPV[`\0` \x82\x01\x90Pa\x1C\x07`\0\x83\x01\x84a\x1B\xE3V[\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x1C8\x82a\x1C\rV[\x90P\x91\x90PV[a\x1CH\x81a\x1C-V[\x81\x14a\x1CSW`\0\x80\xFD[PV[`\0\x815\x90Pa\x1Ce\x81a\x1C?V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\x81Wa\x1C\x80a\x1BHV[[`\0a\x1C\x8F\x84\x82\x85\x01a\x1CVV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x1C\xAB\x81a\x1C\x98V[\x81\x14a\x1C\xB6W`\0\x80\xFD[PV[`\0\x815\x90Pa\x1C\xC8\x81a\x1C\xA2V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\xE4Wa\x1C\xE3a\x1BHV[[`\0a\x1C\xF2\x84\x82\x85\x01a\x1C\xB9V[\x91PP\x92\x91PPV[a\x1D\x04\x81a\x1C\x98V[\x82RPPV[`\0` \x82\x01\x90Pa\x1D\x1F`\0\x83\x01\x84a\x1C\xFBV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D<Wa\x1D;a\x1BHV[[`\0a\x1DJ\x85\x82\x86\x01a\x1C\xB9V[\x92PP` a\x1D[\x85\x82\x86\x01a\x1CVV[\x91PP\x92P\x92\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x1D\xB8\x82a\x1DoV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\xD7Wa\x1D\xD6a\x1D\x80V[[\x80`@RPPPV[`\0a\x1D\xEAa\x1B>V[\x90Pa\x1D\xF6\x82\x82a\x1D\xAFV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1E\x16Wa\x1E\x15a\x1D\x80V[[a\x1E\x1F\x82a\x1DoV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x1ENa\x1EI\x84a\x1D\xFBV[a\x1D\xE0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x1EjWa\x1Eia\x1DjV[[a\x1Eu\x84\x82\x85a\x1E,V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x1E\x92Wa\x1E\x91a\x1DeV[[\x815a\x1E\xA2\x84\x82` \x86\x01a\x1E;V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1E\xC1Wa\x1E\xC0a\x1BHV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xDFWa\x1E\xDEa\x1BMV[[a\x1E\xEB\x84\x82\x85\x01a\x1E}V[\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1F.W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1F\x13V[`\0\x84\x84\x01RPPPPV[`\0a\x1FE\x82a\x1E\xF4V[a\x1FO\x81\x85a\x1E\xFFV[\x93Pa\x1F_\x81\x85` \x86\x01a\x1F\x10V[a\x1Fh\x81a\x1DoV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1F\x8D\x81\x84a\x1F:V[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x1F\xBAa\x1F\xB5a\x1F\xB0\x84a\x1C\rV[a\x1F\x95V[a\x1C\rV[\x90P\x91\x90PV[`\0a\x1F\xCC\x82a\x1F\x9FV[\x90P\x91\x90PV[`\0a\x1F\xDE\x82a\x1F\xC1V[\x90P\x91\x90PV[a\x1F\xEE\x81a\x1F\xD3V[\x82RPPV[`\0` \x82\x01\x90Pa \t`\0\x83\x01\x84a\x1F\xE5V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a \"\x81a \x0FV[\x81\x14a -W`\0\x80\xFD[PV[`\0\x815\x90Pa ?\x81a \x19V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a [Wa Za\x1BHV[[`\0a i\x84\x82\x85\x01a 0V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a \x8DWa \x8Ca\x1D\x80V[[a \x96\x82a\x1DoV[\x90P` \x81\x01\x90P\x91\x90PV[`\0a \xB6a \xB1\x84a rV[a\x1D\xE0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a \xD2Wa \xD1a\x1DjV[[a \xDD\x84\x82\x85a\x1E,V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a \xFAWa \xF9a\x1DeV[[\x815a!\n\x84\x82` \x86\x01a \xA3V[\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a!*Wa!)a\x1BHV[[`\0a!8\x85\x82\x86\x01a 0V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!YWa!Xa\x1BMV[[a!e\x85\x82\x86\x01a \xE5V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x88Wa!\x87a\x1BHV[[`\0a!\x96\x86\x82\x87\x01a 0V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xB7Wa!\xB6a\x1BMV[[a!\xC3\x86\x82\x87\x01a\x1E}V[\x92PP`@a!\xD4\x86\x82\x87\x01a\x1CVV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\"\x1EWa\"\x1Da!\xDEV[[PV[`\0\x81\x90Pa\"/\x82a\"\rV[\x91\x90PV[`\0a\"?\x82a\"!V[\x90P\x91\x90PV[a\"O\x81a\"4V[\x82RPPV[`\0` \x82\x01\x90Pa\"j`\0\x83\x01\x84a\"FV[\x92\x91PPV[\x7FPKPNFTMetadata: must had admin r`\0\x82\x01R\x7Fole\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\"\xCC`#\x83a\x1E\xFFV[\x91Pa\"\xD7\x82a\"pV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\"\xFB\x81a\"\xBFV[\x90P\x91\x90PV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a#^`/\x83a\x1E\xFFV[\x91Pa#i\x82a#\x02V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra#\x8D\x81a#QV[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a#\xCE\x82a \x0FV[\x91Pa#\xD9\x83a \x0FV[\x92P\x82\x82\x02a#\xE7\x81a \x0FV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a#\xFEWa#\xFDa#\x94V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[`\0a$n\x82a \x0FV[\x91Pa$y\x83a \x0FV[\x92P\x82a$\x89Wa$\x88a$4V[[\x82\x82\x04\x90P\x92\x91PPV[`\0a$\x9F\x82a \x0FV[\x91Pa$\xAA\x83a \x0FV[\x92P\x82a$\xBAWa$\xB9a$4V[[\x82\x82\x06\x90P\x92\x91PPV[`\0a$\xD0\x82a \x0FV[\x91Pa$\xDB\x83a \x0FV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a$\xF3Wa$\xF2a#\x94V[[\x92\x91PPV[`\0a%\x04\x82a \x0FV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a%6Wa%5a#\x94V[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[\x7F0x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a%\x82`\x02\x83a%AV[\x91Pa%\x8D\x82a%LV[`\x02\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0a%\xB9\x82a%\x98V[a%\xC3\x81\x85a%\xA3V[\x93Pa%\xD3\x81\x85` \x86\x01a\x1F\x10V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a%\xEA\x82a%uV[\x91Pa%\xF6\x82\x84a%\xAEV[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90Pa&\x10\x81a\x1C\xA2V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a&,Wa&+a\x1BHV[[`\0a&:\x84\x82\x85\x01a&\x01V[\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa&X`\0\x83\x01\x85a\x1C\xFBV[a&e` \x83\x01\x84a\"FV[\x93\x92PPPV[`\0\x81Q\x90Pa&{\x81a\x1C?V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a&\x97Wa&\x96a\x1BHV[[`\0a&\xA5\x84\x82\x85\x01a&lV[\x91PP\x92\x91PPV[\x7FPKPHelper: only the Domain Walle`\0\x82\x01R\x7Ft registry is allowed to mint do` \x82\x01R\x7Fmain wallets, who are you?\0\0\0\0\0\0`@\x82\x01RPV[`\0a'0`Z\x83a\x1E\xFFV[\x91Pa';\x82a&\xAEV[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra'_\x81a'#V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a'\xADW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\xC0Wa'\xBFa'fV[[P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a((\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a'\xEBV[a(2\x86\x83a'\xEBV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0a(ea(`a([\x84a \x0FV[a\x1F\x95V[a \x0FV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a(\x7F\x83a(JV[a(\x93a(\x8B\x82a(lV[\x84\x84Ta'\xF8V[\x82UPPPPV[`\0\x90V[a(\xA8a(\x9BV[a(\xB3\x81\x84\x84a(vV[PPPV[[\x81\x81\x10\x15a(\xD7Wa(\xCC`\0\x82a(\xA0V[`\x01\x81\x01\x90Pa(\xB9V[PPV[`\x1F\x82\x11\x15a)\x1CWa(\xED\x81a'\xC6V[a(\xF6\x84a'\xDBV[\x81\x01` \x85\x10\x15a)\x05W\x81\x90P[a)\x19a)\x11\x85a'\xDBV[\x83\x01\x82a(\xB8V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a)?`\0\x19\x84`\x08\x02a)!V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a)X\x83\x83a).V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a)q\x82a\x1E\xF4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\x8AWa)\x89a\x1D\x80V[[a)\x94\x82Ta'\x95V[a)\x9F\x82\x82\x85a(\xDBV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a)\xD2W`\0\x84\x15a)\xC0W\x82\x87\x01Q\x90P[a)\xCA\x85\x82a)LV[\x86UPa*2V[`\x1F\x19\x84\x16a)\xE0\x86a'\xC6V[`\0[\x82\x81\x10\x15a*\x08W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa)\xE3V[\x86\x83\x10\x15a*%W\x84\x89\x01Qa*!`\x1F\x89\x16\x82a).V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7Fdata:application/json;base64,\0\0\0`\0\x82\x01RPV[`\0a*p`\x1D\x83a%AV[\x91Pa*{\x82a*:V[`\x1D\x82\x01\x90P\x91\x90PV[`\0a*\x91\x82a\x1E\xF4V[a*\x9B\x81\x85a%AV[\x93Pa*\xAB\x81\x85` \x86\x01a\x1F\x10V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a*\xC2\x82a*cV[\x91Pa*\xCE\x82\x84a*\x86V[\x91P\x81\x90P\x92\x91PPV[\x7FLit PKP #\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPV[`\0a+\n\x82a*\xD9V[`\t\x82\x01\x91Pa+\x1A\x82\x84a*\x86V[\x91P\x81\x90P\x92\x91PPV[\x7F{\"name\":\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a+[`\t\x83a%AV[\x91Pa+f\x82a+%V[`\t\x82\x01\x90P\x91\x90PV[\x7F\", \"description\": \"This NFT enti`\0\x82\x01R\x7Ftles the holder to use a Lit Pro` \x82\x01R\x7Ftocol PKP, and to grant access t`@\x82\x01R\x7Fo other users and Lit Actions to``\x82\x01R\x7F use this PKP\",\"image_data\": \"\0\0`\x80\x82\x01RPV[`\0a,?`\x9E\x83a%AV[\x91Pa,J\x82a+qV[`\x9E\x82\x01\x90P\x91\x90PV[\x7F\",\"attributes\": [{\"trait_type\": `\0\x82\x01R\x7F\"Public Key\", \"value\": \"\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a,\xB1`8\x83a%AV[\x91Pa,\xBC\x82a,UV[`8\x82\x01\x90P\x91\x90PV[\x7F\"}, {\"trait_type\": \"ETH Wallet A`\0\x82\x01R\x7Fddress\", \"value\": \"\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a-#`3\x83a%AV[\x91Pa-.\x82a,\xC7V[`3\x82\x01\x90P\x91\x90PV[\x7F\"}, {\"trait_type\": \"Token ID\", \"`\0\x82\x01R\x7Fvalue\": \"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a-\x95`)\x83a%AV[\x91Pa-\xA0\x82a-9V[`)\x82\x01\x90P\x91\x90PV[\x7F\"}]}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a-\xE1`\x04\x83a%AV[\x91Pa-\xEC\x82a-\xABV[`\x04\x82\x01\x90P\x91\x90PV[`\0a.\x02\x82a+NV[\x91Pa.\x0E\x82\x88a*\x86V[\x91Pa.\x19\x82a,2V[\x91Pa.%\x82\x87a%\xAEV[\x91Pa.0\x82a,\xA4V[\x91Pa.<\x82\x86a*\x86V[\x91Pa.G\x82a-\x16V[\x91Pa.S\x82\x85a*\x86V[\x91Pa.^\x82a-\x88V[\x91Pa.j\x82\x84a*\x86V[\x91Pa.u\x82a-\xD4V[\x91P\x81\x90P\x96\x95PPPPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a.\xBA`\x17\x83a%AV[\x91Pa.\xC5\x82a.\x84V[`\x17\x82\x01\x90P\x91\x90PV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a/\x06`\x11\x83a%AV[\x91Pa/\x11\x82a.\xD0V[`\x11\x82\x01\x90P\x91\x90PV[`\0a/'\x82a.\xADV[\x91Pa/3\x82\x85a*\x86V[\x91Pa/>\x82a.\xF9V[\x91Pa/J\x82\x84a*\x86V[\x91P\x81\x90P\x93\x92PPPV[`\0a/a\x82a \x0FV[\x91P`\0\x82\x03a/tWa/sa#\x94V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0a/\xB5` \x83a\x1E\xFFV[\x91Pa/\xC0\x82a/\x7FV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra/\xE4\x81a/\xA8V[\x90P\x91\x90PV\xFE<svg xmlns='http://www.w3.org/2000/svg' width='1080' height='1080' fill='none' xmlns:v='https://vecta.io/nano'><path d='M363.076 392.227s-.977 18.524-36.874 78.947c-41.576 70.018-45.481 151.978-3.017 220.4 89.521 144.245 332.481 141.52 422.556.089 34.832-54.707 44.816-117.479 32.924-181.248 0 0-28.819-133.144-127.237-217.099 1.553 1.308 5.369 19.122 6.101 26.722 2.241 23.354.045 47.838-7.787 70.062-5.746 16.33-13.711 30.467-27.178 41.368 0-3.811-.954-10.635-.976-12.918-.644-46.508-18.659-89.582-48.011-125.743-25.647-31.552-60.812-53.089-97.84-68.932.931 3.191 2.662 16.419 2.906 19.033 1.908 21.958 2.263 52.713-.621 74.649s-7.832 33.878-14.554 54.441c-10.184 31.175-24.05 54.285-41.621 82.004-3.24 5.096-12.913 19.078-18.082 26.146 0 0-8.897-56.191-40.667-87.921h-.022z' fill='#000'/><path d='M562.5 27.28l410.279 236.874c13.923 8.039 22.5 22.895 22.5 38.971v473.75c0 16.076-8.577 30.932-22.5 38.971L562.5 1052.72c-13.923 8.04-31.077 8.04-45 0L107.221 815.846c-13.923-8.039-22.5-22.895-22.5-38.971v-473.75a45 45 0 0 1 22.5-38.971L517.5 27.28a45 45 0 0 1 45 0z' stroke='#000' stroke-width='24.75'/></svg>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\xA2dipfsX\"\x12 \x0C\xEB,\x1A{y|\xD2\x1A\xE5\x91\x01gvQF'Z(\xFD(\xF1{[D4`\x8B\x05gG\x08dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static PKPNFTMETADATA_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PKPNFTMetadata<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PKPNFTMetadata<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PKPNFTMetadata<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PKPNFTMetadata<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PKPNFTMetadata<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PKPNFTMetadata))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PKPNFTMetadata<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PKPNFTMETADATA_ABI.clone(),
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
                PKPNFTMETADATA_ABI.clone(),
                PKPNFTMETADATA_BYTECODE.clone().into(),
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
        ///Calls the contract's `WRITER_ROLE` (0x9beaab7b) function
        pub fn writer_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([155, 234, 171, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bytesToHex` (0x451d89fa) function
        pub fn bytes_to_hex(
            &self,
            buffer: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([69, 29, 137, 250], buffer)
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
        ///Calls the contract's `removeProfileForPkp` (0xb63a7677) function
        pub fn remove_profile_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 58, 118, 119], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeUrlForPKP` (0x519a218e) function
        pub fn remove_url_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 154, 33, 142], token_id)
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
        ///Calls the contract's `setPKPHelperWriterAddress` (0x0fa8ae2f) function
        pub fn set_pkp_helper_writer_address(
            &self,
            pkp_helper_writer_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 168, 174, 47], pkp_helper_writer_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProfileForPKP` (0x9000fee1) function
        pub fn set_profile_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
            img_url: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 0, 254, 225], (token_id, img_url))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUrlForPKP` (0x855eec22) function
        pub fn set_url_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
            url: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 94, 236, 34], (token_id, url))
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
        ///Calls the contract's `tokenURI` (0x950462ee) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
            pub_key: ::ethers::core::types::Bytes,
            eth_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 4, 98, 238], (token_id, pub_key, eth_address))
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
            PKPNFTMetadataEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PKPNFTMetadata<M> {
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
    pub enum PKPNFTMetadataEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PKPNFTMetadataEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(PKPNFTMetadataEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(PKPNFTMetadataEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(PKPNFTMetadataEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PKPNFTMetadataEvents {
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
    impl ::core::convert::From<RoleAdminChangedFilter> for PKPNFTMetadataEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for PKPNFTMetadataEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for PKPNFTMetadataEvents {
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
    ///Container type for all input parameters for the `WRITER_ROLE` function with signature `WRITER_ROLE()` and selector `0x9beaab7b`
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
    #[ethcall(name = "WRITER_ROLE", abi = "WRITER_ROLE()")]
    pub struct WriterRoleCall;
    ///Container type for all input parameters for the `bytesToHex` function with signature `bytesToHex(bytes)` and selector `0x451d89fa`
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
    #[ethcall(name = "bytesToHex", abi = "bytesToHex(bytes)")]
    pub struct BytesToHexCall {
        pub buffer: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `removeProfileForPkp` function with signature `removeProfileForPkp(uint256)` and selector `0xb63a7677`
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
    #[ethcall(name = "removeProfileForPkp", abi = "removeProfileForPkp(uint256)")]
    pub struct RemoveProfileForPkpCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeUrlForPKP` function with signature `removeUrlForPKP(uint256)` and selector `0x519a218e`
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
    #[ethcall(name = "removeUrlForPKP", abi = "removeUrlForPKP(uint256)")]
    pub struct RemoveUrlForPKPCall {
        pub token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setPKPHelperWriterAddress` function with signature `setPKPHelperWriterAddress(address)` and selector `0x0fa8ae2f`
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
        name = "setPKPHelperWriterAddress",
        abi = "setPKPHelperWriterAddress(address)"
    )]
    pub struct SetPKPHelperWriterAddressCall {
        pub pkp_helper_writer_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setProfileForPKP` function with signature `setProfileForPKP(uint256,string)` and selector `0x9000fee1`
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
    #[ethcall(name = "setProfileForPKP", abi = "setProfileForPKP(uint256,string)")]
    pub struct SetProfileForPKPCall {
        pub token_id: ::ethers::core::types::U256,
        pub img_url: ::std::string::String,
    }
    ///Container type for all input parameters for the `setUrlForPKP` function with signature `setUrlForPKP(uint256,string)` and selector `0x855eec22`
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
    #[ethcall(name = "setUrlForPKP", abi = "setUrlForPKP(uint256,string)")]
    pub struct SetUrlForPKPCall {
        pub token_id: ::ethers::core::types::U256,
        pub url: ::std::string::String,
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
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256,bytes,address)` and selector `0x950462ee`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256,bytes,address)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
        pub pub_key: ::ethers::core::types::Bytes,
        pub eth_address: ::ethers::core::types::Address,
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
    pub enum PKPNFTMetadataCalls {
        AdminRole(AdminRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        WriterRole(WriterRoleCall),
        BytesToHex(BytesToHexCall),
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RemoveProfileForPkp(RemoveProfileForPkpCall),
        RemoveUrlForPKP(RemoveUrlForPKPCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetPKPHelperWriterAddress(SetPKPHelperWriterAddressCall),
        SetProfileForPKP(SetProfileForPKPCall),
        SetUrlForPKP(SetUrlForPKPCall),
        SupportsInterface(SupportsInterfaceCall),
        TokenURI(TokenURICall),
    }
    impl ::ethers::core::abi::AbiDecode for PKPNFTMetadataCalls {
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
                = <WriterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WriterRole(decoded));
            }
            if let Ok(decoded)
                = <BytesToHexCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BytesToHex(decoded));
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
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <RemoveProfileForPkpCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveProfileForPkp(decoded));
            }
            if let Ok(decoded)
                = <RemoveUrlForPKPCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveUrlForPKP(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <SetPKPHelperWriterAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPKPHelperWriterAddress(decoded));
            }
            if let Ok(decoded)
                = <SetProfileForPKPCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetProfileForPKP(decoded));
            }
            if let Ok(decoded)
                = <SetUrlForPKPCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetUrlForPKP(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PKPNFTMetadataCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BytesToHex(element) => {
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
                Self::RemoveProfileForPkp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveUrlForPKP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPKPHelperWriterAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProfileForPKP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUrlForPKP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PKPNFTMetadataCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::BytesToHex(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Env(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveProfileForPkp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveUrlForPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPKPHelperWriterAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetProfileForPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUrlForPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for PKPNFTMetadataCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for PKPNFTMetadataCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<WriterRoleCall> for PKPNFTMetadataCalls {
        fn from(value: WriterRoleCall) -> Self {
            Self::WriterRole(value)
        }
    }
    impl ::core::convert::From<BytesToHexCall> for PKPNFTMetadataCalls {
        fn from(value: BytesToHexCall) -> Self {
            Self::BytesToHex(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for PKPNFTMetadataCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for PKPNFTMetadataCalls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for PKPNFTMetadataCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for PKPNFTMetadataCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for PKPNFTMetadataCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<RemoveProfileForPkpCall> for PKPNFTMetadataCalls {
        fn from(value: RemoveProfileForPkpCall) -> Self {
            Self::RemoveProfileForPkp(value)
        }
    }
    impl ::core::convert::From<RemoveUrlForPKPCall> for PKPNFTMetadataCalls {
        fn from(value: RemoveUrlForPKPCall) -> Self {
            Self::RemoveUrlForPKP(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for PKPNFTMetadataCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for PKPNFTMetadataCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetPKPHelperWriterAddressCall> for PKPNFTMetadataCalls {
        fn from(value: SetPKPHelperWriterAddressCall) -> Self {
            Self::SetPKPHelperWriterAddress(value)
        }
    }
    impl ::core::convert::From<SetProfileForPKPCall> for PKPNFTMetadataCalls {
        fn from(value: SetProfileForPKPCall) -> Self {
            Self::SetProfileForPKP(value)
        }
    }
    impl ::core::convert::From<SetUrlForPKPCall> for PKPNFTMetadataCalls {
        fn from(value: SetUrlForPKPCall) -> Self {
            Self::SetUrlForPKP(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for PKPNFTMetadataCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for PKPNFTMetadataCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
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
    ///Container type for all return fields from the `WRITER_ROLE` function with signature `WRITER_ROLE()` and selector `0x9beaab7b`
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
    pub struct WriterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `bytesToHex` function with signature `bytesToHex(bytes)` and selector `0x451d89fa`
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
    pub struct BytesToHexReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256,bytes,address)` and selector `0x950462ee`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}
