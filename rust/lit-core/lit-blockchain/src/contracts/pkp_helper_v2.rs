pub use pkp_helper_v2::*;
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
pub mod pkp_helper_v2 {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/PKPHelperV2.json",
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
                    ::std::borrow::ToOwned::to_owned("getDomainWalletRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDomainWalletRegistry",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getPKPNftMetdataAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPKPNftMetdataAddress",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getPkpNftAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPkpNftAddress"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getPkpPermissionsAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPkpPermissionsAddress",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("mintNextAndAddAuthMethods"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "mintNextAndAddAuthMethods",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PKPHelperV2.NewPKPParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "mintNextAndAddAuthMethodsWithTypes",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "mintNextAndAddAuthMethodsWithTypes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PKPHelperV2.NewPKPParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
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
                                        "newResolverAddress",
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ContractResolverAddressSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractResolverAddressSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newResolverAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
    pub static PKPHELPERV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0%\08\x03\x80b\0%\0\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x02+V[b\0\0Wb\0\0Kb\0\0\xCD` \x1B` \x1CV[b\0\0\xD5` \x1B` \x1CV[\x81`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01`\x14a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x02\x81\x11\x15b\0\0\xC0Wb\0\0\xBFb\0\x02rV[[\x02\x17\x90UPPPb\0\x02\xA1V[`\x003\x90P\x90V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x01\xCB\x82b\0\x01\x9EV[\x90P\x91\x90PV[b\0\x01\xDD\x81b\0\x01\xBEV[\x81\x14b\0\x01\xE9W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x01\xFD\x81b\0\x01\xD2V[\x92\x91PPV[`\x03\x81\x10b\0\x02\x11W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x02%\x81b\0\x02\x03V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x02EWb\0\x02Db\0\x01\x99V[[`\0b\0\x02U\x85\x82\x86\x01b\0\x01\xECV[\x92PP` b\0\x02h\x85\x82\x86\x01b\0\x02\x14V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[a\"O\x80b\0\x02\xB1`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xC2W`\x005`\xE0\x1C\x80cs\xCCA\x11\x11a\0\x7FW\x80c\xB3%\x97\xB8\x11a\0YW\x80c\xB3%\x97\xB8\x14a\x02MW\x80c\xCA\xEA\xD0\xC7\x14a\x02}W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xA8W\x80c\xF9]q\xB1\x14a\x02\xD1Wa\0\xC2V[\x80cs\xCCA\x11\x14a\x01\xCCW\x80c\x8D\xA5\xCB[\x14a\x01\xF7W\x80c\x9D\xCA\x002\x14a\x02\"Wa\0\xC2V[\x80c\x15\x0Bz\x02\x14a\0\xC7W\x80c2vU\x8C\x14a\x01\x04W\x80c5\xFD\xBC@\x14a\x01/W\x80cPC\x02l\x14a\x01_W\x80cP\xD1{^\x14a\x01\x8AW\x80cqP\x18\xA6\x14a\x01\xB5W[`\0\x80\xFD[4\x80\x15a\0\xD3W`\0\x80\xFD[Pa\0\xEE`\x04\x806\x03\x81\x01\x90a\0\xE9\x91\x90a\x12qV[a\x02\xFAV[`@Qa\0\xFB\x91\x90a\x134V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x10W`\0\x80\xFD[Pa\x01\x19a\x03\x84V[`@Qa\x01&\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[a\x01I`\x04\x806\x03\x81\x01\x90a\x01D\x91\x90a\x19\x03V[a\x04\xC8V[`@Qa\x01V\x91\x90a\x19[V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01kW`\0\x80\xFD[Pa\x01ta\n\xC0V[`@Qa\x01\x81\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x96W`\0\x80\xFD[Pa\x01\x9Fa\x0C\x04V[`@Qa\x01\xAC\x91\x90a\x19\xD5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xC1W`\0\x80\xFD[Pa\x01\xCAa\x0C*V[\0[4\x80\x15a\x01\xD8W`\0\x80\xFD[Pa\x01\xE1a\x0C>V[`@Qa\x01\xEE\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x03W`\0\x80\xFD[Pa\x02\x0Ca\r\x82V[`@Qa\x02\x19\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02.W`\0\x80\xFD[Pa\x027a\r\xABV[`@Qa\x02D\x91\x90a\x1AgV[`@Q\x80\x91\x03\x90\xF3[a\x02g`\x04\x806\x03\x81\x01\x90a\x02b\x91\x90a\x19\x03V[a\r\xBEV[`@Qa\x02t\x91\x90a\x19[V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x89W`\0\x80\xFD[Pa\x02\x92a\r\xD0V[`@Qa\x02\x9F\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB4W`\0\x80\xFD[Pa\x02\xCF`\x04\x806\x03\x81\x01\x90a\x02\xCA\x91\x90a\x1A\x82V[a\x0F\x14V[\0[4\x80\x15a\x02\xDDW`\0\x80\xFD[Pa\x02\xF8`\x04\x806\x03\x81\x01\x90a\x02\xF3\x91\x90a\x1A\x82V[a\x0F\x97V[\0[`\0a\x03\x04a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03qW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03h\x90a\x1B2V[`@Q\x80\x91\x03\x90\xFD[c\x15\x0Bz\x02`\xE0\x1B\x90P\x95\x94PPPPPV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90r\xF88`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x041W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04U\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x82\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC3\x91\x90a\x1C\x02V[\x90P\x90V[`\0\x80a\x04\xD3a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c]\"\x8B\x164\x85`\0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x10\x91\x90a\x19[V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x05.W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05S\x91\x90a\x1CDV[\x90P\x82`@\x01QQ\x83` \x01QQ\x14a\x05\xA1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x98\x90a\x1C\xE3V[`@Q\x80\x91\x03\x90\xFD[\x82``\x01QQ\x83` \x01QQ\x14a\x05\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xE4\x90a\x1DuV[`@Q\x80\x91\x03\x90\xFD[\x82`\x80\x01QQ\x83` \x01QQ\x14a\x069W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x060\x90a\x1E\x07V[`@Q\x80\x91\x03\x90\xFD[`\0\x83` \x01QQ\x14a\x07sW`\0[\x83` \x01QQ\x81\x10\x15a\x07qWa\x06^a\x03\x84V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x88` \x01Q\x86\x81Q\x81\x10a\x06\x9CWa\x06\x9Ba\x1E'V[[` \x02` \x01\x01Q\x81R` \x01\x88`@\x01Q\x86\x81Q\x81\x10a\x06\xC0Wa\x06\xBFa\x1E'V[[` \x02` \x01\x01Q\x81R` \x01\x88``\x01Q\x86\x81Q\x81\x10a\x06\xE4Wa\x06\xE3a\x1E'V[[` \x02` \x01\x01Q\x81RP\x87`\x80\x01Q\x85\x81Q\x81\x10a\x07\x06Wa\x07\x05a\x1E'V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07,\x93\x92\x91\x90a\x1F\xEAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07ZW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x07i\x90a ^V[\x91PPa\x06IV[P[`\0a\x07}a\x03\x84V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xB5\x91\x90a\x19[V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF6\x91\x90a\x1C\x02V[\x90P\x83`\xA0\x01Q\x15a\x08}Wa\x08\na\x03\x84V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x16c\xC1!\x83\x83\x87`\xC0\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08J\x93\x92\x91\x90a \xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08dW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08xW=`\0\x80>=`\0\xFD[PPPP[\x83`\xE0\x01Q\x15a\t\x02Wa\x08\x8Fa\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xCB\x93\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xF9W=`\0\x80>=`\0\xFD[PPPPa\n\xB6V[\x83a\x01\0\x01Q\x15a\t\x84Wa\t\x15a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x96lh\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tM\x91\x90a\x19[V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tgW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t{W=`\0\x80>=`\0\xFD[PPPPa\n\xB5V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84a\x01 \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n=Wa\t\xC5a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x84.\x0E0\x86a\x01 \x01Q\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x06\x93\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n4W=`\0\x80>=`\0\xFD[PPPPa\n\xB4V[a\nEa\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x81\x93\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xAFW=`\0\x80>=`\0\xFD[PPPP[[[\x81\x92PPP\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x16\xF7k\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x91\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xBE\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xFF\x91\x90a\x1C\x02V[\x90P\x90V[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x0C2a\x10\x1AV[a\x0C<`\0a\x10\x98V[V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x16\xE7:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x0F\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r<\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rYW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r}\x91\x90a\x1C\x02V[\x90P\x90V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0a\r\xC9\x82a\x04\xC8V[\x90P\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c,\x0B\x8B\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA1\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xCE\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x0F\x91\x90a\x1C\x02V[\x90P\x90V[a\x0F\x1Ca\x10\x1AV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0F\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x82\x90a!\x8DV[`@Q\x80\x91\x03\x90\xFD[a\x0F\x94\x81a\x10\x98V[PV[a\x0F\x9Fa\x10\x1AV[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F'`\x07<|\xD8\xCA\xC51\xD7\xF6C\xBE\xCB\xFB\xB7M\x8B\x81VD>\xAC\xF8yb%2\xDB\xBB<\xD5\x81`@Qa\x10\x0F\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xA1PV[a\x10\"a\x11\\V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x10@a\r\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10\x96W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x8D\x90a!\xF9V[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x003\x90P\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x11\xA3\x82a\x11xV[\x90P\x91\x90PV[a\x11\xB3\x81a\x11\x98V[\x81\x14a\x11\xBEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x11\xD0\x81a\x11\xAAV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x11\xE9\x81a\x11\xD6V[\x81\x14a\x11\xF4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\x06\x81a\x11\xE0V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x121Wa\x120a\x12\x0CV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12NWa\x12Ma\x12\x11V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x12jWa\x12ia\x12\x16V[[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x12\x8DWa\x12\x8Ca\x11nV[[`\0a\x12\x9B\x88\x82\x89\x01a\x11\xC1V[\x95PP` a\x12\xAC\x88\x82\x89\x01a\x11\xC1V[\x94PP`@a\x12\xBD\x88\x82\x89\x01a\x11\xF7V[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xDEWa\x12\xDDa\x11sV[[a\x12\xEA\x88\x82\x89\x01a\x12\x1BV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x13.\x81a\x12\xF9V[\x82RPPV[`\0` \x82\x01\x90Pa\x13I`\0\x83\x01\x84a\x13%V[\x92\x91PPV[a\x13X\x81a\x11\x98V[\x82RPPV[`\0` \x82\x01\x90Pa\x13s`\0\x83\x01\x84a\x13OV[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x13\xC7\x82a\x13~V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13\xE6Wa\x13\xE5a\x13\x8FV[[\x80`@RPPPV[`\0a\x13\xF9a\x11dV[\x90Pa\x14\x05\x82\x82a\x13\xBEV[\x91\x90PV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14*Wa\x14)a\x13\x8FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x14Na\x14I\x84a\x14\x0FV[a\x13\xEFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x14qWa\x14pa\x12\x16V[[\x83[\x81\x81\x10\x15a\x14\x9AW\x80a\x14\x86\x88\x82a\x11\xF7V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x14sV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14\xB9Wa\x14\xB8a\x12\x0CV[[\x815a\x14\xC9\x84\x82` \x86\x01a\x14;V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\xEDWa\x14\xECa\x13\x8FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\x1EWa\x15\x1Da\x13\x8FV[[a\x15'\x82a\x13~V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x15Va\x15Q\x84a\x15\x03V[a\x13\xEFV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x15rWa\x15qa\x14\xFEV[[a\x15}\x84\x82\x85a\x154V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x15\x9AWa\x15\x99a\x12\x0CV[[\x815a\x15\xAA\x84\x82` \x86\x01a\x15CV[\x91PP\x92\x91PPV[`\0a\x15\xC6a\x15\xC1\x84a\x14\xD2V[a\x13\xEFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x15\xE9Wa\x15\xE8a\x12\x16V[[\x83[\x81\x81\x10\x15a\x160W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x0EWa\x16\ra\x12\x0CV[[\x80\x86\x01a\x16\x1B\x89\x82a\x15\x85V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x15\xEBV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x16OWa\x16Na\x12\x0CV[[\x815a\x16_\x84\x82` \x86\x01a\x15\xB3V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x16\x83Wa\x16\x82a\x13\x8FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x16\xA7a\x16\xA2\x84a\x16hV[a\x13\xEFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x16\xCAWa\x16\xC9a\x12\x16V[[\x83[\x81\x81\x10\x15a\x17\x11W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xEFWa\x16\xEEa\x12\x0CV[[\x80\x86\x01a\x16\xFC\x89\x82a\x14\xA4V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x16\xCCV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x170Wa\x17/a\x12\x0CV[[\x815a\x17@\x84\x82` \x86\x01a\x16\x94V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x17^\x81a\x17IV[\x81\x14a\x17iW`\0\x80\xFD[PV[`\0\x815\x90Pa\x17{\x81a\x17UV[\x92\x91PPV[`\0a\x01@\x82\x84\x03\x12\x15a\x17\x98Wa\x17\x97a\x13yV[[a\x17\xA3a\x01@a\x13\xEFV[\x90P`\0a\x17\xB3\x84\x82\x85\x01a\x11\xF7V[`\0\x83\x01RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xD7Wa\x17\xD6a\x14\nV[[a\x17\xE3\x84\x82\x85\x01a\x14\xA4V[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x07Wa\x18\x06a\x14\nV[[a\x18\x13\x84\x82\x85\x01a\x16:V[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x187Wa\x186a\x14\nV[[a\x18C\x84\x82\x85\x01a\x16:V[``\x83\x01RP`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18gWa\x18fa\x14\nV[[a\x18s\x84\x82\x85\x01a\x17\x1BV[`\x80\x83\x01RP`\xA0a\x18\x87\x84\x82\x85\x01a\x17lV[`\xA0\x83\x01RP`\xC0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xABWa\x18\xAAa\x14\nV[[a\x18\xB7\x84\x82\x85\x01a\x14\xA4V[`\xC0\x83\x01RP`\xE0a\x18\xCB\x84\x82\x85\x01a\x17lV[`\xE0\x83\x01RPa\x01\0a\x18\xE0\x84\x82\x85\x01a\x17lV[a\x01\0\x83\x01RPa\x01 a\x18\xF6\x84\x82\x85\x01a\x11\xC1V[a\x01 \x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x19\x19Wa\x19\x18a\x11nV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x197Wa\x196a\x11sV[[a\x19C\x84\x82\x85\x01a\x17\x81V[\x91PP\x92\x91PPV[a\x19U\x81a\x11\xD6V[\x82RPPV[`\0` \x82\x01\x90Pa\x19p`\0\x83\x01\x84a\x19LV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x19\x9Ba\x19\x96a\x19\x91\x84a\x11xV[a\x19vV[a\x11xV[\x90P\x91\x90PV[`\0a\x19\xAD\x82a\x19\x80V[\x90P\x91\x90PV[`\0a\x19\xBF\x82a\x19\xA2V[\x90P\x91\x90PV[a\x19\xCF\x81a\x19\xB4V[\x82RPPV[`\0` \x82\x01\x90Pa\x19\xEA`\0\x83\x01\x84a\x19\xC6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x1A0Wa\x1A/a\x19\xF0V[[PV[`\0\x81\x90Pa\x1AA\x82a\x1A\x1FV[\x91\x90PV[`\0a\x1AQ\x82a\x1A3V[\x90P\x91\x90PV[a\x1Aa\x81a\x1AFV[\x82RPPV[`\0` \x82\x01\x90Pa\x1A|`\0\x83\x01\x84a\x1AXV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A\x98Wa\x1A\x97a\x11nV[[`\0a\x1A\xA6\x84\x82\x85\x01a\x11\xC1V[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FPKPHelper: only accepts transfer`\0\x82\x01R\x7Fs from the PKPNFT contract\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1B\x1C`:\x83a\x1A\xAFV[\x91Pa\x1B'\x82a\x1A\xC0V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1BK\x81a\x1B\x0FV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x1Be\x81a\x1BRV[\x81\x14a\x1BpW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x1B\x82\x81a\x1B\\V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1B\x9EWa\x1B\x9Da\x11nV[[`\0a\x1B\xAC\x84\x82\x85\x01a\x1BsV[\x91PP\x92\x91PPV[a\x1B\xBE\x81a\x1BRV[\x82RPPV[`\0`@\x82\x01\x90Pa\x1B\xD9`\0\x83\x01\x85a\x1B\xB5V[a\x1B\xE6` \x83\x01\x84a\x1AXV[\x93\x92PPPV[`\0\x81Q\x90Pa\x1B\xFC\x81a\x11\xAAV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\x18Wa\x1C\x17a\x11nV[[`\0a\x1C&\x84\x82\x85\x01a\x1B\xEDV[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa\x1C>\x81a\x11\xE0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1CZWa\x1CYa\x11nV[[`\0a\x1Ch\x84\x82\x85\x01a\x1C/V[\x91PP\x92\x91PPV[\x7FPKPHelper: auth method type and `\0\x82\x01R\x7Fid array lengths must match\0\0\0\0\0` \x82\x01RPV[`\0a\x1C\xCD`;\x83a\x1A\xAFV[\x91Pa\x1C\xD8\x82a\x1CqV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\xFC\x81a\x1C\xC0V[\x90P\x91\x90PV[\x7FPKPHelper: auth method type and `\0\x82\x01R\x7Fpubkey array lengths must match\0` \x82\x01RPV[`\0a\x1D_`?\x83a\x1A\xAFV[\x91Pa\x1Dj\x82a\x1D\x03V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1D\x8E\x81a\x1DRV[\x90P\x91\x90PV[\x7FPKPHelper: auth method type and `\0\x82\x01R\x7Fscopes array lengths must match\0` \x82\x01RPV[`\0a\x1D\xF1`?\x83a\x1A\xAFV[\x91Pa\x1D\xFC\x82a\x1D\x95V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1E \x81a\x1D\xE4V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x1E_\x81a\x11\xD6V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1E\x9FW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1E\x84V[`\0\x84\x84\x01RPPPPV[`\0a\x1E\xB6\x82a\x1EeV[a\x1E\xC0\x81\x85a\x1EpV[\x93Pa\x1E\xD0\x81\x85` \x86\x01a\x1E\x81V[a\x1E\xD9\x81a\x13~V[\x84\x01\x91PP\x92\x91PPV[`\0``\x83\x01`\0\x83\x01Qa\x1E\xFC`\0\x86\x01\x82a\x1EVV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra\x1F\x14\x82\x82a\x1E\xABV[\x91PP`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x1F.\x82\x82a\x1E\xABV[\x91PP\x80\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x1Fs\x83\x83a\x1EVV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x1F\x97\x82a\x1F;V[a\x1F\xA1\x81\x85a\x1FFV[\x93Pa\x1F\xAC\x83a\x1FWV[\x80`\0[\x83\x81\x10\x15a\x1F\xDDW\x81Qa\x1F\xC4\x88\x82a\x1FgV[\x97Pa\x1F\xCF\x83a\x1F\x7FV[\x92PP`\x01\x81\x01\x90Pa\x1F\xB0V[P\x85\x93PPPP\x92\x91PPV[`\0``\x82\x01\x90Pa\x1F\xFF`\0\x83\x01\x86a\x19LV[\x81\x81\x03` \x83\x01Ra \x11\x81\x85a\x1E\xE4V[\x90P\x81\x81\x03`@\x83\x01Ra %\x81\x84a\x1F\x8CV[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a i\x82a\x11\xD6V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a \x9BWa \x9Aa /V[[`\x01\x82\x01\x90P\x91\x90PV[`\0``\x82\x01\x90Pa \xBB`\0\x83\x01\x86a\x19LV[a \xC8` \x83\x01\x85a\x13OV[\x81\x81\x03`@\x83\x01Ra \xDA\x81\x84a\x1F\x8CV[\x90P\x94\x93PPPPV[`\0``\x82\x01\x90Pa \xF9`\0\x83\x01\x86a\x13OV[a!\x06` \x83\x01\x85a\x13OV[a!\x13`@\x83\x01\x84a\x19LV[\x94\x93PPPPV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a!w`&\x83a\x1A\xAFV[\x91Pa!\x82\x82a!\x1BV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra!\xA6\x81a!jV[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a!\xE3` \x83a\x1A\xAFV[\x91Pa!\xEE\x82a!\xADV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\"\x12\x81a!\xD6V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 ??\r\xC9\xE6\xE7)\x8C\x9C\xE5%\xB0\xBAQ\xC6\x94dx\xA5\x02jH\xF9A\xF5\xEB\x13+\x86\x8B#<dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static PKPHELPERV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xC2W`\x005`\xE0\x1C\x80cs\xCCA\x11\x11a\0\x7FW\x80c\xB3%\x97\xB8\x11a\0YW\x80c\xB3%\x97\xB8\x14a\x02MW\x80c\xCA\xEA\xD0\xC7\x14a\x02}W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xA8W\x80c\xF9]q\xB1\x14a\x02\xD1Wa\0\xC2V[\x80cs\xCCA\x11\x14a\x01\xCCW\x80c\x8D\xA5\xCB[\x14a\x01\xF7W\x80c\x9D\xCA\x002\x14a\x02\"Wa\0\xC2V[\x80c\x15\x0Bz\x02\x14a\0\xC7W\x80c2vU\x8C\x14a\x01\x04W\x80c5\xFD\xBC@\x14a\x01/W\x80cPC\x02l\x14a\x01_W\x80cP\xD1{^\x14a\x01\x8AW\x80cqP\x18\xA6\x14a\x01\xB5W[`\0\x80\xFD[4\x80\x15a\0\xD3W`\0\x80\xFD[Pa\0\xEE`\x04\x806\x03\x81\x01\x90a\0\xE9\x91\x90a\x12qV[a\x02\xFAV[`@Qa\0\xFB\x91\x90a\x134V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x10W`\0\x80\xFD[Pa\x01\x19a\x03\x84V[`@Qa\x01&\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[a\x01I`\x04\x806\x03\x81\x01\x90a\x01D\x91\x90a\x19\x03V[a\x04\xC8V[`@Qa\x01V\x91\x90a\x19[V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01kW`\0\x80\xFD[Pa\x01ta\n\xC0V[`@Qa\x01\x81\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x96W`\0\x80\xFD[Pa\x01\x9Fa\x0C\x04V[`@Qa\x01\xAC\x91\x90a\x19\xD5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xC1W`\0\x80\xFD[Pa\x01\xCAa\x0C*V[\0[4\x80\x15a\x01\xD8W`\0\x80\xFD[Pa\x01\xE1a\x0C>V[`@Qa\x01\xEE\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x03W`\0\x80\xFD[Pa\x02\x0Ca\r\x82V[`@Qa\x02\x19\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02.W`\0\x80\xFD[Pa\x027a\r\xABV[`@Qa\x02D\x91\x90a\x1AgV[`@Q\x80\x91\x03\x90\xF3[a\x02g`\x04\x806\x03\x81\x01\x90a\x02b\x91\x90a\x19\x03V[a\r\xBEV[`@Qa\x02t\x91\x90a\x19[V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x89W`\0\x80\xFD[Pa\x02\x92a\r\xD0V[`@Qa\x02\x9F\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB4W`\0\x80\xFD[Pa\x02\xCF`\x04\x806\x03\x81\x01\x90a\x02\xCA\x91\x90a\x1A\x82V[a\x0F\x14V[\0[4\x80\x15a\x02\xDDW`\0\x80\xFD[Pa\x02\xF8`\x04\x806\x03\x81\x01\x90a\x02\xF3\x91\x90a\x1A\x82V[a\x0F\x97V[\0[`\0a\x03\x04a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x03qW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03h\x90a\x1B2V[`@Q\x80\x91\x03\x90\xFD[c\x15\x0Bz\x02`\xE0\x1B\x90P\x95\x94PPPPPV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90r\xF88`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x041W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04U\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x82\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC3\x91\x90a\x1C\x02V[\x90P\x90V[`\0\x80a\x04\xD3a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c]\"\x8B\x164\x85`\0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x10\x91\x90a\x19[V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x05.W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05S\x91\x90a\x1CDV[\x90P\x82`@\x01QQ\x83` \x01QQ\x14a\x05\xA1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\x98\x90a\x1C\xE3V[`@Q\x80\x91\x03\x90\xFD[\x82``\x01QQ\x83` \x01QQ\x14a\x05\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xE4\x90a\x1DuV[`@Q\x80\x91\x03\x90\xFD[\x82`\x80\x01QQ\x83` \x01QQ\x14a\x069W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x060\x90a\x1E\x07V[`@Q\x80\x91\x03\x90\xFD[`\0\x83` \x01QQ\x14a\x07sW`\0[\x83` \x01QQ\x81\x10\x15a\x07qWa\x06^a\x03\x84V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x88` \x01Q\x86\x81Q\x81\x10a\x06\x9CWa\x06\x9Ba\x1E'V[[` \x02` \x01\x01Q\x81R` \x01\x88`@\x01Q\x86\x81Q\x81\x10a\x06\xC0Wa\x06\xBFa\x1E'V[[` \x02` \x01\x01Q\x81R` \x01\x88``\x01Q\x86\x81Q\x81\x10a\x06\xE4Wa\x06\xE3a\x1E'V[[` \x02` \x01\x01Q\x81RP\x87`\x80\x01Q\x85\x81Q\x81\x10a\x07\x06Wa\x07\x05a\x1E'V[[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07,\x93\x92\x91\x90a\x1F\xEAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07ZW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x07i\x90a ^V[\x91PPa\x06IV[P[`\0a\x07}a\x03\x84V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xB5\x91\x90a\x19[V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF6\x91\x90a\x1C\x02V[\x90P\x83`\xA0\x01Q\x15a\x08}Wa\x08\na\x03\x84V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x16c\xC1!\x83\x83\x87`\xC0\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08J\x93\x92\x91\x90a \xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08dW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08xW=`\0\x80>=`\0\xFD[PPPP[\x83`\xE0\x01Q\x15a\t\x02Wa\x08\x8Fa\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xCB\x93\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xF9W=`\0\x80>=`\0\xFD[PPPPa\n\xB6V[\x83a\x01\0\x01Q\x15a\t\x84Wa\t\x15a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x96lh\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tM\x91\x90a\x19[V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\tgW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t{W=`\0\x80>=`\0\xFD[PPPPa\n\xB5V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84a\x01 \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n=Wa\t\xC5a\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x84.\x0E0\x86a\x01 \x01Q\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x06\x93\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n4W=`\0\x80>=`\0\xFD[PPPPa\n\xB4V[a\nEa\r\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x81\x93\x92\x91\x90a \xE4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xAFW=`\0\x80>=`\0\xFD[PPPP[[[\x81\x92PPP\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x16\xF7k\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x91\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xBE\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xFF\x91\x90a\x1C\x02V[\x90P\x90V[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x0C2a\x10\x1AV[a\x0C<`\0a\x10\x98V[V[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cB\x16\xE7:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x0F\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r<\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rYW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r}\x91\x90a\x1C\x02V[\x90P\x90V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0a\r\xC9\x82a\x04\xC8V[\x90P\x91\x90PV[`\0`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8E\x8D\xFD\x16`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c,\x0B\x8B\xF7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA1\x91\x90a\x1B\x88V[`\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xCE\x92\x91\x90a\x1B\xC4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x0F\x91\x90a\x1C\x02V[\x90P\x90V[a\x0F\x1Ca\x10\x1AV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0F\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x82\x90a!\x8DV[`@Q\x80\x91\x03\x90\xFD[a\x0F\x94\x81a\x10\x98V[PV[a\x0F\x9Fa\x10\x1AV[\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F'`\x07<|\xD8\xCA\xC51\xD7\xF6C\xBE\xCB\xFB\xB7M\x8B\x81VD>\xAC\xF8yb%2\xDB\xBB<\xD5\x81`@Qa\x10\x0F\x91\x90a\x13^V[`@Q\x80\x91\x03\x90\xA1PV[a\x10\"a\x11\\V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x10@a\r\x82V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10\x96W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x8D\x90a!\xF9V[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x003\x90P\x90V[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x11\xA3\x82a\x11xV[\x90P\x91\x90PV[a\x11\xB3\x81a\x11\x98V[\x81\x14a\x11\xBEW`\0\x80\xFD[PV[`\0\x815\x90Pa\x11\xD0\x81a\x11\xAAV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x11\xE9\x81a\x11\xD6V[\x81\x14a\x11\xF4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\x06\x81a\x11\xE0V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x121Wa\x120a\x12\x0CV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12NWa\x12Ma\x12\x11V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x12jWa\x12ia\x12\x16V[[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x12\x8DWa\x12\x8Ca\x11nV[[`\0a\x12\x9B\x88\x82\x89\x01a\x11\xC1V[\x95PP` a\x12\xAC\x88\x82\x89\x01a\x11\xC1V[\x94PP`@a\x12\xBD\x88\x82\x89\x01a\x11\xF7V[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xDEWa\x12\xDDa\x11sV[[a\x12\xEA\x88\x82\x89\x01a\x12\x1BV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x13.\x81a\x12\xF9V[\x82RPPV[`\0` \x82\x01\x90Pa\x13I`\0\x83\x01\x84a\x13%V[\x92\x91PPV[a\x13X\x81a\x11\x98V[\x82RPPV[`\0` \x82\x01\x90Pa\x13s`\0\x83\x01\x84a\x13OV[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x13\xC7\x82a\x13~V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13\xE6Wa\x13\xE5a\x13\x8FV[[\x80`@RPPPV[`\0a\x13\xF9a\x11dV[\x90Pa\x14\x05\x82\x82a\x13\xBEV[\x91\x90PV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14*Wa\x14)a\x13\x8FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x14Na\x14I\x84a\x14\x0FV[a\x13\xEFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x14qWa\x14pa\x12\x16V[[\x83[\x81\x81\x10\x15a\x14\x9AW\x80a\x14\x86\x88\x82a\x11\xF7V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x14sV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x14\xB9Wa\x14\xB8a\x12\x0CV[[\x815a\x14\xC9\x84\x82` \x86\x01a\x14;V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14\xEDWa\x14\xECa\x13\x8FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\x1EWa\x15\x1Da\x13\x8FV[[a\x15'\x82a\x13~V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x15Va\x15Q\x84a\x15\x03V[a\x13\xEFV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x15rWa\x15qa\x14\xFEV[[a\x15}\x84\x82\x85a\x154V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x15\x9AWa\x15\x99a\x12\x0CV[[\x815a\x15\xAA\x84\x82` \x86\x01a\x15CV[\x91PP\x92\x91PPV[`\0a\x15\xC6a\x15\xC1\x84a\x14\xD2V[a\x13\xEFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x15\xE9Wa\x15\xE8a\x12\x16V[[\x83[\x81\x81\x10\x15a\x160W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x0EWa\x16\ra\x12\x0CV[[\x80\x86\x01a\x16\x1B\x89\x82a\x15\x85V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x15\xEBV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x16OWa\x16Na\x12\x0CV[[\x815a\x16_\x84\x82` \x86\x01a\x15\xB3V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x16\x83Wa\x16\x82a\x13\x8FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x16\xA7a\x16\xA2\x84a\x16hV[a\x13\xEFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x16\xCAWa\x16\xC9a\x12\x16V[[\x83[\x81\x81\x10\x15a\x17\x11W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xEFWa\x16\xEEa\x12\x0CV[[\x80\x86\x01a\x16\xFC\x89\x82a\x14\xA4V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x16\xCCV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x170Wa\x17/a\x12\x0CV[[\x815a\x17@\x84\x82` \x86\x01a\x16\x94V[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x17^\x81a\x17IV[\x81\x14a\x17iW`\0\x80\xFD[PV[`\0\x815\x90Pa\x17{\x81a\x17UV[\x92\x91PPV[`\0a\x01@\x82\x84\x03\x12\x15a\x17\x98Wa\x17\x97a\x13yV[[a\x17\xA3a\x01@a\x13\xEFV[\x90P`\0a\x17\xB3\x84\x82\x85\x01a\x11\xF7V[`\0\x83\x01RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xD7Wa\x17\xD6a\x14\nV[[a\x17\xE3\x84\x82\x85\x01a\x14\xA4V[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x07Wa\x18\x06a\x14\nV[[a\x18\x13\x84\x82\x85\x01a\x16:V[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x187Wa\x186a\x14\nV[[a\x18C\x84\x82\x85\x01a\x16:V[``\x83\x01RP`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18gWa\x18fa\x14\nV[[a\x18s\x84\x82\x85\x01a\x17\x1BV[`\x80\x83\x01RP`\xA0a\x18\x87\x84\x82\x85\x01a\x17lV[`\xA0\x83\x01RP`\xC0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xABWa\x18\xAAa\x14\nV[[a\x18\xB7\x84\x82\x85\x01a\x14\xA4V[`\xC0\x83\x01RP`\xE0a\x18\xCB\x84\x82\x85\x01a\x17lV[`\xE0\x83\x01RPa\x01\0a\x18\xE0\x84\x82\x85\x01a\x17lV[a\x01\0\x83\x01RPa\x01 a\x18\xF6\x84\x82\x85\x01a\x11\xC1V[a\x01 \x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x19\x19Wa\x19\x18a\x11nV[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x197Wa\x196a\x11sV[[a\x19C\x84\x82\x85\x01a\x17\x81V[\x91PP\x92\x91PPV[a\x19U\x81a\x11\xD6V[\x82RPPV[`\0` \x82\x01\x90Pa\x19p`\0\x83\x01\x84a\x19LV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0a\x19\x9Ba\x19\x96a\x19\x91\x84a\x11xV[a\x19vV[a\x11xV[\x90P\x91\x90PV[`\0a\x19\xAD\x82a\x19\x80V[\x90P\x91\x90PV[`\0a\x19\xBF\x82a\x19\xA2V[\x90P\x91\x90PV[a\x19\xCF\x81a\x19\xB4V[\x82RPPV[`\0` \x82\x01\x90Pa\x19\xEA`\0\x83\x01\x84a\x19\xC6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x1A0Wa\x1A/a\x19\xF0V[[PV[`\0\x81\x90Pa\x1AA\x82a\x1A\x1FV[\x91\x90PV[`\0a\x1AQ\x82a\x1A3V[\x90P\x91\x90PV[a\x1Aa\x81a\x1AFV[\x82RPPV[`\0` \x82\x01\x90Pa\x1A|`\0\x83\x01\x84a\x1AXV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1A\x98Wa\x1A\x97a\x11nV[[`\0a\x1A\xA6\x84\x82\x85\x01a\x11\xC1V[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FPKPHelper: only accepts transfer`\0\x82\x01R\x7Fs from the PKPNFT contract\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1B\x1C`:\x83a\x1A\xAFV[\x91Pa\x1B'\x82a\x1A\xC0V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1BK\x81a\x1B\x0FV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x1Be\x81a\x1BRV[\x81\x14a\x1BpW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x1B\x82\x81a\x1B\\V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1B\x9EWa\x1B\x9Da\x11nV[[`\0a\x1B\xAC\x84\x82\x85\x01a\x1BsV[\x91PP\x92\x91PPV[a\x1B\xBE\x81a\x1BRV[\x82RPPV[`\0`@\x82\x01\x90Pa\x1B\xD9`\0\x83\x01\x85a\x1B\xB5V[a\x1B\xE6` \x83\x01\x84a\x1AXV[\x93\x92PPPV[`\0\x81Q\x90Pa\x1B\xFC\x81a\x11\xAAV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\x18Wa\x1C\x17a\x11nV[[`\0a\x1C&\x84\x82\x85\x01a\x1B\xEDV[\x91PP\x92\x91PPV[`\0\x81Q\x90Pa\x1C>\x81a\x11\xE0V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1CZWa\x1CYa\x11nV[[`\0a\x1Ch\x84\x82\x85\x01a\x1C/V[\x91PP\x92\x91PPV[\x7FPKPHelper: auth method type and `\0\x82\x01R\x7Fid array lengths must match\0\0\0\0\0` \x82\x01RPV[`\0a\x1C\xCD`;\x83a\x1A\xAFV[\x91Pa\x1C\xD8\x82a\x1CqV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\xFC\x81a\x1C\xC0V[\x90P\x91\x90PV[\x7FPKPHelper: auth method type and `\0\x82\x01R\x7Fpubkey array lengths must match\0` \x82\x01RPV[`\0a\x1D_`?\x83a\x1A\xAFV[\x91Pa\x1Dj\x82a\x1D\x03V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1D\x8E\x81a\x1DRV[\x90P\x91\x90PV[\x7FPKPHelper: auth method type and `\0\x82\x01R\x7Fscopes array lengths must match\0` \x82\x01RPV[`\0a\x1D\xF1`?\x83a\x1A\xAFV[\x91Pa\x1D\xFC\x82a\x1D\x95V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1E \x81a\x1D\xE4V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[a\x1E_\x81a\x11\xD6V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x1E\x9FW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x1E\x84V[`\0\x84\x84\x01RPPPPV[`\0a\x1E\xB6\x82a\x1EeV[a\x1E\xC0\x81\x85a\x1EpV[\x93Pa\x1E\xD0\x81\x85` \x86\x01a\x1E\x81V[a\x1E\xD9\x81a\x13~V[\x84\x01\x91PP\x92\x91PPV[`\0``\x83\x01`\0\x83\x01Qa\x1E\xFC`\0\x86\x01\x82a\x1EVV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra\x1F\x14\x82\x82a\x1E\xABV[\x91PP`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x1F.\x82\x82a\x1E\xABV[\x91PP\x80\x91PP\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x1Fs\x83\x83a\x1EVV[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x1F\x97\x82a\x1F;V[a\x1F\xA1\x81\x85a\x1FFV[\x93Pa\x1F\xAC\x83a\x1FWV[\x80`\0[\x83\x81\x10\x15a\x1F\xDDW\x81Qa\x1F\xC4\x88\x82a\x1FgV[\x97Pa\x1F\xCF\x83a\x1F\x7FV[\x92PP`\x01\x81\x01\x90Pa\x1F\xB0V[P\x85\x93PPPP\x92\x91PPV[`\0``\x82\x01\x90Pa\x1F\xFF`\0\x83\x01\x86a\x19LV[\x81\x81\x03` \x83\x01Ra \x11\x81\x85a\x1E\xE4V[\x90P\x81\x81\x03`@\x83\x01Ra %\x81\x84a\x1F\x8CV[\x90P\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a i\x82a\x11\xD6V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a \x9BWa \x9Aa /V[[`\x01\x82\x01\x90P\x91\x90PV[`\0``\x82\x01\x90Pa \xBB`\0\x83\x01\x86a\x19LV[a \xC8` \x83\x01\x85a\x13OV[\x81\x81\x03`@\x83\x01Ra \xDA\x81\x84a\x1F\x8CV[\x90P\x94\x93PPPPV[`\0``\x82\x01\x90Pa \xF9`\0\x83\x01\x86a\x13OV[a!\x06` \x83\x01\x85a\x13OV[a!\x13`@\x83\x01\x84a\x19LV[\x94\x93PPPPV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a!w`&\x83a\x1A\xAFV[\x91Pa!\x82\x82a!\x1BV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra!\xA6\x81a!jV[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a!\xE3` \x83a\x1A\xAFV[\x91Pa!\xEE\x82a!\xADV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\"\x12\x81a!\xD6V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 ??\r\xC9\xE6\xE7)\x8C\x9C\xE5%\xB0\xBAQ\xC6\x94dx\xA5\x02jH\xF9A\xF5\xEB\x13+\x86\x8B#<dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static PKPHELPERV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PKPHelperV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PKPHelperV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PKPHelperV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PKPHelperV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PKPHelperV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PKPHelperV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PKPHelperV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PKPHELPERV2_ABI.clone(),
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
                PKPHELPERV2_ABI.clone(),
                PKPHELPERV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `getDomainWalletRegistry` (0x73cc4111) function
        pub fn get_domain_wallet_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([115, 204, 65, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPKPNftMetdataAddress` (0x5043026c) function
        pub fn get_pkp_nft_metdata_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 67, 2, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPkpNftAddress` (0xcaead0c7) function
        pub fn get_pkp_nft_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([202, 234, 208, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPkpPermissionsAddress` (0x3276558c) function
        pub fn get_pkp_permissions_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([50, 118, 85, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintNextAndAddAuthMethods` (0xb32597b8) function
        pub fn mint_next_and_add_auth_methods(
            &self,
            params: NewPKPParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 37, 151, 184], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintNextAndAddAuthMethodsWithTypes` (0x35fdbc40) function
        pub fn mint_next_and_add_auth_methods_with_types(
            &self,
            params: NewPKPParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 253, 188, 64], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setContractResolver` (0xf95d71b1) function
        pub fn set_contract_resolver(
            &self,
            new_resolver_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 93, 113, 177], new_resolver_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ContractResolverAddressSet` event
        pub fn contract_resolver_address_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractResolverAddressSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PKPHelperV2Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PKPHelperV2<M> {
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
        name = "ContractResolverAddressSet",
        abi = "ContractResolverAddressSet(address)"
    )]
    pub struct ContractResolverAddressSetFilter {
        pub new_resolver_address: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum PKPHelperV2Events {
        ContractResolverAddressSetFilter(ContractResolverAddressSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for PKPHelperV2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ContractResolverAddressSetFilter::decode_log(log) {
                return Ok(PKPHelperV2Events::ContractResolverAddressSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PKPHelperV2Events::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PKPHelperV2Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ContractResolverAddressSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ContractResolverAddressSetFilter> for PKPHelperV2Events {
        fn from(value: ContractResolverAddressSetFilter) -> Self {
            Self::ContractResolverAddressSetFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for PKPHelperV2Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
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
    ///Container type for all input parameters for the `getDomainWalletRegistry` function with signature `getDomainWalletRegistry()` and selector `0x73cc4111`
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
    #[ethcall(name = "getDomainWalletRegistry", abi = "getDomainWalletRegistry()")]
    pub struct GetDomainWalletRegistryCall;
    ///Container type for all input parameters for the `getPKPNftMetdataAddress` function with signature `getPKPNftMetdataAddress()` and selector `0x5043026c`
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
    #[ethcall(name = "getPKPNftMetdataAddress", abi = "getPKPNftMetdataAddress()")]
    pub struct GetPKPNftMetdataAddressCall;
    ///Container type for all input parameters for the `getPkpNftAddress` function with signature `getPkpNftAddress()` and selector `0xcaead0c7`
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
    #[ethcall(name = "getPkpNftAddress", abi = "getPkpNftAddress()")]
    pub struct GetPkpNftAddressCall;
    ///Container type for all input parameters for the `getPkpPermissionsAddress` function with signature `getPkpPermissionsAddress()` and selector `0x3276558c`
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
    #[ethcall(name = "getPkpPermissionsAddress", abi = "getPkpPermissionsAddress()")]
    pub struct GetPkpPermissionsAddressCall;
    ///Container type for all input parameters for the `mintNextAndAddAuthMethods` function with signature `mintNextAndAddAuthMethods((uint256,uint256[],bytes[],bytes[],uint256[][],bool,uint256[],bool,bool,address))` and selector `0xb32597b8`
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
        name = "mintNextAndAddAuthMethods",
        abi = "mintNextAndAddAuthMethods((uint256,uint256[],bytes[],bytes[],uint256[][],bool,uint256[],bool,bool,address))"
    )]
    pub struct MintNextAndAddAuthMethodsCall {
        pub params: NewPKPParams,
    }
    ///Container type for all input parameters for the `mintNextAndAddAuthMethodsWithTypes` function with signature `mintNextAndAddAuthMethodsWithTypes((uint256,uint256[],bytes[],bytes[],uint256[][],bool,uint256[],bool,bool,address))` and selector `0x35fdbc40`
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
        name = "mintNextAndAddAuthMethodsWithTypes",
        abi = "mintNextAndAddAuthMethodsWithTypes((uint256,uint256[],bytes[],bytes[],uint256[][],bool,uint256[],bool,bool,address))"
    )]
    pub struct MintNextAndAddAuthMethodsWithTypesCall {
        pub params: NewPKPParams,
    }
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
        pub new_resolver_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum PKPHelperV2Calls {
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        GetDomainWalletRegistry(GetDomainWalletRegistryCall),
        GetPKPNftMetdataAddress(GetPKPNftMetdataAddressCall),
        GetPkpNftAddress(GetPkpNftAddressCall),
        GetPkpPermissionsAddress(GetPkpPermissionsAddressCall),
        MintNextAndAddAuthMethods(MintNextAndAddAuthMethodsCall),
        MintNextAndAddAuthMethodsWithTypes(MintNextAndAddAuthMethodsWithTypesCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetContractResolver(SetContractResolverCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for PKPHelperV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <GetDomainWalletRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDomainWalletRegistry(decoded));
            }
            if let Ok(decoded) = <GetPKPNftMetdataAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPKPNftMetdataAddress(decoded));
            }
            if let Ok(decoded) = <GetPkpNftAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPkpNftAddress(decoded));
            }
            if let Ok(decoded) = <GetPkpPermissionsAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPkpPermissionsAddress(decoded));
            }
            if let Ok(decoded) = <MintNextAndAddAuthMethodsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintNextAndAddAuthMethods(decoded));
            }
            if let Ok(decoded) = <MintNextAndAddAuthMethodsWithTypesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintNextAndAddAuthMethodsWithTypes(decoded));
            }
            if let Ok(decoded) = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetContractResolver(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PKPHelperV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Env(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDomainWalletRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPKPNftMetdataAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPkpNftAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPkpPermissionsAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintNextAndAddAuthMethods(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintNextAndAddAuthMethodsWithTypes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PKPHelperV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Env(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDomainWalletRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPKPNftMetdataAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPkpNftAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPkpPermissionsAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintNextAndAddAuthMethods(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintNextAndAddAuthMethodsWithTypes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractResolver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ContractResolverCall> for PKPHelperV2Calls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for PKPHelperV2Calls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<GetDomainWalletRegistryCall> for PKPHelperV2Calls {
        fn from(value: GetDomainWalletRegistryCall) -> Self {
            Self::GetDomainWalletRegistry(value)
        }
    }
    impl ::core::convert::From<GetPKPNftMetdataAddressCall> for PKPHelperV2Calls {
        fn from(value: GetPKPNftMetdataAddressCall) -> Self {
            Self::GetPKPNftMetdataAddress(value)
        }
    }
    impl ::core::convert::From<GetPkpNftAddressCall> for PKPHelperV2Calls {
        fn from(value: GetPkpNftAddressCall) -> Self {
            Self::GetPkpNftAddress(value)
        }
    }
    impl ::core::convert::From<GetPkpPermissionsAddressCall> for PKPHelperV2Calls {
        fn from(value: GetPkpPermissionsAddressCall) -> Self {
            Self::GetPkpPermissionsAddress(value)
        }
    }
    impl ::core::convert::From<MintNextAndAddAuthMethodsCall> for PKPHelperV2Calls {
        fn from(value: MintNextAndAddAuthMethodsCall) -> Self {
            Self::MintNextAndAddAuthMethods(value)
        }
    }
    impl ::core::convert::From<MintNextAndAddAuthMethodsWithTypesCall>
    for PKPHelperV2Calls {
        fn from(value: MintNextAndAddAuthMethodsWithTypesCall) -> Self {
            Self::MintNextAndAddAuthMethodsWithTypes(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for PKPHelperV2Calls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PKPHelperV2Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PKPHelperV2Calls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetContractResolverCall> for PKPHelperV2Calls {
        fn from(value: SetContractResolverCall) -> Self {
            Self::SetContractResolver(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PKPHelperV2Calls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
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
    ///Container type for all return fields from the `getDomainWalletRegistry` function with signature `getDomainWalletRegistry()` and selector `0x73cc4111`
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
    pub struct GetDomainWalletRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPKPNftMetdataAddress` function with signature `getPKPNftMetdataAddress()` and selector `0x5043026c`
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
    pub struct GetPKPNftMetdataAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPkpNftAddress` function with signature `getPkpNftAddress()` and selector `0xcaead0c7`
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
    pub struct GetPkpNftAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPkpPermissionsAddress` function with signature `getPkpPermissionsAddress()` and selector `0x3276558c`
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
    pub struct GetPkpPermissionsAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mintNextAndAddAuthMethods` function with signature `mintNextAndAddAuthMethods((uint256,uint256[],bytes[],bytes[],uint256[][],bool,uint256[],bool,bool,address))` and selector `0xb32597b8`
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
    pub struct MintNextAndAddAuthMethodsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mintNextAndAddAuthMethodsWithTypes` function with signature `mintNextAndAddAuthMethodsWithTypes((uint256,uint256[],bytes[],bytes[],uint256[][],bool,uint256[],bool,bool,address))` and selector `0x35fdbc40`
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
    pub struct MintNextAndAddAuthMethodsWithTypesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///`NewPKPParams(uint256,uint256[],bytes[],bytes[],uint256[][],bool,uint256[],bool,bool,address)`
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
    pub struct NewPKPParams {
        pub key_type: ::ethers::core::types::U256,
        pub permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub add_pkp_eth_address_as_permitted_address: bool,
        pub pkp_eth_address_scopes: ::std::vec::Vec<::ethers::core::types::U256>,
        pub send_pkp_to_itself: bool,
        pub burn_pkp: bool,
        pub send_to_address_after_minting: ::ethers::core::types::Address,
    }
}
