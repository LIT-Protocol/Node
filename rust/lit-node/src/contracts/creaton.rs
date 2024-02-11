pub use creaton::*;
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
pub mod creaton {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/Creaton.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptSubscribe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptSubscribe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_address"),
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
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("afterAgreementCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "afterAgreementCreated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISuperToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementClass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cbdata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newCtx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("afterAgreementTerminated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "afterAgreementTerminated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISuperToken"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cbdata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newCtx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("afterAgreementUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "afterAgreementUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISuperToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementClass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cbdata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newCtx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeAgreementCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beforeAgreementCreated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("superToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISuperToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementClass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ctx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cbdata"),
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
                    ::std::borrow::ToOwned::to_owned("beforeAgreementTerminated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beforeAgreementTerminated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("superToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISuperToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementClass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                    name: ::std::borrow::ToOwned::to_owned("cbdata"),
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
                    ::std::borrow::ToOwned::to_owned("beforeAgreementUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beforeAgreementUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("superToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISuperToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementClass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("agreementId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                    name: ::std::borrow::ToOwned::to_owned("cbdata"),
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
                    ::std::borrow::ToOwned::to_owned("blockSubscription"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blockSubscription"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_address"),
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
                    ::std::borrow::ToOwned::to_owned("bulkAcceptSubscribe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "bulkAcceptSubscribe",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_addresses"),
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
                    ::std::borrow::ToOwned::to_owned("bulkBlockSubscription"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "bulkBlockSubscription",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_addresses"),
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
                    ::std::borrow::ToOwned::to_owned("creator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("creator"),
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
                    ::std::borrow::ToOwned::to_owned("description"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("description"),
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
                    ::std::borrow::ToOwned::to_owned("getSubscriberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSubscriberCount"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("host"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cfa"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("acceptedToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_creator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_subscriptionPrice",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nftName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nftSymbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isTrustedForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isTrustedForwarder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forwarder"),
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
                    ::std::borrow::ToOwned::to_owned("like"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("like"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approvalEnum"),
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
                    ::std::borrow::ToOwned::to_owned("percentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("percentage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("num"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("percent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int96"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("postNFT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("postNFT"),
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
                    ::std::borrow::ToOwned::to_owned("recoverTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recoverTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
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
                    ::std::borrow::ToOwned::to_owned("requestSubscribe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestSubscribe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pubKey"),
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
                    ::std::borrow::ToOwned::to_owned("revokeSubscribe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeSubscribe"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("subscribers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("subscribers"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CreatorV1.Status"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("subscriptionPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("subscriptionPrice"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trustedForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trustedForwarder"),
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
                    ::std::borrow::ToOwned::to_owned("upload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_metadataURI"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dataJSON"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contentType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CreatorV1.Type"),
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
                    ::std::borrow::ToOwned::to_owned("versionRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("versionRecipient"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawEth"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Like"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Like"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                                    name: ::std::borrow::ToOwned::to_owned("approval"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPost"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewPost"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("jsonData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("contentType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PostContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PostContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nftContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SubscriberEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SubscriberEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CREATON_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Creaton<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Creaton<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Creaton<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Creaton<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Creaton<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Creaton)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Creaton<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CREATON_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `acceptSubscribe` (0xa0d2aea3) function
        pub fn accept_subscribe(
            &self,
            address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 210, 174, 163], address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `afterAgreementCreated` (0xd86ed3e5) function
        pub fn after_agreement_created(
            &self,
            p0: ::ethers::core::types::Address,
            agreement_class: ::ethers::core::types::Address,
            agreement_id: [u8; 32],
            p3: ::ethers::core::types::Bytes,
            cbdata: ::ethers::core::types::Bytes,
            ctx: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [216, 110, 211, 229],
                    (p0, agreement_class, agreement_id, p3, cbdata, ctx),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `afterAgreementTerminated` (0x53c11f99) function
        pub fn after_agreement_terminated(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: [u8; 32],
            p3: ::ethers::core::types::Bytes,
            cbdata: ::ethers::core::types::Bytes,
            ctx: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([83, 193, 31, 153], (p0, p1, p2, p3, cbdata, ctx))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `afterAgreementUpdated` (0x230dbd29) function
        pub fn after_agreement_updated(
            &self,
            p0: ::ethers::core::types::Address,
            agreement_class: ::ethers::core::types::Address,
            agreement_id: [u8; 32],
            p3: ::ethers::core::types::Bytes,
            cbdata: ::ethers::core::types::Bytes,
            ctx: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [35, 13, 189, 41],
                    (p0, agreement_class, agreement_id, p3, cbdata, ctx),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beforeAgreementCreated` (0x30d9c915) function
        pub fn before_agreement_created(
            &self,
            super_token: ::ethers::core::types::Address,
            agreement_class: ::ethers::core::types::Address,
            p2: [u8; 32],
            p3: ::ethers::core::types::Bytes,
            ctx: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [48, 217, 201, 21],
                    (super_token, agreement_class, p2, p3, ctx),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beforeAgreementTerminated` (0x5f9e7d77) function
        pub fn before_agreement_terminated(
            &self,
            super_token: ::ethers::core::types::Address,
            agreement_class: ::ethers::core::types::Address,
            p2: [u8; 32],
            p3: ::ethers::core::types::Bytes,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [95, 158, 125, 119],
                    (super_token, agreement_class, p2, p3, p4),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beforeAgreementUpdated` (0x884d1f40) function
        pub fn before_agreement_updated(
            &self,
            super_token: ::ethers::core::types::Address,
            agreement_class: ::ethers::core::types::Address,
            agreement_id: [u8; 32],
            p3: ::ethers::core::types::Bytes,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [136, 77, 31, 64],
                    (super_token, agreement_class, agreement_id, p3, p4),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockSubscription` (0x9e87ad21) function
        pub fn block_subscription(
            &self,
            address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 135, 173, 33], address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bulkAcceptSubscribe` (0x32725c89) function
        pub fn bulk_accept_subscribe(
            &self,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 114, 92, 137], addresses)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bulkBlockSubscription` (0xb73410d1) function
        pub fn bulk_block_subscription(
            &self,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 52, 16, 209], addresses)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `creator` (0x02d05d3f) function
        pub fn creator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([2, 208, 93, 63], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `description` (0x7284e416) function
        pub fn description(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubscriberCount` (0x17e703ab) function
        pub fn get_subscriber_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([23, 231, 3, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x93bdfdd8) function
        pub fn initialize(
            &self,
            host: ::ethers::core::types::Address,
            cfa: ::ethers::core::types::Address,
            accepted_token: ::ethers::core::types::Address,
            creator: ::ethers::core::types::Address,
            description: ::std::string::String,
            subscription_price: ::ethers::core::types::U256,
            nft_name: ::std::string::String,
            nft_symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [147, 189, 253, 216],
                    (
                        host,
                        cfa,
                        accepted_token,
                        creator,
                        description,
                        subscription_price,
                        nft_name,
                        nft_symbol,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTrustedForwarder` (0x572b6c05) function
        pub fn is_trusted_forwarder(
            &self,
            forwarder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([87, 43, 108, 5], forwarder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `like` (0xb2345458) function
        pub fn like(
            &self,
            token_id: ::ethers::core::types::U256,
            approval_enum: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 52, 84, 88], (token_id, approval_enum))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `percentage` (0x123ff984) function
        pub fn percentage(
            &self,
            num: i128,
            percent: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([18, 63, 249, 132], (num, percent))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `postNFT` (0x88f9c946) function
        pub fn post_nft(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([136, 249, 201, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recoverTokens` (0x16114acd) function
        pub fn recover_tokens(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 17, 74, 205], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestSubscribe` (0x3d3acb1f) function
        pub fn request_subscribe(
            &self,
            pub_key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 58, 203, 31], pub_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeSubscribe` (0xf9680f82) function
        pub fn revoke_subscribe(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 104, 15, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `subscribers` (0x5745ae28) function
        pub fn subscribers(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([87, 69, 174, 40], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `subscriptionPrice` (0xbdc8e54c) function
        pub fn subscription_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([189, 200, 229, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trustedForwarder` (0x7da0a877) function
        pub fn trusted_forwarder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 160, 168, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upload` (0xcbd5497c) function
        pub fn upload(
            &self,
            metadata_uri: ::std::string::String,
            data_json: ::std::string::String,
            content_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 213, 73, 124],
                    (metadata_uri, data_json, content_type),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `versionRecipient` (0x486ff0cd) function
        pub fn version_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([72, 111, 240, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawEth` (0xa0ef91df) function
        pub fn withdraw_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 239, 145, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Like` event
        pub fn like_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LikeFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewPost` event
        pub fn new_post_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewPostFilter> {
            self.0.event()
        }
        ///Gets the contract's `PostContract` event
        pub fn post_contract_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PostContractFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SubscriberEvent` event
        pub fn subscriber_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SubscriberEventFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CreatonEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Creaton<M> {
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
    #[ethevent(name = "Like", abi = "Like(address,uint256,uint8)")]
    pub struct LikeFilter {
        pub user: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub approval: u8,
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
    #[ethevent(name = "NewPost", abi = "NewPost(uint256,string,uint8)")]
    pub struct NewPostFilter {
        pub token_id: ::ethers::core::types::U256,
        pub json_data: ::std::string::String,
        pub content_type: u8,
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
    #[ethevent(name = "PostContract", abi = "PostContract(address)")]
    pub struct PostContractFilter {
        pub nft_contract: ::ethers::core::types::Address,
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
    #[ethevent(name = "SubscriberEvent", abi = "SubscriberEvent(address,string,uint8)")]
    pub struct SubscriberEventFilter {
        pub user: ::ethers::core::types::Address,
        pub pub_key: ::std::string::String,
        pub status: u8,
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
    pub enum CreatonEvents {
        LikeFilter(LikeFilter),
        NewPostFilter(NewPostFilter),
        PostContractFilter(PostContractFilter),
        SubscriberEventFilter(SubscriberEventFilter),
    }
    impl ::ethers::contract::EthLogDecode for CreatonEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LikeFilter::decode_log(log) {
                return Ok(CreatonEvents::LikeFilter(decoded));
            }
            if let Ok(decoded) = NewPostFilter::decode_log(log) {
                return Ok(CreatonEvents::NewPostFilter(decoded));
            }
            if let Ok(decoded) = PostContractFilter::decode_log(log) {
                return Ok(CreatonEvents::PostContractFilter(decoded));
            }
            if let Ok(decoded) = SubscriberEventFilter::decode_log(log) {
                return Ok(CreatonEvents::SubscriberEventFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CreatonEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LikeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewPostFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PostContractFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubscriberEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<LikeFilter> for CreatonEvents {
        fn from(value: LikeFilter) -> Self {
            Self::LikeFilter(value)
        }
    }
    impl ::core::convert::From<NewPostFilter> for CreatonEvents {
        fn from(value: NewPostFilter) -> Self {
            Self::NewPostFilter(value)
        }
    }
    impl ::core::convert::From<PostContractFilter> for CreatonEvents {
        fn from(value: PostContractFilter) -> Self {
            Self::PostContractFilter(value)
        }
    }
    impl ::core::convert::From<SubscriberEventFilter> for CreatonEvents {
        fn from(value: SubscriberEventFilter) -> Self {
            Self::SubscriberEventFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptSubscribe` function with signature `acceptSubscribe(address)` and selector `0xa0d2aea3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "acceptSubscribe", abi = "acceptSubscribe(address)")]
    pub struct AcceptSubscribeCall {
        pub address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `afterAgreementCreated` function with signature `afterAgreementCreated(address,address,bytes32,bytes,bytes,bytes)` and selector `0xd86ed3e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "afterAgreementCreated",
        abi = "afterAgreementCreated(address,address,bytes32,bytes,bytes,bytes)"
    )]
    pub struct AfterAgreementCreatedCall {
        pub p0: ::ethers::core::types::Address,
        pub agreement_class: ::ethers::core::types::Address,
        pub agreement_id: [u8; 32],
        pub p3: ::ethers::core::types::Bytes,
        pub cbdata: ::ethers::core::types::Bytes,
        pub ctx: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `afterAgreementTerminated` function with signature `afterAgreementTerminated(address,address,bytes32,bytes,bytes,bytes)` and selector `0x53c11f99`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "afterAgreementTerminated",
        abi = "afterAgreementTerminated(address,address,bytes32,bytes,bytes,bytes)"
    )]
    pub struct AfterAgreementTerminatedCall {
        pub p0: ::ethers::core::types::Address,
        pub p1: ::ethers::core::types::Address,
        pub p2: [u8; 32],
        pub p3: ::ethers::core::types::Bytes,
        pub cbdata: ::ethers::core::types::Bytes,
        pub ctx: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `afterAgreementUpdated` function with signature `afterAgreementUpdated(address,address,bytes32,bytes,bytes,bytes)` and selector `0x230dbd29`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "afterAgreementUpdated",
        abi = "afterAgreementUpdated(address,address,bytes32,bytes,bytes,bytes)"
    )]
    pub struct AfterAgreementUpdatedCall {
        pub p0: ::ethers::core::types::Address,
        pub agreement_class: ::ethers::core::types::Address,
        pub agreement_id: [u8; 32],
        pub p3: ::ethers::core::types::Bytes,
        pub cbdata: ::ethers::core::types::Bytes,
        pub ctx: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `beforeAgreementCreated` function with signature `beforeAgreementCreated(address,address,bytes32,bytes,bytes)` and selector `0x30d9c915`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "beforeAgreementCreated",
        abi = "beforeAgreementCreated(address,address,bytes32,bytes,bytes)"
    )]
    pub struct BeforeAgreementCreatedCall {
        pub super_token: ::ethers::core::types::Address,
        pub agreement_class: ::ethers::core::types::Address,
        pub p2: [u8; 32],
        pub p3: ::ethers::core::types::Bytes,
        pub ctx: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `beforeAgreementTerminated` function with signature `beforeAgreementTerminated(address,address,bytes32,bytes,bytes)` and selector `0x5f9e7d77`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "beforeAgreementTerminated",
        abi = "beforeAgreementTerminated(address,address,bytes32,bytes,bytes)"
    )]
    pub struct BeforeAgreementTerminatedCall {
        pub super_token: ::ethers::core::types::Address,
        pub agreement_class: ::ethers::core::types::Address,
        pub p2: [u8; 32],
        pub p3: ::ethers::core::types::Bytes,
        pub p4: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `beforeAgreementUpdated` function with signature `beforeAgreementUpdated(address,address,bytes32,bytes,bytes)` and selector `0x884d1f40`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "beforeAgreementUpdated",
        abi = "beforeAgreementUpdated(address,address,bytes32,bytes,bytes)"
    )]
    pub struct BeforeAgreementUpdatedCall {
        pub super_token: ::ethers::core::types::Address,
        pub agreement_class: ::ethers::core::types::Address,
        pub agreement_id: [u8; 32],
        pub p3: ::ethers::core::types::Bytes,
        pub p4: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `blockSubscription` function with signature `blockSubscription(address)` and selector `0x9e87ad21`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "blockSubscription", abi = "blockSubscription(address)")]
    pub struct BlockSubscriptionCall {
        pub address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `bulkAcceptSubscribe` function with signature `bulkAcceptSubscribe(address[])` and selector `0x32725c89`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bulkAcceptSubscribe", abi = "bulkAcceptSubscribe(address[])")]
    pub struct BulkAcceptSubscribeCall {
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `bulkBlockSubscription` function with signature `bulkBlockSubscription(address[])` and selector `0xb73410d1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bulkBlockSubscription", abi = "bulkBlockSubscription(address[])")]
    pub struct BulkBlockSubscriptionCall {
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `creator` function with signature `creator()` and selector `0x02d05d3f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "creator", abi = "creator()")]
    pub struct CreatorCall;
    ///Container type for all input parameters for the `description` function with signature `description()` and selector `0x7284e416`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    ///Container type for all input parameters for the `getSubscriberCount` function with signature `getSubscriberCount()` and selector `0x17e703ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSubscriberCount", abi = "getSubscriberCount()")]
    pub struct GetSubscriberCountCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,string,uint256,string,string)` and selector `0x93bdfdd8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "initialize",
        abi = "initialize(address,address,address,address,string,uint256,string,string)"
    )]
    pub struct InitializeCall {
        pub host: ::ethers::core::types::Address,
        pub cfa: ::ethers::core::types::Address,
        pub accepted_token: ::ethers::core::types::Address,
        pub creator: ::ethers::core::types::Address,
        pub description: ::std::string::String,
        pub subscription_price: ::ethers::core::types::U256,
        pub nft_name: ::std::string::String,
        pub nft_symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `isTrustedForwarder` function with signature `isTrustedForwarder(address)` and selector `0x572b6c05`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isTrustedForwarder", abi = "isTrustedForwarder(address)")]
    pub struct IsTrustedForwarderCall {
        pub forwarder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `like` function with signature `like(uint256,uint256)` and selector `0xb2345458`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "like", abi = "like(uint256,uint256)")]
    pub struct LikeCall {
        pub token_id: ::ethers::core::types::U256,
        pub approval_enum: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `percentage` function with signature `percentage(int96,int96)` and selector `0x123ff984`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "percentage", abi = "percentage(int96,int96)")]
    pub struct PercentageCall {
        pub num: i128,
        pub percent: i128,
    }
    ///Container type for all input parameters for the `postNFT` function with signature `postNFT()` and selector `0x88f9c946`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "postNFT", abi = "postNFT()")]
    pub struct PostNFTCall;
    ///Container type for all input parameters for the `recoverTokens` function with signature `recoverTokens(address)` and selector `0x16114acd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recoverTokens", abi = "recoverTokens(address)")]
    pub struct RecoverTokensCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `requestSubscribe` function with signature `requestSubscribe(string)` and selector `0x3d3acb1f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "requestSubscribe", abi = "requestSubscribe(string)")]
    pub struct RequestSubscribeCall {
        pub pub_key: ::std::string::String,
    }
    ///Container type for all input parameters for the `revokeSubscribe` function with signature `revokeSubscribe()` and selector `0xf9680f82`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokeSubscribe", abi = "revokeSubscribe()")]
    pub struct RevokeSubscribeCall;
    ///Container type for all input parameters for the `subscribers` function with signature `subscribers(address)` and selector `0x5745ae28`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "subscribers", abi = "subscribers(address)")]
    pub struct SubscribersCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `subscriptionPrice` function with signature `subscriptionPrice()` and selector `0xbdc8e54c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "subscriptionPrice", abi = "subscriptionPrice()")]
    pub struct SubscriptionPriceCall;
    ///Container type for all input parameters for the `trustedForwarder` function with signature `trustedForwarder()` and selector `0x7da0a877`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "trustedForwarder", abi = "trustedForwarder()")]
    pub struct TrustedForwarderCall;
    ///Container type for all input parameters for the `upload` function with signature `upload(string,string,uint8)` and selector `0xcbd5497c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "upload", abi = "upload(string,string,uint8)")]
    pub struct UploadCall {
        pub metadata_uri: ::std::string::String,
        pub data_json: ::std::string::String,
        pub content_type: u8,
    }
    ///Container type for all input parameters for the `versionRecipient` function with signature `versionRecipient()` and selector `0x486ff0cd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "versionRecipient", abi = "versionRecipient()")]
    pub struct VersionRecipientCall;
    ///Container type for all input parameters for the `withdrawEth` function with signature `withdrawEth()` and selector `0xa0ef91df`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawEth", abi = "withdrawEth()")]
    pub struct WithdrawEthCall;
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
    pub enum CreatonCalls {
        AcceptSubscribe(AcceptSubscribeCall),
        Admin(AdminCall),
        AfterAgreementCreated(AfterAgreementCreatedCall),
        AfterAgreementTerminated(AfterAgreementTerminatedCall),
        AfterAgreementUpdated(AfterAgreementUpdatedCall),
        BeforeAgreementCreated(BeforeAgreementCreatedCall),
        BeforeAgreementTerminated(BeforeAgreementTerminatedCall),
        BeforeAgreementUpdated(BeforeAgreementUpdatedCall),
        BlockSubscription(BlockSubscriptionCall),
        BulkAcceptSubscribe(BulkAcceptSubscribeCall),
        BulkBlockSubscription(BulkBlockSubscriptionCall),
        Creator(CreatorCall),
        Description(DescriptionCall),
        GetSubscriberCount(GetSubscriberCountCall),
        Initialize(InitializeCall),
        IsTrustedForwarder(IsTrustedForwarderCall),
        Like(LikeCall),
        Percentage(PercentageCall),
        PostNFT(PostNFTCall),
        RecoverTokens(RecoverTokensCall),
        RequestSubscribe(RequestSubscribeCall),
        RevokeSubscribe(RevokeSubscribeCall),
        Subscribers(SubscribersCall),
        SubscriptionPrice(SubscriptionPriceCall),
        TrustedForwarder(TrustedForwarderCall),
        Upload(UploadCall),
        VersionRecipient(VersionRecipientCall),
        WithdrawEth(WithdrawEthCall),
    }
    impl ::ethers::core::abi::AbiDecode for CreatonCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptSubscribeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptSubscribe(decoded));
            }
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) = <AfterAgreementCreatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AfterAgreementCreated(decoded));
            }
            if let Ok(decoded) = <AfterAgreementTerminatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AfterAgreementTerminated(decoded));
            }
            if let Ok(decoded) = <AfterAgreementUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AfterAgreementUpdated(decoded));
            }
            if let Ok(decoded) = <BeforeAgreementCreatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeforeAgreementCreated(decoded));
            }
            if let Ok(decoded) = <BeforeAgreementTerminatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeforeAgreementTerminated(decoded));
            }
            if let Ok(decoded) = <BeforeAgreementUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeforeAgreementUpdated(decoded));
            }
            if let Ok(decoded) = <BlockSubscriptionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlockSubscription(decoded));
            }
            if let Ok(decoded) = <BulkAcceptSubscribeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BulkAcceptSubscribe(decoded));
            }
            if let Ok(decoded) = <BulkBlockSubscriptionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BulkBlockSubscription(decoded));
            }
            if let Ok(decoded) = <CreatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Creator(decoded));
            }
            if let Ok(decoded) = <DescriptionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Description(decoded));
            }
            if let Ok(decoded) = <GetSubscriberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSubscriberCount(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsTrustedForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTrustedForwarder(decoded));
            }
            if let Ok(decoded) = <LikeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Like(decoded));
            }
            if let Ok(decoded) = <PercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Percentage(decoded));
            }
            if let Ok(decoded) = <PostNFTCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PostNFT(decoded));
            }
            if let Ok(decoded) = <RecoverTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecoverTokens(decoded));
            }
            if let Ok(decoded) = <RequestSubscribeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestSubscribe(decoded));
            }
            if let Ok(decoded) = <RevokeSubscribeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeSubscribe(decoded));
            }
            if let Ok(decoded) = <SubscribersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Subscribers(decoded));
            }
            if let Ok(decoded) = <SubscriptionPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubscriptionPrice(decoded));
            }
            if let Ok(decoded) = <TrustedForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TrustedForwarder(decoded));
            }
            if let Ok(decoded) = <UploadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Upload(decoded));
            }
            if let Ok(decoded) = <VersionRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VersionRecipient(decoded));
            }
            if let Ok(decoded) = <WithdrawEthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawEth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CreatonCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptSubscribe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AfterAgreementCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AfterAgreementTerminated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AfterAgreementUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeAgreementCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeAgreementTerminated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeAgreementUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlockSubscription(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BulkAcceptSubscribe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BulkBlockSubscription(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Creator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Description(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubscriberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsTrustedForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Like(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Percentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PostNFT(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecoverTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestSubscribe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeSubscribe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Subscribers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubscriptionPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrustedForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Upload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VersionRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CreatonCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptSubscribe(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AfterAgreementCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AfterAgreementTerminated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AfterAgreementUpdated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeforeAgreementCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeforeAgreementTerminated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeforeAgreementUpdated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlockSubscription(element) => ::core::fmt::Display::fmt(element, f),
                Self::BulkAcceptSubscribe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BulkBlockSubscription(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Creator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Description(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubscriberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsTrustedForwarder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Like(element) => ::core::fmt::Display::fmt(element, f),
                Self::Percentage(element) => ::core::fmt::Display::fmt(element, f),
                Self::PostNFT(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecoverTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestSubscribe(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeSubscribe(element) => ::core::fmt::Display::fmt(element, f),
                Self::Subscribers(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubscriptionPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedForwarder(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upload(element) => ::core::fmt::Display::fmt(element, f),
                Self::VersionRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawEth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptSubscribeCall> for CreatonCalls {
        fn from(value: AcceptSubscribeCall) -> Self {
            Self::AcceptSubscribe(value)
        }
    }
    impl ::core::convert::From<AdminCall> for CreatonCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<AfterAgreementCreatedCall> for CreatonCalls {
        fn from(value: AfterAgreementCreatedCall) -> Self {
            Self::AfterAgreementCreated(value)
        }
    }
    impl ::core::convert::From<AfterAgreementTerminatedCall> for CreatonCalls {
        fn from(value: AfterAgreementTerminatedCall) -> Self {
            Self::AfterAgreementTerminated(value)
        }
    }
    impl ::core::convert::From<AfterAgreementUpdatedCall> for CreatonCalls {
        fn from(value: AfterAgreementUpdatedCall) -> Self {
            Self::AfterAgreementUpdated(value)
        }
    }
    impl ::core::convert::From<BeforeAgreementCreatedCall> for CreatonCalls {
        fn from(value: BeforeAgreementCreatedCall) -> Self {
            Self::BeforeAgreementCreated(value)
        }
    }
    impl ::core::convert::From<BeforeAgreementTerminatedCall> for CreatonCalls {
        fn from(value: BeforeAgreementTerminatedCall) -> Self {
            Self::BeforeAgreementTerminated(value)
        }
    }
    impl ::core::convert::From<BeforeAgreementUpdatedCall> for CreatonCalls {
        fn from(value: BeforeAgreementUpdatedCall) -> Self {
            Self::BeforeAgreementUpdated(value)
        }
    }
    impl ::core::convert::From<BlockSubscriptionCall> for CreatonCalls {
        fn from(value: BlockSubscriptionCall) -> Self {
            Self::BlockSubscription(value)
        }
    }
    impl ::core::convert::From<BulkAcceptSubscribeCall> for CreatonCalls {
        fn from(value: BulkAcceptSubscribeCall) -> Self {
            Self::BulkAcceptSubscribe(value)
        }
    }
    impl ::core::convert::From<BulkBlockSubscriptionCall> for CreatonCalls {
        fn from(value: BulkBlockSubscriptionCall) -> Self {
            Self::BulkBlockSubscription(value)
        }
    }
    impl ::core::convert::From<CreatorCall> for CreatonCalls {
        fn from(value: CreatorCall) -> Self {
            Self::Creator(value)
        }
    }
    impl ::core::convert::From<DescriptionCall> for CreatonCalls {
        fn from(value: DescriptionCall) -> Self {
            Self::Description(value)
        }
    }
    impl ::core::convert::From<GetSubscriberCountCall> for CreatonCalls {
        fn from(value: GetSubscriberCountCall) -> Self {
            Self::GetSubscriberCount(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for CreatonCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsTrustedForwarderCall> for CreatonCalls {
        fn from(value: IsTrustedForwarderCall) -> Self {
            Self::IsTrustedForwarder(value)
        }
    }
    impl ::core::convert::From<LikeCall> for CreatonCalls {
        fn from(value: LikeCall) -> Self {
            Self::Like(value)
        }
    }
    impl ::core::convert::From<PercentageCall> for CreatonCalls {
        fn from(value: PercentageCall) -> Self {
            Self::Percentage(value)
        }
    }
    impl ::core::convert::From<PostNFTCall> for CreatonCalls {
        fn from(value: PostNFTCall) -> Self {
            Self::PostNFT(value)
        }
    }
    impl ::core::convert::From<RecoverTokensCall> for CreatonCalls {
        fn from(value: RecoverTokensCall) -> Self {
            Self::RecoverTokens(value)
        }
    }
    impl ::core::convert::From<RequestSubscribeCall> for CreatonCalls {
        fn from(value: RequestSubscribeCall) -> Self {
            Self::RequestSubscribe(value)
        }
    }
    impl ::core::convert::From<RevokeSubscribeCall> for CreatonCalls {
        fn from(value: RevokeSubscribeCall) -> Self {
            Self::RevokeSubscribe(value)
        }
    }
    impl ::core::convert::From<SubscribersCall> for CreatonCalls {
        fn from(value: SubscribersCall) -> Self {
            Self::Subscribers(value)
        }
    }
    impl ::core::convert::From<SubscriptionPriceCall> for CreatonCalls {
        fn from(value: SubscriptionPriceCall) -> Self {
            Self::SubscriptionPrice(value)
        }
    }
    impl ::core::convert::From<TrustedForwarderCall> for CreatonCalls {
        fn from(value: TrustedForwarderCall) -> Self {
            Self::TrustedForwarder(value)
        }
    }
    impl ::core::convert::From<UploadCall> for CreatonCalls {
        fn from(value: UploadCall) -> Self {
            Self::Upload(value)
        }
    }
    impl ::core::convert::From<VersionRecipientCall> for CreatonCalls {
        fn from(value: VersionRecipientCall) -> Self {
            Self::VersionRecipient(value)
        }
    }
    impl ::core::convert::From<WithdrawEthCall> for CreatonCalls {
        fn from(value: WithdrawEthCall) -> Self {
            Self::WithdrawEth(value)
        }
    }
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `afterAgreementCreated` function with signature `afterAgreementCreated(address,address,bytes32,bytes,bytes,bytes)` and selector `0xd86ed3e5`
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
    pub struct AfterAgreementCreatedReturn {
        pub new_ctx: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `afterAgreementTerminated` function with signature `afterAgreementTerminated(address,address,bytes32,bytes,bytes,bytes)` and selector `0x53c11f99`
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
    pub struct AfterAgreementTerminatedReturn {
        pub new_ctx: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `afterAgreementUpdated` function with signature `afterAgreementUpdated(address,address,bytes32,bytes,bytes,bytes)` and selector `0x230dbd29`
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
    pub struct AfterAgreementUpdatedReturn {
        pub new_ctx: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `beforeAgreementCreated` function with signature `beforeAgreementCreated(address,address,bytes32,bytes,bytes)` and selector `0x30d9c915`
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
    pub struct BeforeAgreementCreatedReturn {
        pub cbdata: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `beforeAgreementTerminated` function with signature `beforeAgreementTerminated(address,address,bytes32,bytes,bytes)` and selector `0x5f9e7d77`
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
    pub struct BeforeAgreementTerminatedReturn {
        pub cbdata: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `beforeAgreementUpdated` function with signature `beforeAgreementUpdated(address,address,bytes32,bytes,bytes)` and selector `0x884d1f40`
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
    pub struct BeforeAgreementUpdatedReturn {
        pub cbdata: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `creator` function with signature `creator()` and selector `0x02d05d3f`
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
    pub struct CreatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `description` function with signature `description()` and selector `0x7284e416`
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
    pub struct DescriptionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getSubscriberCount` function with signature `getSubscriberCount()` and selector `0x17e703ab`
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
    pub struct GetSubscriberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isTrustedForwarder` function with signature `isTrustedForwarder(address)` and selector `0x572b6c05`
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
    pub struct IsTrustedForwarderReturn(pub bool);
    ///Container type for all return fields from the `percentage` function with signature `percentage(int96,int96)` and selector `0x123ff984`
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
    pub struct PercentageReturn(pub i128);
    ///Container type for all return fields from the `postNFT` function with signature `postNFT()` and selector `0x88f9c946`
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
    pub struct PostNFTReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `subscribers` function with signature `subscribers(address)` and selector `0x5745ae28`
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
    pub struct SubscribersReturn {
        pub status: u8,
    }
    ///Container type for all return fields from the `subscriptionPrice` function with signature `subscriptionPrice()` and selector `0xbdc8e54c`
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
    pub struct SubscriptionPriceReturn(pub i128);
    ///Container type for all return fields from the `trustedForwarder` function with signature `trustedForwarder()` and selector `0x7da0a877`
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
    pub struct TrustedForwarderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `versionRecipient` function with signature `versionRecipient()` and selector `0x486ff0cd`
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
    pub struct VersionRecipientReturn(pub ::std::string::String);
}
