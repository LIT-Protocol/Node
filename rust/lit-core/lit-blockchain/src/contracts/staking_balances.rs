pub use staking_balances::*;
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
pub mod staking_balances {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/StakingBalances.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addAlias"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAlias"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aliasAccount"),
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
                    ::std::borrow::ToOwned::to_owned("addPermittedStaker"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addPermittedStaker"),
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
                    ::std::borrow::ToOwned::to_owned("addPermittedStakers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addPermittedStakers",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakers"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("checkStakingAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkStakingAmounts",
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
                    ::std::borrow::ToOwned::to_owned("getReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReward"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getStakingAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStakingAddress"),
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
                    ::std::borrow::ToOwned::to_owned("isPermittedStaker"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPermittedStaker"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
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
                    ::std::borrow::ToOwned::to_owned("maximumStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maximumStake"),
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
                    ::std::borrow::ToOwned::to_owned("minimumStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minimumStake"),
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
                    ::std::borrow::ToOwned::to_owned("penalizeTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("penalizeTokens"),
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
                    ::std::borrow::ToOwned::to_owned("permittedStakersOn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permittedStakersOn"),
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
                    ::std::borrow::ToOwned::to_owned("removeAlias"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAlias"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aliasAccount"),
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
                    ::std::borrow::ToOwned::to_owned("removePermittedStaker"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removePermittedStaker",
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
                    ::std::borrow::ToOwned::to_owned("restakePenaltyTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "restakePenaltyTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("rewardOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardOf"),
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
                    ::std::borrow::ToOwned::to_owned("rewardValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardValidator"),
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
                    ::std::borrow::ToOwned::to_owned("setMaxAliasCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaxAliasCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxAliasCount"),
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
                    ::std::borrow::ToOwned::to_owned("setMaximumStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMaximumStake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMaximumStake"),
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
                    ::std::borrow::ToOwned::to_owned("setMinimumStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMinimumStake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMinimumStake"),
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
                    ::std::borrow::ToOwned::to_owned("setPermittedStakersOn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPermittedStakersOn",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitted"),
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
                    ::std::borrow::ToOwned::to_owned("totalStaked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalStaked"),
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
                    ::std::borrow::ToOwned::to_owned("transferPenaltyTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "transferPenaltyTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawPenaltyTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawPenaltyTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("AliasAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AliasAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aliasAccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AliasRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AliasRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aliasAccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("MaxAliasCountSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MaxAliasCountSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxAliasCount"),
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
                    ::std::borrow::ToOwned::to_owned("MaximumStakeSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MaximumStakeSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMaximumStake"),
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
                    ::std::borrow::ToOwned::to_owned("MinimumStakeSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MinimumStakeSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMinimumStake"),
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
                    ::std::borrow::ToOwned::to_owned("PermittedStakerAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PermittedStakerAdded",
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
                    ::std::borrow::ToOwned::to_owned("PermittedStakerRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PermittedStakerRemoved",
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
                    ::std::borrow::ToOwned::to_owned("PermittedStakersOnChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PermittedStakersOnChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedStakersOn",
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
                    ::std::borrow::ToOwned::to_owned("ResolverContractAddressSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResolverContractAddressSet",
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
                    ::std::borrow::ToOwned::to_owned("RewardPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardPaid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
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
                    ::std::borrow::ToOwned::to_owned("Staked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Staked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("TokenRewardPerTokenPerEpochSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TokenRewardPerTokenPerEpochSet",
                            ),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorNotRewardedBecauseAlias"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ValidatorNotRewardedBecauseAlias",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aliasAccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorRewarded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ValidatorRewarded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("ValidatorTokensPenalized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ValidatorTokensPenalized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("Withdrawn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdrawn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("AliasNotOwnedBySender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AliasNotOwnedBySender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aliasAccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                        "CannotRemoveAliasOfActiveValidator",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRemoveAliasOfActiveValidator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aliasAccount"),
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
                    ::std::borrow::ToOwned::to_owned("CannotStakeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CannotStakeZero"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("MaxAliasCountReached"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MaxAliasCountReached",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aliasCount"),
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
                    ::std::borrow::ToOwned::to_owned("OnlyStakingContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OnlyStakingContract",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "StakeMustBeGreaterThanMinimumStake",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StakeMustBeGreaterThanMinimumStake",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountStaked"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumStake"),
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
                    ::std::borrow::ToOwned::to_owned("StakeMustBeLessThanMaximumStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StakeMustBeLessThanMaximumStake",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountStaked"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maximumStake"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKINGBALANCES_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct StakingBalances<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakingBalances<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakingBalances<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakingBalances<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakingBalances<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakingBalances))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakingBalances<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STAKINGBALANCES_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addAlias` (0x40b96482) function
        pub fn add_alias(
            &self,
            alias_account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 185, 100, 130], alias_account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPermittedStaker` (0x74c2e2fc) function
        pub fn add_permitted_staker(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 194, 226, 252], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPermittedStakers` (0x1dc4ab3c) function
        pub fn add_permitted_stakers(
            &self,
            stakers: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 196, 171, 60], stakers)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkStakingAmounts` (0x49919966) function
        pub fn check_staking_amounts(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 145, 153, 102], account)
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
        ///Calls the contract's `getReward` (0xc00007b0) function
        pub fn get_reward(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 0, 7, 176], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakingAddress` (0x0e9ed68b) function
        pub fn get_staking_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([14, 158, 214, 139], ())
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
        ///Calls the contract's `isPermittedStaker` (0xd3dbad7d) function
        pub fn is_permitted_staker(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([211, 219, 173, 125], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maximumStake` (0xb5c6b453) function
        pub fn maximum_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([181, 198, 180, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumStake` (0xec5ffac2) function
        pub fn minimum_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 95, 250, 194], ())
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
        ///Calls the contract's `penalizeTokens` (0x0a0e3dea) function
        pub fn penalize_tokens(
            &self,
            amount: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 14, 61, 234], (amount, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permittedStakersOn` (0x27a199d0) function
        pub fn permitted_stakers_on(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([39, 161, 153, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAlias` (0xbc979787) function
        pub fn remove_alias(
            &self,
            alias_account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 151, 151, 135], alias_account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePermittedStaker` (0xa498072c) function
        pub fn remove_permitted_staker(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 152, 7, 44], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `restakePenaltyTokens` (0xf21aec06) function
        pub fn restake_penalty_tokens(
            &self,
            staker: ::ethers::core::types::Address,
            balance: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 26, 236, 6], (staker, balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardOf` (0x1d62ebd9) function
        pub fn reward_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 98, 235, 217], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardValidator` (0x02fa04c9) function
        pub fn reward_validator(
            &self,
            amount: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 250, 4, 201], (amount, account))
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
        ///Calls the contract's `setMaxAliasCount` (0xddb6e6e1) function
        pub fn set_max_alias_count(
            &self,
            new_max_alias_count: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 182, 230, 225], new_max_alias_count)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaximumStake` (0xbcb5c636) function
        pub fn set_maximum_stake(
            &self,
            new_maximum_stake: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 181, 198, 54], new_maximum_stake)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinimumStake` (0x233e9903) function
        pub fn set_minimum_stake(
            &self,
            new_minimum_stake: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 62, 153, 3], new_minimum_stake)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPermittedStakersOn` (0x842648c2) function
        pub fn set_permitted_stakers_on(
            &self,
            permitted: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 38, 72, 194], permitted)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0x7acb7757) function
        pub fn stake(
            &self,
            amount: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 203, 119, 87], (amount, account))
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
        ///Calls the contract's `totalStaked` (0x817b1cd2) function
        pub fn total_staked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([129, 123, 28, 210], ())
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
        ///Calls the contract's `transferPenaltyTokens` (0x3560db3c) function
        pub fn transfer_penalty_tokens(
            &self,
            balance: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 96, 219, 60], (balance, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x00f714ce) function
        pub fn withdraw_with_amount_and_account(
            &self,
            amount: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 247, 20, 206], (amount, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x3ccfd60b) function
        pub fn withdraw(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawPenaltyTokens` (0xdc157ce2) function
        pub fn withdraw_penalty_tokens(
            &self,
            balance: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 21, 124, 226], balance)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AliasAdded` event
        pub fn alias_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AliasAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AliasRemoved` event
        pub fn alias_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AliasRemovedFilter,
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
        ///Gets the contract's `MaxAliasCountSet` event
        pub fn max_alias_count_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxAliasCountSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaximumStakeSet` event
        pub fn maximum_stake_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaximumStakeSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinimumStakeSet` event
        pub fn minimum_stake_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinimumStakeSetFilter,
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
        ///Gets the contract's `PermittedStakerAdded` event
        pub fn permitted_staker_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermittedStakerAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PermittedStakerRemoved` event
        pub fn permitted_staker_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermittedStakerRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PermittedStakersOnChanged` event
        pub fn permitted_stakers_on_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermittedStakersOnChangedFilter,
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
        ///Gets the contract's `RewardPaid` event
        pub fn reward_paid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardPaidFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Staked` event
        pub fn staked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakedFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokenRewardPerTokenPerEpochSet` event
        pub fn token_reward_per_token_per_epoch_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenRewardPerTokenPerEpochSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorNotRewardedBecauseAlias` event
        pub fn validator_not_rewarded_because_alias_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorNotRewardedBecauseAliasFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorRewarded` event
        pub fn validator_rewarded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorRewardedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorTokensPenalized` event
        pub fn validator_tokens_penalized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorTokensPenalizedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdrawn` event
        pub fn withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawnFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakingBalancesEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StakingBalances<M> {
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
    ///Custom Error type `AliasNotOwnedBySender` with signature `AliasNotOwnedBySender(address,address)` and selector `0xb1b82d79`
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
        name = "AliasNotOwnedBySender",
        abi = "AliasNotOwnedBySender(address,address)"
    )]
    pub struct AliasNotOwnedBySender {
        pub alias_account: ::ethers::core::types::Address,
        pub staker_address: ::ethers::core::types::Address,
    }
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
    ///Custom Error type `CannotRemoveAliasOfActiveValidator` with signature `CannotRemoveAliasOfActiveValidator(address)` and selector `0xc400065d`
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
        name = "CannotRemoveAliasOfActiveValidator",
        abi = "CannotRemoveAliasOfActiveValidator(address)"
    )]
    pub struct CannotRemoveAliasOfActiveValidator {
        pub alias_account: ::ethers::core::types::Address,
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
    ///Custom Error type `MaxAliasCountReached` with signature `MaxAliasCountReached(uint256)` and selector `0x2176682c`
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
    #[etherror(name = "MaxAliasCountReached", abi = "MaxAliasCountReached(uint256)")]
    pub struct MaxAliasCountReached {
        pub alias_count: ::ethers::core::types::U256,
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
    ///Custom Error type `OnlyStakingContract` with signature `OnlyStakingContract(address)` and selector `0xc95e6e49`
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
    #[etherror(name = "OnlyStakingContract", abi = "OnlyStakingContract(address)")]
    pub struct OnlyStakingContract {
        pub sender: ::ethers::core::types::Address,
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
    ///Custom Error type `StakeMustBeGreaterThanMinimumStake` with signature `StakeMustBeGreaterThanMinimumStake(uint256,uint256)` and selector `0x4806a505`
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
        name = "StakeMustBeGreaterThanMinimumStake",
        abi = "StakeMustBeGreaterThanMinimumStake(uint256,uint256)"
    )]
    pub struct StakeMustBeGreaterThanMinimumStake {
        pub amount_staked: ::ethers::core::types::U256,
        pub minimum_stake: ::ethers::core::types::U256,
    }
    ///Custom Error type `StakeMustBeLessThanMaximumStake` with signature `StakeMustBeLessThanMaximumStake(uint256,uint256)` and selector `0x2b3c6b63`
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
        name = "StakeMustBeLessThanMaximumStake",
        abi = "StakeMustBeLessThanMaximumStake(uint256,uint256)"
    )]
    pub struct StakeMustBeLessThanMaximumStake {
        pub amount_staked: ::ethers::core::types::U256,
        pub maximum_stake: ::ethers::core::types::U256,
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
    pub enum StakingBalancesErrors {
        ActiveValidatorsCannotLeave(ActiveValidatorsCannotLeave),
        AliasNotOwnedBySender(AliasNotOwnedBySender),
        CallerNotOwner(CallerNotOwner),
        CannotAddFunctionToDiamondThatAlreadyExists(
            CannotAddFunctionToDiamondThatAlreadyExists,
        ),
        CannotAddSelectorsToZeroAddress(CannotAddSelectorsToZeroAddress),
        CannotRemoveAliasOfActiveValidator(CannotRemoveAliasOfActiveValidator),
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
        CannotStakeZero(CannotStakeZero),
        CannotWithdrawZero(CannotWithdrawZero),
        IncorrectFacetCutAction(IncorrectFacetCutAction),
        InitializationFunctionReverted(InitializationFunctionReverted),
        MaxAliasCountReached(MaxAliasCountReached),
        NoBytecodeAtAddress(NoBytecodeAtAddress),
        NoSelectorsProvidedForFacetForCut(NoSelectorsProvidedForFacetForCut),
        NotContractOwner(NotContractOwner),
        OnlyStakingContract(OnlyStakingContract),
        RemoveFacetAddressMustBeZeroAddress(RemoveFacetAddressMustBeZeroAddress),
        StakeMustBeGreaterThanMinimumStake(StakeMustBeGreaterThanMinimumStake),
        StakeMustBeLessThanMaximumStake(StakeMustBeLessThanMaximumStake),
        StakerNotPermitted(StakerNotPermitted),
        TryingToWithdrawMoreThanStaked(TryingToWithdrawMoreThanStaked),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for StakingBalancesErrors {
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
                = <AliasNotOwnedBySender as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AliasNotOwnedBySender(decoded));
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
                = <CannotRemoveAliasOfActiveValidator as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CannotRemoveAliasOfActiveValidator(decoded));
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
                = <CannotStakeZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotStakeZero(decoded));
            }
            if let Ok(decoded)
                = <CannotWithdrawZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CannotWithdrawZero(decoded));
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
                = <MaxAliasCountReached as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxAliasCountReached(decoded));
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
                = <OnlyStakingContract as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyStakingContract(decoded));
            }
            if let Ok(decoded)
                = <RemoveFacetAddressMustBeZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveFacetAddressMustBeZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <StakeMustBeGreaterThanMinimumStake as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::StakeMustBeGreaterThanMinimumStake(decoded));
            }
            if let Ok(decoded)
                = <StakeMustBeLessThanMaximumStake as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::StakeMustBeLessThanMaximumStake(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakingBalancesErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActiveValidatorsCannotLeave(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AliasNotOwnedBySender(element) => {
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
                Self::CannotRemoveAliasOfActiveValidator(element) => {
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
                Self::CannotStakeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotWithdrawZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxAliasCountReached(element) => {
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
                Self::OnlyStakingContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeMustBeGreaterThanMinimumStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeMustBeLessThanMaximumStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerNotPermitted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryingToWithdrawMoreThanStaked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for StakingBalancesErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ActiveValidatorsCannotLeave as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AliasNotOwnedBySender as ::ethers::contract::EthError>::selector() => {
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
                    == <CannotRemoveAliasOfActiveValidator as ::ethers::contract::EthError>::selector() => {
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
                    == <CannotStakeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotWithdrawZero as ::ethers::contract::EthError>::selector() => {
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
                    == <MaxAliasCountReached as ::ethers::contract::EthError>::selector() => {
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
                    == <OnlyStakingContract as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RemoveFacetAddressMustBeZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StakeMustBeGreaterThanMinimumStake as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StakeMustBeLessThanMaximumStake as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for StakingBalancesErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActiveValidatorsCannotLeave(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AliasNotOwnedBySender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveAliasOfActiveValidator(element) => {
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
                Self::CannotStakeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotWithdrawZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxAliasCountReached(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotContractOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyStakingContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeMustBeGreaterThanMinimumStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeMustBeLessThanMaximumStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerNotPermitted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TryingToWithdrawMoreThanStaked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for StakingBalancesErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ActiveValidatorsCannotLeave> for StakingBalancesErrors {
        fn from(value: ActiveValidatorsCannotLeave) -> Self {
            Self::ActiveValidatorsCannotLeave(value)
        }
    }
    impl ::core::convert::From<AliasNotOwnedBySender> for StakingBalancesErrors {
        fn from(value: AliasNotOwnedBySender) -> Self {
            Self::AliasNotOwnedBySender(value)
        }
    }
    impl ::core::convert::From<CallerNotOwner> for StakingBalancesErrors {
        fn from(value: CallerNotOwner) -> Self {
            Self::CallerNotOwner(value)
        }
    }
    impl ::core::convert::From<CannotAddFunctionToDiamondThatAlreadyExists>
    for StakingBalancesErrors {
        fn from(value: CannotAddFunctionToDiamondThatAlreadyExists) -> Self {
            Self::CannotAddFunctionToDiamondThatAlreadyExists(value)
        }
    }
    impl ::core::convert::From<CannotAddSelectorsToZeroAddress>
    for StakingBalancesErrors {
        fn from(value: CannotAddSelectorsToZeroAddress) -> Self {
            Self::CannotAddSelectorsToZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotRemoveAliasOfActiveValidator>
    for StakingBalancesErrors {
        fn from(value: CannotRemoveAliasOfActiveValidator) -> Self {
            Self::CannotRemoveAliasOfActiveValidator(value)
        }
    }
    impl ::core::convert::From<CannotRemoveFunctionThatDoesNotExist>
    for StakingBalancesErrors {
        fn from(value: CannotRemoveFunctionThatDoesNotExist) -> Self {
            Self::CannotRemoveFunctionThatDoesNotExist(value)
        }
    }
    impl ::core::convert::From<CannotRemoveImmutableFunction> for StakingBalancesErrors {
        fn from(value: CannotRemoveImmutableFunction) -> Self {
            Self::CannotRemoveImmutableFunction(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionThatDoesNotExists>
    for StakingBalancesErrors {
        fn from(value: CannotReplaceFunctionThatDoesNotExists) -> Self {
            Self::CannotReplaceFunctionThatDoesNotExists(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet>
    for StakingBalancesErrors {
        fn from(
            value: CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ) -> Self {
            Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionsFromFacetWithZeroAddress>
    for StakingBalancesErrors {
        fn from(value: CannotReplaceFunctionsFromFacetWithZeroAddress) -> Self {
            Self::CannotReplaceFunctionsFromFacetWithZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotReplaceImmutableFunction>
    for StakingBalancesErrors {
        fn from(value: CannotReplaceImmutableFunction) -> Self {
            Self::CannotReplaceImmutableFunction(value)
        }
    }
    impl ::core::convert::From<CannotStakeZero> for StakingBalancesErrors {
        fn from(value: CannotStakeZero) -> Self {
            Self::CannotStakeZero(value)
        }
    }
    impl ::core::convert::From<CannotWithdrawZero> for StakingBalancesErrors {
        fn from(value: CannotWithdrawZero) -> Self {
            Self::CannotWithdrawZero(value)
        }
    }
    impl ::core::convert::From<IncorrectFacetCutAction> for StakingBalancesErrors {
        fn from(value: IncorrectFacetCutAction) -> Self {
            Self::IncorrectFacetCutAction(value)
        }
    }
    impl ::core::convert::From<InitializationFunctionReverted>
    for StakingBalancesErrors {
        fn from(value: InitializationFunctionReverted) -> Self {
            Self::InitializationFunctionReverted(value)
        }
    }
    impl ::core::convert::From<MaxAliasCountReached> for StakingBalancesErrors {
        fn from(value: MaxAliasCountReached) -> Self {
            Self::MaxAliasCountReached(value)
        }
    }
    impl ::core::convert::From<NoBytecodeAtAddress> for StakingBalancesErrors {
        fn from(value: NoBytecodeAtAddress) -> Self {
            Self::NoBytecodeAtAddress(value)
        }
    }
    impl ::core::convert::From<NoSelectorsProvidedForFacetForCut>
    for StakingBalancesErrors {
        fn from(value: NoSelectorsProvidedForFacetForCut) -> Self {
            Self::NoSelectorsProvidedForFacetForCut(value)
        }
    }
    impl ::core::convert::From<NotContractOwner> for StakingBalancesErrors {
        fn from(value: NotContractOwner) -> Self {
            Self::NotContractOwner(value)
        }
    }
    impl ::core::convert::From<OnlyStakingContract> for StakingBalancesErrors {
        fn from(value: OnlyStakingContract) -> Self {
            Self::OnlyStakingContract(value)
        }
    }
    impl ::core::convert::From<RemoveFacetAddressMustBeZeroAddress>
    for StakingBalancesErrors {
        fn from(value: RemoveFacetAddressMustBeZeroAddress) -> Self {
            Self::RemoveFacetAddressMustBeZeroAddress(value)
        }
    }
    impl ::core::convert::From<StakeMustBeGreaterThanMinimumStake>
    for StakingBalancesErrors {
        fn from(value: StakeMustBeGreaterThanMinimumStake) -> Self {
            Self::StakeMustBeGreaterThanMinimumStake(value)
        }
    }
    impl ::core::convert::From<StakeMustBeLessThanMaximumStake>
    for StakingBalancesErrors {
        fn from(value: StakeMustBeLessThanMaximumStake) -> Self {
            Self::StakeMustBeLessThanMaximumStake(value)
        }
    }
    impl ::core::convert::From<StakerNotPermitted> for StakingBalancesErrors {
        fn from(value: StakerNotPermitted) -> Self {
            Self::StakerNotPermitted(value)
        }
    }
    impl ::core::convert::From<TryingToWithdrawMoreThanStaked>
    for StakingBalancesErrors {
        fn from(value: TryingToWithdrawMoreThanStaked) -> Self {
            Self::TryingToWithdrawMoreThanStaked(value)
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
    #[ethevent(name = "AliasAdded", abi = "AliasAdded(address,address)")]
    pub struct AliasAddedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        pub alias_account: ::ethers::core::types::Address,
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
    #[ethevent(name = "AliasRemoved", abi = "AliasRemoved(address,address)")]
    pub struct AliasRemovedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        pub alias_account: ::ethers::core::types::Address,
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
    #[ethevent(name = "MaxAliasCountSet", abi = "MaxAliasCountSet(uint256)")]
    pub struct MaxAliasCountSetFilter {
        pub new_max_alias_count: ::ethers::core::types::U256,
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
    #[ethevent(name = "MaximumStakeSet", abi = "MaximumStakeSet(uint256)")]
    pub struct MaximumStakeSetFilter {
        pub new_maximum_stake: ::ethers::core::types::U256,
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
    #[ethevent(name = "MinimumStakeSet", abi = "MinimumStakeSet(uint256)")]
    pub struct MinimumStakeSetFilter {
        pub new_minimum_stake: ::ethers::core::types::U256,
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
    #[ethevent(name = "PermittedStakerAdded", abi = "PermittedStakerAdded(address)")]
    pub struct PermittedStakerAddedFilter {
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
    #[ethevent(name = "PermittedStakerRemoved", abi = "PermittedStakerRemoved(address)")]
    pub struct PermittedStakerRemovedFilter {
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
        name = "PermittedStakersOnChanged",
        abi = "PermittedStakersOnChanged(bool)"
    )]
    pub struct PermittedStakersOnChangedFilter {
        pub permitted_stakers_on: bool,
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
    #[ethevent(name = "RewardPaid", abi = "RewardPaid(address,uint256)")]
    pub struct RewardPaidFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
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
    #[ethevent(name = "Staked", abi = "Staked(address,uint256)")]
    pub struct StakedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "TokenRewardPerTokenPerEpochSet",
        abi = "TokenRewardPerTokenPerEpochSet(uint256)"
    )]
    pub struct TokenRewardPerTokenPerEpochSetFilter {
        pub new_token_reward_per_token_per_epoch: ::ethers::core::types::U256,
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
        name = "ValidatorNotRewardedBecauseAlias",
        abi = "ValidatorNotRewardedBecauseAlias(address,address)"
    )]
    pub struct ValidatorNotRewardedBecauseAliasFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        pub alias_account: ::ethers::core::types::Address,
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
    #[ethevent(name = "ValidatorRewarded", abi = "ValidatorRewarded(address,uint256)")]
    pub struct ValidatorRewardedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "ValidatorTokensPenalized",
        abi = "ValidatorTokensPenalized(address,uint256)"
    )]
    pub struct ValidatorTokensPenalizedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
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
    #[ethevent(name = "Withdrawn", abi = "Withdrawn(address,uint256)")]
    pub struct WithdrawnFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum StakingBalancesEvents {
        AliasAddedFilter(AliasAddedFilter),
        AliasRemovedFilter(AliasRemovedFilter),
        DiamondCutFilter(DiamondCutFilter),
        MaxAliasCountSetFilter(MaxAliasCountSetFilter),
        MaximumStakeSetFilter(MaximumStakeSetFilter),
        MinimumStakeSetFilter(MinimumStakeSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PermittedStakerAddedFilter(PermittedStakerAddedFilter),
        PermittedStakerRemovedFilter(PermittedStakerRemovedFilter),
        PermittedStakersOnChangedFilter(PermittedStakersOnChangedFilter),
        ResolverContractAddressSetFilter(ResolverContractAddressSetFilter),
        RewardPaidFilter(RewardPaidFilter),
        StakedFilter(StakedFilter),
        TokenRewardPerTokenPerEpochSetFilter(TokenRewardPerTokenPerEpochSetFilter),
        ValidatorNotRewardedBecauseAliasFilter(ValidatorNotRewardedBecauseAliasFilter),
        ValidatorRewardedFilter(ValidatorRewardedFilter),
        ValidatorTokensPenalizedFilter(ValidatorTokensPenalizedFilter),
        WithdrawnFilter(WithdrawnFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakingBalancesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AliasAddedFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::AliasAddedFilter(decoded));
            }
            if let Ok(decoded) = AliasRemovedFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::AliasRemovedFilter(decoded));
            }
            if let Ok(decoded) = DiamondCutFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::DiamondCutFilter(decoded));
            }
            if let Ok(decoded) = MaxAliasCountSetFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::MaxAliasCountSetFilter(decoded));
            }
            if let Ok(decoded) = MaximumStakeSetFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::MaximumStakeSetFilter(decoded));
            }
            if let Ok(decoded) = MinimumStakeSetFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::MinimumStakeSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PermittedStakerAddedFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::PermittedStakerAddedFilter(decoded));
            }
            if let Ok(decoded) = PermittedStakerRemovedFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::PermittedStakerRemovedFilter(decoded));
            }
            if let Ok(decoded) = PermittedStakersOnChangedFilter::decode_log(log) {
                return Ok(
                    StakingBalancesEvents::PermittedStakersOnChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ResolverContractAddressSetFilter::decode_log(log) {
                return Ok(
                    StakingBalancesEvents::ResolverContractAddressSetFilter(decoded),
                );
            }
            if let Ok(decoded) = RewardPaidFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::RewardPaidFilter(decoded));
            }
            if let Ok(decoded) = StakedFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::StakedFilter(decoded));
            }
            if let Ok(decoded) = TokenRewardPerTokenPerEpochSetFilter::decode_log(log) {
                return Ok(
                    StakingBalancesEvents::TokenRewardPerTokenPerEpochSetFilter(decoded),
                );
            }
            if let Ok(decoded)
                = ValidatorNotRewardedBecauseAliasFilter::decode_log(log) {
                return Ok(
                    StakingBalancesEvents::ValidatorNotRewardedBecauseAliasFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ValidatorRewardedFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::ValidatorRewardedFilter(decoded));
            }
            if let Ok(decoded) = ValidatorTokensPenalizedFilter::decode_log(log) {
                return Ok(
                    StakingBalancesEvents::ValidatorTokensPenalizedFilter(decoded),
                );
            }
            if let Ok(decoded) = WithdrawnFilter::decode_log(log) {
                return Ok(StakingBalancesEvents::WithdrawnFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakingBalancesEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AliasAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AliasRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondCutFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxAliasCountSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaximumStakeSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinimumStakeSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermittedStakerAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermittedStakerRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermittedStakersOnChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResolverContractAddressSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardPaidFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenRewardPerTokenPerEpochSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorNotRewardedBecauseAliasFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorRewardedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorTokensPenalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AliasAddedFilter> for StakingBalancesEvents {
        fn from(value: AliasAddedFilter) -> Self {
            Self::AliasAddedFilter(value)
        }
    }
    impl ::core::convert::From<AliasRemovedFilter> for StakingBalancesEvents {
        fn from(value: AliasRemovedFilter) -> Self {
            Self::AliasRemovedFilter(value)
        }
    }
    impl ::core::convert::From<DiamondCutFilter> for StakingBalancesEvents {
        fn from(value: DiamondCutFilter) -> Self {
            Self::DiamondCutFilter(value)
        }
    }
    impl ::core::convert::From<MaxAliasCountSetFilter> for StakingBalancesEvents {
        fn from(value: MaxAliasCountSetFilter) -> Self {
            Self::MaxAliasCountSetFilter(value)
        }
    }
    impl ::core::convert::From<MaximumStakeSetFilter> for StakingBalancesEvents {
        fn from(value: MaximumStakeSetFilter) -> Self {
            Self::MaximumStakeSetFilter(value)
        }
    }
    impl ::core::convert::From<MinimumStakeSetFilter> for StakingBalancesEvents {
        fn from(value: MinimumStakeSetFilter) -> Self {
            Self::MinimumStakeSetFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for StakingBalancesEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PermittedStakerAddedFilter> for StakingBalancesEvents {
        fn from(value: PermittedStakerAddedFilter) -> Self {
            Self::PermittedStakerAddedFilter(value)
        }
    }
    impl ::core::convert::From<PermittedStakerRemovedFilter> for StakingBalancesEvents {
        fn from(value: PermittedStakerRemovedFilter) -> Self {
            Self::PermittedStakerRemovedFilter(value)
        }
    }
    impl ::core::convert::From<PermittedStakersOnChangedFilter>
    for StakingBalancesEvents {
        fn from(value: PermittedStakersOnChangedFilter) -> Self {
            Self::PermittedStakersOnChangedFilter(value)
        }
    }
    impl ::core::convert::From<ResolverContractAddressSetFilter>
    for StakingBalancesEvents {
        fn from(value: ResolverContractAddressSetFilter) -> Self {
            Self::ResolverContractAddressSetFilter(value)
        }
    }
    impl ::core::convert::From<RewardPaidFilter> for StakingBalancesEvents {
        fn from(value: RewardPaidFilter) -> Self {
            Self::RewardPaidFilter(value)
        }
    }
    impl ::core::convert::From<StakedFilter> for StakingBalancesEvents {
        fn from(value: StakedFilter) -> Self {
            Self::StakedFilter(value)
        }
    }
    impl ::core::convert::From<TokenRewardPerTokenPerEpochSetFilter>
    for StakingBalancesEvents {
        fn from(value: TokenRewardPerTokenPerEpochSetFilter) -> Self {
            Self::TokenRewardPerTokenPerEpochSetFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorNotRewardedBecauseAliasFilter>
    for StakingBalancesEvents {
        fn from(value: ValidatorNotRewardedBecauseAliasFilter) -> Self {
            Self::ValidatorNotRewardedBecauseAliasFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorRewardedFilter> for StakingBalancesEvents {
        fn from(value: ValidatorRewardedFilter) -> Self {
            Self::ValidatorRewardedFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorTokensPenalizedFilter>
    for StakingBalancesEvents {
        fn from(value: ValidatorTokensPenalizedFilter) -> Self {
            Self::ValidatorTokensPenalizedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawnFilter> for StakingBalancesEvents {
        fn from(value: WithdrawnFilter) -> Self {
            Self::WithdrawnFilter(value)
        }
    }
    ///Container type for all input parameters for the `addAlias` function with signature `addAlias(address)` and selector `0x40b96482`
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
    #[ethcall(name = "addAlias", abi = "addAlias(address)")]
    pub struct AddAliasCall {
        pub alias_account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addPermittedStaker` function with signature `addPermittedStaker(address)` and selector `0x74c2e2fc`
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
    #[ethcall(name = "addPermittedStaker", abi = "addPermittedStaker(address)")]
    pub struct AddPermittedStakerCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addPermittedStakers` function with signature `addPermittedStakers(address[])` and selector `0x1dc4ab3c`
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
    #[ethcall(name = "addPermittedStakers", abi = "addPermittedStakers(address[])")]
    pub struct AddPermittedStakersCall {
        pub stakers: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `checkStakingAmounts` function with signature `checkStakingAmounts(address)` and selector `0x49919966`
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
    #[ethcall(name = "checkStakingAmounts", abi = "checkStakingAmounts(address)")]
    pub struct CheckStakingAmountsCall {
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `getReward` function with signature `getReward(address)` and selector `0xc00007b0`
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
    #[ethcall(name = "getReward", abi = "getReward(address)")]
    pub struct GetRewardCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getStakingAddress` function with signature `getStakingAddress()` and selector `0x0e9ed68b`
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
    #[ethcall(name = "getStakingAddress", abi = "getStakingAddress()")]
    pub struct GetStakingAddressCall;
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
    ///Container type for all input parameters for the `isPermittedStaker` function with signature `isPermittedStaker(address)` and selector `0xd3dbad7d`
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
    #[ethcall(name = "isPermittedStaker", abi = "isPermittedStaker(address)")]
    pub struct IsPermittedStakerCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maximumStake` function with signature `maximumStake()` and selector `0xb5c6b453`
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
    #[ethcall(name = "maximumStake", abi = "maximumStake()")]
    pub struct MaximumStakeCall;
    ///Container type for all input parameters for the `minimumStake` function with signature `minimumStake()` and selector `0xec5ffac2`
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
    #[ethcall(name = "minimumStake", abi = "minimumStake()")]
    pub struct MinimumStakeCall;
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
    ///Container type for all input parameters for the `penalizeTokens` function with signature `penalizeTokens(uint256,address)` and selector `0x0a0e3dea`
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
    #[ethcall(name = "penalizeTokens", abi = "penalizeTokens(uint256,address)")]
    pub struct PenalizeTokensCall {
        pub amount: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `permittedStakersOn` function with signature `permittedStakersOn()` and selector `0x27a199d0`
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
    #[ethcall(name = "permittedStakersOn", abi = "permittedStakersOn()")]
    pub struct PermittedStakersOnCall;
    ///Container type for all input parameters for the `removeAlias` function with signature `removeAlias(address)` and selector `0xbc979787`
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
    #[ethcall(name = "removeAlias", abi = "removeAlias(address)")]
    pub struct RemoveAliasCall {
        pub alias_account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removePermittedStaker` function with signature `removePermittedStaker(address)` and selector `0xa498072c`
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
    #[ethcall(name = "removePermittedStaker", abi = "removePermittedStaker(address)")]
    pub struct RemovePermittedStakerCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `restakePenaltyTokens` function with signature `restakePenaltyTokens(address,uint256)` and selector `0xf21aec06`
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
        name = "restakePenaltyTokens",
        abi = "restakePenaltyTokens(address,uint256)"
    )]
    pub struct RestakePenaltyTokensCall {
        pub staker: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rewardOf` function with signature `rewardOf(address)` and selector `0x1d62ebd9`
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
    #[ethcall(name = "rewardOf", abi = "rewardOf(address)")]
    pub struct RewardOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rewardValidator` function with signature `rewardValidator(uint256,address)` and selector `0x02fa04c9`
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
    #[ethcall(name = "rewardValidator", abi = "rewardValidator(uint256,address)")]
    pub struct RewardValidatorCall {
        pub amount: ::ethers::core::types::U256,
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
        pub new_resolver_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setMaxAliasCount` function with signature `setMaxAliasCount(uint256)` and selector `0xddb6e6e1`
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
    #[ethcall(name = "setMaxAliasCount", abi = "setMaxAliasCount(uint256)")]
    pub struct SetMaxAliasCountCall {
        pub new_max_alias_count: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMaximumStake` function with signature `setMaximumStake(uint256)` and selector `0xbcb5c636`
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
    #[ethcall(name = "setMaximumStake", abi = "setMaximumStake(uint256)")]
    pub struct SetMaximumStakeCall {
        pub new_maximum_stake: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMinimumStake` function with signature `setMinimumStake(uint256)` and selector `0x233e9903`
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
    #[ethcall(name = "setMinimumStake", abi = "setMinimumStake(uint256)")]
    pub struct SetMinimumStakeCall {
        pub new_minimum_stake: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPermittedStakersOn` function with signature `setPermittedStakersOn(bool)` and selector `0x842648c2`
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
    #[ethcall(name = "setPermittedStakersOn", abi = "setPermittedStakersOn(bool)")]
    pub struct SetPermittedStakersOnCall {
        pub permitted: bool,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(uint256,address)` and selector `0x7acb7757`
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
    #[ethcall(name = "stake", abi = "stake(uint256,address)")]
    pub struct StakeCall {
        pub amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `totalStaked` function with signature `totalStaked()` and selector `0x817b1cd2`
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
    #[ethcall(name = "totalStaked", abi = "totalStaked()")]
    pub struct TotalStakedCall;
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
    ///Container type for all input parameters for the `transferPenaltyTokens` function with signature `transferPenaltyTokens(uint256,address)` and selector `0x3560db3c`
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
        name = "transferPenaltyTokens",
        abi = "transferPenaltyTokens(uint256,address)"
    )]
    pub struct TransferPenaltyTokensCall {
        pub balance: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address)` and selector `0x00f714ce`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address)")]
    pub struct WithdrawWithAmountAndAccountCall {
        pub amount: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw()` and selector `0x3ccfd60b`
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
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    ///Container type for all input parameters for the `withdrawPenaltyTokens` function with signature `withdrawPenaltyTokens(uint256)` and selector `0xdc157ce2`
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
    #[ethcall(name = "withdrawPenaltyTokens", abi = "withdrawPenaltyTokens(uint256)")]
    pub struct WithdrawPenaltyTokensCall {
        pub balance: ::ethers::core::types::U256,
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
    pub enum StakingBalancesCalls {
        AddAlias(AddAliasCall),
        AddPermittedStaker(AddPermittedStakerCall),
        AddPermittedStakers(AddPermittedStakersCall),
        BalanceOf(BalanceOfCall),
        CheckStakingAmounts(CheckStakingAmountsCall),
        ContractResolver(ContractResolverCall),
        DiamondCut(DiamondCutCall),
        FacetAddress(FacetAddressCall),
        FacetAddresses(FacetAddressesCall),
        FacetFunctionSelectors(FacetFunctionSelectorsCall),
        Facets(FacetsCall),
        GetReward(GetRewardCall),
        GetStakingAddress(GetStakingAddressCall),
        GetTokenAddress(GetTokenAddressCall),
        IsPermittedStaker(IsPermittedStakerCall),
        MaximumStake(MaximumStakeCall),
        MinimumStake(MinimumStakeCall),
        Owner(OwnerCall),
        PenalizeTokens(PenalizeTokensCall),
        PermittedStakersOn(PermittedStakersOnCall),
        RemoveAlias(RemoveAliasCall),
        RemovePermittedStaker(RemovePermittedStakerCall),
        RestakePenaltyTokens(RestakePenaltyTokensCall),
        RewardOf(RewardOfCall),
        RewardValidator(RewardValidatorCall),
        SetContractResolver(SetContractResolverCall),
        SetMaxAliasCount(SetMaxAliasCountCall),
        SetMaximumStake(SetMaximumStakeCall),
        SetMinimumStake(SetMinimumStakeCall),
        SetPermittedStakersOn(SetPermittedStakersOnCall),
        Stake(StakeCall),
        SupportsInterface(SupportsInterfaceCall),
        TotalStaked(TotalStakedCall),
        TransferOwnership(TransferOwnershipCall),
        TransferPenaltyTokens(TransferPenaltyTokensCall),
        WithdrawWithAmountAndAccount(WithdrawWithAmountAndAccountCall),
        Withdraw(WithdrawCall),
        WithdrawPenaltyTokens(WithdrawPenaltyTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakingBalancesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddAliasCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddAlias(decoded));
            }
            if let Ok(decoded)
                = <AddPermittedStakerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddPermittedStaker(decoded));
            }
            if let Ok(decoded)
                = <AddPermittedStakersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddPermittedStakers(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <CheckStakingAmountsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckStakingAmounts(decoded));
            }
            if let Ok(decoded)
                = <ContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ContractResolver(decoded));
            }
            if let Ok(decoded)
                = <DiamondCutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DiamondCut(decoded));
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
                = <GetRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReward(decoded));
            }
            if let Ok(decoded)
                = <GetStakingAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStakingAddress(decoded));
            }
            if let Ok(decoded)
                = <GetTokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTokenAddress(decoded));
            }
            if let Ok(decoded)
                = <IsPermittedStakerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsPermittedStaker(decoded));
            }
            if let Ok(decoded)
                = <MaximumStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaximumStake(decoded));
            }
            if let Ok(decoded)
                = <MinimumStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinimumStake(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PenalizeTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PenalizeTokens(decoded));
            }
            if let Ok(decoded)
                = <PermittedStakersOnCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PermittedStakersOn(decoded));
            }
            if let Ok(decoded)
                = <RemoveAliasCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveAlias(decoded));
            }
            if let Ok(decoded)
                = <RemovePermittedStakerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemovePermittedStaker(decoded));
            }
            if let Ok(decoded)
                = <RestakePenaltyTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RestakePenaltyTokens(decoded));
            }
            if let Ok(decoded)
                = <RewardOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardOf(decoded));
            }
            if let Ok(decoded)
                = <RewardValidatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardValidator(decoded));
            }
            if let Ok(decoded)
                = <SetContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetContractResolver(decoded));
            }
            if let Ok(decoded)
                = <SetMaxAliasCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMaxAliasCount(decoded));
            }
            if let Ok(decoded)
                = <SetMaximumStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMaximumStake(decoded));
            }
            if let Ok(decoded)
                = <SetMinimumStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinimumStake(decoded));
            }
            if let Ok(decoded)
                = <SetPermittedStakersOnCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPermittedStakersOn(decoded));
            }
            if let Ok(decoded)
                = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TotalStakedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalStaked(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <TransferPenaltyTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferPenaltyTokens(decoded));
            }
            if let Ok(decoded)
                = <WithdrawWithAmountAndAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawWithAmountAndAccount(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded)
                = <WithdrawPenaltyTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawPenaltyTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakingBalancesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAlias(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPermittedStaker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPermittedStakers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckStakingAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiamondCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::GetReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakingAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPermittedStaker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaximumStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PenalizeTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermittedStakersOn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAlias(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePermittedStaker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RestakePenaltyTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxAliasCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaximumStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMinimumStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPermittedStakersOn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalStaked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferPenaltyTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawWithAmountAndAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawPenaltyTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StakingBalancesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAlias(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddPermittedStaker(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddPermittedStakers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckStakingAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::DiamondCut(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetFunctionSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Facets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakingAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPermittedStaker(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaximumStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PenalizeTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermittedStakersOn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveAlias(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePermittedStaker(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RestakePenaltyTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractResolver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxAliasCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMaximumStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinimumStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPermittedStakersOn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalStaked(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferPenaltyTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawWithAmountAndAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawPenaltyTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddAliasCall> for StakingBalancesCalls {
        fn from(value: AddAliasCall) -> Self {
            Self::AddAlias(value)
        }
    }
    impl ::core::convert::From<AddPermittedStakerCall> for StakingBalancesCalls {
        fn from(value: AddPermittedStakerCall) -> Self {
            Self::AddPermittedStaker(value)
        }
    }
    impl ::core::convert::From<AddPermittedStakersCall> for StakingBalancesCalls {
        fn from(value: AddPermittedStakersCall) -> Self {
            Self::AddPermittedStakers(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for StakingBalancesCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<CheckStakingAmountsCall> for StakingBalancesCalls {
        fn from(value: CheckStakingAmountsCall) -> Self {
            Self::CheckStakingAmounts(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for StakingBalancesCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<DiamondCutCall> for StakingBalancesCalls {
        fn from(value: DiamondCutCall) -> Self {
            Self::DiamondCut(value)
        }
    }
    impl ::core::convert::From<FacetAddressCall> for StakingBalancesCalls {
        fn from(value: FacetAddressCall) -> Self {
            Self::FacetAddress(value)
        }
    }
    impl ::core::convert::From<FacetAddressesCall> for StakingBalancesCalls {
        fn from(value: FacetAddressesCall) -> Self {
            Self::FacetAddresses(value)
        }
    }
    impl ::core::convert::From<FacetFunctionSelectorsCall> for StakingBalancesCalls {
        fn from(value: FacetFunctionSelectorsCall) -> Self {
            Self::FacetFunctionSelectors(value)
        }
    }
    impl ::core::convert::From<FacetsCall> for StakingBalancesCalls {
        fn from(value: FacetsCall) -> Self {
            Self::Facets(value)
        }
    }
    impl ::core::convert::From<GetRewardCall> for StakingBalancesCalls {
        fn from(value: GetRewardCall) -> Self {
            Self::GetReward(value)
        }
    }
    impl ::core::convert::From<GetStakingAddressCall> for StakingBalancesCalls {
        fn from(value: GetStakingAddressCall) -> Self {
            Self::GetStakingAddress(value)
        }
    }
    impl ::core::convert::From<GetTokenAddressCall> for StakingBalancesCalls {
        fn from(value: GetTokenAddressCall) -> Self {
            Self::GetTokenAddress(value)
        }
    }
    impl ::core::convert::From<IsPermittedStakerCall> for StakingBalancesCalls {
        fn from(value: IsPermittedStakerCall) -> Self {
            Self::IsPermittedStaker(value)
        }
    }
    impl ::core::convert::From<MaximumStakeCall> for StakingBalancesCalls {
        fn from(value: MaximumStakeCall) -> Self {
            Self::MaximumStake(value)
        }
    }
    impl ::core::convert::From<MinimumStakeCall> for StakingBalancesCalls {
        fn from(value: MinimumStakeCall) -> Self {
            Self::MinimumStake(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for StakingBalancesCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PenalizeTokensCall> for StakingBalancesCalls {
        fn from(value: PenalizeTokensCall) -> Self {
            Self::PenalizeTokens(value)
        }
    }
    impl ::core::convert::From<PermittedStakersOnCall> for StakingBalancesCalls {
        fn from(value: PermittedStakersOnCall) -> Self {
            Self::PermittedStakersOn(value)
        }
    }
    impl ::core::convert::From<RemoveAliasCall> for StakingBalancesCalls {
        fn from(value: RemoveAliasCall) -> Self {
            Self::RemoveAlias(value)
        }
    }
    impl ::core::convert::From<RemovePermittedStakerCall> for StakingBalancesCalls {
        fn from(value: RemovePermittedStakerCall) -> Self {
            Self::RemovePermittedStaker(value)
        }
    }
    impl ::core::convert::From<RestakePenaltyTokensCall> for StakingBalancesCalls {
        fn from(value: RestakePenaltyTokensCall) -> Self {
            Self::RestakePenaltyTokens(value)
        }
    }
    impl ::core::convert::From<RewardOfCall> for StakingBalancesCalls {
        fn from(value: RewardOfCall) -> Self {
            Self::RewardOf(value)
        }
    }
    impl ::core::convert::From<RewardValidatorCall> for StakingBalancesCalls {
        fn from(value: RewardValidatorCall) -> Self {
            Self::RewardValidator(value)
        }
    }
    impl ::core::convert::From<SetContractResolverCall> for StakingBalancesCalls {
        fn from(value: SetContractResolverCall) -> Self {
            Self::SetContractResolver(value)
        }
    }
    impl ::core::convert::From<SetMaxAliasCountCall> for StakingBalancesCalls {
        fn from(value: SetMaxAliasCountCall) -> Self {
            Self::SetMaxAliasCount(value)
        }
    }
    impl ::core::convert::From<SetMaximumStakeCall> for StakingBalancesCalls {
        fn from(value: SetMaximumStakeCall) -> Self {
            Self::SetMaximumStake(value)
        }
    }
    impl ::core::convert::From<SetMinimumStakeCall> for StakingBalancesCalls {
        fn from(value: SetMinimumStakeCall) -> Self {
            Self::SetMinimumStake(value)
        }
    }
    impl ::core::convert::From<SetPermittedStakersOnCall> for StakingBalancesCalls {
        fn from(value: SetPermittedStakersOnCall) -> Self {
            Self::SetPermittedStakersOn(value)
        }
    }
    impl ::core::convert::From<StakeCall> for StakingBalancesCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for StakingBalancesCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TotalStakedCall> for StakingBalancesCalls {
        fn from(value: TotalStakedCall) -> Self {
            Self::TotalStaked(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for StakingBalancesCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TransferPenaltyTokensCall> for StakingBalancesCalls {
        fn from(value: TransferPenaltyTokensCall) -> Self {
            Self::TransferPenaltyTokens(value)
        }
    }
    impl ::core::convert::From<WithdrawWithAmountAndAccountCall>
    for StakingBalancesCalls {
        fn from(value: WithdrawWithAmountAndAccountCall) -> Self {
            Self::WithdrawWithAmountAndAccount(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for StakingBalancesCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawPenaltyTokensCall> for StakingBalancesCalls {
        fn from(value: WithdrawPenaltyTokensCall) -> Self {
            Self::WithdrawPenaltyTokens(value)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `checkStakingAmounts` function with signature `checkStakingAmounts(address)` and selector `0x49919966`
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
    pub struct CheckStakingAmountsReturn(pub bool);
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
    ///Container type for all return fields from the `getStakingAddress` function with signature `getStakingAddress()` and selector `0x0e9ed68b`
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
    pub struct GetStakingAddressReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `isPermittedStaker` function with signature `isPermittedStaker(address)` and selector `0xd3dbad7d`
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
    pub struct IsPermittedStakerReturn(pub bool);
    ///Container type for all return fields from the `maximumStake` function with signature `maximumStake()` and selector `0xb5c6b453`
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
    pub struct MaximumStakeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minimumStake` function with signature `minimumStake()` and selector `0xec5ffac2`
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
    pub struct MinimumStakeReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `permittedStakersOn` function with signature `permittedStakersOn()` and selector `0x27a199d0`
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
    pub struct PermittedStakersOnReturn(pub bool);
    ///Container type for all return fields from the `rewardOf` function with signature `rewardOf(address)` and selector `0x1d62ebd9`
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
    pub struct RewardOfReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `totalStaked` function with signature `totalStaked()` and selector `0x817b1cd2`
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
    pub struct TotalStakedReturn(pub ::ethers::core::types::U256);
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
}
