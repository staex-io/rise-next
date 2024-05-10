#![allow(warnings)]
pub use ground_cycle_no_crypto_contract::*;
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
pub mod ground_cycle_no_crypto_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_landingWaitTime"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Info"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("landingByDrone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("landingByDrone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("station"),
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
                    ::std::borrow::ToOwned::to_owned("landingByStation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("landingByStation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("drone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("landlord"),
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
                    ::std::borrow::ToOwned::to_owned("landings"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("landings"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("drone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("station"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("landlord"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("reject"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reject"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("station"),
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
                    ::std::borrow::ToOwned::to_owned("takeoff"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("takeoff"),
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
                    ::std::borrow::ToOwned::to_owned("Landing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Landing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Reject"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Reject"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Takeoff"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Takeoff"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
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
                    ::std::borrow::ToOwned::to_owned("ErrHandshake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrHandshake"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrNoApprovedLanding"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ErrNoApprovedLanding",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrNoLanding"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrNoLanding"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrRejectApprovedLanding"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ErrRejectApprovedLanding",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrRejectTooEarly"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrRejectTooEarly"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrTakeoffRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrTakeoffRequired"),
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
    pub static GROUNDCYCLENOCRYPTOCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\n\x138\x03\x80a\n\x13\x839\x81\x01`@\x81\x90Ra\0/\x91a\0<V[`\x01`\0\x81\x90UUa\0UV[`\0` \x82\x84\x03\x12\x15a\0NW`\0\x80\xFD[PQ\x91\x90PV[a\t\xAF\x80a\0d`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x1C\xA8\x7F\xB4\x14a\0gW\x80cB\xFFS\xF5\x14a\0|W\x80c_\x01\xFFz\x14a\x01\x04W\x80c\xAB\r\xA5\xA9\x14a\x01\x0CW\x80c\xC2\xBC.\xFC\x14a\x01\x1FW\x80c\xC8x\xA3\xAC\x14a\x02\x08W[`\0\x80\xFD[a\0za\0u6`\x04a\x08\xDCV[a\x02\x1BV[\0[a\0\xC9a\0\x8A6`\x04a\t\x0FV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x93`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x91\x81\x16\x92\x91\x16\x90\x85V[`@\x80Q\x95\x86R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x90\x91\x16``\x83\x01R`\x80\x82\x01R`\xA0\x01[`@Q\x80\x91\x03\x90\xF3[a\0za\x03\x9AV[a\0za\x01\x1A6`\x04a\t\x0FV[a\x04\xC0V[a\x01\xB8a\x01-6`\x04a\t\x0FV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x86\x16\x92\x81\x01\x92\x90\x92R\x91\x82\x01T\x84\x16\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T\x90\x92\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x90V[`@Qa\0\xFB\x91\x90\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x83\x01\x91\x90\x91R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`\x80\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\0za\x02\x166`\x04a\t\x0FV[a\x07 V[3`\0\x90\x81R`\x02` R`@\x90 \x80T\x15a\x02JW`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xF5WP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x80\x84\x01\x91\x82R3\x84\x86\x01\x81\x81R\x96\x88\x16``\x86\x01\x90\x81RB`\x80\x87\x01\x90\x81R\x91\x85R`\x02\x92\x83\x90R\x95\x90\x93 \x93Q\x84U\x90Q`\x01\x84\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x94Q\x90\x83\x01\x80T\x91\x87\x16\x91\x86\x16\x91\x90\x91\x17\x90U\x91Q`\x03\x82\x01\x80T\x91\x90\x95\x16\x93\x16\x92\x90\x92\x17\x90\x92U\x90Q`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x037W3`\0\x90\x81R`\x02` R`@\x90 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ua\x03\\V[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15\x90a\x03RWP\x80T\x15[\x15a\x03\\WPPPV[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x03\x8CW`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x95\x81a\x08LV[PPPV[3`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x80\x82R`\x01\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x83\x01\x95\x90\x95R\x94\x82\x01T\x84\x16\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T\x90\x92\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x91\x03a\x04\x14W`@Qc\x17\xEE\xE8\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7Ft\x80\x15\xBF\xCD\xB2|bFa\x17\xBD\xB4\xA7\0\xF5C~j\x08\x82\x160\xA14\xE9D,\x10\xFC\xF0\xB0\x84`\0\x01Q`@Qa\x04s\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4P3`\0\x90\x81R`\x02` \x81\x90R`@\x82 \x82\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x91\x81\x01\x80T\x83\x16\x90U`\x03\x81\x01\x80T\x90\x92\x16\x90\x91U`\x04\x01UV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x80\x82R`\x01\x83\x01T\x87\x16\x93\x82\x01\x93\x90\x93R\x92\x81\x01T\x85\x16\x93\x83\x01\x93\x90\x93R`\x03\x83\x01T\x90\x93\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x90\x15a\x05<W`@Qc\x02\xCD\xA4\x85`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x05gW`@Qc=\x18\x1D\xE9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\x80\x82\x01Qa\x05y\x90Ba\tGV[\x10\x15a\x05\x98W`@Qc\x1D\xBEo\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x06(W\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fcaller should be drone of this l`D\x82\x01Reanding`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\x9EV[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Fcaller should be station of this`D\x82\x01Rg landing`\xC0\x1B`d\x82\x01R`\x84\x01a\x06\x1AV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x93\x81\x01\x80T\x85\x16\x90U`\x03\x81\x01\x80T\x90\x94\x16\x90\x93U`\x04\x90\x92\x01\x83\x90U\x85\x01Q\x90Q\x92\x93\x16\x91\x7Fv\xA0xJ\xFAF6af\x8D\xEAeS\x85pV\xC0O}\x92-\x13\x1A\xF7o)\xA5\xCA\xD6\xEBU\xBC\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 \x80T\x15a\x07XW`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x07\xFCWP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R3` \x80\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x84\x86\x01\x81\x81R``\x86\x01\x85\x81RB`\x80\x88\x01\x90\x81R\x92\x86R`\x02\x93\x84\x90R\x96\x90\x94 \x94Q\x85U\x91Q`\x01\x85\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x92Q\x90\x84\x01\x80T\x91\x87\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x92Q`\x03\x83\x01\x80T\x91\x90\x95\x16\x91\x16\x17\x90\x92UQ`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x08\x12WPPV[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08?W`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08H\x81a\x08LV[PPV[`\0T`\x03\x82\x01T`\x02\x83\x01T`\x01\x84\x01T`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x92\x83\x16\x92\x90\x91\x16\x90\x7F>\x01\xB6`\xE7T\xB0\xF3\xA7\x92w\xED\0\xCF\x07\x8C\xDF\xB1\x82\xBA\xB9\x06lN\x8F\xD8P\xB0\ni\x8DA\x90` \x01`@Q\x80\x91\x03\x90\xA4\x80\x82U`\0\x80T\x90\x80a\x08\xB7\x83a\t`V[\x91\x90PUPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xD7W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xEFW`\0\x80\xFD[a\x08\xF8\x83a\x08\xC0V[\x91Pa\t\x06` \x84\x01a\x08\xC0V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\t!W`\0\x80\xFD[a\t*\x82a\x08\xC0V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\tZWa\tZa\t1V[\x92\x91PPV[`\0`\x01\x82\x01a\trWa\tra\t1V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 _\x9B\xCC\xF19\t\x03\x1D\xC8\xBA\x96\x1D\xDA\xA8\n5\x9E\xED\x84 ^eze\xFC\xE6\x80\x19rnv\xEAdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GROUNDCYCLENOCRYPTOCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x1C\xA8\x7F\xB4\x14a\0gW\x80cB\xFFS\xF5\x14a\0|W\x80c_\x01\xFFz\x14a\x01\x04W\x80c\xAB\r\xA5\xA9\x14a\x01\x0CW\x80c\xC2\xBC.\xFC\x14a\x01\x1FW\x80c\xC8x\xA3\xAC\x14a\x02\x08W[`\0\x80\xFD[a\0za\0u6`\x04a\x08\xDCV[a\x02\x1BV[\0[a\0\xC9a\0\x8A6`\x04a\t\x0FV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x93`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x91\x81\x16\x92\x91\x16\x90\x85V[`@\x80Q\x95\x86R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x90\x91\x16``\x83\x01R`\x80\x82\x01R`\xA0\x01[`@Q\x80\x91\x03\x90\xF3[a\0za\x03\x9AV[a\0za\x01\x1A6`\x04a\t\x0FV[a\x04\xC0V[a\x01\xB8a\x01-6`\x04a\t\x0FV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x86\x16\x92\x81\x01\x92\x90\x92R\x91\x82\x01T\x84\x16\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T\x90\x92\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x90V[`@Qa\0\xFB\x91\x90\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x83\x01\x91\x90\x91R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`\x80\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\0za\x02\x166`\x04a\t\x0FV[a\x07 V[3`\0\x90\x81R`\x02` R`@\x90 \x80T\x15a\x02JW`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xF5WP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x80\x84\x01\x91\x82R3\x84\x86\x01\x81\x81R\x96\x88\x16``\x86\x01\x90\x81RB`\x80\x87\x01\x90\x81R\x91\x85R`\x02\x92\x83\x90R\x95\x90\x93 \x93Q\x84U\x90Q`\x01\x84\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x94Q\x90\x83\x01\x80T\x91\x87\x16\x91\x86\x16\x91\x90\x91\x17\x90U\x91Q`\x03\x82\x01\x80T\x91\x90\x95\x16\x93\x16\x92\x90\x92\x17\x90\x92U\x90Q`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x037W3`\0\x90\x81R`\x02` R`@\x90 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ua\x03\\V[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15\x90a\x03RWP\x80T\x15[\x15a\x03\\WPPPV[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x03\x8CW`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x95\x81a\x08LV[PPPV[3`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x80\x82R`\x01\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x83\x01\x95\x90\x95R\x94\x82\x01T\x84\x16\x92\x81\x01\x92\x90\x92R`\x03\x81\x01T\x90\x92\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x91\x03a\x04\x14W`@Qc\x17\xEE\xE8\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7Ft\x80\x15\xBF\xCD\xB2|bFa\x17\xBD\xB4\xA7\0\xF5C~j\x08\x82\x160\xA14\xE9D,\x10\xFC\xF0\xB0\x84`\0\x01Q`@Qa\x04s\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4P3`\0\x90\x81R`\x02` \x81\x90R`@\x82 \x82\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x91\x81\x01\x80T\x83\x16\x90U`\x03\x81\x01\x80T\x90\x92\x16\x90\x91U`\x04\x01UV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x80\x82R`\x01\x83\x01T\x87\x16\x93\x82\x01\x93\x90\x93R\x92\x81\x01T\x85\x16\x93\x83\x01\x93\x90\x93R`\x03\x83\x01T\x90\x93\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x90\x15a\x05<W`@Qc\x02\xCD\xA4\x85`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x05gW`@Qc=\x18\x1D\xE9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\x80\x82\x01Qa\x05y\x90Ba\tGV[\x10\x15a\x05\x98W`@Qc\x1D\xBEo\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x06(W\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fcaller should be drone of this l`D\x82\x01Reanding`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06\x9EV[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Fcaller should be station of this`D\x82\x01Rg landing`\xC0\x1B`d\x82\x01R`\x84\x01a\x06\x1AV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U\x93\x81\x01\x80T\x85\x16\x90U`\x03\x81\x01\x80T\x90\x94\x16\x90\x93U`\x04\x90\x92\x01\x83\x90U\x85\x01Q\x90Q\x92\x93\x16\x91\x7Fv\xA0xJ\xFAF6af\x8D\xEAeS\x85pV\xC0O}\x92-\x13\x1A\xF7o)\xA5\xCA\xD6\xEBU\xBC\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 \x80T\x15a\x07XW`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x07\xFCWP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R3` \x80\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x84\x86\x01\x81\x81R``\x86\x01\x85\x81RB`\x80\x88\x01\x90\x81R\x92\x86R`\x02\x93\x84\x90R\x96\x90\x94 \x94Q\x85U\x91Q`\x01\x85\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x92Q\x90\x84\x01\x80T\x91\x87\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x92Q`\x03\x83\x01\x80T\x91\x90\x95\x16\x91\x16\x17\x90\x92UQ`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x08\x12WPPV[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08?W`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08H\x81a\x08LV[PPV[`\0T`\x03\x82\x01T`\x02\x83\x01T`\x01\x84\x01T`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x92\x83\x16\x92\x90\x91\x16\x90\x7F>\x01\xB6`\xE7T\xB0\xF3\xA7\x92w\xED\0\xCF\x07\x8C\xDF\xB1\x82\xBA\xB9\x06lN\x8F\xD8P\xB0\ni\x8DA\x90` \x01`@Q\x80\x91\x03\x90\xA4\x80\x82U`\0\x80T\x90\x80a\x08\xB7\x83a\t`V[\x91\x90PUPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xD7W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xEFW`\0\x80\xFD[a\x08\xF8\x83a\x08\xC0V[\x91Pa\t\x06` \x84\x01a\x08\xC0V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\t!W`\0\x80\xFD[a\t*\x82a\x08\xC0V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\tZWa\tZa\t1V[\x92\x91PPV[`\0`\x01\x82\x01a\trWa\tra\t1V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 _\x9B\xCC\xF19\t\x03\x1D\xC8\xBA\x96\x1D\xDA\xA8\n5\x9E\xED\x84 ^eze\xFC\xE6\x80\x19rnv\xEAdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GROUNDCYCLENOCRYPTOCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GroundCycleNoCryptoContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GroundCycleNoCryptoContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GroundCycleNoCryptoContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GroundCycleNoCryptoContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GroundCycleNoCryptoContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GroundCycleNoCryptoContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GroundCycleNoCryptoContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GROUNDCYCLENOCRYPTOCONTRACT_ABI.clone(),
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
                GROUNDCYCLENOCRYPTOCONTRACT_ABI.clone(),
                GROUNDCYCLENOCRYPTOCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `get` (0xc2bc2efc) function
        pub fn get(
            &self,
            station: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Info> {
            self.0
                .method_hash([194, 188, 46, 252], station)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `landingByDrone` (0xc878a3ac) function
        pub fn landing_by_drone(
            &self,
            station: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 120, 163, 172], station)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `landingByStation` (0x1ca87fb4) function
        pub fn landing_by_station(
            &self,
            drone: ::ethers::core::types::Address,
            landlord: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 168, 127, 180], (drone, landlord))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `landings` (0x42ff53f5) function
        pub fn landings(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([66, 255, 83, 245], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reject` (0xab0da5a9) function
        pub fn reject(
            &self,
            station: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 13, 165, 169], station)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `takeoff` (0x5f01ff7a) function
        pub fn takeoff(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 1, 255, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Landing` event
        pub fn landing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LandingFilter> {
            self.0.event()
        }
        ///Gets the contract's `Reject` event
        pub fn reject_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RejectFilter> {
            self.0.event()
        }
        ///Gets the contract's `Takeoff` event
        pub fn takeoff_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TakeoffFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GroundCycleNoCryptoContractEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GroundCycleNoCryptoContract<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ErrHandshake` with signature `ErrHandshake()` and selector `0x6bf20624`
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
    #[etherror(name = "ErrHandshake", abi = "ErrHandshake()")]
    pub struct ErrHandshake;
    ///Custom Error type `ErrNoApprovedLanding` with signature `ErrNoApprovedLanding()` and selector `0x17eee897`
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
    #[etherror(name = "ErrNoApprovedLanding", abi = "ErrNoApprovedLanding()")]
    pub struct ErrNoApprovedLanding;
    ///Custom Error type `ErrNoLanding` with signature `ErrNoLanding()` and selector `0x3d181de9`
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
    #[etherror(name = "ErrNoLanding", abi = "ErrNoLanding()")]
    pub struct ErrNoLanding;
    ///Custom Error type `ErrRejectApprovedLanding` with signature `ErrRejectApprovedLanding()` and selector `0x2cda4850`
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
    #[etherror(name = "ErrRejectApprovedLanding", abi = "ErrRejectApprovedLanding()")]
    pub struct ErrRejectApprovedLanding;
    ///Custom Error type `ErrRejectTooEarly` with signature `ErrRejectTooEarly()` and selector `0x3b7cdf9e`
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
    #[etherror(name = "ErrRejectTooEarly", abi = "ErrRejectTooEarly()")]
    pub struct ErrRejectTooEarly;
    ///Custom Error type `ErrTakeoffRequired` with signature `ErrTakeoffRequired()` and selector `0x171ca981`
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
    #[etherror(name = "ErrTakeoffRequired", abi = "ErrTakeoffRequired()")]
    pub struct ErrTakeoffRequired;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GroundCycleNoCryptoContractErrors {
        ErrHandshake(ErrHandshake),
        ErrNoApprovedLanding(ErrNoApprovedLanding),
        ErrNoLanding(ErrNoLanding),
        ErrRejectApprovedLanding(ErrRejectApprovedLanding),
        ErrRejectTooEarly(ErrRejectTooEarly),
        ErrTakeoffRequired(ErrTakeoffRequired),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GroundCycleNoCryptoContractErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ErrHandshake as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrHandshake(decoded));
            }
            if let Ok(decoded) = <ErrNoApprovedLanding as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrNoApprovedLanding(decoded));
            }
            if let Ok(decoded) = <ErrNoLanding as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrNoLanding(decoded));
            }
            if let Ok(decoded) = <ErrRejectApprovedLanding as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrRejectApprovedLanding(decoded));
            }
            if let Ok(decoded) = <ErrRejectTooEarly as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrRejectTooEarly(decoded));
            }
            if let Ok(decoded) = <ErrTakeoffRequired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrTakeoffRequired(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GroundCycleNoCryptoContractErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ErrHandshake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrNoApprovedLanding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrNoLanding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrRejectApprovedLanding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrRejectTooEarly(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrTakeoffRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for GroundCycleNoCryptoContractErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ErrHandshake as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ErrNoApprovedLanding as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrNoLanding as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ErrRejectApprovedLanding as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrRejectTooEarly as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrTakeoffRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GroundCycleNoCryptoContractErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ErrHandshake(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrNoApprovedLanding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ErrNoLanding(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrRejectApprovedLanding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ErrRejectTooEarly(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrTakeoffRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for GroundCycleNoCryptoContractErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ErrHandshake> for GroundCycleNoCryptoContractErrors {
        fn from(value: ErrHandshake) -> Self {
            Self::ErrHandshake(value)
        }
    }
    impl ::core::convert::From<ErrNoApprovedLanding>
    for GroundCycleNoCryptoContractErrors {
        fn from(value: ErrNoApprovedLanding) -> Self {
            Self::ErrNoApprovedLanding(value)
        }
    }
    impl ::core::convert::From<ErrNoLanding> for GroundCycleNoCryptoContractErrors {
        fn from(value: ErrNoLanding) -> Self {
            Self::ErrNoLanding(value)
        }
    }
    impl ::core::convert::From<ErrRejectApprovedLanding>
    for GroundCycleNoCryptoContractErrors {
        fn from(value: ErrRejectApprovedLanding) -> Self {
            Self::ErrRejectApprovedLanding(value)
        }
    }
    impl ::core::convert::From<ErrRejectTooEarly> for GroundCycleNoCryptoContractErrors {
        fn from(value: ErrRejectTooEarly) -> Self {
            Self::ErrRejectTooEarly(value)
        }
    }
    impl ::core::convert::From<ErrTakeoffRequired>
    for GroundCycleNoCryptoContractErrors {
        fn from(value: ErrTakeoffRequired) -> Self {
            Self::ErrTakeoffRequired(value)
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
    #[ethevent(name = "Landing", abi = "Landing(uint256,address,address,address)")]
    pub struct LandingFilter(
        pub ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
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
    #[ethevent(name = "Reject", abi = "Reject(address,address)")]
    pub struct RejectFilter(
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
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
    #[ethevent(name = "Takeoff", abi = "Takeoff(uint256,address,address,address)")]
    pub struct TakeoffFilter(
        pub ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub ::ethers::core::types::Address,
    );
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GroundCycleNoCryptoContractEvents {
        LandingFilter(LandingFilter),
        RejectFilter(RejectFilter),
        TakeoffFilter(TakeoffFilter),
    }
    impl ::ethers::contract::EthLogDecode for GroundCycleNoCryptoContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LandingFilter::decode_log(log) {
                return Ok(GroundCycleNoCryptoContractEvents::LandingFilter(decoded));
            }
            if let Ok(decoded) = RejectFilter::decode_log(log) {
                return Ok(GroundCycleNoCryptoContractEvents::RejectFilter(decoded));
            }
            if let Ok(decoded) = TakeoffFilter::decode_log(log) {
                return Ok(GroundCycleNoCryptoContractEvents::TakeoffFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GroundCycleNoCryptoContractEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LandingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RejectFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TakeoffFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LandingFilter> for GroundCycleNoCryptoContractEvents {
        fn from(value: LandingFilter) -> Self {
            Self::LandingFilter(value)
        }
    }
    impl ::core::convert::From<RejectFilter> for GroundCycleNoCryptoContractEvents {
        fn from(value: RejectFilter) -> Self {
            Self::RejectFilter(value)
        }
    }
    impl ::core::convert::From<TakeoffFilter> for GroundCycleNoCryptoContractEvents {
        fn from(value: TakeoffFilter) -> Self {
            Self::TakeoffFilter(value)
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
        pub station: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `landingByDrone` function with signature `landingByDrone(address)` and selector `0xc878a3ac`
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
    #[ethcall(name = "landingByDrone", abi = "landingByDrone(address)")]
    pub struct LandingByDroneCall {
        pub station: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `landingByStation` function with signature `landingByStation(address,address)` and selector `0x1ca87fb4`
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
    #[ethcall(name = "landingByStation", abi = "landingByStation(address,address)")]
    pub struct LandingByStationCall {
        pub drone: ::ethers::core::types::Address,
        pub landlord: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `landings` function with signature `landings(address)` and selector `0x42ff53f5`
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
    #[ethcall(name = "landings", abi = "landings(address)")]
    pub struct LandingsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `reject` function with signature `reject(address)` and selector `0xab0da5a9`
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
    #[ethcall(name = "reject", abi = "reject(address)")]
    pub struct RejectCall {
        pub station: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `takeoff` function with signature `takeoff()` and selector `0x5f01ff7a`
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
    #[ethcall(name = "takeoff", abi = "takeoff()")]
    pub struct TakeoffCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GroundCycleNoCryptoContractCalls {
        Get(GetCall),
        LandingByDrone(LandingByDroneCall),
        LandingByStation(LandingByStationCall),
        Landings(LandingsCall),
        Reject(RejectCall),
        Takeoff(TakeoffCall),
    }
    impl ::ethers::core::abi::AbiDecode for GroundCycleNoCryptoContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <LandingByDroneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LandingByDrone(decoded));
            }
            if let Ok(decoded) = <LandingByStationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LandingByStation(decoded));
            }
            if let Ok(decoded) = <LandingsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Landings(decoded));
            }
            if let Ok(decoded) = <RejectCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reject(decoded));
            }
            if let Ok(decoded) = <TakeoffCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Takeoff(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GroundCycleNoCryptoContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LandingByDrone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LandingByStation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Landings(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reject(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Takeoff(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GroundCycleNoCryptoContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::LandingByDrone(element) => ::core::fmt::Display::fmt(element, f),
                Self::LandingByStation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Landings(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reject(element) => ::core::fmt::Display::fmt(element, f),
                Self::Takeoff(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCall> for GroundCycleNoCryptoContractCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<LandingByDroneCall> for GroundCycleNoCryptoContractCalls {
        fn from(value: LandingByDroneCall) -> Self {
            Self::LandingByDrone(value)
        }
    }
    impl ::core::convert::From<LandingByStationCall>
    for GroundCycleNoCryptoContractCalls {
        fn from(value: LandingByStationCall) -> Self {
            Self::LandingByStation(value)
        }
    }
    impl ::core::convert::From<LandingsCall> for GroundCycleNoCryptoContractCalls {
        fn from(value: LandingsCall) -> Self {
            Self::Landings(value)
        }
    }
    impl ::core::convert::From<RejectCall> for GroundCycleNoCryptoContractCalls {
        fn from(value: RejectCall) -> Self {
            Self::Reject(value)
        }
    }
    impl ::core::convert::From<TakeoffCall> for GroundCycleNoCryptoContractCalls {
        fn from(value: TakeoffCall) -> Self {
            Self::Takeoff(value)
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
    pub struct GetReturn(pub Info);
    ///Container type for all return fields from the `landings` function with signature `landings(address)` and selector `0x42ff53f5`
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
    pub struct LandingsReturn {
        pub id: ::ethers::core::types::U256,
        pub drone: ::ethers::core::types::Address,
        pub station: ::ethers::core::types::Address,
        pub landlord: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///`Info(uint256,address,address,address,uint256)`
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
    pub struct Info {
        pub id: ::ethers::core::types::U256,
        pub drone: ::ethers::core::types::Address,
        pub station: ::ethers::core::types::Address,
        pub landlord: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
    }
}
