pub use allowlist::*;
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
pub mod allowlist {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/Allowlist.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("allowAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowAll"),
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
                    ::std::borrow::ToOwned::to_owned("allowedItems"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowedItems"),
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
                    ::std::borrow::ToOwned::to_owned("isAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("removeAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("setAllowAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAllowAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_allowAll"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("setNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setNotAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("AdminAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdminRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ItemAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ItemAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("ItemNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ItemNotAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
    pub static ALLOWLIST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0-a\0\"a\0R` \x1B` \x1CV[a\0Z` \x1B` \x1CV[`\x01\x80\x81\x90UPa\0L3`\x03a\x01\x1E` \x1Ba\x05\x81\x17\x90\x91\x90` \x1CV[Pa\x01\xEDV[`\x003\x90P\x90V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0a\x01L\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x01T` \x1B` \x1CV[\x90P\x92\x91PPV[`\0a\x01f\x83\x83a\x01\xCA` \x1B` \x1CV[a\x01\xBFW\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pa\x01\xC4V[`\0\x90P[\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[a\r*\x80a\x01\xFC`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0qW\x80cqP\x18\xA6\x14a\x01PW\x80cxuR\x95\x14a\x01ZW\x80c\x86X\x15\x97\x14a\x01\x8AW\x80c\x87g\xD9\xAA\x14a\x01\xA6W\x80c\x8D\xA5\xCB[\x14a\x01\xC2W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xE0Wa\0\xA9V[\x80c\x17\x85\xF5<\x14a\0\xAEW\x80cM}\x9C\x01\x14a\0\xCAW\x80cN\xE6C\xA5\x14a\0\xE6W\x80cR\xF9u6\x14a\x01\x04W\x80cpH\x02u\x14a\x014W[`\0\x80\xFD[a\0\xC8`\x04\x806\x03\x81\x01\x90a\0\xC3\x91\x90a\teV[a\x01\xFCV[\0[a\0\xE4`\x04\x806\x03\x81\x01\x90a\0\xDF\x91\x90a\t\xCAV[a\x02_V[\0[a\0\xEEa\x02\x84V[`@Qa\0\xFB\x91\x90a\n\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1E`\x04\x806\x03\x81\x01\x90a\x01\x19\x91\x90a\nWV[a\x02\x97V[`@Qa\x01+\x91\x90a\n\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01N`\x04\x806\x03\x81\x01\x90a\x01I\x91\x90a\teV[a\x02\xB7V[\0[a\x01Xa\x03\x1AV[\0[a\x01t`\x04\x806\x03\x81\x01\x90a\x01o\x91\x90a\nWV[a\x03.V[`@Qa\x01\x81\x91\x90a\n\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA4`\x04\x806\x03\x81\x01\x90a\x01\x9F\x91\x90a\nWV[a\x03wV[\0[a\x01\xC0`\x04\x806\x03\x81\x01\x90a\x01\xBB\x91\x90a\nWV[a\x04&V[\0[a\x01\xCAa\x04\xD5V[`@Qa\x01\xD7\x91\x90a\n\x93V[`@Q\x80\x91\x03\x90\xF3[a\x01\xFA`\x04\x806\x03\x81\x01\x90a\x01\xF5\x91\x90a\teV[a\x04\xFEV[\0[a\x02\x04a\x05\xB1V[a\x02\x18\x81`\x03a\x06/\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xA3\xB6+\xC3c&\x05-\x97\xEAb\xD6<=`0\x8E\xD4\xC3\xEA\x8A\xC0y\xDD\x84\x99\xF1\xE9\xC4\xF8\x0C\x0F`@Q`@Q\x80\x91\x03\x90\xA2PV[a\x02ga\x05\xB1V[\x80`\x05`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x02` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[a\x02\xBFa\x05\xB1V[a\x02\xD3\x81`\x03a\x05\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39`@Q`@Q\x80\x91\x03\x90\xA2PV[a\x03\"a\x05\xB1V[a\x03,`\0a\x06_V[V[`\0`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x03NW`\x01\x90Pa\x03rV[`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P[\x91\x90PV[a\x03\x8B3`\x03a\x07#\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xC1\x90a\x0B\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x7F\xE4\xBE\x98\x88j<\x8C\xD9\x02\x7F\xDBD\x06_k\x81QL\\\xF5\xA1\xDA\xB8^\xB7s;\xEBS\x15\x80\xEF`@Q`@Q\x80\x91\x03\x90\xA2PV[a\x04:3`\x03a\x07#\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04yW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04p\x90a\x0B\x0BV[`@Q\x80\x91\x03\x90\xFD[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x7F\xA6v\xEE~\xED\x1B\x9E\x9E\x90\xC0\xCE\x19d\x91\x9B\x8A\x08K\x89\x1B\xAF\xA6\xB7x\xB6Eq\xF38\xC0\xCD\x95`@Q`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x06a\x05\xB1V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05uW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05l\x90a\x0B\x9DV[`@Q\x80\x91\x03\x90\xFD[a\x05~\x81a\x06_V[PV[`\0a\x05\xA9\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x07SV[\x90P\x92\x91PPV[a\x05\xB9a\x07\xC3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xD7a\x04\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06-W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06$\x90a\x0C\tV[`@Q\x80\x91\x03\x90\xFD[V[`\0a\x06W\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x07\xCBV[\x90P\x92\x91PPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0a\x07K\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x08\xDFV[\x90P\x92\x91PPV[`\0a\x07_\x83\x83a\x08\xDFV[a\x07\xB8W\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pa\x07\xBDV[`\0\x90P[\x92\x91PPV[`\x003\x90P\x90V[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x14a\x08\xD3W`\0`\x01\x82a\x07\xFD\x91\x90a\x0CbV[\x90P`\0`\x01\x86`\0\x01\x80T\x90Pa\x08\x15\x91\x90a\x0CbV[\x90P\x81\x81\x14a\x08\x84W`\0\x86`\0\x01\x82\x81T\x81\x10a\x086Wa\x085a\x0C\x96V[[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x08ZWa\x08Ya\x0C\x96V[[\x90`\0R` `\0 \x01\x81\x90UP\x83\x87`\x01\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[\x85`\0\x01\x80T\x80a\x08\x98Wa\x08\x97a\x0C\xC5V[[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x08\xD9V[`\0\x91PP[\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\t2\x82a\t\x07V[\x90P\x91\x90PV[a\tB\x81a\t'V[\x81\x14a\tMW`\0\x80\xFD[PV[`\0\x815\x90Pa\t_\x81a\t9V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t{Wa\tza\t\x02V[[`\0a\t\x89\x84\x82\x85\x01a\tPV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\t\xA7\x81a\t\x92V[\x81\x14a\t\xB2W`\0\x80\xFD[PV[`\0\x815\x90Pa\t\xC4\x81a\t\x9EV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t\xE0Wa\t\xDFa\t\x02V[[`\0a\t\xEE\x84\x82\x85\x01a\t\xB5V[\x91PP\x92\x91PPV[a\n\0\x81a\t\x92V[\x82RPPV[`\0` \x82\x01\x90Pa\n\x1B`\0\x83\x01\x84a\t\xF7V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\n4\x81a\n!V[\x81\x14a\n?W`\0\x80\xFD[PV[`\0\x815\x90Pa\nQ\x81a\n+V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\nmWa\nla\t\x02V[[`\0a\n{\x84\x82\x85\x01a\nBV[\x91PP\x92\x91PPV[a\n\x8D\x81a\t'V[\x82RPPV[`\0` \x82\x01\x90Pa\n\xA8`\0\x83\x01\x84a\n\x84V[\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\n\xF5`\x0C\x83a\n\xAEV[\x91Pa\x0B\0\x82a\n\xBFV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0B$\x81a\n\xE8V[\x90P\x91\x90PV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x0B\x87`&\x83a\n\xAEV[\x91Pa\x0B\x92\x82a\x0B+V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0B\xB6\x81a\x0BzV[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a\x0B\xF3` \x83a\n\xAEV[\x91Pa\x0B\xFE\x82a\x0B\xBDV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0C\"\x81a\x0B\xE6V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x0Cm\x82a\x0C)V[\x91Pa\x0Cx\x83a\x0C)V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0C\x90Wa\x0C\x8Fa\x0C3V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD26\xF8\x9E\xEAg-\xC5_V/c#9!\x18\xF8\xE2\x1BET\xE2A\xBF'\x8D{+\xB6\x94\xCF\x04dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static ALLOWLIST_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0qW\x80cqP\x18\xA6\x14a\x01PW\x80cxuR\x95\x14a\x01ZW\x80c\x86X\x15\x97\x14a\x01\x8AW\x80c\x87g\xD9\xAA\x14a\x01\xA6W\x80c\x8D\xA5\xCB[\x14a\x01\xC2W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xE0Wa\0\xA9V[\x80c\x17\x85\xF5<\x14a\0\xAEW\x80cM}\x9C\x01\x14a\0\xCAW\x80cN\xE6C\xA5\x14a\0\xE6W\x80cR\xF9u6\x14a\x01\x04W\x80cpH\x02u\x14a\x014W[`\0\x80\xFD[a\0\xC8`\x04\x806\x03\x81\x01\x90a\0\xC3\x91\x90a\teV[a\x01\xFCV[\0[a\0\xE4`\x04\x806\x03\x81\x01\x90a\0\xDF\x91\x90a\t\xCAV[a\x02_V[\0[a\0\xEEa\x02\x84V[`@Qa\0\xFB\x91\x90a\n\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1E`\x04\x806\x03\x81\x01\x90a\x01\x19\x91\x90a\nWV[a\x02\x97V[`@Qa\x01+\x91\x90a\n\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01N`\x04\x806\x03\x81\x01\x90a\x01I\x91\x90a\teV[a\x02\xB7V[\0[a\x01Xa\x03\x1AV[\0[a\x01t`\x04\x806\x03\x81\x01\x90a\x01o\x91\x90a\nWV[a\x03.V[`@Qa\x01\x81\x91\x90a\n\x06V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA4`\x04\x806\x03\x81\x01\x90a\x01\x9F\x91\x90a\nWV[a\x03wV[\0[a\x01\xC0`\x04\x806\x03\x81\x01\x90a\x01\xBB\x91\x90a\nWV[a\x04&V[\0[a\x01\xCAa\x04\xD5V[`@Qa\x01\xD7\x91\x90a\n\x93V[`@Q\x80\x91\x03\x90\xF3[a\x01\xFA`\x04\x806\x03\x81\x01\x90a\x01\xF5\x91\x90a\teV[a\x04\xFEV[\0[a\x02\x04a\x05\xB1V[a\x02\x18\x81`\x03a\x06/\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xA3\xB6+\xC3c&\x05-\x97\xEAb\xD6<=`0\x8E\xD4\xC3\xEA\x8A\xC0y\xDD\x84\x99\xF1\xE9\xC4\xF8\x0C\x0F`@Q`@Q\x80\x91\x03\x90\xA2PV[a\x02ga\x05\xB1V[\x80`\x05`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPV[`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x02` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[a\x02\xBFa\x05\xB1V[a\x02\xD3\x81`\x03a\x05\x81\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39`@Q`@Q\x80\x91\x03\x90\xA2PV[a\x03\"a\x05\xB1V[a\x03,`\0a\x06_V[V[`\0`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x03NW`\x01\x90Pa\x03rV[`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P[\x91\x90PV[a\x03\x8B3`\x03a\x07#\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x03\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\xC1\x90a\x0B\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x7F\xE4\xBE\x98\x88j<\x8C\xD9\x02\x7F\xDBD\x06_k\x81QL\\\xF5\xA1\xDA\xB8^\xB7s;\xEBS\x15\x80\xEF`@Q`@Q\x80\x91\x03\x90\xA2PV[a\x04:3`\x03a\x07#\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04yW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04p\x90a\x0B\x0BV[`@Q\x80\x91\x03\x90\xFD[`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x7F\xA6v\xEE~\xED\x1B\x9E\x9E\x90\xC0\xCE\x19d\x91\x9B\x8A\x08K\x89\x1B\xAF\xA6\xB7x\xB6Eq\xF38\xC0\xCD\x95`@Q`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x06a\x05\xB1V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05uW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05l\x90a\x0B\x9DV[`@Q\x80\x91\x03\x90\xFD[a\x05~\x81a\x06_V[PV[`\0a\x05\xA9\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x07SV[\x90P\x92\x91PPV[a\x05\xB9a\x07\xC3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xD7a\x04\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06-W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06$\x90a\x0C\tV[`@Q\x80\x91\x03\x90\xFD[V[`\0a\x06W\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x07\xCBV[\x90P\x92\x91PPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0a\x07K\x83`\0\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x1Ba\x08\xDFV[\x90P\x92\x91PPV[`\0a\x07_\x83\x83a\x08\xDFV[a\x07\xB8W\x82`\0\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x01`\0\x90\x91\x90\x91\x90\x91PU\x82`\0\x01\x80T\x90P\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x90Pa\x07\xBDV[`\0\x90P[\x92\x91PPV[`\x003\x90P\x90V[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0\x81\x14a\x08\xD3W`\0`\x01\x82a\x07\xFD\x91\x90a\x0CbV[\x90P`\0`\x01\x86`\0\x01\x80T\x90Pa\x08\x15\x91\x90a\x0CbV[\x90P\x81\x81\x14a\x08\x84W`\0\x86`\0\x01\x82\x81T\x81\x10a\x086Wa\x085a\x0C\x96V[[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x08ZWa\x08Ya\x0C\x96V[[\x90`\0R` `\0 \x01\x81\x90UP\x83\x87`\x01\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UPP[\x85`\0\x01\x80T\x80a\x08\x98Wa\x08\x97a\x0C\xC5V[[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x08\xD9V[`\0\x91PP[\x92\x91PPV[`\0\x80\x83`\x01\x01`\0\x84\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15\x90P\x92\x91PPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\t2\x82a\t\x07V[\x90P\x91\x90PV[a\tB\x81a\t'V[\x81\x14a\tMW`\0\x80\xFD[PV[`\0\x815\x90Pa\t_\x81a\t9V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t{Wa\tza\t\x02V[[`\0a\t\x89\x84\x82\x85\x01a\tPV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\t\xA7\x81a\t\x92V[\x81\x14a\t\xB2W`\0\x80\xFD[PV[`\0\x815\x90Pa\t\xC4\x81a\t\x9EV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t\xE0Wa\t\xDFa\t\x02V[[`\0a\t\xEE\x84\x82\x85\x01a\t\xB5V[\x91PP\x92\x91PPV[a\n\0\x81a\t\x92V[\x82RPPV[`\0` \x82\x01\x90Pa\n\x1B`\0\x83\x01\x84a\t\xF7V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\n4\x81a\n!V[\x81\x14a\n?W`\0\x80\xFD[PV[`\0\x815\x90Pa\nQ\x81a\n+V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\nmWa\nla\t\x02V[[`\0a\n{\x84\x82\x85\x01a\nBV[\x91PP\x92\x91PPV[a\n\x8D\x81a\t'V[\x82RPPV[`\0` \x82\x01\x90Pa\n\xA8`\0\x83\x01\x84a\n\x84V[\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FNot an admin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\n\xF5`\x0C\x83a\n\xAEV[\x91Pa\x0B\0\x82a\n\xBFV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0B$\x81a\n\xE8V[\x90P\x91\x90PV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x0B\x87`&\x83a\n\xAEV[\x91Pa\x0B\x92\x82a\x0B+V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0B\xB6\x81a\x0BzV[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a\x0B\xF3` \x83a\n\xAEV[\x91Pa\x0B\xFE\x82a\x0B\xBDV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0C\"\x81a\x0B\xE6V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x0Cm\x82a\x0C)V[\x91Pa\x0Cx\x83a\x0C)V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x0C\x90Wa\x0C\x8Fa\x0C3V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD26\xF8\x9E\xEAg-\xC5_V/c#9!\x18\xF8\xE2\x1BET\xE2A\xBF'\x8D{+\xB6\x94\xCF\x04dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static ALLOWLIST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Allowlist<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Allowlist<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Allowlist<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Allowlist<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Allowlist<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Allowlist)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Allowlist<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ALLOWLIST_ABI.clone(),
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
                ALLOWLIST_ABI.clone(),
                ALLOWLIST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `allowAll` (0x4ee643a5) function
        pub fn allow_all(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([78, 230, 67, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowedItems` (0x52f97536) function
        pub fn allowed_items(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 249, 117, 54], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAllowed` (0x78755295) function
        pub fn is_allowed(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([120, 117, 82, 149], key)
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
        ///Calls the contract's `removeAdmin` (0x1785f53c) function
        pub fn remove_admin(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 133, 245, 60], new_admin)
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
        ///Calls the contract's `setAllowAll` (0x4d7d9c01) function
        pub fn set_allow_all(
            &self,
            allow_all: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 125, 156, 1], allow_all)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAllowed` (0x86581597) function
        pub fn set_allowed(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 88, 21, 151], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNotAllowed` (0x8767d9aa) function
        pub fn set_not_allowed(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 103, 217, 170], key)
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
        ///Gets the contract's `AdminAdded` event
        pub fn admin_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AdminRemoved` event
        pub fn admin_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ItemAllowed` event
        pub fn item_allowed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ItemAllowedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ItemNotAllowed` event
        pub fn item_not_allowed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ItemNotAllowedFilter,
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
            AllowlistEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Allowlist<M> {
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
    #[ethevent(name = "AdminAdded", abi = "AdminAdded(address)")]
    pub struct AdminAddedFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "AdminRemoved", abi = "AdminRemoved(address)")]
    pub struct AdminRemovedFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "ItemAllowed", abi = "ItemAllowed(bytes32)")]
    pub struct ItemAllowedFilter {
        #[ethevent(indexed)]
        pub key: [u8; 32],
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
    #[ethevent(name = "ItemNotAllowed", abi = "ItemNotAllowed(bytes32)")]
    pub struct ItemNotAllowedFilter {
        #[ethevent(indexed)]
        pub key: [u8; 32],
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
    pub enum AllowlistEvents {
        AdminAddedFilter(AdminAddedFilter),
        AdminRemovedFilter(AdminRemovedFilter),
        ItemAllowedFilter(ItemAllowedFilter),
        ItemNotAllowedFilter(ItemNotAllowedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for AllowlistEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminAddedFilter::decode_log(log) {
                return Ok(AllowlistEvents::AdminAddedFilter(decoded));
            }
            if let Ok(decoded) = AdminRemovedFilter::decode_log(log) {
                return Ok(AllowlistEvents::AdminRemovedFilter(decoded));
            }
            if let Ok(decoded) = ItemAllowedFilter::decode_log(log) {
                return Ok(AllowlistEvents::ItemAllowedFilter(decoded));
            }
            if let Ok(decoded) = ItemNotAllowedFilter::decode_log(log) {
                return Ok(AllowlistEvents::ItemNotAllowedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AllowlistEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AllowlistEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ItemAllowedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ItemNotAllowedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminAddedFilter> for AllowlistEvents {
        fn from(value: AdminAddedFilter) -> Self {
            Self::AdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<AdminRemovedFilter> for AllowlistEvents {
        fn from(value: AdminRemovedFilter) -> Self {
            Self::AdminRemovedFilter(value)
        }
    }
    impl ::core::convert::From<ItemAllowedFilter> for AllowlistEvents {
        fn from(value: ItemAllowedFilter) -> Self {
            Self::ItemAllowedFilter(value)
        }
    }
    impl ::core::convert::From<ItemNotAllowedFilter> for AllowlistEvents {
        fn from(value: ItemNotAllowedFilter) -> Self {
            Self::ItemNotAllowedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AllowlistEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `allowAll` function with signature `allowAll()` and selector `0x4ee643a5`
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
    #[ethcall(name = "allowAll", abi = "allowAll()")]
    pub struct AllowAllCall;
    ///Container type for all input parameters for the `allowedItems` function with signature `allowedItems(bytes32)` and selector `0x52f97536`
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
    #[ethcall(name = "allowedItems", abi = "allowedItems(bytes32)")]
    pub struct AllowedItemsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `isAllowed` function with signature `isAllowed(bytes32)` and selector `0x78755295`
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
    #[ethcall(name = "isAllowed", abi = "isAllowed(bytes32)")]
    pub struct IsAllowedCall {
        pub key: [u8; 32],
    }
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
        pub new_admin: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `setAllowAll` function with signature `setAllowAll(bool)` and selector `0x4d7d9c01`
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
    #[ethcall(name = "setAllowAll", abi = "setAllowAll(bool)")]
    pub struct SetAllowAllCall {
        pub allow_all: bool,
    }
    ///Container type for all input parameters for the `setAllowed` function with signature `setAllowed(bytes32)` and selector `0x86581597`
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
    #[ethcall(name = "setAllowed", abi = "setAllowed(bytes32)")]
    pub struct SetAllowedCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `setNotAllowed` function with signature `setNotAllowed(bytes32)` and selector `0x8767d9aa`
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
    #[ethcall(name = "setNotAllowed", abi = "setNotAllowed(bytes32)")]
    pub struct SetNotAllowedCall {
        pub key: [u8; 32],
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
    pub enum AllowlistCalls {
        AddAdmin(AddAdminCall),
        AllowAll(AllowAllCall),
        AllowedItems(AllowedItemsCall),
        IsAllowed(IsAllowedCall),
        Owner(OwnerCall),
        RemoveAdmin(RemoveAdminCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetAllowAll(SetAllowAllCall),
        SetAllowed(SetAllowedCall),
        SetNotAllowed(SetNotAllowedCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for AllowlistCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAdmin(decoded));
            }
            if let Ok(decoded) = <AllowAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowAll(decoded));
            }
            if let Ok(decoded) = <AllowedItemsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowedItems(decoded));
            }
            if let Ok(decoded) = <IsAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsAllowed(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RemoveAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAdmin(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetAllowAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAllowAll(decoded));
            }
            if let Ok(decoded) = <SetAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAllowed(decoded));
            }
            if let Ok(decoded) = <SetNotAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNotAllowed(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AllowlistCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowedItems(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAllowAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AllowlistCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowedItems(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAdminCall> for AllowlistCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AllowAllCall> for AllowlistCalls {
        fn from(value: AllowAllCall) -> Self {
            Self::AllowAll(value)
        }
    }
    impl ::core::convert::From<AllowedItemsCall> for AllowlistCalls {
        fn from(value: AllowedItemsCall) -> Self {
            Self::AllowedItems(value)
        }
    }
    impl ::core::convert::From<IsAllowedCall> for AllowlistCalls {
        fn from(value: IsAllowedCall) -> Self {
            Self::IsAllowed(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AllowlistCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemoveAdminCall> for AllowlistCalls {
        fn from(value: RemoveAdminCall) -> Self {
            Self::RemoveAdmin(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AllowlistCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetAllowAllCall> for AllowlistCalls {
        fn from(value: SetAllowAllCall) -> Self {
            Self::SetAllowAll(value)
        }
    }
    impl ::core::convert::From<SetAllowedCall> for AllowlistCalls {
        fn from(value: SetAllowedCall) -> Self {
            Self::SetAllowed(value)
        }
    }
    impl ::core::convert::From<SetNotAllowedCall> for AllowlistCalls {
        fn from(value: SetNotAllowedCall) -> Self {
            Self::SetNotAllowed(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AllowlistCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `allowAll` function with signature `allowAll()` and selector `0x4ee643a5`
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
    pub struct AllowAllReturn(pub bool);
    ///Container type for all return fields from the `allowedItems` function with signature `allowedItems(bytes32)` and selector `0x52f97536`
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
    pub struct AllowedItemsReturn(pub bool);
    ///Container type for all return fields from the `isAllowed` function with signature `isAllowed(bytes32)` and selector `0x78755295`
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
    pub struct IsAllowedReturn(pub bool);
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
}
