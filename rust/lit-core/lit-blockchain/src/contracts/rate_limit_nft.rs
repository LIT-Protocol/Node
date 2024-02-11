pub use rate_limit_nft::*;
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
pub mod rate_limit_nft {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/RateLimitNFT.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RLIHolderRateLimitWindowSeconds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RLIHolderRateLimitWindowSeconds",
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
                        "additionalRequestsPerKilosecondCost",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "additionalRequestsPerKilosecondCost",
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
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
                    ::std::borrow::ToOwned::to_owned("calculateCost"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateCost"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestsPerKilosecond",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiresAt"),
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
                    ::std::borrow::ToOwned::to_owned("calculateRequestsPerKilosecond"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateRequestsPerKilosecond",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payingAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiresAt"),
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
                    ::std::borrow::ToOwned::to_owned("capacity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("capacity"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibRateLimitNFTStorage.RateLimit",
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
                        "checkBelowMaxRequestsPerKilosecond",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "checkBelowMaxRequestsPerKilosecond",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestedRequestsPerKilosecond",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("currentSoldRequestsPerKilosecond"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "currentSoldRequestsPerKilosecond",
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
                    ::std::borrow::ToOwned::to_owned("defaultRateLimitWindowSeconds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultRateLimitWindowSeconds",
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
                    ::std::borrow::ToOwned::to_owned("freeMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freeMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiresAt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestsPerKilosecond",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sVal"),
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
                    ::std::borrow::ToOwned::to_owned("freeMintSigTest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freeMintSigTest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiresAt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestsPerKilosecond",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sVal"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("freeMintSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freeMintSigner"),
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
                    ::std::borrow::ToOwned::to_owned("freeRequestsPerRateLimitWindow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "freeRequestsPerRateLimitWindow",
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("isExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isExpired"),
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
                    ::std::borrow::ToOwned::to_owned("maxExpirationSeconds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maxExpirationSeconds",
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
                    ::std::borrow::ToOwned::to_owned("maxRequestsPerKilosecond"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maxRequestsPerKilosecond",
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiresAt"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
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
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
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
                    ::std::borrow::ToOwned::to_owned("prefixed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prefixed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hash"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("redeemedFreeMints"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemedFreeMints"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned(
                        "setAdditionalRequestsPerKilosecondCost",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setAdditionalRequestsPerKilosecondCost",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newAdditionalRequestsPerKilosecondCost",
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
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
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
                    ::std::borrow::ToOwned::to_owned("setFreeMintSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFreeMintSigner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newFreeMintSigner"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "setFreeRequestsPerRateLimitWindow",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setFreeRequestsPerRateLimitWindow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFreeRequestsPerRateLimitWindow",
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
                    ::std::borrow::ToOwned::to_owned("setMaxExpirationSeconds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setMaxExpirationSeconds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxExpirationSeconds",
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
                    ::std::borrow::ToOwned::to_owned("setMaxRequestsPerKilosecond"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setMaxRequestsPerKilosecond",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMaxRequestsPerKilosecond",
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
                    ::std::borrow::ToOwned::to_owned(
                        "setRLIHolderRateLimitWindowSeconds",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setRLIHolderRateLimitWindowSeconds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRLIHolderRateLimitWindowSeconds",
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
                    ::std::borrow::ToOwned::to_owned("setRateLimitWindowSeconds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setRateLimitWindowSeconds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRateLimitWindowSeconds",
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("tokenIdCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenIdCounter"),
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
                    ::std::borrow::ToOwned::to_owned("tokenOfOwnerByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "tokenOfOwnerByIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("tokenSVG"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenSVG"),
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
                (
                    ::std::borrow::ToOwned::to_owned(
                        "totalSoldRequestsPerKilosecondByExpirationTime",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "totalSoldRequestsPerKilosecondByExpirationTime",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiresAt"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "AdditionalRequestsPerKilosecondCostSet",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AdditionalRequestsPerKilosecondCostSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newAdditionalRequestsPerKilosecondCost",
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
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
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
                    ::std::borrow::ToOwned::to_owned("FreeMintSignerSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FreeMintSignerSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newFreeMintSigner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "FreeRequestsPerRateLimitWindowSet",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FreeRequestsPerRateLimitWindowSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFreeRequestsPerRateLimitWindow",
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned(
                        "RLIHolderRateLimitWindowSecondsSet",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RLIHolderRateLimitWindowSecondsSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRLIHolderRateLimitWindowSeconds",
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
                    ::std::borrow::ToOwned::to_owned("RateLimitWindowSecondsSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RateLimitWindowSecondsSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRateLimitWindowSeconds",
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
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdrew"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdrew"),
                            inputs: ::std::vec![
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RATELIMITNFT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct RateLimitNFT<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RateLimitNFT<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RateLimitNFT<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RateLimitNFT<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RateLimitNFT<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RateLimitNFT))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RateLimitNFT<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RATELIMITNFT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `RLIHolderRateLimitWindowSeconds` (0x4a5f3acd) function
        pub fn rli_holder_rate_limit_window_seconds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([74, 95, 58, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `additionalRequestsPerKilosecondCost` (0x4659470d) function
        pub fn additional_requests_per_kilosecond_cost(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([70, 89, 71, 13], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateCost` (0xce394696) function
        pub fn calculate_cost(
            &self,
            requests_per_kilosecond: ::ethers::core::types::U256,
            expires_at: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 57, 70, 150], (requests_per_kilosecond, expires_at))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateRequestsPerKilosecond` (0xba45b2ba) function
        pub fn calculate_requests_per_kilosecond(
            &self,
            paying_amount: ::ethers::core::types::U256,
            expires_at: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 69, 178, 186], (paying_amount, expires_at))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `capacity` (0xab1bbeca) function
        pub fn capacity(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, RateLimit> {
            self.0
                .method_hash([171, 27, 190, 202], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkBelowMaxRequestsPerKilosecond` (0x72384aea) function
        pub fn check_below_max_requests_per_kilosecond(
            &self,
            requested_requests_per_kilosecond: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([114, 56, 74, 234], requested_requests_per_kilosecond)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentSoldRequestsPerKilosecond` (0xea736df1) function
        pub fn current_sold_requests_per_kilosecond(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([234, 115, 109, 241], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultRateLimitWindowSeconds` (0xfb24b22e) function
        pub fn default_rate_limit_window_seconds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([251, 36, 178, 46], ())
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
        ///Calls the contract's `freeMint` (0x995eebab) function
        pub fn free_mint(
            &self,
            expires_at: ::ethers::core::types::U256,
            requests_per_kilosecond: ::ethers::core::types::U256,
            msg_hash: [u8; 32],
            v: u8,
            r: [u8; 32],
            s_val: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [153, 94, 235, 171],
                    (expires_at, requests_per_kilosecond, msg_hash, v, r, s_val),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freeMintSigTest` (0x3488ab13) function
        pub fn free_mint_sig_test(
            &self,
            expires_at: ::ethers::core::types::U256,
            requests_per_kilosecond: ::ethers::core::types::U256,
            msg_hash: [u8; 32],
            v: u8,
            r: [u8; 32],
            s_val: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [52, 136, 171, 19],
                    (expires_at, requests_per_kilosecond, msg_hash, v, r, s_val),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freeMintSigner` (0xb94a2102) function
        pub fn free_mint_signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([185, 74, 33, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freeRequestsPerRateLimitWindow` (0x26894764) function
        pub fn free_requests_per_rate_limit_window(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 137, 71, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isExpired` (0xd9548e53) function
        pub fn is_expired(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 84, 142, 83], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxExpirationSeconds` (0xba49dcdd) function
        pub fn max_expiration_seconds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 73, 220, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxRequestsPerKilosecond` (0x056bcfeb) function
        pub fn max_requests_per_kilosecond(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([5, 107, 207, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0xa0712d68) function
        pub fn mint(
            &self,
            expires_at: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 113, 45, 104], expires_at)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prefixed` (0x1f275713) function
        pub fn prefixed(
            &self,
            hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([31, 39, 87, 19], hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemedFreeMints` (0x3b1a72cc) function
        pub fn redeemed_free_mints(
            &self,
            msg_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 26, 114, 204], msg_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAdditionalRequestsPerKilosecondCost` (0xe62a2195) function
        pub fn set_additional_requests_per_kilosecond_cost(
            &self,
            new_additional_requests_per_kilosecond_cost: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [230, 42, 33, 149],
                    new_additional_requests_per_kilosecond_cost,
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFreeMintSigner` (0x3b189852) function
        pub fn set_free_mint_signer(
            &self,
            new_free_mint_signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 24, 152, 82], new_free_mint_signer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFreeRequestsPerRateLimitWindow` (0x28b9b37c) function
        pub fn set_free_requests_per_rate_limit_window(
            &self,
            new_free_requests_per_rate_limit_window: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [40, 185, 179, 124],
                    new_free_requests_per_rate_limit_window,
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxExpirationSeconds` (0x82d81905) function
        pub fn set_max_expiration_seconds(
            &self,
            new_max_expiration_seconds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 216, 25, 5], new_max_expiration_seconds)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxRequestsPerKilosecond` (0x756d1fb9) function
        pub fn set_max_requests_per_kilosecond(
            &self,
            new_max_requests_per_kilosecond: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 109, 31, 185], new_max_requests_per_kilosecond)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRLIHolderRateLimitWindowSeconds` (0x39f1a4f1) function
        pub fn set_rli_holder_rate_limit_window_seconds(
            &self,
            new_rli_holder_rate_limit_window_seconds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [57, 241, 164, 241],
                    new_rli_holder_rate_limit_window_seconds,
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRateLimitWindowSeconds` (0x11fc4562) function
        pub fn set_rate_limit_window_seconds(
            &self,
            new_rate_limit_window_seconds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 252, 69, 98], new_rate_limit_window_seconds)
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenByIndex` (0x4f6ccce7) function
        pub fn token_by_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenIdCounter` (0x98bdf6f5) function
        pub fn token_id_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 189, 246, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function
        pub fn token_of_owner_by_index(
            &self,
            owner: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenSVG` (0x9bac5f7a) function
        pub fn token_svg(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([155, 172, 95, 122], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSoldRequestsPerKilosecondByExpirationTime` (0x8b49038d) function
        pub fn total_sold_requests_per_kilosecond_by_expiration_time(
            &self,
            expires_at: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 73, 3, 141], expires_at)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
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
        ///Calls the contract's `withdraw` (0x3ccfd60b) function
        pub fn withdraw(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdditionalRequestsPerKilosecondCostSet` event
        pub fn additional_requests_per_kilosecond_cost_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdditionalRequestsPerKilosecondCostSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
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
        ///Gets the contract's `FreeMintSignerSet` event
        pub fn free_mint_signer_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FreeMintSignerSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FreeRequestsPerRateLimitWindowSet` event
        pub fn free_requests_per_rate_limit_window_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FreeRequestsPerRateLimitWindowSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
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
        ///Gets the contract's `RLIHolderRateLimitWindowSecondsSet` event
        pub fn rli_holder_rate_limit_window_seconds_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RliholderRateLimitWindowSecondsSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RateLimitWindowSecondsSet` event
        pub fn rate_limit_window_seconds_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RateLimitWindowSecondsSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdrew` event
        pub fn withdrew_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrewFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RateLimitNFTEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RateLimitNFT<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub enum RateLimitNFTErrors {
        CallerNotOwner(CallerNotOwner),
        CannotAddFunctionToDiamondThatAlreadyExists(
            CannotAddFunctionToDiamondThatAlreadyExists,
        ),
        CannotAddSelectorsToZeroAddress(CannotAddSelectorsToZeroAddress),
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
        IncorrectFacetCutAction(IncorrectFacetCutAction),
        InitializationFunctionReverted(InitializationFunctionReverted),
        NoBytecodeAtAddress(NoBytecodeAtAddress),
        NoSelectorsProvidedForFacetForCut(NoSelectorsProvidedForFacetForCut),
        NotContractOwner(NotContractOwner),
        RemoveFacetAddressMustBeZeroAddress(RemoveFacetAddressMustBeZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RateLimitNFTErrors {
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
                = <RemoveFacetAddressMustBeZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveFacetAddressMustBeZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RateLimitNFTErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerNotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
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
                Self::IncorrectFacetCutAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializationFunctionReverted(element) => {
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
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RateLimitNFTErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
                    == <IncorrectFacetCutAction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InitializationFunctionReverted as ::ethers::contract::EthError>::selector() => {
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
                    == <RemoveFacetAddressMustBeZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RateLimitNFTErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerNotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
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
                Self::IncorrectFacetCutAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotContractOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RateLimitNFTErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerNotOwner> for RateLimitNFTErrors {
        fn from(value: CallerNotOwner) -> Self {
            Self::CallerNotOwner(value)
        }
    }
    impl ::core::convert::From<CannotAddFunctionToDiamondThatAlreadyExists>
    for RateLimitNFTErrors {
        fn from(value: CannotAddFunctionToDiamondThatAlreadyExists) -> Self {
            Self::CannotAddFunctionToDiamondThatAlreadyExists(value)
        }
    }
    impl ::core::convert::From<CannotAddSelectorsToZeroAddress> for RateLimitNFTErrors {
        fn from(value: CannotAddSelectorsToZeroAddress) -> Self {
            Self::CannotAddSelectorsToZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotRemoveFunctionThatDoesNotExist>
    for RateLimitNFTErrors {
        fn from(value: CannotRemoveFunctionThatDoesNotExist) -> Self {
            Self::CannotRemoveFunctionThatDoesNotExist(value)
        }
    }
    impl ::core::convert::From<CannotRemoveImmutableFunction> for RateLimitNFTErrors {
        fn from(value: CannotRemoveImmutableFunction) -> Self {
            Self::CannotRemoveImmutableFunction(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionThatDoesNotExists>
    for RateLimitNFTErrors {
        fn from(value: CannotReplaceFunctionThatDoesNotExists) -> Self {
            Self::CannotReplaceFunctionThatDoesNotExists(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet>
    for RateLimitNFTErrors {
        fn from(
            value: CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ) -> Self {
            Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionsFromFacetWithZeroAddress>
    for RateLimitNFTErrors {
        fn from(value: CannotReplaceFunctionsFromFacetWithZeroAddress) -> Self {
            Self::CannotReplaceFunctionsFromFacetWithZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotReplaceImmutableFunction> for RateLimitNFTErrors {
        fn from(value: CannotReplaceImmutableFunction) -> Self {
            Self::CannotReplaceImmutableFunction(value)
        }
    }
    impl ::core::convert::From<IncorrectFacetCutAction> for RateLimitNFTErrors {
        fn from(value: IncorrectFacetCutAction) -> Self {
            Self::IncorrectFacetCutAction(value)
        }
    }
    impl ::core::convert::From<InitializationFunctionReverted> for RateLimitNFTErrors {
        fn from(value: InitializationFunctionReverted) -> Self {
            Self::InitializationFunctionReverted(value)
        }
    }
    impl ::core::convert::From<NoBytecodeAtAddress> for RateLimitNFTErrors {
        fn from(value: NoBytecodeAtAddress) -> Self {
            Self::NoBytecodeAtAddress(value)
        }
    }
    impl ::core::convert::From<NoSelectorsProvidedForFacetForCut>
    for RateLimitNFTErrors {
        fn from(value: NoSelectorsProvidedForFacetForCut) -> Self {
            Self::NoSelectorsProvidedForFacetForCut(value)
        }
    }
    impl ::core::convert::From<NotContractOwner> for RateLimitNFTErrors {
        fn from(value: NotContractOwner) -> Self {
            Self::NotContractOwner(value)
        }
    }
    impl ::core::convert::From<RemoveFacetAddressMustBeZeroAddress>
    for RateLimitNFTErrors {
        fn from(value: RemoveFacetAddressMustBeZeroAddress) -> Self {
            Self::RemoveFacetAddressMustBeZeroAddress(value)
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
        name = "AdditionalRequestsPerKilosecondCostSet",
        abi = "AdditionalRequestsPerKilosecondCostSet(uint256)"
    )]
    pub struct AdditionalRequestsPerKilosecondCostSetFilter {
        pub new_additional_requests_per_kilosecond_cost: ::ethers::core::types::U256,
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "FreeMintSignerSet", abi = "FreeMintSignerSet(address)")]
    pub struct FreeMintSignerSetFilter {
        #[ethevent(indexed)]
        pub new_free_mint_signer: ::ethers::core::types::Address,
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
        name = "FreeRequestsPerRateLimitWindowSet",
        abi = "FreeRequestsPerRateLimitWindowSet(uint256)"
    )]
    pub struct FreeRequestsPerRateLimitWindowSetFilter {
        pub new_free_requests_per_rate_limit_window: ::ethers::core::types::U256,
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
    #[ethevent(
        name = "RLIHolderRateLimitWindowSecondsSet",
        abi = "RLIHolderRateLimitWindowSecondsSet(uint256)"
    )]
    pub struct RliholderRateLimitWindowSecondsSetFilter {
        pub new_rli_holder_rate_limit_window_seconds: ::ethers::core::types::U256,
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
        name = "RateLimitWindowSecondsSet",
        abi = "RateLimitWindowSecondsSet(uint256)"
    )]
    pub struct RateLimitWindowSecondsSetFilter {
        pub new_rate_limit_window_seconds: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Withdrew", abi = "Withdrew(uint256)")]
    pub struct WithdrewFilter {
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
    pub enum RateLimitNFTEvents {
        AdditionalRequestsPerKilosecondCostSetFilter(
            AdditionalRequestsPerKilosecondCostSetFilter,
        ),
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        DiamondCutFilter(DiamondCutFilter),
        FreeMintSignerSetFilter(FreeMintSignerSetFilter),
        FreeRequestsPerRateLimitWindowSetFilter(FreeRequestsPerRateLimitWindowSetFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RliholderRateLimitWindowSecondsSetFilter(
            RliholderRateLimitWindowSecondsSetFilter,
        ),
        RateLimitWindowSecondsSetFilter(RateLimitWindowSecondsSetFilter),
        TransferFilter(TransferFilter),
        WithdrewFilter(WithdrewFilter),
    }
    impl ::ethers::contract::EthLogDecode for RateLimitNFTEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded)
                = AdditionalRequestsPerKilosecondCostSetFilter::decode_log(log) {
                return Ok(
                    RateLimitNFTEvents::AdditionalRequestsPerKilosecondCostSetFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = DiamondCutFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::DiamondCutFilter(decoded));
            }
            if let Ok(decoded) = FreeMintSignerSetFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::FreeMintSignerSetFilter(decoded));
            }
            if let Ok(decoded)
                = FreeRequestsPerRateLimitWindowSetFilter::decode_log(log) {
                return Ok(
                    RateLimitNFTEvents::FreeRequestsPerRateLimitWindowSetFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded)
                = RliholderRateLimitWindowSecondsSetFilter::decode_log(log) {
                return Ok(
                    RateLimitNFTEvents::RliholderRateLimitWindowSecondsSetFilter(decoded),
                );
            }
            if let Ok(decoded) = RateLimitWindowSecondsSetFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::RateLimitWindowSecondsSetFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrewFilter::decode_log(log) {
                return Ok(RateLimitNFTEvents::WithdrewFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RateLimitNFTEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdditionalRequestsPerKilosecondCostSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondCutFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreeMintSignerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FreeRequestsPerRateLimitWindowSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RliholderRateLimitWindowSecondsSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RateLimitWindowSecondsSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrewFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdditionalRequestsPerKilosecondCostSetFilter>
    for RateLimitNFTEvents {
        fn from(value: AdditionalRequestsPerKilosecondCostSetFilter) -> Self {
            Self::AdditionalRequestsPerKilosecondCostSetFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for RateLimitNFTEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for RateLimitNFTEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<DiamondCutFilter> for RateLimitNFTEvents {
        fn from(value: DiamondCutFilter) -> Self {
            Self::DiamondCutFilter(value)
        }
    }
    impl ::core::convert::From<FreeMintSignerSetFilter> for RateLimitNFTEvents {
        fn from(value: FreeMintSignerSetFilter) -> Self {
            Self::FreeMintSignerSetFilter(value)
        }
    }
    impl ::core::convert::From<FreeRequestsPerRateLimitWindowSetFilter>
    for RateLimitNFTEvents {
        fn from(value: FreeRequestsPerRateLimitWindowSetFilter) -> Self {
            Self::FreeRequestsPerRateLimitWindowSetFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for RateLimitNFTEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for RateLimitNFTEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RliholderRateLimitWindowSecondsSetFilter>
    for RateLimitNFTEvents {
        fn from(value: RliholderRateLimitWindowSecondsSetFilter) -> Self {
            Self::RliholderRateLimitWindowSecondsSetFilter(value)
        }
    }
    impl ::core::convert::From<RateLimitWindowSecondsSetFilter> for RateLimitNFTEvents {
        fn from(value: RateLimitWindowSecondsSetFilter) -> Self {
            Self::RateLimitWindowSecondsSetFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for RateLimitNFTEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<WithdrewFilter> for RateLimitNFTEvents {
        fn from(value: WithdrewFilter) -> Self {
            Self::WithdrewFilter(value)
        }
    }
    ///Container type for all input parameters for the `RLIHolderRateLimitWindowSeconds` function with signature `RLIHolderRateLimitWindowSeconds()` and selector `0x4a5f3acd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "RLIHolderRateLimitWindowSeconds",
        abi = "RLIHolderRateLimitWindowSeconds()"
    )]
    pub struct RliholderRateLimitWindowSecondsCall;
    ///Container type for all input parameters for the `additionalRequestsPerKilosecondCost` function with signature `additionalRequestsPerKilosecondCost()` and selector `0x4659470d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "additionalRequestsPerKilosecondCost",
        abi = "additionalRequestsPerKilosecondCost()"
    )]
    pub struct AdditionalRequestsPerKilosecondCostCall;
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
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
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateCost` function with signature `calculateCost(uint256,uint256)` and selector `0xce394696`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "calculateCost", abi = "calculateCost(uint256,uint256)")]
    pub struct CalculateCostCall {
        pub requests_per_kilosecond: ::ethers::core::types::U256,
        pub expires_at: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateRequestsPerKilosecond` function with signature `calculateRequestsPerKilosecond(uint256,uint256)` and selector `0xba45b2ba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "calculateRequestsPerKilosecond",
        abi = "calculateRequestsPerKilosecond(uint256,uint256)"
    )]
    pub struct CalculateRequestsPerKilosecondCall {
        pub paying_amount: ::ethers::core::types::U256,
        pub expires_at: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `capacity` function with signature `capacity(uint256)` and selector `0xab1bbeca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "capacity", abi = "capacity(uint256)")]
    pub struct CapacityCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkBelowMaxRequestsPerKilosecond` function with signature `checkBelowMaxRequestsPerKilosecond(uint256)` and selector `0x72384aea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "checkBelowMaxRequestsPerKilosecond",
        abi = "checkBelowMaxRequestsPerKilosecond(uint256)"
    )]
    pub struct CheckBelowMaxRequestsPerKilosecondCall {
        pub requested_requests_per_kilosecond: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `currentSoldRequestsPerKilosecond` function with signature `currentSoldRequestsPerKilosecond()` and selector `0xea736df1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "currentSoldRequestsPerKilosecond",
        abi = "currentSoldRequestsPerKilosecond()"
    )]
    pub struct CurrentSoldRequestsPerKilosecondCall;
    ///Container type for all input parameters for the `defaultRateLimitWindowSeconds` function with signature `defaultRateLimitWindowSeconds()` and selector `0xfb24b22e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "defaultRateLimitWindowSeconds",
        abi = "defaultRateLimitWindowSeconds()"
    )]
    pub struct DefaultRateLimitWindowSecondsCall;
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
    ///Container type for all input parameters for the `freeMint` function with signature `freeMint(uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0x995eebab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "freeMint",
        abi = "freeMint(uint256,uint256,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct FreeMintCall {
        pub expires_at: ::ethers::core::types::U256,
        pub requests_per_kilosecond: ::ethers::core::types::U256,
        pub msg_hash: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s_val: [u8; 32],
    }
    ///Container type for all input parameters for the `freeMintSigTest` function with signature `freeMintSigTest(uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0x3488ab13`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "freeMintSigTest",
        abi = "freeMintSigTest(uint256,uint256,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct FreeMintSigTestCall {
        pub expires_at: ::ethers::core::types::U256,
        pub requests_per_kilosecond: ::ethers::core::types::U256,
        pub msg_hash: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s_val: [u8; 32],
    }
    ///Container type for all input parameters for the `freeMintSigner` function with signature `freeMintSigner()` and selector `0xb94a2102`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "freeMintSigner", abi = "freeMintSigner()")]
    pub struct FreeMintSignerCall;
    ///Container type for all input parameters for the `freeRequestsPerRateLimitWindow` function with signature `freeRequestsPerRateLimitWindow()` and selector `0x26894764`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "freeRequestsPerRateLimitWindow",
        abi = "freeRequestsPerRateLimitWindow()"
    )]
    pub struct FreeRequestsPerRateLimitWindowCall;
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isExpired` function with signature `isExpired(uint256)` and selector `0xd9548e53`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isExpired", abi = "isExpired(uint256)")]
    pub struct IsExpiredCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `maxExpirationSeconds` function with signature `maxExpirationSeconds()` and selector `0xba49dcdd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxExpirationSeconds", abi = "maxExpirationSeconds()")]
    pub struct MaxExpirationSecondsCall;
    ///Container type for all input parameters for the `maxRequestsPerKilosecond` function with signature `maxRequestsPerKilosecond()` and selector `0x056bcfeb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxRequestsPerKilosecond", abi = "maxRequestsPerKilosecond()")]
    pub struct MaxRequestsPerKilosecondCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(uint256)` and selector `0xa0712d68`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mint", abi = "mint(uint256)")]
    pub struct MintCall {
        pub expires_at: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prefixed` function with signature `prefixed(bytes32)` and selector `0x1f275713`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "prefixed", abi = "prefixed(bytes32)")]
    pub struct PrefixedCall {
        pub hash: [u8; 32],
    }
    ///Container type for all input parameters for the `redeemedFreeMints` function with signature `redeemedFreeMints(bytes32)` and selector `0x3b1a72cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "redeemedFreeMints", abi = "redeemedFreeMints(bytes32)")]
    pub struct RedeemedFreeMintsCall {
        pub msg_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setAdditionalRequestsPerKilosecondCost` function with signature `setAdditionalRequestsPerKilosecondCost(uint256)` and selector `0xe62a2195`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setAdditionalRequestsPerKilosecondCost",
        abi = "setAdditionalRequestsPerKilosecondCost(uint256)"
    )]
    pub struct SetAdditionalRequestsPerKilosecondCostCall {
        pub new_additional_requests_per_kilosecond_cost: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `setFreeMintSigner` function with signature `setFreeMintSigner(address)` and selector `0x3b189852`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFreeMintSigner", abi = "setFreeMintSigner(address)")]
    pub struct SetFreeMintSignerCall {
        pub new_free_mint_signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFreeRequestsPerRateLimitWindow` function with signature `setFreeRequestsPerRateLimitWindow(uint256)` and selector `0x28b9b37c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setFreeRequestsPerRateLimitWindow",
        abi = "setFreeRequestsPerRateLimitWindow(uint256)"
    )]
    pub struct SetFreeRequestsPerRateLimitWindowCall {
        pub new_free_requests_per_rate_limit_window: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMaxExpirationSeconds` function with signature `setMaxExpirationSeconds(uint256)` and selector `0x82d81905`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setMaxExpirationSeconds",
        abi = "setMaxExpirationSeconds(uint256)"
    )]
    pub struct SetMaxExpirationSecondsCall {
        pub new_max_expiration_seconds: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setMaxRequestsPerKilosecond` function with signature `setMaxRequestsPerKilosecond(uint256)` and selector `0x756d1fb9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setMaxRequestsPerKilosecond",
        abi = "setMaxRequestsPerKilosecond(uint256)"
    )]
    pub struct SetMaxRequestsPerKilosecondCall {
        pub new_max_requests_per_kilosecond: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setRLIHolderRateLimitWindowSeconds` function with signature `setRLIHolderRateLimitWindowSeconds(uint256)` and selector `0x39f1a4f1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setRLIHolderRateLimitWindowSeconds",
        abi = "setRLIHolderRateLimitWindowSeconds(uint256)"
    )]
    pub struct SetRLIHolderRateLimitWindowSecondsCall {
        pub new_rli_holder_rate_limit_window_seconds: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setRateLimitWindowSeconds` function with signature `setRateLimitWindowSeconds(uint256)` and selector `0x11fc4562`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setRateLimitWindowSeconds",
        abi = "setRateLimitWindowSeconds(uint256)"
    )]
    pub struct SetRateLimitWindowSecondsCall {
        pub new_rate_limit_window_seconds: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenByIndex", abi = "tokenByIndex(uint256)")]
    pub struct TokenByIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenIdCounter` function with signature `tokenIdCounter()` and selector `0x98bdf6f5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenIdCounter", abi = "tokenIdCounter()")]
    pub struct TokenIdCounterCall;
    ///Container type for all input parameters for the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "tokenOfOwnerByIndex",
        abi = "tokenOfOwnerByIndex(address,uint256)"
    )]
    pub struct TokenOfOwnerByIndexCall {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenSVG` function with signature `tokenSVG(uint256)` and selector `0x9bac5f7a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenSVG", abi = "tokenSVG(uint256)")]
    pub struct TokenSVGCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSoldRequestsPerKilosecondByExpirationTime` function with signature `totalSoldRequestsPerKilosecondByExpirationTime(uint256)` and selector `0x8b49038d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "totalSoldRequestsPerKilosecondByExpirationTime",
        abi = "totalSoldRequestsPerKilosecondByExpirationTime(uint256)"
    )]
    pub struct TotalSoldRequestsPerKilosecondByExpirationTimeCall {
        pub expires_at: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
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
    pub enum RateLimitNFTCalls {
        RliholderRateLimitWindowSeconds(RliholderRateLimitWindowSecondsCall),
        AdditionalRequestsPerKilosecondCost(AdditionalRequestsPerKilosecondCostCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        CalculateCost(CalculateCostCall),
        CalculateRequestsPerKilosecond(CalculateRequestsPerKilosecondCall),
        Capacity(CapacityCall),
        CheckBelowMaxRequestsPerKilosecond(CheckBelowMaxRequestsPerKilosecondCall),
        CurrentSoldRequestsPerKilosecond(CurrentSoldRequestsPerKilosecondCall),
        DefaultRateLimitWindowSeconds(DefaultRateLimitWindowSecondsCall),
        DiamondCut(DiamondCutCall),
        FacetAddress(FacetAddressCall),
        FacetAddresses(FacetAddressesCall),
        FacetFunctionSelectors(FacetFunctionSelectorsCall),
        Facets(FacetsCall),
        FreeMint(FreeMintCall),
        FreeMintSigTest(FreeMintSigTestCall),
        FreeMintSigner(FreeMintSignerCall),
        FreeRequestsPerRateLimitWindow(FreeRequestsPerRateLimitWindowCall),
        GetApproved(GetApprovedCall),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        IsExpired(IsExpiredCall),
        MaxExpirationSeconds(MaxExpirationSecondsCall),
        MaxRequestsPerKilosecond(MaxRequestsPerKilosecondCall),
        Mint(MintCall),
        Name(NameCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        Prefixed(PrefixedCall),
        RedeemedFreeMints(RedeemedFreeMintsCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetAdditionalRequestsPerKilosecondCost(
            SetAdditionalRequestsPerKilosecondCostCall,
        ),
        SetApprovalForAll(SetApprovalForAllCall),
        SetFreeMintSigner(SetFreeMintSignerCall),
        SetFreeRequestsPerRateLimitWindow(SetFreeRequestsPerRateLimitWindowCall),
        SetMaxExpirationSeconds(SetMaxExpirationSecondsCall),
        SetMaxRequestsPerKilosecond(SetMaxRequestsPerKilosecondCall),
        SetRLIHolderRateLimitWindowSeconds(SetRLIHolderRateLimitWindowSecondsCall),
        SetRateLimitWindowSeconds(SetRateLimitWindowSecondsCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenIdCounter(TokenIdCounterCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenSVG(TokenSVGCall),
        TokenURI(TokenURICall),
        TotalSoldRequestsPerKilosecondByExpirationTime(
            TotalSoldRequestsPerKilosecondByExpirationTimeCall,
        ),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for RateLimitNFTCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <RliholderRateLimitWindowSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RliholderRateLimitWindowSeconds(decoded));
            }
            if let Ok(decoded)
                = <AdditionalRequestsPerKilosecondCostCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AdditionalRequestsPerKilosecondCost(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <CalculateCostCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CalculateCost(decoded));
            }
            if let Ok(decoded)
                = <CalculateRequestsPerKilosecondCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateRequestsPerKilosecond(decoded));
            }
            if let Ok(decoded)
                = <CapacityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Capacity(decoded));
            }
            if let Ok(decoded)
                = <CheckBelowMaxRequestsPerKilosecondCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckBelowMaxRequestsPerKilosecond(decoded));
            }
            if let Ok(decoded)
                = <CurrentSoldRequestsPerKilosecondCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentSoldRequestsPerKilosecond(decoded));
            }
            if let Ok(decoded)
                = <DefaultRateLimitWindowSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultRateLimitWindowSeconds(decoded));
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
                = <FreeMintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreeMint(decoded));
            }
            if let Ok(decoded)
                = <FreeMintSigTestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreeMintSigTest(decoded));
            }
            if let Ok(decoded)
                = <FreeMintSignerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreeMintSigner(decoded));
            }
            if let Ok(decoded)
                = <FreeRequestsPerRateLimitWindowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FreeRequestsPerRateLimitWindow(decoded));
            }
            if let Ok(decoded)
                = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded)
                = <IsExpiredCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsExpired(decoded));
            }
            if let Ok(decoded)
                = <MaxExpirationSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxExpirationSeconds(decoded));
            }
            if let Ok(decoded)
                = <MaxRequestsPerKilosecondCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxRequestsPerKilosecond(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded)
                = <PrefixedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Prefixed(decoded));
            }
            if let Ok(decoded)
                = <RedeemedFreeMintsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RedeemedFreeMints(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded)
                = <SetAdditionalRequestsPerKilosecondCostCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetAdditionalRequestsPerKilosecondCost(decoded));
            }
            if let Ok(decoded)
                = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded)
                = <SetFreeMintSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFreeMintSigner(decoded));
            }
            if let Ok(decoded)
                = <SetFreeRequestsPerRateLimitWindowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFreeRequestsPerRateLimitWindow(decoded));
            }
            if let Ok(decoded)
                = <SetMaxExpirationSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMaxExpirationSeconds(decoded));
            }
            if let Ok(decoded)
                = <SetMaxRequestsPerKilosecondCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMaxRequestsPerKilosecond(decoded));
            }
            if let Ok(decoded)
                = <SetRLIHolderRateLimitWindowSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetRLIHolderRateLimitWindowSeconds(decoded));
            }
            if let Ok(decoded)
                = <SetRateLimitWindowSecondsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetRateLimitWindowSeconds(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TokenByIndexCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenByIndex(decoded));
            }
            if let Ok(decoded)
                = <TokenIdCounterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenIdCounter(decoded));
            }
            if let Ok(decoded)
                = <TokenOfOwnerByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TokenOfOwnerByIndex(decoded));
            }
            if let Ok(decoded)
                = <TokenSVGCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenSVG(decoded));
            }
            if let Ok(decoded)
                = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded)
                = <TotalSoldRequestsPerKilosecondByExpirationTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TotalSoldRequestsPerKilosecondByExpirationTime(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RateLimitNFTCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RliholderRateLimitWindowSeconds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdditionalRequestsPerKilosecondCost(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CalculateCost(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateRequestsPerKilosecond(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Capacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckBelowMaxRequestsPerKilosecond(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentSoldRequestsPerKilosecond(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultRateLimitWindowSeconds(element) => {
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
                Self::FreeMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FreeMintSigTest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FreeMintSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FreeRequestsPerRateLimitWindow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxExpirationSeconds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxRequestsPerKilosecond(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Prefixed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemedFreeMints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAdditionalRequestsPerKilosecondCost(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFreeMintSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFreeRequestsPerRateLimitWindow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxExpirationSeconds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxRequestsPerKilosecond(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRLIHolderRateLimitWindowSeconds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRateLimitWindowSeconds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenIdCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenOfOwnerByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenSVG(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSoldRequestsPerKilosecondByExpirationTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RateLimitNFTCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RliholderRateLimitWindowSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdditionalRequestsPerKilosecondCost(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateCost(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateRequestsPerKilosecond(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Capacity(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckBelowMaxRequestsPerKilosecond(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurrentSoldRequestsPerKilosecond(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultRateLimitWindowSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DiamondCut(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetFunctionSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Facets(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreeMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreeMintSigTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreeMintSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreeRequestsPerRateLimitWindow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxExpirationSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxRequestsPerKilosecond(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prefixed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemedFreeMints(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetAdditionalRequestsPerKilosecondCost(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFreeMintSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFreeRequestsPerRateLimitWindow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxExpirationSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxRequestsPerKilosecond(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRLIHolderRateLimitWindowSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRateLimitWindowSeconds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenIdCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenOfOwnerByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenSVG(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSoldRequestsPerKilosecondByExpirationTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RliholderRateLimitWindowSecondsCall>
    for RateLimitNFTCalls {
        fn from(value: RliholderRateLimitWindowSecondsCall) -> Self {
            Self::RliholderRateLimitWindowSeconds(value)
        }
    }
    impl ::core::convert::From<AdditionalRequestsPerKilosecondCostCall>
    for RateLimitNFTCalls {
        fn from(value: AdditionalRequestsPerKilosecondCostCall) -> Self {
            Self::AdditionalRequestsPerKilosecondCost(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for RateLimitNFTCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for RateLimitNFTCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for RateLimitNFTCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<CalculateCostCall> for RateLimitNFTCalls {
        fn from(value: CalculateCostCall) -> Self {
            Self::CalculateCost(value)
        }
    }
    impl ::core::convert::From<CalculateRequestsPerKilosecondCall>
    for RateLimitNFTCalls {
        fn from(value: CalculateRequestsPerKilosecondCall) -> Self {
            Self::CalculateRequestsPerKilosecond(value)
        }
    }
    impl ::core::convert::From<CapacityCall> for RateLimitNFTCalls {
        fn from(value: CapacityCall) -> Self {
            Self::Capacity(value)
        }
    }
    impl ::core::convert::From<CheckBelowMaxRequestsPerKilosecondCall>
    for RateLimitNFTCalls {
        fn from(value: CheckBelowMaxRequestsPerKilosecondCall) -> Self {
            Self::CheckBelowMaxRequestsPerKilosecond(value)
        }
    }
    impl ::core::convert::From<CurrentSoldRequestsPerKilosecondCall>
    for RateLimitNFTCalls {
        fn from(value: CurrentSoldRequestsPerKilosecondCall) -> Self {
            Self::CurrentSoldRequestsPerKilosecond(value)
        }
    }
    impl ::core::convert::From<DefaultRateLimitWindowSecondsCall> for RateLimitNFTCalls {
        fn from(value: DefaultRateLimitWindowSecondsCall) -> Self {
            Self::DefaultRateLimitWindowSeconds(value)
        }
    }
    impl ::core::convert::From<DiamondCutCall> for RateLimitNFTCalls {
        fn from(value: DiamondCutCall) -> Self {
            Self::DiamondCut(value)
        }
    }
    impl ::core::convert::From<FacetAddressCall> for RateLimitNFTCalls {
        fn from(value: FacetAddressCall) -> Self {
            Self::FacetAddress(value)
        }
    }
    impl ::core::convert::From<FacetAddressesCall> for RateLimitNFTCalls {
        fn from(value: FacetAddressesCall) -> Self {
            Self::FacetAddresses(value)
        }
    }
    impl ::core::convert::From<FacetFunctionSelectorsCall> for RateLimitNFTCalls {
        fn from(value: FacetFunctionSelectorsCall) -> Self {
            Self::FacetFunctionSelectors(value)
        }
    }
    impl ::core::convert::From<FacetsCall> for RateLimitNFTCalls {
        fn from(value: FacetsCall) -> Self {
            Self::Facets(value)
        }
    }
    impl ::core::convert::From<FreeMintCall> for RateLimitNFTCalls {
        fn from(value: FreeMintCall) -> Self {
            Self::FreeMint(value)
        }
    }
    impl ::core::convert::From<FreeMintSigTestCall> for RateLimitNFTCalls {
        fn from(value: FreeMintSigTestCall) -> Self {
            Self::FreeMintSigTest(value)
        }
    }
    impl ::core::convert::From<FreeMintSignerCall> for RateLimitNFTCalls {
        fn from(value: FreeMintSignerCall) -> Self {
            Self::FreeMintSigner(value)
        }
    }
    impl ::core::convert::From<FreeRequestsPerRateLimitWindowCall>
    for RateLimitNFTCalls {
        fn from(value: FreeRequestsPerRateLimitWindowCall) -> Self {
            Self::FreeRequestsPerRateLimitWindow(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for RateLimitNFTCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for RateLimitNFTCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for RateLimitNFTCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<IsExpiredCall> for RateLimitNFTCalls {
        fn from(value: IsExpiredCall) -> Self {
            Self::IsExpired(value)
        }
    }
    impl ::core::convert::From<MaxExpirationSecondsCall> for RateLimitNFTCalls {
        fn from(value: MaxExpirationSecondsCall) -> Self {
            Self::MaxExpirationSeconds(value)
        }
    }
    impl ::core::convert::From<MaxRequestsPerKilosecondCall> for RateLimitNFTCalls {
        fn from(value: MaxRequestsPerKilosecondCall) -> Self {
            Self::MaxRequestsPerKilosecond(value)
        }
    }
    impl ::core::convert::From<MintCall> for RateLimitNFTCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for RateLimitNFTCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RateLimitNFTCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for RateLimitNFTCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<PrefixedCall> for RateLimitNFTCalls {
        fn from(value: PrefixedCall) -> Self {
            Self::Prefixed(value)
        }
    }
    impl ::core::convert::From<RedeemedFreeMintsCall> for RateLimitNFTCalls {
        fn from(value: RedeemedFreeMintsCall) -> Self {
            Self::RedeemedFreeMints(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for RateLimitNFTCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
    for RateLimitNFTCalls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetAdditionalRequestsPerKilosecondCostCall>
    for RateLimitNFTCalls {
        fn from(value: SetAdditionalRequestsPerKilosecondCostCall) -> Self {
            Self::SetAdditionalRequestsPerKilosecondCost(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for RateLimitNFTCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetFreeMintSignerCall> for RateLimitNFTCalls {
        fn from(value: SetFreeMintSignerCall) -> Self {
            Self::SetFreeMintSigner(value)
        }
    }
    impl ::core::convert::From<SetFreeRequestsPerRateLimitWindowCall>
    for RateLimitNFTCalls {
        fn from(value: SetFreeRequestsPerRateLimitWindowCall) -> Self {
            Self::SetFreeRequestsPerRateLimitWindow(value)
        }
    }
    impl ::core::convert::From<SetMaxExpirationSecondsCall> for RateLimitNFTCalls {
        fn from(value: SetMaxExpirationSecondsCall) -> Self {
            Self::SetMaxExpirationSeconds(value)
        }
    }
    impl ::core::convert::From<SetMaxRequestsPerKilosecondCall> for RateLimitNFTCalls {
        fn from(value: SetMaxRequestsPerKilosecondCall) -> Self {
            Self::SetMaxRequestsPerKilosecond(value)
        }
    }
    impl ::core::convert::From<SetRLIHolderRateLimitWindowSecondsCall>
    for RateLimitNFTCalls {
        fn from(value: SetRLIHolderRateLimitWindowSecondsCall) -> Self {
            Self::SetRLIHolderRateLimitWindowSeconds(value)
        }
    }
    impl ::core::convert::From<SetRateLimitWindowSecondsCall> for RateLimitNFTCalls {
        fn from(value: SetRateLimitWindowSecondsCall) -> Self {
            Self::SetRateLimitWindowSeconds(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for RateLimitNFTCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for RateLimitNFTCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenByIndexCall> for RateLimitNFTCalls {
        fn from(value: TokenByIndexCall) -> Self {
            Self::TokenByIndex(value)
        }
    }
    impl ::core::convert::From<TokenIdCounterCall> for RateLimitNFTCalls {
        fn from(value: TokenIdCounterCall) -> Self {
            Self::TokenIdCounter(value)
        }
    }
    impl ::core::convert::From<TokenOfOwnerByIndexCall> for RateLimitNFTCalls {
        fn from(value: TokenOfOwnerByIndexCall) -> Self {
            Self::TokenOfOwnerByIndex(value)
        }
    }
    impl ::core::convert::From<TokenSVGCall> for RateLimitNFTCalls {
        fn from(value: TokenSVGCall) -> Self {
            Self::TokenSVG(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for RateLimitNFTCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TotalSoldRequestsPerKilosecondByExpirationTimeCall>
    for RateLimitNFTCalls {
        fn from(value: TotalSoldRequestsPerKilosecondByExpirationTimeCall) -> Self {
            Self::TotalSoldRequestsPerKilosecondByExpirationTime(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for RateLimitNFTCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for RateLimitNFTCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for RateLimitNFTCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for RateLimitNFTCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `RLIHolderRateLimitWindowSeconds` function with signature `RLIHolderRateLimitWindowSeconds()` and selector `0x4a5f3acd`
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
    pub struct RliholderRateLimitWindowSecondsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `additionalRequestsPerKilosecondCost` function with signature `additionalRequestsPerKilosecondCost()` and selector `0x4659470d`
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
    pub struct AdditionalRequestsPerKilosecondCostReturn(
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `calculateCost` function with signature `calculateCost(uint256,uint256)` and selector `0xce394696`
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
    pub struct CalculateCostReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateRequestsPerKilosecond` function with signature `calculateRequestsPerKilosecond(uint256,uint256)` and selector `0xba45b2ba`
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
    pub struct CalculateRequestsPerKilosecondReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `capacity` function with signature `capacity(uint256)` and selector `0xab1bbeca`
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
    pub struct CapacityReturn(pub RateLimit);
    ///Container type for all return fields from the `checkBelowMaxRequestsPerKilosecond` function with signature `checkBelowMaxRequestsPerKilosecond(uint256)` and selector `0x72384aea`
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
    pub struct CheckBelowMaxRequestsPerKilosecondReturn(pub bool);
    ///Container type for all return fields from the `currentSoldRequestsPerKilosecond` function with signature `currentSoldRequestsPerKilosecond()` and selector `0xea736df1`
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
    pub struct CurrentSoldRequestsPerKilosecondReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `defaultRateLimitWindowSeconds` function with signature `defaultRateLimitWindowSeconds()` and selector `0xfb24b22e`
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
    pub struct DefaultRateLimitWindowSecondsReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `freeMint` function with signature `freeMint(uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0x995eebab`
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
    pub struct FreeMintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `freeMintSigner` function with signature `freeMintSigner()` and selector `0xb94a2102`
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
    pub struct FreeMintSignerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `freeRequestsPerRateLimitWindow` function with signature `freeRequestsPerRateLimitWindow()` and selector `0x26894764`
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
    pub struct FreeRequestsPerRateLimitWindowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `isExpired` function with signature `isExpired(uint256)` and selector `0xd9548e53`
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
    pub struct IsExpiredReturn(pub bool);
    ///Container type for all return fields from the `maxExpirationSeconds` function with signature `maxExpirationSeconds()` and selector `0xba49dcdd`
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
    pub struct MaxExpirationSecondsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxRequestsPerKilosecond` function with signature `maxRequestsPerKilosecond()` and selector `0x056bcfeb`
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
    pub struct MaxRequestsPerKilosecondReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mint` function with signature `mint(uint256)` and selector `0xa0712d68`
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
    pub struct MintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `prefixed` function with signature `prefixed(bytes32)` and selector `0x1f275713`
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
    pub struct PrefixedReturn(pub [u8; 32]);
    ///Container type for all return fields from the `redeemedFreeMints` function with signature `redeemedFreeMints(bytes32)` and selector `0x3b1a72cc`
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
    pub struct RedeemedFreeMintsReturn(pub bool);
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
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
    pub struct TokenByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenIdCounter` function with signature `tokenIdCounter()` and selector `0x98bdf6f5`
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
    pub struct TokenIdCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
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
    pub struct TokenOfOwnerByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenSVG` function with signature `tokenSVG(uint256)` and selector `0x9bac5f7a`
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
    pub struct TokenSVGReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    ///Container type for all return fields from the `totalSoldRequestsPerKilosecondByExpirationTime` function with signature `totalSoldRequestsPerKilosecondByExpirationTime(uint256)` and selector `0x8b49038d`
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
    pub struct TotalSoldRequestsPerKilosecondByExpirationTimeReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
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
    ///`RateLimit(uint256,uint256)`
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
    pub struct RateLimit {
        pub requests_per_kilosecond: ::ethers::core::types::U256,
        pub expires_at: ::ethers::core::types::U256,
    }
}
