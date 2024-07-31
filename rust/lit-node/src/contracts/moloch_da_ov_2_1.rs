pub use moloch_da_ov_2_1::*;
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
pub mod moloch_da_ov_2_1 {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/MolochDAOv2_1.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ESCROW"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ESCROW"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GUILD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("GUILD"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TOTAL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TOTAL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approvedTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvedTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("canRagequit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("canRagequit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "highestIndexYesVote",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cancelProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cancelProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collectTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("collectTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dilutionBound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dilutionBound"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCurrentPeriod"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMemberProposalVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMemberProposalVote",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("memberAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalFlags"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProposalFlags"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                        6usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalQueueLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProposalQueueLength",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenCount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserTokenBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUserTokenBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gracePeriodLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gracePeriodLength"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasVotingPeriodExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hasVotingPeriodExpired",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("startingPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_summoner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_approvedTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_periodDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_votingPeriodLength",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_gracePeriodLength",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proposalDeposit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dilutionBound"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_processingReward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_summonerShares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("memberAddressByDelegateKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "memberAddressByDelegateKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("memberList"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("memberList"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("members"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("members"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("loot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exists"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "highestIndexYesVote",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("jailed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("periodDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("periodDuration"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processGuildKickProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processGuildKickProposal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("processProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processWhitelistProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processWhitelistProposal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processingReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("processingReward"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalCount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalDeposit"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalQueue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalQueue"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("applicant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sponsor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sharesRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lootRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tributeOffered"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tributeToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("startingPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("yesVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("noVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("details"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "maxTotalSharesAndLootAtYesVote",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposedToKick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposedToKick"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposedToWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "proposedToWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ragekick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ragekick"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("memberToKick"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ragequit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ragequit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sharesToBurn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lootToBurn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sponsorProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sponsorProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitGuildKickProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "submitGuildKickProposal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("memberToKick"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("details"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("applicant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sharesRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lootRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tributeOffered"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tributeToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("details"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitVote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uintVote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitWhitelistProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "submitWhitelistProposal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenToWhitelist"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("details"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("summoningTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("summoningTime"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenWhitelist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalGuildBankTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "totalGuildBankTokens",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalLoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalLoot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalShares"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateDelegateKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateDelegateKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDelegateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("userTokenBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("userTokenBalances"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("votingPeriodLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votingPeriodLength"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawBalances"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::None,
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::None,
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CancelProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CancelProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("applicantAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProcessGuildKickProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProcessGuildKickProposal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("didPass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProcessProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProcessProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("didPass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProcessWhitelistProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProcessWhitelistProposal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("didPass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Ragequit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Ragequit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("memberAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sharesToBurn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lootToBurn"),
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
                    ::std::borrow::ToOwned::to_owned("SponsorProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SponsorProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delegateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("memberAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("startingPeriod"),
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
                    ::std::borrow::ToOwned::to_owned("SubmitProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SubmitProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("applicant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sharesRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lootRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tributeOffered"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tributeToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paymentRequested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("details"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("flags"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                        6usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delegateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("memberAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SubmitVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SubmitVote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delegateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("memberAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("uintVote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SummonComplete"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SummonComplete"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("summoner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("summoningTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("periodDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "votingPeriodLength",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gracePeriodLength"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalDeposit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dilutionBound"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("processingReward"),
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
                    ::std::borrow::ToOwned::to_owned("TokensCollected"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokensCollected"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountToCollect"),
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
                    ::std::borrow::ToOwned::to_owned("UpdateDelegateKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UpdateDelegateKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("memberAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDelegateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("memberAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOLOCHDAOV2_1_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct MolochDAOv2_1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MolochDAOv2_1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MolochDAOv2_1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MolochDAOv2_1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MolochDAOv2_1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MolochDAOv2_1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MolochDAOv2_1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOLOCHDAOV2_1_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `ESCROW` (0xe681c4aa) function
        pub fn escrow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([230, 129, 196, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `GUILD` (0xf5d54c77) function
        pub fn guild(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([245, 213, 76, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TOTAL` (0x27efc086) function
        pub fn total(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([39, 239, 192, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvedTokens` (0x1dafede0) function
        pub fn approved_tokens(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([29, 175, 237, 224], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `canRagequit` (0xa3dc3800) function
        pub fn can_ragequit(
            &self,
            highest_index_yes_vote: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([163, 220, 56, 0], highest_index_yes_vote)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelProposal` (0xe0a8f6f5) function
        pub fn cancel_proposal(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 168, 246, 245], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collectTokens` (0x59999b41) function
        pub fn collect_tokens(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 153, 155, 65], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositToken` (0xc89039c5) function
        pub fn deposit_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 144, 57, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dilutionBound` (0xafe5475f) function
        pub fn dilution_bound(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 229, 71, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentPeriod` (0x086146d2) function
        pub fn get_current_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([8, 97, 70, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMemberProposalVote` (0x044a0ca8) function
        pub fn get_member_proposal_vote(
            &self,
            member_address: ::ethers::core::types::Address,
            proposal_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([4, 74, 12, 168], (member_address, proposal_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalFlags` (0xb2643aab) function
        pub fn get_proposal_flags(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [bool; 6]> {
            self.0
                .method_hash([178, 100, 58, 171], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalQueueLength` (0x797daf70) function
        pub fn get_proposal_queue_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([121, 125, 175, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenCount` (0x78a89567) function
        pub fn get_token_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 168, 149, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserTokenBalance` (0x73f8fd4b) function
        pub fn get_user_token_balance(
            &self,
            user: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([115, 248, 253, 75], (user, token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gracePeriodLength` (0x63858f2d) function
        pub fn grace_period_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([99, 133, 143, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasVotingPeriodExpired` (0x9425a476) function
        pub fn has_voting_period_expired(
            &self,
            starting_period: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([148, 37, 164, 118], starting_period)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0xa9162619) function
        pub fn init(
            &self,
            summoner: ::std::vec::Vec<::ethers::core::types::Address>,
            approved_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            period_duration: ::ethers::core::types::U256,
            voting_period_length: ::ethers::core::types::U256,
            grace_period_length: ::ethers::core::types::U256,
            proposal_deposit: ::ethers::core::types::U256,
            dilution_bound: ::ethers::core::types::U256,
            processing_reward: ::ethers::core::types::U256,
            summoner_shares: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [169, 22, 38, 25],
                    (
                        summoner,
                        approved_tokens,
                        period_duration,
                        voting_period_length,
                        grace_period_length,
                        proposal_deposit,
                        dilution_bound,
                        processing_reward,
                        summoner_shares,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `memberAddressByDelegateKey` (0x402c1794) function
        pub fn member_address_by_delegate_key(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([64, 44, 23, 148], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `memberList` (0xb307fc6d) function
        pub fn member_list(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([179, 7, 252, 109], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `members` (0x08ae4b0c) function
        pub fn members(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([8, 174, 75, 12], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `periodDuration` (0xb470aade) function
        pub fn period_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 112, 170, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processGuildKickProposal` (0xe1a0e3fa) function
        pub fn process_guild_kick_proposal(
            &self,
            proposal_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 160, 227, 250], proposal_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processProposal` (0xe63bc62d) function
        pub fn process_proposal(
            &self,
            proposal_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 59, 198, 45], proposal_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processWhitelistProposal` (0x3793ab3c) function
        pub fn process_whitelist_proposal(
            &self,
            proposal_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 147, 171, 60], proposal_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processingReward` (0x03e32fa1) function
        pub fn processing_reward(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([3, 227, 47, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalCount` (0xda35c664) function
        pub fn proposal_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([218, 53, 198, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalDeposit` (0x8b15a605) function
        pub fn proposal_deposit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 21, 166, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalQueue` (0x3b214a74) function
        pub fn proposal_queue(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 33, 74, 116], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposals` (0x013cf08b) function
        pub fn proposals(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::std::string::String,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([1, 60, 240, 139], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposedToKick` (0x3fc24bba) function
        pub fn proposed_to_kick(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([63, 194, 75, 186], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposedToWhitelist` (0xe1780345) function
        pub fn proposed_to_whitelist(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([225, 120, 3, 69], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ragekick` (0xdfdd369e) function
        pub fn ragekick(
            &self,
            member_to_kick: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 221, 54, 158], member_to_kick)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ragequit` (0x15eb349e) function
        pub fn ragequit(
            &self,
            shares_to_burn: ::ethers::core::types::U256,
            loot_to_burn: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 235, 52, 158], (shares_to_burn, loot_to_burn))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sponsorProposal` (0x9746d940) function
        pub fn sponsor_proposal(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 70, 217, 64], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitGuildKickProposal` (0x115b2d18) function
        pub fn submit_guild_kick_proposal(
            &self,
            member_to_kick: ::ethers::core::types::Address,
            details: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([17, 91, 45, 24], (member_to_kick, details))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitProposal` (0x590f940b) function
        pub fn submit_proposal(
            &self,
            applicant: ::ethers::core::types::Address,
            shares_requested: ::ethers::core::types::U256,
            loot_requested: ::ethers::core::types::U256,
            tribute_offered: ::ethers::core::types::U256,
            tribute_token: ::ethers::core::types::Address,
            payment_requested: ::ethers::core::types::U256,
            payment_token: ::ethers::core::types::Address,
            details: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [89, 15, 148, 11],
                    (
                        applicant,
                        shares_requested,
                        loot_requested,
                        tribute_offered,
                        tribute_token,
                        payment_requested,
                        payment_token,
                        details,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitVote` (0x99653fbe) function
        pub fn submit_vote(
            &self,
            proposal_index: ::ethers::core::types::U256,
            uint_vote: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 101, 63, 190], (proposal_index, uint_vote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitWhitelistProposal` (0xfeb7ea1d) function
        pub fn submit_whitelist_proposal(
            &self,
            token_to_whitelist: ::ethers::core::types::Address,
            details: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 183, 234, 29], (token_to_whitelist, details))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `summoningTime` (0x7d5b6c72) function
        pub fn summoning_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 91, 108, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenWhitelist` (0x753d7563) function
        pub fn token_whitelist(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([117, 61, 117, 99], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalGuildBankTokens` (0x9d1722cb) function
        pub fn total_guild_bank_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([157, 23, 34, 203], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLoot` (0x635e99aa) function
        pub fn total_loot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([99, 94, 153, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalShares` (0x3a98ef39) function
        pub fn total_shares(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 152, 239, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDelegateKey` (0x2582bf2a) function
        pub fn update_delegate_key(
            &self,
            new_delegate_key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 130, 191, 42], new_delegate_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userTokenBalances` (0x45f2d105) function
        pub fn user_token_balances(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([69, 242, 209, 5], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingPeriodLength` (0x8340bbce) function
        pub fn voting_period_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 64, 187, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawBalance` (0x0cf20cc9) function
        pub fn withdraw_balance(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 242, 12, 201], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawBalances` (0x4482394b) function
        pub fn withdraw_balances(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            max: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 130, 57, 75], (tokens, amounts, max))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CancelProposal` event
        pub fn cancel_proposal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CancelProposalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProcessGuildKickProposal` event
        pub fn process_guild_kick_proposal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessGuildKickProposalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProcessProposal` event
        pub fn process_proposal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessProposalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProcessWhitelistProposal` event
        pub fn process_whitelist_proposal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessWhitelistProposalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Ragequit` event
        pub fn ragequit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RagequitFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SponsorProposal` event
        pub fn sponsor_proposal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SponsorProposalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SubmitProposal` event
        pub fn submit_proposal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SubmitProposalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SubmitVote` event
        pub fn submit_vote_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SubmitVoteFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SummonComplete` event
        pub fn summon_complete_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SummonCompleteFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokensCollected` event
        pub fn tokens_collected_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokensCollectedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UpdateDelegateKey` event
        pub fn update_delegate_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateDelegateKeyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MolochDAOv2_1Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MolochDAOv2_1<M> {
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
    #[ethevent(name = "CancelProposal", abi = "CancelProposal(uint256,address)")]
    pub struct CancelProposalFilter {
        #[ethevent(indexed)]
        pub proposal_id: ::ethers::core::types::U256,
        pub applicant_address: ::ethers::core::types::Address,
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
        name = "ProcessGuildKickProposal",
        abi = "ProcessGuildKickProposal(uint256,uint256,bool)"
    )]
    pub struct ProcessGuildKickProposalFilter {
        #[ethevent(indexed)]
        pub proposal_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub proposal_id: ::ethers::core::types::U256,
        pub did_pass: bool,
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
    #[ethevent(name = "ProcessProposal", abi = "ProcessProposal(uint256,uint256,bool)")]
    pub struct ProcessProposalFilter {
        #[ethevent(indexed)]
        pub proposal_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub proposal_id: ::ethers::core::types::U256,
        pub did_pass: bool,
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
        name = "ProcessWhitelistProposal",
        abi = "ProcessWhitelistProposal(uint256,uint256,bool)"
    )]
    pub struct ProcessWhitelistProposalFilter {
        #[ethevent(indexed)]
        pub proposal_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub proposal_id: ::ethers::core::types::U256,
        pub did_pass: bool,
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
    #[ethevent(name = "Ragequit", abi = "Ragequit(address,uint256,uint256)")]
    pub struct RagequitFilter {
        #[ethevent(indexed)]
        pub member_address: ::ethers::core::types::Address,
        pub shares_to_burn: ::ethers::core::types::U256,
        pub loot_to_burn: ::ethers::core::types::U256,
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
        name = "SponsorProposal",
        abi = "SponsorProposal(address,address,uint256,uint256,uint256)"
    )]
    pub struct SponsorProposalFilter {
        #[ethevent(indexed)]
        pub delegate_key: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub member_address: ::ethers::core::types::Address,
        pub proposal_id: ::ethers::core::types::U256,
        pub proposal_index: ::ethers::core::types::U256,
        pub starting_period: ::ethers::core::types::U256,
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
        name = "SubmitProposal",
        abi = "SubmitProposal(address,uint256,uint256,uint256,address,uint256,address,string,bool[6],uint256,address,address)"
    )]
    pub struct SubmitProposalFilter {
        #[ethevent(indexed)]
        pub applicant: ::ethers::core::types::Address,
        pub shares_requested: ::ethers::core::types::U256,
        pub loot_requested: ::ethers::core::types::U256,
        pub tribute_offered: ::ethers::core::types::U256,
        pub tribute_token: ::ethers::core::types::Address,
        pub payment_requested: ::ethers::core::types::U256,
        pub payment_token: ::ethers::core::types::Address,
        pub details: ::std::string::String,
        pub flags: [bool; 6],
        pub proposal_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub delegate_key: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub member_address: ::ethers::core::types::Address,
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
        name = "SubmitVote",
        abi = "SubmitVote(uint256,uint256,address,address,uint8)"
    )]
    pub struct SubmitVoteFilter {
        pub proposal_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub proposal_index: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub delegate_key: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub member_address: ::ethers::core::types::Address,
        pub uint_vote: u8,
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
        name = "SummonComplete",
        abi = "SummonComplete(address,address[],uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct SummonCompleteFilter {
        #[ethevent(indexed)]
        pub summoner: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub summoning_time: ::ethers::core::types::U256,
        pub period_duration: ::ethers::core::types::U256,
        pub voting_period_length: ::ethers::core::types::U256,
        pub grace_period_length: ::ethers::core::types::U256,
        pub proposal_deposit: ::ethers::core::types::U256,
        pub dilution_bound: ::ethers::core::types::U256,
        pub processing_reward: ::ethers::core::types::U256,
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
    #[ethevent(name = "TokensCollected", abi = "TokensCollected(address,uint256)")]
    pub struct TokensCollectedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount_to_collect: ::ethers::core::types::U256,
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
    #[ethevent(name = "UpdateDelegateKey", abi = "UpdateDelegateKey(address,address)")]
    pub struct UpdateDelegateKeyFilter {
        #[ethevent(indexed)]
        pub member_address: ::ethers::core::types::Address,
        pub new_delegate_key: ::ethers::core::types::Address,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub member_address: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
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
    pub enum MolochDAOv2_1Events {
        CancelProposalFilter(CancelProposalFilter),
        ProcessGuildKickProposalFilter(ProcessGuildKickProposalFilter),
        ProcessProposalFilter(ProcessProposalFilter),
        ProcessWhitelistProposalFilter(ProcessWhitelistProposalFilter),
        RagequitFilter(RagequitFilter),
        SponsorProposalFilter(SponsorProposalFilter),
        SubmitProposalFilter(SubmitProposalFilter),
        SubmitVoteFilter(SubmitVoteFilter),
        SummonCompleteFilter(SummonCompleteFilter),
        TokensCollectedFilter(TokensCollectedFilter),
        UpdateDelegateKeyFilter(UpdateDelegateKeyFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for MolochDAOv2_1Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CancelProposalFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::CancelProposalFilter(decoded));
            }
            if let Ok(decoded) = ProcessGuildKickProposalFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::ProcessGuildKickProposalFilter(decoded));
            }
            if let Ok(decoded) = ProcessProposalFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::ProcessProposalFilter(decoded));
            }
            if let Ok(decoded) = ProcessWhitelistProposalFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::ProcessWhitelistProposalFilter(decoded));
            }
            if let Ok(decoded) = RagequitFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::RagequitFilter(decoded));
            }
            if let Ok(decoded) = SponsorProposalFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::SponsorProposalFilter(decoded));
            }
            if let Ok(decoded) = SubmitProposalFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::SubmitProposalFilter(decoded));
            }
            if let Ok(decoded) = SubmitVoteFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::SubmitVoteFilter(decoded));
            }
            if let Ok(decoded) = SummonCompleteFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::SummonCompleteFilter(decoded));
            }
            if let Ok(decoded) = TokensCollectedFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::TokensCollectedFilter(decoded));
            }
            if let Ok(decoded) = UpdateDelegateKeyFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::UpdateDelegateKeyFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(MolochDAOv2_1Events::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MolochDAOv2_1Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancelProposalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessGuildKickProposalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessProposalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessWhitelistProposalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RagequitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SponsorProposalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitProposalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitVoteFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SummonCompleteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokensCollectedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateDelegateKeyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CancelProposalFilter> for MolochDAOv2_1Events {
        fn from(value: CancelProposalFilter) -> Self {
            Self::CancelProposalFilter(value)
        }
    }
    impl ::core::convert::From<ProcessGuildKickProposalFilter> for MolochDAOv2_1Events {
        fn from(value: ProcessGuildKickProposalFilter) -> Self {
            Self::ProcessGuildKickProposalFilter(value)
        }
    }
    impl ::core::convert::From<ProcessProposalFilter> for MolochDAOv2_1Events {
        fn from(value: ProcessProposalFilter) -> Self {
            Self::ProcessProposalFilter(value)
        }
    }
    impl ::core::convert::From<ProcessWhitelistProposalFilter> for MolochDAOv2_1Events {
        fn from(value: ProcessWhitelistProposalFilter) -> Self {
            Self::ProcessWhitelistProposalFilter(value)
        }
    }
    impl ::core::convert::From<RagequitFilter> for MolochDAOv2_1Events {
        fn from(value: RagequitFilter) -> Self {
            Self::RagequitFilter(value)
        }
    }
    impl ::core::convert::From<SponsorProposalFilter> for MolochDAOv2_1Events {
        fn from(value: SponsorProposalFilter) -> Self {
            Self::SponsorProposalFilter(value)
        }
    }
    impl ::core::convert::From<SubmitProposalFilter> for MolochDAOv2_1Events {
        fn from(value: SubmitProposalFilter) -> Self {
            Self::SubmitProposalFilter(value)
        }
    }
    impl ::core::convert::From<SubmitVoteFilter> for MolochDAOv2_1Events {
        fn from(value: SubmitVoteFilter) -> Self {
            Self::SubmitVoteFilter(value)
        }
    }
    impl ::core::convert::From<SummonCompleteFilter> for MolochDAOv2_1Events {
        fn from(value: SummonCompleteFilter) -> Self {
            Self::SummonCompleteFilter(value)
        }
    }
    impl ::core::convert::From<TokensCollectedFilter> for MolochDAOv2_1Events {
        fn from(value: TokensCollectedFilter) -> Self {
            Self::TokensCollectedFilter(value)
        }
    }
    impl ::core::convert::From<UpdateDelegateKeyFilter> for MolochDAOv2_1Events {
        fn from(value: UpdateDelegateKeyFilter) -> Self {
            Self::UpdateDelegateKeyFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for MolochDAOv2_1Events {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `ESCROW` function with signature `ESCROW()` and selector `0xe681c4aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ESCROW", abi = "ESCROW()")]
    pub struct EscrowCall;
    ///Container type for all input parameters for the `GUILD` function with signature `GUILD()` and selector `0xf5d54c77`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "GUILD", abi = "GUILD()")]
    pub struct GuildCall;
    ///Container type for all input parameters for the `TOTAL` function with signature `TOTAL()` and selector `0x27efc086`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "TOTAL", abi = "TOTAL()")]
    pub struct TotalCall;
    ///Container type for all input parameters for the `approvedTokens` function with signature `approvedTokens(uint256)` and selector `0x1dafede0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approvedTokens", abi = "approvedTokens(uint256)")]
    pub struct ApprovedTokensCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `canRagequit` function with signature `canRagequit(uint256)` and selector `0xa3dc3800`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "canRagequit", abi = "canRagequit(uint256)")]
    pub struct CanRagequitCall {
        pub highest_index_yes_vote: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `cancelProposal` function with signature `cancelProposal(uint256)` and selector `0xe0a8f6f5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cancelProposal", abi = "cancelProposal(uint256)")]
    pub struct CancelProposalCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `collectTokens` function with signature `collectTokens(address)` and selector `0x59999b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "collectTokens", abi = "collectTokens(address)")]
    pub struct CollectTokensCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `depositToken` function with signature `depositToken()` and selector `0xc89039c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "depositToken", abi = "depositToken()")]
    pub struct DepositTokenCall;
    ///Container type for all input parameters for the `dilutionBound` function with signature `dilutionBound()` and selector `0xafe5475f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "dilutionBound", abi = "dilutionBound()")]
    pub struct DilutionBoundCall;
    ///Container type for all input parameters for the `getCurrentPeriod` function with signature `getCurrentPeriod()` and selector `0x086146d2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCurrentPeriod", abi = "getCurrentPeriod()")]
    pub struct GetCurrentPeriodCall;
    ///Container type for all input parameters for the `getMemberProposalVote` function with signature `getMemberProposalVote(address,uint256)` and selector `0x044a0ca8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getMemberProposalVote",
        abi = "getMemberProposalVote(address,uint256)"
    )]
    pub struct GetMemberProposalVoteCall {
        pub member_address: ::ethers::core::types::Address,
        pub proposal_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getProposalFlags` function with signature `getProposalFlags(uint256)` and selector `0xb2643aab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getProposalFlags", abi = "getProposalFlags(uint256)")]
    pub struct GetProposalFlagsCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getProposalQueueLength` function with signature `getProposalQueueLength()` and selector `0x797daf70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getProposalQueueLength", abi = "getProposalQueueLength()")]
    pub struct GetProposalQueueLengthCall;
    ///Container type for all input parameters for the `getTokenCount` function with signature `getTokenCount()` and selector `0x78a89567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getTokenCount", abi = "getTokenCount()")]
    pub struct GetTokenCountCall;
    ///Container type for all input parameters for the `getUserTokenBalance` function with signature `getUserTokenBalance(address,address)` and selector `0x73f8fd4b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getUserTokenBalance",
        abi = "getUserTokenBalance(address,address)"
    )]
    pub struct GetUserTokenBalanceCall {
        pub user: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `gracePeriodLength` function with signature `gracePeriodLength()` and selector `0x63858f2d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gracePeriodLength", abi = "gracePeriodLength()")]
    pub struct GracePeriodLengthCall;
    ///Container type for all input parameters for the `hasVotingPeriodExpired` function with signature `hasVotingPeriodExpired(uint256)` and selector `0x9425a476`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasVotingPeriodExpired", abi = "hasVotingPeriodExpired(uint256)")]
    pub struct HasVotingPeriodExpiredCall {
        pub starting_period: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `init` function with signature `init(address[],address[],uint256,uint256,uint256,uint256,uint256,uint256,uint256[])` and selector `0xa9162619`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "init",
        abi = "init(address[],address[],uint256,uint256,uint256,uint256,uint256,uint256,uint256[])"
    )]
    pub struct InitCall {
        pub summoner: ::std::vec::Vec<::ethers::core::types::Address>,
        pub approved_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub period_duration: ::ethers::core::types::U256,
        pub voting_period_length: ::ethers::core::types::U256,
        pub grace_period_length: ::ethers::core::types::U256,
        pub proposal_deposit: ::ethers::core::types::U256,
        pub dilution_bound: ::ethers::core::types::U256,
        pub processing_reward: ::ethers::core::types::U256,
        pub summoner_shares: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `memberAddressByDelegateKey` function with signature `memberAddressByDelegateKey(address)` and selector `0x402c1794`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "memberAddressByDelegateKey",
        abi = "memberAddressByDelegateKey(address)"
    )]
    pub struct MemberAddressByDelegateKeyCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `memberList` function with signature `memberList(uint256)` and selector `0xb307fc6d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "memberList", abi = "memberList(uint256)")]
    pub struct MemberListCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `members` function with signature `members(address)` and selector `0x08ae4b0c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "members", abi = "members(address)")]
    pub struct MembersCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `periodDuration` function with signature `periodDuration()` and selector `0xb470aade`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "periodDuration", abi = "periodDuration()")]
    pub struct PeriodDurationCall;
    ///Container type for all input parameters for the `processGuildKickProposal` function with signature `processGuildKickProposal(uint256)` and selector `0xe1a0e3fa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "processGuildKickProposal",
        abi = "processGuildKickProposal(uint256)"
    )]
    pub struct ProcessGuildKickProposalCall {
        pub proposal_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `processProposal` function with signature `processProposal(uint256)` and selector `0xe63bc62d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "processProposal", abi = "processProposal(uint256)")]
    pub struct ProcessProposalCall {
        pub proposal_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `processWhitelistProposal` function with signature `processWhitelistProposal(uint256)` and selector `0x3793ab3c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "processWhitelistProposal",
        abi = "processWhitelistProposal(uint256)"
    )]
    pub struct ProcessWhitelistProposalCall {
        pub proposal_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `processingReward` function with signature `processingReward()` and selector `0x03e32fa1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "processingReward", abi = "processingReward()")]
    pub struct ProcessingRewardCall;
    ///Container type for all input parameters for the `proposalCount` function with signature `proposalCount()` and selector `0xda35c664`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalCount", abi = "proposalCount()")]
    pub struct ProposalCountCall;
    ///Container type for all input parameters for the `proposalDeposit` function with signature `proposalDeposit()` and selector `0x8b15a605`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalDeposit", abi = "proposalDeposit()")]
    pub struct ProposalDepositCall;
    ///Container type for all input parameters for the `proposalQueue` function with signature `proposalQueue(uint256)` and selector `0x3b214a74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalQueue", abi = "proposalQueue(uint256)")]
    pub struct ProposalQueueCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `proposals` function with signature `proposals(uint256)` and selector `0x013cf08b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposals", abi = "proposals(uint256)")]
    pub struct ProposalsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `proposedToKick` function with signature `proposedToKick(address)` and selector `0x3fc24bba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposedToKick", abi = "proposedToKick(address)")]
    pub struct ProposedToKickCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `proposedToWhitelist` function with signature `proposedToWhitelist(address)` and selector `0xe1780345`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposedToWhitelist", abi = "proposedToWhitelist(address)")]
    pub struct ProposedToWhitelistCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `ragekick` function with signature `ragekick(address)` and selector `0xdfdd369e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ragekick", abi = "ragekick(address)")]
    pub struct RagekickCall {
        pub member_to_kick: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `ragequit` function with signature `ragequit(uint256,uint256)` and selector `0x15eb349e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ragequit", abi = "ragequit(uint256,uint256)")]
    pub struct RagequitCall {
        pub shares_to_burn: ::ethers::core::types::U256,
        pub loot_to_burn: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sponsorProposal` function with signature `sponsorProposal(uint256)` and selector `0x9746d940`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sponsorProposal", abi = "sponsorProposal(uint256)")]
    pub struct SponsorProposalCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `submitGuildKickProposal` function with signature `submitGuildKickProposal(address,string)` and selector `0x115b2d18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "submitGuildKickProposal",
        abi = "submitGuildKickProposal(address,string)"
    )]
    pub struct SubmitGuildKickProposalCall {
        pub member_to_kick: ::ethers::core::types::Address,
        pub details: ::std::string::String,
    }
    ///Container type for all input parameters for the `submitProposal` function with signature `submitProposal(address,uint256,uint256,uint256,address,uint256,address,string)` and selector `0x590f940b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "submitProposal",
        abi = "submitProposal(address,uint256,uint256,uint256,address,uint256,address,string)"
    )]
    pub struct SubmitProposalCall {
        pub applicant: ::ethers::core::types::Address,
        pub shares_requested: ::ethers::core::types::U256,
        pub loot_requested: ::ethers::core::types::U256,
        pub tribute_offered: ::ethers::core::types::U256,
        pub tribute_token: ::ethers::core::types::Address,
        pub payment_requested: ::ethers::core::types::U256,
        pub payment_token: ::ethers::core::types::Address,
        pub details: ::std::string::String,
    }
    ///Container type for all input parameters for the `submitVote` function with signature `submitVote(uint256,uint8)` and selector `0x99653fbe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "submitVote", abi = "submitVote(uint256,uint8)")]
    pub struct SubmitVoteCall {
        pub proposal_index: ::ethers::core::types::U256,
        pub uint_vote: u8,
    }
    ///Container type for all input parameters for the `submitWhitelistProposal` function with signature `submitWhitelistProposal(address,string)` and selector `0xfeb7ea1d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "submitWhitelistProposal",
        abi = "submitWhitelistProposal(address,string)"
    )]
    pub struct SubmitWhitelistProposalCall {
        pub token_to_whitelist: ::ethers::core::types::Address,
        pub details: ::std::string::String,
    }
    ///Container type for all input parameters for the `summoningTime` function with signature `summoningTime()` and selector `0x7d5b6c72`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "summoningTime", abi = "summoningTime()")]
    pub struct SummoningTimeCall;
    ///Container type for all input parameters for the `tokenWhitelist` function with signature `tokenWhitelist(address)` and selector `0x753d7563`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenWhitelist", abi = "tokenWhitelist(address)")]
    pub struct TokenWhitelistCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `totalGuildBankTokens` function with signature `totalGuildBankTokens()` and selector `0x9d1722cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalGuildBankTokens", abi = "totalGuildBankTokens()")]
    pub struct TotalGuildBankTokensCall;
    ///Container type for all input parameters for the `totalLoot` function with signature `totalLoot()` and selector `0x635e99aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalLoot", abi = "totalLoot()")]
    pub struct TotalLootCall;
    ///Container type for all input parameters for the `totalShares` function with signature `totalShares()` and selector `0x3a98ef39`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalShares", abi = "totalShares()")]
    pub struct TotalSharesCall;
    ///Container type for all input parameters for the `updateDelegateKey` function with signature `updateDelegateKey(address)` and selector `0x2582bf2a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateDelegateKey", abi = "updateDelegateKey(address)")]
    pub struct UpdateDelegateKeyCall {
        pub new_delegate_key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `userTokenBalances` function with signature `userTokenBalances(address,address)` and selector `0x45f2d105`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "userTokenBalances", abi = "userTokenBalances(address,address)")]
    pub struct UserTokenBalancesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `votingPeriodLength` function with signature `votingPeriodLength()` and selector `0x8340bbce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "votingPeriodLength", abi = "votingPeriodLength()")]
    pub struct VotingPeriodLengthCall;
    ///Container type for all input parameters for the `withdrawBalance` function with signature `withdrawBalance(address,uint256)` and selector `0x0cf20cc9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawBalance", abi = "withdrawBalance(address,uint256)")]
    pub struct WithdrawBalanceCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawBalances` function with signature `withdrawBalances(address[],uint256[],bool)` and selector `0x4482394b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "withdrawBalances",
        abi = "withdrawBalances(address[],uint256[],bool)"
    )]
    pub struct WithdrawBalancesCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub max: bool,
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
    pub enum MolochDAOv2_1Calls {
        Escrow(EscrowCall),
        Guild(GuildCall),
        Total(TotalCall),
        ApprovedTokens(ApprovedTokensCall),
        CanRagequit(CanRagequitCall),
        CancelProposal(CancelProposalCall),
        CollectTokens(CollectTokensCall),
        DepositToken(DepositTokenCall),
        DilutionBound(DilutionBoundCall),
        GetCurrentPeriod(GetCurrentPeriodCall),
        GetMemberProposalVote(GetMemberProposalVoteCall),
        GetProposalFlags(GetProposalFlagsCall),
        GetProposalQueueLength(GetProposalQueueLengthCall),
        GetTokenCount(GetTokenCountCall),
        GetUserTokenBalance(GetUserTokenBalanceCall),
        GracePeriodLength(GracePeriodLengthCall),
        HasVotingPeriodExpired(HasVotingPeriodExpiredCall),
        Init(InitCall),
        MemberAddressByDelegateKey(MemberAddressByDelegateKeyCall),
        MemberList(MemberListCall),
        Members(MembersCall),
        PeriodDuration(PeriodDurationCall),
        ProcessGuildKickProposal(ProcessGuildKickProposalCall),
        ProcessProposal(ProcessProposalCall),
        ProcessWhitelistProposal(ProcessWhitelistProposalCall),
        ProcessingReward(ProcessingRewardCall),
        ProposalCount(ProposalCountCall),
        ProposalDeposit(ProposalDepositCall),
        ProposalQueue(ProposalQueueCall),
        Proposals(ProposalsCall),
        ProposedToKick(ProposedToKickCall),
        ProposedToWhitelist(ProposedToWhitelistCall),
        Ragekick(RagekickCall),
        Ragequit(RagequitCall),
        SponsorProposal(SponsorProposalCall),
        SubmitGuildKickProposal(SubmitGuildKickProposalCall),
        SubmitProposal(SubmitProposalCall),
        SubmitVote(SubmitVoteCall),
        SubmitWhitelistProposal(SubmitWhitelistProposalCall),
        SummoningTime(SummoningTimeCall),
        TokenWhitelist(TokenWhitelistCall),
        TotalGuildBankTokens(TotalGuildBankTokensCall),
        TotalLoot(TotalLootCall),
        TotalShares(TotalSharesCall),
        UpdateDelegateKey(UpdateDelegateKeyCall),
        UserTokenBalances(UserTokenBalancesCall),
        VotingPeriodLength(VotingPeriodLengthCall),
        WithdrawBalance(WithdrawBalanceCall),
        WithdrawBalances(WithdrawBalancesCall),
    }
    impl ::ethers::core::abi::AbiDecode for MolochDAOv2_1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EscrowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Escrow(decoded));
            }
            if let Ok(decoded) = <GuildCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Guild(decoded));
            }
            if let Ok(decoded) = <TotalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Total(decoded));
            }
            if let Ok(decoded) = <ApprovedTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApprovedTokens(decoded));
            }
            if let Ok(decoded) = <CanRagequitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanRagequit(decoded));
            }
            if let Ok(decoded) = <CancelProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelProposal(decoded));
            }
            if let Ok(decoded) = <CollectTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollectTokens(decoded));
            }
            if let Ok(decoded) = <DepositTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositToken(decoded));
            }
            if let Ok(decoded) = <DilutionBoundCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DilutionBound(decoded));
            }
            if let Ok(decoded) = <GetCurrentPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentPeriod(decoded));
            }
            if let Ok(decoded) = <GetMemberProposalVoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMemberProposalVote(decoded));
            }
            if let Ok(decoded) = <GetProposalFlagsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProposalFlags(decoded));
            }
            if let Ok(decoded) = <GetProposalQueueLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProposalQueueLength(decoded));
            }
            if let Ok(decoded) = <GetTokenCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenCount(decoded));
            }
            if let Ok(decoded) = <GetUserTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUserTokenBalance(decoded));
            }
            if let Ok(decoded) = <GracePeriodLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GracePeriodLength(decoded));
            }
            if let Ok(decoded) = <HasVotingPeriodExpiredCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasVotingPeriodExpired(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <MemberAddressByDelegateKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MemberAddressByDelegateKey(decoded));
            }
            if let Ok(decoded) = <MemberListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MemberList(decoded));
            }
            if let Ok(decoded) = <MembersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Members(decoded));
            }
            if let Ok(decoded) = <PeriodDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeriodDuration(decoded));
            }
            if let Ok(decoded) = <ProcessGuildKickProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessGuildKickProposal(decoded));
            }
            if let Ok(decoded) = <ProcessProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessProposal(decoded));
            }
            if let Ok(decoded) = <ProcessWhitelistProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessWhitelistProposal(decoded));
            }
            if let Ok(decoded) = <ProcessingRewardCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessingReward(decoded));
            }
            if let Ok(decoded) = <ProposalCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalCount(decoded));
            }
            if let Ok(decoded) = <ProposalDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalDeposit(decoded));
            }
            if let Ok(decoded) = <ProposalQueueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalQueue(decoded));
            }
            if let Ok(decoded) = <ProposalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Proposals(decoded));
            }
            if let Ok(decoded) = <ProposedToKickCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposedToKick(decoded));
            }
            if let Ok(decoded) = <ProposedToWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposedToWhitelist(decoded));
            }
            if let Ok(decoded) = <RagekickCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ragekick(decoded));
            }
            if let Ok(decoded) = <RagequitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ragequit(decoded));
            }
            if let Ok(decoded) = <SponsorProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SponsorProposal(decoded));
            }
            if let Ok(decoded) = <SubmitGuildKickProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitGuildKickProposal(decoded));
            }
            if let Ok(decoded) = <SubmitProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitProposal(decoded));
            }
            if let Ok(decoded) = <SubmitVoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitVote(decoded));
            }
            if let Ok(decoded) = <SubmitWhitelistProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitWhitelistProposal(decoded));
            }
            if let Ok(decoded) = <SummoningTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SummoningTime(decoded));
            }
            if let Ok(decoded) = <TokenWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenWhitelist(decoded));
            }
            if let Ok(decoded) = <TotalGuildBankTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalGuildBankTokens(decoded));
            }
            if let Ok(decoded) = <TotalLootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalLoot(decoded));
            }
            if let Ok(decoded) = <TotalSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalShares(decoded));
            }
            if let Ok(decoded) = <UpdateDelegateKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDelegateKey(decoded));
            }
            if let Ok(decoded) = <UserTokenBalancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserTokenBalances(decoded));
            }
            if let Ok(decoded) = <VotingPeriodLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VotingPeriodLength(decoded));
            }
            if let Ok(decoded) = <WithdrawBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawBalance(decoded));
            }
            if let Ok(decoded) = <WithdrawBalancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawBalances(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MolochDAOv2_1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Escrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Guild(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Total(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApprovedTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanRagequit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollectTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DilutionBound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMemberProposalVote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalFlags(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalQueueLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserTokenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GracePeriodLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasVotingPeriodExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MemberAddressByDelegateKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MemberList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Members(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PeriodDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessGuildKickProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessWhitelistProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessingReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalQueue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Proposals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposedToKick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposedToWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ragekick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ragequit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SponsorProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitGuildKickProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitVote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitWhitelistProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SummoningTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalGuildBankTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalLoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDelegateKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserTokenBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VotingPeriodLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MolochDAOv2_1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Escrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Guild(element) => ::core::fmt::Display::fmt(element, f),
                Self::Total(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovedTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::CanRagequit(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::DilutionBound(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMemberProposalVote(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposalFlags(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProposalQueueLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUserTokenBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GracePeriodLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasVotingPeriodExpired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::MemberAddressByDelegateKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MemberList(element) => ::core::fmt::Display::fmt(element, f),
                Self::Members(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeriodDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessGuildKickProposal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessWhitelistProposal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessingReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalQueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::Proposals(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposedToKick(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposedToWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Ragekick(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ragequit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SponsorProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitGuildKickProposal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitVote(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitWhitelistProposal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SummoningTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalGuildBankTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalLoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDelegateKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserTokenBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingPeriodLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawBalances(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EscrowCall> for MolochDAOv2_1Calls {
        fn from(value: EscrowCall) -> Self {
            Self::Escrow(value)
        }
    }
    impl ::core::convert::From<GuildCall> for MolochDAOv2_1Calls {
        fn from(value: GuildCall) -> Self {
            Self::Guild(value)
        }
    }
    impl ::core::convert::From<TotalCall> for MolochDAOv2_1Calls {
        fn from(value: TotalCall) -> Self {
            Self::Total(value)
        }
    }
    impl ::core::convert::From<ApprovedTokensCall> for MolochDAOv2_1Calls {
        fn from(value: ApprovedTokensCall) -> Self {
            Self::ApprovedTokens(value)
        }
    }
    impl ::core::convert::From<CanRagequitCall> for MolochDAOv2_1Calls {
        fn from(value: CanRagequitCall) -> Self {
            Self::CanRagequit(value)
        }
    }
    impl ::core::convert::From<CancelProposalCall> for MolochDAOv2_1Calls {
        fn from(value: CancelProposalCall) -> Self {
            Self::CancelProposal(value)
        }
    }
    impl ::core::convert::From<CollectTokensCall> for MolochDAOv2_1Calls {
        fn from(value: CollectTokensCall) -> Self {
            Self::CollectTokens(value)
        }
    }
    impl ::core::convert::From<DepositTokenCall> for MolochDAOv2_1Calls {
        fn from(value: DepositTokenCall) -> Self {
            Self::DepositToken(value)
        }
    }
    impl ::core::convert::From<DilutionBoundCall> for MolochDAOv2_1Calls {
        fn from(value: DilutionBoundCall) -> Self {
            Self::DilutionBound(value)
        }
    }
    impl ::core::convert::From<GetCurrentPeriodCall> for MolochDAOv2_1Calls {
        fn from(value: GetCurrentPeriodCall) -> Self {
            Self::GetCurrentPeriod(value)
        }
    }
    impl ::core::convert::From<GetMemberProposalVoteCall> for MolochDAOv2_1Calls {
        fn from(value: GetMemberProposalVoteCall) -> Self {
            Self::GetMemberProposalVote(value)
        }
    }
    impl ::core::convert::From<GetProposalFlagsCall> for MolochDAOv2_1Calls {
        fn from(value: GetProposalFlagsCall) -> Self {
            Self::GetProposalFlags(value)
        }
    }
    impl ::core::convert::From<GetProposalQueueLengthCall> for MolochDAOv2_1Calls {
        fn from(value: GetProposalQueueLengthCall) -> Self {
            Self::GetProposalQueueLength(value)
        }
    }
    impl ::core::convert::From<GetTokenCountCall> for MolochDAOv2_1Calls {
        fn from(value: GetTokenCountCall) -> Self {
            Self::GetTokenCount(value)
        }
    }
    impl ::core::convert::From<GetUserTokenBalanceCall> for MolochDAOv2_1Calls {
        fn from(value: GetUserTokenBalanceCall) -> Self {
            Self::GetUserTokenBalance(value)
        }
    }
    impl ::core::convert::From<GracePeriodLengthCall> for MolochDAOv2_1Calls {
        fn from(value: GracePeriodLengthCall) -> Self {
            Self::GracePeriodLength(value)
        }
    }
    impl ::core::convert::From<HasVotingPeriodExpiredCall> for MolochDAOv2_1Calls {
        fn from(value: HasVotingPeriodExpiredCall) -> Self {
            Self::HasVotingPeriodExpired(value)
        }
    }
    impl ::core::convert::From<InitCall> for MolochDAOv2_1Calls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<MemberAddressByDelegateKeyCall> for MolochDAOv2_1Calls {
        fn from(value: MemberAddressByDelegateKeyCall) -> Self {
            Self::MemberAddressByDelegateKey(value)
        }
    }
    impl ::core::convert::From<MemberListCall> for MolochDAOv2_1Calls {
        fn from(value: MemberListCall) -> Self {
            Self::MemberList(value)
        }
    }
    impl ::core::convert::From<MembersCall> for MolochDAOv2_1Calls {
        fn from(value: MembersCall) -> Self {
            Self::Members(value)
        }
    }
    impl ::core::convert::From<PeriodDurationCall> for MolochDAOv2_1Calls {
        fn from(value: PeriodDurationCall) -> Self {
            Self::PeriodDuration(value)
        }
    }
    impl ::core::convert::From<ProcessGuildKickProposalCall> for MolochDAOv2_1Calls {
        fn from(value: ProcessGuildKickProposalCall) -> Self {
            Self::ProcessGuildKickProposal(value)
        }
    }
    impl ::core::convert::From<ProcessProposalCall> for MolochDAOv2_1Calls {
        fn from(value: ProcessProposalCall) -> Self {
            Self::ProcessProposal(value)
        }
    }
    impl ::core::convert::From<ProcessWhitelistProposalCall> for MolochDAOv2_1Calls {
        fn from(value: ProcessWhitelistProposalCall) -> Self {
            Self::ProcessWhitelistProposal(value)
        }
    }
    impl ::core::convert::From<ProcessingRewardCall> for MolochDAOv2_1Calls {
        fn from(value: ProcessingRewardCall) -> Self {
            Self::ProcessingReward(value)
        }
    }
    impl ::core::convert::From<ProposalCountCall> for MolochDAOv2_1Calls {
        fn from(value: ProposalCountCall) -> Self {
            Self::ProposalCount(value)
        }
    }
    impl ::core::convert::From<ProposalDepositCall> for MolochDAOv2_1Calls {
        fn from(value: ProposalDepositCall) -> Self {
            Self::ProposalDeposit(value)
        }
    }
    impl ::core::convert::From<ProposalQueueCall> for MolochDAOv2_1Calls {
        fn from(value: ProposalQueueCall) -> Self {
            Self::ProposalQueue(value)
        }
    }
    impl ::core::convert::From<ProposalsCall> for MolochDAOv2_1Calls {
        fn from(value: ProposalsCall) -> Self {
            Self::Proposals(value)
        }
    }
    impl ::core::convert::From<ProposedToKickCall> for MolochDAOv2_1Calls {
        fn from(value: ProposedToKickCall) -> Self {
            Self::ProposedToKick(value)
        }
    }
    impl ::core::convert::From<ProposedToWhitelistCall> for MolochDAOv2_1Calls {
        fn from(value: ProposedToWhitelistCall) -> Self {
            Self::ProposedToWhitelist(value)
        }
    }
    impl ::core::convert::From<RagekickCall> for MolochDAOv2_1Calls {
        fn from(value: RagekickCall) -> Self {
            Self::Ragekick(value)
        }
    }
    impl ::core::convert::From<RagequitCall> for MolochDAOv2_1Calls {
        fn from(value: RagequitCall) -> Self {
            Self::Ragequit(value)
        }
    }
    impl ::core::convert::From<SponsorProposalCall> for MolochDAOv2_1Calls {
        fn from(value: SponsorProposalCall) -> Self {
            Self::SponsorProposal(value)
        }
    }
    impl ::core::convert::From<SubmitGuildKickProposalCall> for MolochDAOv2_1Calls {
        fn from(value: SubmitGuildKickProposalCall) -> Self {
            Self::SubmitGuildKickProposal(value)
        }
    }
    impl ::core::convert::From<SubmitProposalCall> for MolochDAOv2_1Calls {
        fn from(value: SubmitProposalCall) -> Self {
            Self::SubmitProposal(value)
        }
    }
    impl ::core::convert::From<SubmitVoteCall> for MolochDAOv2_1Calls {
        fn from(value: SubmitVoteCall) -> Self {
            Self::SubmitVote(value)
        }
    }
    impl ::core::convert::From<SubmitWhitelistProposalCall> for MolochDAOv2_1Calls {
        fn from(value: SubmitWhitelistProposalCall) -> Self {
            Self::SubmitWhitelistProposal(value)
        }
    }
    impl ::core::convert::From<SummoningTimeCall> for MolochDAOv2_1Calls {
        fn from(value: SummoningTimeCall) -> Self {
            Self::SummoningTime(value)
        }
    }
    impl ::core::convert::From<TokenWhitelistCall> for MolochDAOv2_1Calls {
        fn from(value: TokenWhitelistCall) -> Self {
            Self::TokenWhitelist(value)
        }
    }
    impl ::core::convert::From<TotalGuildBankTokensCall> for MolochDAOv2_1Calls {
        fn from(value: TotalGuildBankTokensCall) -> Self {
            Self::TotalGuildBankTokens(value)
        }
    }
    impl ::core::convert::From<TotalLootCall> for MolochDAOv2_1Calls {
        fn from(value: TotalLootCall) -> Self {
            Self::TotalLoot(value)
        }
    }
    impl ::core::convert::From<TotalSharesCall> for MolochDAOv2_1Calls {
        fn from(value: TotalSharesCall) -> Self {
            Self::TotalShares(value)
        }
    }
    impl ::core::convert::From<UpdateDelegateKeyCall> for MolochDAOv2_1Calls {
        fn from(value: UpdateDelegateKeyCall) -> Self {
            Self::UpdateDelegateKey(value)
        }
    }
    impl ::core::convert::From<UserTokenBalancesCall> for MolochDAOv2_1Calls {
        fn from(value: UserTokenBalancesCall) -> Self {
            Self::UserTokenBalances(value)
        }
    }
    impl ::core::convert::From<VotingPeriodLengthCall> for MolochDAOv2_1Calls {
        fn from(value: VotingPeriodLengthCall) -> Self {
            Self::VotingPeriodLength(value)
        }
    }
    impl ::core::convert::From<WithdrawBalanceCall> for MolochDAOv2_1Calls {
        fn from(value: WithdrawBalanceCall) -> Self {
            Self::WithdrawBalance(value)
        }
    }
    impl ::core::convert::From<WithdrawBalancesCall> for MolochDAOv2_1Calls {
        fn from(value: WithdrawBalancesCall) -> Self {
            Self::WithdrawBalances(value)
        }
    }
    ///Container type for all return fields from the `ESCROW` function with signature `ESCROW()` and selector `0xe681c4aa`
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
    pub struct EscrowReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `GUILD` function with signature `GUILD()` and selector `0xf5d54c77`
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
    pub struct GuildReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `TOTAL` function with signature `TOTAL()` and selector `0x27efc086`
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
    pub struct TotalReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `approvedTokens` function with signature `approvedTokens(uint256)` and selector `0x1dafede0`
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
    pub struct ApprovedTokensReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `canRagequit` function with signature `canRagequit(uint256)` and selector `0xa3dc3800`
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
    pub struct CanRagequitReturn(pub bool);
    ///Container type for all return fields from the `depositToken` function with signature `depositToken()` and selector `0xc89039c5`
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
    pub struct DepositTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `dilutionBound` function with signature `dilutionBound()` and selector `0xafe5475f`
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
    pub struct DilutionBoundReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getCurrentPeriod` function with signature `getCurrentPeriod()` and selector `0x086146d2`
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
    pub struct GetCurrentPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMemberProposalVote` function with signature `getMemberProposalVote(address,uint256)` and selector `0x044a0ca8`
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
    pub struct GetMemberProposalVoteReturn(pub u8);
    ///Container type for all return fields from the `getProposalFlags` function with signature `getProposalFlags(uint256)` and selector `0xb2643aab`
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
    pub struct GetProposalFlagsReturn(pub [bool; 6]);
    ///Container type for all return fields from the `getProposalQueueLength` function with signature `getProposalQueueLength()` and selector `0x797daf70`
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
    pub struct GetProposalQueueLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTokenCount` function with signature `getTokenCount()` and selector `0x78a89567`
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
    pub struct GetTokenCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUserTokenBalance` function with signature `getUserTokenBalance(address,address)` and selector `0x73f8fd4b`
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
    pub struct GetUserTokenBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gracePeriodLength` function with signature `gracePeriodLength()` and selector `0x63858f2d`
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
    pub struct GracePeriodLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasVotingPeriodExpired` function with signature `hasVotingPeriodExpired(uint256)` and selector `0x9425a476`
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
    pub struct HasVotingPeriodExpiredReturn(pub bool);
    ///Container type for all return fields from the `memberAddressByDelegateKey` function with signature `memberAddressByDelegateKey(address)` and selector `0x402c1794`
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
    pub struct MemberAddressByDelegateKeyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `memberList` function with signature `memberList(uint256)` and selector `0xb307fc6d`
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
    pub struct MemberListReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `members` function with signature `members(address)` and selector `0x08ae4b0c`
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
    pub struct MembersReturn {
        pub delegate_key: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
        pub loot: ::ethers::core::types::U256,
        pub exists: bool,
        pub highest_index_yes_vote: ::ethers::core::types::U256,
        pub jailed: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `periodDuration` function with signature `periodDuration()` and selector `0xb470aade`
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
    pub struct PeriodDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `processingReward` function with signature `processingReward()` and selector `0x03e32fa1`
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
    pub struct ProcessingRewardReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalCount` function with signature `proposalCount()` and selector `0xda35c664`
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
    pub struct ProposalCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalDeposit` function with signature `proposalDeposit()` and selector `0x8b15a605`
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
    pub struct ProposalDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalQueue` function with signature `proposalQueue(uint256)` and selector `0x3b214a74`
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
    pub struct ProposalQueueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposals` function with signature `proposals(uint256)` and selector `0x013cf08b`
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
    pub struct ProposalsReturn {
        pub applicant: ::ethers::core::types::Address,
        pub proposer: ::ethers::core::types::Address,
        pub sponsor: ::ethers::core::types::Address,
        pub shares_requested: ::ethers::core::types::U256,
        pub loot_requested: ::ethers::core::types::U256,
        pub tribute_offered: ::ethers::core::types::U256,
        pub tribute_token: ::ethers::core::types::Address,
        pub payment_requested: ::ethers::core::types::U256,
        pub payment_token: ::ethers::core::types::Address,
        pub starting_period: ::ethers::core::types::U256,
        pub yes_votes: ::ethers::core::types::U256,
        pub no_votes: ::ethers::core::types::U256,
        pub details: ::std::string::String,
        pub max_total_shares_and_loot_at_yes_vote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `proposedToKick` function with signature `proposedToKick(address)` and selector `0x3fc24bba`
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
    pub struct ProposedToKickReturn(pub bool);
    ///Container type for all return fields from the `proposedToWhitelist` function with signature `proposedToWhitelist(address)` and selector `0xe1780345`
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
    pub struct ProposedToWhitelistReturn(pub bool);
    ///Container type for all return fields from the `submitGuildKickProposal` function with signature `submitGuildKickProposal(address,string)` and selector `0x115b2d18`
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
    pub struct SubmitGuildKickProposalReturn {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `submitProposal` function with signature `submitProposal(address,uint256,uint256,uint256,address,uint256,address,string)` and selector `0x590f940b`
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
    pub struct SubmitProposalReturn {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `submitWhitelistProposal` function with signature `submitWhitelistProposal(address,string)` and selector `0xfeb7ea1d`
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
    pub struct SubmitWhitelistProposalReturn {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `summoningTime` function with signature `summoningTime()` and selector `0x7d5b6c72`
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
    pub struct SummoningTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenWhitelist` function with signature `tokenWhitelist(address)` and selector `0x753d7563`
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
    pub struct TokenWhitelistReturn(pub bool);
    ///Container type for all return fields from the `totalGuildBankTokens` function with signature `totalGuildBankTokens()` and selector `0x9d1722cb`
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
    pub struct TotalGuildBankTokensReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalLoot` function with signature `totalLoot()` and selector `0x635e99aa`
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
    pub struct TotalLootReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalShares` function with signature `totalShares()` and selector `0x3a98ef39`
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
    pub struct TotalSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userTokenBalances` function with signature `userTokenBalances(address,address)` and selector `0x45f2d105`
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
    pub struct UserTokenBalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `votingPeriodLength` function with signature `votingPeriodLength()` and selector `0x8340bbce`
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
    pub struct VotingPeriodLengthReturn(pub ::ethers::core::types::U256);
}
