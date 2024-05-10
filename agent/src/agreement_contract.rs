#![allow(warnings)]
pub use agreement_contract::*;
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
pub mod agreement_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("create"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("create"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("entity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("get"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("station"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("entity"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Agreement"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sign"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sign"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("station"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("Created"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Created"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("Signed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Signed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ErrAlreadySigned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrAlreadySigned"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrInvalidAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrInvalidAmount"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrNoAgreement"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrNoAgreement"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static AGREEMENTCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x04Z\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x0E\xCA\xEAs\x14a\0FW\x80c_\xF6Om\x14a\0[W\x80c\xD8\x1E\x84#\x14a\0nW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x03xV[a\0\x97V[\0[a\0Ya\0i6`\x04a\x03xV[a\x01\x9BV[a\0\x81a\0|6`\x04a\x03\xA2V[a\x02\xA5V[`@Qa\0\x8E\x91\x90a\x03\xEBV[`@Q\x80\x91\x03\x90\xF3[`\x023`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\0\xD2Wa\0\xD2a\x03\xD5V[\x03a\0\xF0W`@Qcl\xB2\xEA\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x81\x81R` \x81\x01`\x01\x90R3`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x82R\x90\x91 \x82Q\x81U\x90\x82\x01Q`\x01\x80\x83\x01\x80T\x90\x91`\xFF\x19\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\x01PWa\x01Pa\x03\xD5V[\x02\x17\x90UPP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91P3\x90\x7F\x82+0s\xBEb\xC5\xC7\xF1C\xC2\xDC\xD7\x1E\xE2fCN\xE95\xD9\n\x1E\xEC;\xE3G\x10\xAC\x8E\xC1\xA2\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x90`\x01\x82\x01T`\xFF\x16`\x02\x81\x11\x15a\x01\xD6Wa\x01\xD6a\x03\xD5V[\x03a\x01\xF7W`@Q`\x01b\x0Cs\xB1`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x82\x01T`\xFF\x16`\x02\x81\x11\x15a\x02\x12Wa\x02\x12a\x03\xD5V[\x03a\x020W`@Qcl\xB2\xEA\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T\x82\x14a\x02[W\x80T`@Qc2\xBE\xDEG`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@Q3\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F.\xE4\xF8\x12\x1E\xE0\xC4\x95S\x1Esq\xA9\x91\xF0i9\xBB`\xD0\xE1dM\x9Fe\x03\xFD0\xDFR\xBA\xAD\x90`\0\x90\xA3PPPV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83R\x82\x82R\x84\x83 \x90\x86\x16\x83R\x81R\x83\x82 \x84Q\x80\x86\x01\x90\x95R\x80T\x85R`\x01\x81\x01T\x93\x94\x92\x93\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x03\x06Wa\x03\x06a\x03\xD5V[`\x02\x81\x11\x15a\x03\x17Wa\x03\x17a\x03\xD5V[\x90RP\x90P`\0\x81` \x01Q`\x02\x81\x11\x15a\x034Wa\x034a\x03\xD5V[\x03a\x03UW`@Q`\x01b\x0Cs\xB1`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03sW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\x8BW`\0\x80\xFD[a\x03\x94\x83a\x03\\V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\xB5W`\0\x80\xFD[a\x03\xBE\x83a\x03\\V[\x91Pa\x03\xCC` \x84\x01a\x03\\V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81Q\x81R` \x82\x01Q`@\x82\x01\x90`\x03\x81\x10a\x04\x17WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80` \x84\x01RP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x1F\x1B\x85\xA2\x13\x91\xAF,l\x13\xE1\xCA\x1Eil\xFF\xA9T\xCA\xD7\x85\xACN\xDF?\x99\x817\xFA\xFFv\xE6dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static AGREEMENTCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x0E\xCA\xEAs\x14a\0FW\x80c_\xF6Om\x14a\0[W\x80c\xD8\x1E\x84#\x14a\0nW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x03xV[a\0\x97V[\0[a\0Ya\0i6`\x04a\x03xV[a\x01\x9BV[a\0\x81a\0|6`\x04a\x03\xA2V[a\x02\xA5V[`@Qa\0\x8E\x91\x90a\x03\xEBV[`@Q\x80\x91\x03\x90\xF3[`\x023`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\0\xD2Wa\0\xD2a\x03\xD5V[\x03a\0\xF0W`@Qcl\xB2\xEA\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x81\x81R` \x81\x01`\x01\x90R3`\0\x90\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x82R\x90\x91 \x82Q\x81U\x90\x82\x01Q`\x01\x80\x83\x01\x80T\x90\x91`\xFF\x19\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\x01PWa\x01Pa\x03\xD5V[\x02\x17\x90UPP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91P3\x90\x7F\x82+0s\xBEb\xC5\xC7\xF1C\xC2\xDC\xD7\x1E\xE2fCN\xE95\xD9\n\x1E\xEC;\xE3G\x10\xAC\x8E\xC1\xA2\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x90`\x01\x82\x01T`\xFF\x16`\x02\x81\x11\x15a\x01\xD6Wa\x01\xD6a\x03\xD5V[\x03a\x01\xF7W`@Q`\x01b\x0Cs\xB1`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x82\x01T`\xFF\x16`\x02\x81\x11\x15a\x02\x12Wa\x02\x12a\x03\xD5V[\x03a\x020W`@Qcl\xB2\xEA\x17`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T\x82\x14a\x02[W\x80T`@Qc2\xBE\xDEG`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@Q3\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F.\xE4\xF8\x12\x1E\xE0\xC4\x95S\x1Esq\xA9\x91\xF0i9\xBB`\xD0\xE1dM\x9Fe\x03\xFD0\xDFR\xBA\xAD\x90`\0\x90\xA3PPPV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83R\x82\x82R\x84\x83 \x90\x86\x16\x83R\x81R\x83\x82 \x84Q\x80\x86\x01\x90\x95R\x80T\x85R`\x01\x81\x01T\x93\x94\x92\x93\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x03\x06Wa\x03\x06a\x03\xD5V[`\x02\x81\x11\x15a\x03\x17Wa\x03\x17a\x03\xD5V[\x90RP\x90P`\0\x81` \x01Q`\x02\x81\x11\x15a\x034Wa\x034a\x03\xD5V[\x03a\x03UW`@Q`\x01b\x0Cs\xB1`\xE2\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03sW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\x8BW`\0\x80\xFD[a\x03\x94\x83a\x03\\V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\xB5W`\0\x80\xFD[a\x03\xBE\x83a\x03\\V[\x91Pa\x03\xCC` \x84\x01a\x03\\V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81Q\x81R` \x82\x01Q`@\x82\x01\x90`\x03\x81\x10a\x04\x17WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80` \x84\x01RP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x1F\x1B\x85\xA2\x13\x91\xAF,l\x13\xE1\xCA\x1Eil\xFF\xA9T\xCA\xD7\x85\xACN\xDF?\x99\x817\xFA\xFFv\xE6dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static AGREEMENTCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AgreementContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AgreementContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AgreementContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AgreementContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AgreementContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AgreementContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AgreementContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AGREEMENTCONTRACT_ABI.clone(),
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
                AGREEMENTCONTRACT_ABI.clone(),
                AGREEMENTCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `create` (0x0ecaea73) function
        pub fn create(
            &self,
            entity: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 202, 234, 115], (entity, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get` (0xd81e8423) function
        pub fn get(
            &self,
            station: ::ethers::core::types::Address,
            entity: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Agreement> {
            self.0
                .method_hash([216, 30, 132, 35], (station, entity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sign` (0x5ff64f6d) function
        pub fn sign(
            &self,
            station: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 246, 79, 109], (station, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Created` event
        pub fn created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CreatedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Signed` event
        pub fn signed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementContractEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AgreementContract<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ErrAlreadySigned` with signature `ErrAlreadySigned()` and selector `0xd965d42e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ErrAlreadySigned", abi = "ErrAlreadySigned()")]
    pub struct ErrAlreadySigned;
    ///Custom Error type `ErrInvalidAmount` with signature `ErrInvalidAmount(uint256)` and selector `0xcafb791c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ErrInvalidAmount", abi = "ErrInvalidAmount(uint256)")]
    pub struct ErrInvalidAmount(pub ::ethers::core::types::U256);
    ///Custom Error type `ErrNoAgreement` with signature `ErrNoAgreement()` and selector `0xffce313c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ErrNoAgreement", abi = "ErrNoAgreement()")]
    pub struct ErrNoAgreement;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AgreementContractErrors {
        ErrAlreadySigned(ErrAlreadySigned),
        ErrInvalidAmount(ErrInvalidAmount),
        ErrNoAgreement(ErrNoAgreement),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AgreementContractErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ErrAlreadySigned as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrAlreadySigned(decoded));
            }
            if let Ok(decoded) = <ErrInvalidAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrInvalidAmount(decoded));
            }
            if let Ok(decoded) = <ErrNoAgreement as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrNoAgreement(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AgreementContractErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ErrAlreadySigned(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrInvalidAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrNoAgreement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AgreementContractErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ErrAlreadySigned as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrInvalidAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrNoAgreement as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AgreementContractErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ErrAlreadySigned(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrInvalidAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrNoAgreement(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AgreementContractErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ErrAlreadySigned> for AgreementContractErrors {
        fn from(value: ErrAlreadySigned) -> Self {
            Self::ErrAlreadySigned(value)
        }
    }
    impl ::core::convert::From<ErrInvalidAmount> for AgreementContractErrors {
        fn from(value: ErrInvalidAmount) -> Self {
            Self::ErrInvalidAmount(value)
        }
    }
    impl ::core::convert::From<ErrNoAgreement> for AgreementContractErrors {
        fn from(value: ErrNoAgreement) -> Self {
            Self::ErrNoAgreement(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Created", abi = "Created(address,address,uint256)")]
    pub struct CreatedFilter(
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Signed", abi = "Signed(address,address)")]
    pub struct SignedFilter(
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
    );
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AgreementContractEvents {
        CreatedFilter(CreatedFilter),
        SignedFilter(SignedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AgreementContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CreatedFilter::decode_log(log) {
                return Ok(AgreementContractEvents::CreatedFilter(decoded));
            }
            if let Ok(decoded) = SignedFilter::decode_log(log) {
                return Ok(AgreementContractEvents::SignedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AgreementContractEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreatedFilter> for AgreementContractEvents {
        fn from(value: CreatedFilter) -> Self {
            Self::CreatedFilter(value)
        }
    }
    impl ::core::convert::From<SignedFilter> for AgreementContractEvents {
        fn from(value: SignedFilter) -> Self {
            Self::SignedFilter(value)
        }
    }
    ///Container type for all input parameters for the `create` function with signature `create(address,uint256)` and selector `0x0ecaea73`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "create", abi = "create(address,uint256)")]
    pub struct CreateCall {
        pub entity: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get` function with signature `get(address,address)` and selector `0xd81e8423`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "get", abi = "get(address,address)")]
    pub struct GetCall {
        pub station: ::ethers::core::types::Address,
        pub entity: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sign` function with signature `sign(address,uint256)` and selector `0x5ff64f6d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sign", abi = "sign(address,uint256)")]
    pub struct SignCall {
        pub station: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AgreementContractCalls {
        Create(CreateCall),
        Get(GetCall),
        Sign(SignCall),
    }
    impl ::ethers::core::abi::AbiDecode for AgreementContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CreateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Create(decoded));
            }
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <SignCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sign(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AgreementContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Create(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sign(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AgreementContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Create(element) => ::core::fmt::Display::fmt(element, f),
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sign(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateCall> for AgreementContractCalls {
        fn from(value: CreateCall) -> Self {
            Self::Create(value)
        }
    }
    impl ::core::convert::From<GetCall> for AgreementContractCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<SignCall> for AgreementContractCalls {
        fn from(value: SignCall) -> Self {
            Self::Sign(value)
        }
    }
    ///Container type for all return fields from the `get` function with signature `get(address,address)` and selector `0xd81e8423`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetReturn(pub Agreement);
    ///`Agreement(uint256,uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Agreement {
        pub amount: ::ethers::core::types::U256,
        pub status: u8,
    }
}
