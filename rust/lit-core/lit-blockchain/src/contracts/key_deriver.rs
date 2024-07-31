pub use key_deriver::*;
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
pub mod key_deriver {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/KeyDeriver.json",
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
                    ::std::borrow::ToOwned::to_owned("HD_KDF"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HD_KDF"),
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
                    ::std::borrow::ToOwned::to_owned("computeHDPubKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeHDPubKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("derivedKeyId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootHDKeys"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPubkeyRouter.RootKey[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyType"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static KEYDERIVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\nS\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cb\xE4\xC4d\x14a\0;W\x80c\xA3,+\x99\x14a\0YW[`\0\x80\xFD[a\0Ca\0\x8AV[`@Qa\0P\x91\x90a\x02\xEFV[`@Q\x80\x91\x03\x90\xF3[a\0s`\x04\x806\x03\x81\x01\x90a\0n\x91\x90a\x06,V[a\0\x8FV[`@Qa\0\x81\x92\x91\x90a\x075V[`@Q\x80\x91\x03\x90\xF3[`\xF5\x81V[`\0```\0a\0\xA0\x86\x86\x86a\x01!V[\x90P`\0\x80`\xF5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\0\xCB\x91\x90a\x07\xA1V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x01\x06W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x0BV[``\x91P[P\x91P\x91P\x81\x81\x94P\x94PPPP\x93P\x93\x91PPV[```\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01?Wa\x01>a\x03jV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x01qW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0\x80[\x85Q\x81\x10\x15a\x02\nW\x84\x86\x82\x81Q\x81\x10a\x01\x95Wa\x01\x94a\x07\xB8V[[` \x02` \x01\x01Q` \x01Q\x03a\x01\xF7W\x82\x86\x82\x81Q\x81\x10a\x01\xBAWa\x01\xB9a\x07\xB8V[[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\x01\xD7\x92\x91\x90a\x07\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\x01\xF3\x90a\x08JV[\x92PP[\x80\x80a\x02\x02\x90a\x08vV[\x91PPa\x01xV[P`\x02\x84\x03a\x02\x1CW`\x01\x93Pa\x02*V[`\x03\x84\x03a\x02)W`\0\x93P[[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\t\xF3`+\x919\x90P`\0\x85`\xF8\x1B\x90P`\0` `\xFF\x16`\xE0\x1B\x90P`\0\x83Q`\xE0\x1B\x90P`\0\x85`\xE0\x1B\x90P`\0\x84\x84\x8D\x85\x89\x86\x8D`@Q` \x01a\x02\x8B\x97\x96\x95\x94\x93\x92\x91\x90a\tyV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x98PPPPPPPPP\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x02\xD9\x82a\x02\xAEV[\x90P\x91\x90PV[a\x02\xE9\x81a\x02\xCEV[\x82RPPV[`\0` \x82\x01\x90Pa\x03\x04`\0\x83\x01\x84a\x02\xE0V[\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x031\x81a\x03\x1EV[\x81\x14a\x03<W`\0\x80\xFD[PV[`\0\x815\x90Pa\x03N\x81a\x03(V[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x03\xA2\x82a\x03YV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x03\xC1Wa\x03\xC0a\x03jV[[\x80`@RPPPV[`\0a\x03\xD4a\x03\nV[\x90Pa\x03\xE0\x82\x82a\x03\x99V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04\0Wa\x03\xFFa\x03jV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04@Wa\x04?a\x03jV[[a\x04I\x82a\x03YV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x04xa\x04s\x84a\x04%V[a\x03\xCAV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x04\x94Wa\x04\x93a\x04 V[[a\x04\x9F\x84\x82\x85a\x04VV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x04\xBCWa\x04\xBBa\x03TV[[\x815a\x04\xCC\x84\x82` \x86\x01a\x04eV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x04\xE8\x81a\x04\xD5V[\x81\x14a\x04\xF3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\x05\x81a\x04\xDFV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x05!Wa\x05 a\x04\x16V[[a\x05+`@a\x03\xCAV[\x90P`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05KWa\x05Ja\x04\x1BV[[a\x05W\x84\x82\x85\x01a\x04\xA7V[`\0\x83\x01RP` a\x05k\x84\x82\x85\x01a\x04\xF6V[` \x83\x01RP\x92\x91PPV[`\0a\x05\x8Aa\x05\x85\x84a\x03\xE5V[a\x03\xCAV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x05\xADWa\x05\xACa\x04\x11V[[\x83[\x81\x81\x10\x15a\x05\xF4W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xD2Wa\x05\xD1a\x03TV[[\x80\x86\x01a\x05\xDF\x89\x82a\x05\x0BV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x05\xAFV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x06\x13Wa\x06\x12a\x03TV[[\x815a\x06#\x84\x82` \x86\x01a\x05wV[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06EWa\x06Da\x03\x14V[[`\0a\x06S\x86\x82\x87\x01a\x03?V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06tWa\x06sa\x03\x19V[[a\x06\x80\x86\x82\x87\x01a\x05\xFEV[\x92PP`@a\x06\x91\x86\x82\x87\x01a\x04\xF6V[\x91PP\x92P\x92P\x92V[`\0\x81\x15\x15\x90P\x91\x90PV[a\x06\xB0\x81a\x06\x9BV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x06\xF0W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x06\xD5V[`\0\x84\x84\x01RPPPPV[`\0a\x07\x07\x82a\x06\xB6V[a\x07\x11\x81\x85a\x06\xC1V[\x93Pa\x07!\x81\x85` \x86\x01a\x06\xD2V[a\x07*\x81a\x03YV[\x84\x01\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa\x07J`\0\x83\x01\x85a\x06\xA7V[\x81\x81\x03` \x83\x01Ra\x07\\\x81\x84a\x06\xFCV[\x90P\x93\x92PPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x07{\x82a\x06\xB6V[a\x07\x85\x81\x85a\x07eV[\x93Pa\x07\x95\x81\x85` \x86\x01a\x06\xD2V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x07\xAD\x82\x84a\x07pV[\x91P\x81\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a\x07\xF3\x82\x85a\x07pV[\x91Pa\x07\xFF\x82\x84a\x07pV[\x91P\x81\x90P\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x08U\x82a\x08:V[\x91Pc\xFF\xFF\xFF\xFF\x82\x03a\x08kWa\x08ja\x08\x0BV[[`\x01\x82\x01\x90P\x91\x90PV[`\0a\x08\x81\x82a\x04\xD5V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x08\xB3Wa\x08\xB2a\x08\x0BV[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\t\x05a\t\0\x82a\x08\xBEV[a\x08\xEAV[\x82RPPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\tRa\tM\x82a\t\x0BV[a\t7V[\x82RPPV[`\0\x81\x90P\x91\x90PV[a\tsa\tn\x82a\x03\x1EV[a\tXV[\x82RPPV[`\0a\t\x85\x82\x8Aa\x08\xF4V[`\x01\x82\x01\x91Pa\t\x95\x82\x89a\tAV[`\x04\x82\x01\x91Pa\t\xA5\x82\x88a\tbV[` \x82\x01\x91Pa\t\xB5\x82\x87a\tAV[`\x04\x82\x01\x91Pa\t\xC5\x82\x86a\x07pV[\x91Pa\t\xD1\x82\x85a\tAV[`\x04\x82\x01\x91Pa\t\xE1\x82\x84a\x07pV[\x91P\x81\x90P\x98\x97PPPPPPPPV\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 \n\xDD'\xA1\x07\x1C\xB4\xE3(=\xF5;\xB5M\x0E\xF5\xC1\x13\x17\x0F\x81P\xC2w\x80\xBB\x06\xF7q\xD3\x95udsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static KEYDERIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cb\xE4\xC4d\x14a\0;W\x80c\xA3,+\x99\x14a\0YW[`\0\x80\xFD[a\0Ca\0\x8AV[`@Qa\0P\x91\x90a\x02\xEFV[`@Q\x80\x91\x03\x90\xF3[a\0s`\x04\x806\x03\x81\x01\x90a\0n\x91\x90a\x06,V[a\0\x8FV[`@Qa\0\x81\x92\x91\x90a\x075V[`@Q\x80\x91\x03\x90\xF3[`\xF5\x81V[`\0```\0a\0\xA0\x86\x86\x86a\x01!V[\x90P`\0\x80`\xF5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\0\xCB\x91\x90a\x07\xA1V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x01\x06W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x0BV[``\x91P[P\x91P\x91P\x81\x81\x94P\x94PPPP\x93P\x93\x91PPV[```\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01?Wa\x01>a\x03jV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x01qW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P`\0\x80[\x85Q\x81\x10\x15a\x02\nW\x84\x86\x82\x81Q\x81\x10a\x01\x95Wa\x01\x94a\x07\xB8V[[` \x02` \x01\x01Q` \x01Q\x03a\x01\xF7W\x82\x86\x82\x81Q\x81\x10a\x01\xBAWa\x01\xB9a\x07\xB8V[[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\x01\xD7\x92\x91\x90a\x07\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\x01\xF3\x90a\x08JV[\x92PP[\x80\x80a\x02\x02\x90a\x08vV[\x91PPa\x01xV[P`\x02\x84\x03a\x02\x1CW`\x01\x93Pa\x02*V[`\x03\x84\x03a\x02)W`\0\x93P[[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\t\xF3`+\x919\x90P`\0\x85`\xF8\x1B\x90P`\0` `\xFF\x16`\xE0\x1B\x90P`\0\x83Q`\xE0\x1B\x90P`\0\x85`\xE0\x1B\x90P`\0\x84\x84\x8D\x85\x89\x86\x8D`@Q` \x01a\x02\x8B\x97\x96\x95\x94\x93\x92\x91\x90a\tyV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x98PPPPPPPPP\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x02\xD9\x82a\x02\xAEV[\x90P\x91\x90PV[a\x02\xE9\x81a\x02\xCEV[\x82RPPV[`\0` \x82\x01\x90Pa\x03\x04`\0\x83\x01\x84a\x02\xE0V[\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x031\x81a\x03\x1EV[\x81\x14a\x03<W`\0\x80\xFD[PV[`\0\x815\x90Pa\x03N\x81a\x03(V[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x03\xA2\x82a\x03YV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x03\xC1Wa\x03\xC0a\x03jV[[\x80`@RPPPV[`\0a\x03\xD4a\x03\nV[\x90Pa\x03\xE0\x82\x82a\x03\x99V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04\0Wa\x03\xFFa\x03jV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04@Wa\x04?a\x03jV[[a\x04I\x82a\x03YV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x04xa\x04s\x84a\x04%V[a\x03\xCAV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x04\x94Wa\x04\x93a\x04 V[[a\x04\x9F\x84\x82\x85a\x04VV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x04\xBCWa\x04\xBBa\x03TV[[\x815a\x04\xCC\x84\x82` \x86\x01a\x04eV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x04\xE8\x81a\x04\xD5V[\x81\x14a\x04\xF3W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\x05\x81a\x04\xDFV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x05!Wa\x05 a\x04\x16V[[a\x05+`@a\x03\xCAV[\x90P`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05KWa\x05Ja\x04\x1BV[[a\x05W\x84\x82\x85\x01a\x04\xA7V[`\0\x83\x01RP` a\x05k\x84\x82\x85\x01a\x04\xF6V[` \x83\x01RP\x92\x91PPV[`\0a\x05\x8Aa\x05\x85\x84a\x03\xE5V[a\x03\xCAV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x05\xADWa\x05\xACa\x04\x11V[[\x83[\x81\x81\x10\x15a\x05\xF4W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xD2Wa\x05\xD1a\x03TV[[\x80\x86\x01a\x05\xDF\x89\x82a\x05\x0BV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x05\xAFV[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x06\x13Wa\x06\x12a\x03TV[[\x815a\x06#\x84\x82` \x86\x01a\x05wV[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06EWa\x06Da\x03\x14V[[`\0a\x06S\x86\x82\x87\x01a\x03?V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06tWa\x06sa\x03\x19V[[a\x06\x80\x86\x82\x87\x01a\x05\xFEV[\x92PP`@a\x06\x91\x86\x82\x87\x01a\x04\xF6V[\x91PP\x92P\x92P\x92V[`\0\x81\x15\x15\x90P\x91\x90PV[a\x06\xB0\x81a\x06\x9BV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x06\xF0W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x06\xD5V[`\0\x84\x84\x01RPPPPV[`\0a\x07\x07\x82a\x06\xB6V[a\x07\x11\x81\x85a\x06\xC1V[\x93Pa\x07!\x81\x85` \x86\x01a\x06\xD2V[a\x07*\x81a\x03YV[\x84\x01\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa\x07J`\0\x83\x01\x85a\x06\xA7V[\x81\x81\x03` \x83\x01Ra\x07\\\x81\x84a\x06\xFCV[\x90P\x93\x92PPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x07{\x82a\x06\xB6V[a\x07\x85\x81\x85a\x07eV[\x93Pa\x07\x95\x81\x85` \x86\x01a\x06\xD2V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x07\xAD\x82\x84a\x07pV[\x91P\x81\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0a\x07\xF3\x82\x85a\x07pV[\x91Pa\x07\xFF\x82\x84a\x07pV[\x91P\x81\x90P\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x08U\x82a\x08:V[\x91Pc\xFF\xFF\xFF\xFF\x82\x03a\x08kWa\x08ja\x08\x0BV[[`\x01\x82\x01\x90P\x91\x90PV[`\0a\x08\x81\x82a\x04\xD5V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x08\xB3Wa\x08\xB2a\x08\x0BV[[`\x01\x82\x01\x90P\x91\x90PV[`\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\t\x05a\t\0\x82a\x08\xBEV[a\x08\xEAV[\x82RPPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\tRa\tM\x82a\t\x0BV[a\t7V[\x82RPPV[`\0\x81\x90P\x91\x90PV[a\tsa\tn\x82a\x03\x1EV[a\tXV[\x82RPPV[`\0a\t\x85\x82\x8Aa\x08\xF4V[`\x01\x82\x01\x91Pa\t\x95\x82\x89a\tAV[`\x04\x82\x01\x91Pa\t\xA5\x82\x88a\tbV[` \x82\x01\x91Pa\t\xB5\x82\x87a\tAV[`\x04\x82\x01\x91Pa\t\xC5\x82\x86a\x07pV[\x91Pa\t\xD1\x82\x85a\tAV[`\x04\x82\x01\x91Pa\t\xE1\x82\x84a\x07pV[\x91P\x81\x90P\x98\x97PPPPPPPPV\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 \n\xDD'\xA1\x07\x1C\xB4\xE3(=\xF5;\xB5M\x0E\xF5\xC1\x13\x17\x0F\x81P\xC2w\x80\xBB\x06\xF7q\xD3\x95udsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static KEYDERIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct KeyDeriver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for KeyDeriver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for KeyDeriver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for KeyDeriver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for KeyDeriver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(KeyDeriver)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> KeyDeriver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KEYDERIVER_ABI.clone(),
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
                KEYDERIVER_ABI.clone(),
                KEYDERIVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `HD_KDF` (0x62e4c464) function
        pub fn hd_kdf(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([98, 228, 196, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeHDPubKey` (0xa32c2b99) function
        pub fn compute_hd_pub_key(
            &self,
            derived_key_id: [u8; 32],
            root_hd_keys: ::std::vec::Vec<RootKey>,
            key_type: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash(
                    [163, 44, 43, 153],
                    (derived_key_id, root_hd_keys, key_type),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for KeyDeriver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `HD_KDF` function with signature `HD_KDF()` and selector `0x62e4c464`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "HD_KDF", abi = "HD_KDF()")]
    pub struct HdKdfCall;
    ///Container type for all input parameters for the `computeHDPubKey` function with signature `computeHDPubKey(bytes32,(bytes,uint256)[],uint256)` and selector `0xa32c2b99`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "computeHDPubKey",
        abi = "computeHDPubKey(bytes32,(bytes,uint256)[],uint256)"
    )]
    pub struct ComputeHDPubKeyCall {
        pub derived_key_id: [u8; 32],
        pub root_hd_keys: ::std::vec::Vec<RootKey>,
        pub key_type: ::ethers::core::types::U256,
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
    pub enum KeyDeriverCalls {
        HdKdf(HdKdfCall),
        ComputeHDPubKey(ComputeHDPubKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for KeyDeriverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <HdKdfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HdKdf(decoded));
            }
            if let Ok(decoded)
                = <ComputeHDPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ComputeHDPubKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KeyDeriverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HdKdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeHDPubKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for KeyDeriverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HdKdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHDPubKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HdKdfCall> for KeyDeriverCalls {
        fn from(value: HdKdfCall) -> Self {
            Self::HdKdf(value)
        }
    }
    impl ::core::convert::From<ComputeHDPubKeyCall> for KeyDeriverCalls {
        fn from(value: ComputeHDPubKeyCall) -> Self {
            Self::ComputeHDPubKey(value)
        }
    }
    ///Container type for all return fields from the `HD_KDF` function with signature `HD_KDF()` and selector `0x62e4c464`
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
    pub struct HdKdfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `computeHDPubKey` function with signature `computeHDPubKey(bytes32,(bytes,uint256)[],uint256)` and selector `0xa32c2b99`
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
    pub struct ComputeHDPubKeyReturn(pub bool, pub ::ethers::core::types::Bytes);
    ///`RootKey(bytes,uint256)`
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
    pub struct RootKey {
        pub pubkey: ::ethers::core::types::Bytes,
        pub key_type: ::ethers::core::types::U256,
    }
}
