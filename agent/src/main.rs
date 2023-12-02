use std::{str::FromStr, sync::Arc, time::Duration};

use assertables::{assert_in_delta, assert_in_delta_as_result};
use contracts::agreement_contract::AgreementContractEvents;
use ethers::{
    contract::ContractError,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, PendingTransaction, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
    utils,
};
use log::{debug, LevelFilter};

use crate::contracts::agreement_contract::{Agreement, AgreementContract, AgreementContractErrors};
use crate::contracts::ground_cycle_contract::{
    GroundCycleContract, GroundCycleContractErrors, GroundCycleContractEvents,
};

mod contracts;

const RPC_URL: &str = "http://127.0.0.1:8545";
const CHAIN_ID: u64 = 31337;

const AGREEMENT_CONTRACT_ADDR: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
const GROUND_CYCLE_CONTRACT_ADDR: &str = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";

const DRONE_PRIVATE_KEY: &str =
    "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";
const STATION_PRIVATE_KEY: &str =
    "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97";
const LANDLORD_PRIVATE_KEY: &str =
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();

    // Client to Ethereum node.
    let provider: Provider<Http> = Provider::<Http>::try_from(RPC_URL)?;

    let agreement_contract_addr: Address = AGREEMENT_CONTRACT_ADDR.parse()?;
    let ground_cycle_contract_addr: Address = GROUND_CYCLE_CONTRACT_ADDR.parse()?;

    let drone_wallet: LocalWallet =
        LocalWallet::from_str(DRONE_PRIVATE_KEY)?.with_chain_id(CHAIN_ID);
    let station_wallet: LocalWallet =
        LocalWallet::from_str(STATION_PRIVATE_KEY)?.with_chain_id(CHAIN_ID);
    let landlord_wallet: LocalWallet =
        LocalWallet::from_str(LANDLORD_PRIVATE_KEY)?.with_chain_id(CHAIN_ID);

    debug!("drone wallet address: {:?}", drone_wallet.address());
    debug!("station wallet address: {:?}", station_wallet.address());
    debug!("landlord wallet address: {:?}", landlord_wallet.address());

    // Amounts of agreements between entities.
    let drone_station_amount: U256 = utils::parse_ether(3)?;
    let station_landlord_amount: U256 = utils::parse_ether(5)?;

    let agreement_contract_balance = provider.get_balance(agreement_contract_addr, None).await?;
    assert!(agreement_contract_balance.is_zero());
    let drone_balance_before = provider.get_balance(drone_wallet.address(), None).await?;
    let station_balance_before = provider.get_balance(station_wallet.address(), None).await?;
    let landlord_balance_before = provider
        .get_balance(landlord_wallet.address(), None)
        .await?;

    let contracts_client = Client::new(
        provider.clone(),
        agreement_contract_addr,
        ground_cycle_contract_addr,
    );

    /*
        Create agreements.
    */

    // Create drone - station agreement.
    debug!("creating drone - station agreement");
    let create_agreement_call = contracts_client
        .agreement(station_wallet.clone())
        .create(drone_wallet.address(), drone_station_amount);
    let pending_tx: PendingTransaction<Http> = create_agreement_call.send().await?;
    let tx = pending_tx
        .log_msg("waiting for pending tx after creating agreement")
        .interval(Duration::from_millis(250))
        .retries(10)
        .confirmations(1)
        .await?
        .unwrap();
    debug!("tx index: {:?}", tx.transaction_index);
    debug!("tx hash: {:?}", tx.transaction_hash);
    debug!("tx block number: {:?}", tx.block_number);
    debug!("tx block hash: {:?}", tx.block_hash);
    debug!("tx status: {:?}", tx.status);
    debug!("tx gas used: {:?}", tx.gas_used);

    // Create station - landlord agreement.
    debug!("creating station - landlord agreement");
    let create_agreement_call = contracts_client
        .agreement(station_wallet.clone())
        .create(landlord_wallet.address(), station_landlord_amount);
    let pending_tx: PendingTransaction<Http> = create_agreement_call.send().await?;
    let tx = pending_tx
        .log_msg("waiting for pending tx after creating agreement")
        .interval(Duration::from_millis(250))
        .retries(10)
        .confirmations(1)
        .await?
        .unwrap();
    debug!("tx index: {:?}", tx.transaction_index);
    debug!("tx hash: {:?}", tx.transaction_hash);
    debug!("tx block number: {:?}", tx.block_number);
    debug!("tx block hash: {:?}", tx.block_hash);
    debug!("tx status: {:?}", tx.status);
    debug!("tx gas used: {:?}", tx.gas_used);

    /*
        Sign agreements.
    */

    debug!("signing drone - station agreement");
    contracts_client
        .agreement(drone_wallet.clone())
        .sign(station_wallet.address(), drone_station_amount)
        .send()
        .await?
        .await?;

    debug!("signing station - landlord agreement");
    contracts_client
        .agreement(landlord_wallet.clone())
        .sign(station_wallet.address(), station_landlord_amount)
        .send()
        .await?
        .await?;

    /*
        Get agreements.
    */

    // Get drone - station agreement.
    let drone_station_agreement: Agreement = contracts_client
        .agreement(station_wallet.clone())
        .get(station_wallet.address(), drone_wallet.address())
        .call()
        .await?;
    debug!(
        "agreement between drone and station: {:?}",
        drone_station_agreement
    );
    assert_eq!(
        AgreementStatus::Signed,
        drone_station_agreement.status.into()
    );
    assert_eq!(drone_station_amount, drone_station_agreement.amount);

    // Get station - landlord agreement.
    let station_landlord_agreement: Agreement = contracts_client
        .agreement(station_wallet.clone())
        .get(station_wallet.address(), landlord_wallet.address())
        .call()
        .await?;
    debug!(
        "agreement between station and landlord: {:?}",
        station_landlord_agreement
    );
    assert_eq!(
        AgreementStatus::Signed,
        station_landlord_agreement.status.into()
    );
    assert_eq!(station_landlord_amount, station_landlord_agreement.amount);

    /*
        Landing by drone.
    */

    debug!("starting drone landing");
    let landing_by_drone_call = contracts_client
        .ground_cycle(drone_wallet.clone())
        .landing_by_drone(station_wallet.address())
        .value(drone_station_amount);
    let call_res = landing_by_drone_call.send().await;
    check_contract_res(call_res)?.await?;
    debug!("drone landed successfully");

    /*
        Landing by station.
    */

    debug!("starting station landing");
    let landing_by_station_call = contracts_client
        .ground_cycle(station_wallet.clone())
        .landing_by_station(station_wallet.address(), landlord_wallet.address())
        .value(station_landlord_amount);
    let call_res = landing_by_station_call.send().await;
    check_contract_res(call_res)?.await?;
    debug!("station landed successfully");

    /*
       Takeoff.
    */

    debug!("starting takeoff");
    contracts_client
        .ground_cycle(station_wallet.clone())
        .takeoff()
        .send()
        .await?
        .await?;
    debug!("takeoff was successful");

    /*
        Check balances.
    */

    let agreement_contract_balance = provider.get_balance(agreement_contract_addr, None).await?;
    let drone_balance_after = provider.get_balance(drone_wallet.address(), None).await?;
    let station_balance_after = provider.get_balance(station_wallet.address(), None).await?;
    let landlord_balance_after = provider
        .get_balance(landlord_wallet.address(), None)
        .await?;

    debug!("{:?}", utils::format_ether(drone_balance_after));
    debug!("{:?}", utils::format_ether(station_balance_after));
    debug!("{:?}", utils::format_ether(landlord_balance_after));

    assert!(agreement_contract_balance.is_zero());
    assert_in_delta!(
        drone_balance_after,
        drone_balance_before - drone_station_amount,
        utils::parse_ether(1)?
    );
    assert_in_delta!(
        station_balance_after,
        station_balance_before + drone_station_amount - station_landlord_amount,
        utils::parse_ether(1)?
    );
    assert_in_delta!(
        landlord_balance_after,
        landlord_balance_before + station_landlord_amount,
        utils::parse_ether(1)?
    );

    /*
        Check events.
    */

    let agreement_contract_events = contracts_client
        .agreement(station_wallet.clone())
        .events()
        .from_block(0)
        .query()
        .await?;
    assert_eq!(4, agreement_contract_events.len());
    for (i, event) in agreement_contract_events.iter().enumerate() {
        match event {
            AgreementContractEvents::CreatedFilter(created) => match i {
                0 => {
                    assert_eq!(station_wallet.address(), created.0);
                    assert_eq!(drone_wallet.address(), created.1);
                }
                1 => {
                    assert_eq!(station_wallet.address(), created.0);
                    assert_eq!(landlord_wallet.address(), created.1);
                }
                _ => unreachable!(),
            },
            AgreementContractEvents::SignedFilter(signed) => match i {
                2 => {
                    assert_eq!(station_wallet.address(), signed.0);
                    assert_eq!(drone_wallet.address(), signed.1);
                }
                3 => {
                    assert_eq!(station_wallet.address(), signed.0);
                    assert_eq!(landlord_wallet.address(), signed.1);
                }
                _ => unreachable!(),
            },
        }
    }

    let ground_cycle_contract_events = contracts_client
        .ground_cycle(station_wallet.clone())
        .events()
        .from_block(0)
        .query()
        .await?;
    assert_eq!(2, ground_cycle_contract_events.len());
    for event in ground_cycle_contract_events {
        match event {
            GroundCycleContractEvents::LandingFilter(landing) => {
                assert_eq!(1, landing.0.as_u64());
                assert_eq!(drone_wallet.address(), landing.1);
                assert_eq!(station_wallet.address(), landing.2);
                assert_eq!(landlord_wallet.address(), landing.3);
            }
            GroundCycleContractEvents::TakeoffFilter(takeoff) => {
                assert_eq!(1, takeoff.0.as_u64());
                assert_eq!(drone_wallet.address(), takeoff.1);
                assert_eq!(station_wallet.address(), takeoff.2);
                assert_eq!(landlord_wallet.address(), takeoff.3);
            }
        }
    }

    Ok(())
}

// Client to interact with smart contract on behalf of face of some wallet.
struct Client {
    provider: Provider<Http>,
    agreement_addr: Address,
    ground_cycle_addr: Address,
}

impl Client {
    fn new(provider: Provider<Http>, agreement_addr: Address, ground_cycle_addr: Address) -> Self {
        Self {
            provider,
            agreement_addr,
            ground_cycle_addr,
        }
    }

    fn agreement(
        &self,
        wallet: LocalWallet,
    ) -> AgreementContract<SignerMiddleware<Provider<Http>, LocalWallet>> {
        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), wallet));
        AgreementContract::new(self.agreement_addr, client)
    }

    fn ground_cycle(
        &self,
        wallet: LocalWallet,
    ) -> GroundCycleContract<SignerMiddleware<Provider<Http>, LocalWallet>> {
        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), wallet));
        GroundCycleContract::new(self.ground_cycle_addr, client)
    }
}

// Foundry and ethers-rs don't generate enum, so we do it by our own.
// Be careful when add new enum variants in Solidity and do not forget to update this binding.
#[repr(u8)]
#[derive(Eq, PartialEq, Debug)]
enum AgreementStatus {
    Empty,
    Created,
    Signed,
}

impl From<u8> for AgreementStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Created,
            2 => Self::Signed,
            _ => unimplemented!(),
        }
    }
}

fn check_contract_res(
    res: Result<
        PendingTransaction<Http>,
        ContractError<SignerMiddleware<Provider<Http>, LocalWallet>>,
    >,
) -> Result<PendingTransaction<Http>, Box<dyn std::error::Error>> {
    match res {
        Ok(res) => Ok(res),
        Err(e) => {
            if !e.is_revert() {
                return Err(e.into());
            }
            if let Some(contract_err) = e.decode_contract_revert::<AgreementContractErrors>() {
                return Err(match contract_err {
                    AgreementContractErrors::ErrAlreadySigned(_) => {
                        "contract already signed".to_string()
                    }
                    AgreementContractErrors::ErrInvalidAmount(_) => {
                        "invalid amount to sign the contract".to_string()
                    }
                    AgreementContractErrors::ErrNoAgreement(_) => {
                        "agreement is not found".to_string()
                    }
                    AgreementContractErrors::RevertString(e) => e,
                }
                .into());
            }
            if let Some(contract_err) = e.decode_contract_revert::<GroundCycleContractErrors>() {
                return Err(match contract_err {
                    GroundCycleContractErrors::ErrReceivedNotEnough(tokens) => format!(
                        "required to send {} tokens while execute this method but sent: {}",
                        utils::format_ether(tokens.1),
                        utils::format_ether(tokens.0)
                    ),
                    GroundCycleContractErrors::ErrNoLanding(_) => {
                        "there was no landing for these entities".to_string()
                    }
                    GroundCycleContractErrors::ErrNotSigned(_) => {
                        "agreement is not signed".to_string()
                    }
                    GroundCycleContractErrors::RevertString(e) => e,
                }
                .into());
            }
            Err(e.into())
        }
    }
}
