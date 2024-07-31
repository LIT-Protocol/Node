pub use staking::*;
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
pub mod staking {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/Staking.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("adminKickValidatorInNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "adminKickValidatorInNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorStakerAddress",
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
                    ::std::borrow::ToOwned::to_owned("adminRejoinValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "adminRejoinValidator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
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
                    ::std::borrow::ToOwned::to_owned("adminResetEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adminResetEpoch"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminSlashValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "adminSlashValidator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorStakerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToPenalize"),
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
                    ::std::borrow::ToOwned::to_owned("advanceEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("advanceEpoch"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkVersion"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Version",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("complaintConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("complaintConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.ComplaintConfig",
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
                    ::std::borrow::ToOwned::to_owned("config"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("config"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Config",
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
                    ::std::borrow::ToOwned::to_owned(
                        "countOfCurrentValidatorsReadyForNextEpoch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "countOfCurrentValidatorsReadyForNextEpoch",
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
                    ::std::borrow::ToOwned::to_owned(
                        "countOfNextValidatorsReadyForNextEpoch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "countOfNextValidatorsReadyForNextEpoch",
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
                    ::std::borrow::ToOwned::to_owned(
                        "currentValidatorCountForConsensus",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentValidatorCountForConsensus",
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
                    ::std::borrow::ToOwned::to_owned("diamondCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("diamondCut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_diamondCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDiamond.FacetCut[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_init"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
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
                    ::std::borrow::ToOwned::to_owned("epoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("epoch"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Epoch",
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
                    ::std::borrow::ToOwned::to_owned("exit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exit"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("facetAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_functionSelector"),
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
                                    name: ::std::borrow::ToOwned::to_owned("facetAddress_"),
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
                    ::std::borrow::ToOwned::to_owned("facetAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddresses"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facetAddresses_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("facetFunctionSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "facetFunctionSelectors",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_facetFunctionSelectors",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("facets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facets_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDiamondLoupe.Facet[]",
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
                    ::std::borrow::ToOwned::to_owned("getKeyTypes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getKeyTypes"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getKickedValidators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getKickedValidators",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxVersion"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Version",
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
                    ::std::borrow::ToOwned::to_owned("getMaxVersionString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMaxVersionString",
                            ),
                            inputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("getMinVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMinVersion"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Version",
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
                    ::std::borrow::ToOwned::to_owned("getMinVersionString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMinVersionString",
                            ),
                            inputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("getNodeStakerAddressMappings"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getNodeStakerAddressMappings",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.AddressMapping[]",
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
                    ::std::borrow::ToOwned::to_owned("getReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReward"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakingBalancesAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakingBalancesAddress",
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
                    ::std::borrow::ToOwned::to_owned("getTokenAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenAddress"),
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
                    ::std::borrow::ToOwned::to_owned("getValidatorsInCurrentEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorsInCurrentEpoch",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getValidatorsInCurrentEpochLength",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorsInCurrentEpochLength",
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
                    ::std::borrow::ToOwned::to_owned("getValidatorsInNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorsInNextEpoch",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getValidatorsStructs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorsStructs",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Validator[]",
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
                    ::std::borrow::ToOwned::to_owned(
                        "getValidatorsStructsInCurrentEpoch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorsStructsInCurrentEpoch",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Validator[]",
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
                    ::std::borrow::ToOwned::to_owned("getValidatorsStructsInNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorsStructsInNextEpoch",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Validator[]",
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
                    ::std::borrow::ToOwned::to_owned("getVotingStatusToKickValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVotingStatusToKickValidator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epochNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorStakerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "voterStakerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("isActiveValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isActiveValidator"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("isActiveValidatorByNodeAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isActiveValidatorByNodeAddress",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("isReadyForNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isReadyForNextEpoch",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("kickPenaltyPercentByReason"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "kickPenaltyPercentByReason",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
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
                    ::std::borrow::ToOwned::to_owned("kickValidatorInNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "kickValidatorInNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorStakerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("lockValidatorsForNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lockValidatorsForNextEpoch",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextValidatorCountForConsensus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "nextValidatorCountForConsensus",
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
                    ::std::borrow::ToOwned::to_owned("nodeAddressToStakerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "nodeAddressToStakerAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nodeAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner_"),
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
                    ::std::borrow::ToOwned::to_owned("readyForNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readyForNextEpoch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("requestToJoin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestToJoin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ip"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ipv6"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("port"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nodeAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("senderPubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverPubKey"),
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
                    ::std::borrow::ToOwned::to_owned("requestToLeave"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestToLeave"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestToLeaveAsNode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestToLeaveAsNode",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setComplaintConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setComplaintConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.ComplaintConfig",
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
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newConfig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Config",
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
                    ::std::borrow::ToOwned::to_owned("setEpochEndTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEpochEndTime"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newEpochEndTime"),
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
                    ::std::borrow::ToOwned::to_owned("setEpochLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEpochLength"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newEpochLength"),
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
                    ::std::borrow::ToOwned::to_owned("setEpochState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEpochState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
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
                    ::std::borrow::ToOwned::to_owned("setEpochTimeout"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEpochTimeout"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newEpochTimeout"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "setIpPortNodeAddressAndCommunicationPubKeys",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setIpPortNodeAddressAndCommunicationPubKeys",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ip"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ipv6"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("port"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nodeAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("senderPubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverPubKey"),
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
                    ::std::borrow::ToOwned::to_owned("setKickPenaltyPercent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setKickPenaltyPercent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newKickPenaltyPercent",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setMaxVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaxVersion"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Version",
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
                    ::std::borrow::ToOwned::to_owned("setMinVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMinVersion"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Version",
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
                    ::std::borrow::ToOwned::to_owned("shouldKickValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "shouldKickValidator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("signalReadyForNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "signalReadyForNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epochNumber"),
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
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("stakeAndJoin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakeAndJoin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ip"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ipv6"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("port"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nodeAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("senderPubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverPubKey"),
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
                    ::std::borrow::ToOwned::to_owned("state"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_interfaceId"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("validators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validators"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakerAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibStakingStorage.Validator",
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ComplaintConfigSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ComplaintConfigSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ConfigSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ConfigSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newTokenRewardPerTokenPerEpoch",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newKeyTypes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMinimumValidatorCount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxConcurrentRequests",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxTripleCount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMinTripleCount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newPeerCheckingIntervalSecs",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxTripleConcurrency",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRpcHealthcheckEnabled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DiamondCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DiamondCut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_diamondCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_init"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EpochEndTimeSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EpochEndTimeSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newEpochEndTime"),
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
                    ::std::borrow::ToOwned::to_owned("EpochLengthSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EpochLengthSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newEpochLength"),
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
                    ::std::borrow::ToOwned::to_owned("EpochTimeoutSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EpochTimeoutSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newEpochTimeout"),
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
                    ::std::borrow::ToOwned::to_owned("KickPenaltyPercentSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "KickPenaltyPercentSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newKickPenaltyPercent",
                                    ),
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
                (
                    ::std::borrow::ToOwned::to_owned("ReadyForNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReadyForNextEpoch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("epochNumber"),
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
                    ::std::borrow::ToOwned::to_owned("Recovered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Recovered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("RequestToJoin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RequestToJoin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequestToLeave"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RequestToLeave"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ResolverContractAddressSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResolverContractAddressSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newResolverContractAddress",
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
                    ::std::borrow::ToOwned::to_owned("RewardsDurationUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RewardsDurationUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDuration"),
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
                    ::std::borrow::ToOwned::to_owned("StakingTokenSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StakingTokenSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newStakingTokenAddress",
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
                    ::std::borrow::ToOwned::to_owned("StateChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StateChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorKickedFromNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ValidatorKickedFromNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountBurned"),
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
                    ::std::borrow::ToOwned::to_owned("ValidatorRejoinedNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ValidatorRejoinedNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VersionRequirementsUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VersionRequirementsUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VotedToKickValidatorInNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VotedToKickValidatorInNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reporter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorStakerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
                    ::std::borrow::ToOwned::to_owned("ActiveValidatorsCannotLeave"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ActiveValidatorsCannotLeave",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallerNotOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallerNotOwner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotAddFunctionToDiamondThatAlreadyExists",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotAddFunctionToDiamondThatAlreadyExists",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotAddSelectorsToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotAddSelectorsToZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selectors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotKickBelowCurrentValidatorThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotKickBelowCurrentValidatorThreshold",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotRejoinUntilNextEpochBecauseKicked",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRejoinUntilNextEpochBecauseKicked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakingAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotRemoveFunctionThatDoesNotExist",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRemoveFunctionThatDoesNotExist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotRemoveImmutableFunction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRemoveImmutableFunction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionThatDoesNotExists",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionThatDoesNotExists",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionsFromFacetWithZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionsFromFacetWithZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selectors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotReplaceImmutableFunction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceImmutableFunction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotReuseCommsKeys"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReuseCommsKeys",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("senderPubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverPubKey"),
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
                    ::std::borrow::ToOwned::to_owned("CannotStakeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CannotStakeZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotVoteTwice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CannotVoteTwice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakerAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotWithdrawZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CannotWithdrawZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CouldNotMapNodeAddressToStakerAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CouldNotMapNodeAddressToStakerAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nodeAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectFacetCutAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IncorrectFacetCutAction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_action"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InitializationFunctionReverted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InitializationFunctionReverted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initializationContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MustBeInActiveOrUnlockedOrPausedState",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MustBeInActiveOrUnlockedOrPausedState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MustBeInActiveOrUnlockedState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MustBeInActiveOrUnlockedState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MustBeInNextValidatorSetLockedOrReadyForNextEpochState",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MustBeInNextValidatorSetLockedOrReadyForNextEpochState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MustBeInNextValidatorSetLockedState",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MustBeInNextValidatorSetLockedState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MustBeInReadyForNextEpochState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MustBeInReadyForNextEpochState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum LibStakingStorage.States",
                                        ),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MustBeValidatorInNextEpochToKick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MustBeValidatorInNextEpochToKick",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakerAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoBytecodeAtAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoBytecodeAtAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NoSelectorsProvidedForFacetForCut",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoSelectorsProvidedForFacetForCut",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facetAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotContractOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotContractOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contractOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NotEnoughTimeElapsedForTimeoutSinceLastEpoch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotEnoughTimeElapsedForTimeoutSinceLastEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epochEndTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timeout"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "NotEnoughTimeElapsedSinceLastEpoch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotEnoughTimeElapsedSinceLastEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("epochEndTime"),
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
                    ::std::borrow::ToOwned::to_owned("NotEnoughValidatorsInNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotEnoughValidatorsInNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validatorCount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "minimumValidatorCount",
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
                    ::std::borrow::ToOwned::to_owned(
                        "NotEnoughValidatorsReadyForNextEpoch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotEnoughValidatorsReadyForNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "currentReadyValidatorCount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nextReadyValidatorCount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "minimumValidatorCountToBeReady",
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
                    ::std::borrow::ToOwned::to_owned(
                        "RemoveFacetAddressMustBeZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RemoveFacetAddressMustBeZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facetAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignaledReadyForWrongEpochNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SignaledReadyForWrongEpochNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "currentEpochNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "receivedEpochNumber",
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
                    ::std::borrow::ToOwned::to_owned("StakerNotPermitted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StakerNotPermitted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakerAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TryingToWithdrawMoreThanStaked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TryingToWithdrawMoreThanStaked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("yourBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestedWithdrawlAmount",
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
                    ::std::borrow::ToOwned::to_owned("ValidatorIsNotInNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ValidatorIsNotInNextEpoch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "validatorsInNextEpoch",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
    pub static STAKING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Staking<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Staking<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Staking<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Staking<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Staking<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Staking)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Staking<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STAKING_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `adminKickValidatorInNextEpoch` (0x7aa086e7) function
        pub fn admin_kick_validator_in_next_epoch(
            &self,
            validator_staker_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 160, 134, 231], validator_staker_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminRejoinValidator` (0x7392c76b) function
        pub fn admin_rejoin_validator(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 146, 199, 107], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminResetEpoch` (0x3c88123d) function
        pub fn admin_reset_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 136, 18, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminSlashValidator` (0x8b80d833) function
        pub fn admin_slash_validator(
            &self,
            validator_staker_address: ::ethers::core::types::Address,
            amount_to_penalize: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [139, 128, 216, 51],
                    (validator_staker_address, amount_to_penalize),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `advanceEpoch` (0x3cf80e6c) function
        pub fn advance_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 248, 14, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkVersion` (0x7720ff0f) function
        pub fn check_version(
            &self,
            version: Version,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([119, 32, 255, 15], (version,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `complaintConfig` (0x02f6da56) function
        pub fn complaint_config(
            &self,
            reason: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ComplaintConfig> {
            self.0
                .method_hash([2, 246, 218, 86], reason)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `config` (0x79502c55) function
        pub fn config(&self) -> ::ethers::contract::builders::ContractCall<M, Config> {
            self.0
                .method_hash([121, 80, 44, 85], ())
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
        ///Calls the contract's `countOfCurrentValidatorsReadyForNextEpoch` (0xe8684ed1) function
        pub fn count_of_current_validators_ready_for_next_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 104, 78, 209], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `countOfNextValidatorsReadyForNextEpoch` (0x89965883) function
        pub fn count_of_next_validators_ready_for_next_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([137, 150, 88, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentValidatorCountForConsensus` (0x43cb0a0e) function
        pub fn current_validator_count_for_consensus(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([67, 203, 10, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `diamondCut` (0x1f931c1c) function
        pub fn diamond_cut(
            &self,
            diamond_cut: ::std::vec::Vec<FacetCut>,
            init: ::ethers::core::types::Address,
            calldata: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 147, 28, 28], (diamond_cut, init, calldata))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `epoch` (0x900cf0cf) function
        pub fn epoch(&self) -> ::ethers::contract::builders::ContractCall<M, Epoch> {
            self.0
                .method_hash([144, 12, 240, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exit` (0xe9fad8ee) function
        pub fn exit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 250, 216, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetAddress` (0xcdffacc6) function
        pub fn facet_address(
            &self,
            function_selector: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([205, 255, 172, 198], function_selector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetAddresses` (0x52ef6b2c) function
        pub fn facet_addresses(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([82, 239, 107, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetFunctionSelectors` (0xadfca15e) function
        pub fn facet_function_selectors(
            &self,
            facet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([173, 252, 161, 94], facet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facets` (0x7a0ed627) function
        pub fn facets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Facet>> {
            self.0
                .method_hash([122, 14, 214, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getKeyTypes` (0xf1b877a8) function
        pub fn get_key_types(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([241, 184, 119, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getKickedValidators` (0x5995a4c4) function
        pub fn get_kicked_validators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([89, 149, 164, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxVersion` (0x2c20d23a) function
        pub fn get_max_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Version> {
            self.0
                .method_hash([44, 32, 210, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxVersionString` (0xdbd68aa7) function
        pub fn get_max_version_string(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([219, 214, 138, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinVersion` (0xd1e3f3d7) function
        pub fn get_min_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Version> {
            self.0
                .method_hash([209, 227, 243, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinVersionString` (0x517d2324) function
        pub fn get_min_version_string(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([81, 125, 35, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNodeStakerAddressMappings` (0x90fba112) function
        pub fn get_node_staker_address_mappings(
            &self,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<AddressMapping>,
        > {
            self.0
                .method_hash([144, 251, 161, 18], addresses)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReward` (0x3d18b912) function
        pub fn get_reward(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 24, 185, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakingBalancesAddress` (0x5b677eac) function
        pub fn get_staking_balances_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([91, 103, 126, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenAddress` (0x10fe9ae8) function
        pub fn get_token_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([16, 254, 154, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorsInCurrentEpoch` (0x857b7663) function
        pub fn get_validators_in_current_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([133, 123, 118, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorsInCurrentEpochLength` (0xd4818fca) function
        pub fn get_validators_in_current_epoch_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([212, 129, 143, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorsInNextEpoch` (0xc35d4d09) function
        pub fn get_validators_in_next_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([195, 93, 77, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorsStructs` (0x533d463e) function
        pub fn get_validators_structs(
            &self,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Validator>> {
            self.0
                .method_hash([83, 61, 70, 62], addresses)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorsStructsInCurrentEpoch` (0xe7c08720) function
        pub fn get_validators_structs_in_current_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Validator>> {
            self.0
                .method_hash([231, 192, 135, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorsStructsInNextEpoch` (0x61dee8a3) function
        pub fn get_validators_structs_in_next_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Validator>> {
            self.0
                .method_hash([97, 222, 232, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotingStatusToKickValidator` (0x70fe276a) function
        pub fn get_voting_status_to_kick_validator(
            &self,
            epoch_number: ::ethers::core::types::U256,
            validator_staker_address: ::ethers::core::types::Address,
            voter_staker_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash(
                    [112, 254, 39, 106],
                    (epoch_number, validator_staker_address, voter_staker_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isActiveValidator` (0x40550a1c) function
        pub fn is_active_validator(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([64, 85, 10, 28], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isActiveValidatorByNodeAddress` (0xa25e49a4) function
        pub fn is_active_validator_by_node_address(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([162, 94, 73, 164], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isReadyForNextEpoch` (0xf1887fec) function
        pub fn is_ready_for_next_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([241, 136, 127, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kickPenaltyPercentByReason` (0x3e685266) function
        pub fn kick_penalty_percent_by_reason(
            &self,
            reason: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 104, 82, 102], reason)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kickValidatorInNextEpoch` (0x865419e9) function
        pub fn kick_validator_in_next_epoch(
            &self,
            validator_staker_address: ::ethers::core::types::Address,
            reason: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [134, 84, 25, 233],
                    (validator_staker_address, reason, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockValidatorsForNextEpoch` (0x16930f4d) function
        pub fn lock_validators_for_next_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 147, 15, 77], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextValidatorCountForConsensus` (0x0297d4db) function
        pub fn next_validator_count_for_consensus(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([2, 151, 212, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nodeAddressToStakerAddress` (0x5081f66f) function
        pub fn node_address_to_staker_address(
            &self,
            node_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 129, 246, 111], node_address)
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
        ///Calls the contract's `readyForNextEpoch` (0x519877eb) function
        pub fn ready_for_next_epoch(
            &self,
            staker_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([81, 152, 119, 235], staker_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestToJoin` (0x3528db88) function
        pub fn request_to_join(
            &self,
            ip: u32,
            ipv_6: u128,
            port: u32,
            node_address: ::ethers::core::types::Address,
            sender_pub_key: ::ethers::core::types::U256,
            receiver_pub_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [53, 40, 219, 136],
                    (ip, ipv_6, port, node_address, sender_pub_key, receiver_pub_key),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestToLeave` (0xac2f8afe) function
        pub fn request_to_leave(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 47, 138, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestToLeaveAsNode` (0x8a3d070e) function
        pub fn request_to_leave_as_node(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 61, 7, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setComplaintConfig` (0xdc509d50) function
        pub fn set_complaint_config(
            &self,
            reason: ::ethers::core::types::U256,
            config: ComplaintConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 80, 157, 80], (reason, config))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0x87389dd7) function
        pub fn set_config(
            &self,
            new_config: Config,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 56, 157, 215], (new_config,))
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
        ///Calls the contract's `setEpochEndTime` (0x4a6e51f5) function
        pub fn set_epoch_end_time(
            &self,
            new_epoch_end_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 110, 81, 245], new_epoch_end_time)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEpochLength` (0x54eea796) function
        pub fn set_epoch_length(
            &self,
            new_epoch_length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 238, 167, 150], new_epoch_length)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEpochState` (0x3f819713) function
        pub fn set_epoch_state(
            &self,
            new_state: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 129, 151, 19], new_state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEpochTimeout` (0x1fab87c4) function
        pub fn set_epoch_timeout(
            &self,
            new_epoch_timeout: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 171, 135, 196], new_epoch_timeout)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setIpPortNodeAddressAndCommunicationPubKeys` (0x4f8f0102) function
        pub fn set_ip_port_node_address_and_communication_pub_keys(
            &self,
            ip: u32,
            ipv_6: u128,
            port: u32,
            node_address: ::ethers::core::types::Address,
            sender_pub_key: ::ethers::core::types::U256,
            receiver_pub_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [79, 143, 1, 2],
                    (ip, ipv_6, port, node_address, sender_pub_key, receiver_pub_key),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setKickPenaltyPercent` (0x09c7c7d0) function
        pub fn set_kick_penalty_percent(
            &self,
            reason: ::ethers::core::types::U256,
            new_kick_penalty_percent: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 199, 199, 208], (reason, new_kick_penalty_percent))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxVersion` (0xec159d7a) function
        pub fn set_max_version(
            &self,
            version: Version,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 21, 157, 122], (version,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinVersion` (0xd900fcbb) function
        pub fn set_min_version(
            &self,
            version: Version,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 0, 252, 187], (version,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shouldKickValidator` (0x847e0625) function
        pub fn should_kick_validator(
            &self,
            staker_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([132, 126, 6, 37], staker_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signalReadyForNextEpoch` (0xf99b5623) function
        pub fn signal_ready_for_next_epoch(
            &self,
            epoch_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 155, 86, 35], epoch_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0xa694fc3a) function
        pub fn stake(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 148, 252, 58], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeAndJoin` (0xba3bd22e) function
        pub fn stake_and_join(
            &self,
            amount: ::ethers::core::types::U256,
            ip: u32,
            ipv_6: u128,
            port: u32,
            node_address: ::ethers::core::types::Address,
            sender_pub_key: ::ethers::core::types::U256,
            receiver_pub_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [186, 59, 210, 46],
                    (
                        amount,
                        ip,
                        ipv_6,
                        port,
                        node_address,
                        sender_pub_key,
                        receiver_pub_key,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `state` (0xc19d93fb) function
        pub fn state(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([193, 157, 147, 251], ())
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validators` (0xfa52c7d8) function
        pub fn validators(
            &self,
            staker_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Validator> {
            self.0
                .method_hash([250, 82, 199, 216], staker_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ComplaintConfigSet` event
        pub fn complaint_config_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ComplaintConfigSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ConfigSet` event
        pub fn config_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ConfigSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DiamondCut` event
        pub fn diamond_cut_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DiamondCutFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EpochEndTimeSet` event
        pub fn epoch_end_time_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EpochEndTimeSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EpochLengthSet` event
        pub fn epoch_length_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EpochLengthSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EpochTimeoutSet` event
        pub fn epoch_timeout_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EpochTimeoutSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `KickPenaltyPercentSet` event
        pub fn kick_penalty_percent_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            KickPenaltyPercentSetFilter,
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
        ///Gets the contract's `ReadyForNextEpoch` event
        pub fn ready_for_next_epoch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReadyForNextEpochFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Recovered` event
        pub fn recovered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RecoveredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestToJoin` event
        pub fn request_to_join_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestToJoinFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestToLeave` event
        pub fn request_to_leave_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestToLeaveFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ResolverContractAddressSet` event
        pub fn resolver_contract_address_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ResolverContractAddressSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RewardsDurationUpdated` event
        pub fn rewards_duration_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsDurationUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakingTokenSet` event
        pub fn staking_token_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakingTokenSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StateChanged` event
        pub fn state_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StateChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorKickedFromNextEpoch` event
        pub fn validator_kicked_from_next_epoch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorKickedFromNextEpochFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorRejoinedNextEpoch` event
        pub fn validator_rejoined_next_epoch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorRejoinedNextEpochFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VersionRequirementsUpdated` event
        pub fn version_requirements_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VersionRequirementsUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VotedToKickValidatorInNextEpoch` event
        pub fn voted_to_kick_validator_in_next_epoch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VotedToKickValidatorInNextEpochFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakingEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Staking<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActiveValidatorsCannotLeave` with signature `ActiveValidatorsCannotLeave()` and selector `0x74fc692a`
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
        name = "ActiveValidatorsCannotLeave",
        abi = "ActiveValidatorsCannotLeave()"
    )]
    pub struct ActiveValidatorsCannotLeave;
    ///Custom Error type `CallerNotOwner` with signature `CallerNotOwner()` and selector `0x5cd83192`
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
    #[etherror(name = "CallerNotOwner", abi = "CallerNotOwner()")]
    pub struct CallerNotOwner;
    ///Custom Error type `CannotAddFunctionToDiamondThatAlreadyExists` with signature `CannotAddFunctionToDiamondThatAlreadyExists(bytes4)` and selector `0xebbf5d07`
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
        name = "CannotAddFunctionToDiamondThatAlreadyExists",
        abi = "CannotAddFunctionToDiamondThatAlreadyExists(bytes4)"
    )]
    pub struct CannotAddFunctionToDiamondThatAlreadyExists {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotAddSelectorsToZeroAddress` with signature `CannotAddSelectorsToZeroAddress(bytes4[])` and selector `0x0ae3681c`
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
        name = "CannotAddSelectorsToZeroAddress",
        abi = "CannotAddSelectorsToZeroAddress(bytes4[])"
    )]
    pub struct CannotAddSelectorsToZeroAddress {
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Custom Error type `CannotKickBelowCurrentValidatorThreshold` with signature `CannotKickBelowCurrentValidatorThreshold()` and selector `0xe81f2804`
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
        name = "CannotKickBelowCurrentValidatorThreshold",
        abi = "CannotKickBelowCurrentValidatorThreshold()"
    )]
    pub struct CannotKickBelowCurrentValidatorThreshold;
    ///Custom Error type `CannotRejoinUntilNextEpochBecauseKicked` with signature `CannotRejoinUntilNextEpochBecauseKicked(address)` and selector `0x7c6d6c6b`
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
        name = "CannotRejoinUntilNextEpochBecauseKicked",
        abi = "CannotRejoinUntilNextEpochBecauseKicked(address)"
    )]
    pub struct CannotRejoinUntilNextEpochBecauseKicked {
        pub staking_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `CannotRemoveFunctionThatDoesNotExist` with signature `CannotRemoveFunctionThatDoesNotExist(bytes4)` and selector `0x7a08a22d`
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
        name = "CannotRemoveFunctionThatDoesNotExist",
        abi = "CannotRemoveFunctionThatDoesNotExist(bytes4)"
    )]
    pub struct CannotRemoveFunctionThatDoesNotExist {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotRemoveImmutableFunction` with signature `CannotRemoveImmutableFunction(bytes4)` and selector `0x6fafeb08`
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
        name = "CannotRemoveImmutableFunction",
        abi = "CannotRemoveImmutableFunction(bytes4)"
    )]
    pub struct CannotRemoveImmutableFunction {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionThatDoesNotExists` with signature `CannotReplaceFunctionThatDoesNotExists(bytes4)` and selector `0x7479f939`
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
        name = "CannotReplaceFunctionThatDoesNotExists",
        abi = "CannotReplaceFunctionThatDoesNotExists(bytes4)"
    )]
    pub struct CannotReplaceFunctionThatDoesNotExists {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet` with signature `CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)` and selector `0x358d9d1a`
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
        name = "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
        abi = "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)"
    )]
    pub struct CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionsFromFacetWithZeroAddress` with signature `CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])` and selector `0xcd98a96f`
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
        name = "CannotReplaceFunctionsFromFacetWithZeroAddress",
        abi = "CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])"
    )]
    pub struct CannotReplaceFunctionsFromFacetWithZeroAddress {
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Custom Error type `CannotReplaceImmutableFunction` with signature `CannotReplaceImmutableFunction(bytes4)` and selector `0x520300da`
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
        name = "CannotReplaceImmutableFunction",
        abi = "CannotReplaceImmutableFunction(bytes4)"
    )]
    pub struct CannotReplaceImmutableFunction {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReuseCommsKeys` with signature `CannotReuseCommsKeys(uint256,uint256)` and selector `0x1179010e`
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
        name = "CannotReuseCommsKeys",
        abi = "CannotReuseCommsKeys(uint256,uint256)"
    )]
    pub struct CannotReuseCommsKeys {
        pub sender_pub_key: ::ethers::core::types::U256,
        pub receiver_pub_key: ::ethers::core::types::U256,
    }
    ///Custom Error type `CannotStakeZero` with signature `CannotStakeZero()` and selector `0x6a76ff9f`
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
    #[etherror(name = "CannotStakeZero", abi = "CannotStakeZero()")]
    pub struct CannotStakeZero;
    ///Custom Error type `CannotVoteTwice` with signature `CannotVoteTwice(address)` and selector `0x384ce38a`
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
    #[etherror(name = "CannotVoteTwice", abi = "CannotVoteTwice(address)")]
    pub struct CannotVoteTwice {
        pub staker_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `CannotWithdrawZero` with signature `CannotWithdrawZero()` and selector `0xc3771360`
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
    #[etherror(name = "CannotWithdrawZero", abi = "CannotWithdrawZero()")]
    pub struct CannotWithdrawZero;
    ///Custom Error type `CouldNotMapNodeAddressToStakerAddress` with signature `CouldNotMapNodeAddressToStakerAddress(address)` and selector `0x64ffeb3d`
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
        name = "CouldNotMapNodeAddressToStakerAddress",
        abi = "CouldNotMapNodeAddressToStakerAddress(address)"
    )]
    pub struct CouldNotMapNodeAddressToStakerAddress {
        pub node_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `IncorrectFacetCutAction` with signature `IncorrectFacetCutAction(uint8)` and selector `0x7fe9a41e`
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
    #[etherror(name = "IncorrectFacetCutAction", abi = "IncorrectFacetCutAction(uint8)")]
    pub struct IncorrectFacetCutAction {
        pub action: u8,
    }
    ///Custom Error type `InitializationFunctionReverted` with signature `InitializationFunctionReverted(address,bytes)` and selector `0x192105d7`
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
        name = "InitializationFunctionReverted",
        abi = "InitializationFunctionReverted(address,bytes)"
    )]
    pub struct InitializationFunctionReverted {
        pub initialization_contract_address: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `MustBeInActiveOrUnlockedOrPausedState` with signature `MustBeInActiveOrUnlockedOrPausedState(uint8)` and selector `0xc1f8741d`
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
        name = "MustBeInActiveOrUnlockedOrPausedState",
        abi = "MustBeInActiveOrUnlockedOrPausedState(uint8)"
    )]
    pub struct MustBeInActiveOrUnlockedOrPausedState {
        pub state: u8,
    }
    ///Custom Error type `MustBeInActiveOrUnlockedState` with signature `MustBeInActiveOrUnlockedState(uint8)` and selector `0x9ef5b6f5`
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
        name = "MustBeInActiveOrUnlockedState",
        abi = "MustBeInActiveOrUnlockedState(uint8)"
    )]
    pub struct MustBeInActiveOrUnlockedState {
        pub state: u8,
    }
    ///Custom Error type `MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState` with signature `MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(uint8)` and selector `0xf928cb7b`
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
        name = "MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState",
        abi = "MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(uint8)"
    )]
    pub struct MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState {
        pub state: u8,
    }
    ///Custom Error type `MustBeInNextValidatorSetLockedOrReadyForNextEpochState` with signature `MustBeInNextValidatorSetLockedOrReadyForNextEpochState(uint8)` and selector `0xe1b4c12e`
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
        name = "MustBeInNextValidatorSetLockedOrReadyForNextEpochState",
        abi = "MustBeInNextValidatorSetLockedOrReadyForNextEpochState(uint8)"
    )]
    pub struct MustBeInNextValidatorSetLockedOrReadyForNextEpochState {
        pub state: u8,
    }
    ///Custom Error type `MustBeInNextValidatorSetLockedState` with signature `MustBeInNextValidatorSetLockedState(uint8)` and selector `0x7203d9de`
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
        name = "MustBeInNextValidatorSetLockedState",
        abi = "MustBeInNextValidatorSetLockedState(uint8)"
    )]
    pub struct MustBeInNextValidatorSetLockedState {
        pub state: u8,
    }
    ///Custom Error type `MustBeInReadyForNextEpochState` with signature `MustBeInReadyForNextEpochState(uint8)` and selector `0x17ce3ae1`
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
        name = "MustBeInReadyForNextEpochState",
        abi = "MustBeInReadyForNextEpochState(uint8)"
    )]
    pub struct MustBeInReadyForNextEpochState {
        pub state: u8,
    }
    ///Custom Error type `MustBeValidatorInNextEpochToKick` with signature `MustBeValidatorInNextEpochToKick(address)` and selector `0x5f543082`
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
        name = "MustBeValidatorInNextEpochToKick",
        abi = "MustBeValidatorInNextEpochToKick(address)"
    )]
    pub struct MustBeValidatorInNextEpochToKick {
        pub staker_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `NoBytecodeAtAddress` with signature `NoBytecodeAtAddress(address,string)` and selector `0x919834b9`
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
        name = "NoBytecodeAtAddress",
        abi = "NoBytecodeAtAddress(address,string)"
    )]
    pub struct NoBytecodeAtAddress {
        pub contract_address: ::ethers::core::types::Address,
        pub message: ::std::string::String,
    }
    ///Custom Error type `NoSelectorsProvidedForFacetForCut` with signature `NoSelectorsProvidedForFacetForCut(address)` and selector `0xe767f91f`
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
        name = "NoSelectorsProvidedForFacetForCut",
        abi = "NoSelectorsProvidedForFacetForCut(address)"
    )]
    pub struct NoSelectorsProvidedForFacetForCut {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `NotContractOwner` with signature `NotContractOwner(address,address)` and selector `0xff4127cb`
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
    #[etherror(name = "NotContractOwner", abi = "NotContractOwner(address,address)")]
    pub struct NotContractOwner {
        pub user: ::ethers::core::types::Address,
        pub contract_owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `NotEnoughTimeElapsedForTimeoutSinceLastEpoch` with signature `NotEnoughTimeElapsedForTimeoutSinceLastEpoch(uint256,uint256,uint256)` and selector `0x9312e856`
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
        name = "NotEnoughTimeElapsedForTimeoutSinceLastEpoch",
        abi = "NotEnoughTimeElapsedForTimeoutSinceLastEpoch(uint256,uint256,uint256)"
    )]
    pub struct NotEnoughTimeElapsedForTimeoutSinceLastEpoch {
        pub current_timestamp: ::ethers::core::types::U256,
        pub epoch_end_time: ::ethers::core::types::U256,
        pub timeout: ::ethers::core::types::U256,
    }
    ///Custom Error type `NotEnoughTimeElapsedSinceLastEpoch` with signature `NotEnoughTimeElapsedSinceLastEpoch(uint256,uint256)` and selector `0xf44bc0a7`
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
        name = "NotEnoughTimeElapsedSinceLastEpoch",
        abi = "NotEnoughTimeElapsedSinceLastEpoch(uint256,uint256)"
    )]
    pub struct NotEnoughTimeElapsedSinceLastEpoch {
        pub current_timestamp: ::ethers::core::types::U256,
        pub epoch_end_time: ::ethers::core::types::U256,
    }
    ///Custom Error type `NotEnoughValidatorsInNextEpoch` with signature `NotEnoughValidatorsInNextEpoch(uint256,uint256)` and selector `0x8a0defa4`
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
        name = "NotEnoughValidatorsInNextEpoch",
        abi = "NotEnoughValidatorsInNextEpoch(uint256,uint256)"
    )]
    pub struct NotEnoughValidatorsInNextEpoch {
        pub validator_count: ::ethers::core::types::U256,
        pub minimum_validator_count: ::ethers::core::types::U256,
    }
    ///Custom Error type `NotEnoughValidatorsReadyForNextEpoch` with signature `NotEnoughValidatorsReadyForNextEpoch(uint256,uint256,uint256)` and selector `0x26d6b3de`
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
        name = "NotEnoughValidatorsReadyForNextEpoch",
        abi = "NotEnoughValidatorsReadyForNextEpoch(uint256,uint256,uint256)"
    )]
    pub struct NotEnoughValidatorsReadyForNextEpoch {
        pub current_ready_validator_count: ::ethers::core::types::U256,
        pub next_ready_validator_count: ::ethers::core::types::U256,
        pub minimum_validator_count_to_be_ready: ::ethers::core::types::U256,
    }
    ///Custom Error type `RemoveFacetAddressMustBeZeroAddress` with signature `RemoveFacetAddressMustBeZeroAddress(address)` and selector `0xd091bc81`
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
        name = "RemoveFacetAddressMustBeZeroAddress",
        abi = "RemoveFacetAddressMustBeZeroAddress(address)"
    )]
    pub struct RemoveFacetAddressMustBeZeroAddress {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `SignaledReadyForWrongEpochNumber` with signature `SignaledReadyForWrongEpochNumber(uint256,uint256)` and selector `0x068cde2a`
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
        name = "SignaledReadyForWrongEpochNumber",
        abi = "SignaledReadyForWrongEpochNumber(uint256,uint256)"
    )]
    pub struct SignaledReadyForWrongEpochNumber {
        pub current_epoch_number: ::ethers::core::types::U256,
        pub received_epoch_number: ::ethers::core::types::U256,
    }
    ///Custom Error type `StakerNotPermitted` with signature `StakerNotPermitted(address)` and selector `0x924a5910`
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
    #[etherror(name = "StakerNotPermitted", abi = "StakerNotPermitted(address)")]
    pub struct StakerNotPermitted {
        pub staker_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `TryingToWithdrawMoreThanStaked` with signature `TryingToWithdrawMoreThanStaked(uint256,uint256)` and selector `0xfdf3c18d`
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
        name = "TryingToWithdrawMoreThanStaked",
        abi = "TryingToWithdrawMoreThanStaked(uint256,uint256)"
    )]
    pub struct TryingToWithdrawMoreThanStaked {
        pub your_balance: ::ethers::core::types::U256,
        pub requested_withdrawl_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `ValidatorIsNotInNextEpoch` with signature `ValidatorIsNotInNextEpoch(address,address[])` and selector `0xa3113c0e`
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
        name = "ValidatorIsNotInNextEpoch",
        abi = "ValidatorIsNotInNextEpoch(address,address[])"
    )]
    pub struct ValidatorIsNotInNextEpoch {
        pub validator: ::ethers::core::types::Address,
        pub validators_in_next_epoch: ::std::vec::Vec<::ethers::core::types::Address>,
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
    pub enum StakingErrors {
        ActiveValidatorsCannotLeave(ActiveValidatorsCannotLeave),
        CallerNotOwner(CallerNotOwner),
        CannotAddFunctionToDiamondThatAlreadyExists(
            CannotAddFunctionToDiamondThatAlreadyExists,
        ),
        CannotAddSelectorsToZeroAddress(CannotAddSelectorsToZeroAddress),
        CannotKickBelowCurrentValidatorThreshold(
            CannotKickBelowCurrentValidatorThreshold,
        ),
        CannotRejoinUntilNextEpochBecauseKicked(CannotRejoinUntilNextEpochBecauseKicked),
        CannotRemoveFunctionThatDoesNotExist(CannotRemoveFunctionThatDoesNotExist),
        CannotRemoveImmutableFunction(CannotRemoveImmutableFunction),
        CannotReplaceFunctionThatDoesNotExists(CannotReplaceFunctionThatDoesNotExists),
        CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
            CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ),
        CannotReplaceFunctionsFromFacetWithZeroAddress(
            CannotReplaceFunctionsFromFacetWithZeroAddress,
        ),
        CannotReplaceImmutableFunction(CannotReplaceImmutableFunction),
        CannotReuseCommsKeys(CannotReuseCommsKeys),
        CannotStakeZero(CannotStakeZero),
        CannotVoteTwice(CannotVoteTwice),
        CannotWithdrawZero(CannotWithdrawZero),
        CouldNotMapNodeAddressToStakerAddress(CouldNotMapNodeAddressToStakerAddress),
        IncorrectFacetCutAction(IncorrectFacetCutAction),
        InitializationFunctionReverted(InitializationFunctionReverted),
        MustBeInActiveOrUnlockedOrPausedState(MustBeInActiveOrUnlockedOrPausedState),
        MustBeInActiveOrUnlockedState(MustBeInActiveOrUnlockedState),
        MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
            MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState,
        ),
        MustBeInNextValidatorSetLockedOrReadyForNextEpochState(
            MustBeInNextValidatorSetLockedOrReadyForNextEpochState,
        ),
        MustBeInNextValidatorSetLockedState(MustBeInNextValidatorSetLockedState),
        MustBeInReadyForNextEpochState(MustBeInReadyForNextEpochState),
        MustBeValidatorInNextEpochToKick(MustBeValidatorInNextEpochToKick),
        NoBytecodeAtAddress(NoBytecodeAtAddress),
        NoSelectorsProvidedForFacetForCut(NoSelectorsProvidedForFacetForCut),
        NotContractOwner(NotContractOwner),
        NotEnoughTimeElapsedForTimeoutSinceLastEpoch(
            NotEnoughTimeElapsedForTimeoutSinceLastEpoch,
        ),
        NotEnoughTimeElapsedSinceLastEpoch(NotEnoughTimeElapsedSinceLastEpoch),
        NotEnoughValidatorsInNextEpoch(NotEnoughValidatorsInNextEpoch),
        NotEnoughValidatorsReadyForNextEpoch(NotEnoughValidatorsReadyForNextEpoch),
        RemoveFacetAddressMustBeZeroAddress(RemoveFacetAddressMustBeZeroAddress),
        SignaledReadyForWrongEpochNumber(SignaledReadyForWrongEpochNumber),
        StakerNotPermitted(StakerNotPermitted),
        TryingToWithdrawMoreThanStaked(TryingToWithdrawMoreThanStaked),
        ValidatorIsNotInNextEpoch(ValidatorIsNotInNextEpoch),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for StakingErrors {
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
                = <ActiveValidatorsCannotLeave as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ActiveValidatorsCannotLeave(decoded));
            }
            if let Ok(decoded)
                = <CallerNotOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerNotOwner(decoded));
            }
            if let Ok(decoded)
                = <CannotAddFunctionToDiamondThatAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotAddFunctionToDiamondThatAlreadyExists(decoded));
            }
            if let Ok(decoded)
                = <CannotAddSelectorsToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotAddSelectorsToZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <CannotKickBelowCurrentValidatorThreshold as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotKickBelowCurrentValidatorThreshold(decoded));
            }
            if let Ok(decoded)
                = <CannotRejoinUntilNextEpochBecauseKicked as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotRejoinUntilNextEpochBecauseKicked(decoded));
            }
            if let Ok(decoded)
                = <CannotRemoveFunctionThatDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotRemoveFunctionThatDoesNotExist(decoded));
            }
            if let Ok(decoded)
                = <CannotRemoveImmutableFunction as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotRemoveImmutableFunction(decoded));
            }
            if let Ok(decoded)
                = <CannotReplaceFunctionThatDoesNotExists as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotReplaceFunctionThatDoesNotExists(decoded));
            }
            if let Ok(decoded)
                = <CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = <CannotReplaceFunctionsFromFacetWithZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotReplaceFunctionsFromFacetWithZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <CannotReplaceImmutableFunction as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotReplaceImmutableFunction(decoded));
            }
            if let Ok(decoded)
                = <CannotReuseCommsKeys as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotReuseCommsKeys(decoded));
            }
            if let Ok(decoded)
                = <CannotStakeZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotStakeZero(decoded));
            }
            if let Ok(decoded)
                = <CannotVoteTwice as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotVoteTwice(decoded));
            }
            if let Ok(decoded)
                = <CannotWithdrawZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotWithdrawZero(decoded));
            }
            if let Ok(decoded)
                = <CouldNotMapNodeAddressToStakerAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CouldNotMapNodeAddressToStakerAddress(decoded));
            }
            if let Ok(decoded)
                = <IncorrectFacetCutAction as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncorrectFacetCutAction(decoded));
            }
            if let Ok(decoded)
                = <InitializationFunctionReverted as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InitializationFunctionReverted(decoded));
            }
            if let Ok(decoded)
                = <MustBeInActiveOrUnlockedOrPausedState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MustBeInActiveOrUnlockedOrPausedState(decoded));
            }
            if let Ok(decoded)
                = <MustBeInActiveOrUnlockedState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MustBeInActiveOrUnlockedState(decoded));
            }
            if let Ok(decoded)
                = <MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = <MustBeInNextValidatorSetLockedOrReadyForNextEpochState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochState(decoded),
                );
            }
            if let Ok(decoded)
                = <MustBeInNextValidatorSetLockedState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MustBeInNextValidatorSetLockedState(decoded));
            }
            if let Ok(decoded)
                = <MustBeInReadyForNextEpochState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MustBeInReadyForNextEpochState(decoded));
            }
            if let Ok(decoded)
                = <MustBeValidatorInNextEpochToKick as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MustBeValidatorInNextEpochToKick(decoded));
            }
            if let Ok(decoded)
                = <NoBytecodeAtAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoBytecodeAtAddress(decoded));
            }
            if let Ok(decoded)
                = <NoSelectorsProvidedForFacetForCut as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoSelectorsProvidedForFacetForCut(decoded));
            }
            if let Ok(decoded)
                = <NotContractOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotContractOwner(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughTimeElapsedForTimeoutSinceLastEpoch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotEnoughTimeElapsedForTimeoutSinceLastEpoch(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughTimeElapsedSinceLastEpoch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotEnoughTimeElapsedSinceLastEpoch(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughValidatorsInNextEpoch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotEnoughValidatorsInNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughValidatorsReadyForNextEpoch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotEnoughValidatorsReadyForNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <RemoveFacetAddressMustBeZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveFacetAddressMustBeZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <SignaledReadyForWrongEpochNumber as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SignaledReadyForWrongEpochNumber(decoded));
            }
            if let Ok(decoded)
                = <StakerNotPermitted as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakerNotPermitted(decoded));
            }
            if let Ok(decoded)
                = <TryingToWithdrawMoreThanStaked as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TryingToWithdrawMoreThanStaked(decoded));
            }
            if let Ok(decoded)
                = <ValidatorIsNotInNextEpoch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidatorIsNotInNextEpoch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakingErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActiveValidatorsCannotLeave(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotKickBelowCurrentValidatorThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRejoinUntilNextEpochBecauseKicked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRemoveFunctionThatDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRemoveImmutableFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceFunctionThatDoesNotExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CannotReplaceFunctionsFromFacetWithZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceImmutableFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReuseCommsKeys(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotStakeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotVoteTwice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotWithdrawZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CouldNotMapNodeAddressToStakerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeInActiveOrUnlockedOrPausedState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeInActiveOrUnlockedState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeInNextValidatorSetLockedState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeInReadyForNextEpochState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MustBeValidatorInNextEpochToKick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotContractOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughTimeElapsedForTimeoutSinceLastEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughTimeElapsedSinceLastEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughValidatorsInNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughValidatorsReadyForNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignaledReadyForWrongEpochNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerNotPermitted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryingToWithdrawMoreThanStaked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorIsNotInNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for StakingErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ActiveValidatorsCannotLeave as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CallerNotOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotAddFunctionToDiamondThatAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotAddSelectorsToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotKickBelowCurrentValidatorThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRejoinUntilNextEpochBecauseKicked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRemoveFunctionThatDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRemoveImmutableFunction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionThatDoesNotExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionsFromFacetWithZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceImmutableFunction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReuseCommsKeys as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotStakeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotVoteTwice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotWithdrawZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CouldNotMapNodeAddressToStakerAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectFacetCutAction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InitializationFunctionReverted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeInActiveOrUnlockedOrPausedState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeInActiveOrUnlockedState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeInNextValidatorSetLockedOrReadyForNextEpochState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeInNextValidatorSetLockedState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeInReadyForNextEpochState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MustBeValidatorInNextEpochToKick as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoBytecodeAtAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoSelectorsProvidedForFacetForCut as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotContractOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughTimeElapsedForTimeoutSinceLastEpoch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughTimeElapsedSinceLastEpoch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughValidatorsInNextEpoch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughValidatorsReadyForNextEpoch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RemoveFacetAddressMustBeZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SignaledReadyForWrongEpochNumber as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StakerNotPermitted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TryingToWithdrawMoreThanStaked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ValidatorIsNotInNextEpoch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for StakingErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActiveValidatorsCannotLeave(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotKickBelowCurrentValidatorThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRejoinUntilNextEpochBecauseKicked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveFunctionThatDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveImmutableFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceFunctionThatDoesNotExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::CannotReplaceFunctionsFromFacetWithZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceImmutableFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReuseCommsKeys(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotStakeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotVoteTwice(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotWithdrawZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CouldNotMapNodeAddressToStakerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MustBeInActiveOrUnlockedOrPausedState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MustBeInActiveOrUnlockedState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MustBeInNextValidatorSetLockedState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MustBeInReadyForNextEpochState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MustBeValidatorInNextEpochToKick(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotContractOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughTimeElapsedForTimeoutSinceLastEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughTimeElapsedSinceLastEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughValidatorsInNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughValidatorsReadyForNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignaledReadyForWrongEpochNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerNotPermitted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TryingToWithdrawMoreThanStaked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorIsNotInNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for StakingErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ActiveValidatorsCannotLeave> for StakingErrors {
        fn from(value: ActiveValidatorsCannotLeave) -> Self {
            Self::ActiveValidatorsCannotLeave(value)
        }
    }
    impl ::core::convert::From<CallerNotOwner> for StakingErrors {
        fn from(value: CallerNotOwner) -> Self {
            Self::CallerNotOwner(value)
        }
    }
    impl ::core::convert::From<CannotAddFunctionToDiamondThatAlreadyExists>
    for StakingErrors {
        fn from(value: CannotAddFunctionToDiamondThatAlreadyExists) -> Self {
            Self::CannotAddFunctionToDiamondThatAlreadyExists(value)
        }
    }
    impl ::core::convert::From<CannotAddSelectorsToZeroAddress> for StakingErrors {
        fn from(value: CannotAddSelectorsToZeroAddress) -> Self {
            Self::CannotAddSelectorsToZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotKickBelowCurrentValidatorThreshold>
    for StakingErrors {
        fn from(value: CannotKickBelowCurrentValidatorThreshold) -> Self {
            Self::CannotKickBelowCurrentValidatorThreshold(value)
        }
    }
    impl ::core::convert::From<CannotRejoinUntilNextEpochBecauseKicked>
    for StakingErrors {
        fn from(value: CannotRejoinUntilNextEpochBecauseKicked) -> Self {
            Self::CannotRejoinUntilNextEpochBecauseKicked(value)
        }
    }
    impl ::core::convert::From<CannotRemoveFunctionThatDoesNotExist> for StakingErrors {
        fn from(value: CannotRemoveFunctionThatDoesNotExist) -> Self {
            Self::CannotRemoveFunctionThatDoesNotExist(value)
        }
    }
    impl ::core::convert::From<CannotRemoveImmutableFunction> for StakingErrors {
        fn from(value: CannotRemoveImmutableFunction) -> Self {
            Self::CannotRemoveImmutableFunction(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionThatDoesNotExists>
    for StakingErrors {
        fn from(value: CannotReplaceFunctionThatDoesNotExists) -> Self {
            Self::CannotReplaceFunctionThatDoesNotExists(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet>
    for StakingErrors {
        fn from(
            value: CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ) -> Self {
            Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionsFromFacetWithZeroAddress>
    for StakingErrors {
        fn from(value: CannotReplaceFunctionsFromFacetWithZeroAddress) -> Self {
            Self::CannotReplaceFunctionsFromFacetWithZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotReplaceImmutableFunction> for StakingErrors {
        fn from(value: CannotReplaceImmutableFunction) -> Self {
            Self::CannotReplaceImmutableFunction(value)
        }
    }
    impl ::core::convert::From<CannotReuseCommsKeys> for StakingErrors {
        fn from(value: CannotReuseCommsKeys) -> Self {
            Self::CannotReuseCommsKeys(value)
        }
    }
    impl ::core::convert::From<CannotStakeZero> for StakingErrors {
        fn from(value: CannotStakeZero) -> Self {
            Self::CannotStakeZero(value)
        }
    }
    impl ::core::convert::From<CannotVoteTwice> for StakingErrors {
        fn from(value: CannotVoteTwice) -> Self {
            Self::CannotVoteTwice(value)
        }
    }
    impl ::core::convert::From<CannotWithdrawZero> for StakingErrors {
        fn from(value: CannotWithdrawZero) -> Self {
            Self::CannotWithdrawZero(value)
        }
    }
    impl ::core::convert::From<CouldNotMapNodeAddressToStakerAddress> for StakingErrors {
        fn from(value: CouldNotMapNodeAddressToStakerAddress) -> Self {
            Self::CouldNotMapNodeAddressToStakerAddress(value)
        }
    }
    impl ::core::convert::From<IncorrectFacetCutAction> for StakingErrors {
        fn from(value: IncorrectFacetCutAction) -> Self {
            Self::IncorrectFacetCutAction(value)
        }
    }
    impl ::core::convert::From<InitializationFunctionReverted> for StakingErrors {
        fn from(value: InitializationFunctionReverted) -> Self {
            Self::InitializationFunctionReverted(value)
        }
    }
    impl ::core::convert::From<MustBeInActiveOrUnlockedOrPausedState> for StakingErrors {
        fn from(value: MustBeInActiveOrUnlockedOrPausedState) -> Self {
            Self::MustBeInActiveOrUnlockedOrPausedState(value)
        }
    }
    impl ::core::convert::From<MustBeInActiveOrUnlockedState> for StakingErrors {
        fn from(value: MustBeInActiveOrUnlockedState) -> Self {
            Self::MustBeInActiveOrUnlockedState(value)
        }
    }
    impl ::core::convert::From<
        MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState,
    > for StakingErrors {
        fn from(
            value: MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState,
        ) -> Self {
            Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochOrRestoreState(value)
        }
    }
    impl ::core::convert::From<MustBeInNextValidatorSetLockedOrReadyForNextEpochState>
    for StakingErrors {
        fn from(value: MustBeInNextValidatorSetLockedOrReadyForNextEpochState) -> Self {
            Self::MustBeInNextValidatorSetLockedOrReadyForNextEpochState(value)
        }
    }
    impl ::core::convert::From<MustBeInNextValidatorSetLockedState> for StakingErrors {
        fn from(value: MustBeInNextValidatorSetLockedState) -> Self {
            Self::MustBeInNextValidatorSetLockedState(value)
        }
    }
    impl ::core::convert::From<MustBeInReadyForNextEpochState> for StakingErrors {
        fn from(value: MustBeInReadyForNextEpochState) -> Self {
            Self::MustBeInReadyForNextEpochState(value)
        }
    }
    impl ::core::convert::From<MustBeValidatorInNextEpochToKick> for StakingErrors {
        fn from(value: MustBeValidatorInNextEpochToKick) -> Self {
            Self::MustBeValidatorInNextEpochToKick(value)
        }
    }
    impl ::core::convert::From<NoBytecodeAtAddress> for StakingErrors {
        fn from(value: NoBytecodeAtAddress) -> Self {
            Self::NoBytecodeAtAddress(value)
        }
    }
    impl ::core::convert::From<NoSelectorsProvidedForFacetForCut> for StakingErrors {
        fn from(value: NoSelectorsProvidedForFacetForCut) -> Self {
            Self::NoSelectorsProvidedForFacetForCut(value)
        }
    }
    impl ::core::convert::From<NotContractOwner> for StakingErrors {
        fn from(value: NotContractOwner) -> Self {
            Self::NotContractOwner(value)
        }
    }
    impl ::core::convert::From<NotEnoughTimeElapsedForTimeoutSinceLastEpoch>
    for StakingErrors {
        fn from(value: NotEnoughTimeElapsedForTimeoutSinceLastEpoch) -> Self {
            Self::NotEnoughTimeElapsedForTimeoutSinceLastEpoch(value)
        }
    }
    impl ::core::convert::From<NotEnoughTimeElapsedSinceLastEpoch> for StakingErrors {
        fn from(value: NotEnoughTimeElapsedSinceLastEpoch) -> Self {
            Self::NotEnoughTimeElapsedSinceLastEpoch(value)
        }
    }
    impl ::core::convert::From<NotEnoughValidatorsInNextEpoch> for StakingErrors {
        fn from(value: NotEnoughValidatorsInNextEpoch) -> Self {
            Self::NotEnoughValidatorsInNextEpoch(value)
        }
    }
    impl ::core::convert::From<NotEnoughValidatorsReadyForNextEpoch> for StakingErrors {
        fn from(value: NotEnoughValidatorsReadyForNextEpoch) -> Self {
            Self::NotEnoughValidatorsReadyForNextEpoch(value)
        }
    }
    impl ::core::convert::From<RemoveFacetAddressMustBeZeroAddress> for StakingErrors {
        fn from(value: RemoveFacetAddressMustBeZeroAddress) -> Self {
            Self::RemoveFacetAddressMustBeZeroAddress(value)
        }
    }
    impl ::core::convert::From<SignaledReadyForWrongEpochNumber> for StakingErrors {
        fn from(value: SignaledReadyForWrongEpochNumber) -> Self {
            Self::SignaledReadyForWrongEpochNumber(value)
        }
    }
    impl ::core::convert::From<StakerNotPermitted> for StakingErrors {
        fn from(value: StakerNotPermitted) -> Self {
            Self::StakerNotPermitted(value)
        }
    }
    impl ::core::convert::From<TryingToWithdrawMoreThanStaked> for StakingErrors {
        fn from(value: TryingToWithdrawMoreThanStaked) -> Self {
            Self::TryingToWithdrawMoreThanStaked(value)
        }
    }
    impl ::core::convert::From<ValidatorIsNotInNextEpoch> for StakingErrors {
        fn from(value: ValidatorIsNotInNextEpoch) -> Self {
            Self::ValidatorIsNotInNextEpoch(value)
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
        name = "ComplaintConfigSet",
        abi = "ComplaintConfigSet(uint256,(uint256,uint256,uint256))"
    )]
    pub struct ComplaintConfigSetFilter {
        pub reason: ::ethers::core::types::U256,
        pub config: ComplaintConfig,
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
        name = "ConfigSet",
        abi = "ConfigSet(uint256,uint256[],uint256,uint256,uint256,uint256,uint256,uint256,bool)"
    )]
    pub struct ConfigSetFilter {
        pub new_token_reward_per_token_per_epoch: ::ethers::core::types::U256,
        pub new_key_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub new_minimum_validator_count: ::ethers::core::types::U256,
        pub new_max_concurrent_requests: ::ethers::core::types::U256,
        pub new_max_triple_count: ::ethers::core::types::U256,
        pub new_min_triple_count: ::ethers::core::types::U256,
        pub new_peer_checking_interval_secs: ::ethers::core::types::U256,
        pub new_max_triple_concurrency: ::ethers::core::types::U256,
        pub new_rpc_healthcheck_enabled: bool,
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
        name = "DiamondCut",
        abi = "DiamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutFilter {
        pub diamond_cut: ::std::vec::Vec<FacetCut>,
        pub init: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "EpochEndTimeSet", abi = "EpochEndTimeSet(uint256)")]
    pub struct EpochEndTimeSetFilter {
        pub new_epoch_end_time: ::ethers::core::types::U256,
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
    #[ethevent(name = "EpochLengthSet", abi = "EpochLengthSet(uint256)")]
    pub struct EpochLengthSetFilter {
        pub new_epoch_length: ::ethers::core::types::U256,
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
    #[ethevent(name = "EpochTimeoutSet", abi = "EpochTimeoutSet(uint256)")]
    pub struct EpochTimeoutSetFilter {
        pub new_epoch_timeout: ::ethers::core::types::U256,
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
        name = "KickPenaltyPercentSet",
        abi = "KickPenaltyPercentSet(uint256,uint256)"
    )]
    pub struct KickPenaltyPercentSetFilter {
        pub reason: ::ethers::core::types::U256,
        pub new_kick_penalty_percent: ::ethers::core::types::U256,
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
    #[ethevent(name = "ReadyForNextEpoch", abi = "ReadyForNextEpoch(address,uint256)")]
    pub struct ReadyForNextEpochFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        pub epoch_number: ::ethers::core::types::U256,
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
    #[ethevent(name = "Recovered", abi = "Recovered(address,uint256)")]
    pub struct RecoveredFilter {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "RequestToJoin", abi = "RequestToJoin(address)")]
    pub struct RequestToJoinFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
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
    #[ethevent(name = "RequestToLeave", abi = "RequestToLeave(address)")]
    pub struct RequestToLeaveFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
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
        name = "ResolverContractAddressSet",
        abi = "ResolverContractAddressSet(address)"
    )]
    pub struct ResolverContractAddressSetFilter {
        pub new_resolver_contract_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "RewardsDurationUpdated", abi = "RewardsDurationUpdated(uint256)")]
    pub struct RewardsDurationUpdatedFilter {
        pub new_duration: ::ethers::core::types::U256,
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
    #[ethevent(name = "StakingTokenSet", abi = "StakingTokenSet(address)")]
    pub struct StakingTokenSetFilter {
        pub new_staking_token_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "StateChanged", abi = "StateChanged(uint8)")]
    pub struct StateChangedFilter {
        pub new_state: u8,
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
        name = "ValidatorKickedFromNextEpoch",
        abi = "ValidatorKickedFromNextEpoch(address,uint256)"
    )]
    pub struct ValidatorKickedFromNextEpochFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        pub amount_burned: ::ethers::core::types::U256,
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
        name = "ValidatorRejoinedNextEpoch",
        abi = "ValidatorRejoinedNextEpoch(address)"
    )]
    pub struct ValidatorRejoinedNextEpochFilter {
        pub staker: ::ethers::core::types::Address,
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
        name = "VersionRequirementsUpdated",
        abi = "VersionRequirementsUpdated(uint256,(uint256,uint256,uint256))"
    )]
    pub struct VersionRequirementsUpdatedFilter {
        pub index: ::ethers::core::types::U256,
        pub version: Version,
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
        name = "VotedToKickValidatorInNextEpoch",
        abi = "VotedToKickValidatorInNextEpoch(address,address,uint256,bytes)"
    )]
    pub struct VotedToKickValidatorInNextEpochFilter {
        #[ethevent(indexed)]
        pub reporter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub validator_staker_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reason: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum StakingEvents {
        ComplaintConfigSetFilter(ComplaintConfigSetFilter),
        ConfigSetFilter(ConfigSetFilter),
        DiamondCutFilter(DiamondCutFilter),
        EpochEndTimeSetFilter(EpochEndTimeSetFilter),
        EpochLengthSetFilter(EpochLengthSetFilter),
        EpochTimeoutSetFilter(EpochTimeoutSetFilter),
        KickPenaltyPercentSetFilter(KickPenaltyPercentSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ReadyForNextEpochFilter(ReadyForNextEpochFilter),
        RecoveredFilter(RecoveredFilter),
        RequestToJoinFilter(RequestToJoinFilter),
        RequestToLeaveFilter(RequestToLeaveFilter),
        ResolverContractAddressSetFilter(ResolverContractAddressSetFilter),
        RewardsDurationUpdatedFilter(RewardsDurationUpdatedFilter),
        StakingTokenSetFilter(StakingTokenSetFilter),
        StateChangedFilter(StateChangedFilter),
        ValidatorKickedFromNextEpochFilter(ValidatorKickedFromNextEpochFilter),
        ValidatorRejoinedNextEpochFilter(ValidatorRejoinedNextEpochFilter),
        VersionRequirementsUpdatedFilter(VersionRequirementsUpdatedFilter),
        VotedToKickValidatorInNextEpochFilter(VotedToKickValidatorInNextEpochFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ComplaintConfigSetFilter::decode_log(log) {
                return Ok(StakingEvents::ComplaintConfigSetFilter(decoded));
            }
            if let Ok(decoded) = ConfigSetFilter::decode_log(log) {
                return Ok(StakingEvents::ConfigSetFilter(decoded));
            }
            if let Ok(decoded) = DiamondCutFilter::decode_log(log) {
                return Ok(StakingEvents::DiamondCutFilter(decoded));
            }
            if let Ok(decoded) = EpochEndTimeSetFilter::decode_log(log) {
                return Ok(StakingEvents::EpochEndTimeSetFilter(decoded));
            }
            if let Ok(decoded) = EpochLengthSetFilter::decode_log(log) {
                return Ok(StakingEvents::EpochLengthSetFilter(decoded));
            }
            if let Ok(decoded) = EpochTimeoutSetFilter::decode_log(log) {
                return Ok(StakingEvents::EpochTimeoutSetFilter(decoded));
            }
            if let Ok(decoded) = KickPenaltyPercentSetFilter::decode_log(log) {
                return Ok(StakingEvents::KickPenaltyPercentSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(StakingEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ReadyForNextEpochFilter::decode_log(log) {
                return Ok(StakingEvents::ReadyForNextEpochFilter(decoded));
            }
            if let Ok(decoded) = RecoveredFilter::decode_log(log) {
                return Ok(StakingEvents::RecoveredFilter(decoded));
            }
            if let Ok(decoded) = RequestToJoinFilter::decode_log(log) {
                return Ok(StakingEvents::RequestToJoinFilter(decoded));
            }
            if let Ok(decoded) = RequestToLeaveFilter::decode_log(log) {
                return Ok(StakingEvents::RequestToLeaveFilter(decoded));
            }
            if let Ok(decoded) = ResolverContractAddressSetFilter::decode_log(log) {
                return Ok(StakingEvents::ResolverContractAddressSetFilter(decoded));
            }
            if let Ok(decoded) = RewardsDurationUpdatedFilter::decode_log(log) {
                return Ok(StakingEvents::RewardsDurationUpdatedFilter(decoded));
            }
            if let Ok(decoded) = StakingTokenSetFilter::decode_log(log) {
                return Ok(StakingEvents::StakingTokenSetFilter(decoded));
            }
            if let Ok(decoded) = StateChangedFilter::decode_log(log) {
                return Ok(StakingEvents::StateChangedFilter(decoded));
            }
            if let Ok(decoded) = ValidatorKickedFromNextEpochFilter::decode_log(log) {
                return Ok(StakingEvents::ValidatorKickedFromNextEpochFilter(decoded));
            }
            if let Ok(decoded) = ValidatorRejoinedNextEpochFilter::decode_log(log) {
                return Ok(StakingEvents::ValidatorRejoinedNextEpochFilter(decoded));
            }
            if let Ok(decoded) = VersionRequirementsUpdatedFilter::decode_log(log) {
                return Ok(StakingEvents::VersionRequirementsUpdatedFilter(decoded));
            }
            if let Ok(decoded) = VotedToKickValidatorInNextEpochFilter::decode_log(log) {
                return Ok(StakingEvents::VotedToKickValidatorInNextEpochFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakingEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComplaintConfigSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DiamondCutFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EpochEndTimeSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EpochLengthSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EpochTimeoutSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::KickPenaltyPercentSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReadyForNextEpochFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecoveredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestToJoinFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestToLeaveFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResolverContractAddressSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardsDurationUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingTokenSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StateChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorKickedFromNextEpochFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorRejoinedNextEpochFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VersionRequirementsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VotedToKickValidatorInNextEpochFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ComplaintConfigSetFilter> for StakingEvents {
        fn from(value: ComplaintConfigSetFilter) -> Self {
            Self::ComplaintConfigSetFilter(value)
        }
    }
    impl ::core::convert::From<ConfigSetFilter> for StakingEvents {
        fn from(value: ConfigSetFilter) -> Self {
            Self::ConfigSetFilter(value)
        }
    }
    impl ::core::convert::From<DiamondCutFilter> for StakingEvents {
        fn from(value: DiamondCutFilter) -> Self {
            Self::DiamondCutFilter(value)
        }
    }
    impl ::core::convert::From<EpochEndTimeSetFilter> for StakingEvents {
        fn from(value: EpochEndTimeSetFilter) -> Self {
            Self::EpochEndTimeSetFilter(value)
        }
    }
    impl ::core::convert::From<EpochLengthSetFilter> for StakingEvents {
        fn from(value: EpochLengthSetFilter) -> Self {
            Self::EpochLengthSetFilter(value)
        }
    }
    impl ::core::convert::From<EpochTimeoutSetFilter> for StakingEvents {
        fn from(value: EpochTimeoutSetFilter) -> Self {
            Self::EpochTimeoutSetFilter(value)
        }
    }
    impl ::core::convert::From<KickPenaltyPercentSetFilter> for StakingEvents {
        fn from(value: KickPenaltyPercentSetFilter) -> Self {
            Self::KickPenaltyPercentSetFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for StakingEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ReadyForNextEpochFilter> for StakingEvents {
        fn from(value: ReadyForNextEpochFilter) -> Self {
            Self::ReadyForNextEpochFilter(value)
        }
    }
    impl ::core::convert::From<RecoveredFilter> for StakingEvents {
        fn from(value: RecoveredFilter) -> Self {
            Self::RecoveredFilter(value)
        }
    }
    impl ::core::convert::From<RequestToJoinFilter> for StakingEvents {
        fn from(value: RequestToJoinFilter) -> Self {
            Self::RequestToJoinFilter(value)
        }
    }
    impl ::core::convert::From<RequestToLeaveFilter> for StakingEvents {
        fn from(value: RequestToLeaveFilter) -> Self {
            Self::RequestToLeaveFilter(value)
        }
    }
    impl ::core::convert::From<ResolverContractAddressSetFilter> for StakingEvents {
        fn from(value: ResolverContractAddressSetFilter) -> Self {
            Self::ResolverContractAddressSetFilter(value)
        }
    }
    impl ::core::convert::From<RewardsDurationUpdatedFilter> for StakingEvents {
        fn from(value: RewardsDurationUpdatedFilter) -> Self {
            Self::RewardsDurationUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StakingTokenSetFilter> for StakingEvents {
        fn from(value: StakingTokenSetFilter) -> Self {
            Self::StakingTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<StateChangedFilter> for StakingEvents {
        fn from(value: StateChangedFilter) -> Self {
            Self::StateChangedFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorKickedFromNextEpochFilter> for StakingEvents {
        fn from(value: ValidatorKickedFromNextEpochFilter) -> Self {
            Self::ValidatorKickedFromNextEpochFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorRejoinedNextEpochFilter> for StakingEvents {
        fn from(value: ValidatorRejoinedNextEpochFilter) -> Self {
            Self::ValidatorRejoinedNextEpochFilter(value)
        }
    }
    impl ::core::convert::From<VersionRequirementsUpdatedFilter> for StakingEvents {
        fn from(value: VersionRequirementsUpdatedFilter) -> Self {
            Self::VersionRequirementsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<VotedToKickValidatorInNextEpochFilter> for StakingEvents {
        fn from(value: VotedToKickValidatorInNextEpochFilter) -> Self {
            Self::VotedToKickValidatorInNextEpochFilter(value)
        }
    }
    ///Container type for all input parameters for the `adminKickValidatorInNextEpoch` function with signature `adminKickValidatorInNextEpoch(address)` and selector `0x7aa086e7`
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
        name = "adminKickValidatorInNextEpoch",
        abi = "adminKickValidatorInNextEpoch(address)"
    )]
    pub struct AdminKickValidatorInNextEpochCall {
        pub validator_staker_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `adminRejoinValidator` function with signature `adminRejoinValidator(address)` and selector `0x7392c76b`
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
    #[ethcall(name = "adminRejoinValidator", abi = "adminRejoinValidator(address)")]
    pub struct AdminRejoinValidatorCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `adminResetEpoch` function with signature `adminResetEpoch()` and selector `0x3c88123d`
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
    #[ethcall(name = "adminResetEpoch", abi = "adminResetEpoch()")]
    pub struct AdminResetEpochCall;
    ///Container type for all input parameters for the `adminSlashValidator` function with signature `adminSlashValidator(address,uint256)` and selector `0x8b80d833`
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
        name = "adminSlashValidator",
        abi = "adminSlashValidator(address,uint256)"
    )]
    pub struct AdminSlashValidatorCall {
        pub validator_staker_address: ::ethers::core::types::Address,
        pub amount_to_penalize: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `advanceEpoch` function with signature `advanceEpoch()` and selector `0x3cf80e6c`
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
    #[ethcall(name = "advanceEpoch", abi = "advanceEpoch()")]
    pub struct AdvanceEpochCall;
    ///Container type for all input parameters for the `checkVersion` function with signature `checkVersion((uint256,uint256,uint256))` and selector `0x7720ff0f`
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
    #[ethcall(name = "checkVersion", abi = "checkVersion((uint256,uint256,uint256))")]
    pub struct CheckVersionCall {
        pub version: Version,
    }
    ///Container type for all input parameters for the `complaintConfig` function with signature `complaintConfig(uint256)` and selector `0x02f6da56`
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
    #[ethcall(name = "complaintConfig", abi = "complaintConfig(uint256)")]
    pub struct ComplaintConfigCall {
        pub reason: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `config` function with signature `config()` and selector `0x79502c55`
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
    #[ethcall(name = "config", abi = "config()")]
    pub struct ConfigCall;
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
    ///Container type for all input parameters for the `countOfCurrentValidatorsReadyForNextEpoch` function with signature `countOfCurrentValidatorsReadyForNextEpoch()` and selector `0xe8684ed1`
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
        name = "countOfCurrentValidatorsReadyForNextEpoch",
        abi = "countOfCurrentValidatorsReadyForNextEpoch()"
    )]
    pub struct CountOfCurrentValidatorsReadyForNextEpochCall;
    ///Container type for all input parameters for the `countOfNextValidatorsReadyForNextEpoch` function with signature `countOfNextValidatorsReadyForNextEpoch()` and selector `0x89965883`
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
        name = "countOfNextValidatorsReadyForNextEpoch",
        abi = "countOfNextValidatorsReadyForNextEpoch()"
    )]
    pub struct CountOfNextValidatorsReadyForNextEpochCall;
    ///Container type for all input parameters for the `currentValidatorCountForConsensus` function with signature `currentValidatorCountForConsensus()` and selector `0x43cb0a0e`
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
        name = "currentValidatorCountForConsensus",
        abi = "currentValidatorCountForConsensus()"
    )]
    pub struct CurrentValidatorCountForConsensusCall;
    ///Container type for all input parameters for the `diamondCut` function with signature `diamondCut((address,uint8,bytes4[])[],address,bytes)` and selector `0x1f931c1c`
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
        name = "diamondCut",
        abi = "diamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutCall {
        pub diamond_cut: ::std::vec::Vec<FacetCut>,
        pub init: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `epoch` function with signature `epoch()` and selector `0x900cf0cf`
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
    #[ethcall(name = "epoch", abi = "epoch()")]
    pub struct EpochCall;
    ///Container type for all input parameters for the `exit` function with signature `exit()` and selector `0xe9fad8ee`
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
    #[ethcall(name = "exit", abi = "exit()")]
    pub struct ExitCall;
    ///Container type for all input parameters for the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    #[ethcall(name = "facetAddress", abi = "facetAddress(bytes4)")]
    pub struct FacetAddressCall {
        pub function_selector: [u8; 4],
    }
    ///Container type for all input parameters for the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    #[ethcall(name = "facetAddresses", abi = "facetAddresses()")]
    pub struct FacetAddressesCall;
    ///Container type for all input parameters for the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    #[ethcall(name = "facetFunctionSelectors", abi = "facetFunctionSelectors(address)")]
    pub struct FacetFunctionSelectorsCall {
        pub facet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    #[ethcall(name = "facets", abi = "facets()")]
    pub struct FacetsCall;
    ///Container type for all input parameters for the `getKeyTypes` function with signature `getKeyTypes()` and selector `0xf1b877a8`
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
    #[ethcall(name = "getKeyTypes", abi = "getKeyTypes()")]
    pub struct GetKeyTypesCall;
    ///Container type for all input parameters for the `getKickedValidators` function with signature `getKickedValidators()` and selector `0x5995a4c4`
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
    #[ethcall(name = "getKickedValidators", abi = "getKickedValidators()")]
    pub struct GetKickedValidatorsCall;
    ///Container type for all input parameters for the `getMaxVersion` function with signature `getMaxVersion()` and selector `0x2c20d23a`
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
    #[ethcall(name = "getMaxVersion", abi = "getMaxVersion()")]
    pub struct GetMaxVersionCall;
    ///Container type for all input parameters for the `getMaxVersionString` function with signature `getMaxVersionString()` and selector `0xdbd68aa7`
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
    #[ethcall(name = "getMaxVersionString", abi = "getMaxVersionString()")]
    pub struct GetMaxVersionStringCall;
    ///Container type for all input parameters for the `getMinVersion` function with signature `getMinVersion()` and selector `0xd1e3f3d7`
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
    #[ethcall(name = "getMinVersion", abi = "getMinVersion()")]
    pub struct GetMinVersionCall;
    ///Container type for all input parameters for the `getMinVersionString` function with signature `getMinVersionString()` and selector `0x517d2324`
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
    #[ethcall(name = "getMinVersionString", abi = "getMinVersionString()")]
    pub struct GetMinVersionStringCall;
    ///Container type for all input parameters for the `getNodeStakerAddressMappings` function with signature `getNodeStakerAddressMappings(address[])` and selector `0x90fba112`
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
        name = "getNodeStakerAddressMappings",
        abi = "getNodeStakerAddressMappings(address[])"
    )]
    pub struct GetNodeStakerAddressMappingsCall {
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getReward` function with signature `getReward()` and selector `0x3d18b912`
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
    #[ethcall(name = "getReward", abi = "getReward()")]
    pub struct GetRewardCall;
    ///Container type for all input parameters for the `getStakingBalancesAddress` function with signature `getStakingBalancesAddress()` and selector `0x5b677eac`
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
    #[ethcall(name = "getStakingBalancesAddress", abi = "getStakingBalancesAddress()")]
    pub struct GetStakingBalancesAddressCall;
    ///Container type for all input parameters for the `getTokenAddress` function with signature `getTokenAddress()` and selector `0x10fe9ae8`
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
    #[ethcall(name = "getTokenAddress", abi = "getTokenAddress()")]
    pub struct GetTokenAddressCall;
    ///Container type for all input parameters for the `getValidatorsInCurrentEpoch` function with signature `getValidatorsInCurrentEpoch()` and selector `0x857b7663`
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
        name = "getValidatorsInCurrentEpoch",
        abi = "getValidatorsInCurrentEpoch()"
    )]
    pub struct GetValidatorsInCurrentEpochCall;
    ///Container type for all input parameters for the `getValidatorsInCurrentEpochLength` function with signature `getValidatorsInCurrentEpochLength()` and selector `0xd4818fca`
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
        name = "getValidatorsInCurrentEpochLength",
        abi = "getValidatorsInCurrentEpochLength()"
    )]
    pub struct GetValidatorsInCurrentEpochLengthCall;
    ///Container type for all input parameters for the `getValidatorsInNextEpoch` function with signature `getValidatorsInNextEpoch()` and selector `0xc35d4d09`
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
    #[ethcall(name = "getValidatorsInNextEpoch", abi = "getValidatorsInNextEpoch()")]
    pub struct GetValidatorsInNextEpochCall;
    ///Container type for all input parameters for the `getValidatorsStructs` function with signature `getValidatorsStructs(address[])` and selector `0x533d463e`
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
    #[ethcall(name = "getValidatorsStructs", abi = "getValidatorsStructs(address[])")]
    pub struct GetValidatorsStructsCall {
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getValidatorsStructsInCurrentEpoch` function with signature `getValidatorsStructsInCurrentEpoch()` and selector `0xe7c08720`
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
        name = "getValidatorsStructsInCurrentEpoch",
        abi = "getValidatorsStructsInCurrentEpoch()"
    )]
    pub struct GetValidatorsStructsInCurrentEpochCall;
    ///Container type for all input parameters for the `getValidatorsStructsInNextEpoch` function with signature `getValidatorsStructsInNextEpoch()` and selector `0x61dee8a3`
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
        name = "getValidatorsStructsInNextEpoch",
        abi = "getValidatorsStructsInNextEpoch()"
    )]
    pub struct GetValidatorsStructsInNextEpochCall;
    ///Container type for all input parameters for the `getVotingStatusToKickValidator` function with signature `getVotingStatusToKickValidator(uint256,address,address)` and selector `0x70fe276a`
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
        name = "getVotingStatusToKickValidator",
        abi = "getVotingStatusToKickValidator(uint256,address,address)"
    )]
    pub struct GetVotingStatusToKickValidatorCall {
        pub epoch_number: ::ethers::core::types::U256,
        pub validator_staker_address: ::ethers::core::types::Address,
        pub voter_staker_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isActiveValidator` function with signature `isActiveValidator(address)` and selector `0x40550a1c`
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
    #[ethcall(name = "isActiveValidator", abi = "isActiveValidator(address)")]
    pub struct IsActiveValidatorCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isActiveValidatorByNodeAddress` function with signature `isActiveValidatorByNodeAddress(address)` and selector `0xa25e49a4`
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
        name = "isActiveValidatorByNodeAddress",
        abi = "isActiveValidatorByNodeAddress(address)"
    )]
    pub struct IsActiveValidatorByNodeAddressCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isReadyForNextEpoch` function with signature `isReadyForNextEpoch()` and selector `0xf1887fec`
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
    #[ethcall(name = "isReadyForNextEpoch", abi = "isReadyForNextEpoch()")]
    pub struct IsReadyForNextEpochCall;
    ///Container type for all input parameters for the `kickPenaltyPercentByReason` function with signature `kickPenaltyPercentByReason(uint256)` and selector `0x3e685266`
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
        name = "kickPenaltyPercentByReason",
        abi = "kickPenaltyPercentByReason(uint256)"
    )]
    pub struct KickPenaltyPercentByReasonCall {
        pub reason: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `kickValidatorInNextEpoch` function with signature `kickValidatorInNextEpoch(address,uint256,bytes)` and selector `0x865419e9`
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
        name = "kickValidatorInNextEpoch",
        abi = "kickValidatorInNextEpoch(address,uint256,bytes)"
    )]
    pub struct KickValidatorInNextEpochCall {
        pub validator_staker_address: ::ethers::core::types::Address,
        pub reason: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `lockValidatorsForNextEpoch` function with signature `lockValidatorsForNextEpoch()` and selector `0x16930f4d`
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
    #[ethcall(name = "lockValidatorsForNextEpoch", abi = "lockValidatorsForNextEpoch()")]
    pub struct LockValidatorsForNextEpochCall;
    ///Container type for all input parameters for the `nextValidatorCountForConsensus` function with signature `nextValidatorCountForConsensus()` and selector `0x0297d4db`
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
        name = "nextValidatorCountForConsensus",
        abi = "nextValidatorCountForConsensus()"
    )]
    pub struct NextValidatorCountForConsensusCall;
    ///Container type for all input parameters for the `nodeAddressToStakerAddress` function with signature `nodeAddressToStakerAddress(address)` and selector `0x5081f66f`
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
        name = "nodeAddressToStakerAddress",
        abi = "nodeAddressToStakerAddress(address)"
    )]
    pub struct NodeAddressToStakerAddressCall {
        pub node_address: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `readyForNextEpoch` function with signature `readyForNextEpoch(address)` and selector `0x519877eb`
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
    #[ethcall(name = "readyForNextEpoch", abi = "readyForNextEpoch(address)")]
    pub struct ReadyForNextEpochCall {
        pub staker_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `requestToJoin` function with signature `requestToJoin(uint32,uint128,uint32,address,uint256,uint256)` and selector `0x3528db88`
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
        name = "requestToJoin",
        abi = "requestToJoin(uint32,uint128,uint32,address,uint256,uint256)"
    )]
    pub struct RequestToJoinCall {
        pub ip: u32,
        pub ipv_6: u128,
        pub port: u32,
        pub node_address: ::ethers::core::types::Address,
        pub sender_pub_key: ::ethers::core::types::U256,
        pub receiver_pub_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requestToLeave` function with signature `requestToLeave()` and selector `0xac2f8afe`
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
    #[ethcall(name = "requestToLeave", abi = "requestToLeave()")]
    pub struct RequestToLeaveCall;
    ///Container type for all input parameters for the `requestToLeaveAsNode` function with signature `requestToLeaveAsNode()` and selector `0x8a3d070e`
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
    #[ethcall(name = "requestToLeaveAsNode", abi = "requestToLeaveAsNode()")]
    pub struct RequestToLeaveAsNodeCall;
    ///Container type for all input parameters for the `setComplaintConfig` function with signature `setComplaintConfig(uint256,(uint256,uint256,uint256))` and selector `0xdc509d50`
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
        name = "setComplaintConfig",
        abi = "setComplaintConfig(uint256,(uint256,uint256,uint256))"
    )]
    pub struct SetComplaintConfigCall {
        pub reason: ::ethers::core::types::U256,
        pub config: ComplaintConfig,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig((uint256,uint256,uint256,uint256[],uint256,uint256,uint256,uint256,uint256,uint256,bool))` and selector `0x87389dd7`
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
        name = "setConfig",
        abi = "setConfig((uint256,uint256,uint256,uint256[],uint256,uint256,uint256,uint256,uint256,uint256,bool))"
    )]
    pub struct SetConfigCall {
        pub new_config: Config,
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
        pub new_resolver_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEpochEndTime` function with signature `setEpochEndTime(uint256)` and selector `0x4a6e51f5`
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
    #[ethcall(name = "setEpochEndTime", abi = "setEpochEndTime(uint256)")]
    pub struct SetEpochEndTimeCall {
        pub new_epoch_end_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setEpochLength` function with signature `setEpochLength(uint256)` and selector `0x54eea796`
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
    #[ethcall(name = "setEpochLength", abi = "setEpochLength(uint256)")]
    pub struct SetEpochLengthCall {
        pub new_epoch_length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setEpochState` function with signature `setEpochState(uint8)` and selector `0x3f819713`
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
    #[ethcall(name = "setEpochState", abi = "setEpochState(uint8)")]
    pub struct SetEpochStateCall {
        pub new_state: u8,
    }
    ///Container type for all input parameters for the `setEpochTimeout` function with signature `setEpochTimeout(uint256)` and selector `0x1fab87c4`
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
    #[ethcall(name = "setEpochTimeout", abi = "setEpochTimeout(uint256)")]
    pub struct SetEpochTimeoutCall {
        pub new_epoch_timeout: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setIpPortNodeAddressAndCommunicationPubKeys` function with signature `setIpPortNodeAddressAndCommunicationPubKeys(uint32,uint128,uint32,address,uint256,uint256)` and selector `0x4f8f0102`
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
        name = "setIpPortNodeAddressAndCommunicationPubKeys",
        abi = "setIpPortNodeAddressAndCommunicationPubKeys(uint32,uint128,uint32,address,uint256,uint256)"
    )]
    pub struct SetIpPortNodeAddressAndCommunicationPubKeysCall {
        pub ip: u32,
        pub ipv_6: u128,
        pub port: u32,
        pub node_address: ::ethers::core::types::Address,
        pub sender_pub_key: ::ethers::core::types::U256,
        pub receiver_pub_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setKickPenaltyPercent` function with signature `setKickPenaltyPercent(uint256,uint256)` and selector `0x09c7c7d0`
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
        name = "setKickPenaltyPercent",
        abi = "setKickPenaltyPercent(uint256,uint256)"
    )]
    pub struct SetKickPenaltyPercentCall {
        pub reason: ::ethers::core::types::U256,
        pub new_kick_penalty_percent: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMaxVersion` function with signature `setMaxVersion((uint256,uint256,uint256))` and selector `0xec159d7a`
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
    #[ethcall(name = "setMaxVersion", abi = "setMaxVersion((uint256,uint256,uint256))")]
    pub struct SetMaxVersionCall {
        pub version: Version,
    }
    ///Container type for all input parameters for the `setMinVersion` function with signature `setMinVersion((uint256,uint256,uint256))` and selector `0xd900fcbb`
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
    #[ethcall(name = "setMinVersion", abi = "setMinVersion((uint256,uint256,uint256))")]
    pub struct SetMinVersionCall {
        pub version: Version,
    }
    ///Container type for all input parameters for the `shouldKickValidator` function with signature `shouldKickValidator(address)` and selector `0x847e0625`
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
    #[ethcall(name = "shouldKickValidator", abi = "shouldKickValidator(address)")]
    pub struct ShouldKickValidatorCall {
        pub staker_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `signalReadyForNextEpoch` function with signature `signalReadyForNextEpoch(uint256)` and selector `0xf99b5623`
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
        name = "signalReadyForNextEpoch",
        abi = "signalReadyForNextEpoch(uint256)"
    )]
    pub struct SignalReadyForNextEpochCall {
        pub epoch_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(uint256)` and selector `0xa694fc3a`
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
    #[ethcall(name = "stake", abi = "stake(uint256)")]
    pub struct StakeCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakeAndJoin` function with signature `stakeAndJoin(uint256,uint32,uint128,uint32,address,uint256,uint256)` and selector `0xba3bd22e`
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
        name = "stakeAndJoin",
        abi = "stakeAndJoin(uint256,uint32,uint128,uint32,address,uint256,uint256)"
    )]
    pub struct StakeAndJoinCall {
        pub amount: ::ethers::core::types::U256,
        pub ip: u32,
        pub ipv_6: u128,
        pub port: u32,
        pub node_address: ::ethers::core::types::Address,
        pub sender_pub_key: ::ethers::core::types::U256,
        pub receiver_pub_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `state` function with signature `state()` and selector `0xc19d93fb`
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
    #[ethcall(name = "state", abi = "state()")]
    pub struct StateCall;
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
    ///Container type for all input parameters for the `validators` function with signature `validators(address)` and selector `0xfa52c7d8`
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
    #[ethcall(name = "validators", abi = "validators(address)")]
    pub struct ValidatorsCall {
        pub staker_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
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
    pub enum StakingCalls {
        AdminKickValidatorInNextEpoch(AdminKickValidatorInNextEpochCall),
        AdminRejoinValidator(AdminRejoinValidatorCall),
        AdminResetEpoch(AdminResetEpochCall),
        AdminSlashValidator(AdminSlashValidatorCall),
        AdvanceEpoch(AdvanceEpochCall),
        CheckVersion(CheckVersionCall),
        ComplaintConfig(ComplaintConfigCall),
        Config(ConfigCall),
        ContractResolver(ContractResolverCall),
        CountOfCurrentValidatorsReadyForNextEpoch(
            CountOfCurrentValidatorsReadyForNextEpochCall,
        ),
        CountOfNextValidatorsReadyForNextEpoch(
            CountOfNextValidatorsReadyForNextEpochCall,
        ),
        CurrentValidatorCountForConsensus(CurrentValidatorCountForConsensusCall),
        DiamondCut(DiamondCutCall),
        Epoch(EpochCall),
        Exit(ExitCall),
        FacetAddress(FacetAddressCall),
        FacetAddresses(FacetAddressesCall),
        FacetFunctionSelectors(FacetFunctionSelectorsCall),
        Facets(FacetsCall),
        GetKeyTypes(GetKeyTypesCall),
        GetKickedValidators(GetKickedValidatorsCall),
        GetMaxVersion(GetMaxVersionCall),
        GetMaxVersionString(GetMaxVersionStringCall),
        GetMinVersion(GetMinVersionCall),
        GetMinVersionString(GetMinVersionStringCall),
        GetNodeStakerAddressMappings(GetNodeStakerAddressMappingsCall),
        GetReward(GetRewardCall),
        GetStakingBalancesAddress(GetStakingBalancesAddressCall),
        GetTokenAddress(GetTokenAddressCall),
        GetValidatorsInCurrentEpoch(GetValidatorsInCurrentEpochCall),
        GetValidatorsInCurrentEpochLength(GetValidatorsInCurrentEpochLengthCall),
        GetValidatorsInNextEpoch(GetValidatorsInNextEpochCall),
        GetValidatorsStructs(GetValidatorsStructsCall),
        GetValidatorsStructsInCurrentEpoch(GetValidatorsStructsInCurrentEpochCall),
        GetValidatorsStructsInNextEpoch(GetValidatorsStructsInNextEpochCall),
        GetVotingStatusToKickValidator(GetVotingStatusToKickValidatorCall),
        IsActiveValidator(IsActiveValidatorCall),
        IsActiveValidatorByNodeAddress(IsActiveValidatorByNodeAddressCall),
        IsReadyForNextEpoch(IsReadyForNextEpochCall),
        KickPenaltyPercentByReason(KickPenaltyPercentByReasonCall),
        KickValidatorInNextEpoch(KickValidatorInNextEpochCall),
        LockValidatorsForNextEpoch(LockValidatorsForNextEpochCall),
        NextValidatorCountForConsensus(NextValidatorCountForConsensusCall),
        NodeAddressToStakerAddress(NodeAddressToStakerAddressCall),
        Owner(OwnerCall),
        ReadyForNextEpoch(ReadyForNextEpochCall),
        RequestToJoin(RequestToJoinCall),
        RequestToLeave(RequestToLeaveCall),
        RequestToLeaveAsNode(RequestToLeaveAsNodeCall),
        SetComplaintConfig(SetComplaintConfigCall),
        SetConfig(SetConfigCall),
        SetContractResolver(SetContractResolverCall),
        SetEpochEndTime(SetEpochEndTimeCall),
        SetEpochLength(SetEpochLengthCall),
        SetEpochState(SetEpochStateCall),
        SetEpochTimeout(SetEpochTimeoutCall),
        SetIpPortNodeAddressAndCommunicationPubKeys(
            SetIpPortNodeAddressAndCommunicationPubKeysCall,
        ),
        SetKickPenaltyPercent(SetKickPenaltyPercentCall),
        SetMaxVersion(SetMaxVersionCall),
        SetMinVersion(SetMinVersionCall),
        ShouldKickValidator(ShouldKickValidatorCall),
        SignalReadyForNextEpoch(SignalReadyForNextEpochCall),
        Stake(StakeCall),
        StakeAndJoin(StakeAndJoinCall),
        State(StateCall),
        SupportsInterface(SupportsInterfaceCall),
        TransferOwnership(TransferOwnershipCall),
        Validators(ValidatorsCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AdminKickValidatorInNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AdminKickValidatorInNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <AdminRejoinValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AdminRejoinValidator(decoded));
            }
            if let Ok(decoded)
                = <AdminResetEpochCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdminResetEpoch(decoded));
            }
            if let Ok(decoded)
                = <AdminSlashValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AdminSlashValidator(decoded));
            }
            if let Ok(decoded)
                = <AdvanceEpochCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AdvanceEpoch(decoded));
            }
            if let Ok(decoded)
                = <CheckVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckVersion(decoded));
            }
            if let Ok(decoded)
                = <ComplaintConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ComplaintConfig(decoded));
            }
            if let Ok(decoded)
                = <ConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Config(decoded));
            }
            if let Ok(decoded)
                = <ContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ContractResolver(decoded));
            }
            if let Ok(decoded)
                = <CountOfCurrentValidatorsReadyForNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CountOfCurrentValidatorsReadyForNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <CountOfNextValidatorsReadyForNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CountOfNextValidatorsReadyForNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <CurrentValidatorCountForConsensusCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentValidatorCountForConsensus(decoded));
            }
            if let Ok(decoded)
                = <DiamondCutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DiamondCut(decoded));
            }
            if let Ok(decoded)
                = <EpochCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Epoch(decoded));
            }
            if let Ok(decoded)
                = <ExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Exit(decoded));
            }
            if let Ok(decoded)
                = <FacetAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FacetAddress(decoded));
            }
            if let Ok(decoded)
                = <FacetAddressesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FacetAddresses(decoded));
            }
            if let Ok(decoded)
                = <FacetFunctionSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FacetFunctionSelectors(decoded));
            }
            if let Ok(decoded)
                = <FacetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Facets(decoded));
            }
            if let Ok(decoded)
                = <GetKeyTypesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetKeyTypes(decoded));
            }
            if let Ok(decoded)
                = <GetKickedValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetKickedValidators(decoded));
            }
            if let Ok(decoded)
                = <GetMaxVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxVersion(decoded));
            }
            if let Ok(decoded)
                = <GetMaxVersionStringCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetMaxVersionString(decoded));
            }
            if let Ok(decoded)
                = <GetMinVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinVersion(decoded));
            }
            if let Ok(decoded)
                = <GetMinVersionStringCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetMinVersionString(decoded));
            }
            if let Ok(decoded)
                = <GetNodeStakerAddressMappingsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetNodeStakerAddressMappings(decoded));
            }
            if let Ok(decoded)
                = <GetRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReward(decoded));
            }
            if let Ok(decoded)
                = <GetStakingBalancesAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStakingBalancesAddress(decoded));
            }
            if let Ok(decoded)
                = <GetTokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTokenAddress(decoded));
            }
            if let Ok(decoded)
                = <GetValidatorsInCurrentEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetValidatorsInCurrentEpoch(decoded));
            }
            if let Ok(decoded)
                = <GetValidatorsInCurrentEpochLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetValidatorsInCurrentEpochLength(decoded));
            }
            if let Ok(decoded)
                = <GetValidatorsInNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetValidatorsInNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <GetValidatorsStructsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetValidatorsStructs(decoded));
            }
            if let Ok(decoded)
                = <GetValidatorsStructsInCurrentEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetValidatorsStructsInCurrentEpoch(decoded));
            }
            if let Ok(decoded)
                = <GetValidatorsStructsInNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetValidatorsStructsInNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <GetVotingStatusToKickValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVotingStatusToKickValidator(decoded));
            }
            if let Ok(decoded)
                = <IsActiveValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsActiveValidator(decoded));
            }
            if let Ok(decoded)
                = <IsActiveValidatorByNodeAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsActiveValidatorByNodeAddress(decoded));
            }
            if let Ok(decoded)
                = <IsReadyForNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsReadyForNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <KickPenaltyPercentByReasonCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::KickPenaltyPercentByReason(decoded));
            }
            if let Ok(decoded)
                = <KickValidatorInNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::KickValidatorInNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <LockValidatorsForNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LockValidatorsForNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <NextValidatorCountForConsensusCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NextValidatorCountForConsensus(decoded));
            }
            if let Ok(decoded)
                = <NodeAddressToStakerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NodeAddressToStakerAddress(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <ReadyForNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReadyForNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <RequestToJoinCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestToJoin(decoded));
            }
            if let Ok(decoded)
                = <RequestToLeaveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestToLeave(decoded));
            }
            if let Ok(decoded)
                = <RequestToLeaveAsNodeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RequestToLeaveAsNode(decoded));
            }
            if let Ok(decoded)
                = <SetComplaintConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetComplaintConfig(decoded));
            }
            if let Ok(decoded)
                = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded)
                = <SetContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetContractResolver(decoded));
            }
            if let Ok(decoded)
                = <SetEpochEndTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEpochEndTime(decoded));
            }
            if let Ok(decoded)
                = <SetEpochLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEpochLength(decoded));
            }
            if let Ok(decoded)
                = <SetEpochStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEpochState(decoded));
            }
            if let Ok(decoded)
                = <SetEpochTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEpochTimeout(decoded));
            }
            if let Ok(decoded)
                = <SetIpPortNodeAddressAndCommunicationPubKeysCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetIpPortNodeAddressAndCommunicationPubKeys(decoded));
            }
            if let Ok(decoded)
                = <SetKickPenaltyPercentCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetKickPenaltyPercent(decoded));
            }
            if let Ok(decoded)
                = <SetMaxVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMaxVersion(decoded));
            }
            if let Ok(decoded)
                = <SetMinVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinVersion(decoded));
            }
            if let Ok(decoded)
                = <ShouldKickValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ShouldKickValidator(decoded));
            }
            if let Ok(decoded)
                = <SignalReadyForNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SignalReadyForNextEpoch(decoded));
            }
            if let Ok(decoded)
                = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded)
                = <StakeAndJoinCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakeAndJoin(decoded));
            }
            if let Ok(decoded)
                = <StateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::State(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <ValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Validators(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminKickValidatorInNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminRejoinValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminResetEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminSlashValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdvanceEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComplaintConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Config(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CountOfCurrentValidatorsReadyForNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CountOfNextValidatorsReadyForNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentValidatorCountForConsensus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Epoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Exit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FacetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetFunctionSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Facets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetKeyTypes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetKickedValidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxVersionString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMinVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMinVersionString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNodeStakerAddressMappings(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakingBalancesAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorsInCurrentEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorsInCurrentEpochLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorsInNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorsStructs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorsStructsInCurrentEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorsStructsInNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVotingStatusToKickValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsActiveValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsActiveValidatorByNodeAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsReadyForNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KickPenaltyPercentByReason(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KickValidatorInNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockValidatorsForNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextValidatorCountForConsensus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NodeAddressToStakerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReadyForNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestToJoin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestToLeave(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestToLeaveAsNode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetComplaintConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEpochEndTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEpochLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEpochState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEpochTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetIpPortNodeAddressAndCommunicationPubKeys(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetKickPenaltyPercent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMinVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldKickValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignalReadyForNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeAndJoin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Validators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StakingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminKickValidatorInNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdminRejoinValidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdminResetEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminSlashValidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdvanceEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComplaintConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Config(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::CountOfCurrentValidatorsReadyForNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CountOfNextValidatorsReadyForNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurrentValidatorCountForConsensus(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondCut(element) => ::core::fmt::Display::fmt(element, f),
                Self::Epoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetFunctionSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Facets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetKeyTypes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetKickedValidators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMaxVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxVersionString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMinVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinVersionString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNodeStakerAddressMappings(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakingBalancesAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetValidatorsInCurrentEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidatorsInCurrentEpochLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidatorsInNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidatorsStructs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidatorsStructsInCurrentEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidatorsStructsInNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVotingStatusToKickValidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsActiveValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsActiveValidatorByNodeAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsReadyForNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::KickPenaltyPercentByReason(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::KickValidatorInNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LockValidatorsForNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NextValidatorCountForConsensus(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NodeAddressToStakerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadyForNextEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestToJoin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestToLeave(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestToLeaveAsNode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetComplaintConfig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractResolver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetEpochEndTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEpochLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEpochState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEpochTimeout(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetIpPortNodeAddressAndCommunicationPubKeys(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetKickPenaltyPercent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShouldKickValidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignalReadyForNextEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeAndJoin(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validators(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminKickValidatorInNextEpochCall> for StakingCalls {
        fn from(value: AdminKickValidatorInNextEpochCall) -> Self {
            Self::AdminKickValidatorInNextEpoch(value)
        }
    }
    impl ::core::convert::From<AdminRejoinValidatorCall> for StakingCalls {
        fn from(value: AdminRejoinValidatorCall) -> Self {
            Self::AdminRejoinValidator(value)
        }
    }
    impl ::core::convert::From<AdminResetEpochCall> for StakingCalls {
        fn from(value: AdminResetEpochCall) -> Self {
            Self::AdminResetEpoch(value)
        }
    }
    impl ::core::convert::From<AdminSlashValidatorCall> for StakingCalls {
        fn from(value: AdminSlashValidatorCall) -> Self {
            Self::AdminSlashValidator(value)
        }
    }
    impl ::core::convert::From<AdvanceEpochCall> for StakingCalls {
        fn from(value: AdvanceEpochCall) -> Self {
            Self::AdvanceEpoch(value)
        }
    }
    impl ::core::convert::From<CheckVersionCall> for StakingCalls {
        fn from(value: CheckVersionCall) -> Self {
            Self::CheckVersion(value)
        }
    }
    impl ::core::convert::From<ComplaintConfigCall> for StakingCalls {
        fn from(value: ComplaintConfigCall) -> Self {
            Self::ComplaintConfig(value)
        }
    }
    impl ::core::convert::From<ConfigCall> for StakingCalls {
        fn from(value: ConfigCall) -> Self {
            Self::Config(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for StakingCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<CountOfCurrentValidatorsReadyForNextEpochCall>
    for StakingCalls {
        fn from(value: CountOfCurrentValidatorsReadyForNextEpochCall) -> Self {
            Self::CountOfCurrentValidatorsReadyForNextEpoch(value)
        }
    }
    impl ::core::convert::From<CountOfNextValidatorsReadyForNextEpochCall>
    for StakingCalls {
        fn from(value: CountOfNextValidatorsReadyForNextEpochCall) -> Self {
            Self::CountOfNextValidatorsReadyForNextEpoch(value)
        }
    }
    impl ::core::convert::From<CurrentValidatorCountForConsensusCall> for StakingCalls {
        fn from(value: CurrentValidatorCountForConsensusCall) -> Self {
            Self::CurrentValidatorCountForConsensus(value)
        }
    }
    impl ::core::convert::From<DiamondCutCall> for StakingCalls {
        fn from(value: DiamondCutCall) -> Self {
            Self::DiamondCut(value)
        }
    }
    impl ::core::convert::From<EpochCall> for StakingCalls {
        fn from(value: EpochCall) -> Self {
            Self::Epoch(value)
        }
    }
    impl ::core::convert::From<ExitCall> for StakingCalls {
        fn from(value: ExitCall) -> Self {
            Self::Exit(value)
        }
    }
    impl ::core::convert::From<FacetAddressCall> for StakingCalls {
        fn from(value: FacetAddressCall) -> Self {
            Self::FacetAddress(value)
        }
    }
    impl ::core::convert::From<FacetAddressesCall> for StakingCalls {
        fn from(value: FacetAddressesCall) -> Self {
            Self::FacetAddresses(value)
        }
    }
    impl ::core::convert::From<FacetFunctionSelectorsCall> for StakingCalls {
        fn from(value: FacetFunctionSelectorsCall) -> Self {
            Self::FacetFunctionSelectors(value)
        }
    }
    impl ::core::convert::From<FacetsCall> for StakingCalls {
        fn from(value: FacetsCall) -> Self {
            Self::Facets(value)
        }
    }
    impl ::core::convert::From<GetKeyTypesCall> for StakingCalls {
        fn from(value: GetKeyTypesCall) -> Self {
            Self::GetKeyTypes(value)
        }
    }
    impl ::core::convert::From<GetKickedValidatorsCall> for StakingCalls {
        fn from(value: GetKickedValidatorsCall) -> Self {
            Self::GetKickedValidators(value)
        }
    }
    impl ::core::convert::From<GetMaxVersionCall> for StakingCalls {
        fn from(value: GetMaxVersionCall) -> Self {
            Self::GetMaxVersion(value)
        }
    }
    impl ::core::convert::From<GetMaxVersionStringCall> for StakingCalls {
        fn from(value: GetMaxVersionStringCall) -> Self {
            Self::GetMaxVersionString(value)
        }
    }
    impl ::core::convert::From<GetMinVersionCall> for StakingCalls {
        fn from(value: GetMinVersionCall) -> Self {
            Self::GetMinVersion(value)
        }
    }
    impl ::core::convert::From<GetMinVersionStringCall> for StakingCalls {
        fn from(value: GetMinVersionStringCall) -> Self {
            Self::GetMinVersionString(value)
        }
    }
    impl ::core::convert::From<GetNodeStakerAddressMappingsCall> for StakingCalls {
        fn from(value: GetNodeStakerAddressMappingsCall) -> Self {
            Self::GetNodeStakerAddressMappings(value)
        }
    }
    impl ::core::convert::From<GetRewardCall> for StakingCalls {
        fn from(value: GetRewardCall) -> Self {
            Self::GetReward(value)
        }
    }
    impl ::core::convert::From<GetStakingBalancesAddressCall> for StakingCalls {
        fn from(value: GetStakingBalancesAddressCall) -> Self {
            Self::GetStakingBalancesAddress(value)
        }
    }
    impl ::core::convert::From<GetTokenAddressCall> for StakingCalls {
        fn from(value: GetTokenAddressCall) -> Self {
            Self::GetTokenAddress(value)
        }
    }
    impl ::core::convert::From<GetValidatorsInCurrentEpochCall> for StakingCalls {
        fn from(value: GetValidatorsInCurrentEpochCall) -> Self {
            Self::GetValidatorsInCurrentEpoch(value)
        }
    }
    impl ::core::convert::From<GetValidatorsInCurrentEpochLengthCall> for StakingCalls {
        fn from(value: GetValidatorsInCurrentEpochLengthCall) -> Self {
            Self::GetValidatorsInCurrentEpochLength(value)
        }
    }
    impl ::core::convert::From<GetValidatorsInNextEpochCall> for StakingCalls {
        fn from(value: GetValidatorsInNextEpochCall) -> Self {
            Self::GetValidatorsInNextEpoch(value)
        }
    }
    impl ::core::convert::From<GetValidatorsStructsCall> for StakingCalls {
        fn from(value: GetValidatorsStructsCall) -> Self {
            Self::GetValidatorsStructs(value)
        }
    }
    impl ::core::convert::From<GetValidatorsStructsInCurrentEpochCall> for StakingCalls {
        fn from(value: GetValidatorsStructsInCurrentEpochCall) -> Self {
            Self::GetValidatorsStructsInCurrentEpoch(value)
        }
    }
    impl ::core::convert::From<GetValidatorsStructsInNextEpochCall> for StakingCalls {
        fn from(value: GetValidatorsStructsInNextEpochCall) -> Self {
            Self::GetValidatorsStructsInNextEpoch(value)
        }
    }
    impl ::core::convert::From<GetVotingStatusToKickValidatorCall> for StakingCalls {
        fn from(value: GetVotingStatusToKickValidatorCall) -> Self {
            Self::GetVotingStatusToKickValidator(value)
        }
    }
    impl ::core::convert::From<IsActiveValidatorCall> for StakingCalls {
        fn from(value: IsActiveValidatorCall) -> Self {
            Self::IsActiveValidator(value)
        }
    }
    impl ::core::convert::From<IsActiveValidatorByNodeAddressCall> for StakingCalls {
        fn from(value: IsActiveValidatorByNodeAddressCall) -> Self {
            Self::IsActiveValidatorByNodeAddress(value)
        }
    }
    impl ::core::convert::From<IsReadyForNextEpochCall> for StakingCalls {
        fn from(value: IsReadyForNextEpochCall) -> Self {
            Self::IsReadyForNextEpoch(value)
        }
    }
    impl ::core::convert::From<KickPenaltyPercentByReasonCall> for StakingCalls {
        fn from(value: KickPenaltyPercentByReasonCall) -> Self {
            Self::KickPenaltyPercentByReason(value)
        }
    }
    impl ::core::convert::From<KickValidatorInNextEpochCall> for StakingCalls {
        fn from(value: KickValidatorInNextEpochCall) -> Self {
            Self::KickValidatorInNextEpoch(value)
        }
    }
    impl ::core::convert::From<LockValidatorsForNextEpochCall> for StakingCalls {
        fn from(value: LockValidatorsForNextEpochCall) -> Self {
            Self::LockValidatorsForNextEpoch(value)
        }
    }
    impl ::core::convert::From<NextValidatorCountForConsensusCall> for StakingCalls {
        fn from(value: NextValidatorCountForConsensusCall) -> Self {
            Self::NextValidatorCountForConsensus(value)
        }
    }
    impl ::core::convert::From<NodeAddressToStakerAddressCall> for StakingCalls {
        fn from(value: NodeAddressToStakerAddressCall) -> Self {
            Self::NodeAddressToStakerAddress(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for StakingCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ReadyForNextEpochCall> for StakingCalls {
        fn from(value: ReadyForNextEpochCall) -> Self {
            Self::ReadyForNextEpoch(value)
        }
    }
    impl ::core::convert::From<RequestToJoinCall> for StakingCalls {
        fn from(value: RequestToJoinCall) -> Self {
            Self::RequestToJoin(value)
        }
    }
    impl ::core::convert::From<RequestToLeaveCall> for StakingCalls {
        fn from(value: RequestToLeaveCall) -> Self {
            Self::RequestToLeave(value)
        }
    }
    impl ::core::convert::From<RequestToLeaveAsNodeCall> for StakingCalls {
        fn from(value: RequestToLeaveAsNodeCall) -> Self {
            Self::RequestToLeaveAsNode(value)
        }
    }
    impl ::core::convert::From<SetComplaintConfigCall> for StakingCalls {
        fn from(value: SetComplaintConfigCall) -> Self {
            Self::SetComplaintConfig(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for StakingCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetContractResolverCall> for StakingCalls {
        fn from(value: SetContractResolverCall) -> Self {
            Self::SetContractResolver(value)
        }
    }
    impl ::core::convert::From<SetEpochEndTimeCall> for StakingCalls {
        fn from(value: SetEpochEndTimeCall) -> Self {
            Self::SetEpochEndTime(value)
        }
    }
    impl ::core::convert::From<SetEpochLengthCall> for StakingCalls {
        fn from(value: SetEpochLengthCall) -> Self {
            Self::SetEpochLength(value)
        }
    }
    impl ::core::convert::From<SetEpochStateCall> for StakingCalls {
        fn from(value: SetEpochStateCall) -> Self {
            Self::SetEpochState(value)
        }
    }
    impl ::core::convert::From<SetEpochTimeoutCall> for StakingCalls {
        fn from(value: SetEpochTimeoutCall) -> Self {
            Self::SetEpochTimeout(value)
        }
    }
    impl ::core::convert::From<SetIpPortNodeAddressAndCommunicationPubKeysCall>
    for StakingCalls {
        fn from(value: SetIpPortNodeAddressAndCommunicationPubKeysCall) -> Self {
            Self::SetIpPortNodeAddressAndCommunicationPubKeys(value)
        }
    }
    impl ::core::convert::From<SetKickPenaltyPercentCall> for StakingCalls {
        fn from(value: SetKickPenaltyPercentCall) -> Self {
            Self::SetKickPenaltyPercent(value)
        }
    }
    impl ::core::convert::From<SetMaxVersionCall> for StakingCalls {
        fn from(value: SetMaxVersionCall) -> Self {
            Self::SetMaxVersion(value)
        }
    }
    impl ::core::convert::From<SetMinVersionCall> for StakingCalls {
        fn from(value: SetMinVersionCall) -> Self {
            Self::SetMinVersion(value)
        }
    }
    impl ::core::convert::From<ShouldKickValidatorCall> for StakingCalls {
        fn from(value: ShouldKickValidatorCall) -> Self {
            Self::ShouldKickValidator(value)
        }
    }
    impl ::core::convert::From<SignalReadyForNextEpochCall> for StakingCalls {
        fn from(value: SignalReadyForNextEpochCall) -> Self {
            Self::SignalReadyForNextEpoch(value)
        }
    }
    impl ::core::convert::From<StakeCall> for StakingCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<StakeAndJoinCall> for StakingCalls {
        fn from(value: StakeAndJoinCall) -> Self {
            Self::StakeAndJoin(value)
        }
    }
    impl ::core::convert::From<StateCall> for StakingCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for StakingCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for StakingCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<ValidatorsCall> for StakingCalls {
        fn from(value: ValidatorsCall) -> Self {
            Self::Validators(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for StakingCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `checkVersion` function with signature `checkVersion((uint256,uint256,uint256))` and selector `0x7720ff0f`
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
    pub struct CheckVersionReturn(pub bool);
    ///Container type for all return fields from the `complaintConfig` function with signature `complaintConfig(uint256)` and selector `0x02f6da56`
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
    pub struct ComplaintConfigReturn(pub ComplaintConfig);
    ///Container type for all return fields from the `config` function with signature `config()` and selector `0x79502c55`
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
    pub struct ConfigReturn(pub Config);
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
    ///Container type for all return fields from the `countOfCurrentValidatorsReadyForNextEpoch` function with signature `countOfCurrentValidatorsReadyForNextEpoch()` and selector `0xe8684ed1`
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
    pub struct CountOfCurrentValidatorsReadyForNextEpochReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `countOfNextValidatorsReadyForNextEpoch` function with signature `countOfNextValidatorsReadyForNextEpoch()` and selector `0x89965883`
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
    pub struct CountOfNextValidatorsReadyForNextEpochReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `currentValidatorCountForConsensus` function with signature `currentValidatorCountForConsensus()` and selector `0x43cb0a0e`
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
    pub struct CurrentValidatorCountForConsensusReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `epoch` function with signature `epoch()` and selector `0x900cf0cf`
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
    pub struct EpochReturn(pub Epoch);
    ///Container type for all return fields from the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    pub struct FacetAddressReturn {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    pub struct FacetAddressesReturn {
        pub facet_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    pub struct FacetFunctionSelectorsReturn {
        pub facet_function_selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Container type for all return fields from the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    pub struct FacetsReturn {
        pub facets: ::std::vec::Vec<Facet>,
    }
    ///Container type for all return fields from the `getKeyTypes` function with signature `getKeyTypes()` and selector `0xf1b877a8`
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
    pub struct GetKeyTypesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getKickedValidators` function with signature `getKickedValidators()` and selector `0x5995a4c4`
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
    pub struct GetKickedValidatorsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getMaxVersion` function with signature `getMaxVersion()` and selector `0x2c20d23a`
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
    pub struct GetMaxVersionReturn(pub Version);
    ///Container type for all return fields from the `getMaxVersionString` function with signature `getMaxVersionString()` and selector `0xdbd68aa7`
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
    pub struct GetMaxVersionStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getMinVersion` function with signature `getMinVersion()` and selector `0xd1e3f3d7`
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
    pub struct GetMinVersionReturn(pub Version);
    ///Container type for all return fields from the `getMinVersionString` function with signature `getMinVersionString()` and selector `0x517d2324`
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
    pub struct GetMinVersionStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getNodeStakerAddressMappings` function with signature `getNodeStakerAddressMappings(address[])` and selector `0x90fba112`
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
    pub struct GetNodeStakerAddressMappingsReturn(pub ::std::vec::Vec<AddressMapping>);
    ///Container type for all return fields from the `getStakingBalancesAddress` function with signature `getStakingBalancesAddress()` and selector `0x5b677eac`
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
    pub struct GetStakingBalancesAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getTokenAddress` function with signature `getTokenAddress()` and selector `0x10fe9ae8`
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
    pub struct GetTokenAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getValidatorsInCurrentEpoch` function with signature `getValidatorsInCurrentEpoch()` and selector `0x857b7663`
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
    pub struct GetValidatorsInCurrentEpochReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getValidatorsInCurrentEpochLength` function with signature `getValidatorsInCurrentEpochLength()` and selector `0xd4818fca`
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
    pub struct GetValidatorsInCurrentEpochLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getValidatorsInNextEpoch` function with signature `getValidatorsInNextEpoch()` and selector `0xc35d4d09`
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
    pub struct GetValidatorsInNextEpochReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getValidatorsStructs` function with signature `getValidatorsStructs(address[])` and selector `0x533d463e`
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
    pub struct GetValidatorsStructsReturn(pub ::std::vec::Vec<Validator>);
    ///Container type for all return fields from the `getValidatorsStructsInCurrentEpoch` function with signature `getValidatorsStructsInCurrentEpoch()` and selector `0xe7c08720`
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
    pub struct GetValidatorsStructsInCurrentEpochReturn(pub ::std::vec::Vec<Validator>);
    ///Container type for all return fields from the `getValidatorsStructsInNextEpoch` function with signature `getValidatorsStructsInNextEpoch()` and selector `0x61dee8a3`
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
    pub struct GetValidatorsStructsInNextEpochReturn(pub ::std::vec::Vec<Validator>);
    ///Container type for all return fields from the `getVotingStatusToKickValidator` function with signature `getVotingStatusToKickValidator(uint256,address,address)` and selector `0x70fe276a`
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
    pub struct GetVotingStatusToKickValidatorReturn(
        pub ::ethers::core::types::U256,
        pub bool,
    );
    ///Container type for all return fields from the `isActiveValidator` function with signature `isActiveValidator(address)` and selector `0x40550a1c`
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
    pub struct IsActiveValidatorReturn(pub bool);
    ///Container type for all return fields from the `isActiveValidatorByNodeAddress` function with signature `isActiveValidatorByNodeAddress(address)` and selector `0xa25e49a4`
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
    pub struct IsActiveValidatorByNodeAddressReturn(pub bool);
    ///Container type for all return fields from the `isReadyForNextEpoch` function with signature `isReadyForNextEpoch()` and selector `0xf1887fec`
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
    pub struct IsReadyForNextEpochReturn(pub bool);
    ///Container type for all return fields from the `kickPenaltyPercentByReason` function with signature `kickPenaltyPercentByReason(uint256)` and selector `0x3e685266`
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
    pub struct KickPenaltyPercentByReasonReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nextValidatorCountForConsensus` function with signature `nextValidatorCountForConsensus()` and selector `0x0297d4db`
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
    pub struct NextValidatorCountForConsensusReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nodeAddressToStakerAddress` function with signature `nodeAddressToStakerAddress(address)` and selector `0x5081f66f`
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
    pub struct NodeAddressToStakerAddressReturn(pub ::ethers::core::types::Address);
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
    pub struct OwnerReturn {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `readyForNextEpoch` function with signature `readyForNextEpoch(address)` and selector `0x519877eb`
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
    pub struct ReadyForNextEpochReturn(pub bool);
    ///Container type for all return fields from the `shouldKickValidator` function with signature `shouldKickValidator(address)` and selector `0x847e0625`
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
    pub struct ShouldKickValidatorReturn(pub bool);
    ///Container type for all return fields from the `state` function with signature `state()` and selector `0xc19d93fb`
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
    pub struct StateReturn(pub u8);
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
    ///Container type for all return fields from the `validators` function with signature `validators(address)` and selector `0xfa52c7d8`
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
    pub struct ValidatorsReturn(pub Validator);
    ///`FacetCut(address,uint8,bytes4[])`
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
    pub struct FacetCut {
        pub facet_address: ::ethers::core::types::Address,
        pub action: u8,
        pub function_selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///`Facet(address,bytes4[])`
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
    pub struct Facet {
        pub facet_address: ::ethers::core::types::Address,
        pub function_selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///`AddressMapping(address,address)`
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
    pub struct AddressMapping {
        pub node_address: ::ethers::core::types::Address,
        pub staker_address: ::ethers::core::types::Address,
    }
    ///`ComplaintConfig(uint256,uint256,uint256)`
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
    pub struct ComplaintConfig {
        pub tolerance: ::ethers::core::types::U256,
        pub interval_secs: ::ethers::core::types::U256,
        pub kick_penalty_percent: ::ethers::core::types::U256,
    }
    ///`Config(uint256,uint256,uint256,uint256[],uint256,uint256,uint256,uint256,uint256,uint256,bool)`
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
    pub struct Config {
        pub token_reward_per_token_per_epoch: ::ethers::core::types::U256,
        pub deprecated_complaint_tolerance: ::ethers::core::types::U256,
        pub deprecated_complaint_interval_secs: ::ethers::core::types::U256,
        pub key_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub minimum_validator_count: ::ethers::core::types::U256,
        pub max_concurrent_requests: ::ethers::core::types::U256,
        pub max_triple_count: ::ethers::core::types::U256,
        pub min_triple_count: ::ethers::core::types::U256,
        pub peer_checking_interval_secs: ::ethers::core::types::U256,
        pub max_triple_concurrency: ::ethers::core::types::U256,
        pub rpc_healthcheck_enabled: bool,
    }
    ///`Epoch(uint256,uint256,uint256,uint256,uint256)`
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
    pub struct Epoch {
        pub epoch_length: ::ethers::core::types::U256,
        pub number: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
        pub retries: ::ethers::core::types::U256,
        pub timeout: ::ethers::core::types::U256,
    }
    ///`Validator(uint32,uint128,uint32,address,uint256,uint256,uint256)`
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
    pub struct Validator {
        pub ip: u32,
        pub ipv_6: u128,
        pub port: u32,
        pub node_address: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
        pub sender_pub_key: ::ethers::core::types::U256,
        pub receiver_pub_key: ::ethers::core::types::U256,
    }
    ///`Version(uint256,uint256,uint256)`
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
    pub struct Version {
        pub major: ::ethers::core::types::U256,
        pub minor: ::ethers::core::types::U256,
        pub patch: ::ethers::core::types::U256,
    }
}
