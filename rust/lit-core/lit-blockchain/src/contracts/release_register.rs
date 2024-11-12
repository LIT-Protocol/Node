pub use release_register::*;
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
pub mod release_register {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/ReleaseRegister.json",
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
                            ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ACTIVATOR_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ACTIVATOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("BURNER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BURNER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("CREATOR_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CREATOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("DEACTIVATOR_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEACTIVATOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_RO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_RO"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_SSH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_SSH"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_USERS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RELEASE_OPTION_USERS",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("addAllowedAdminSigningPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addAllowedAdminSigningPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
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
                    ::std::borrow::ToOwned::to_owned("addAllowedSubnet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAllowedSubnet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("burnRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("createRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Status",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Type",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platform"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Platform",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("options"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id_key_digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("public_key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("date"),
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
                    ::std::borrow::ToOwned::to_owned("getActiveRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActiveRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Type",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platform"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Platform",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("getActiveReleases"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActiveReleases"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCreatorDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCreatorDomain"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReleaseRegister.Release",
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
                    ::std::borrow::ToOwned::to_owned("hasAllowedAdminSigningPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hasAllowedAdminSigningPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasAllowedAuthorKeyDigest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hasAllowedAuthorKeyDigest",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasAllowedEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasAllowedEnv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
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
                    ::std::borrow::ToOwned::to_owned("hasAllowedSubnet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasAllowedSubnet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("hasCreatorInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasCreatorInit"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("initCreator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initCreator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorKeyDigest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "removeAllowedAdminSigningPublicKey",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeAllowedAdminSigningPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
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
                    ::std::borrow::ToOwned::to_owned("removeAllowedSubnet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeAllowedSubnet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("setReleaseStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReleaseStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Status",
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
                    ::std::borrow::ToOwned::to_owned(
                        "AllowedAdminSigningPublicKeyAdded",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAdminSigningPublicKeyAdded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AllowedAdminSigningPublicKeyRemoved",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAdminSigningPublicKeyRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedAuthorKeyDigestAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAuthorKeyDigestAdded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedAuthorKeyDigestRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAuthorKeyDigestRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("AllowedSubnetAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllowedSubnetAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedSubnetRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedSubnetRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InitCreator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("InitCreator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("domain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorKeyDigest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseBurned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReleaseBurned"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReleaseCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("date"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("platform"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("options"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id_key_digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("public_key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseStatusChange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReleaseStatusChange",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActivatorRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ActivatorRoleRequired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BurnerRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BurnerRoleRequired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CreatorRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CreatorRoleRequired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeactivatorRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeactivatorRoleRequired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidEnv"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidStatus"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ReleaseNotFound"),
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
    pub static RELEASEREGISTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0Q\x188\x03\x80b\0Q\x18\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x04\xA1V[b\0\0i\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3b\0\x02y` \x1B` \x1CV[b\0\0\x9B\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x80b\0\x02\x8F` \x1B` \x1CV[b\0\0\xED\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB6\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBb\0\x02\x8F` \x1B` \x1CV[b\0\x01?\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBb\0\x02\x8F` \x1B` \x1CV[b\0\x01\x91\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBb\0\x02\x8F` \x1B` \x1CV[b\0\x01\xE3\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC8\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBb\0\x02\x8F` \x1B` \x1CV[`\x01`\x03`\0\x83`\x02\x81\x11\x15b\0\x01\xFFWb\0\x01\xFEb\0\x04\xD3V[[`\x02\x81\x11\x15b\0\x02\x14Wb\0\x02\x13b\0\x04\xD3V[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qb\0\x02j\x91\x90b\0\x05SV[`@Q\x80\x91\x03\x90\xA1Pb\0\x05pV[b\0\x02\x8B\x82\x82b\0\x02\xF2` \x1B` \x1CV[PPV[`\0b\0\x02\xA2\x83b\0\x03\xE3` \x1B` \x1CV[\x90P\x81`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01\x81\x90UP\x81\x81\x84\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[b\0\x03\x04\x82\x82b\0\x04\x02` \x1B` \x1CV[b\0\x03\xDFW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPb\0\x03\x84b\0\x04l` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\x03\x81\x10b\0\x04\x87W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x04\x9B\x81b\0\x04yV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x04\xBAWb\0\x04\xB9b\0\x04tV[[`\0b\0\x04\xCA\x84\x82\x85\x01b\0\x04\x8AV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10b\0\x05\x16Wb\0\x05\x15b\0\x04\xD3V[[PV[`\0\x81\x90Pb\0\x05)\x82b\0\x05\x02V[\x91\x90PV[`\0b\0\x05;\x82b\0\x05\x19V[\x90P\x91\x90PV[b\0\x05M\x81b\0\x05.V[\x82RPPV[`\0` \x82\x01\x90Pb\0\x05j`\0\x83\x01\x84b\0\x05BV[\x92\x91PPV[aK\x98\x80b\0\x05\x80`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80cp\xE6ZE\x11a\x01\x1AW\x80c\xA0\x90\x83\0\x11a\0\xADW\x80c\xD5Gt\x1F\x11a\0|W\x80c\xD5Gt\x1F\x14a\x05\xEFW\x80c\xD6\xBCbm\x14a\x06\x0BW\x80c\xDB-0;\x14a\x06;W\x80c\xE1\xC0\xAF\x08\x14a\x06WW\x80c\xF2\xDC\x99\x16\x14a\x06sWa\x02\x06V[\x80c\xA0\x90\x83\0\x14a\x05gW\x80c\xA2\x17\xFD\xDF\x14a\x05\x97W\x80c\xADv\x93\x94\x14a\x05\xB5W\x80c\xBC|\xA3\x17\x14a\x05\xD3Wa\x02\x06V[\x80c\x8A\xED\xA2Z\x11a\0\xE9W\x80c\x8A\xED\xA2Z\x14a\x04\xDFW\x80c\x8D\xEB8\x93\x14a\x04\xFDW\x80c\x91\xD1HT\x14a\x05\x19W\x80c\x9B\xB4\xE2\xF7\x14a\x05IWa\x02\x06V[\x80cp\xE6ZE\x14a\x04YW\x80ct\xBC\x819\x14a\x04uW\x80cu\xB28\xFC\x14a\x04\x91W\x80c\x7Fi\x8E\x92\x14a\x04\xAFWa\x02\x06V[\x80c&\t\xE5\x86\x11a\x01\x9DW\x80c//\xF1]\x11a\x01lW\x80c//\xF1]\x14a\x03\xC7W\x80c6V\x8A\xBE\x14a\x03\xE3W\x80c:\xCD\x1E\xA3\x14a\x03\xFFW\x80c=\xC6\xC8X\x14a\x04\x1DW\x80cE\x8B\xE7\xDC\x14a\x04;Wa\x02\x06V[\x80c&\t\xE5\x86\x14a\x03?W\x80c'}\xCE\xAF\x14a\x03[W\x80c(,Q\xF3\x14a\x03yW\x80c*\xE7\x9Bm\x14a\x03\x97Wa\x02\x06V[\x80c\x0E\x1EY\xDD\x11a\x01\xD9W\x80c\x0E\x1EY\xDD\x14a\x02\xA7W\x80c\x19r@e\x14a\x02\xC3W\x80c\x1B\xD5d\xDC\x14a\x02\xDFW\x80c$\x8A\x9C\xA3\x14a\x03\x0FWa\x02\x06V[\x80c\x01\xFF\xC9\xA7\x14a\x02\x0BW\x80c\x02>\x92\x88\x14a\x02;W\x80c\x08t\n;\x14a\x02YW\x80c\x0E\t+\x18\x14a\x02wW[`\0\x80\xFD[a\x02%`\x04\x806\x03\x81\x01\x90a\x02 \x91\x90a3\xD7V[a\x06\x8FV[`@Qa\x022\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x02Ca\x07\tV[`@Qa\x02P\x91\x90a5\x02V[`@Q\x80\x91\x03\x90\xF3[a\x02aa\x07\x1AV[`@Qa\x02n\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a6\x94V[a\x07>V[`@Qa\x02\x9E\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x02\xC1`\x04\x806\x03\x81\x01\x90a\x02\xBC\x91\x90a7.V[a\x07sV[\0[a\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a8\x13V[a\x0E\tV[\0[a\x02\xF9`\x04\x806\x03\x81\x01\x90a\x02\xF4\x91\x90a9wV[a\x14\xABV[`@Qa\x03\x06\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x03)`\x04\x806\x03\x81\x01\x90a\x03$\x91\x90a9\xA4V[a\x14\xF9V[`@Qa\x036\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x03Y`\x04\x806\x03\x81\x01\x90a\x03T\x91\x90a9\xA4V[a\x15\x18V[\0[a\x03ca\x16\xFEV[`@Qa\x03p\x91\x90a9\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x03\x81a\x17\x03V[`@Qa\x03\x8E\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x03\xB1`\x04\x806\x03\x81\x01\x90a\x03\xAC\x91\x90a9\xFBV[a\x17'V[`@Qa\x03\xBE\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x03\xE1`\x04\x806\x03\x81\x01\x90a\x03\xDC\x91\x90a:\xDCV[a\x17xV[\0[a\x03\xFD`\x04\x806\x03\x81\x01\x90a\x03\xF8\x91\x90a:\xDCV[a\x17\x99V[\0[a\x04\x07a\x18\x1CV[`@Qa\x04\x14\x91\x90a9\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x04%a\x18!V[`@Qa\x042\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04Ca\x188V[`@Qa\x04P\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x04s`\x04\x806\x03\x81\x01\x90a\x04n\x91\x90a6\x94V[a\x18\\V[\0[a\x04\x8F`\x04\x806\x03\x81\x01\x90a\x04\x8A\x91\x90a9wV[a\x19-V[\0[a\x04\x99a\x1A\x17V[`@Qa\x04\xA6\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x04\xC9`\x04\x806\x03\x81\x01\x90a\x04\xC4\x91\x90a9\xA4V[a\x1A;V[`@Qa\x04\xD6\x91\x90a=\xE8V[`@Q\x80\x91\x03\x90\xF3[a\x04\xE7a\x1D\xB1V[`@Qa\x04\xF4\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x05\x17`\x04\x806\x03\x81\x01\x90a\x05\x12\x91\x90a9wV[a\x1D\xD5V[\0[a\x053`\x04\x806\x03\x81\x01\x90a\x05.\x91\x90a:\xDCV[a\x1E\xB6V[`@Qa\x05@\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x05Qa\x1F V[`@Qa\x05^\x91\x90a>TV[`@Q\x80\x91\x03\x90\xF3[a\x05\x81`\x04\x806\x03\x81\x01\x90a\x05|\x91\x90a>vV[a\x1F\xB2V[`@Qa\x05\x8E\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x05\x9Fa \x08V[`@Qa\x05\xAC\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x05\xBDa \x0FV[`@Qa\x05\xCA\x91\x90a9\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x05\xED`\x04\x806\x03\x81\x01\x90a\x05\xE8\x91\x90a>\xA3V[a \x14V[\0[a\x06\t`\x04\x806\x03\x81\x01\x90a\x06\x04\x91\x90a:\xDCV[a\"BV[\0[a\x06%`\x04\x806\x03\x81\x01\x90a\x06 \x91\x90a6\x94V[a\"cV[`@Qa\x062\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x06U`\x04\x806\x03\x81\x01\x90a\x06P\x91\x90a>vV[a\"\x98V[\0[a\x06q`\x04\x806\x03\x81\x01\x90a\x06l\x91\x90a6\x94V[a#\x81V[\0[a\x06\x8D`\x04\x806\x03\x81\x01\x90a\x06\x88\x91\x90a>vV[a$IV[\0[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x07\x02WPa\x07\x01\x82a%;V[[\x90P\x91\x90PV[``a\x07\x15`\ta%\xA5V[\x90P\x90V[\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<\x81V[`\0`\x05\x82`@Qa\x07P\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\x02`\x03\x81\x11\x15a\x07\x87Wa\x07\x86a;\x1CV[[\x81`\x03\x81\x11\x15a\x07\x9AWa\x07\x99a;\x1CV[[\x03a\x08\x04Wa\x07\xC9\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<3a\x1E\xB6V[a\x07\xFFW`@Q\x7F\\!\x12>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xC7V[`\x03\x80\x81\x11\x15a\x08\x17Wa\x08\x16a;\x1CV[[\x81`\x03\x81\x11\x15a\x08*Wa\x08)a;\x1CV[[\x03a\x08\x94Wa\x08Y\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF3a\x1E\xB6V[a\x08\x8FW`@Q\x7F\t>\xFA\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xC6V[`@Q\x7F\xF5%\xE3 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[[`\0`\x03\x81\x11\x15a\x08\xDBWa\x08\xDAa;\x1CV[[`\x07`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\t\x11Wa\t\x10a;\x1CV[[\x03a\tHW`@Q\x7F(f?\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x07`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\t\x8EWa\t\x8Da;\x1CV[[`\x03\x81\x11\x15a\t\xA0Wa\t\x9Fa;\x1CV[[\x81R` \x01`\0\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x02\x81\x11\x15a\t\xC9Wa\t\xC8a;\x1CV[[`\x02\x81\x11\x15a\t\xDBWa\t\xDAa;\x1CV[[\x81R` \x01`\0\x82\x01`\x02\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\n\x04Wa\n\x03a;\x1CV[[`\x03\x81\x11\x15a\n\x16Wa\n\x15a;\x1CV[[\x81R` \x01`\x01\x82\x01\x80Ta\n*\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nV\x90a?\xC4V[\x80\x15a\n\xA3W\x80`\x1F\x10a\nxWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xA3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x86W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\0\x81\x11\x15a\n\xDBWa\n\xDAa;\x1CV[[`\0\x81\x11\x15a\n\xEDWa\n\xECa;\x1CV[[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x0B\x0B\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B7\x90a?\xC4V[\x80\x15a\x0B\x84W\x80`\x1F\x10a\x0BYWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x84V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BgW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x0B\x9D\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xC9\x90a?\xC4V[\x80\x15a\x0C\x16W\x80`\x1F\x10a\x0B\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x16V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xF9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x0C/\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C[\x90a?\xC4V[\x80\x15a\x0C\xA8W\x80`\x1F\x10a\x0C}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a\x0C\xC7Wa\x0C\xC6a;\x1CV[[\x82`\x03\x81\x11\x15a\x0C\xDAWa\x0C\xD9a;\x1CV[[\x14a\r=W\x82a\x0C\xFC\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x85`\xA0\x01Qa\x17'V[\x03a\r<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r3\x90a@xV[`@Q\x80\x91\x03\x90\xFD[[\x81`\x07`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\rvWa\rua;\x1CV[[\x02\x17\x90UP\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x83\x83`@Qa\r\xAC\x92\x91\x90a@\xA7V[`@Q\x80\x91\x03\x90\xA1`\x02`\x03\x81\x11\x15a\r\xC8Wa\r\xC7a;\x1CV[[\x82`\x03\x81\x11\x15a\r\xDBWa\r\xDAa;\x1CV[[\x03a\r\xEEWa\r\xE9\x83a%\xC6V[a\x0E\x04V[a\x0E\x02\x83`\ta*\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[PPPV[a\x0E3\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB63a\x1E\xB6V[a\x0EiW`@Q\x7F\x80Q\x0F\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a\x0E}Wa\x0E|a;\x1CV[[\x8A`\x03\x81\x11\x15a\x0E\x90Wa\x0E\x8Fa;\x1CV[[\x03a\x0E\xFAWa\x0E\xBF\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<3a\x1E\xB6V[a\x0E\xF5W`@Q\x7F\\!\x12>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x8BV[`\x01`\x03\x81\x11\x15a\x0F\x0EWa\x0F\ra;\x1CV[[\x8A`\x03\x81\x11\x15a\x0F!Wa\x0F a;\x1CV[[\x14\x15\x80\x15a\x0FSWP`\x03\x80\x81\x11\x15a\x0F=Wa\x0F<a;\x1CV[[\x8A`\x03\x81\x11\x15a\x0FPWa\x0FOa;\x1CV[[\x14\x15[\x15a\x0F\x8AW`@Q\x7F\xF5%\xE3 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[[`\x01\x15\x15`\x03`\0\x8B`\x02\x81\x11\x15a\x0F\xA6Wa\x0F\xA5a;\x1CV[[`\x02\x81\x11\x15a\x0F\xB8Wa\x0F\xB7a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a\x10\rW`@Q\x7F\xC8\xE7\xA9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x15\x15`\x04`\0a\x10 \x8E`\x04a+\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a\x10\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xA1\x90aAhV[`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x81\x11\x15a\x10\xBEWa\x10\xBDa;\x1CV[[\x89`\x02\x81\x11\x15a\x10\xD1Wa\x10\xD0a;\x1CV[[\x14\x15\x80\x15a\x11\x04WP`\x01`\x02\x81\x11\x15a\x10\xEEWa\x10\xEDa;\x1CV[[\x89`\x02\x81\x11\x15a\x11\x01Wa\x11\0a;\x1CV[[\x14\x15[\x15a\x11\x94W`\0`\x03\x81\x11\x15a\x11\x1DWa\x11\x1Ca;\x1CV[[`\x07`\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x11SWa\x11Ra;\x1CV[[\x14a\x11\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x8A\x90aA\xFAV[`@Q\x80\x91\x03\x90\xFD[[`\x02\x80\x81\x11\x15a\x11\xA7Wa\x11\xA6a;\x1CV[[\x89`\x02\x81\x11\x15a\x11\xBAWa\x11\xB9a;\x1CV[[\x03a\x12\x06W`\0`\x02\x86\x16\x03a\x12\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xFC\x90aB\x8CV[`@Q\x80\x91\x03\x90\xFD[[`\0\x81\x03a\x12\x12WB\x90P[`@Q\x80a\x01@\x01`@R\x80\x8B`\x03\x81\x11\x15a\x121Wa\x120a;\x1CV[[\x81R` \x01\x8A`\x02\x81\x11\x15a\x12IWa\x12Ha;\x1CV[[\x81R` \x01\x89`\x03\x81\x11\x15a\x12aWa\x12`a;\x1CV[[\x81R` \x01\x88\x81R` \x01\x82\x81R` \x01\x87`\0\x81\x11\x15a\x12\x85Wa\x12\x84a;\x1CV[[\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x83\x81RP`\x07`\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x12\xDEWa\x12\xDDa;\x1CV[[\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x02\x81\x11\x15a\x13\x0EWa\x13\ra;\x1CV[[\x02\x17\x90UP`@\x82\x01Q\x81`\0\x01`\x02a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x13>Wa\x13=a;\x1CV[[\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01\x90\x81a\x13X\x91\x90aDXV[P`\x80\x82\x01Q\x81`\x02\x01U`\xA0\x82\x01Q\x81`\x03\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\0\x81\x11\x15a\x13\x8EWa\x13\x8Da;\x1CV[[\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01U`\xE0\x82\x01Q\x81`\x05\x01\x90\x81a\x13\xB2\x91\x90aDXV[Pa\x01\0\x82\x01Q\x81`\x06\x01\x90\x81a\x13\xC9\x91\x90aDXV[Pa\x01 \x82\x01Q\x81`\x07\x01\x90\x81a\x13\xE0\x91\x90aDXV[P\x90PP\x7F\x90\xA8b\xCC\x16\xEB\xCB\x9BT\x9C\x93 \x13\xF7|B-\xF1\x17\xE2MH\xD7\xD2j}\x90\xFB\xC4<<\x8F\x8B\x8B\x8B\x8B\x8B\x86\x8C\x8C\x8C\x8C\x8C`@Qa\x14'\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aEWV[`@Q\x80\x91\x03\x90\xA1`\x02`\x03\x81\x11\x15a\x14CWa\x14Ba;\x1CV[[\x8A`\x03\x81\x11\x15a\x14VWa\x14Ua;\x1CV[[\x03a\x14\x9EWa\x14d\x8Ba%\xC6V[\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x8B\x8B`@Qa\x14\x95\x92\x91\x90a@\xA7V[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPPV[`\0`\x03`\0\x83`\x02\x81\x11\x15a\x14\xC4Wa\x14\xC3a;\x1CV[[`\x02\x81\x11\x15a\x14\xD6Wa\x14\xD5a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x15B\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC83a\x1E\xB6V[a\x15xW`@Q\x7F\xF4\xEC\xADd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x81\x11\x15a\x15\x8CWa\x15\x8Ba;\x1CV[[`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x15\xC2Wa\x15\xC1a;\x1CV[[\x03a\x15\xF9W`@Q\x7F(f?\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x80\x82\x01`\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\0\x82\x01`\x01a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\0\x82\x01`\x02a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x01\x82\x01`\0a\x16Y\x91\x90a2tV[`\x02\x82\x01`\0\x90U`\x03\x82\x01`\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x04\x82\x01`\0\x90U`\x05\x82\x01`\0a\x16\x8D\x91\x90a2tV[`\x06\x82\x01`\0a\x16\x9D\x91\x90a2tV[`\x07\x82\x01`\0a\x16\xAD\x91\x90a2tV[PPa\x16\xC3\x81`\ta*\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x7F\xE4\xA7\xF7\xB4\x82Q\xADp\xE6?\x80\x07X\xA4E\xB0\x03\x86\xE2\xFA\x98\xD5\xAF\xCE\x96\xA5F\xE8\xFC\xE2\x11N\x81`@Qa\x16\xF3\x91\x90a53V[`@Q\x80\x91\x03\x90\xA1PV[`\x04\x81V[\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC8\x81V[`\0\x80\x85\x85\x85\x85`@Q` \x01a\x17A\x94\x93\x92\x91\x90aF\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x08`\0\x82\x81R` \x01\x90\x81R` \x01`\0 T\x91PP\x94\x93PPPPV[a\x17\x81\x82a\x14\xF9V[a\x17\x8A\x81a+\x8FV[a\x17\x94\x83\x83a+\xA3V[PPPV[a\x17\xA1a,\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\x05\x90aG>V[`@Q\x80\x91\x03\x90\xFD[a\x18\x18\x82\x82a,\x8BV[PPV[`\x08\x81V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF\x81V[a\x18\x86\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\x18\xBCW`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x82`@Qa\x18\xCE\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xE7s^\x9FV\x9F\xE6\x16qf*\x88)\xDBw\xDE8\xEFaLw\xB1\xB16\xC9X\xAF\xF7\x81\xDF|u\x81`@Qa\x19\"\x91\x90a>TV[`@Q\x80\x91\x03\x90\xA1PV[a\x19W\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\x19\x8DW`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x83`\x02\x81\x11\x15a\x19\xA6Wa\x19\xA5a;\x1CV[[`\x02\x81\x11\x15a\x19\xB8Wa\x19\xB7a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\x1A\x0C\x91\x90aG^V[`@Q\x80\x91\x03\x90\xA1PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[a\x1ACa2\xB4V[`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1A\x87Wa\x1A\x86a;\x1CV[[`\x03\x81\x11\x15a\x1A\x99Wa\x1A\x98a;\x1CV[[\x81R` \x01`\0\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1A\xC2Wa\x1A\xC1a;\x1CV[[`\x02\x81\x11\x15a\x1A\xD4Wa\x1A\xD3a;\x1CV[[\x81R` \x01`\0\x82\x01`\x02\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1A\xFDWa\x1A\xFCa;\x1CV[[`\x03\x81\x11\x15a\x1B\x0FWa\x1B\x0Ea;\x1CV[[\x81R` \x01`\x01\x82\x01\x80Ta\x1B#\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1BO\x90a?\xC4V[\x80\x15a\x1B\x9CW\x80`\x1F\x10a\x1BqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\0\x81\x11\x15a\x1B\xD4Wa\x1B\xD3a;\x1CV[[`\0\x81\x11\x15a\x1B\xE6Wa\x1B\xE5a;\x1CV[[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x1C\x04\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C0\x90a?\xC4V[\x80\x15a\x1C}W\x80`\x1F\x10a\x1CRWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C}V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x1C\x96\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xC2\x90a?\xC4V[\x80\x15a\x1D\x0FW\x80`\x1F\x10a\x1C\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x1D(\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1DT\x90a?\xC4V[\x80\x15a\x1D\xA1W\x80`\x1F\x10a\x1DvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\xA1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\x84W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB6\x81V[a\x1D\xFF\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\x1E5W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x82`\x02\x81\x11\x15a\x1ELWa\x1EKa;\x1CV[[`\x02\x81\x11\x15a\x1E^Wa\x1E]a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x81`@Qa\x1E\xAB\x91\x90aG^V[`@Q\x80\x91\x03\x90\xA1PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[```\x02\x80Ta\x1F/\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F[\x90a?\xC4V[\x80\x15a\x1F\xA8W\x80`\x1F\x10a\x1F}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0`\x04`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x80\x1B\x81V[`\x02\x81V[a >\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB63a\x1E\xB6V[a tW`@Q\x7F\x80Q\x0F\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x15\x15`\x03`\0\x86`\x02\x81\x11\x15a \x8FWa \x8Ea;\x1CV[[`\x02\x81\x11\x15a \xA1Wa \xA0a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a \xF6W`@Q\x7F\xC8\xE7\xA9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x02\x90\x81a!\x05\x91\x90aDXV[P`\x01`\x04`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\x06\x82`@Qa!p\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x18\xF09lm\x01\x87mv\x10\xD9hw\xB4O\x01z\xB4\xCA\"e\xB10y\xB1\x0E\x0B\xE6\xB6\xAF0\xD4\x81`@Qa!\xC4\x91\x90a>TV[`@Q\x80\x91\x03\x90\xA1\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x83`@Qa!\xFB\x91\x90aG\x88V[`@Q\x80\x91\x03\x90\xA1\x7F[Y\x0C\xCE\xAB\xBE\xB2\x8C\xD1nA\xA8\x11\xBAF\xDF[\xB8,\x98\x92\r^\x7F\xF7~O\xA9\xDBp\x9B3\x82\x82`@Qa\"4\x92\x91\x90aG\xA3V[`@Q\x80\x91\x03\x90\xA1PPPPV[a\"K\x82a\x14\xF9V[a\"T\x81a+\x8FV[a\"^\x83\x83a,\x8BV[PPPV[`\0`\x06\x82`@Qa\"u\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[a\"\xC2\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\"\xF8W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F\xF5\x80\xA1\xEA\x01\xC0\xED>\xC5\xE4y|V\x0E\xE7\x04l5\x91X\x8C\x0F\xEB\x81O_-N\xCAP',\x81`@Qa#v\x91\x90aG\x88V[`@Q\x80\x91\x03\x90\xA1PV[a#\xAB\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a#\xE1W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x81`@Qa#\xF1\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F\xF10B%]\x0BX\xF7\xAC\xC8, \x1A\x0C\xD4\xD84\xB0x=\xD7\xB3\xC2\x9C\xCA\x9B\x96\x84\rc?t\x81`@Qa$>\x91\x90a>TV[`@Q\x80\x91\x03\x90\xA1PV[a$s\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a$\xA9W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x04`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x81`@Qa%0\x91\x90aG\x88V[`@Q\x80\x91\x03\x90\xA1PV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[```\0a%\xB5\x83`\0\x01a-lV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[`\0`\x03\x81\x11\x15a%\xDAWa%\xD9a;\x1CV[[`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a&\x10Wa&\x0Fa;\x1CV[[\x03a&GW`@Q\x7F(f?\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a&\x8DWa&\x8Ca;\x1CV[[`\x03\x81\x11\x15a&\x9FWa&\x9Ea;\x1CV[[\x81R` \x01`\0\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x02\x81\x11\x15a&\xC8Wa&\xC7a;\x1CV[[`\x02\x81\x11\x15a&\xDAWa&\xD9a;\x1CV[[\x81R` \x01`\0\x82\x01`\x02\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a'\x03Wa'\x02a;\x1CV[[`\x03\x81\x11\x15a'\x15Wa'\x14a;\x1CV[[\x81R` \x01`\x01\x82\x01\x80Ta')\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'U\x90a?\xC4V[\x80\x15a'\xA2W\x80`\x1F\x10a'wWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\xA2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\x85W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\0\x81\x11\x15a'\xDAWa'\xD9a;\x1CV[[`\0\x81\x11\x15a'\xECWa'\xEBa;\x1CV[[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta(\n\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(6\x90a?\xC4V[\x80\x15a(\x83W\x80`\x1F\x10a(XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\x83V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(fW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta(\x9C\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\xC8\x90a?\xC4V[\x80\x15a)\x15W\x80`\x1F\x10a(\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a)\x15V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xF8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta).\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta)Z\x90a?\xC4V[\x80\x15a)\xA7W\x80`\x1F\x10a)|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a)\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a)\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a)\xC6Wa)\xC5a;\x1CV[[\x81`\0\x01Q`\x03\x81\x11\x15a)\xDDWa)\xDCa;\x1CV[[\x14a*\x14W`@Q\x7F\xF5%\xE3 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84`\xA0\x01Q`@Q` \x01a*=\x94\x93\x92\x91\x90aF\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\x08`\0\x83\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x80\x1B\x81\x14a*\xB7W`\x07`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01T\x83`\x80\x01Q\x11\x15a*\xB2W\x83`\x08`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP[a*\xD0V[\x83`\x08`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP[a*\xE4\x84`\ta-\xC8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPV[`\0a*\xFA\x83`\0\x01\x83a-\xDFV[\x90P\x92\x91PPV[`\0\x80`\0[`\x14\x81\x10\x15a+\x81W`\x08\x81a+\x1E\x91\x90aH\tV[`\xFF`\xF8\x1B\x86\x83\x87a+0\x91\x90aHKV[` \x81\x10a+AWa+@aH\x7FV[[\x1A`\xF8\x1B\x16~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x1C\x82\x17\x91P\x80\x80a+y\x90aH\xAEV[\x91PPa+\x08V[P\x80``\x1C\x91PP\x92\x91PPV[a+\xA0\x81a+\x9Ba,\x83V[a.\xF3V[PV[a+\xAD\x82\x82a\x1E\xB6V[a,\x7FW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa,$a,\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a,\x95\x82\x82a\x1E\xB6V[\x15a-hW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa-\ra,\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a-\xBCW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xA8W[PPPPP\x90P\x91\x90PV[`\0a-\xD7\x83`\0\x01\x83a/xV[\x90P\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x14a.\xE7W`\0`\x01\x82a.\x11\x91\x90aH\xF6V[\x90P`\0`\x01\x86`\0\x01\x80T\x90Pa.)\x91\x90aH\xF6V[\x90P\x81\x81\x14a.\x98W`\0\x86`\0\x01\x82\x81T\x81\x10a.JWa.IaH\x7FV[[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a.nWa.maH\x7FV[[\x90`\0R` `\0 \x01\x81\x90UP\x83\x87`\x01\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[\x85`\0\x01\x80T\x80a.\xACWa.\xABaI*V[[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa.\xEDV[`\0\x91PP[\x92\x91PPV[a.\xFD\x82\x82a\x1E\xB6V[a/tWa/\n\x81a/\xE8V[a/\x18\x83`\0\x1C` a0\x15V[`@Q` \x01a/)\x92\x91\x90aJ8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/k\x91\x90aJ\xABV[`@Q\x80\x91\x03\x90\xFD[PPV[`\0a/\x84\x83\x83a2QV[a/\xDDW\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pa/\xE2V[`\0\x90P[\x92\x91PPV[``a0\x0E\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a0\x15V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a0(\x91\x90aH\tV[a02\x91\x90aHKV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0KWa0Ja5iV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a0}W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a0\xB5Wa0\xB4aH\x7FV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a1\x19Wa1\x18aH\x7FV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a1Y\x91\x90aH\tV[a1c\x91\x90aHKV[\x90P[`\x01\x81\x11\x15a2\x03W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a1\xA5Wa1\xA4aH\x7FV[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a1\xBCWa1\xBBaH\x7FV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a1\xFC\x90aJ\xCDV[\x90Pa1fV[P`\0\x84\x14a2GW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2>\x90aKBV[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[P\x80Ta2\x80\x90a?\xC4V[`\0\x82U\x80`\x1F\x10a2\x92WPa2\xB1V[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a2\xB0\x91\x90a3NV[[PV[`@Q\x80a\x01@\x01`@R\x80`\0`\x03\x81\x11\x15a2\xD4Wa2\xD3a;\x1CV[[\x81R` \x01`\0`\x02\x81\x11\x15a2\xEDWa2\xECa;\x1CV[[\x81R` \x01`\0`\x03\x81\x11\x15a3\x06Wa3\x05a;\x1CV[[\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x80\x81\x11\x15a3,Wa3+a;\x1CV[[\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[[\x80\x82\x11\x15a3gW`\0\x81`\0\x90UP`\x01\x01a3OV[P\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a3\xB4\x81a3\x7FV[\x81\x14a3\xBFW`\0\x80\xFD[PV[`\0\x815\x90Pa3\xD1\x81a3\xABV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\xEDWa3\xECa3uV[[`\0a3\xFB\x84\x82\x85\x01a3\xC2V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a4\x19\x81a4\x04V[\x82RPPV[`\0` \x82\x01\x90Pa44`\0\x83\x01\x84a4\x10V[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a4y\x81a4fV[\x82RPPV[`\0a4\x8B\x83\x83a4pV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a4\xAF\x82a4:V[a4\xB9\x81\x85a4EV[\x93Pa4\xC4\x83a4VV[\x80`\0[\x83\x81\x10\x15a4\xF5W\x81Qa4\xDC\x88\x82a4\x7FV[\x97Pa4\xE7\x83a4\x97V[\x92PP`\x01\x81\x01\x90Pa4\xC8V[P\x85\x93PPPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra5\x1C\x81\x84a4\xA4V[\x90P\x92\x91PPV[a5-\x81a4fV[\x82RPPV[`\0` \x82\x01\x90Pa5H`\0\x83\x01\x84a5$V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a5\xA1\x82a5XV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a5\xC0Wa5\xBFa5iV[[\x80`@RPPPV[`\0a5\xD3a3kV[\x90Pa5\xDF\x82\x82a5\x98V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a5\xFFWa5\xFEa5iV[[a6\x08\x82a5XV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a67a62\x84a5\xE4V[a5\xC9V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a6SWa6Ra5SV[[a6^\x84\x82\x85a6\x15V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a6{Wa6za5NV[[\x815a6\x8B\x84\x82` \x86\x01a6$V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a6\xAAWa6\xA9a3uV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xC8Wa6\xC7a3zV[[a6\xD4\x84\x82\x85\x01a6fV[\x91PP\x92\x91PPV[a6\xE6\x81a4fV[\x81\x14a6\xF1W`\0\x80\xFD[PV[`\0\x815\x90Pa7\x03\x81a6\xDDV[\x92\x91PPV[`\x04\x81\x10a7\x16W`\0\x80\xFD[PV[`\0\x815\x90Pa7(\x81a7\tV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a7EWa7Da3uV[[`\0a7S\x85\x82\x86\x01a6\xF4V[\x92PP` a7d\x85\x82\x86\x01a7\x19V[\x91PP\x92P\x92\x90PV[`\x03\x81\x10a7{W`\0\x80\xFD[PV[`\0\x815\x90Pa7\x8D\x81a7nV[\x92\x91PPV[`\x04\x81\x10a7\xA0W`\0\x80\xFD[PV[`\0\x815\x90Pa7\xB2\x81a7\x93V[\x92\x91PPV[`\x01\x81\x10a7\xC5W`\0\x80\xFD[PV[`\0\x815\x90Pa7\xD7\x81a7\xB8V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a7\xF0\x81a7\xDDV[\x81\x14a7\xFBW`\0\x80\xFD[PV[`\0\x815\x90Pa8\r\x81a7\xE7V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a89Wa88a3uV[[`\0a8G\x8E\x82\x8F\x01a6\xF4V[\x9BPP` a8X\x8E\x82\x8F\x01a7\x19V[\x9APP`@a8i\x8E\x82\x8F\x01a7~V[\x99PP``a8z\x8E\x82\x8F\x01a7\xA3V[\x98PP`\x80\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\x9BWa8\x9Aa3zV[[a8\xA7\x8E\x82\x8F\x01a6fV[\x97PP`\xA0a8\xB8\x8E\x82\x8F\x01a7\xC8V[\x96PP`\xC0a8\xC9\x8E\x82\x8F\x01a7\xFEV[\x95PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xEAWa8\xE9a3zV[[a8\xF6\x8E\x82\x8F\x01a6fV[\x94PPa\x01\0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x18Wa9\x17a3zV[[a9$\x8E\x82\x8F\x01a6fV[\x93PPa\x01 \x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9FWa9Ea3zV[[a9R\x8E\x82\x8F\x01a6fV[\x92PPa\x01@a9d\x8E\x82\x8F\x01a7\xFEV[\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0` \x82\x84\x03\x12\x15a9\x8DWa9\x8Ca3uV[[`\0a9\x9B\x84\x82\x85\x01a7~V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a9\xBAWa9\xB9a3uV[[`\0a9\xC8\x84\x82\x85\x01a6\xF4V[\x91PP\x92\x91PPV[a9\xDA\x81a7\xDDV[\x82RPPV[`\0` \x82\x01\x90Pa9\xF5`\0\x83\x01\x84a9\xD1V[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\x15Wa:\x14a3uV[[`\0a:#\x87\x82\x88\x01a7~V[\x94PP` a:4\x87\x82\x88\x01a7\xA3V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:UWa:Ta3zV[[a:a\x87\x82\x88\x01a6fV[\x92PP``a:r\x87\x82\x88\x01a7\xC8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a:\xA9\x82a:~V[\x90P\x91\x90PV[a:\xB9\x81a:\x9EV[\x81\x14a:\xC4W`\0\x80\xFD[PV[`\0\x815\x90Pa:\xD6\x81a:\xB0V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a:\xF3Wa:\xF2a3uV[[`\0a;\x01\x85\x82\x86\x01a6\xF4V[\x92PP` a;\x12\x85\x82\x86\x01a:\xC7V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x04\x81\x10a;\\Wa;[a;\x1CV[[PV[`\0\x81\x90Pa;m\x82a;KV[\x91\x90PV[`\0a;}\x82a;_V[\x90P\x91\x90PV[a;\x8D\x81a;rV[\x82RPPV[`\x03\x81\x10a;\xA4Wa;\xA3a;\x1CV[[PV[`\0\x81\x90Pa;\xB5\x82a;\x93V[\x91\x90PV[`\0a;\xC5\x82a;\xA7V[\x90P\x91\x90PV[a;\xD5\x81a;\xBAV[\x82RPPV[`\x04\x81\x10a;\xECWa;\xEBa;\x1CV[[PV[`\0\x81\x90Pa;\xFD\x82a;\xDBV[\x91\x90PV[`\0a<\r\x82a;\xEFV[\x90P\x91\x90PV[a<\x1D\x81a<\x02V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a<]W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa<BV[`\0\x84\x84\x01RPPPPV[`\0a<t\x82a<#V[a<~\x81\x85a<.V[\x93Pa<\x8E\x81\x85` \x86\x01a<?V[a<\x97\x81a5XV[\x84\x01\x91PP\x92\x91PPV[a<\xAB\x81a7\xDDV[\x82RPPV[`\x01\x81\x10a<\xC2Wa<\xC1a;\x1CV[[PV[`\0\x81\x90Pa<\xD3\x82a<\xB1V[\x91\x90PV[`\0a<\xE3\x82a<\xC5V[\x90P\x91\x90PV[a<\xF3\x81a<\xD8V[\x82RPPV[`\0a\x01@\x83\x01`\0\x83\x01Qa=\x12`\0\x86\x01\x82a;\x84V[P` \x83\x01Qa=%` \x86\x01\x82a;\xCCV[P`@\x83\x01Qa=8`@\x86\x01\x82a<\x14V[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra=P\x82\x82a<iV[\x91PP`\x80\x83\x01Qa=e`\x80\x86\x01\x82a<\xA2V[P`\xA0\x83\x01Qa=x`\xA0\x86\x01\x82a<\xEAV[P`\xC0\x83\x01Qa=\x8B`\xC0\x86\x01\x82a<\xA2V[P`\xE0\x83\x01Q\x84\x82\x03`\xE0\x86\x01Ra=\xA3\x82\x82a<iV[\x91PPa\x01\0\x83\x01Q\x84\x82\x03a\x01\0\x86\x01Ra=\xBF\x82\x82a<iV[\x91PPa\x01 \x83\x01Q\x84\x82\x03a\x01 \x86\x01Ra=\xDB\x82\x82a<iV[\x91PP\x80\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>\x02\x81\x84a<\xF9V[\x90P\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a>&\x82a<#V[a>0\x81\x85a>\nV[\x93Pa>@\x81\x85` \x86\x01a<?V[a>I\x81a5XV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>n\x81\x84a>\x1BV[\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a>\x8CWa>\x8Ba3uV[[`\0a>\x9A\x84\x82\x85\x01a:\xC7V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a>\xBDWa>\xBCa3uV[[`\0a>\xCB\x87\x82\x88\x01a7~V[\x94PP` a>\xDC\x87\x82\x88\x01a:\xC7V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xFDWa>\xFCa3zV[[a?\t\x87\x82\x88\x01a6fV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?*Wa?)a3zV[[a?6\x87\x82\x88\x01a6fV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81\x90P\x92\x91PPV[`\0a?X\x82a<#V[a?b\x81\x85a?BV[\x93Pa?r\x81\x85` \x86\x01a<?V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a?\x8A\x82\x84a?MV[\x91P\x81\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a?\xDCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a?\xEFWa?\xEEa?\x95V[[P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FMust replace active release befo`\0\x82\x01R\x7Fre changing status from Active\0\0` \x82\x01RPV[`\0a@b`>\x83a?\xF5V[\x91Pa@m\x82a@\x06V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra@\x91\x81a@UV[\x90P\x91\x90PV[a@\xA1\x81a;rV[\x82RPPV[`\0`@\x82\x01\x90Pa@\xBC`\0\x83\x01\x85a5$V[a@\xC9` \x83\x01\x84a@\x98V[\x93\x92PPPV[\x7FThe provided subnet (within the `\0\x82\x01R\x7Frelease id) is not valid for thi` \x82\x01R\x7Fs contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aAR`J\x83a?\xF5V[\x91PaA]\x82a@\xD0V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaA\x81\x81aAEV[\x90P\x91\x90PV[\x7FA release with this ID already e`\0\x82\x01R\x7Fxists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aA\xE4`%\x83a?\xF5V[\x91PaA\xEF\x82aA\x88V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaB\x13\x81aA\xD7V[\x90P\x91\x90PV[\x7FThe RO option is required for pr`\0\x82\x01R\x7Fod releases\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aBv`+\x83a?\xF5V[\x91PaB\x81\x82aB\x1AV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaB\xA5\x81aBiV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02aC\x0E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aB\xD1V[aC\x18\x86\x83aB\xD1V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0aCUaCPaCK\x84a7\xDDV[aC0V[a7\xDDV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[aCo\x83aC:V[aC\x83aC{\x82aC\\V[\x84\x84TaB\xDEV[\x82UPPPPV[`\0\x90V[aC\x98aC\x8BV[aC\xA3\x81\x84\x84aCfV[PPPV[[\x81\x81\x10\x15aC\xC7WaC\xBC`\0\x82aC\x90V[`\x01\x81\x01\x90PaC\xA9V[PPV[`\x1F\x82\x11\x15aD\x0CWaC\xDD\x81aB\xACV[aC\xE6\x84aB\xC1V[\x81\x01` \x85\x10\x15aC\xF5W\x81\x90P[aD\taD\x01\x85aB\xC1V[\x83\x01\x82aC\xA8V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0aD/`\0\x19\x84`\x08\x02aD\x11V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0aDH\x83\x83aD\x1EV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aDa\x82a<#V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aDzWaDya5iV[[aD\x84\x82Ta?\xC4V[aD\x8F\x82\x82\x85aC\xCBV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14aD\xC2W`\0\x84\x15aD\xB0W\x82\x87\x01Q\x90P[aD\xBA\x85\x82aD<V[\x86UPaE\"V[`\x1F\x19\x84\x16aD\xD0\x86aB\xACV[`\0[\x82\x81\x10\x15aD\xF8W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaD\xD3V[\x86\x83\x10\x15aE\x15W\x84\x89\x01QaE\x11`\x1F\x89\x16\x82aD\x1EV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[aE3\x81a;\xBAV[\x82RPPV[aEB\x81a<\x02V[\x82RPPV[aEQ\x81a<\xD8V[\x82RPPV[`\0a\x01`\x82\x01\x90PaEm`\0\x83\x01\x8Ea5$V[aEz` \x83\x01\x8Da@\x98V[aE\x87`@\x83\x01\x8CaE*V[aE\x94``\x83\x01\x8BaE9V[\x81\x81\x03`\x80\x83\x01RaE\xA6\x81\x8Aa>\x1BV[\x90PaE\xB5`\xA0\x83\x01\x89a9\xD1V[aE\xC2`\xC0\x83\x01\x88aEHV[aE\xCF`\xE0\x83\x01\x87a9\xD1V[\x81\x81\x03a\x01\0\x83\x01RaE\xE2\x81\x86a>\x1BV[\x90P\x81\x81\x03a\x01 \x83\x01RaE\xF7\x81\x85a>\x1BV[\x90P\x81\x81\x03a\x01@\x83\x01RaF\x0C\x81\x84a>\x1BV[\x90P\x9C\x9BPPPPPPPPPPPPV[`\0\x81`\xF8\x1B\x90P\x91\x90PV[`\0aF6\x82aF\x1EV[\x90P\x91\x90PV[aFNaFI\x82a;\xBAV[aF+V[\x82RPPV[aFeaF`\x82a<\x02V[aF+V[\x82RPPV[aF|aFw\x82a<\xD8V[aF+V[\x82RPPV[`\0aF\x8E\x82\x87aF=V[`\x01\x82\x01\x91PaF\x9E\x82\x86aFTV[`\x01\x82\x01\x91PaF\xAE\x82\x85a?MV[\x91PaF\xBA\x82\x84aFkV[`\x01\x82\x01\x91P\x81\x90P\x95\x94PPPPPV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aG(`/\x83a?\xF5V[\x91PaG3\x82aF\xCCV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaGW\x81aG\x1BV[\x90P\x91\x90PV[`\0` \x82\x01\x90PaGs`\0\x83\x01\x84aE*V[\x92\x91PPV[aG\x82\x81a:\x9EV[\x82RPPV[`\0` \x82\x01\x90PaG\x9D`\0\x83\x01\x84aGyV[\x92\x91PPV[`\0`@\x82\x01\x90P\x81\x81\x03`\0\x83\x01RaG\xBD\x81\x85a>\x1BV[\x90P\x81\x81\x03` \x83\x01RaG\xD1\x81\x84a>\x1BV[\x90P\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0aH\x14\x82a7\xDDV[\x91PaH\x1F\x83a7\xDDV[\x92P\x82\x82\x02aH-\x81a7\xDDV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aHDWaHCaG\xDAV[[P\x92\x91PPV[`\0aHV\x82a7\xDDV[\x91PaHa\x83a7\xDDV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aHyWaHxaG\xDAV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0aH\xB9\x82a7\xDDV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aH\xEBWaH\xEAaG\xDAV[[`\x01\x82\x01\x90P\x91\x90PV[`\0aI\x01\x82a7\xDDV[\x91PaI\x0C\x83a7\xDDV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aI$WaI#aG\xDAV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0aI\x9A`\x17\x83aIYV[\x91PaI\xA5\x82aIdV[`\x17\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0aI\xC6\x82aI\xB0V[aI\xD0\x81\x85aIYV[\x93PaI\xE0\x81\x85` \x86\x01a<?V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0aJ\"`\x11\x83aIYV[\x91PaJ-\x82aI\xECV[`\x11\x82\x01\x90P\x91\x90PV[`\0aJC\x82aI\x8DV[\x91PaJO\x82\x85aI\xBBV[\x91PaJZ\x82aJ\x15V[\x91PaJf\x82\x84aI\xBBV[\x91P\x81\x90P\x93\x92PPPV[`\0aJ}\x82aI\xB0V[aJ\x87\x81\x85a?\xF5V[\x93PaJ\x97\x81\x85` \x86\x01a<?V[aJ\xA0\x81a5XV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaJ\xC5\x81\x84aJrV[\x90P\x92\x91PPV[`\0aJ\xD8\x82a7\xDDV[\x91P`\0\x82\x03aJ\xEBWaJ\xEAaG\xDAV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0aK,` \x83a?\xF5V[\x91PaK7\x82aJ\xF6V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaK[\x81aK\x1FV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 Zo\x07\xDA\rU\xA7\x9F?\x04\xB0?Y\x81\xBAo}\x15\x8B\xAA^bI\xF7\xE9,1e\xC8H\x80bdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static RELEASEREGISTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80cp\xE6ZE\x11a\x01\x1AW\x80c\xA0\x90\x83\0\x11a\0\xADW\x80c\xD5Gt\x1F\x11a\0|W\x80c\xD5Gt\x1F\x14a\x05\xEFW\x80c\xD6\xBCbm\x14a\x06\x0BW\x80c\xDB-0;\x14a\x06;W\x80c\xE1\xC0\xAF\x08\x14a\x06WW\x80c\xF2\xDC\x99\x16\x14a\x06sWa\x02\x06V[\x80c\xA0\x90\x83\0\x14a\x05gW\x80c\xA2\x17\xFD\xDF\x14a\x05\x97W\x80c\xADv\x93\x94\x14a\x05\xB5W\x80c\xBC|\xA3\x17\x14a\x05\xD3Wa\x02\x06V[\x80c\x8A\xED\xA2Z\x11a\0\xE9W\x80c\x8A\xED\xA2Z\x14a\x04\xDFW\x80c\x8D\xEB8\x93\x14a\x04\xFDW\x80c\x91\xD1HT\x14a\x05\x19W\x80c\x9B\xB4\xE2\xF7\x14a\x05IWa\x02\x06V[\x80cp\xE6ZE\x14a\x04YW\x80ct\xBC\x819\x14a\x04uW\x80cu\xB28\xFC\x14a\x04\x91W\x80c\x7Fi\x8E\x92\x14a\x04\xAFWa\x02\x06V[\x80c&\t\xE5\x86\x11a\x01\x9DW\x80c//\xF1]\x11a\x01lW\x80c//\xF1]\x14a\x03\xC7W\x80c6V\x8A\xBE\x14a\x03\xE3W\x80c:\xCD\x1E\xA3\x14a\x03\xFFW\x80c=\xC6\xC8X\x14a\x04\x1DW\x80cE\x8B\xE7\xDC\x14a\x04;Wa\x02\x06V[\x80c&\t\xE5\x86\x14a\x03?W\x80c'}\xCE\xAF\x14a\x03[W\x80c(,Q\xF3\x14a\x03yW\x80c*\xE7\x9Bm\x14a\x03\x97Wa\x02\x06V[\x80c\x0E\x1EY\xDD\x11a\x01\xD9W\x80c\x0E\x1EY\xDD\x14a\x02\xA7W\x80c\x19r@e\x14a\x02\xC3W\x80c\x1B\xD5d\xDC\x14a\x02\xDFW\x80c$\x8A\x9C\xA3\x14a\x03\x0FWa\x02\x06V[\x80c\x01\xFF\xC9\xA7\x14a\x02\x0BW\x80c\x02>\x92\x88\x14a\x02;W\x80c\x08t\n;\x14a\x02YW\x80c\x0E\t+\x18\x14a\x02wW[`\0\x80\xFD[a\x02%`\x04\x806\x03\x81\x01\x90a\x02 \x91\x90a3\xD7V[a\x06\x8FV[`@Qa\x022\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x02Ca\x07\tV[`@Qa\x02P\x91\x90a5\x02V[`@Q\x80\x91\x03\x90\xF3[a\x02aa\x07\x1AV[`@Qa\x02n\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a6\x94V[a\x07>V[`@Qa\x02\x9E\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x02\xC1`\x04\x806\x03\x81\x01\x90a\x02\xBC\x91\x90a7.V[a\x07sV[\0[a\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a8\x13V[a\x0E\tV[\0[a\x02\xF9`\x04\x806\x03\x81\x01\x90a\x02\xF4\x91\x90a9wV[a\x14\xABV[`@Qa\x03\x06\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x03)`\x04\x806\x03\x81\x01\x90a\x03$\x91\x90a9\xA4V[a\x14\xF9V[`@Qa\x036\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x03Y`\x04\x806\x03\x81\x01\x90a\x03T\x91\x90a9\xA4V[a\x15\x18V[\0[a\x03ca\x16\xFEV[`@Qa\x03p\x91\x90a9\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x03\x81a\x17\x03V[`@Qa\x03\x8E\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x03\xB1`\x04\x806\x03\x81\x01\x90a\x03\xAC\x91\x90a9\xFBV[a\x17'V[`@Qa\x03\xBE\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x03\xE1`\x04\x806\x03\x81\x01\x90a\x03\xDC\x91\x90a:\xDCV[a\x17xV[\0[a\x03\xFD`\x04\x806\x03\x81\x01\x90a\x03\xF8\x91\x90a:\xDCV[a\x17\x99V[\0[a\x04\x07a\x18\x1CV[`@Qa\x04\x14\x91\x90a9\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x04%a\x18!V[`@Qa\x042\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04Ca\x188V[`@Qa\x04P\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x04s`\x04\x806\x03\x81\x01\x90a\x04n\x91\x90a6\x94V[a\x18\\V[\0[a\x04\x8F`\x04\x806\x03\x81\x01\x90a\x04\x8A\x91\x90a9wV[a\x19-V[\0[a\x04\x99a\x1A\x17V[`@Qa\x04\xA6\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x04\xC9`\x04\x806\x03\x81\x01\x90a\x04\xC4\x91\x90a9\xA4V[a\x1A;V[`@Qa\x04\xD6\x91\x90a=\xE8V[`@Q\x80\x91\x03\x90\xF3[a\x04\xE7a\x1D\xB1V[`@Qa\x04\xF4\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x05\x17`\x04\x806\x03\x81\x01\x90a\x05\x12\x91\x90a9wV[a\x1D\xD5V[\0[a\x053`\x04\x806\x03\x81\x01\x90a\x05.\x91\x90a:\xDCV[a\x1E\xB6V[`@Qa\x05@\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x05Qa\x1F V[`@Qa\x05^\x91\x90a>TV[`@Q\x80\x91\x03\x90\xF3[a\x05\x81`\x04\x806\x03\x81\x01\x90a\x05|\x91\x90a>vV[a\x1F\xB2V[`@Qa\x05\x8E\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x05\x9Fa \x08V[`@Qa\x05\xAC\x91\x90a53V[`@Q\x80\x91\x03\x90\xF3[a\x05\xBDa \x0FV[`@Qa\x05\xCA\x91\x90a9\xE0V[`@Q\x80\x91\x03\x90\xF3[a\x05\xED`\x04\x806\x03\x81\x01\x90a\x05\xE8\x91\x90a>\xA3V[a \x14V[\0[a\x06\t`\x04\x806\x03\x81\x01\x90a\x06\x04\x91\x90a:\xDCV[a\"BV[\0[a\x06%`\x04\x806\x03\x81\x01\x90a\x06 \x91\x90a6\x94V[a\"cV[`@Qa\x062\x91\x90a4\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x06U`\x04\x806\x03\x81\x01\x90a\x06P\x91\x90a>vV[a\"\x98V[\0[a\x06q`\x04\x806\x03\x81\x01\x90a\x06l\x91\x90a6\x94V[a#\x81V[\0[a\x06\x8D`\x04\x806\x03\x81\x01\x90a\x06\x88\x91\x90a>vV[a$IV[\0[`\0\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x07\x02WPa\x07\x01\x82a%;V[[\x90P\x91\x90PV[``a\x07\x15`\ta%\xA5V[\x90P\x90V[\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<\x81V[`\0`\x05\x82`@Qa\x07P\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\x02`\x03\x81\x11\x15a\x07\x87Wa\x07\x86a;\x1CV[[\x81`\x03\x81\x11\x15a\x07\x9AWa\x07\x99a;\x1CV[[\x03a\x08\x04Wa\x07\xC9\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<3a\x1E\xB6V[a\x07\xFFW`@Q\x7F\\!\x12>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xC7V[`\x03\x80\x81\x11\x15a\x08\x17Wa\x08\x16a;\x1CV[[\x81`\x03\x81\x11\x15a\x08*Wa\x08)a;\x1CV[[\x03a\x08\x94Wa\x08Y\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF3a\x1E\xB6V[a\x08\x8FW`@Q\x7F\t>\xFA\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xC6V[`@Q\x7F\xF5%\xE3 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[[`\0`\x03\x81\x11\x15a\x08\xDBWa\x08\xDAa;\x1CV[[`\x07`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\t\x11Wa\t\x10a;\x1CV[[\x03a\tHW`@Q\x7F(f?\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x07`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\t\x8EWa\t\x8Da;\x1CV[[`\x03\x81\x11\x15a\t\xA0Wa\t\x9Fa;\x1CV[[\x81R` \x01`\0\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x02\x81\x11\x15a\t\xC9Wa\t\xC8a;\x1CV[[`\x02\x81\x11\x15a\t\xDBWa\t\xDAa;\x1CV[[\x81R` \x01`\0\x82\x01`\x02\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\n\x04Wa\n\x03a;\x1CV[[`\x03\x81\x11\x15a\n\x16Wa\n\x15a;\x1CV[[\x81R` \x01`\x01\x82\x01\x80Ta\n*\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nV\x90a?\xC4V[\x80\x15a\n\xA3W\x80`\x1F\x10a\nxWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xA3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x86W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\0\x81\x11\x15a\n\xDBWa\n\xDAa;\x1CV[[`\0\x81\x11\x15a\n\xEDWa\n\xECa;\x1CV[[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x0B\x0B\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B7\x90a?\xC4V[\x80\x15a\x0B\x84W\x80`\x1F\x10a\x0BYWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x84V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BgW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x0B\x9D\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xC9\x90a?\xC4V[\x80\x15a\x0C\x16W\x80`\x1F\x10a\x0B\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x16V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xF9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x0C/\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C[\x90a?\xC4V[\x80\x15a\x0C\xA8W\x80`\x1F\x10a\x0C}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a\x0C\xC7Wa\x0C\xC6a;\x1CV[[\x82`\x03\x81\x11\x15a\x0C\xDAWa\x0C\xD9a;\x1CV[[\x14a\r=W\x82a\x0C\xFC\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x85`\xA0\x01Qa\x17'V[\x03a\r<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r3\x90a@xV[`@Q\x80\x91\x03\x90\xFD[[\x81`\x07`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\rvWa\rua;\x1CV[[\x02\x17\x90UP\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x83\x83`@Qa\r\xAC\x92\x91\x90a@\xA7V[`@Q\x80\x91\x03\x90\xA1`\x02`\x03\x81\x11\x15a\r\xC8Wa\r\xC7a;\x1CV[[\x82`\x03\x81\x11\x15a\r\xDBWa\r\xDAa;\x1CV[[\x03a\r\xEEWa\r\xE9\x83a%\xC6V[a\x0E\x04V[a\x0E\x02\x83`\ta*\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[PPPV[a\x0E3\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB63a\x1E\xB6V[a\x0EiW`@Q\x7F\x80Q\x0F\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a\x0E}Wa\x0E|a;\x1CV[[\x8A`\x03\x81\x11\x15a\x0E\x90Wa\x0E\x8Fa;\x1CV[[\x03a\x0E\xFAWa\x0E\xBF\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<3a\x1E\xB6V[a\x0E\xF5W`@Q\x7F\\!\x12>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x8BV[`\x01`\x03\x81\x11\x15a\x0F\x0EWa\x0F\ra;\x1CV[[\x8A`\x03\x81\x11\x15a\x0F!Wa\x0F a;\x1CV[[\x14\x15\x80\x15a\x0FSWP`\x03\x80\x81\x11\x15a\x0F=Wa\x0F<a;\x1CV[[\x8A`\x03\x81\x11\x15a\x0FPWa\x0FOa;\x1CV[[\x14\x15[\x15a\x0F\x8AW`@Q\x7F\xF5%\xE3 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[[`\x01\x15\x15`\x03`\0\x8B`\x02\x81\x11\x15a\x0F\xA6Wa\x0F\xA5a;\x1CV[[`\x02\x81\x11\x15a\x0F\xB8Wa\x0F\xB7a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a\x10\rW`@Q\x7F\xC8\xE7\xA9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x15\x15`\x04`\0a\x10 \x8E`\x04a+\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a\x10\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xA1\x90aAhV[`@Q\x80\x91\x03\x90\xFD[`\0`\x02\x81\x11\x15a\x10\xBEWa\x10\xBDa;\x1CV[[\x89`\x02\x81\x11\x15a\x10\xD1Wa\x10\xD0a;\x1CV[[\x14\x15\x80\x15a\x11\x04WP`\x01`\x02\x81\x11\x15a\x10\xEEWa\x10\xEDa;\x1CV[[\x89`\x02\x81\x11\x15a\x11\x01Wa\x11\0a;\x1CV[[\x14\x15[\x15a\x11\x94W`\0`\x03\x81\x11\x15a\x11\x1DWa\x11\x1Ca;\x1CV[[`\x07`\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x11SWa\x11Ra;\x1CV[[\x14a\x11\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x8A\x90aA\xFAV[`@Q\x80\x91\x03\x90\xFD[[`\x02\x80\x81\x11\x15a\x11\xA7Wa\x11\xA6a;\x1CV[[\x89`\x02\x81\x11\x15a\x11\xBAWa\x11\xB9a;\x1CV[[\x03a\x12\x06W`\0`\x02\x86\x16\x03a\x12\x05W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xFC\x90aB\x8CV[`@Q\x80\x91\x03\x90\xFD[[`\0\x81\x03a\x12\x12WB\x90P[`@Q\x80a\x01@\x01`@R\x80\x8B`\x03\x81\x11\x15a\x121Wa\x120a;\x1CV[[\x81R` \x01\x8A`\x02\x81\x11\x15a\x12IWa\x12Ha;\x1CV[[\x81R` \x01\x89`\x03\x81\x11\x15a\x12aWa\x12`a;\x1CV[[\x81R` \x01\x88\x81R` \x01\x82\x81R` \x01\x87`\0\x81\x11\x15a\x12\x85Wa\x12\x84a;\x1CV[[\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x83\x81RP`\x07`\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x12\xDEWa\x12\xDDa;\x1CV[[\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x02\x81\x11\x15a\x13\x0EWa\x13\ra;\x1CV[[\x02\x17\x90UP`@\x82\x01Q\x81`\0\x01`\x02a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x13>Wa\x13=a;\x1CV[[\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01\x90\x81a\x13X\x91\x90aDXV[P`\x80\x82\x01Q\x81`\x02\x01U`\xA0\x82\x01Q\x81`\x03\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\0\x81\x11\x15a\x13\x8EWa\x13\x8Da;\x1CV[[\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01U`\xE0\x82\x01Q\x81`\x05\x01\x90\x81a\x13\xB2\x91\x90aDXV[Pa\x01\0\x82\x01Q\x81`\x06\x01\x90\x81a\x13\xC9\x91\x90aDXV[Pa\x01 \x82\x01Q\x81`\x07\x01\x90\x81a\x13\xE0\x91\x90aDXV[P\x90PP\x7F\x90\xA8b\xCC\x16\xEB\xCB\x9BT\x9C\x93 \x13\xF7|B-\xF1\x17\xE2MH\xD7\xD2j}\x90\xFB\xC4<<\x8F\x8B\x8B\x8B\x8B\x8B\x86\x8C\x8C\x8C\x8C\x8C`@Qa\x14'\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aEWV[`@Q\x80\x91\x03\x90\xA1`\x02`\x03\x81\x11\x15a\x14CWa\x14Ba;\x1CV[[\x8A`\x03\x81\x11\x15a\x14VWa\x14Ua;\x1CV[[\x03a\x14\x9EWa\x14d\x8Ba%\xC6V[\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x8B\x8B`@Qa\x14\x95\x92\x91\x90a@\xA7V[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPPV[`\0`\x03`\0\x83`\x02\x81\x11\x15a\x14\xC4Wa\x14\xC3a;\x1CV[[`\x02\x81\x11\x15a\x14\xD6Wa\x14\xD5a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01T\x90P\x91\x90PV[a\x15B\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC83a\x1E\xB6V[a\x15xW`@Q\x7F\xF4\xEC\xADd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x81\x11\x15a\x15\x8CWa\x15\x8Ba;\x1CV[[`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x15\xC2Wa\x15\xC1a;\x1CV[[\x03a\x15\xF9W`@Q\x7F(f?\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\0\x80\x82\x01`\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\0\x82\x01`\x01a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\0\x82\x01`\x02a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x01\x82\x01`\0a\x16Y\x91\x90a2tV[`\x02\x82\x01`\0\x90U`\x03\x82\x01`\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x04\x82\x01`\0\x90U`\x05\x82\x01`\0a\x16\x8D\x91\x90a2tV[`\x06\x82\x01`\0a\x16\x9D\x91\x90a2tV[`\x07\x82\x01`\0a\x16\xAD\x91\x90a2tV[PPa\x16\xC3\x81`\ta*\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x7F\xE4\xA7\xF7\xB4\x82Q\xADp\xE6?\x80\x07X\xA4E\xB0\x03\x86\xE2\xFA\x98\xD5\xAF\xCE\x96\xA5F\xE8\xFC\xE2\x11N\x81`@Qa\x16\xF3\x91\x90a53V[`@Q\x80\x91\x03\x90\xA1PV[`\x04\x81V[\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC8\x81V[`\0\x80\x85\x85\x85\x85`@Q` \x01a\x17A\x94\x93\x92\x91\x90aF\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x08`\0\x82\x81R` \x01\x90\x81R` \x01`\0 T\x91PP\x94\x93PPPPV[a\x17\x81\x82a\x14\xF9V[a\x17\x8A\x81a+\x8FV[a\x17\x94\x83\x83a+\xA3V[PPPV[a\x17\xA1a,\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\x05\x90aG>V[`@Q\x80\x91\x03\x90\xFD[a\x18\x18\x82\x82a,\x8BV[PPV[`\x08\x81V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF\x81V[a\x18\x86\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\x18\xBCW`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x82`@Qa\x18\xCE\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xE7s^\x9FV\x9F\xE6\x16qf*\x88)\xDBw\xDE8\xEFaLw\xB1\xB16\xC9X\xAF\xF7\x81\xDF|u\x81`@Qa\x19\"\x91\x90a>TV[`@Q\x80\x91\x03\x90\xA1PV[a\x19W\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\x19\x8DW`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x83`\x02\x81\x11\x15a\x19\xA6Wa\x19\xA5a;\x1CV[[`\x02\x81\x11\x15a\x19\xB8Wa\x19\xB7a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\x1A\x0C\x91\x90aG^V[`@Q\x80\x91\x03\x90\xA1PV[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[a\x1ACa2\xB4V[`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1A\x87Wa\x1A\x86a;\x1CV[[`\x03\x81\x11\x15a\x1A\x99Wa\x1A\x98a;\x1CV[[\x81R` \x01`\0\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1A\xC2Wa\x1A\xC1a;\x1CV[[`\x02\x81\x11\x15a\x1A\xD4Wa\x1A\xD3a;\x1CV[[\x81R` \x01`\0\x82\x01`\x02\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1A\xFDWa\x1A\xFCa;\x1CV[[`\x03\x81\x11\x15a\x1B\x0FWa\x1B\x0Ea;\x1CV[[\x81R` \x01`\x01\x82\x01\x80Ta\x1B#\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1BO\x90a?\xC4V[\x80\x15a\x1B\x9CW\x80`\x1F\x10a\x1BqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\0\x81\x11\x15a\x1B\xD4Wa\x1B\xD3a;\x1CV[[`\0\x81\x11\x15a\x1B\xE6Wa\x1B\xE5a;\x1CV[[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x1C\x04\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C0\x90a?\xC4V[\x80\x15a\x1C}W\x80`\x1F\x10a\x1CRWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C}V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x1C\x96\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xC2\x90a?\xC4V[\x80\x15a\x1D\x0FW\x80`\x1F\x10a\x1C\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x1D(\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1DT\x90a?\xC4V[\x80\x15a\x1D\xA1W\x80`\x1F\x10a\x1DvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\xA1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\x84W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB6\x81V[a\x1D\xFF\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\x1E5W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x82`\x02\x81\x11\x15a\x1ELWa\x1EKa;\x1CV[[`\x02\x81\x11\x15a\x1E^Wa\x1E]a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x81`@Qa\x1E\xAB\x91\x90aG^V[`@Q\x80\x91\x03\x90\xA1PV[`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[```\x02\x80Ta\x1F/\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F[\x90a?\xC4V[\x80\x15a\x1F\xA8W\x80`\x1F\x10a\x1F}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0`\x04`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[`\0\x80\x1B\x81V[`\x02\x81V[a >\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB63a\x1E\xB6V[a tW`@Q\x7F\x80Q\x0F\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x15\x15`\x03`\0\x86`\x02\x81\x11\x15a \x8FWa \x8Ea;\x1CV[[`\x02\x81\x11\x15a \xA1Wa \xA0a;\x1CV[[\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x14a \xF6W`@Q\x7F\xC8\xE7\xA9|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x02\x90\x81a!\x05\x91\x90aDXV[P`\x01`\x04`\0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\x06\x82`@Qa!p\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x18\xF09lm\x01\x87mv\x10\xD9hw\xB4O\x01z\xB4\xCA\"e\xB10y\xB1\x0E\x0B\xE6\xB6\xAF0\xD4\x81`@Qa!\xC4\x91\x90a>TV[`@Q\x80\x91\x03\x90\xA1\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x83`@Qa!\xFB\x91\x90aG\x88V[`@Q\x80\x91\x03\x90\xA1\x7F[Y\x0C\xCE\xAB\xBE\xB2\x8C\xD1nA\xA8\x11\xBAF\xDF[\xB8,\x98\x92\r^\x7F\xF7~O\xA9\xDBp\x9B3\x82\x82`@Qa\"4\x92\x91\x90aG\xA3V[`@Q\x80\x91\x03\x90\xA1PPPPV[a\"K\x82a\x14\xF9V[a\"T\x81a+\x8FV[a\"^\x83\x83a,\x8BV[PPPV[`\0`\x06\x82`@Qa\"u\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x91\x90PV[a\"\xC2\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a\"\xF8W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F\xF5\x80\xA1\xEA\x01\xC0\xED>\xC5\xE4y|V\x0E\xE7\x04l5\x91X\x8C\x0F\xEB\x81O_-N\xCAP',\x81`@Qa#v\x91\x90aG\x88V[`@Q\x80\x91\x03\x90\xA1PV[a#\xAB\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a#\xE1W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x81`@Qa#\xF1\x91\x90a?~V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\0a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x7F\xF10B%]\x0BX\xF7\xAC\xC8, \x1A\x0C\xD4\xD84\xB0x=\xD7\xB3\xC2\x9C\xCA\x9B\x96\x84\rc?t\x81`@Qa$>\x91\x90a>TV[`@Q\x80\x91\x03\x90\xA1PV[a$s\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB3a\x1E\xB6V[a$\xA9W`@Q\x7F\xC8\x90\xF8J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x04`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x81`@Qa%0\x91\x90aG\x88V[`@Q\x80\x91\x03\x90\xA1PV[`\0\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[```\0a%\xB5\x83`\0\x01a-lV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[`\0`\x03\x81\x11\x15a%\xDAWa%\xD9a;\x1CV[[`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a&\x10Wa&\x0Fa;\x1CV[[\x03a&GW`@Q\x7F(f?\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x07`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a&\x8DWa&\x8Ca;\x1CV[[`\x03\x81\x11\x15a&\x9FWa&\x9Ea;\x1CV[[\x81R` \x01`\0\x82\x01`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x02\x81\x11\x15a&\xC8Wa&\xC7a;\x1CV[[`\x02\x81\x11\x15a&\xDAWa&\xD9a;\x1CV[[\x81R` \x01`\0\x82\x01`\x02\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a'\x03Wa'\x02a;\x1CV[[`\x03\x81\x11\x15a'\x15Wa'\x14a;\x1CV[[\x81R` \x01`\x01\x82\x01\x80Ta')\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'U\x90a?\xC4V[\x80\x15a'\xA2W\x80`\x1F\x10a'wWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\xA2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\x85W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\0\x81\x11\x15a'\xDAWa'\xD9a;\x1CV[[`\0\x81\x11\x15a'\xECWa'\xEBa;\x1CV[[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta(\n\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(6\x90a?\xC4V[\x80\x15a(\x83W\x80`\x1F\x10a(XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\x83V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(fW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta(\x9C\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\xC8\x90a?\xC4V[\x80\x15a)\x15W\x80`\x1F\x10a(\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a)\x15V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xF8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta).\x90a?\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta)Z\x90a?\xC4V[\x80\x15a)\xA7W\x80`\x1F\x10a)|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a)\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a)\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a)\xC6Wa)\xC5a;\x1CV[[\x81`\0\x01Q`\x03\x81\x11\x15a)\xDDWa)\xDCa;\x1CV[[\x14a*\x14W`@Q\x7F\xF5%\xE3 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84`\xA0\x01Q`@Q` \x01a*=\x94\x93\x92\x91\x90aF\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\x08`\0\x83\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x80\x1B\x81\x14a*\xB7W`\x07`\0\x82\x81R` \x01\x90\x81R` \x01`\0 `\x02\x01T\x83`\x80\x01Q\x11\x15a*\xB2W\x83`\x08`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP[a*\xD0V[\x83`\x08`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP[a*\xE4\x84`\ta-\xC8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[PPPPPV[`\0a*\xFA\x83`\0\x01\x83a-\xDFV[\x90P\x92\x91PPV[`\0\x80`\0[`\x14\x81\x10\x15a+\x81W`\x08\x81a+\x1E\x91\x90aH\tV[`\xFF`\xF8\x1B\x86\x83\x87a+0\x91\x90aHKV[` \x81\x10a+AWa+@aH\x7FV[[\x1A`\xF8\x1B\x16~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x1C\x82\x17\x91P\x80\x80a+y\x90aH\xAEV[\x91PPa+\x08V[P\x80``\x1C\x91PP\x92\x91PPV[a+\xA0\x81a+\x9Ba,\x83V[a.\xF3V[PV[a+\xAD\x82\x82a\x1E\xB6V[a,\x7FW`\x01`\0\x80\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa,$a,\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\x003\x90P\x90V[a,\x95\x82\x82a\x1E\xB6V[\x15a-hW`\0\x80`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa-\ra,\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4[PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a-\xBCW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xA8W[PPPPP\x90P\x91\x90PV[`\0a-\xD7\x83`\0\x01\x83a/xV[\x90P\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x14a.\xE7W`\0`\x01\x82a.\x11\x91\x90aH\xF6V[\x90P`\0`\x01\x86`\0\x01\x80T\x90Pa.)\x91\x90aH\xF6V[\x90P\x81\x81\x14a.\x98W`\0\x86`\0\x01\x82\x81T\x81\x10a.JWa.IaH\x7FV[[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a.nWa.maH\x7FV[[\x90`\0R` `\0 \x01\x81\x90UP\x83\x87`\x01\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[\x85`\0\x01\x80T\x80a.\xACWa.\xABaI*V[[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa.\xEDV[`\0\x91PP[\x92\x91PPV[a.\xFD\x82\x82a\x1E\xB6V[a/tWa/\n\x81a/\xE8V[a/\x18\x83`\0\x1C` a0\x15V[`@Q` \x01a/)\x92\x91\x90aJ8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/k\x91\x90aJ\xABV[`@Q\x80\x91\x03\x90\xFD[PPV[`\0a/\x84\x83\x83a2QV[a/\xDDW\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pa/\xE2V[`\0\x90P[\x92\x91PPV[``a0\x0E\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x14`\xFF\x16a0\x15V[\x90P\x91\x90PV[```\0`\x02\x83`\x02a0(\x91\x90aH\tV[a02\x91\x90aHKV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0KWa0Ja5iV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a0}W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a0\xB5Wa0\xB4aH\x7FV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a1\x19Wa1\x18aH\x7FV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0`\x01\x84`\x02a1Y\x91\x90aH\tV[a1c\x91\x90aHKV[\x90P[`\x01\x81\x11\x15a2\x03W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0F\x86\x16`\x10\x81\x10a1\xA5Wa1\xA4aH\x7FV[[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a1\xBCWa1\xBBaH\x7FV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x85\x90\x1C\x94P\x80a1\xFC\x90aJ\xCDV[\x90Pa1fV[P`\0\x84\x14a2GW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2>\x90aKBV[`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[P\x80Ta2\x80\x90a?\xC4V[`\0\x82U\x80`\x1F\x10a2\x92WPa2\xB1V[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a2\xB0\x91\x90a3NV[[PV[`@Q\x80a\x01@\x01`@R\x80`\0`\x03\x81\x11\x15a2\xD4Wa2\xD3a;\x1CV[[\x81R` \x01`\0`\x02\x81\x11\x15a2\xEDWa2\xECa;\x1CV[[\x81R` \x01`\0`\x03\x81\x11\x15a3\x06Wa3\x05a;\x1CV[[\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x80\x81\x11\x15a3,Wa3+a;\x1CV[[\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[[\x80\x82\x11\x15a3gW`\0\x81`\0\x90UP`\x01\x01a3OV[P\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a3\xB4\x81a3\x7FV[\x81\x14a3\xBFW`\0\x80\xFD[PV[`\0\x815\x90Pa3\xD1\x81a3\xABV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\xEDWa3\xECa3uV[[`\0a3\xFB\x84\x82\x85\x01a3\xC2V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a4\x19\x81a4\x04V[\x82RPPV[`\0` \x82\x01\x90Pa44`\0\x83\x01\x84a4\x10V[\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a4y\x81a4fV[\x82RPPV[`\0a4\x8B\x83\x83a4pV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a4\xAF\x82a4:V[a4\xB9\x81\x85a4EV[\x93Pa4\xC4\x83a4VV[\x80`\0[\x83\x81\x10\x15a4\xF5W\x81Qa4\xDC\x88\x82a4\x7FV[\x97Pa4\xE7\x83a4\x97V[\x92PP`\x01\x81\x01\x90Pa4\xC8V[P\x85\x93PPPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra5\x1C\x81\x84a4\xA4V[\x90P\x92\x91PPV[a5-\x81a4fV[\x82RPPV[`\0` \x82\x01\x90Pa5H`\0\x83\x01\x84a5$V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a5\xA1\x82a5XV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a5\xC0Wa5\xBFa5iV[[\x80`@RPPPV[`\0a5\xD3a3kV[\x90Pa5\xDF\x82\x82a5\x98V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a5\xFFWa5\xFEa5iV[[a6\x08\x82a5XV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a67a62\x84a5\xE4V[a5\xC9V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a6SWa6Ra5SV[[a6^\x84\x82\x85a6\x15V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a6{Wa6za5NV[[\x815a6\x8B\x84\x82` \x86\x01a6$V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a6\xAAWa6\xA9a3uV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xC8Wa6\xC7a3zV[[a6\xD4\x84\x82\x85\x01a6fV[\x91PP\x92\x91PPV[a6\xE6\x81a4fV[\x81\x14a6\xF1W`\0\x80\xFD[PV[`\0\x815\x90Pa7\x03\x81a6\xDDV[\x92\x91PPV[`\x04\x81\x10a7\x16W`\0\x80\xFD[PV[`\0\x815\x90Pa7(\x81a7\tV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a7EWa7Da3uV[[`\0a7S\x85\x82\x86\x01a6\xF4V[\x92PP` a7d\x85\x82\x86\x01a7\x19V[\x91PP\x92P\x92\x90PV[`\x03\x81\x10a7{W`\0\x80\xFD[PV[`\0\x815\x90Pa7\x8D\x81a7nV[\x92\x91PPV[`\x04\x81\x10a7\xA0W`\0\x80\xFD[PV[`\0\x815\x90Pa7\xB2\x81a7\x93V[\x92\x91PPV[`\x01\x81\x10a7\xC5W`\0\x80\xFD[PV[`\0\x815\x90Pa7\xD7\x81a7\xB8V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a7\xF0\x81a7\xDDV[\x81\x14a7\xFBW`\0\x80\xFD[PV[`\0\x815\x90Pa8\r\x81a7\xE7V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a89Wa88a3uV[[`\0a8G\x8E\x82\x8F\x01a6\xF4V[\x9BPP` a8X\x8E\x82\x8F\x01a7\x19V[\x9APP`@a8i\x8E\x82\x8F\x01a7~V[\x99PP``a8z\x8E\x82\x8F\x01a7\xA3V[\x98PP`\x80\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\x9BWa8\x9Aa3zV[[a8\xA7\x8E\x82\x8F\x01a6fV[\x97PP`\xA0a8\xB8\x8E\x82\x8F\x01a7\xC8V[\x96PP`\xC0a8\xC9\x8E\x82\x8F\x01a7\xFEV[\x95PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xEAWa8\xE9a3zV[[a8\xF6\x8E\x82\x8F\x01a6fV[\x94PPa\x01\0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x18Wa9\x17a3zV[[a9$\x8E\x82\x8F\x01a6fV[\x93PPa\x01 \x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9FWa9Ea3zV[[a9R\x8E\x82\x8F\x01a6fV[\x92PPa\x01@a9d\x8E\x82\x8F\x01a7\xFEV[\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0` \x82\x84\x03\x12\x15a9\x8DWa9\x8Ca3uV[[`\0a9\x9B\x84\x82\x85\x01a7~V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a9\xBAWa9\xB9a3uV[[`\0a9\xC8\x84\x82\x85\x01a6\xF4V[\x91PP\x92\x91PPV[a9\xDA\x81a7\xDDV[\x82RPPV[`\0` \x82\x01\x90Pa9\xF5`\0\x83\x01\x84a9\xD1V[\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\x15Wa:\x14a3uV[[`\0a:#\x87\x82\x88\x01a7~V[\x94PP` a:4\x87\x82\x88\x01a7\xA3V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:UWa:Ta3zV[[a:a\x87\x82\x88\x01a6fV[\x92PP``a:r\x87\x82\x88\x01a7\xC8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a:\xA9\x82a:~V[\x90P\x91\x90PV[a:\xB9\x81a:\x9EV[\x81\x14a:\xC4W`\0\x80\xFD[PV[`\0\x815\x90Pa:\xD6\x81a:\xB0V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a:\xF3Wa:\xF2a3uV[[`\0a;\x01\x85\x82\x86\x01a6\xF4V[\x92PP` a;\x12\x85\x82\x86\x01a:\xC7V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x04\x81\x10a;\\Wa;[a;\x1CV[[PV[`\0\x81\x90Pa;m\x82a;KV[\x91\x90PV[`\0a;}\x82a;_V[\x90P\x91\x90PV[a;\x8D\x81a;rV[\x82RPPV[`\x03\x81\x10a;\xA4Wa;\xA3a;\x1CV[[PV[`\0\x81\x90Pa;\xB5\x82a;\x93V[\x91\x90PV[`\0a;\xC5\x82a;\xA7V[\x90P\x91\x90PV[a;\xD5\x81a;\xBAV[\x82RPPV[`\x04\x81\x10a;\xECWa;\xEBa;\x1CV[[PV[`\0\x81\x90Pa;\xFD\x82a;\xDBV[\x91\x90PV[`\0a<\r\x82a;\xEFV[\x90P\x91\x90PV[a<\x1D\x81a<\x02V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a<]W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa<BV[`\0\x84\x84\x01RPPPPV[`\0a<t\x82a<#V[a<~\x81\x85a<.V[\x93Pa<\x8E\x81\x85` \x86\x01a<?V[a<\x97\x81a5XV[\x84\x01\x91PP\x92\x91PPV[a<\xAB\x81a7\xDDV[\x82RPPV[`\x01\x81\x10a<\xC2Wa<\xC1a;\x1CV[[PV[`\0\x81\x90Pa<\xD3\x82a<\xB1V[\x91\x90PV[`\0a<\xE3\x82a<\xC5V[\x90P\x91\x90PV[a<\xF3\x81a<\xD8V[\x82RPPV[`\0a\x01@\x83\x01`\0\x83\x01Qa=\x12`\0\x86\x01\x82a;\x84V[P` \x83\x01Qa=%` \x86\x01\x82a;\xCCV[P`@\x83\x01Qa=8`@\x86\x01\x82a<\x14V[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra=P\x82\x82a<iV[\x91PP`\x80\x83\x01Qa=e`\x80\x86\x01\x82a<\xA2V[P`\xA0\x83\x01Qa=x`\xA0\x86\x01\x82a<\xEAV[P`\xC0\x83\x01Qa=\x8B`\xC0\x86\x01\x82a<\xA2V[P`\xE0\x83\x01Q\x84\x82\x03`\xE0\x86\x01Ra=\xA3\x82\x82a<iV[\x91PPa\x01\0\x83\x01Q\x84\x82\x03a\x01\0\x86\x01Ra=\xBF\x82\x82a<iV[\x91PPa\x01 \x83\x01Q\x84\x82\x03a\x01 \x86\x01Ra=\xDB\x82\x82a<iV[\x91PP\x80\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>\x02\x81\x84a<\xF9V[\x90P\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a>&\x82a<#V[a>0\x81\x85a>\nV[\x93Pa>@\x81\x85` \x86\x01a<?V[a>I\x81a5XV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra>n\x81\x84a>\x1BV[\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a>\x8CWa>\x8Ba3uV[[`\0a>\x9A\x84\x82\x85\x01a:\xC7V[\x91PP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a>\xBDWa>\xBCa3uV[[`\0a>\xCB\x87\x82\x88\x01a7~V[\x94PP` a>\xDC\x87\x82\x88\x01a:\xC7V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xFDWa>\xFCa3zV[[a?\t\x87\x82\x88\x01a6fV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?*Wa?)a3zV[[a?6\x87\x82\x88\x01a6fV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x81\x90P\x92\x91PPV[`\0a?X\x82a<#V[a?b\x81\x85a?BV[\x93Pa?r\x81\x85` \x86\x01a<?V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a?\x8A\x82\x84a?MV[\x91P\x81\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a?\xDCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a?\xEFWa?\xEEa?\x95V[[P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FMust replace active release befo`\0\x82\x01R\x7Fre changing status from Active\0\0` \x82\x01RPV[`\0a@b`>\x83a?\xF5V[\x91Pa@m\x82a@\x06V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra@\x91\x81a@UV[\x90P\x91\x90PV[a@\xA1\x81a;rV[\x82RPPV[`\0`@\x82\x01\x90Pa@\xBC`\0\x83\x01\x85a5$V[a@\xC9` \x83\x01\x84a@\x98V[\x93\x92PPPV[\x7FThe provided subnet (within the `\0\x82\x01R\x7Frelease id) is not valid for thi` \x82\x01R\x7Fs contract\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[`\0aAR`J\x83a?\xF5V[\x91PaA]\x82a@\xD0V[``\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaA\x81\x81aAEV[\x90P\x91\x90PV[\x7FA release with this ID already e`\0\x82\x01R\x7Fxists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aA\xE4`%\x83a?\xF5V[\x91PaA\xEF\x82aA\x88V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaB\x13\x81aA\xD7V[\x90P\x91\x90PV[\x7FThe RO option is required for pr`\0\x82\x01R\x7Fod releases\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aBv`+\x83a?\xF5V[\x91PaB\x81\x82aB\x1AV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaB\xA5\x81aBiV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02aC\x0E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aB\xD1V[aC\x18\x86\x83aB\xD1V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0aCUaCPaCK\x84a7\xDDV[aC0V[a7\xDDV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[aCo\x83aC:V[aC\x83aC{\x82aC\\V[\x84\x84TaB\xDEV[\x82UPPPPV[`\0\x90V[aC\x98aC\x8BV[aC\xA3\x81\x84\x84aCfV[PPPV[[\x81\x81\x10\x15aC\xC7WaC\xBC`\0\x82aC\x90V[`\x01\x81\x01\x90PaC\xA9V[PPV[`\x1F\x82\x11\x15aD\x0CWaC\xDD\x81aB\xACV[aC\xE6\x84aB\xC1V[\x81\x01` \x85\x10\x15aC\xF5W\x81\x90P[aD\taD\x01\x85aB\xC1V[\x83\x01\x82aC\xA8V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0aD/`\0\x19\x84`\x08\x02aD\x11V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0aDH\x83\x83aD\x1EV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aDa\x82a<#V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aDzWaDya5iV[[aD\x84\x82Ta?\xC4V[aD\x8F\x82\x82\x85aC\xCBV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14aD\xC2W`\0\x84\x15aD\xB0W\x82\x87\x01Q\x90P[aD\xBA\x85\x82aD<V[\x86UPaE\"V[`\x1F\x19\x84\x16aD\xD0\x86aB\xACV[`\0[\x82\x81\x10\x15aD\xF8W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaD\xD3V[\x86\x83\x10\x15aE\x15W\x84\x89\x01QaE\x11`\x1F\x89\x16\x82aD\x1EV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[aE3\x81a;\xBAV[\x82RPPV[aEB\x81a<\x02V[\x82RPPV[aEQ\x81a<\xD8V[\x82RPPV[`\0a\x01`\x82\x01\x90PaEm`\0\x83\x01\x8Ea5$V[aEz` \x83\x01\x8Da@\x98V[aE\x87`@\x83\x01\x8CaE*V[aE\x94``\x83\x01\x8BaE9V[\x81\x81\x03`\x80\x83\x01RaE\xA6\x81\x8Aa>\x1BV[\x90PaE\xB5`\xA0\x83\x01\x89a9\xD1V[aE\xC2`\xC0\x83\x01\x88aEHV[aE\xCF`\xE0\x83\x01\x87a9\xD1V[\x81\x81\x03a\x01\0\x83\x01RaE\xE2\x81\x86a>\x1BV[\x90P\x81\x81\x03a\x01 \x83\x01RaE\xF7\x81\x85a>\x1BV[\x90P\x81\x81\x03a\x01@\x83\x01RaF\x0C\x81\x84a>\x1BV[\x90P\x9C\x9BPPPPPPPPPPPPV[`\0\x81`\xF8\x1B\x90P\x91\x90PV[`\0aF6\x82aF\x1EV[\x90P\x91\x90PV[aFNaFI\x82a;\xBAV[aF+V[\x82RPPV[aFeaF`\x82a<\x02V[aF+V[\x82RPPV[aF|aFw\x82a<\xD8V[aF+V[\x82RPPV[`\0aF\x8E\x82\x87aF=V[`\x01\x82\x01\x91PaF\x9E\x82\x86aFTV[`\x01\x82\x01\x91PaF\xAE\x82\x85a?MV[\x91PaF\xBA\x82\x84aFkV[`\x01\x82\x01\x91P\x81\x90P\x95\x94PPPPPV[\x7FAccessControl: can only renounce`\0\x82\x01R\x7F roles for self\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0aG(`/\x83a?\xF5V[\x91PaG3\x82aF\xCCV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaGW\x81aG\x1BV[\x90P\x91\x90PV[`\0` \x82\x01\x90PaGs`\0\x83\x01\x84aE*V[\x92\x91PPV[aG\x82\x81a:\x9EV[\x82RPPV[`\0` \x82\x01\x90PaG\x9D`\0\x83\x01\x84aGyV[\x92\x91PPV[`\0`@\x82\x01\x90P\x81\x81\x03`\0\x83\x01RaG\xBD\x81\x85a>\x1BV[\x90P\x81\x81\x03` \x83\x01RaG\xD1\x81\x84a>\x1BV[\x90P\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0aH\x14\x82a7\xDDV[\x91PaH\x1F\x83a7\xDDV[\x92P\x82\x82\x02aH-\x81a7\xDDV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aHDWaHCaG\xDAV[[P\x92\x91PPV[`\0aHV\x82a7\xDDV[\x91PaHa\x83a7\xDDV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aHyWaHxaG\xDAV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0aH\xB9\x82a7\xDDV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aH\xEBWaH\xEAaG\xDAV[[`\x01\x82\x01\x90P\x91\x90PV[`\0aI\x01\x82a7\xDDV[\x91PaI\x0C\x83a7\xDDV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aI$WaI#aG\xDAV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD[`\0\x81\x90P\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0aI\x9A`\x17\x83aIYV[\x91PaI\xA5\x82aIdV[`\x17\x82\x01\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0aI\xC6\x82aI\xB0V[aI\xD0\x81\x85aIYV[\x93PaI\xE0\x81\x85` \x86\x01a<?V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F is missing role \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0aJ\"`\x11\x83aIYV[\x91PaJ-\x82aI\xECV[`\x11\x82\x01\x90P\x91\x90PV[`\0aJC\x82aI\x8DV[\x91PaJO\x82\x85aI\xBBV[\x91PaJZ\x82aJ\x15V[\x91PaJf\x82\x84aI\xBBV[\x91P\x81\x90P\x93\x92PPPV[`\0aJ}\x82aI\xB0V[aJ\x87\x81\x85a?\xF5V[\x93PaJ\x97\x81\x85` \x86\x01a<?V[aJ\xA0\x81a5XV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaJ\xC5\x81\x84aJrV[\x90P\x92\x91PPV[`\0aJ\xD8\x82a7\xDDV[\x91P`\0\x82\x03aJ\xEBWaJ\xEAaG\xDAV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStrings: hex length insufficient`\0\x82\x01RPV[`\0aK,` \x83a?\xF5V[\x91PaK7\x82aJ\xF6V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01RaK[\x81aK\x1FV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 Zo\x07\xDA\rU\xA7\x9F?\x04\xB0?Y\x81\xBAo}\x15\x8B\xAA^bI\xF7\xE9,1e\xC8H\x80bdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static RELEASEREGISTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ReleaseRegister<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ReleaseRegister<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ReleaseRegister<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ReleaseRegister<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ReleaseRegister<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ReleaseRegister))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ReleaseRegister<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RELEASEREGISTER_ABI.clone(),
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
                RELEASEREGISTER_ABI.clone(),
                RELEASEREGISTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ACTIVATOR_ROLE` (0x08740a3b) function
        pub fn activator_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 116, 10, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ADMIN_ROLE` (0x75b238fc) function
        pub fn admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([117, 178, 56, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BURNER_ROLE` (0x282c51f3) function
        pub fn burner_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([40, 44, 81, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CREATOR_ROLE` (0x8aeda25a) function
        pub fn creator_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([138, 237, 162, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEACTIVATOR_ROLE` (0x458be7dc) function
        pub fn deactivator_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([69, 139, 231, 220], ())
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
        ///Calls the contract's `RELEASE_OPTION_RO` (0xad769394) function
        pub fn release_option_ro(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 118, 147, 148], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELEASE_OPTION_SSH` (0x3acd1ea3) function
        pub fn release_option_ssh(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 205, 30, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELEASE_OPTION_USERS` (0x277dceaf) function
        pub fn release_option_users(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 125, 206, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAllowedAdminSigningPublicKey` (0x70e65a45) function
        pub fn add_allowed_admin_signing_public_key(
            &self,
            pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 230, 90, 69], pub_key)
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
        ///Calls the contract's `addAllowedSubnet` (0xf2dc9916) function
        pub fn add_allowed_subnet(
            &self,
            subnet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 220, 153, 22], subnet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnRelease` (0x2609e586) function
        pub fn burn_release(
            &self,
            release_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 9, 229, 134], release_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createRelease` (0x19724065) function
        pub fn create_release(
            &self,
            release_id: [u8; 32],
            status: u8,
            env: u8,
            typ: u8,
            kind: ::ethers::core::types::Bytes,
            platform: u8,
            options: ::ethers::core::types::U256,
            id_key_digest: ::ethers::core::types::Bytes,
            public_key: ::ethers::core::types::Bytes,
            cid: ::ethers::core::types::Bytes,
            date: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [25, 114, 64, 101],
                    (
                        release_id,
                        status,
                        env,
                        typ,
                        kind,
                        platform,
                        options,
                        id_key_digest,
                        public_key,
                        cid,
                        date,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActiveRelease` (0x2ae79b6d) function
        pub fn get_active_release(
            &self,
            env: u8,
            typ: u8,
            kind: ::ethers::core::types::Bytes,
            platform: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([42, 231, 155, 109], (env, typ, kind, platform))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActiveReleases` (0x023e9288) function
        pub fn get_active_releases(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([2, 62, 146, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCreatorDomain` (0x9bb4e2f7) function
        pub fn get_creator_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([155, 180, 226, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRelease` (0x7f698e92) function
        pub fn get_release(
            &self,
            release_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Release> {
            self.0
                .method_hash([127, 105, 142, 146], release_id)
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
        ///Calls the contract's `hasAllowedAdminSigningPublicKey` (0x0e092b18) function
        pub fn has_allowed_admin_signing_public_key(
            &self,
            pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 9, 43, 24], pub_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAllowedAuthorKeyDigest` (0xd6bc626d) function
        pub fn has_allowed_author_key_digest(
            &self,
            digest: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([214, 188, 98, 109], digest)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAllowedEnv` (0x1bd564dc) function
        pub fn has_allowed_env(
            &self,
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 213, 100, 220], env)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAllowedSubnet` (0xa0908300) function
        pub fn has_allowed_subnet(
            &self,
            subnet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([160, 144, 131, 0], subnet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasCreatorInit` (0x3dc6c858) function
        pub fn has_creator_init(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 198, 200, 88], ())
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
        ///Calls the contract's `initCreator` (0xbc7ca317) function
        pub fn init_creator(
            &self,
            env: u8,
            subnet_id: ::ethers::core::types::Address,
            domain: ::ethers::core::types::Bytes,
            author_key_digest: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [188, 124, 163, 23],
                    (env, subnet_id, domain, author_key_digest),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAllowedAdminSigningPublicKey` (0xe1c0af08) function
        pub fn remove_allowed_admin_signing_public_key(
            &self,
            pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 192, 175, 8], pub_key)
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
        ///Calls the contract's `removeAllowedSubnet` (0xdb2d303b) function
        pub fn remove_allowed_subnet(
            &self,
            subnet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 45, 48, 59], subnet)
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
        ///Calls the contract's `setReleaseStatus` (0x0e1e59dd) function
        pub fn set_release_status(
            &self,
            release_id: [u8; 32],
            status: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 30, 89, 221], (release_id, status))
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
        ///Gets the contract's `AllowedAdminSigningPublicKeyAdded` event
        pub fn allowed_admin_signing_public_key_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAdminSigningPublicKeyAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedAdminSigningPublicKeyRemoved` event
        pub fn allowed_admin_signing_public_key_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAdminSigningPublicKeyRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedAuthorKeyDigestAdded` event
        pub fn allowed_author_key_digest_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAuthorKeyDigestAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedAuthorKeyDigestRemoved` event
        pub fn allowed_author_key_digest_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAuthorKeyDigestRemovedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `AllowedSubnetAdded` event
        pub fn allowed_subnet_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedSubnetAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedSubnetRemoved` event
        pub fn allowed_subnet_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedSubnetRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InitCreator` event
        pub fn init_creator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitCreatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReleaseBurned` event
        pub fn release_burned_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseBurnedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReleaseCreated` event
        pub fn release_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReleaseStatusChange` event
        pub fn release_status_change_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseStatusChangeFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseRegisterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ReleaseRegister<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActivatorRoleRequired` with signature `ActivatorRoleRequired()` and selector `0x5c21123e`
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
    #[etherror(name = "ActivatorRoleRequired", abi = "ActivatorRoleRequired()")]
    pub struct ActivatorRoleRequired;
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
    ///Custom Error type `BurnerRoleRequired` with signature `BurnerRoleRequired()` and selector `0xf4ecad64`
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
    #[etherror(name = "BurnerRoleRequired", abi = "BurnerRoleRequired()")]
    pub struct BurnerRoleRequired;
    ///Custom Error type `CreatorRoleRequired` with signature `CreatorRoleRequired()` and selector `0x80510fe1`
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
    #[etherror(name = "CreatorRoleRequired", abi = "CreatorRoleRequired()")]
    pub struct CreatorRoleRequired;
    ///Custom Error type `DeactivatorRoleRequired` with signature `DeactivatorRoleRequired()` and selector `0x093efaf9`
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
    #[etherror(name = "DeactivatorRoleRequired", abi = "DeactivatorRoleRequired()")]
    pub struct DeactivatorRoleRequired;
    ///Custom Error type `InvalidEnv` with signature `InvalidEnv()` and selector `0xc8e7a97c`
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
    #[etherror(name = "InvalidEnv", abi = "InvalidEnv()")]
    pub struct InvalidEnv;
    ///Custom Error type `InvalidStatus` with signature `InvalidStatus()` and selector `0xf525e320`
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
    #[etherror(name = "InvalidStatus", abi = "InvalidStatus()")]
    pub struct InvalidStatus;
    ///Custom Error type `ReleaseNotFound` with signature `ReleaseNotFound()` and selector `0x28663ff8`
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
    #[etherror(name = "ReleaseNotFound", abi = "ReleaseNotFound()")]
    pub struct ReleaseNotFound;
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
    pub enum ReleaseRegisterErrors {
        ActivatorRoleRequired(ActivatorRoleRequired),
        AdminRoleRequired(AdminRoleRequired),
        BurnerRoleRequired(BurnerRoleRequired),
        CreatorRoleRequired(CreatorRoleRequired),
        DeactivatorRoleRequired(DeactivatorRoleRequired),
        InvalidEnv(InvalidEnv),
        InvalidStatus(InvalidStatus),
        ReleaseNotFound(ReleaseNotFound),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ReleaseRegisterErrors {
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
                = <ActivatorRoleRequired as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ActivatorRoleRequired(decoded));
            }
            if let Ok(decoded)
                = <AdminRoleRequired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminRoleRequired(decoded));
            }
            if let Ok(decoded)
                = <BurnerRoleRequired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BurnerRoleRequired(decoded));
            }
            if let Ok(decoded)
                = <CreatorRoleRequired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatorRoleRequired(decoded));
            }
            if let Ok(decoded)
                = <DeactivatorRoleRequired as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DeactivatorRoleRequired(decoded));
            }
            if let Ok(decoded)
                = <InvalidEnv as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidEnv(decoded));
            }
            if let Ok(decoded)
                = <InvalidStatus as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidStatus(decoded));
            }
            if let Ok(decoded)
                = <ReleaseNotFound as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReleaseNotFound(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReleaseRegisterErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActivatorRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnerRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatorRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivatorRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ReleaseRegisterErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ActivatorRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AdminRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BurnerRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CreatorRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DeactivatorRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEnv as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidStatus as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReleaseNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ReleaseRegisterErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActivatorRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdminRoleRequired(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnerRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreatorRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeactivatorRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ReleaseRegisterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ActivatorRoleRequired> for ReleaseRegisterErrors {
        fn from(value: ActivatorRoleRequired) -> Self {
            Self::ActivatorRoleRequired(value)
        }
    }
    impl ::core::convert::From<AdminRoleRequired> for ReleaseRegisterErrors {
        fn from(value: AdminRoleRequired) -> Self {
            Self::AdminRoleRequired(value)
        }
    }
    impl ::core::convert::From<BurnerRoleRequired> for ReleaseRegisterErrors {
        fn from(value: BurnerRoleRequired) -> Self {
            Self::BurnerRoleRequired(value)
        }
    }
    impl ::core::convert::From<CreatorRoleRequired> for ReleaseRegisterErrors {
        fn from(value: CreatorRoleRequired) -> Self {
            Self::CreatorRoleRequired(value)
        }
    }
    impl ::core::convert::From<DeactivatorRoleRequired> for ReleaseRegisterErrors {
        fn from(value: DeactivatorRoleRequired) -> Self {
            Self::DeactivatorRoleRequired(value)
        }
    }
    impl ::core::convert::From<InvalidEnv> for ReleaseRegisterErrors {
        fn from(value: InvalidEnv) -> Self {
            Self::InvalidEnv(value)
        }
    }
    impl ::core::convert::From<InvalidStatus> for ReleaseRegisterErrors {
        fn from(value: InvalidStatus) -> Self {
            Self::InvalidStatus(value)
        }
    }
    impl ::core::convert::From<ReleaseNotFound> for ReleaseRegisterErrors {
        fn from(value: ReleaseNotFound) -> Self {
            Self::ReleaseNotFound(value)
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
        name = "AllowedAdminSigningPublicKeyAdded",
        abi = "AllowedAdminSigningPublicKeyAdded(bytes)"
    )]
    pub struct AllowedAdminSigningPublicKeyAddedFilter {
        pub pub_key: ::ethers::core::types::Bytes,
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
        name = "AllowedAdminSigningPublicKeyRemoved",
        abi = "AllowedAdminSigningPublicKeyRemoved(bytes)"
    )]
    pub struct AllowedAdminSigningPublicKeyRemovedFilter {
        pub pub_key: ::ethers::core::types::Bytes,
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
        name = "AllowedAuthorKeyDigestAdded",
        abi = "AllowedAuthorKeyDigestAdded(bytes)"
    )]
    pub struct AllowedAuthorKeyDigestAddedFilter {
        pub digest: ::ethers::core::types::Bytes,
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
        name = "AllowedAuthorKeyDigestRemoved",
        abi = "AllowedAuthorKeyDigestRemoved(bytes)"
    )]
    pub struct AllowedAuthorKeyDigestRemovedFilter {
        pub digest: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "AllowedSubnetAdded", abi = "AllowedSubnetAdded(address)")]
    pub struct AllowedSubnetAddedFilter {
        pub subnet: ::ethers::core::types::Address,
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
    #[ethevent(name = "AllowedSubnetRemoved", abi = "AllowedSubnetRemoved(address)")]
    pub struct AllowedSubnetRemovedFilter {
        pub subnet: ::ethers::core::types::Address,
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
    #[ethevent(name = "InitCreator", abi = "InitCreator(bytes,bytes)")]
    pub struct InitCreatorFilter {
        pub domain: ::ethers::core::types::Bytes,
        pub author_key_digest: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "ReleaseBurned", abi = "ReleaseBurned(bytes32)")]
    pub struct ReleaseBurnedFilter {
        pub release_id: [u8; 32],
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
        name = "ReleaseCreated",
        abi = "ReleaseCreated(bytes32,uint8,uint8,uint8,bytes,uint256,uint8,uint256,bytes,bytes,bytes)"
    )]
    pub struct ReleaseCreatedFilter {
        pub release_id: [u8; 32],
        pub status: u8,
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub date: ::ethers::core::types::U256,
        pub platform: u8,
        pub options: ::ethers::core::types::U256,
        pub id_key_digest: ::ethers::core::types::Bytes,
        pub public_key: ::ethers::core::types::Bytes,
        pub cid: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "ReleaseStatusChange", abi = "ReleaseStatusChange(bytes32,uint8)")]
    pub struct ReleaseStatusChangeFilter {
        pub release_id: [u8; 32],
        pub status: u8,
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
    pub enum ReleaseRegisterEvents {
        AllowedAdminSigningPublicKeyAddedFilter(AllowedAdminSigningPublicKeyAddedFilter),
        AllowedAdminSigningPublicKeyRemovedFilter(
            AllowedAdminSigningPublicKeyRemovedFilter,
        ),
        AllowedAuthorKeyDigestAddedFilter(AllowedAuthorKeyDigestAddedFilter),
        AllowedAuthorKeyDigestRemovedFilter(AllowedAuthorKeyDigestRemovedFilter),
        AllowedEnvAddedFilter(AllowedEnvAddedFilter),
        AllowedEnvRemovedFilter(AllowedEnvRemovedFilter),
        AllowedSubnetAddedFilter(AllowedSubnetAddedFilter),
        AllowedSubnetRemovedFilter(AllowedSubnetRemovedFilter),
        InitCreatorFilter(InitCreatorFilter),
        ReleaseBurnedFilter(ReleaseBurnedFilter),
        ReleaseCreatedFilter(ReleaseCreatedFilter),
        ReleaseStatusChangeFilter(ReleaseStatusChangeFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ReleaseRegisterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded)
                = AllowedAdminSigningPublicKeyAddedFilter::decode_log(log) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAdminSigningPublicKeyAddedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = AllowedAdminSigningPublicKeyRemovedFilter::decode_log(log) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAdminSigningPublicKeyRemovedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = AllowedAuthorKeyDigestAddedFilter::decode_log(log) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAuthorKeyDigestAddedFilter(decoded),
                );
            }
            if let Ok(decoded) = AllowedAuthorKeyDigestRemovedFilter::decode_log(log) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAuthorKeyDigestRemovedFilter(decoded),
                );
            }
            if let Ok(decoded) = AllowedEnvAddedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedEnvAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowedEnvRemovedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedEnvRemovedFilter(decoded));
            }
            if let Ok(decoded) = AllowedSubnetAddedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedSubnetAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowedSubnetRemovedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedSubnetRemovedFilter(decoded));
            }
            if let Ok(decoded) = InitCreatorFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::InitCreatorFilter(decoded));
            }
            if let Ok(decoded) = ReleaseBurnedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::ReleaseBurnedFilter(decoded));
            }
            if let Ok(decoded) = ReleaseCreatedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::ReleaseCreatedFilter(decoded));
            }
            if let Ok(decoded) = ReleaseStatusChangeFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::ReleaseStatusChangeFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ReleaseRegisterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowedAdminSigningPublicKeyAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedAdminSigningPublicKeyRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedAuthorKeyDigestAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedAuthorKeyDigestRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedEnvAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedEnvRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedSubnetAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedSubnetRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitCreatorFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseBurnedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReleaseCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReleaseStatusChangeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowedAdminSigningPublicKeyAddedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAdminSigningPublicKeyAddedFilter) -> Self {
            Self::AllowedAdminSigningPublicKeyAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedAdminSigningPublicKeyRemovedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAdminSigningPublicKeyRemovedFilter) -> Self {
            Self::AllowedAdminSigningPublicKeyRemovedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedAuthorKeyDigestAddedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAuthorKeyDigestAddedFilter) -> Self {
            Self::AllowedAuthorKeyDigestAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedAuthorKeyDigestRemovedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAuthorKeyDigestRemovedFilter) -> Self {
            Self::AllowedAuthorKeyDigestRemovedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedEnvAddedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedEnvAddedFilter) -> Self {
            Self::AllowedEnvAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedEnvRemovedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedEnvRemovedFilter) -> Self {
            Self::AllowedEnvRemovedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedSubnetAddedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedSubnetAddedFilter) -> Self {
            Self::AllowedSubnetAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedSubnetRemovedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedSubnetRemovedFilter) -> Self {
            Self::AllowedSubnetRemovedFilter(value)
        }
    }
    impl ::core::convert::From<InitCreatorFilter> for ReleaseRegisterEvents {
        fn from(value: InitCreatorFilter) -> Self {
            Self::InitCreatorFilter(value)
        }
    }
    impl ::core::convert::From<ReleaseBurnedFilter> for ReleaseRegisterEvents {
        fn from(value: ReleaseBurnedFilter) -> Self {
            Self::ReleaseBurnedFilter(value)
        }
    }
    impl ::core::convert::From<ReleaseCreatedFilter> for ReleaseRegisterEvents {
        fn from(value: ReleaseCreatedFilter) -> Self {
            Self::ReleaseCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ReleaseStatusChangeFilter> for ReleaseRegisterEvents {
        fn from(value: ReleaseStatusChangeFilter) -> Self {
            Self::ReleaseStatusChangeFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ReleaseRegisterEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ReleaseRegisterEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ReleaseRegisterEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ACTIVATOR_ROLE` function with signature `ACTIVATOR_ROLE()` and selector `0x08740a3b`
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
    #[ethcall(name = "ACTIVATOR_ROLE", abi = "ACTIVATOR_ROLE()")]
    pub struct ActivatorRoleCall;
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
    ///Container type for all input parameters for the `BURNER_ROLE` function with signature `BURNER_ROLE()` and selector `0x282c51f3`
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
    #[ethcall(name = "BURNER_ROLE", abi = "BURNER_ROLE()")]
    pub struct BurnerRoleCall;
    ///Container type for all input parameters for the `CREATOR_ROLE` function with signature `CREATOR_ROLE()` and selector `0x8aeda25a`
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
    #[ethcall(name = "CREATOR_ROLE", abi = "CREATOR_ROLE()")]
    pub struct CreatorRoleCall;
    ///Container type for all input parameters for the `DEACTIVATOR_ROLE` function with signature `DEACTIVATOR_ROLE()` and selector `0x458be7dc`
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
    #[ethcall(name = "DEACTIVATOR_ROLE", abi = "DEACTIVATOR_ROLE()")]
    pub struct DeactivatorRoleCall;
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
    ///Container type for all input parameters for the `RELEASE_OPTION_RO` function with signature `RELEASE_OPTION_RO()` and selector `0xad769394`
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
    #[ethcall(name = "RELEASE_OPTION_RO", abi = "RELEASE_OPTION_RO()")]
    pub struct ReleaseOptionRoCall;
    ///Container type for all input parameters for the `RELEASE_OPTION_SSH` function with signature `RELEASE_OPTION_SSH()` and selector `0x3acd1ea3`
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
    #[ethcall(name = "RELEASE_OPTION_SSH", abi = "RELEASE_OPTION_SSH()")]
    pub struct ReleaseOptionSshCall;
    ///Container type for all input parameters for the `RELEASE_OPTION_USERS` function with signature `RELEASE_OPTION_USERS()` and selector `0x277dceaf`
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
    #[ethcall(name = "RELEASE_OPTION_USERS", abi = "RELEASE_OPTION_USERS()")]
    pub struct ReleaseOptionUsersCall;
    ///Container type for all input parameters for the `addAllowedAdminSigningPublicKey` function with signature `addAllowedAdminSigningPublicKey(bytes)` and selector `0x70e65a45`
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
        name = "addAllowedAdminSigningPublicKey",
        abi = "addAllowedAdminSigningPublicKey(bytes)"
    )]
    pub struct AddAllowedAdminSigningPublicKeyCall {
        pub pub_key: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `addAllowedSubnet` function with signature `addAllowedSubnet(address)` and selector `0xf2dc9916`
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
    #[ethcall(name = "addAllowedSubnet", abi = "addAllowedSubnet(address)")]
    pub struct AddAllowedSubnetCall {
        pub subnet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burnRelease` function with signature `burnRelease(bytes32)` and selector `0x2609e586`
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
    #[ethcall(name = "burnRelease", abi = "burnRelease(bytes32)")]
    pub struct BurnReleaseCall {
        pub release_id: [u8; 32],
    }
    ///Container type for all input parameters for the `createRelease` function with signature `createRelease(bytes32,uint8,uint8,uint8,bytes,uint8,uint256,bytes,bytes,bytes,uint256)` and selector `0x19724065`
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
        name = "createRelease",
        abi = "createRelease(bytes32,uint8,uint8,uint8,bytes,uint8,uint256,bytes,bytes,bytes,uint256)"
    )]
    pub struct CreateReleaseCall {
        pub release_id: [u8; 32],
        pub status: u8,
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub platform: u8,
        pub options: ::ethers::core::types::U256,
        pub id_key_digest: ::ethers::core::types::Bytes,
        pub public_key: ::ethers::core::types::Bytes,
        pub cid: ::ethers::core::types::Bytes,
        pub date: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getActiveRelease` function with signature `getActiveRelease(uint8,uint8,bytes,uint8)` and selector `0x2ae79b6d`
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
        name = "getActiveRelease",
        abi = "getActiveRelease(uint8,uint8,bytes,uint8)"
    )]
    pub struct GetActiveReleaseCall {
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub platform: u8,
    }
    ///Container type for all input parameters for the `getActiveReleases` function with signature `getActiveReleases()` and selector `0x023e9288`
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
    #[ethcall(name = "getActiveReleases", abi = "getActiveReleases()")]
    pub struct GetActiveReleasesCall;
    ///Container type for all input parameters for the `getCreatorDomain` function with signature `getCreatorDomain()` and selector `0x9bb4e2f7`
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
    #[ethcall(name = "getCreatorDomain", abi = "getCreatorDomain()")]
    pub struct GetCreatorDomainCall;
    ///Container type for all input parameters for the `getRelease` function with signature `getRelease(bytes32)` and selector `0x7f698e92`
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
    #[ethcall(name = "getRelease", abi = "getRelease(bytes32)")]
    pub struct GetReleaseCall {
        pub release_id: [u8; 32],
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
    ///Container type for all input parameters for the `hasAllowedAdminSigningPublicKey` function with signature `hasAllowedAdminSigningPublicKey(bytes)` and selector `0x0e092b18`
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
        name = "hasAllowedAdminSigningPublicKey",
        abi = "hasAllowedAdminSigningPublicKey(bytes)"
    )]
    pub struct HasAllowedAdminSigningPublicKeyCall {
        pub pub_key: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasAllowedAuthorKeyDigest` function with signature `hasAllowedAuthorKeyDigest(bytes)` and selector `0xd6bc626d`
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
        name = "hasAllowedAuthorKeyDigest",
        abi = "hasAllowedAuthorKeyDigest(bytes)"
    )]
    pub struct HasAllowedAuthorKeyDigestCall {
        pub digest: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasAllowedEnv` function with signature `hasAllowedEnv(uint8)` and selector `0x1bd564dc`
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
    #[ethcall(name = "hasAllowedEnv", abi = "hasAllowedEnv(uint8)")]
    pub struct HasAllowedEnvCall {
        pub env: u8,
    }
    ///Container type for all input parameters for the `hasAllowedSubnet` function with signature `hasAllowedSubnet(address)` and selector `0xa0908300`
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
    #[ethcall(name = "hasAllowedSubnet", abi = "hasAllowedSubnet(address)")]
    pub struct HasAllowedSubnetCall {
        pub subnet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasCreatorInit` function with signature `hasCreatorInit()` and selector `0x3dc6c858`
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
    #[ethcall(name = "hasCreatorInit", abi = "hasCreatorInit()")]
    pub struct HasCreatorInitCall;
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
    ///Container type for all input parameters for the `initCreator` function with signature `initCreator(uint8,address,bytes,bytes)` and selector `0xbc7ca317`
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
    #[ethcall(name = "initCreator", abi = "initCreator(uint8,address,bytes,bytes)")]
    pub struct InitCreatorCall {
        pub env: u8,
        pub subnet_id: ::ethers::core::types::Address,
        pub domain: ::ethers::core::types::Bytes,
        pub author_key_digest: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `removeAllowedAdminSigningPublicKey` function with signature `removeAllowedAdminSigningPublicKey(bytes)` and selector `0xe1c0af08`
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
        name = "removeAllowedAdminSigningPublicKey",
        abi = "removeAllowedAdminSigningPublicKey(bytes)"
    )]
    pub struct RemoveAllowedAdminSigningPublicKeyCall {
        pub pub_key: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `removeAllowedSubnet` function with signature `removeAllowedSubnet(address)` and selector `0xdb2d303b`
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
    #[ethcall(name = "removeAllowedSubnet", abi = "removeAllowedSubnet(address)")]
    pub struct RemoveAllowedSubnetCall {
        pub subnet: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setReleaseStatus` function with signature `setReleaseStatus(bytes32,uint8)` and selector `0x0e1e59dd`
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
    #[ethcall(name = "setReleaseStatus", abi = "setReleaseStatus(bytes32,uint8)")]
    pub struct SetReleaseStatusCall {
        pub release_id: [u8; 32],
        pub status: u8,
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
    pub enum ReleaseRegisterCalls {
        ActivatorRole(ActivatorRoleCall),
        AdminRole(AdminRoleCall),
        BurnerRole(BurnerRoleCall),
        CreatorRole(CreatorRoleCall),
        DeactivatorRole(DeactivatorRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        ReleaseOptionRo(ReleaseOptionRoCall),
        ReleaseOptionSsh(ReleaseOptionSshCall),
        ReleaseOptionUsers(ReleaseOptionUsersCall),
        AddAllowedAdminSigningPublicKey(AddAllowedAdminSigningPublicKeyCall),
        AddAllowedEnv(AddAllowedEnvCall),
        AddAllowedSubnet(AddAllowedSubnetCall),
        BurnRelease(BurnReleaseCall),
        CreateRelease(CreateReleaseCall),
        GetActiveRelease(GetActiveReleaseCall),
        GetActiveReleases(GetActiveReleasesCall),
        GetCreatorDomain(GetCreatorDomainCall),
        GetRelease(GetReleaseCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasAllowedAdminSigningPublicKey(HasAllowedAdminSigningPublicKeyCall),
        HasAllowedAuthorKeyDigest(HasAllowedAuthorKeyDigestCall),
        HasAllowedEnv(HasAllowedEnvCall),
        HasAllowedSubnet(HasAllowedSubnetCall),
        HasCreatorInit(HasCreatorInitCall),
        HasRole(HasRoleCall),
        InitCreator(InitCreatorCall),
        RemoveAllowedAdminSigningPublicKey(RemoveAllowedAdminSigningPublicKeyCall),
        RemoveAllowedEnv(RemoveAllowedEnvCall),
        RemoveAllowedSubnet(RemoveAllowedSubnetCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetReleaseStatus(SetReleaseStatusCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ReleaseRegisterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ActivatorRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActivatorRole(decoded));
            }
            if let Ok(decoded)
                = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded)
                = <BurnerRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BurnerRole(decoded));
            }
            if let Ok(decoded)
                = <CreatorRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatorRole(decoded));
            }
            if let Ok(decoded)
                = <DeactivatorRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeactivatorRole(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <ReleaseOptionRoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReleaseOptionRo(decoded));
            }
            if let Ok(decoded)
                = <ReleaseOptionSshCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReleaseOptionSsh(decoded));
            }
            if let Ok(decoded)
                = <ReleaseOptionUsersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReleaseOptionUsers(decoded));
            }
            if let Ok(decoded)
                = <AddAllowedAdminSigningPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddAllowedAdminSigningPublicKey(decoded));
            }
            if let Ok(decoded)
                = <AddAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddAllowedEnv(decoded));
            }
            if let Ok(decoded)
                = <AddAllowedSubnetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddAllowedSubnet(decoded));
            }
            if let Ok(decoded)
                = <BurnReleaseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BurnRelease(decoded));
            }
            if let Ok(decoded)
                = <CreateReleaseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateRelease(decoded));
            }
            if let Ok(decoded)
                = <GetActiveReleaseCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetActiveRelease(decoded));
            }
            if let Ok(decoded)
                = <GetActiveReleasesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetActiveReleases(decoded));
            }
            if let Ok(decoded)
                = <GetCreatorDomainCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetCreatorDomain(decoded));
            }
            if let Ok(decoded)
                = <GetReleaseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRelease(decoded));
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
                = <HasAllowedAdminSigningPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HasAllowedAdminSigningPublicKey(decoded));
            }
            if let Ok(decoded)
                = <HasAllowedAuthorKeyDigestCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HasAllowedAuthorKeyDigest(decoded));
            }
            if let Ok(decoded)
                = <HasAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasAllowedEnv(decoded));
            }
            if let Ok(decoded)
                = <HasAllowedSubnetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HasAllowedSubnet(decoded));
            }
            if let Ok(decoded)
                = <HasCreatorInitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasCreatorInit(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <InitCreatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitCreator(decoded));
            }
            if let Ok(decoded)
                = <RemoveAllowedAdminSigningPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveAllowedAdminSigningPublicKey(decoded));
            }
            if let Ok(decoded)
                = <RemoveAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveAllowedEnv(decoded));
            }
            if let Ok(decoded)
                = <RemoveAllowedSubnetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveAllowedSubnet(decoded));
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
                = <SetReleaseStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReleaseStatus(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ReleaseRegisterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ActivatorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivatorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseOptionRo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseOptionSsh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseOptionUsers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedAdminSigningPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActiveRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActiveReleases(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCreatorDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedAdminSigningPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedAuthorKeyDigest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasCreatorInit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitCreator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedAdminSigningPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReleaseStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ReleaseRegisterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActivatorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeactivatorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseOptionRo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseOptionSsh(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseOptionUsers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddAllowedAdminSigningPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAllowedSubnet(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActiveRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActiveReleases(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCreatorDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasAllowedAdminSigningPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasAllowedAuthorKeyDigest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasAllowedSubnet(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasCreatorInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitCreator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAllowedAdminSigningPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAllowedSubnet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReleaseStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ActivatorRoleCall> for ReleaseRegisterCalls {
        fn from(value: ActivatorRoleCall) -> Self {
            Self::ActivatorRole(value)
        }
    }
    impl ::core::convert::From<AdminRoleCall> for ReleaseRegisterCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<BurnerRoleCall> for ReleaseRegisterCalls {
        fn from(value: BurnerRoleCall) -> Self {
            Self::BurnerRole(value)
        }
    }
    impl ::core::convert::From<CreatorRoleCall> for ReleaseRegisterCalls {
        fn from(value: CreatorRoleCall) -> Self {
            Self::CreatorRole(value)
        }
    }
    impl ::core::convert::From<DeactivatorRoleCall> for ReleaseRegisterCalls {
        fn from(value: DeactivatorRoleCall) -> Self {
            Self::DeactivatorRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ReleaseRegisterCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<ReleaseOptionRoCall> for ReleaseRegisterCalls {
        fn from(value: ReleaseOptionRoCall) -> Self {
            Self::ReleaseOptionRo(value)
        }
    }
    impl ::core::convert::From<ReleaseOptionSshCall> for ReleaseRegisterCalls {
        fn from(value: ReleaseOptionSshCall) -> Self {
            Self::ReleaseOptionSsh(value)
        }
    }
    impl ::core::convert::From<ReleaseOptionUsersCall> for ReleaseRegisterCalls {
        fn from(value: ReleaseOptionUsersCall) -> Self {
            Self::ReleaseOptionUsers(value)
        }
    }
    impl ::core::convert::From<AddAllowedAdminSigningPublicKeyCall>
    for ReleaseRegisterCalls {
        fn from(value: AddAllowedAdminSigningPublicKeyCall) -> Self {
            Self::AddAllowedAdminSigningPublicKey(value)
        }
    }
    impl ::core::convert::From<AddAllowedEnvCall> for ReleaseRegisterCalls {
        fn from(value: AddAllowedEnvCall) -> Self {
            Self::AddAllowedEnv(value)
        }
    }
    impl ::core::convert::From<AddAllowedSubnetCall> for ReleaseRegisterCalls {
        fn from(value: AddAllowedSubnetCall) -> Self {
            Self::AddAllowedSubnet(value)
        }
    }
    impl ::core::convert::From<BurnReleaseCall> for ReleaseRegisterCalls {
        fn from(value: BurnReleaseCall) -> Self {
            Self::BurnRelease(value)
        }
    }
    impl ::core::convert::From<CreateReleaseCall> for ReleaseRegisterCalls {
        fn from(value: CreateReleaseCall) -> Self {
            Self::CreateRelease(value)
        }
    }
    impl ::core::convert::From<GetActiveReleaseCall> for ReleaseRegisterCalls {
        fn from(value: GetActiveReleaseCall) -> Self {
            Self::GetActiveRelease(value)
        }
    }
    impl ::core::convert::From<GetActiveReleasesCall> for ReleaseRegisterCalls {
        fn from(value: GetActiveReleasesCall) -> Self {
            Self::GetActiveReleases(value)
        }
    }
    impl ::core::convert::From<GetCreatorDomainCall> for ReleaseRegisterCalls {
        fn from(value: GetCreatorDomainCall) -> Self {
            Self::GetCreatorDomain(value)
        }
    }
    impl ::core::convert::From<GetReleaseCall> for ReleaseRegisterCalls {
        fn from(value: GetReleaseCall) -> Self {
            Self::GetRelease(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ReleaseRegisterCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ReleaseRegisterCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasAllowedAdminSigningPublicKeyCall>
    for ReleaseRegisterCalls {
        fn from(value: HasAllowedAdminSigningPublicKeyCall) -> Self {
            Self::HasAllowedAdminSigningPublicKey(value)
        }
    }
    impl ::core::convert::From<HasAllowedAuthorKeyDigestCall> for ReleaseRegisterCalls {
        fn from(value: HasAllowedAuthorKeyDigestCall) -> Self {
            Self::HasAllowedAuthorKeyDigest(value)
        }
    }
    impl ::core::convert::From<HasAllowedEnvCall> for ReleaseRegisterCalls {
        fn from(value: HasAllowedEnvCall) -> Self {
            Self::HasAllowedEnv(value)
        }
    }
    impl ::core::convert::From<HasAllowedSubnetCall> for ReleaseRegisterCalls {
        fn from(value: HasAllowedSubnetCall) -> Self {
            Self::HasAllowedSubnet(value)
        }
    }
    impl ::core::convert::From<HasCreatorInitCall> for ReleaseRegisterCalls {
        fn from(value: HasCreatorInitCall) -> Self {
            Self::HasCreatorInit(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ReleaseRegisterCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitCreatorCall> for ReleaseRegisterCalls {
        fn from(value: InitCreatorCall) -> Self {
            Self::InitCreator(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedAdminSigningPublicKeyCall>
    for ReleaseRegisterCalls {
        fn from(value: RemoveAllowedAdminSigningPublicKeyCall) -> Self {
            Self::RemoveAllowedAdminSigningPublicKey(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedEnvCall> for ReleaseRegisterCalls {
        fn from(value: RemoveAllowedEnvCall) -> Self {
            Self::RemoveAllowedEnv(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedSubnetCall> for ReleaseRegisterCalls {
        fn from(value: RemoveAllowedSubnetCall) -> Self {
            Self::RemoveAllowedSubnet(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ReleaseRegisterCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ReleaseRegisterCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetReleaseStatusCall> for ReleaseRegisterCalls {
        fn from(value: SetReleaseStatusCall) -> Self {
            Self::SetReleaseStatus(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ReleaseRegisterCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `ACTIVATOR_ROLE` function with signature `ACTIVATOR_ROLE()` and selector `0x08740a3b`
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
    pub struct ActivatorRoleReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `BURNER_ROLE` function with signature `BURNER_ROLE()` and selector `0x282c51f3`
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
    pub struct BurnerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CREATOR_ROLE` function with signature `CREATOR_ROLE()` and selector `0x8aeda25a`
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
    pub struct CreatorRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEACTIVATOR_ROLE` function with signature `DEACTIVATOR_ROLE()` and selector `0x458be7dc`
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
    pub struct DeactivatorRoleReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `RELEASE_OPTION_RO` function with signature `RELEASE_OPTION_RO()` and selector `0xad769394`
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
    pub struct ReleaseOptionRoReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RELEASE_OPTION_SSH` function with signature `RELEASE_OPTION_SSH()` and selector `0x3acd1ea3`
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
    pub struct ReleaseOptionSshReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RELEASE_OPTION_USERS` function with signature `RELEASE_OPTION_USERS()` and selector `0x277dceaf`
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
    pub struct ReleaseOptionUsersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getActiveRelease` function with signature `getActiveRelease(uint8,uint8,bytes,uint8)` and selector `0x2ae79b6d`
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
    pub struct GetActiveReleaseReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getActiveReleases` function with signature `getActiveReleases()` and selector `0x023e9288`
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
    pub struct GetActiveReleasesReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getCreatorDomain` function with signature `getCreatorDomain()` and selector `0x9bb4e2f7`
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
    pub struct GetCreatorDomainReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getRelease` function with signature `getRelease(bytes32)` and selector `0x7f698e92`
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
    pub struct GetReleaseReturn(pub Release);
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
    ///Container type for all return fields from the `hasAllowedAdminSigningPublicKey` function with signature `hasAllowedAdminSigningPublicKey(bytes)` and selector `0x0e092b18`
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
    pub struct HasAllowedAdminSigningPublicKeyReturn(pub bool);
    ///Container type for all return fields from the `hasAllowedAuthorKeyDigest` function with signature `hasAllowedAuthorKeyDigest(bytes)` and selector `0xd6bc626d`
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
    pub struct HasAllowedAuthorKeyDigestReturn(pub bool);
    ///Container type for all return fields from the `hasAllowedEnv` function with signature `hasAllowedEnv(uint8)` and selector `0x1bd564dc`
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
    pub struct HasAllowedEnvReturn(pub bool);
    ///Container type for all return fields from the `hasAllowedSubnet` function with signature `hasAllowedSubnet(address)` and selector `0xa0908300`
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
    pub struct HasAllowedSubnetReturn(pub bool);
    ///Container type for all return fields from the `hasCreatorInit` function with signature `hasCreatorInit()` and selector `0x3dc6c858`
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
    pub struct HasCreatorInitReturn(pub bool);
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
    ///`Release(uint8,uint8,uint8,bytes,uint256,uint8,uint256,bytes,bytes,bytes)`
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
    pub struct Release {
        pub status: u8,
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub date: ::ethers::core::types::U256,
        pub platform: u8,
        pub options: ::ethers::core::types::U256,
        pub id_key_digest: ::ethers::core::types::Bytes,
        pub public_key: ::ethers::core::types::Bytes,
        pub cid: ::ethers::core::types::Bytes,
    }
}
