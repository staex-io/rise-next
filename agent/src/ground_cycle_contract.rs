pub use ground_cycle_contract::*;
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
pub mod ground_cycle_contract {
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_agreementContract"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract AgreementContract",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBalance"),
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
                    ::std::borrow::ToOwned::to_owned("landingByDrone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("landingByDrone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("station"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
                    ::std::borrow::ToOwned::to_owned("landingByStation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("landingByStation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("drone"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("landlord"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("station"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("landlord"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
                    ::std::borrow::ToOwned::to_owned("ErrAgreementNotSigned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ErrAgreementNotSigned",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("ErrReceivedNotEnough"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ErrReceivedNotEnough",
                            ),
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
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GROUNDCYCLECONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r\xFF8\x03\x80a\r\xFF\x839\x81\x01`@\x81\x90Ra\0/\x91a\0^V[`\x01`\0\x81\x90U\x91\x90\x91U`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x9BV[`\0\x80`@\x83\x85\x03\x12\x15a\0qW`\0\x80\xFD[\x82Q` \x84\x01Q\x90\x92P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x90W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[a\rU\x80a\0\xAA`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0nW`\x005`\xE0\x1C\x80c_\x01\xFFz\x11a\0KW\x80c_\x01\xFFz\x14a\x01:W\x80c\xAB\r\xA5\xA9\x14a\x01OW\x80c\xC2\xBC.\xFC\x14a\x01oW\x80c\xC8x\xA3\xAC\x14a\x02cW\0[\x80c\x12\x06_\xE0\x14a\0wW\x80c\x1C\xA8\x7F\xB4\x14a\0\x97W\x80cB\xFFS\xF5\x14a\0\xAAW\0[6a\0uW\0[\0[4\x80\x15a\0\x83W`\0\x80\xFD[P`@QG\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0ua\0\xA56`\x04a\x0B\xFCV[a\x02vV[4\x80\x15a\0\xB6W`\0\x80\xFD[Pa\x01\x04a\0\xC56`\x04a\x0C5V[`\x03` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x93\x83\x01T`\x04\x90\x93\x01T\x91\x93`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x90\x82\x16\x92\x91\x16\x90\x85V[`@\x80Q\x95\x86R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x90\x91\x16``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\x8EV[4\x80\x15a\x01FW`\0\x80\xFD[Pa\0ua\x04.V[4\x80\x15a\x01[W`\0\x80\xFD[Pa\0ua\x01j6`\x04a\x0C5V[a\x05UV[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x02\x13a\x01\x8A6`\x04a\x0C5V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x86\x16\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x85\x16\x93\x82\x01\x93\x90\x93R\x90\x82\x01T\x90\x92\x16``\x83\x01R`\x04\x01T`\x80\x82\x01R\x90V[`@Qa\0\x8E\x91\x90\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x83\x01\x91\x90\x91R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`\x80\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\0ua\x02q6`\x04a\x0C5V[a\x07\xD7V[a\x02\x803\x82a\t:V[3`\0\x90\x81R`\x03` R`@\x90 \x80T\x15a\x02\xAFW`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03[WP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x80\x84\x01\x91\x82R3\x84\x86\x01\x81\x81R\x96\x88\x16``\x86\x01\x90\x81RB`\x80\x87\x01\x90\x81R\x91\x85R`\x03\x92\x83\x90R\x95\x90\x93 \x93Q\x84U\x90Q`\x01\x84\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x94Q`\x02\x84\x01\x80T\x91\x88\x16\x91\x87\x16\x91\x90\x91\x17\x90U\x92Q\x92\x82\x01\x80T\x93\x90\x95\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x92UQ`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x9EW3`\0\x90\x81R`\x03` \x81\x90R`@\x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ua\x03\xF0V[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15\x90a\x03\xB9WP\x80T\x15[\x15a\x03\xF0W`@Q3\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x03\xEAW=`\0\x80>=`\0\xFD[PPPPV[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x04 W`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04)\x81a\n\x15V[PPPV[3`\0\x90\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x80\x82R`\x01\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x83\x01\x95\x90\x95R`\x02\x83\x01T\x85\x16\x93\x82\x01\x93\x90\x93R\x93\x81\x01T\x90\x92\x16``\x84\x01R`\x04\x90\x91\x01T`\x80\x83\x01R\x90\x91\x03a\x04\xA9W`@Qc\x17\xEE\xE8\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7Ft\x80\x15\xBF\xCD\xB2|bFa\x17\xBD\xB4\xA7\0\xF5C~j\x08\x82\x160\xA14\xE9D,\x10\xFC\xF0\xB0\x84`\0\x01Q`@Qa\x05\x08\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4P3`\0\x90\x81R`\x03` \x81\x90R`@\x82 \x82\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x80T\x82\x16\x90U\x91\x81\x01\x80T\x90\x92\x16\x90\x91U`\x04\x01UV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x80\x82R`\x01\x83\x01T\x87\x16\x93\x82\x01\x93\x90\x93R`\x02\x82\x01T\x86\x16\x94\x81\x01\x94\x90\x94R\x91\x82\x01T\x90\x93\x16``\x83\x01R`\x04\x01T`\x80\x82\x01R\x90\x15a\x05\xCFW`@Qc\x02\xCD\xA4\x85`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x05\xFAW`@Qc=\x18\x1D\xE9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\x80\x82\x01Qa\x06\x0C\x90Ba\x0CoV[\x10\x15a\x06+W`@Qc\x1D\xBEo\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x06\xCCW\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fcaller should be drone of this l`D\x82\x01Reanding`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x06\xC7\x90\x83\x90\x80a\n\xCBV[a\x07UV[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Fcaller should be station of this`D\x82\x01Rg landing`\xC0\x1B`d\x82\x01R`\x84\x01a\x06\xADV[a\x07U\x82\x82``\x01Q\x83`@\x01Qa\n\xCBV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x80T\x82\x16\x90U\x93\x81\x01\x80T\x90\x94\x16\x90\x93U`\x04\x90\x92\x01\x83\x90U\x85\x01Q\x90Q\x92\x93\x16\x91\x7Fv\xA0xJ\xFAF6af\x8D\xEAeS\x85pV\xC0O}\x92-\x13\x1A\xF7o)\xA5\xCA\xD6\xEBU\xBC\x91\x90\xA3PPV[a\x07\xE1\x813a\t:V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x15a\x08\x19W`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x08\xC2WP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R3` \x80\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x84\x86\x01\x81\x81R``\x86\x01\x85\x81RB`\x80\x88\x01\x90\x81R\x92\x86R`\x03\x93\x84\x90R\x96\x90\x94 \x94Q\x85U\x91Q`\x01\x85\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x92Q`\x02\x85\x01\x80T\x91\x88\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x93\x83\x01\x80T\x94\x90\x95\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x92UQ`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\t\0W`@Q3\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04)W=`\0\x80>=`\0\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t-W`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t6\x81a\n\x15V[PPV[`\x02T`@Qc\xD8\x1E\x84#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`\0\x92\x16\x90c\xD8\x1E\x84#\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB0\x91\x90a\x0C\x88V[\x90P`\x02\x81` \x01Q`\x02\x81\x11\x15a\t\xCAWa\t\xCAa\x0C\xF0V[\x14a\t\xE7W`@QbTp\xED`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q4\x10\x15a\x04)W\x80Q`@Qc\x92k\xDE\xA9`\xE0\x1B\x81R4`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x06\xADV[`\x02\x81\x01T`\x01\x82\x01Ta\n6\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x81a\n\xCBV[`\x02\x81\x01T`\x03\x82\x01Ta\nW\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x80a\n\xCBV[`\0T`\x03\x82\x01T`\x02\x83\x01T`\x01\x84\x01T`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x92\x83\x16\x92\x90\x91\x16\x90\x7F>\x01\xB6`\xE7T\xB0\xF3\xA7\x92w\xED\0\xCF\x07\x8C\xDF\xB1\x82\xBA\xB9\x06lN\x8F\xD8P\xB0\ni\x8DA\x90` \x01`@Q\x80\x91\x03\x90\xA4\x80\x82U`\0\x80T\x90\x80a\n\xC2\x83a\r\x06V[\x91\x90PUPPPV[`\x02T`@Qc\xD8\x1E\x84#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`\0\x92\x16\x90c\xD8\x1E\x84#\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BA\x91\x90a\x0C\x88V[\x80Q`@Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\x90W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\x95V[``\x91P[PP\x90P\x80a\x0B\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs30\xB4\xB62\xB2\x10:7\x909\xB2\xB72\x102\xBA42\xB9`a\x1B`D\x82\x01R`d\x01a\x06\xADV[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xF9W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x0FW`\0\x80\xFD[\x825a\x0C\x1A\x81a\x0B\xE4V[\x91P` \x83\x015a\x0C*\x81a\x0B\xE4V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0CGW`\0\x80\xFD[\x815a\x0CR\x81a\x0B\xE4V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\x82Wa\x0C\x82a\x0CYV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x0C\x9AW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\xCBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q\x81R` \x83\x01Q`\x03\x81\x10a\x0C\xE4W`\0\x80\xFD[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\r\x18Wa\r\x18a\x0CYV[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 !\x87\xC5`\xA3\xABR\x86N\x9C\xBC\x85\x89\xD1\xF4G\xF4\x06{\xA6\xF0\xE7\xBF\x97C\xF9.d\x95*XJdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GROUNDCYCLECONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0nW`\x005`\xE0\x1C\x80c_\x01\xFFz\x11a\0KW\x80c_\x01\xFFz\x14a\x01:W\x80c\xAB\r\xA5\xA9\x14a\x01OW\x80c\xC2\xBC.\xFC\x14a\x01oW\x80c\xC8x\xA3\xAC\x14a\x02cW\0[\x80c\x12\x06_\xE0\x14a\0wW\x80c\x1C\xA8\x7F\xB4\x14a\0\x97W\x80cB\xFFS\xF5\x14a\0\xAAW\0[6a\0uW\0[\0[4\x80\x15a\0\x83W`\0\x80\xFD[P`@QG\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0ua\0\xA56`\x04a\x0B\xFCV[a\x02vV[4\x80\x15a\0\xB6W`\0\x80\xFD[Pa\x01\x04a\0\xC56`\x04a\x0C5V[`\x03` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x93\x83\x01T`\x04\x90\x93\x01T\x91\x93`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x90\x82\x16\x92\x91\x16\x90\x85V[`@\x80Q\x95\x86R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x87\x01R\x92\x84\x16\x92\x85\x01\x92\x90\x92R\x90\x91\x16``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\x8EV[4\x80\x15a\x01FW`\0\x80\xFD[Pa\0ua\x04.V[4\x80\x15a\x01[W`\0\x80\xFD[Pa\0ua\x01j6`\x04a\x0C5V[a\x05UV[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x02\x13a\x01\x8A6`\x04a\x0C5V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x86\x16\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x85\x16\x93\x82\x01\x93\x90\x93R\x90\x82\x01T\x90\x92\x16``\x83\x01R`\x04\x01T`\x80\x82\x01R\x90V[`@Qa\0\x8E\x91\x90\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x83\x01\x91\x90\x91R`@\x80\x84\x01Q\x82\x16\x90\x83\x01R``\x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`\x80\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\xA0\x01\x90V[a\0ua\x02q6`\x04a\x0C5V[a\x07\xD7V[a\x02\x803\x82a\t:V[3`\0\x90\x81R`\x03` R`@\x90 \x80T\x15a\x02\xAFW`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03[WP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x80\x84\x01\x91\x82R3\x84\x86\x01\x81\x81R\x96\x88\x16``\x86\x01\x90\x81RB`\x80\x87\x01\x90\x81R\x91\x85R`\x03\x92\x83\x90R\x95\x90\x93 \x93Q\x84U\x90Q`\x01\x84\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x94Q`\x02\x84\x01\x80T\x91\x88\x16\x91\x87\x16\x91\x90\x91\x17\x90U\x92Q\x92\x82\x01\x80T\x93\x90\x95\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x92UQ`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x9EW3`\0\x90\x81R`\x03` \x81\x90R`@\x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90Ua\x03\xF0V[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15\x90a\x03\xB9WP\x80T\x15[\x15a\x03\xF0W`@Q3\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x03\xEAW=`\0\x80>=`\0\xFD[PPPPV[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x04 W`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04)\x81a\n\x15V[PPPV[3`\0\x90\x81R`\x03` \x81\x81R`@\x80\x84 \x81Q`\xA0\x81\x01\x83R\x81T\x80\x82R`\x01\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x83\x01\x95\x90\x95R`\x02\x83\x01T\x85\x16\x93\x82\x01\x93\x90\x93R\x93\x81\x01T\x90\x92\x16``\x84\x01R`\x04\x90\x91\x01T`\x80\x83\x01R\x90\x91\x03a\x04\xA9W`@Qc\x17\xEE\xE8\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7Ft\x80\x15\xBF\xCD\xB2|bFa\x17\xBD\xB4\xA7\0\xF5C~j\x08\x82\x160\xA14\xE9D,\x10\xFC\xF0\xB0\x84`\0\x01Q`@Qa\x05\x08\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4P3`\0\x90\x81R`\x03` \x81\x90R`@\x82 \x82\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x80T\x82\x16\x90U\x91\x81\x01\x80T\x90\x92\x16\x90\x91U`\x04\x01UV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\xA0\x81\x01\x85R\x81T\x80\x82R`\x01\x83\x01T\x87\x16\x93\x82\x01\x93\x90\x93R`\x02\x82\x01T\x86\x16\x94\x81\x01\x94\x90\x94R\x91\x82\x01T\x90\x93\x16``\x83\x01R`\x04\x01T`\x80\x82\x01R\x90\x15a\x05\xCFW`@Qc\x02\xCD\xA4\x85`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x05\xFAW`@Qc=\x18\x1D\xE9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\x80\x82\x01Qa\x06\x0C\x90Ba\x0CoV[\x10\x15a\x06+W`@Qc\x1D\xBEo\xCF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x06\xCCW\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fcaller should be drone of this l`D\x82\x01Reanding`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x06\xC7\x90\x83\x90\x80a\n\xCBV[a\x07UV[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7Fcaller should be station of this`D\x82\x01Rg landing`\xC0\x1B`d\x82\x01R`\x84\x01a\x06\xADV[a\x07U\x82\x82``\x01Q\x83`@\x01Qa\n\xCBV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x03` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x80T\x82\x16\x90U\x93\x81\x01\x80T\x90\x94\x16\x90\x93U`\x04\x90\x92\x01\x83\x90U\x85\x01Q\x90Q\x92\x93\x16\x91\x7Fv\xA0xJ\xFAF6af\x8D\xEAeS\x85pV\xC0O}\x92-\x13\x1A\xF7o)\xA5\xCA\xD6\xEBU\xBC\x91\x90\xA3PPV[a\x07\xE1\x813a\t:V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x15a\x08\x19W`@Qc\x17\x1C\xA9\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x08\xC2WP`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R3` \x80\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x84\x86\x01\x81\x81R``\x86\x01\x85\x81RB`\x80\x88\x01\x90\x81R\x92\x86R`\x03\x93\x84\x90R\x96\x90\x94 \x94Q\x85U\x91Q`\x01\x85\x01\x80T\x91\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U\x92Q`\x02\x85\x01\x80T\x91\x88\x16\x91\x85\x16\x91\x90\x91\x17\x90U\x93Q\x93\x83\x01\x80T\x94\x90\x95\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x92UQ`\x04\x90\x91\x01UV[`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16a\t\0W`@Q3\x904\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04)W=`\0\x80>=`\0\xFD[`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t-W`@Qc\x1A\xFC\x81\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t6\x81a\n\x15V[PPV[`\x02T`@Qc\xD8\x1E\x84#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`\0\x92\x16\x90c\xD8\x1E\x84#\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB0\x91\x90a\x0C\x88V[\x90P`\x02\x81` \x01Q`\x02\x81\x11\x15a\t\xCAWa\t\xCAa\x0C\xF0V[\x14a\t\xE7W`@QbTp\xED`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q4\x10\x15a\x04)W\x80Q`@Qc\x92k\xDE\xA9`\xE0\x1B\x81R4`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x06\xADV[`\x02\x81\x01T`\x01\x82\x01Ta\n6\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x81a\n\xCBV[`\x02\x81\x01T`\x03\x82\x01Ta\nW\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x80a\n\xCBV[`\0T`\x03\x82\x01T`\x02\x83\x01T`\x01\x84\x01T`@Q\x84\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x92\x83\x16\x92\x90\x91\x16\x90\x7F>\x01\xB6`\xE7T\xB0\xF3\xA7\x92w\xED\0\xCF\x07\x8C\xDF\xB1\x82\xBA\xB9\x06lN\x8F\xD8P\xB0\ni\x8DA\x90` \x01`@Q\x80\x91\x03\x90\xA4\x80\x82U`\0\x80T\x90\x80a\n\xC2\x83a\r\x06V[\x91\x90PUPPPV[`\x02T`@Qc\xD8\x1E\x84#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`\0\x92\x16\x90c\xD8\x1E\x84#\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BA\x91\x90a\x0C\x88V[\x80Q`@Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\x90W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\x95V[``\x91P[PP\x90P\x80a\x0B\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs30\xB4\xB62\xB2\x10:7\x909\xB2\xB72\x102\xBA42\xB9`a\x1B`D\x82\x01R`d\x01a\x06\xADV[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xF9W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x0FW`\0\x80\xFD[\x825a\x0C\x1A\x81a\x0B\xE4V[\x91P` \x83\x015a\x0C*\x81a\x0B\xE4V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0CGW`\0\x80\xFD[\x815a\x0CR\x81a\x0B\xE4V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\x82Wa\x0C\x82a\x0CYV[\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x0C\x9AW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\xCBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q\x81R` \x83\x01Q`\x03\x81\x10a\x0C\xE4W`\0\x80\xFD[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\r\x18Wa\r\x18a\x0CYV[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 !\x87\xC5`\xA3\xABR\x86N\x9C\xBC\x85\x89\xD1\xF4G\xF4\x06{\xA6\xF0\xE7\xBF\x97C\xF9.d\x95*XJdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GROUNDCYCLECONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GroundCycleContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GroundCycleContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GroundCycleContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GroundCycleContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GroundCycleContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GroundCycleContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GroundCycleContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GROUNDCYCLECONTRACT_ABI.clone(),
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
                GROUNDCYCLECONTRACT_ABI.clone(),
                GROUNDCYCLECONTRACT_BYTECODE.clone().into(),
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
        ///Calls the contract's `getBalance` (0x12065fe0) function
        pub fn get_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 6, 95, 224], ())
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
            GroundCycleContractEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GroundCycleContract<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ErrAgreementNotSigned` with signature `ErrAgreementNotSigned()` and selector `0x02a38768`
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
    #[etherror(name = "ErrAgreementNotSigned", abi = "ErrAgreementNotSigned()")]
    pub struct ErrAgreementNotSigned;
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
    ///Custom Error type `ErrReceivedNotEnough` with signature `ErrReceivedNotEnough(uint256,uint256)` and selector `0x926bdea9`
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
    #[etherror(
        name = "ErrReceivedNotEnough",
        abi = "ErrReceivedNotEnough(uint256,uint256)"
    )]
    pub struct ErrReceivedNotEnough(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    pub enum GroundCycleContractErrors {
        ErrAgreementNotSigned(ErrAgreementNotSigned),
        ErrHandshake(ErrHandshake),
        ErrNoApprovedLanding(ErrNoApprovedLanding),
        ErrNoLanding(ErrNoLanding),
        ErrReceivedNotEnough(ErrReceivedNotEnough),
        ErrRejectApprovedLanding(ErrRejectApprovedLanding),
        ErrRejectTooEarly(ErrRejectTooEarly),
        ErrTakeoffRequired(ErrTakeoffRequired),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GroundCycleContractErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ErrAgreementNotSigned as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrAgreementNotSigned(decoded));
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
            if let Ok(decoded) = <ErrReceivedNotEnough as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrReceivedNotEnough(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GroundCycleContractErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ErrAgreementNotSigned(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrHandshake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrNoApprovedLanding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrNoLanding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrReceivedNotEnough(element) => {
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
    impl ::ethers::contract::ContractRevert for GroundCycleContractErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ErrAgreementNotSigned as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrHandshake as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ErrNoApprovedLanding as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrNoLanding as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ErrReceivedNotEnough as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
    impl ::core::fmt::Display for GroundCycleContractErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ErrAgreementNotSigned(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ErrHandshake(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrNoApprovedLanding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ErrNoLanding(element) => ::core::fmt::Display::fmt(element, f),
                Self::ErrReceivedNotEnough(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<::std::string::String> for GroundCycleContractErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ErrAgreementNotSigned> for GroundCycleContractErrors {
        fn from(value: ErrAgreementNotSigned) -> Self {
            Self::ErrAgreementNotSigned(value)
        }
    }
    impl ::core::convert::From<ErrHandshake> for GroundCycleContractErrors {
        fn from(value: ErrHandshake) -> Self {
            Self::ErrHandshake(value)
        }
    }
    impl ::core::convert::From<ErrNoApprovedLanding> for GroundCycleContractErrors {
        fn from(value: ErrNoApprovedLanding) -> Self {
            Self::ErrNoApprovedLanding(value)
        }
    }
    impl ::core::convert::From<ErrNoLanding> for GroundCycleContractErrors {
        fn from(value: ErrNoLanding) -> Self {
            Self::ErrNoLanding(value)
        }
    }
    impl ::core::convert::From<ErrReceivedNotEnough> for GroundCycleContractErrors {
        fn from(value: ErrReceivedNotEnough) -> Self {
            Self::ErrReceivedNotEnough(value)
        }
    }
    impl ::core::convert::From<ErrRejectApprovedLanding> for GroundCycleContractErrors {
        fn from(value: ErrRejectApprovedLanding) -> Self {
            Self::ErrRejectApprovedLanding(value)
        }
    }
    impl ::core::convert::From<ErrRejectTooEarly> for GroundCycleContractErrors {
        fn from(value: ErrRejectTooEarly) -> Self {
            Self::ErrRejectTooEarly(value)
        }
    }
    impl ::core::convert::From<ErrTakeoffRequired> for GroundCycleContractErrors {
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
    pub enum GroundCycleContractEvents {
        LandingFilter(LandingFilter),
        RejectFilter(RejectFilter),
        TakeoffFilter(TakeoffFilter),
    }
    impl ::ethers::contract::EthLogDecode for GroundCycleContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LandingFilter::decode_log(log) {
                return Ok(GroundCycleContractEvents::LandingFilter(decoded));
            }
            if let Ok(decoded) = RejectFilter::decode_log(log) {
                return Ok(GroundCycleContractEvents::RejectFilter(decoded));
            }
            if let Ok(decoded) = TakeoffFilter::decode_log(log) {
                return Ok(GroundCycleContractEvents::TakeoffFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GroundCycleContractEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LandingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RejectFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TakeoffFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LandingFilter> for GroundCycleContractEvents {
        fn from(value: LandingFilter) -> Self {
            Self::LandingFilter(value)
        }
    }
    impl ::core::convert::From<RejectFilter> for GroundCycleContractEvents {
        fn from(value: RejectFilter) -> Self {
            Self::RejectFilter(value)
        }
    }
    impl ::core::convert::From<TakeoffFilter> for GroundCycleContractEvents {
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
    ///Container type for all input parameters for the `getBalance` function with signature `getBalance()` and selector `0x12065fe0`
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
    #[ethcall(name = "getBalance", abi = "getBalance()")]
    pub struct GetBalanceCall;
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
    pub enum GroundCycleContractCalls {
        Get(GetCall),
        GetBalance(GetBalanceCall),
        LandingByDrone(LandingByDroneCall),
        LandingByStation(LandingByStationCall),
        Landings(LandingsCall),
        Reject(RejectCall),
        Takeoff(TakeoffCall),
    }
    impl ::ethers::core::abi::AbiDecode for GroundCycleContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBalance(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GroundCycleContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::core::fmt::Display for GroundCycleContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::LandingByDrone(element) => ::core::fmt::Display::fmt(element, f),
                Self::LandingByStation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Landings(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reject(element) => ::core::fmt::Display::fmt(element, f),
                Self::Takeoff(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCall> for GroundCycleContractCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<GetBalanceCall> for GroundCycleContractCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    impl ::core::convert::From<LandingByDroneCall> for GroundCycleContractCalls {
        fn from(value: LandingByDroneCall) -> Self {
            Self::LandingByDrone(value)
        }
    }
    impl ::core::convert::From<LandingByStationCall> for GroundCycleContractCalls {
        fn from(value: LandingByStationCall) -> Self {
            Self::LandingByStation(value)
        }
    }
    impl ::core::convert::From<LandingsCall> for GroundCycleContractCalls {
        fn from(value: LandingsCall) -> Self {
            Self::Landings(value)
        }
    }
    impl ::core::convert::From<RejectCall> for GroundCycleContractCalls {
        fn from(value: RejectCall) -> Self {
            Self::Reject(value)
        }
    }
    impl ::core::convert::From<TakeoffCall> for GroundCycleContractCalls {
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
    ///Container type for all return fields from the `getBalance` function with signature `getBalance()` and selector `0x12065fe0`
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
    pub struct GetBalanceReturn(pub ::ethers::core::types::U256);
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
