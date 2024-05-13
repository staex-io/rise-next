pub use did_contract::*;
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
pub mod did_contract {
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
                            inputs: ::std::vec![
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
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DID"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("remove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("remove"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("location"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("Removed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Removed"),
                            inputs: ::std::vec![
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
                (
                    ::std::borrow::ToOwned::to_owned("Updated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Updated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("location"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
    pub static DIDCONTRACT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x078\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cN\xE5\xC2\xEE\x14a\0FW\x80c\xA7\xF47y\x14a\0[W\x80c\xC2\xBC.\xFC\x14a\0cW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x02\xF9V[a\0\x8CV[\0[a\0Ya\x01zV[a\0va\0q6`\x04a\x03qV[a\x01\xCAV[`@Qa\0\x83\x91\x90a\x03\xA1V[`@Q\x80\x91\x03\x90\xF3[3`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x91Qa\0\xAB\x91\x83\x91\x01a\x04>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84`@Q` \x01a\0\xD4\x92\x91\x90a\x04\xB4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\0\xFDW\x80a\0\xFB\x84\x86\x83a\x05+V[P[\x80`\x01\x01T\x82\x14a\x01\x10W`\x01\x81\x01\x82\x90U[3`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90\x80a\x01,\x83\x82a\x05\xECV[P`\x01\x91\x82\x01T\x91\x01U`@Q3\x90\x7F\xD8:\xB1\xDD\x183\xCB\xCB\x07aO\xD9\xA2\xCErK\xBE9\x8D@s\xD7\x99\xB8\xB1\xB5BR\x98\xC3\x04\x08\x90a\x01l\x90\x87\x90\x87\x90\x87\x90a\x06\xC9V[`@Q\x80\x91\x03\x90\xA2PPPPV[3`\0\x90\x81R` \x81\x90R`@\x81 \x90a\x01\x94\x82\x82a\x02\xA3V[P`\0`\x01\x91\x90\x91\x01\x81\x90U`@Q3\x91\x7F\x06j\x90[y\xC0\x12\x1A\xFEa\xE3\xA4N\x0B\x14\xB6\xBC\x1E\xC1m\x85L\xDB\xA0\x9E\xFD\xFC\x9Bj\xA9\xAF\x81\x91\xA2V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\x02\x10\x90a\x04\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02<\x90a\x04\x04V[\x80\x15a\x02\x89W\x80`\x1F\x10a\x02^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[P\x80Ta\x02\xAF\x90a\x04\x04V[`\0\x82U\x80`\x1F\x10a\x02\xBFWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x02\xDD\x91\x90a\x02\xE0V[PV[[\x80\x82\x11\x15a\x02\xF5W`\0\x81U`\x01\x01a\x02\xE1V[P\x90V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x03\x0EW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03&W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x03:W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x03IW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x03[W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x03\x83W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x9AW`\0\x80\xFD[\x93\x92PPPV[`\0` \x80\x83R\x83Q`@` \x85\x01R\x80Q\x80``\x86\x01R`\0[\x81\x81\x10\x15a\x03\xD8W\x82\x81\x01\x84\x01Q\x86\x82\x01`\x80\x01R\x83\x01a\x03\xBCV[P`\0`\x80\x82\x87\x01\x01R` \x86\x01Q`@\x86\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x86\x01\x01\x93PPPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x048WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80\x83Ta\x04L\x81a\x04\x04V[`\x01\x82\x81\x16\x80\x15a\x04dW`\x01\x81\x14a\x04yWa\x04\xA8V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x04\xA8V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x04\x9FW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x04\x86V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x05&W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x05\x03WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x05\"W\x82\x81U`\x01\x01a\x05\x0FV[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x05CWa\x05Ca\x04\xC4V[a\x05W\x83a\x05Q\x83Ta\x04\x04V[\x83a\x04\xDAV[`\0`\x1F\x84\x11`\x01\x81\x14a\x05\x8BW`\0\x85\x15a\x05sWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x05\xE5V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x05\xBCW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x05\x9CV[P\x86\x82\x10\x15a\x05\xD9W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x81\x81\x03a\x05\xF7WPPV[a\x06\x01\x82Ta\x04\x04V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x19Wa\x06\x19a\x04\xC4V[a\x06-\x81a\x06'\x84Ta\x04\x04V[\x84a\x04\xDAV[`\0`\x1F\x82\x11`\x01\x81\x14a\x06aW`\0\x83\x15a\x06IWP\x84\x82\x01T[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x05\xE5V[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90`\0\x86\x81R` \x90 \x84[\x83\x81\x10\x15a\x06\x9BW\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a\x06{V[P\x85\x83\x10\x15a\x06\xB9W\x81\x85\x01T`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R\x82`@\x82\x01R\x82\x84``\x83\x017`\0``\x84\x83\x01\x01R`\0```\x1F\x19`\x1F\x86\x01\x16\x83\x01\x01\x90P\x82` \x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 ?\xB3\xC3T\x83\xAC\xA6\x92\x875#\xD7_*grXx\xD4\r\x8F\xE9\xF5-Zd\xB9E\x86/d\ndsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static DIDCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cN\xE5\xC2\xEE\x14a\0FW\x80c\xA7\xF47y\x14a\0[W\x80c\xC2\xBC.\xFC\x14a\0cW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x02\xF9V[a\0\x8CV[\0[a\0Ya\x01zV[a\0va\0q6`\x04a\x03qV[a\x01\xCAV[`@Qa\0\x83\x91\x90a\x03\xA1V[`@Q\x80\x91\x03\x90\xF3[3`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x91Qa\0\xAB\x91\x83\x91\x01a\x04>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84`@Q` \x01a\0\xD4\x92\x91\x90a\x04\xB4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\0\xFDW\x80a\0\xFB\x84\x86\x83a\x05+V[P[\x80`\x01\x01T\x82\x14a\x01\x10W`\x01\x81\x01\x82\x90U[3`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90\x80a\x01,\x83\x82a\x05\xECV[P`\x01\x91\x82\x01T\x91\x01U`@Q3\x90\x7F\xD8:\xB1\xDD\x183\xCB\xCB\x07aO\xD9\xA2\xCErK\xBE9\x8D@s\xD7\x99\xB8\xB1\xB5BR\x98\xC3\x04\x08\x90a\x01l\x90\x87\x90\x87\x90\x87\x90a\x06\xC9V[`@Q\x80\x91\x03\x90\xA2PPPPV[3`\0\x90\x81R` \x81\x90R`@\x81 \x90a\x01\x94\x82\x82a\x02\xA3V[P`\0`\x01\x91\x90\x91\x01\x81\x90U`@Q3\x91\x7F\x06j\x90[y\xC0\x12\x1A\xFEa\xE3\xA4N\x0B\x14\xB6\xBC\x1E\xC1m\x85L\xDB\xA0\x9E\xFD\xFC\x9Bj\xA9\xAF\x81\x91\xA2V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\x02\x10\x90a\x04\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02<\x90a\x04\x04V[\x80\x15a\x02\x89W\x80`\x1F\x10a\x02^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[P\x80Ta\x02\xAF\x90a\x04\x04V[`\0\x82U\x80`\x1F\x10a\x02\xBFWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x02\xDD\x91\x90a\x02\xE0V[PV[[\x80\x82\x11\x15a\x02\xF5W`\0\x81U`\x01\x01a\x02\xE1V[P\x90V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x03\x0EW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03&W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x03:W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x03IW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x03[W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x03\x83W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x9AW`\0\x80\xFD[\x93\x92PPPV[`\0` \x80\x83R\x83Q`@` \x85\x01R\x80Q\x80``\x86\x01R`\0[\x81\x81\x10\x15a\x03\xD8W\x82\x81\x01\x84\x01Q\x86\x82\x01`\x80\x01R\x83\x01a\x03\xBCV[P`\0`\x80\x82\x87\x01\x01R` \x86\x01Q`@\x86\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x86\x01\x01\x93PPPP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x048WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80\x83Ta\x04L\x81a\x04\x04V[`\x01\x82\x81\x16\x80\x15a\x04dW`\x01\x81\x14a\x04yWa\x04\xA8V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x04\xA8V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x04\x9FW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x04\x86V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x05&W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x05\x03WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x05\"W\x82\x81U`\x01\x01a\x05\x0FV[PPP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x05CWa\x05Ca\x04\xC4V[a\x05W\x83a\x05Q\x83Ta\x04\x04V[\x83a\x04\xDAV[`\0`\x1F\x84\x11`\x01\x81\x14a\x05\x8BW`\0\x85\x15a\x05sWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x05\xE5V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x05\xBCW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x05\x9CV[P\x86\x82\x10\x15a\x05\xD9W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x81\x81\x03a\x05\xF7WPPV[a\x06\x01\x82Ta\x04\x04V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x19Wa\x06\x19a\x04\xC4V[a\x06-\x81a\x06'\x84Ta\x04\x04V[\x84a\x04\xDAV[`\0`\x1F\x82\x11`\x01\x81\x14a\x06aW`\0\x83\x15a\x06IWP\x84\x82\x01T[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x05\xE5V[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90`\0\x86\x81R` \x90 \x84[\x83\x81\x10\x15a\x06\x9BW\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a\x06{V[P\x85\x83\x10\x15a\x06\xB9W\x81\x85\x01T`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R\x82`@\x82\x01R\x82\x84``\x83\x017`\0``\x84\x83\x01\x01R`\0```\x1F\x19`\x1F\x86\x01\x16\x83\x01\x01\x90P\x82` \x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 ?\xB3\xC3T\x83\xAC\xA6\x92\x875#\xD7_*grXx\xD4\r\x8F\xE9\xF5-Zd\xB9E\x86/d\ndsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static DIDCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DIDContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DIDContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DIDContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DIDContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DIDContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DIDContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DIDContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DIDCONTRACT_ABI.clone(),
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
                DIDCONTRACT_ABI.clone(),
                DIDCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `get` (0xc2bc2efc) function
        pub fn get(
            &self,
            entity: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Did> {
            self.0
                .method_hash([194, 188, 46, 252], entity)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `remove` (0xa7f43779) function
        pub fn remove(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 244, 55, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0x4ee5c2ee) function
        pub fn update(
            &self,
            location: ::std::string::String,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 229, 194, 238], (location, price))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Removed` event
        pub fn removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemovedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Updated` event
        pub fn updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpdatedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DIDContractEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DIDContract<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "Removed", abi = "Removed(address)")]
    pub struct RemovedFilter(#[ethevent(indexed)] pub ::ethers::core::types::Address);
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
    #[ethevent(name = "Updated", abi = "Updated(address,string,uint256)")]
    pub struct UpdatedFilter {
        #[ethevent(indexed)]
        pub p0: ::ethers::core::types::Address,
        pub location: ::std::string::String,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DIDContractEvents {
        RemovedFilter(RemovedFilter),
        UpdatedFilter(UpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DIDContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RemovedFilter::decode_log(log) {
                return Ok(DIDContractEvents::RemovedFilter(decoded));
            }
            if let Ok(decoded) = UpdatedFilter::decode_log(log) {
                return Ok(DIDContractEvents::UpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DIDContractEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RemovedFilter> for DIDContractEvents {
        fn from(value: RemovedFilter) -> Self {
            Self::RemovedFilter(value)
        }
    }
    impl ::core::convert::From<UpdatedFilter> for DIDContractEvents {
        fn from(value: UpdatedFilter) -> Self {
            Self::UpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `get` function with signature `get(address)` and selector `0xc2bc2efc`
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
    #[ethcall(name = "get", abi = "get(address)")]
    pub struct GetCall {
        pub entity: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `remove` function with signature `remove()` and selector `0xa7f43779`
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
    #[ethcall(name = "remove", abi = "remove()")]
    pub struct RemoveCall;
    ///Container type for all input parameters for the `update` function with signature `update(string,uint256)` and selector `0x4ee5c2ee`
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
    #[ethcall(name = "update", abi = "update(string,uint256)")]
    pub struct UpdateCall {
        pub location: ::std::string::String,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DIDContractCalls {
        Get(GetCall),
        Remove(RemoveCall),
        Update(UpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for DIDContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <RemoveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Remove(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Update(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DIDContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Remove(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DIDContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::Remove(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCall> for DIDContractCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<RemoveCall> for DIDContractCalls {
        fn from(value: RemoveCall) -> Self {
            Self::Remove(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for DIDContractCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    ///Container type for all return fields from the `get` function with signature `get(address)` and selector `0xc2bc2efc`
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
    pub struct GetReturn(pub Did);
    ///`Did(string,uint256)`
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
    pub struct Did {
        pub location: ::std::string::String,
        pub price: ::ethers::core::types::U256,
    }
}
