#![allow(warnings)]
pub use data_proving_contract::*;
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
pub mod data_proving_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("get"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get"),
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
                    ::std::borrow::ToOwned::to_owned("save"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("save"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hash"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DATAPROVINGCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03j\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c8\xE4\x8F\x06\x14a\0;W\x80cmL\xE6<\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\x12V[a\0nV[\0[a\0Xa\0\x80V[`@Qa\0e\x91\x90a\x01\x84V[`@Q\x80\x91\x03\x90\xF3[`\0a\0{\x82\x84\x83a\x02sV[PPPV[```\0\x80Ta\0\x8F\x90a\x01\xE9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\0\xBB\x90a\x01\xE9V[\x80\x15a\x01\x08W\x80`\x1F\x10a\0\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x08V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\0\xEBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80` \x83\x85\x03\x12\x15a\x01%W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01=W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x01QW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01`W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x01rW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01\xB2W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\x96V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xFDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\x1DWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\0{W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x02LWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02kW\x82\x81U`\x01\x01a\x02XV[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x02\x8BWa\x02\x8Ba\x01\xD3V[a\x02\x9F\x83a\x02\x99\x83Ta\x01\xE9V[\x83a\x02#V[`\0`\x1F\x84\x11`\x01\x81\x14a\x02\xD3W`\0\x85\x15a\x02\xBBWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x03-V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x03\x04W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xE4V[P\x86\x82\x10\x15a\x03!W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV\xFE\xA2dipfsX\"\x12 \x7F\x90\x1C{<\x94a\x15/\xE9q-\xA9\x91F\x87\xAC\x9BW\x1A\xF4\x81@RZ\xF4\xA2\x98\xA43\xF3udsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static DATAPROVINGCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c8\xE4\x8F\x06\x14a\0;W\x80cmL\xE6<\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01\x12V[a\0nV[\0[a\0Xa\0\x80V[`@Qa\0e\x91\x90a\x01\x84V[`@Q\x80\x91\x03\x90\xF3[`\0a\0{\x82\x84\x83a\x02sV[PPPV[```\0\x80Ta\0\x8F\x90a\x01\xE9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\0\xBB\x90a\x01\xE9V[\x80\x15a\x01\x08W\x80`\x1F\x10a\0\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x08V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\0\xEBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80` \x83\x85\x03\x12\x15a\x01%W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01=W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x01QW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01`W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x01rW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01\xB2W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\x96V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xFDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\x1DWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\0{W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x02LWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02kW\x82\x81U`\x01\x01a\x02XV[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x02\x8BWa\x02\x8Ba\x01\xD3V[a\x02\x9F\x83a\x02\x99\x83Ta\x01\xE9V[\x83a\x02#V[`\0`\x1F\x84\x11`\x01\x81\x14a\x02\xD3W`\0\x85\x15a\x02\xBBWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x03-V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x03\x04W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xE4V[P\x86\x82\x10\x15a\x03!W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV\xFE\xA2dipfsX\"\x12 \x7F\x90\x1C{<\x94a\x15/\xE9q-\xA9\x91F\x87\xAC\x9BW\x1A\xF4\x81@RZ\xF4\xA2\x98\xA43\xF3udsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static DATAPROVINGCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DataProvingContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DataProvingContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DataProvingContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DataProvingContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DataProvingContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DataProvingContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DataProvingContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DATAPROVINGCONTRACT_ABI.clone(),
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
                DATAPROVINGCONTRACT_ABI.clone(),
                DATAPROVINGCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `get` (0x6d4ce63c) function
        pub fn get(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([109, 76, 230, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `save` (0x38e48f06) function
        pub fn save(
            &self,
            hash: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 228, 143, 6], hash)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DataProvingContract<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `get` function with signature `get()` and selector `0x6d4ce63c`
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
    #[ethcall(name = "get", abi = "get()")]
    pub struct GetCall;
    ///Container type for all input parameters for the `save` function with signature `save(string)` and selector `0x38e48f06`
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
    #[ethcall(name = "save", abi = "save(string)")]
    pub struct SaveCall {
        pub hash: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DataProvingContractCalls {
        Get(GetCall),
        Save(SaveCall),
    }
    impl ::ethers::core::abi::AbiDecode for DataProvingContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <SaveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Save(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DataProvingContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Save(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DataProvingContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::Save(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCall> for DataProvingContractCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<SaveCall> for DataProvingContractCalls {
        fn from(value: SaveCall) -> Self {
            Self::Save(value)
        }
    }
    ///Container type for all return fields from the `get` function with signature `get()` and selector `0x6d4ce63c`
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
    pub struct GetReturn(pub ::std::string::String);
}
