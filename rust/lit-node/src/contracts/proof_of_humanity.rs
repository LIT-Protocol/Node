pub use proof_of_humanity::*;
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
pub mod proof_of_humanity {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/ProofOfHumanity.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_arbitrator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IArbitrator"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_arbitratorExtraData"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_registrationMetaEvidence",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_clearingMetaEvidence"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_submissionBaseDeposit"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_submissionDuration"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_renewalPeriodDuration"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_challengePeriodDuration",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_multipliers"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ),
                            3usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256[3]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_requiredNumberOfVouches",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addSubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addSubmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addSubmissionManually"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addSubmissionManually",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionIDs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_names"),
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addVouch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addVouch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("arbitratorDataList"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("arbitratorDataList"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arbitrator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IArbitrator"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "metaEvidenceUpdates",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "arbitratorExtraData",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("arbitratorDisputeIDToDisputeData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "arbitratorDisputeIDToDisputeData",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("challengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("challengePeriodDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "challengePeriodDuration",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("challengeRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("challengeRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProofOfHumanity.Reason",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_duplicateID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeArbitrator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeArbitrator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_arbitrator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IArbitrator"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_arbitratorExtraData",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeDurations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeDurations"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_submissionDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_renewalPeriodDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_challengePeriodDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeGovernor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeGovernor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_governor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeLoserStakeMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeLoserStakeMultiplier",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_loserStakeMultiplier",
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeMetaEvidence"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeMetaEvidence"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_registrationMetaEvidence",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_clearingMetaEvidence",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeRequiredNumberOfVouches"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeRequiredNumberOfVouches",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_requiredNumberOfVouches",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeSharedStakeMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeSharedStakeMultiplier",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_sharedStakeMultiplier",
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeStateToPending"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeStateToPending",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_vouches"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signatures"),
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
                                        "_expirationTimestamps",
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeSubmissionBaseDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeSubmissionBaseDeposit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_submissionBaseDeposit",
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeWinnerStakeMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeWinnerStakeMultiplier",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_winnerStakeMultiplier",
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkRequestDuplicates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkRequestDuplicates",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_duplicateID"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fundAppeal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fundAppeal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_side"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProofOfHumanity.Party",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fundSubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fundSubmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getArbitratorDataListCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getArbitratorDataListCount",
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChallengeInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChallengeInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
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
                                    name: ::std::borrow::ToOwned::to_owned("lastRoundID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("challenger"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("disputeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ruling"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProofOfHumanity.Party",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "duplicateSubmissionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getContributions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getContributions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_round"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contributor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contributions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        3usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[3]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumberOfVouches"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNumberOfVouches"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRequestInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRequestInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
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
                                    name: ::std::borrow::ToOwned::to_owned("disputed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resolved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requesterLost"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentReason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProofOfHumanity.Reason",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nbParallelDisputes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastChallengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("arbitratorDataID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requester"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "ultimateChallenger",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("usedReasons"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoundInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoundInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_round"),
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
                                    name: ::std::borrow::ToOwned::to_owned("appealed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paidFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        3usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[3]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sideFunded"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProofOfHumanity.Party",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeRewards"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSubmissionInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSubmissionInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ProofOfHumanity.Status",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("submissionTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("registered"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hasVouched"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("numberOfRequests"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("governor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("governor"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("loserStakeMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "loserStakeMultiplier",
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processVouches"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("processVouches"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_iterations"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reapplySubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reapplySubmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeSubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeSubmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeSubmissionManually"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeSubmissionManually",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeVouch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeVouch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renewalPeriodDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "renewalPeriodDuration",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requiredNumberOfVouches"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requiredNumberOfVouches",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_disputeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ruling"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sharedStakeMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sharedStakeMultiplier",
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submissionBaseDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "submissionBaseDeposit",
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submissionCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submissionCounter"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submissionDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submissionDuration"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitEvidence"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitEvidence"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vouches"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vouches"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("winnerStakeMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "winnerStakeMultiplier",
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawFeesAndRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawFeesAndRewards",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_beneficiary"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_round"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawSubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawSubmission"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddSubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddSubmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
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
                    ::std::borrow::ToOwned::to_owned("AppealContribution"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AppealContribution"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_party"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_contributor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                    ::std::borrow::ToOwned::to_owned("ArbitratorComplete"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ArbitratorComplete"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_arbitrator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_governor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_submissionBaseDeposit",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_submissionDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_challengePeriodDuration",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_requiredNumberOfVouches",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_sharedStakeMultiplier",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_winnerStakeMultiplier",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_loserStakeMultiplier",
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
                    ::std::borrow::ToOwned::to_owned("ChallengeResolved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChallengeResolved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
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
                    ::std::borrow::ToOwned::to_owned("Dispute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Dispute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_arbitrator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_disputeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_metaEvidenceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_evidenceGroupID"),
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
                    ::std::borrow::ToOwned::to_owned("Evidence"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Evidence"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_arbitrator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_evidenceGroupID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_party"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HasPaidAppealFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("HasPaidAppealFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_side"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MetaEvidence"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MetaEvidence"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_metaEvidenceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_evidence"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReapplySubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReapplySubmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
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
                    ::std::borrow::ToOwned::to_owned("RemoveSubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoveSubmission"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_requester"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
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
                    ::std::borrow::ToOwned::to_owned("Ruling"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Ruling"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_arbitrator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_disputeID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_ruling"),
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
                    ::std::borrow::ToOwned::to_owned("SubmissionChallenged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SubmissionChallenged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_requestID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_challengeID"),
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
                    ::std::borrow::ToOwned::to_owned("VouchAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VouchAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_voucher"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VouchRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VouchRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_submissionID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_voucher"),
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
    pub static PROOFOFHUMANITY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ProofOfHumanity<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ProofOfHumanity<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ProofOfHumanity<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ProofOfHumanity<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ProofOfHumanity<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ProofOfHumanity))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ProofOfHumanity<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PROOFOFHUMANITY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addSubmission` (0x4690d55a) function
        pub fn add_submission(
            &self,
            evidence: ::std::string::String,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 144, 213, 90], (evidence, name))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addSubmissionManually` (0x61b90541) function
        pub fn add_submission_manually(
            &self,
            submission_i_ds: ::std::vec::Vec<::ethers::core::types::Address>,
            evidence: ::std::vec::Vec<::std::string::String>,
            names: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 185, 5, 65], (submission_i_ds, evidence, names))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addVouch` (0x32fe596f) function
        pub fn add_vouch(
            &self,
            submission_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 254, 89, 111], submission_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbitratorDataList` (0xec0e71ba) function
        pub fn arbitrator_data_list(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u128, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([236, 14, 113, 186], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbitratorDisputeIDToDisputeData` (0xdd254cd3) function
        pub fn arbitrator_dispute_id_to_dispute_data(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u128, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([221, 37, 76, 211], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `challengePeriodDuration` (0x0082a36d) function
        pub fn challenge_period_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([0, 130, 163, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `challengeRequest` (0xf40e0aed) function
        pub fn challenge_request(
            &self,
            submission_id: ::ethers::core::types::Address,
            reason: u8,
            duplicate_id: ::ethers::core::types::Address,
            evidence: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [244, 14, 10, 237],
                    (submission_id, reason, duplicate_id, evidence),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeArbitrator` (0xba7079ca) function
        pub fn change_arbitrator(
            &self,
            arbitrator: ::ethers::core::types::Address,
            arbitrator_extra_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 112, 121, 202], (arbitrator, arbitrator_extra_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeDurations` (0x26bafe5f) function
        pub fn change_durations(
            &self,
            submission_duration: u64,
            renewal_period_duration: u64,
            challenge_period_duration: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 186, 254, 95],
                    (
                        submission_duration,
                        renewal_period_duration,
                        challenge_period_duration,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeGovernor` (0xe4c0aaf4) function
        pub fn change_governor(
            &self,
            governor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 192, 170, 244], governor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeLoserStakeMultiplier` (0x92239dff) function
        pub fn change_loser_stake_multiplier(
            &self,
            loser_stake_multiplier: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 35, 157, 255], loser_stake_multiplier)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeMetaEvidence` (0xd706be31) function
        pub fn change_meta_evidence(
            &self,
            registration_meta_evidence: ::std::string::String,
            clearing_meta_evidence: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 6, 190, 49],
                    (registration_meta_evidence, clearing_meta_evidence),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeRequiredNumberOfVouches` (0xf65ab1be) function
        pub fn change_required_number_of_vouches(
            &self,
            required_number_of_vouches: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 90, 177, 190], required_number_of_vouches)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeSharedStakeMultiplier` (0x12ce3525) function
        pub fn change_shared_stake_multiplier(
            &self,
            shared_stake_multiplier: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 206, 53, 37], shared_stake_multiplier)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeStateToPending` (0xb4dfe93d) function
        pub fn change_state_to_pending(
            &self,
            submission_id: ::ethers::core::types::Address,
            vouches: ::std::vec::Vec<::ethers::core::types::Address>,
            signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
            expiration_timestamps: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [180, 223, 233, 61],
                    (submission_id, vouches, signatures, expiration_timestamps),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeSubmissionBaseDeposit` (0x33e5d047) function
        pub fn change_submission_base_deposit(
            &self,
            submission_base_deposit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 229, 208, 71], submission_base_deposit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeWinnerStakeMultiplier` (0xadc7faba) function
        pub fn change_winner_stake_multiplier(
            &self,
            winner_stake_multiplier: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 199, 250, 186], winner_stake_multiplier)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkRequestDuplicates` (0x2e848506) function
        pub fn check_request_duplicates(
            &self,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
            duplicate_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [46, 132, 133, 6],
                    (submission_id, request_id, duplicate_id),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRequest` (0x6e98762d) function
        pub fn execute_request(
            &self,
            submission_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 152, 118, 45], submission_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fundAppeal` (0xd7e9f178) function
        pub fn fund_appeal(
            &self,
            submission_id: ::ethers::core::types::Address,
            challenge_id: ::ethers::core::types::U256,
            side: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 233, 241, 120], (submission_id, challenge_id, side))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fundSubmission` (0xa27456bb) function
        pub fn fund_submission(
            &self,
            submission_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 116, 86, 187], submission_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getArbitratorDataListCount` (0x90d7c13c) function
        pub fn get_arbitrator_data_list_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 215, 193, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChallengeInfo` (0xd64240de) function
        pub fn get_challenge_info(
            &self,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
            challenge_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u16, ::ethers::core::types::Address, ::ethers::core::types::U256, u8, u64),
        > {
            self.0
                .method_hash(
                    [214, 66, 64, 222],
                    (submission_id, request_id, challenge_id),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getContributions` (0x3a8363c2) function
        pub fn get_contributions(
            &self,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
            challenge_id: ::ethers::core::types::U256,
            round: ::ethers::core::types::U256,
            contributor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            [::ethers::core::types::U256; 3],
        > {
            self.0
                .method_hash(
                    [58, 131, 99, 194],
                    (submission_id, request_id, challenge_id, round, contributor),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumberOfVouches` (0xdeb8f707) function
        pub fn get_number_of_vouches(
            &self,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([222, 184, 247, 7], (submission_id, request_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRequestInfo` (0x6e112409) function
        pub fn get_request_info(
            &self,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                bool,
                bool,
                u8,
                u16,
                u16,
                u16,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                u8,
            ),
        > {
            self.0
                .method_hash([110, 17, 36, 9], (submission_id, request_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundInfo` (0xa84dc70e) function
        pub fn get_round_info(
            &self,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
            challenge_id: ::ethers::core::types::U256,
            round: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, [::ethers::core::types::U256; 3], u8, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [168, 77, 199, 14],
                    (submission_id, request_id, challenge_id, round),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubmissionInfo` (0x97973043) function
        pub fn get_submission_info(
            &self,
            submission_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, u64, u64, bool, bool, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([151, 151, 48, 67], submission_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `governor` (0x0c340a24) function
        pub fn governor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([12, 52, 10, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isRegistered` (0xc3c5a547) function
        pub fn is_registered(
            &self,
            submission_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([195, 197, 165, 71], submission_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `loserStakeMultiplier` (0x1d512085) function
        pub fn loser_stake_multiplier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 81, 32, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processVouches` (0x649a08bf) function
        pub fn process_vouches(
            &self,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
            iterations: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 154, 8, 191], (submission_id, request_id, iterations))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reapplySubmission` (0x5a9ef341) function
        pub fn reapply_submission(
            &self,
            evidence: ::std::string::String,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 158, 243, 65], (evidence, name))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeSubmission` (0xf4934cdb) function
        pub fn remove_submission(
            &self,
            submission_id: ::ethers::core::types::Address,
            evidence: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 147, 76, 219], (submission_id, evidence))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeSubmissionManually` (0xa6c6ecc9) function
        pub fn remove_submission_manually(
            &self,
            submission_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 198, 236, 201], submission_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeVouch` (0x84d1c91a) function
        pub fn remove_vouch(
            &self,
            submission_id: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 209, 201, 26], submission_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renewalPeriodDuration` (0x876c63d4) function
        pub fn renewal_period_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([135, 108, 99, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requiredNumberOfVouches` (0x2d9489c6) function
        pub fn required_number_of_vouches(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 148, 137, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rule` (0x311a6c56) function
        pub fn rule(
            &self,
            dispute_id: ::ethers::core::types::U256,
            ruling: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 26, 108, 86], (dispute_id, ruling))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharedStakeMultiplier` (0x41658341) function
        pub fn shared_stake_multiplier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 101, 131, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submissionBaseDeposit` (0xbb0b86ff) function
        pub fn submission_base_deposit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([187, 11, 134, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submissionCounter` (0x76c23ff1) function
        pub fn submission_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([118, 194, 63, 241], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submissionDuration` (0xf633c293) function
        pub fn submission_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([246, 51, 194, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitEvidence` (0x5bb5e55b) function
        pub fn submit_evidence(
            &self,
            submission_id: ::ethers::core::types::Address,
            evidence: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 181, 229, 91], (submission_id, evidence))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vouches` (0x0b337be6) function
        pub fn vouches(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([11, 51, 123, 230], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `winnerStakeMultiplier` (0x7b943383) function
        pub fn winner_stake_multiplier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([123, 148, 51, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFeesAndRewards` (0x9a72e0b3) function
        pub fn withdraw_fees_and_rewards(
            &self,
            beneficiary: ::ethers::core::types::Address,
            submission_id: ::ethers::core::types::Address,
            request_id: ::ethers::core::types::U256,
            challenge_id: ::ethers::core::types::U256,
            round: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [154, 114, 224, 179],
                    (beneficiary, submission_id, request_id, challenge_id, round),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawSubmission` (0xce52b9f4) function
        pub fn withdraw_submission(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 82, 185, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddSubmission` event
        pub fn add_submission_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddSubmissionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AppealContribution` event
        pub fn appeal_contribution_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AppealContributionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ArbitratorComplete` event
        pub fn arbitrator_complete_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ArbitratorCompleteFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChallengeResolved` event
        pub fn challenge_resolved_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChallengeResolvedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Dispute` event
        pub fn dispute_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DisputeFilter> {
            self.0.event()
        }
        ///Gets the contract's `Evidence` event
        pub fn evidence_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EvidenceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `HasPaidAppealFee` event
        pub fn has_paid_appeal_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HasPaidAppealFeeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MetaEvidence` event
        pub fn meta_evidence_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MetaEvidenceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReapplySubmission` event
        pub fn reapply_submission_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReapplySubmissionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemoveSubmission` event
        pub fn remove_submission_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoveSubmissionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Ruling` event
        pub fn ruling_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RulingFilter> {
            self.0.event()
        }
        ///Gets the contract's `SubmissionChallenged` event
        pub fn submission_challenged_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SubmissionChallengedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VouchAdded` event
        pub fn vouch_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VouchAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VouchRemoved` event
        pub fn vouch_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VouchRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProofOfHumanityEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ProofOfHumanity<M> {
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
    #[ethevent(name = "AddSubmission", abi = "AddSubmission(address,uint256)")]
    pub struct AddSubmissionFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
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
        name = "AppealContribution",
        abi = "AppealContribution(address,uint256,uint8,address,uint256)"
    )]
    pub struct AppealContributionFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub challenge_id: ::ethers::core::types::U256,
        pub party: u8,
        #[ethevent(indexed)]
        pub contributor: ::ethers::core::types::Address,
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
        name = "ArbitratorComplete",
        abi = "ArbitratorComplete(address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ArbitratorCompleteFilter {
        pub arbitrator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub governor: ::ethers::core::types::Address,
        pub submission_base_deposit: ::ethers::core::types::U256,
        pub submission_duration: ::ethers::core::types::U256,
        pub challenge_period_duration: ::ethers::core::types::U256,
        pub required_number_of_vouches: ::ethers::core::types::U256,
        pub shared_stake_multiplier: ::ethers::core::types::U256,
        pub winner_stake_multiplier: ::ethers::core::types::U256,
        pub loser_stake_multiplier: ::ethers::core::types::U256,
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
        name = "ChallengeResolved",
        abi = "ChallengeResolved(address,uint256,uint256)"
    )]
    pub struct ChallengeResolvedFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub request_id: ::ethers::core::types::U256,
        pub challenge_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Dispute", abi = "Dispute(address,uint256,uint256,uint256)")]
    pub struct DisputeFilter {
        #[ethevent(indexed)]
        pub arbitrator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dispute_id: ::ethers::core::types::U256,
        pub meta_evidence_id: ::ethers::core::types::U256,
        pub evidence_group_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Evidence", abi = "Evidence(address,uint256,address,string)")]
    pub struct EvidenceFilter {
        #[ethevent(indexed)]
        pub arbitrator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub evidence_group_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub party: ::ethers::core::types::Address,
        pub evidence: ::std::string::String,
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
        name = "HasPaidAppealFee",
        abi = "HasPaidAppealFee(address,uint256,uint8)"
    )]
    pub struct HasPaidAppealFeeFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub challenge_id: ::ethers::core::types::U256,
        pub side: u8,
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
    #[ethevent(name = "MetaEvidence", abi = "MetaEvidence(uint256,string)")]
    pub struct MetaEvidenceFilter {
        #[ethevent(indexed)]
        pub meta_evidence_id: ::ethers::core::types::U256,
        pub evidence: ::std::string::String,
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
    #[ethevent(name = "ReapplySubmission", abi = "ReapplySubmission(address,uint256)")]
    pub struct ReapplySubmissionFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
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
        name = "RemoveSubmission",
        abi = "RemoveSubmission(address,address,uint256)"
    )]
    pub struct RemoveSubmissionFilter {
        #[ethevent(indexed)]
        pub requester: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Ruling", abi = "Ruling(address,uint256,uint256)")]
    pub struct RulingFilter {
        #[ethevent(indexed)]
        pub arbitrator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dispute_id: ::ethers::core::types::U256,
        pub ruling: ::ethers::core::types::U256,
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
        name = "SubmissionChallenged",
        abi = "SubmissionChallenged(address,uint256,uint256)"
    )]
    pub struct SubmissionChallengedFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub request_id: ::ethers::core::types::U256,
        pub challenge_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "VouchAdded", abi = "VouchAdded(address,address)")]
    pub struct VouchAddedFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub voucher: ::ethers::core::types::Address,
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
    #[ethevent(name = "VouchRemoved", abi = "VouchRemoved(address,address)")]
    pub struct VouchRemovedFilter {
        #[ethevent(indexed)]
        pub submission_id: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub voucher: ::ethers::core::types::Address,
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
    pub enum ProofOfHumanityEvents {
        AddSubmissionFilter(AddSubmissionFilter),
        AppealContributionFilter(AppealContributionFilter),
        ArbitratorCompleteFilter(ArbitratorCompleteFilter),
        ChallengeResolvedFilter(ChallengeResolvedFilter),
        DisputeFilter(DisputeFilter),
        EvidenceFilter(EvidenceFilter),
        HasPaidAppealFeeFilter(HasPaidAppealFeeFilter),
        MetaEvidenceFilter(MetaEvidenceFilter),
        ReapplySubmissionFilter(ReapplySubmissionFilter),
        RemoveSubmissionFilter(RemoveSubmissionFilter),
        RulingFilter(RulingFilter),
        SubmissionChallengedFilter(SubmissionChallengedFilter),
        VouchAddedFilter(VouchAddedFilter),
        VouchRemovedFilter(VouchRemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ProofOfHumanityEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddSubmissionFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::AddSubmissionFilter(decoded));
            }
            if let Ok(decoded) = AppealContributionFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::AppealContributionFilter(decoded));
            }
            if let Ok(decoded) = ArbitratorCompleteFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::ArbitratorCompleteFilter(decoded));
            }
            if let Ok(decoded) = ChallengeResolvedFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::ChallengeResolvedFilter(decoded));
            }
            if let Ok(decoded) = DisputeFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::DisputeFilter(decoded));
            }
            if let Ok(decoded) = EvidenceFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::EvidenceFilter(decoded));
            }
            if let Ok(decoded) = HasPaidAppealFeeFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::HasPaidAppealFeeFilter(decoded));
            }
            if let Ok(decoded) = MetaEvidenceFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::MetaEvidenceFilter(decoded));
            }
            if let Ok(decoded) = ReapplySubmissionFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::ReapplySubmissionFilter(decoded));
            }
            if let Ok(decoded) = RemoveSubmissionFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::RemoveSubmissionFilter(decoded));
            }
            if let Ok(decoded) = RulingFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::RulingFilter(decoded));
            }
            if let Ok(decoded) = SubmissionChallengedFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::SubmissionChallengedFilter(decoded));
            }
            if let Ok(decoded) = VouchAddedFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::VouchAddedFilter(decoded));
            }
            if let Ok(decoded) = VouchRemovedFilter::decode_log(log) {
                return Ok(ProofOfHumanityEvents::VouchRemovedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ProofOfHumanityEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddSubmissionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AppealContributionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ArbitratorCompleteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChallengeResolvedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DisputeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EvidenceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasPaidAppealFeeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MetaEvidenceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReapplySubmissionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveSubmissionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RulingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmissionChallengedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VouchAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VouchRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddSubmissionFilter> for ProofOfHumanityEvents {
        fn from(value: AddSubmissionFilter) -> Self {
            Self::AddSubmissionFilter(value)
        }
    }
    impl ::core::convert::From<AppealContributionFilter> for ProofOfHumanityEvents {
        fn from(value: AppealContributionFilter) -> Self {
            Self::AppealContributionFilter(value)
        }
    }
    impl ::core::convert::From<ArbitratorCompleteFilter> for ProofOfHumanityEvents {
        fn from(value: ArbitratorCompleteFilter) -> Self {
            Self::ArbitratorCompleteFilter(value)
        }
    }
    impl ::core::convert::From<ChallengeResolvedFilter> for ProofOfHumanityEvents {
        fn from(value: ChallengeResolvedFilter) -> Self {
            Self::ChallengeResolvedFilter(value)
        }
    }
    impl ::core::convert::From<DisputeFilter> for ProofOfHumanityEvents {
        fn from(value: DisputeFilter) -> Self {
            Self::DisputeFilter(value)
        }
    }
    impl ::core::convert::From<EvidenceFilter> for ProofOfHumanityEvents {
        fn from(value: EvidenceFilter) -> Self {
            Self::EvidenceFilter(value)
        }
    }
    impl ::core::convert::From<HasPaidAppealFeeFilter> for ProofOfHumanityEvents {
        fn from(value: HasPaidAppealFeeFilter) -> Self {
            Self::HasPaidAppealFeeFilter(value)
        }
    }
    impl ::core::convert::From<MetaEvidenceFilter> for ProofOfHumanityEvents {
        fn from(value: MetaEvidenceFilter) -> Self {
            Self::MetaEvidenceFilter(value)
        }
    }
    impl ::core::convert::From<ReapplySubmissionFilter> for ProofOfHumanityEvents {
        fn from(value: ReapplySubmissionFilter) -> Self {
            Self::ReapplySubmissionFilter(value)
        }
    }
    impl ::core::convert::From<RemoveSubmissionFilter> for ProofOfHumanityEvents {
        fn from(value: RemoveSubmissionFilter) -> Self {
            Self::RemoveSubmissionFilter(value)
        }
    }
    impl ::core::convert::From<RulingFilter> for ProofOfHumanityEvents {
        fn from(value: RulingFilter) -> Self {
            Self::RulingFilter(value)
        }
    }
    impl ::core::convert::From<SubmissionChallengedFilter> for ProofOfHumanityEvents {
        fn from(value: SubmissionChallengedFilter) -> Self {
            Self::SubmissionChallengedFilter(value)
        }
    }
    impl ::core::convert::From<VouchAddedFilter> for ProofOfHumanityEvents {
        fn from(value: VouchAddedFilter) -> Self {
            Self::VouchAddedFilter(value)
        }
    }
    impl ::core::convert::From<VouchRemovedFilter> for ProofOfHumanityEvents {
        fn from(value: VouchRemovedFilter) -> Self {
            Self::VouchRemovedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addSubmission` function with signature `addSubmission(string,string)` and selector `0x4690d55a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addSubmission", abi = "addSubmission(string,string)")]
    pub struct AddSubmissionCall {
        pub evidence: ::std::string::String,
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `addSubmissionManually` function with signature `addSubmissionManually(address[],string[],string[])` and selector `0x61b90541`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "addSubmissionManually",
        abi = "addSubmissionManually(address[],string[],string[])"
    )]
    pub struct AddSubmissionManuallyCall {
        pub submission_i_ds: ::std::vec::Vec<::ethers::core::types::Address>,
        pub evidence: ::std::vec::Vec<::std::string::String>,
        pub names: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `addVouch` function with signature `addVouch(address)` and selector `0x32fe596f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addVouch", abi = "addVouch(address)")]
    pub struct AddVouchCall {
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `arbitratorDataList` function with signature `arbitratorDataList(uint256)` and selector `0xec0e71ba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "arbitratorDataList", abi = "arbitratorDataList(uint256)")]
    pub struct ArbitratorDataListCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `arbitratorDisputeIDToDisputeData` function with signature `arbitratorDisputeIDToDisputeData(address,uint256)` and selector `0xdd254cd3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "arbitratorDisputeIDToDisputeData",
        abi = "arbitratorDisputeIDToDisputeData(address,uint256)"
    )]
    pub struct ArbitratorDisputeIDToDisputeDataCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `challengePeriodDuration` function with signature `challengePeriodDuration()` and selector `0x0082a36d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "challengePeriodDuration", abi = "challengePeriodDuration()")]
    pub struct ChallengePeriodDurationCall;
    ///Container type for all input parameters for the `challengeRequest` function with signature `challengeRequest(address,uint8,address,string)` and selector `0xf40e0aed`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "challengeRequest",
        abi = "challengeRequest(address,uint8,address,string)"
    )]
    pub struct ChallengeRequestCall {
        pub submission_id: ::ethers::core::types::Address,
        pub reason: u8,
        pub duplicate_id: ::ethers::core::types::Address,
        pub evidence: ::std::string::String,
    }
    ///Container type for all input parameters for the `changeArbitrator` function with signature `changeArbitrator(address,bytes)` and selector `0xba7079ca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "changeArbitrator", abi = "changeArbitrator(address,bytes)")]
    pub struct ChangeArbitratorCall {
        pub arbitrator: ::ethers::core::types::Address,
        pub arbitrator_extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `changeDurations` function with signature `changeDurations(uint64,uint64,uint64)` and selector `0x26bafe5f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "changeDurations", abi = "changeDurations(uint64,uint64,uint64)")]
    pub struct ChangeDurationsCall {
        pub submission_duration: u64,
        pub renewal_period_duration: u64,
        pub challenge_period_duration: u64,
    }
    ///Container type for all input parameters for the `changeGovernor` function with signature `changeGovernor(address)` and selector `0xe4c0aaf4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "changeGovernor", abi = "changeGovernor(address)")]
    pub struct ChangeGovernorCall {
        pub governor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `changeLoserStakeMultiplier` function with signature `changeLoserStakeMultiplier(uint256)` and selector `0x92239dff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "changeLoserStakeMultiplier",
        abi = "changeLoserStakeMultiplier(uint256)"
    )]
    pub struct ChangeLoserStakeMultiplierCall {
        pub loser_stake_multiplier: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `changeMetaEvidence` function with signature `changeMetaEvidence(string,string)` and selector `0xd706be31`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "changeMetaEvidence", abi = "changeMetaEvidence(string,string)")]
    pub struct ChangeMetaEvidenceCall {
        pub registration_meta_evidence: ::std::string::String,
        pub clearing_meta_evidence: ::std::string::String,
    }
    ///Container type for all input parameters for the `changeRequiredNumberOfVouches` function with signature `changeRequiredNumberOfVouches(uint64)` and selector `0xf65ab1be`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "changeRequiredNumberOfVouches",
        abi = "changeRequiredNumberOfVouches(uint64)"
    )]
    pub struct ChangeRequiredNumberOfVouchesCall {
        pub required_number_of_vouches: u64,
    }
    ///Container type for all input parameters for the `changeSharedStakeMultiplier` function with signature `changeSharedStakeMultiplier(uint256)` and selector `0x12ce3525`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "changeSharedStakeMultiplier",
        abi = "changeSharedStakeMultiplier(uint256)"
    )]
    pub struct ChangeSharedStakeMultiplierCall {
        pub shared_stake_multiplier: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `changeStateToPending` function with signature `changeStateToPending(address,address[],bytes[],uint256[])` and selector `0xb4dfe93d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "changeStateToPending",
        abi = "changeStateToPending(address,address[],bytes[],uint256[])"
    )]
    pub struct ChangeStateToPendingCall {
        pub submission_id: ::ethers::core::types::Address,
        pub vouches: ::std::vec::Vec<::ethers::core::types::Address>,
        pub signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub expiration_timestamps: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `changeSubmissionBaseDeposit` function with signature `changeSubmissionBaseDeposit(uint256)` and selector `0x33e5d047`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "changeSubmissionBaseDeposit",
        abi = "changeSubmissionBaseDeposit(uint256)"
    )]
    pub struct ChangeSubmissionBaseDepositCall {
        pub submission_base_deposit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `changeWinnerStakeMultiplier` function with signature `changeWinnerStakeMultiplier(uint256)` and selector `0xadc7faba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "changeWinnerStakeMultiplier",
        abi = "changeWinnerStakeMultiplier(uint256)"
    )]
    pub struct ChangeWinnerStakeMultiplierCall {
        pub winner_stake_multiplier: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkRequestDuplicates` function with signature `checkRequestDuplicates(address,uint256,address)` and selector `0x2e848506`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "checkRequestDuplicates",
        abi = "checkRequestDuplicates(address,uint256,address)"
    )]
    pub struct CheckRequestDuplicatesCall {
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
        pub duplicate_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `executeRequest` function with signature `executeRequest(address)` and selector `0x6e98762d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeRequest", abi = "executeRequest(address)")]
    pub struct ExecuteRequestCall {
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fundAppeal` function with signature `fundAppeal(address,uint256,uint8)` and selector `0xd7e9f178`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fundAppeal", abi = "fundAppeal(address,uint256,uint8)")]
    pub struct FundAppealCall {
        pub submission_id: ::ethers::core::types::Address,
        pub challenge_id: ::ethers::core::types::U256,
        pub side: u8,
    }
    ///Container type for all input parameters for the `fundSubmission` function with signature `fundSubmission(address)` and selector `0xa27456bb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fundSubmission", abi = "fundSubmission(address)")]
    pub struct FundSubmissionCall {
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getArbitratorDataListCount` function with signature `getArbitratorDataListCount()` and selector `0x90d7c13c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getArbitratorDataListCount", abi = "getArbitratorDataListCount()")]
    pub struct GetArbitratorDataListCountCall;
    ///Container type for all input parameters for the `getChallengeInfo` function with signature `getChallengeInfo(address,uint256,uint256)` and selector `0xd64240de`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getChallengeInfo",
        abi = "getChallengeInfo(address,uint256,uint256)"
    )]
    pub struct GetChallengeInfoCall {
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
        pub challenge_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getContributions` function with signature `getContributions(address,uint256,uint256,uint256,address)` and selector `0x3a8363c2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getContributions",
        abi = "getContributions(address,uint256,uint256,uint256,address)"
    )]
    pub struct GetContributionsCall {
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
        pub challenge_id: ::ethers::core::types::U256,
        pub round: ::ethers::core::types::U256,
        pub contributor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getNumberOfVouches` function with signature `getNumberOfVouches(address,uint256)` and selector `0xdeb8f707`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getNumberOfVouches", abi = "getNumberOfVouches(address,uint256)")]
    pub struct GetNumberOfVouchesCall {
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRequestInfo` function with signature `getRequestInfo(address,uint256)` and selector `0x6e112409`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRequestInfo", abi = "getRequestInfo(address,uint256)")]
    pub struct GetRequestInfoCall {
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoundInfo` function with signature `getRoundInfo(address,uint256,uint256,uint256)` and selector `0xa84dc70e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getRoundInfo",
        abi = "getRoundInfo(address,uint256,uint256,uint256)"
    )]
    pub struct GetRoundInfoCall {
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
        pub challenge_id: ::ethers::core::types::U256,
        pub round: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSubmissionInfo` function with signature `getSubmissionInfo(address)` and selector `0x97973043`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSubmissionInfo", abi = "getSubmissionInfo(address)")]
    pub struct GetSubmissionInfoCall {
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `governor` function with signature `governor()` and selector `0x0c340a24`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "governor", abi = "governor()")]
    pub struct GovernorCall;
    ///Container type for all input parameters for the `isRegistered` function with signature `isRegistered(address)` and selector `0xc3c5a547`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isRegistered", abi = "isRegistered(address)")]
    pub struct IsRegisteredCall {
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `loserStakeMultiplier` function with signature `loserStakeMultiplier()` and selector `0x1d512085`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "loserStakeMultiplier", abi = "loserStakeMultiplier()")]
    pub struct LoserStakeMultiplierCall;
    ///Container type for all input parameters for the `processVouches` function with signature `processVouches(address,uint256,uint256)` and selector `0x649a08bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "processVouches", abi = "processVouches(address,uint256,uint256)")]
    pub struct ProcessVouchesCall {
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
        pub iterations: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `reapplySubmission` function with signature `reapplySubmission(string,string)` and selector `0x5a9ef341`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reapplySubmission", abi = "reapplySubmission(string,string)")]
    pub struct ReapplySubmissionCall {
        pub evidence: ::std::string::String,
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `removeSubmission` function with signature `removeSubmission(address,string)` and selector `0xf4934cdb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeSubmission", abi = "removeSubmission(address,string)")]
    pub struct RemoveSubmissionCall {
        pub submission_id: ::ethers::core::types::Address,
        pub evidence: ::std::string::String,
    }
    ///Container type for all input parameters for the `removeSubmissionManually` function with signature `removeSubmissionManually(address)` and selector `0xa6c6ecc9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "removeSubmissionManually",
        abi = "removeSubmissionManually(address)"
    )]
    pub struct RemoveSubmissionManuallyCall {
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeVouch` function with signature `removeVouch(address)` and selector `0x84d1c91a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeVouch", abi = "removeVouch(address)")]
    pub struct RemoveVouchCall {
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renewalPeriodDuration` function with signature `renewalPeriodDuration()` and selector `0x876c63d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renewalPeriodDuration", abi = "renewalPeriodDuration()")]
    pub struct RenewalPeriodDurationCall;
    ///Container type for all input parameters for the `requiredNumberOfVouches` function with signature `requiredNumberOfVouches()` and selector `0x2d9489c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "requiredNumberOfVouches", abi = "requiredNumberOfVouches()")]
    pub struct RequiredNumberOfVouchesCall;
    ///Container type for all input parameters for the `rule` function with signature `rule(uint256,uint256)` and selector `0x311a6c56`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rule", abi = "rule(uint256,uint256)")]
    pub struct RuleCall {
        pub dispute_id: ::ethers::core::types::U256,
        pub ruling: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sharedStakeMultiplier` function with signature `sharedStakeMultiplier()` and selector `0x41658341`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sharedStakeMultiplier", abi = "sharedStakeMultiplier()")]
    pub struct SharedStakeMultiplierCall;
    ///Container type for all input parameters for the `submissionBaseDeposit` function with signature `submissionBaseDeposit()` and selector `0xbb0b86ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "submissionBaseDeposit", abi = "submissionBaseDeposit()")]
    pub struct SubmissionBaseDepositCall;
    ///Container type for all input parameters for the `submissionCounter` function with signature `submissionCounter()` and selector `0x76c23ff1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "submissionCounter", abi = "submissionCounter()")]
    pub struct SubmissionCounterCall;
    ///Container type for all input parameters for the `submissionDuration` function with signature `submissionDuration()` and selector `0xf633c293`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "submissionDuration", abi = "submissionDuration()")]
    pub struct SubmissionDurationCall;
    ///Container type for all input parameters for the `submitEvidence` function with signature `submitEvidence(address,string)` and selector `0x5bb5e55b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "submitEvidence", abi = "submitEvidence(address,string)")]
    pub struct SubmitEvidenceCall {
        pub submission_id: ::ethers::core::types::Address,
        pub evidence: ::std::string::String,
    }
    ///Container type for all input parameters for the `vouches` function with signature `vouches(address,address)` and selector `0x0b337be6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "vouches", abi = "vouches(address,address)")]
    pub struct VouchesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `winnerStakeMultiplier` function with signature `winnerStakeMultiplier()` and selector `0x7b943383`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "winnerStakeMultiplier", abi = "winnerStakeMultiplier()")]
    pub struct WinnerStakeMultiplierCall;
    ///Container type for all input parameters for the `withdrawFeesAndRewards` function with signature `withdrawFeesAndRewards(address,address,uint256,uint256,uint256)` and selector `0x9a72e0b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "withdrawFeesAndRewards",
        abi = "withdrawFeesAndRewards(address,address,uint256,uint256,uint256)"
    )]
    pub struct WithdrawFeesAndRewardsCall {
        pub beneficiary: ::ethers::core::types::Address,
        pub submission_id: ::ethers::core::types::Address,
        pub request_id: ::ethers::core::types::U256,
        pub challenge_id: ::ethers::core::types::U256,
        pub round: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawSubmission` function with signature `withdrawSubmission()` and selector `0xce52b9f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawSubmission", abi = "withdrawSubmission()")]
    pub struct WithdrawSubmissionCall;
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
    pub enum ProofOfHumanityCalls {
        AddSubmission(AddSubmissionCall),
        AddSubmissionManually(AddSubmissionManuallyCall),
        AddVouch(AddVouchCall),
        ArbitratorDataList(ArbitratorDataListCall),
        ArbitratorDisputeIDToDisputeData(ArbitratorDisputeIDToDisputeDataCall),
        ChallengePeriodDuration(ChallengePeriodDurationCall),
        ChallengeRequest(ChallengeRequestCall),
        ChangeArbitrator(ChangeArbitratorCall),
        ChangeDurations(ChangeDurationsCall),
        ChangeGovernor(ChangeGovernorCall),
        ChangeLoserStakeMultiplier(ChangeLoserStakeMultiplierCall),
        ChangeMetaEvidence(ChangeMetaEvidenceCall),
        ChangeRequiredNumberOfVouches(ChangeRequiredNumberOfVouchesCall),
        ChangeSharedStakeMultiplier(ChangeSharedStakeMultiplierCall),
        ChangeStateToPending(ChangeStateToPendingCall),
        ChangeSubmissionBaseDeposit(ChangeSubmissionBaseDepositCall),
        ChangeWinnerStakeMultiplier(ChangeWinnerStakeMultiplierCall),
        CheckRequestDuplicates(CheckRequestDuplicatesCall),
        ExecuteRequest(ExecuteRequestCall),
        FundAppeal(FundAppealCall),
        FundSubmission(FundSubmissionCall),
        GetArbitratorDataListCount(GetArbitratorDataListCountCall),
        GetChallengeInfo(GetChallengeInfoCall),
        GetContributions(GetContributionsCall),
        GetNumberOfVouches(GetNumberOfVouchesCall),
        GetRequestInfo(GetRequestInfoCall),
        GetRoundInfo(GetRoundInfoCall),
        GetSubmissionInfo(GetSubmissionInfoCall),
        Governor(GovernorCall),
        IsRegistered(IsRegisteredCall),
        LoserStakeMultiplier(LoserStakeMultiplierCall),
        ProcessVouches(ProcessVouchesCall),
        ReapplySubmission(ReapplySubmissionCall),
        RemoveSubmission(RemoveSubmissionCall),
        RemoveSubmissionManually(RemoveSubmissionManuallyCall),
        RemoveVouch(RemoveVouchCall),
        RenewalPeriodDuration(RenewalPeriodDurationCall),
        RequiredNumberOfVouches(RequiredNumberOfVouchesCall),
        Rule(RuleCall),
        SharedStakeMultiplier(SharedStakeMultiplierCall),
        SubmissionBaseDeposit(SubmissionBaseDepositCall),
        SubmissionCounter(SubmissionCounterCall),
        SubmissionDuration(SubmissionDurationCall),
        SubmitEvidence(SubmitEvidenceCall),
        Vouches(VouchesCall),
        WinnerStakeMultiplier(WinnerStakeMultiplierCall),
        WithdrawFeesAndRewards(WithdrawFeesAndRewardsCall),
        WithdrawSubmission(WithdrawSubmissionCall),
    }
    impl ::ethers::core::abi::AbiDecode for ProofOfHumanityCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddSubmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddSubmission(decoded));
            }
            if let Ok(decoded) = <AddSubmissionManuallyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddSubmissionManually(decoded));
            }
            if let Ok(decoded) = <AddVouchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddVouch(decoded));
            }
            if let Ok(decoded) = <ArbitratorDataListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ArbitratorDataList(decoded));
            }
            if let Ok(decoded) = <ArbitratorDisputeIDToDisputeDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ArbitratorDisputeIDToDisputeData(decoded));
            }
            if let Ok(decoded) = <ChallengePeriodDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChallengePeriodDuration(decoded));
            }
            if let Ok(decoded) = <ChallengeRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChallengeRequest(decoded));
            }
            if let Ok(decoded) = <ChangeArbitratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeArbitrator(decoded));
            }
            if let Ok(decoded) = <ChangeDurationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeDurations(decoded));
            }
            if let Ok(decoded) = <ChangeGovernorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeGovernor(decoded));
            }
            if let Ok(decoded) = <ChangeLoserStakeMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeLoserStakeMultiplier(decoded));
            }
            if let Ok(decoded) = <ChangeMetaEvidenceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeMetaEvidence(decoded));
            }
            if let Ok(decoded) = <ChangeRequiredNumberOfVouchesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeRequiredNumberOfVouches(decoded));
            }
            if let Ok(decoded) = <ChangeSharedStakeMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeSharedStakeMultiplier(decoded));
            }
            if let Ok(decoded) = <ChangeStateToPendingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeStateToPending(decoded));
            }
            if let Ok(decoded) = <ChangeSubmissionBaseDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeSubmissionBaseDeposit(decoded));
            }
            if let Ok(decoded) = <ChangeWinnerStakeMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeWinnerStakeMultiplier(decoded));
            }
            if let Ok(decoded) = <CheckRequestDuplicatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckRequestDuplicates(decoded));
            }
            if let Ok(decoded) = <ExecuteRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRequest(decoded));
            }
            if let Ok(decoded) = <FundAppealCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FundAppeal(decoded));
            }
            if let Ok(decoded) = <FundSubmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FundSubmission(decoded));
            }
            if let Ok(decoded) = <GetArbitratorDataListCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetArbitratorDataListCount(decoded));
            }
            if let Ok(decoded) = <GetChallengeInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetChallengeInfo(decoded));
            }
            if let Ok(decoded) = <GetContributionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetContributions(decoded));
            }
            if let Ok(decoded) = <GetNumberOfVouchesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNumberOfVouches(decoded));
            }
            if let Ok(decoded) = <GetRequestInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRequestInfo(decoded));
            }
            if let Ok(decoded) = <GetRoundInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoundInfo(decoded));
            }
            if let Ok(decoded) = <GetSubmissionInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubmissionInfo(decoded));
            }
            if let Ok(decoded) = <GovernorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Governor(decoded));
            }
            if let Ok(decoded) = <IsRegisteredCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsRegistered(decoded));
            }
            if let Ok(decoded) = <LoserStakeMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LoserStakeMultiplier(decoded));
            }
            if let Ok(decoded) = <ProcessVouchesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessVouches(decoded));
            }
            if let Ok(decoded) = <ReapplySubmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReapplySubmission(decoded));
            }
            if let Ok(decoded) = <RemoveSubmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveSubmission(decoded));
            }
            if let Ok(decoded) = <RemoveSubmissionManuallyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveSubmissionManually(decoded));
            }
            if let Ok(decoded) = <RemoveVouchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveVouch(decoded));
            }
            if let Ok(decoded) = <RenewalPeriodDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenewalPeriodDuration(decoded));
            }
            if let Ok(decoded) = <RequiredNumberOfVouchesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequiredNumberOfVouches(decoded));
            }
            if let Ok(decoded) = <RuleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rule(decoded));
            }
            if let Ok(decoded) = <SharedStakeMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SharedStakeMultiplier(decoded));
            }
            if let Ok(decoded) = <SubmissionBaseDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmissionBaseDeposit(decoded));
            }
            if let Ok(decoded) = <SubmissionCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmissionCounter(decoded));
            }
            if let Ok(decoded) = <SubmissionDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmissionDuration(decoded));
            }
            if let Ok(decoded) = <SubmitEvidenceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitEvidence(decoded));
            }
            if let Ok(decoded) = <VouchesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Vouches(decoded));
            }
            if let Ok(decoded) = <WinnerStakeMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WinnerStakeMultiplier(decoded));
            }
            if let Ok(decoded) = <WithdrawFeesAndRewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawFeesAndRewards(decoded));
            }
            if let Ok(decoded) = <WithdrawSubmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawSubmission(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ProofOfHumanityCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddSubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddSubmissionManually(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddVouch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ArbitratorDataList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ArbitratorDisputeIDToDisputeData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChallengePeriodDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChallengeRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeArbitrator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeDurations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeGovernor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeLoserStakeMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeMetaEvidence(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeRequiredNumberOfVouches(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeSharedStakeMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeStateToPending(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeSubmissionBaseDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeWinnerStakeMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckRequestDuplicates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FundAppeal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FundSubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetArbitratorDataListCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChallengeInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetContributions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNumberOfVouches(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRequestInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubmissionInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Governor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsRegistered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LoserStakeMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessVouches(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReapplySubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveSubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveSubmissionManually(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveVouch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenewalPeriodDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequiredNumberOfVouches(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rule(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SharedStakeMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmissionBaseDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmissionCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmissionDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitEvidence(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vouches(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WinnerStakeMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawFeesAndRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawSubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ProofOfHumanityCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddSubmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSubmissionManually(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddVouch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbitratorDataList(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ArbitratorDisputeIDToDisputeData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChallengePeriodDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChallengeRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeArbitrator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeDurations(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeGovernor(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeLoserStakeMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeMetaEvidence(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeRequiredNumberOfVouches(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeSharedStakeMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeStateToPending(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeSubmissionBaseDeposit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeWinnerStakeMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckRequestDuplicates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::FundAppeal(element) => ::core::fmt::Display::fmt(element, f),
                Self::FundSubmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetArbitratorDataListCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChallengeInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetContributions(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNumberOfVouches(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRequestInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubmissionInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::Governor(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsRegistered(element) => ::core::fmt::Display::fmt(element, f),
                Self::LoserStakeMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessVouches(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReapplySubmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveSubmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveSubmissionManually(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveVouch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenewalPeriodDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequiredNumberOfVouches(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Rule(element) => ::core::fmt::Display::fmt(element, f),
                Self::SharedStakeMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmissionBaseDeposit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmissionCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmissionDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitEvidence(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vouches(element) => ::core::fmt::Display::fmt(element, f),
                Self::WinnerStakeMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFeesAndRewards(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawSubmission(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddSubmissionCall> for ProofOfHumanityCalls {
        fn from(value: AddSubmissionCall) -> Self {
            Self::AddSubmission(value)
        }
    }
    impl ::core::convert::From<AddSubmissionManuallyCall> for ProofOfHumanityCalls {
        fn from(value: AddSubmissionManuallyCall) -> Self {
            Self::AddSubmissionManually(value)
        }
    }
    impl ::core::convert::From<AddVouchCall> for ProofOfHumanityCalls {
        fn from(value: AddVouchCall) -> Self {
            Self::AddVouch(value)
        }
    }
    impl ::core::convert::From<ArbitratorDataListCall> for ProofOfHumanityCalls {
        fn from(value: ArbitratorDataListCall) -> Self {
            Self::ArbitratorDataList(value)
        }
    }
    impl ::core::convert::From<ArbitratorDisputeIDToDisputeDataCall>
    for ProofOfHumanityCalls {
        fn from(value: ArbitratorDisputeIDToDisputeDataCall) -> Self {
            Self::ArbitratorDisputeIDToDisputeData(value)
        }
    }
    impl ::core::convert::From<ChallengePeriodDurationCall> for ProofOfHumanityCalls {
        fn from(value: ChallengePeriodDurationCall) -> Self {
            Self::ChallengePeriodDuration(value)
        }
    }
    impl ::core::convert::From<ChallengeRequestCall> for ProofOfHumanityCalls {
        fn from(value: ChallengeRequestCall) -> Self {
            Self::ChallengeRequest(value)
        }
    }
    impl ::core::convert::From<ChangeArbitratorCall> for ProofOfHumanityCalls {
        fn from(value: ChangeArbitratorCall) -> Self {
            Self::ChangeArbitrator(value)
        }
    }
    impl ::core::convert::From<ChangeDurationsCall> for ProofOfHumanityCalls {
        fn from(value: ChangeDurationsCall) -> Self {
            Self::ChangeDurations(value)
        }
    }
    impl ::core::convert::From<ChangeGovernorCall> for ProofOfHumanityCalls {
        fn from(value: ChangeGovernorCall) -> Self {
            Self::ChangeGovernor(value)
        }
    }
    impl ::core::convert::From<ChangeLoserStakeMultiplierCall> for ProofOfHumanityCalls {
        fn from(value: ChangeLoserStakeMultiplierCall) -> Self {
            Self::ChangeLoserStakeMultiplier(value)
        }
    }
    impl ::core::convert::From<ChangeMetaEvidenceCall> for ProofOfHumanityCalls {
        fn from(value: ChangeMetaEvidenceCall) -> Self {
            Self::ChangeMetaEvidence(value)
        }
    }
    impl ::core::convert::From<ChangeRequiredNumberOfVouchesCall>
    for ProofOfHumanityCalls {
        fn from(value: ChangeRequiredNumberOfVouchesCall) -> Self {
            Self::ChangeRequiredNumberOfVouches(value)
        }
    }
    impl ::core::convert::From<ChangeSharedStakeMultiplierCall>
    for ProofOfHumanityCalls {
        fn from(value: ChangeSharedStakeMultiplierCall) -> Self {
            Self::ChangeSharedStakeMultiplier(value)
        }
    }
    impl ::core::convert::From<ChangeStateToPendingCall> for ProofOfHumanityCalls {
        fn from(value: ChangeStateToPendingCall) -> Self {
            Self::ChangeStateToPending(value)
        }
    }
    impl ::core::convert::From<ChangeSubmissionBaseDepositCall>
    for ProofOfHumanityCalls {
        fn from(value: ChangeSubmissionBaseDepositCall) -> Self {
            Self::ChangeSubmissionBaseDeposit(value)
        }
    }
    impl ::core::convert::From<ChangeWinnerStakeMultiplierCall>
    for ProofOfHumanityCalls {
        fn from(value: ChangeWinnerStakeMultiplierCall) -> Self {
            Self::ChangeWinnerStakeMultiplier(value)
        }
    }
    impl ::core::convert::From<CheckRequestDuplicatesCall> for ProofOfHumanityCalls {
        fn from(value: CheckRequestDuplicatesCall) -> Self {
            Self::CheckRequestDuplicates(value)
        }
    }
    impl ::core::convert::From<ExecuteRequestCall> for ProofOfHumanityCalls {
        fn from(value: ExecuteRequestCall) -> Self {
            Self::ExecuteRequest(value)
        }
    }
    impl ::core::convert::From<FundAppealCall> for ProofOfHumanityCalls {
        fn from(value: FundAppealCall) -> Self {
            Self::FundAppeal(value)
        }
    }
    impl ::core::convert::From<FundSubmissionCall> for ProofOfHumanityCalls {
        fn from(value: FundSubmissionCall) -> Self {
            Self::FundSubmission(value)
        }
    }
    impl ::core::convert::From<GetArbitratorDataListCountCall> for ProofOfHumanityCalls {
        fn from(value: GetArbitratorDataListCountCall) -> Self {
            Self::GetArbitratorDataListCount(value)
        }
    }
    impl ::core::convert::From<GetChallengeInfoCall> for ProofOfHumanityCalls {
        fn from(value: GetChallengeInfoCall) -> Self {
            Self::GetChallengeInfo(value)
        }
    }
    impl ::core::convert::From<GetContributionsCall> for ProofOfHumanityCalls {
        fn from(value: GetContributionsCall) -> Self {
            Self::GetContributions(value)
        }
    }
    impl ::core::convert::From<GetNumberOfVouchesCall> for ProofOfHumanityCalls {
        fn from(value: GetNumberOfVouchesCall) -> Self {
            Self::GetNumberOfVouches(value)
        }
    }
    impl ::core::convert::From<GetRequestInfoCall> for ProofOfHumanityCalls {
        fn from(value: GetRequestInfoCall) -> Self {
            Self::GetRequestInfo(value)
        }
    }
    impl ::core::convert::From<GetRoundInfoCall> for ProofOfHumanityCalls {
        fn from(value: GetRoundInfoCall) -> Self {
            Self::GetRoundInfo(value)
        }
    }
    impl ::core::convert::From<GetSubmissionInfoCall> for ProofOfHumanityCalls {
        fn from(value: GetSubmissionInfoCall) -> Self {
            Self::GetSubmissionInfo(value)
        }
    }
    impl ::core::convert::From<GovernorCall> for ProofOfHumanityCalls {
        fn from(value: GovernorCall) -> Self {
            Self::Governor(value)
        }
    }
    impl ::core::convert::From<IsRegisteredCall> for ProofOfHumanityCalls {
        fn from(value: IsRegisteredCall) -> Self {
            Self::IsRegistered(value)
        }
    }
    impl ::core::convert::From<LoserStakeMultiplierCall> for ProofOfHumanityCalls {
        fn from(value: LoserStakeMultiplierCall) -> Self {
            Self::LoserStakeMultiplier(value)
        }
    }
    impl ::core::convert::From<ProcessVouchesCall> for ProofOfHumanityCalls {
        fn from(value: ProcessVouchesCall) -> Self {
            Self::ProcessVouches(value)
        }
    }
    impl ::core::convert::From<ReapplySubmissionCall> for ProofOfHumanityCalls {
        fn from(value: ReapplySubmissionCall) -> Self {
            Self::ReapplySubmission(value)
        }
    }
    impl ::core::convert::From<RemoveSubmissionCall> for ProofOfHumanityCalls {
        fn from(value: RemoveSubmissionCall) -> Self {
            Self::RemoveSubmission(value)
        }
    }
    impl ::core::convert::From<RemoveSubmissionManuallyCall> for ProofOfHumanityCalls {
        fn from(value: RemoveSubmissionManuallyCall) -> Self {
            Self::RemoveSubmissionManually(value)
        }
    }
    impl ::core::convert::From<RemoveVouchCall> for ProofOfHumanityCalls {
        fn from(value: RemoveVouchCall) -> Self {
            Self::RemoveVouch(value)
        }
    }
    impl ::core::convert::From<RenewalPeriodDurationCall> for ProofOfHumanityCalls {
        fn from(value: RenewalPeriodDurationCall) -> Self {
            Self::RenewalPeriodDuration(value)
        }
    }
    impl ::core::convert::From<RequiredNumberOfVouchesCall> for ProofOfHumanityCalls {
        fn from(value: RequiredNumberOfVouchesCall) -> Self {
            Self::RequiredNumberOfVouches(value)
        }
    }
    impl ::core::convert::From<RuleCall> for ProofOfHumanityCalls {
        fn from(value: RuleCall) -> Self {
            Self::Rule(value)
        }
    }
    impl ::core::convert::From<SharedStakeMultiplierCall> for ProofOfHumanityCalls {
        fn from(value: SharedStakeMultiplierCall) -> Self {
            Self::SharedStakeMultiplier(value)
        }
    }
    impl ::core::convert::From<SubmissionBaseDepositCall> for ProofOfHumanityCalls {
        fn from(value: SubmissionBaseDepositCall) -> Self {
            Self::SubmissionBaseDeposit(value)
        }
    }
    impl ::core::convert::From<SubmissionCounterCall> for ProofOfHumanityCalls {
        fn from(value: SubmissionCounterCall) -> Self {
            Self::SubmissionCounter(value)
        }
    }
    impl ::core::convert::From<SubmissionDurationCall> for ProofOfHumanityCalls {
        fn from(value: SubmissionDurationCall) -> Self {
            Self::SubmissionDuration(value)
        }
    }
    impl ::core::convert::From<SubmitEvidenceCall> for ProofOfHumanityCalls {
        fn from(value: SubmitEvidenceCall) -> Self {
            Self::SubmitEvidence(value)
        }
    }
    impl ::core::convert::From<VouchesCall> for ProofOfHumanityCalls {
        fn from(value: VouchesCall) -> Self {
            Self::Vouches(value)
        }
    }
    impl ::core::convert::From<WinnerStakeMultiplierCall> for ProofOfHumanityCalls {
        fn from(value: WinnerStakeMultiplierCall) -> Self {
            Self::WinnerStakeMultiplier(value)
        }
    }
    impl ::core::convert::From<WithdrawFeesAndRewardsCall> for ProofOfHumanityCalls {
        fn from(value: WithdrawFeesAndRewardsCall) -> Self {
            Self::WithdrawFeesAndRewards(value)
        }
    }
    impl ::core::convert::From<WithdrawSubmissionCall> for ProofOfHumanityCalls {
        fn from(value: WithdrawSubmissionCall) -> Self {
            Self::WithdrawSubmission(value)
        }
    }
    ///Container type for all return fields from the `arbitratorDataList` function with signature `arbitratorDataList(uint256)` and selector `0xec0e71ba`
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
    pub struct ArbitratorDataListReturn {
        pub arbitrator: ::ethers::core::types::Address,
        pub meta_evidence_updates: u128,
        pub arbitrator_extra_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `arbitratorDisputeIDToDisputeData` function with signature `arbitratorDisputeIDToDisputeData(address,uint256)` and selector `0xdd254cd3`
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
    pub struct ArbitratorDisputeIDToDisputeDataReturn {
        pub challenge_id: u128,
        pub submission_id: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `challengePeriodDuration` function with signature `challengePeriodDuration()` and selector `0x0082a36d`
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
    pub struct ChallengePeriodDurationReturn(pub u64);
    ///Container type for all return fields from the `checkRequestDuplicates` function with signature `checkRequestDuplicates(address,uint256,address)` and selector `0x2e848506`
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
    pub struct CheckRequestDuplicatesReturn(pub bool);
    ///Container type for all return fields from the `getArbitratorDataListCount` function with signature `getArbitratorDataListCount()` and selector `0x90d7c13c`
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
    pub struct GetArbitratorDataListCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getChallengeInfo` function with signature `getChallengeInfo(address,uint256,uint256)` and selector `0xd64240de`
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
    pub struct GetChallengeInfoReturn {
        pub last_round_id: u16,
        pub challenger: ::ethers::core::types::Address,
        pub dispute_id: ::ethers::core::types::U256,
        pub ruling: u8,
        pub duplicate_submission_index: u64,
    }
    ///Container type for all return fields from the `getContributions` function with signature `getContributions(address,uint256,uint256,uint256,address)` and selector `0x3a8363c2`
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
    pub struct GetContributionsReturn {
        pub contributions: [::ethers::core::types::U256; 3],
    }
    ///Container type for all return fields from the `getNumberOfVouches` function with signature `getNumberOfVouches(address,uint256)` and selector `0xdeb8f707`
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
    pub struct GetNumberOfVouchesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRequestInfo` function with signature `getRequestInfo(address,uint256)` and selector `0x6e112409`
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
    pub struct GetRequestInfoReturn {
        pub disputed: bool,
        pub resolved: bool,
        pub requester_lost: bool,
        pub current_reason: u8,
        pub nb_parallel_disputes: u16,
        pub last_challenge_id: u16,
        pub arbitrator_data_id: u16,
        pub requester: ::ethers::core::types::Address,
        pub ultimate_challenger: ::ethers::core::types::Address,
        pub used_reasons: u8,
    }
    ///Container type for all return fields from the `getRoundInfo` function with signature `getRoundInfo(address,uint256,uint256,uint256)` and selector `0xa84dc70e`
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
    pub struct GetRoundInfoReturn {
        pub appealed: bool,
        pub paid_fees: [::ethers::core::types::U256; 3],
        pub side_funded: u8,
        pub fee_rewards: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getSubmissionInfo` function with signature `getSubmissionInfo(address)` and selector `0x97973043`
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
    pub struct GetSubmissionInfoReturn {
        pub status: u8,
        pub submission_time: u64,
        pub index: u64,
        pub registered: bool,
        pub has_vouched: bool,
        pub number_of_requests: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `governor` function with signature `governor()` and selector `0x0c340a24`
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
    pub struct GovernorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isRegistered` function with signature `isRegistered(address)` and selector `0xc3c5a547`
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
    pub struct IsRegisteredReturn(pub bool);
    ///Container type for all return fields from the `loserStakeMultiplier` function with signature `loserStakeMultiplier()` and selector `0x1d512085`
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
    pub struct LoserStakeMultiplierReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `renewalPeriodDuration` function with signature `renewalPeriodDuration()` and selector `0x876c63d4`
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
    pub struct RenewalPeriodDurationReturn(pub u64);
    ///Container type for all return fields from the `requiredNumberOfVouches` function with signature `requiredNumberOfVouches()` and selector `0x2d9489c6`
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
    pub struct RequiredNumberOfVouchesReturn(pub u64);
    ///Container type for all return fields from the `sharedStakeMultiplier` function with signature `sharedStakeMultiplier()` and selector `0x41658341`
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
    pub struct SharedStakeMultiplierReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submissionBaseDeposit` function with signature `submissionBaseDeposit()` and selector `0xbb0b86ff`
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
    pub struct SubmissionBaseDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submissionCounter` function with signature `submissionCounter()` and selector `0x76c23ff1`
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
    pub struct SubmissionCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submissionDuration` function with signature `submissionDuration()` and selector `0xf633c293`
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
    pub struct SubmissionDurationReturn(pub u64);
    ///Container type for all return fields from the `vouches` function with signature `vouches(address,address)` and selector `0x0b337be6`
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
    pub struct VouchesReturn(pub bool);
    ///Container type for all return fields from the `winnerStakeMultiplier` function with signature `winnerStakeMultiplier()` and selector `0x7b943383`
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
    pub struct WinnerStakeMultiplierReturn(pub ::ethers::core::types::U256);
}
