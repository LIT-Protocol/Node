pub use dev_key_deriver::*;
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
pub mod dev_key_deriver {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/DevKeyDeriver.json",
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
    pub static DEVKEYDERIVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x05d\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xA3,+\x99\x14a\x000W[`\0\x80\xFD[a\0J`\x04\x806\x03\x81\x01\x90a\0E\x91\x90a\x03\xB4V[a\0aV[`@Qa\0X\x92\x91\x90a\x04\xBDV[`@Q\x80\x91\x03\x90\xF3[`\0```\0`@Q\x80`\x80\x01`@R\x80`A\x81R` \x01a\x04\xEE`A\x919\x90P`\x01\x81\x92P\x92PP\x93P\x93\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\0\xB9\x81a\0\xA6V[\x81\x14a\0\xC4W`\0\x80\xFD[PV[`\0\x815\x90Pa\0\xD6\x81a\0\xB0V[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x01*\x82a\0\xE1V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x01IWa\x01Ha\0\xF2V[[\x80`@RPPPV[`\0a\x01\\a\0\x92V[\x90Pa\x01h\x82\x82a\x01!V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x01\x88Wa\x01\x87a\0\xF2V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x01\xC8Wa\x01\xC7a\0\xF2V[[a\x01\xD1\x82a\0\xE1V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x02\0a\x01\xFB\x84a\x01\xADV[a\x01RV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x02\x1CWa\x02\x1Ba\x01\xA8V[[a\x02'\x84\x82\x85a\x01\xDEV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x02DWa\x02Ca\0\xDCV[[\x815a\x02T\x84\x82` \x86\x01a\x01\xEDV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x02p\x81a\x02]V[\x81\x14a\x02{W`\0\x80\xFD[PV[`\0\x815\x90Pa\x02\x8D\x81a\x02gV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x02\xA9Wa\x02\xA8a\x01\x9EV[[a\x02\xB3`@a\x01RV[\x90P`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xD3Wa\x02\xD2a\x01\xA3V[[a\x02\xDF\x84\x82\x85\x01a\x02/V[`\0\x83\x01RP` a\x02\xF3\x84\x82\x85\x01a\x02~V[` \x83\x01RP\x92\x91PPV[`\0a\x03\x12a\x03\r\x84a\x01mV[a\x01RV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x035Wa\x034a\x01\x99V[[\x83[\x81\x81\x10\x15a\x03|W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03ZWa\x03Ya\0\xDCV[[\x80\x86\x01a\x03g\x89\x82a\x02\x93V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x037V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x03\x9BWa\x03\x9Aa\0\xDCV[[\x815a\x03\xAB\x84\x82` \x86\x01a\x02\xFFV[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03\xCDWa\x03\xCCa\0\x9CV[[`\0a\x03\xDB\x86\x82\x87\x01a\0\xC7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xFCWa\x03\xFBa\0\xA1V[[a\x04\x08\x86\x82\x87\x01a\x03\x86V[\x92PP`@a\x04\x19\x86\x82\x87\x01a\x02~V[\x91PP\x92P\x92P\x92V[`\0\x81\x15\x15\x90P\x91\x90PV[a\x048\x81a\x04#V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x04xW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x04]V[`\0\x84\x84\x01RPPPPV[`\0a\x04\x8F\x82a\x04>V[a\x04\x99\x81\x85a\x04IV[\x93Pa\x04\xA9\x81\x85` \x86\x01a\x04ZV[a\x04\xB2\x81a\0\xE1V[\x84\x01\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa\x04\xD2`\0\x83\x01\x85a\x04/V[\x81\x81\x03` \x83\x01Ra\x04\xE4\x81\x84a\x04\x84V[\x90P\x93\x92PPPV\xFE\x04|6G4P Sn\x8A\xAC\xCA\xC7\xF7<RH\xBF6\tgy\x97\xFBa\\)\x0C\xC5\x8E\x8A\xC1\xDC\xAD\x1F\xA1\xD4\xF6\xEE\xDFQo\x02=\xEE\x11\xFB\xC0c\x10CLZ~\xE4\x0F_\x8CI\xE2U\xB1\xD1\xBF\xB6\xA2dipfsX\"\x12 OrKT\xD1\xF1rVD\xDATh\x81mE\x13\xAF\x9D\x8C\xE0\xFE\xBC\xCA_\xCB\xE4\x8ByL\xF0\xBC\x9AdsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static DEVKEYDERIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xA3,+\x99\x14a\x000W[`\0\x80\xFD[a\0J`\x04\x806\x03\x81\x01\x90a\0E\x91\x90a\x03\xB4V[a\0aV[`@Qa\0X\x92\x91\x90a\x04\xBDV[`@Q\x80\x91\x03\x90\xF3[`\0```\0`@Q\x80`\x80\x01`@R\x80`A\x81R` \x01a\x04\xEE`A\x919\x90P`\x01\x81\x92P\x92PP\x93P\x93\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\0\xB9\x81a\0\xA6V[\x81\x14a\0\xC4W`\0\x80\xFD[PV[`\0\x815\x90Pa\0\xD6\x81a\0\xB0V[\x92\x91PPV[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x01*\x82a\0\xE1V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x01IWa\x01Ha\0\xF2V[[\x80`@RPPPV[`\0a\x01\\a\0\x92V[\x90Pa\x01h\x82\x82a\x01!V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x01\x88Wa\x01\x87a\0\xF2V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x01\xC8Wa\x01\xC7a\0\xF2V[[a\x01\xD1\x82a\0\xE1V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x02\0a\x01\xFB\x84a\x01\xADV[a\x01RV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x02\x1CWa\x02\x1Ba\x01\xA8V[[a\x02'\x84\x82\x85a\x01\xDEV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x02DWa\x02Ca\0\xDCV[[\x815a\x02T\x84\x82` \x86\x01a\x01\xEDV[\x91PP\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x02p\x81a\x02]V[\x81\x14a\x02{W`\0\x80\xFD[PV[`\0\x815\x90Pa\x02\x8D\x81a\x02gV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x02\xA9Wa\x02\xA8a\x01\x9EV[[a\x02\xB3`@a\x01RV[\x90P`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xD3Wa\x02\xD2a\x01\xA3V[[a\x02\xDF\x84\x82\x85\x01a\x02/V[`\0\x83\x01RP` a\x02\xF3\x84\x82\x85\x01a\x02~V[` \x83\x01RP\x92\x91PPV[`\0a\x03\x12a\x03\r\x84a\x01mV[a\x01RV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x035Wa\x034a\x01\x99V[[\x83[\x81\x81\x10\x15a\x03|W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03ZWa\x03Ya\0\xDCV[[\x80\x86\x01a\x03g\x89\x82a\x02\x93V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x037V[PPP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x03\x9BWa\x03\x9Aa\0\xDCV[[\x815a\x03\xAB\x84\x82` \x86\x01a\x02\xFFV[\x91PP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03\xCDWa\x03\xCCa\0\x9CV[[`\0a\x03\xDB\x86\x82\x87\x01a\0\xC7V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xFCWa\x03\xFBa\0\xA1V[[a\x04\x08\x86\x82\x87\x01a\x03\x86V[\x92PP`@a\x04\x19\x86\x82\x87\x01a\x02~V[\x91PP\x92P\x92P\x92V[`\0\x81\x15\x15\x90P\x91\x90PV[a\x048\x81a\x04#V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x04xW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x04]V[`\0\x84\x84\x01RPPPPV[`\0a\x04\x8F\x82a\x04>V[a\x04\x99\x81\x85a\x04IV[\x93Pa\x04\xA9\x81\x85` \x86\x01a\x04ZV[a\x04\xB2\x81a\0\xE1V[\x84\x01\x91PP\x92\x91PPV[`\0`@\x82\x01\x90Pa\x04\xD2`\0\x83\x01\x85a\x04/V[\x81\x81\x03` \x83\x01Ra\x04\xE4\x81\x84a\x04\x84V[\x90P\x93\x92PPPV\xFE\x04|6G4P Sn\x8A\xAC\xCA\xC7\xF7<RH\xBF6\tgy\x97\xFBa\\)\x0C\xC5\x8E\x8A\xC1\xDC\xAD\x1F\xA1\xD4\xF6\xEE\xDFQo\x02=\xEE\x11\xFB\xC0c\x10CLZ~\xE4\x0F_\x8CI\xE2U\xB1\xD1\xBF\xB6\xA2dipfsX\"\x12 OrKT\xD1\xF1rVD\xDATh\x81mE\x13\xAF\x9D\x8C\xE0\xFE\xBC\xCA_\xCB\xE4\x8ByL\xF0\xBC\x9AdsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static DEVKEYDERIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DevKeyDeriver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DevKeyDeriver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DevKeyDeriver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DevKeyDeriver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DevKeyDeriver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DevKeyDeriver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DevKeyDeriver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEVKEYDERIVER_ABI.clone(),
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
                DEVKEYDERIVER_ABI.clone(),
                DEVKEYDERIVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    for DevKeyDeriver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
