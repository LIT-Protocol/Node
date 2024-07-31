pub use multisender::*;
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
pub mod multisender {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/Multisender.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("sendEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendEth"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipients"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipients"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenContract"),
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
                (
                    ::std::borrow::ToOwned::to_owned("withdrawTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenContract"),
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
    pub static MULTISENDER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0-a\0\"a\x002` \x1B` \x1CV[a\0:` \x1B` \x1CV[a\0\xFEV[`\x003\x90P\x90V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[a\x0B\xF9\x80a\x01\r`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0pW`\x005`\xE0\x1C\x80cn\xCF\x13\x86\x11a\0NW\x80cn\xCF\x13\x86\x14a\0\xD1W\x80cqP\x18\xA6\x14a\0\xFAW\x80c\x8D\xA5\xCB[\x14a\x01\x11W\x80c\xF2\xFD\xE3\x8B\x14a\x01<Wa\0pV[\x80c;/\xE7\x81\x14a\0uW\x80c<\xCF\xD6\x0B\x14a\0\x91W\x80cI\xDFr\x8C\x14a\0\xA8W[`\0\x80\xFD[a\0\x8F`\x04\x806\x03\x81\x01\x90a\0\x8A\x91\x90a\x07LV[a\x01eV[\0[4\x80\x15a\0\x9DW`\0\x80\xFD[Pa\0\xA6a\x02\rV[\0[4\x80\x15a\0\xB4W`\0\x80\xFD[Pa\0\xCF`\x04\x806\x03\x81\x01\x90a\0\xCA\x91\x90a\x07\xF7V[a\x02^V[\0[4\x80\x15a\0\xDDW`\0\x80\xFD[Pa\0\xF8`\x04\x806\x03\x81\x01\x90a\0\xF3\x91\x90a\x08$V[a\x03mV[\0[4\x80\x15a\x01\x06W`\0\x80\xFD[Pa\x01\x0Fa\x04\xD3V[\0[4\x80\x15a\x01\x1DW`\0\x80\xFD[Pa\x01&a\x04\xE7V[`@Qa\x013\x91\x90a\x08\x93V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01HW`\0\x80\xFD[Pa\x01c`\x04\x806\x03\x81\x01\x90a\x01^\x91\x90a\x07\xF7V[a\x05\x10V[\0[`\0\x82\x82\x90P4a\x01v\x91\x90a\t\x16V[\x90P`\0[\x83\x83\x90P\x81\x10\x15a\x02\x07W\x83\x83\x82\x81\x81\x10a\x01\x99Wa\x01\x98a\tGV[[\x90P` \x02\x01` \x81\x01\x90a\x01\xAE\x91\x90a\x07\xF7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xFC\x83\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xF3W=`\0\x80>=`\0\xFD[P\x80\x80a\x01\xFF\x90a\tvV[\x91PPa\x01{V[PPPPV[a\x02\x15a\x05\x93V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xFCG\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02[W=`\0\x80>=`\0\xFD[PV[a\x02fa\x05\x93V[`\0\x81\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xA6\x91\x90a\x08\x93V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE7\x91\x90a\t\xEAV[\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB3\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03$\x92\x91\x90a\n&V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03g\x91\x90a\n\x87V[PPPPV[`\0\x81\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xAD\x91\x90a\x08\x93V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEE\x91\x90a\t\xEAV[\x90P`\0\x85\x85\x90P\x82a\x04\x01\x91\x90a\t\x16V[\x90P`\0[\x86\x86\x90P\x81\x10\x15a\x04\xCAW\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x88\x88\x84\x81\x81\x10a\x04@Wa\x04?a\tGV[[\x90P` \x02\x01` \x81\x01\x90a\x04U\x91\x90a\x07\xF7V[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04s\x92\x91\x90a\n&V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB6\x91\x90a\n\x87V[P\x80\x80a\x04\xC2\x90a\tvV[\x91PPa\x04\x06V[PPPPPPPV[a\x04\xDBa\x05\x93V[a\x04\xE5`\0a\x06\x11V[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x18a\x05\x93V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05~\x90a\x0B7V[`@Q\x80\x91\x03\x90\xFD[a\x05\x90\x81a\x06\x11V[PV[a\x05\x9Ba\x06\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xB9a\x04\xE7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\x0FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\x06\x90a\x0B\xA3V[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x0CWa\x07\x0Ba\x06\xE7V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07)Wa\x07(a\x06\xECV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x07EWa\x07Da\x06\xF1V[[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x07cWa\x07ba\x06\xDDV[[`\0\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x81Wa\x07\x80a\x06\xE2V[[a\x07\x8D\x85\x82\x86\x01a\x06\xF6V[\x92P\x92PP\x92P\x92\x90PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x07\xC4\x82a\x07\x99V[\x90P\x91\x90PV[a\x07\xD4\x81a\x07\xB9V[\x81\x14a\x07\xDFW`\0\x80\xFD[PV[`\0\x815\x90Pa\x07\xF1\x81a\x07\xCBV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08\rWa\x08\x0Ca\x06\xDDV[[`\0a\x08\x1B\x84\x82\x85\x01a\x07\xE2V[\x91PP\x92\x91PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x08=Wa\x08<a\x06\xDDV[[`\0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08[Wa\x08Za\x06\xE2V[[a\x08g\x86\x82\x87\x01a\x06\xF6V[\x93P\x93PP` a\x08z\x86\x82\x87\x01a\x07\xE2V[\x91PP\x92P\x92P\x92V[a\x08\x8D\x81a\x07\xB9V[\x82RPPV[`\0` \x82\x01\x90Pa\x08\xA8`\0\x83\x01\x84a\x08\x84V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\t!\x82a\x08\xAEV[\x91Pa\t,\x83a\x08\xAEV[\x92P\x82a\t<Wa\t;a\x08\xB8V[[\x82\x82\x04\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a\t\x81\x82a\x08\xAEV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\t\xB3Wa\t\xB2a\x08\xE7V[[`\x01\x82\x01\x90P\x91\x90PV[a\t\xC7\x81a\x08\xAEV[\x81\x14a\t\xD2W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\t\xE4\x81a\t\xBEV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\0Wa\t\xFFa\x06\xDDV[[`\0a\n\x0E\x84\x82\x85\x01a\t\xD5V[\x91PP\x92\x91PPV[a\n \x81a\x08\xAEV[\x82RPPV[`\0`@\x82\x01\x90Pa\n;`\0\x83\x01\x85a\x08\x84V[a\nH` \x83\x01\x84a\n\x17V[\x93\x92PPPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\nd\x81a\nOV[\x81\x14a\noW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\n\x81\x81a\n[V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\x9DWa\n\x9Ca\x06\xDDV[[`\0a\n\xAB\x84\x82\x85\x01a\nrV[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x0B!`&\x83a\n\xB4V[\x91Pa\x0B,\x82a\n\xC5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0BP\x81a\x0B\x14V[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a\x0B\x8D` \x83a\n\xB4V[\x91Pa\x0B\x98\x82a\x0BWV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0B\xBC\x81a\x0B\x80V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xEE:\xBD\xF1\x08|-\xD7\x8C\xF9\x15\x1E\x87c\xBEs\xBE\xE2\xFD\x81W\x89\x98D\xB36%\xA3\"O\xB6\rdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static MULTISENDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0pW`\x005`\xE0\x1C\x80cn\xCF\x13\x86\x11a\0NW\x80cn\xCF\x13\x86\x14a\0\xD1W\x80cqP\x18\xA6\x14a\0\xFAW\x80c\x8D\xA5\xCB[\x14a\x01\x11W\x80c\xF2\xFD\xE3\x8B\x14a\x01<Wa\0pV[\x80c;/\xE7\x81\x14a\0uW\x80c<\xCF\xD6\x0B\x14a\0\x91W\x80cI\xDFr\x8C\x14a\0\xA8W[`\0\x80\xFD[a\0\x8F`\x04\x806\x03\x81\x01\x90a\0\x8A\x91\x90a\x07LV[a\x01eV[\0[4\x80\x15a\0\x9DW`\0\x80\xFD[Pa\0\xA6a\x02\rV[\0[4\x80\x15a\0\xB4W`\0\x80\xFD[Pa\0\xCF`\x04\x806\x03\x81\x01\x90a\0\xCA\x91\x90a\x07\xF7V[a\x02^V[\0[4\x80\x15a\0\xDDW`\0\x80\xFD[Pa\0\xF8`\x04\x806\x03\x81\x01\x90a\0\xF3\x91\x90a\x08$V[a\x03mV[\0[4\x80\x15a\x01\x06W`\0\x80\xFD[Pa\x01\x0Fa\x04\xD3V[\0[4\x80\x15a\x01\x1DW`\0\x80\xFD[Pa\x01&a\x04\xE7V[`@Qa\x013\x91\x90a\x08\x93V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01HW`\0\x80\xFD[Pa\x01c`\x04\x806\x03\x81\x01\x90a\x01^\x91\x90a\x07\xF7V[a\x05\x10V[\0[`\0\x82\x82\x90P4a\x01v\x91\x90a\t\x16V[\x90P`\0[\x83\x83\x90P\x81\x10\x15a\x02\x07W\x83\x83\x82\x81\x81\x10a\x01\x99Wa\x01\x98a\tGV[[\x90P` \x02\x01` \x81\x01\x90a\x01\xAE\x91\x90a\x07\xF7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xFC\x83\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xF3W=`\0\x80>=`\0\xFD[P\x80\x80a\x01\xFF\x90a\tvV[\x91PPa\x01{V[PPPPV[a\x02\x15a\x05\x93V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xFCG\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02[W=`\0\x80>=`\0\xFD[PV[a\x02fa\x05\x93V[`\0\x81\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xA6\x91\x90a\x08\x93V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE7\x91\x90a\t\xEAV[\x90P\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB3\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03$\x92\x91\x90a\n&V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03g\x91\x90a\n\x87V[PPPPV[`\0\x81\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xAD\x91\x90a\x08\x93V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xEE\x91\x90a\t\xEAV[\x90P`\0\x85\x85\x90P\x82a\x04\x01\x91\x90a\t\x16V[\x90P`\0[\x86\x86\x90P\x81\x10\x15a\x04\xCAW\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x88\x88\x84\x81\x81\x10a\x04@Wa\x04?a\tGV[[\x90P` \x02\x01` \x81\x01\x90a\x04U\x91\x90a\x07\xF7V[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04s\x92\x91\x90a\n&V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB6\x91\x90a\n\x87V[P\x80\x80a\x04\xC2\x90a\tvV[\x91PPa\x04\x06V[PPPPPPPV[a\x04\xDBa\x05\x93V[a\x04\xE5`\0a\x06\x11V[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a\x05\x18a\x05\x93V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05~\x90a\x0B7V[`@Q\x80\x91\x03\x90\xFD[a\x05\x90\x81a\x06\x11V[PV[a\x05\x9Ba\x06\xD5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xB9a\x04\xE7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\x0FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\x06\x90a\x0B\xA3V[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\x003\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x0CWa\x07\x0Ba\x06\xE7V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07)Wa\x07(a\x06\xECV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x07EWa\x07Da\x06\xF1V[[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x07cWa\x07ba\x06\xDDV[[`\0\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x81Wa\x07\x80a\x06\xE2V[[a\x07\x8D\x85\x82\x86\x01a\x06\xF6V[\x92P\x92PP\x92P\x92\x90PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x07\xC4\x82a\x07\x99V[\x90P\x91\x90PV[a\x07\xD4\x81a\x07\xB9V[\x81\x14a\x07\xDFW`\0\x80\xFD[PV[`\0\x815\x90Pa\x07\xF1\x81a\x07\xCBV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08\rWa\x08\x0Ca\x06\xDDV[[`\0a\x08\x1B\x84\x82\x85\x01a\x07\xE2V[\x91PP\x92\x91PPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x08=Wa\x08<a\x06\xDDV[[`\0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08[Wa\x08Za\x06\xE2V[[a\x08g\x86\x82\x87\x01a\x06\xF6V[\x93P\x93PP` a\x08z\x86\x82\x87\x01a\x07\xE2V[\x91PP\x92P\x92P\x92V[a\x08\x8D\x81a\x07\xB9V[\x82RPPV[`\0` \x82\x01\x90Pa\x08\xA8`\0\x83\x01\x84a\x08\x84V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\t!\x82a\x08\xAEV[\x91Pa\t,\x83a\x08\xAEV[\x92P\x82a\t<Wa\t;a\x08\xB8V[[\x82\x82\x04\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a\t\x81\x82a\x08\xAEV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\t\xB3Wa\t\xB2a\x08\xE7V[[`\x01\x82\x01\x90P\x91\x90PV[a\t\xC7\x81a\x08\xAEV[\x81\x14a\t\xD2W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\t\xE4\x81a\t\xBEV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\0Wa\t\xFFa\x06\xDDV[[`\0a\n\x0E\x84\x82\x85\x01a\t\xD5V[\x91PP\x92\x91PPV[a\n \x81a\x08\xAEV[\x82RPPV[`\0`@\x82\x01\x90Pa\n;`\0\x83\x01\x85a\x08\x84V[a\nH` \x83\x01\x84a\n\x17V[\x93\x92PPPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\nd\x81a\nOV[\x81\x14a\noW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\n\x81\x81a\n[V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\x9DWa\n\x9Ca\x06\xDDV[[`\0a\n\xAB\x84\x82\x85\x01a\nrV[\x91PP\x92\x91PPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x0B!`&\x83a\n\xB4V[\x91Pa\x0B,\x82a\n\xC5V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0BP\x81a\x0B\x14V[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a\x0B\x8D` \x83a\n\xB4V[\x91Pa\x0B\x98\x82a\x0BWV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0B\xBC\x81a\x0B\x80V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xEE:\xBD\xF1\x08|-\xD7\x8C\xF9\x15\x1E\x87c\xBEs\xBE\xE2\xFD\x81W\x89\x98D\xB36%\xA3\"O\xB6\rdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static MULTISENDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Multisender<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Multisender<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Multisender<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Multisender<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Multisender<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Multisender))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Multisender<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTISENDER_ABI.clone(),
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
                MULTISENDER_ABI.clone(),
                MULTISENDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `sendEth` (0x3b2fe781) function
        pub fn send_eth(
            &self,
            recipients: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 47, 231, 129], recipients)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendTokens` (0x6ecf1386) function
        pub fn send_tokens(
            &self,
            recipients: ::std::vec::Vec<::ethers::core::types::Address>,
            token_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 207, 19, 134], (recipients, token_contract))
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
        ///Calls the contract's `withdrawTokens` (0x49df728c) function
        pub fn withdraw_tokens(
            &self,
            token_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 223, 114, 140], token_contract)
                .expect("method not found (this should never happen)")
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
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Multisender<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `sendEth` function with signature `sendEth(address[])` and selector `0x3b2fe781`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sendEth", abi = "sendEth(address[])")]
    pub struct SendEthCall {
        pub recipients: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `sendTokens` function with signature `sendTokens(address[],address)` and selector `0x6ecf1386`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sendTokens", abi = "sendTokens(address[],address)")]
    pub struct SendTokensCall {
        pub recipients: ::std::vec::Vec<::ethers::core::types::Address>,
        pub token_contract: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `withdrawTokens` function with signature `withdrawTokens(address)` and selector `0x49df728c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawTokens", abi = "withdrawTokens(address)")]
    pub struct WithdrawTokensCall {
        pub token_contract: ::ethers::core::types::Address,
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
    pub enum MultisenderCalls {
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SendEth(SendEthCall),
        SendTokens(SendTokensCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
        WithdrawTokens(WithdrawTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for MultisenderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SendEthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendEth(decoded));
            }
            if let Ok(decoded)
                = <SendTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendTokens(decoded));
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
            if let Ok(decoded)
                = <WithdrawTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultisenderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendEth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MultisenderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTokens(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OwnerCall> for MultisenderCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MultisenderCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SendEthCall> for MultisenderCalls {
        fn from(value: SendEthCall) -> Self {
            Self::SendEth(value)
        }
    }
    impl ::core::convert::From<SendTokensCall> for MultisenderCalls {
        fn from(value: SendTokensCall) -> Self {
            Self::SendTokens(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MultisenderCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for MultisenderCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawTokensCall> for MultisenderCalls {
        fn from(value: WithdrawTokensCall) -> Self {
            Self::WithdrawTokens(value)
        }
    }
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
