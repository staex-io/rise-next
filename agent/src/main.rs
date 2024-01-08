use std::{
    fmt::Debug,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use clap::{Parser, Subcommand};
use contracts::Agreement;
use ethers::{
    contract::ContractError,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, PendingTransaction, Provider},
    signers::LocalWallet,
    signers::Signer,
    types::Address,
    utils::{format_ether, parse_ether},
};
use log::{info, warn, LevelFilter};
use tokio::time;

use crate::contracts::{
    AgreementContract, AgreementContractErrors, GroundCycleContract, GroundCycleContractErrors,
    GroundCycleContractEvents,
};

mod contracts;

type Error = Box<dyn std::error::Error>;

/// Command line utility to interact with RISE smart contracts.
#[derive(Parser)]
#[clap(name = "agent")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// RPC url to Ethereum node.
    #[arg(short, long)]
    #[arg(default_value = "https://ethereum-sepolia.publicnode.com")]
    rpc_url: String,
    /// Ethereum node chain id.
    #[arg(short, long)]
    #[arg(default_value = "11155111")]
    chain_id: u64,
    /// Agreement contract address.
    #[arg(short, long)]
    #[arg(default_value = "0x61d6C8D1a59d2e191b5204EaA9C736017B963e95")]
    agreement_contract_addr: String,
    /// Ground cycle contract address.
    #[arg(short, long)]
    #[arg(default_value = "0x9D78aBf1Da69F46227E136Be06d2F1b4a0aaEc52")]
    ground_cycle_contract_addr: String,
    /// Landing wait time. How much time command should wait until
    /// landing will be approved to avoid landing rejection.
    /// Set it using seconds (ex: "300" as 5m).
    #[arg(short, long)]
    #[arg(default_value = "300")]
    landing_wait_time: u64,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateAgreement {
        /// Station private key.
        station_private_key: String,
        /// Drone or landlord address.
        entity_address: String,
        /// Amount in a form like "0.1" in ether.
        amount: String,
    },
    SignAgreement {
        /// Drone or landlord private key.
        entity_private_key: String,
        /// Station address.
        station_address: String,
        /// Amount in a form like "0.1" in ether.
        amount: String,
    },
    LandingByDrone {
        /// Drone private key.
        drone_private_key: String,
        /// Station address.
        station_address: String,
    },
    LandingByStation {
        /// Station private key.
        station_private_key: String,
        /// Drone address.
        drone_address: String,
        /// Landlord address.
        landlord_address: String,
    },
    Takeoff {
        /// Station private key.
        station_private_key: String,
    },
    Events {
        /// Use any private key.
        private_key: String,
        /// Choose block to start query from.
        #[arg(short, long)]
        #[arg(default_value = "4807184")]
        from_block: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    let cli = Cli::parse();
    let app = App::new(
        cli.rpc_url,
        cli.chain_id,
        cli.agreement_contract_addr,
        cli.ground_cycle_contract_addr,
        cli.landing_wait_time,
    )?;
    match cli.command {
        Commands::CreateAgreement {
            station_private_key,
            entity_address,
            amount,
        } => app.create_agreement(station_private_key, entity_address, amount).await?,
        Commands::SignAgreement {
            station_address,
            entity_private_key,
            amount,
        } => app.sign_agreement(station_address, entity_private_key, amount).await?,
        Commands::LandingByDrone {
            drone_private_key,
            station_address,
        } => app.landing_by_drone(drone_private_key, station_address).await?,
        Commands::LandingByStation {
            station_private_key,
            drone_address,
            landlord_address,
        } => app.landing_by_station(station_private_key, drone_address, landlord_address).await?,
        Commands::Takeoff {
            station_private_key,
        } => app.takeoff(station_private_key).await?,
        Commands::Events {
            private_key,
            from_block,
        } => app.events(&private_key, from_block).await?,
    }
    Ok(())
}

struct App {
    provider: Provider<Http>,
    chain_id: u64,
    contracts_client: Client,
    landing_wait_time: u64,
}

impl App {
    fn new(
        rpc_url: String,
        chain_id: u64,
        agreement_contract_addr: String,
        ground_cycle_contract_addr: String,
        landing_wait_time: u64,
    ) -> Result<Self, Error> {
        let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url)?;
        let agreement_contract_addr: Address = agreement_contract_addr.parse()?;
        let ground_cycle_contract_addr: Address = ground_cycle_contract_addr.parse()?;
        let contracts_client =
            Client::new(provider.clone(), agreement_contract_addr, ground_cycle_contract_addr);
        Ok(Self {
            provider,
            chain_id,
            contracts_client,
            landing_wait_time,
        })
    }

    async fn create_agreement(
        &self,
        station_private_key: String,
        entity_address: String,
        amount: String,
    ) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(&station_private_key)?.with_chain_id(self.chain_id);
        let entity_address: Address = entity_address.parse()?;
        let amount = parse_ether(amount)?;
        let create_call = self.contracts_client.agreement(wallet).create(entity_address, amount);
        let call_res = create_call.send().await;
        check_contract_res(call_res)?.await?;
        Ok(())
    }

    async fn sign_agreement(
        &self,
        station_address: String,
        entity_private_key: String,
        amount: String,
    ) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(&entity_private_key)?.with_chain_id(self.chain_id);
        let station_address: Address = station_address.parse()?;
        let amount = parse_ether(amount)?;
        let sign_call = self.contracts_client.agreement(wallet).sign(station_address, amount);
        let call_res = sign_call.send().await;
        check_contract_res(call_res)?.await?;
        Ok(())
    }

    async fn landing_by_drone(
        &self,
        drone_private_key: String,
        station_address: String,
    ) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(&drone_private_key)?.with_chain_id(self.chain_id);
        let station_address: Address = station_address.parse()?;
        let agreement: Agreement = self
            .contracts_client
            .agreement(wallet.clone())
            .get(station_address, wallet.address())
            .call()
            .await?;
        let block_number = self.provider.get_block_number().await?.as_u64();
        info!("starting landing by drone");
        let landing_call = self
            .contracts_client
            .ground_cycle(wallet)
            .landing_by_drone(station_address)
            .value(agreement.amount);
        let call_res = landing_call.send().await;
        check_contract_res(call_res)?.await?;
        info!("drone landed successfully, waiting for confirmation by station");
        self.wait_for_reject(drone_private_key, station_address, block_number).await?;
        Ok(())
    }

    async fn landing_by_station(
        &self,
        station_private_key: String,
        drone_address: String,
        landlord_address: String,
    ) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(&station_private_key)?.with_chain_id(self.chain_id);
        let drone_address: Address = drone_address.parse()?;
        let landlord_address: Address = landlord_address.parse()?;
        let agreement: Agreement = self
            .contracts_client
            .agreement(wallet.clone())
            .get(wallet.address(), landlord_address)
            .call()
            .await?;
        let block_number = self.provider.get_block_number().await?.as_u64();
        info!("starting landing by station");
        let landing_call = self
            .contracts_client
            .ground_cycle(wallet.clone())
            .landing_by_station(drone_address, landlord_address)
            .value(agreement.amount);
        let call_res = landing_call.send().await;
        check_contract_res(call_res)?.await?;
        info!("station landed successfully, waiting for confirmation by drone");
        self.wait_for_reject(station_private_key, wallet.address(), block_number).await?;
        Ok(())
    }

    async fn takeoff(&self, station_private_key: String) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(&station_private_key)?.with_chain_id(self.chain_id);
        self.contracts_client.ground_cycle(wallet).takeoff().send().await?.await?;
        Ok(())
    }

    async fn events(&self, private_key: &str, from_block: u64) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(private_key)?.with_chain_id(self.chain_id);
        const STEP: u64 = 50_000;
        let stop_block = self.provider.get_block_number().await?.as_u64();
        {
            let mut start_from = from_block;
            loop {
                let events = self
                    .contracts_client
                    .agreement(wallet.clone())
                    .events()
                    .from_block(start_from)
                    .to_block(start_from + STEP)
                    .query()
                    .await?;
                for event in events {
                    info!("{:?}", event);
                }
                if start_from + STEP > stop_block {
                    break;
                }
                start_from += STEP;
            }
        }
        {
            let mut start_from = from_block;
            loop {
                let events = self
                    .contracts_client
                    .ground_cycle(wallet.clone())
                    .events()
                    .from_block(start_from)
                    .to_block(start_from + STEP)
                    .query()
                    .await?;
                for event in events {
                    info!("{:?}", event);
                }
                if start_from + STEP > stop_block {
                    break;
                }
                start_from += STEP;
            }
        }
        Ok(())
    }

    async fn wait_for_reject(
        &self,
        private_key: String,
        station_address: Address,
        block_number: u64,
    ) -> Result<(), Error> {
        let started = SystemTime::now();
        let wallet = LocalWallet::from_str(&private_key)?.with_chain_id(self.chain_id);
        const STEP: u64 = 50_000;
        loop {
            let mut start_from = block_number;
            let stop_block = self.provider.get_block_number().await?.as_u64();
            loop {
                let events = self
                    .contracts_client
                    .ground_cycle(wallet.clone())
                    .events()
                    .from_block(start_from)
                    .to_block(start_from + STEP)
                    .query()
                    .await?;
                for event in events {
                    if let GroundCycleContractEvents::LandingFilter(event) = event {
                        if event.2 == station_address {
                            info!("landing was approved");
                            return Ok(());
                        }
                    }
                }
                if start_from + STEP > stop_block {
                    break;
                }
                start_from += STEP;
            }
            if started.elapsed().unwrap() > Duration::from_secs(self.landing_wait_time) {
                break;
            }
            info!("waiting for landing approval");
            time::sleep(Duration::from_secs(1)).await;
        }
        warn!("no approved landing for {}s, starting for landing reject", self.landing_wait_time);
        let reject_call = self.contracts_client.ground_cycle(wallet).reject(station_address);
        let call_res = reject_call.send().await;
        check_contract_res(call_res)?.await?;
        warn!("successfully rejected landing");
        Ok(())
    }
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
) -> Result<PendingTransaction<Http>, Error> {
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
                        format_ether(tokens.1),
                        format_ether(tokens.0)
                    ),
                    GroundCycleContractErrors::ErrNoLanding(_) => {
                        "there was no landing for these entities".to_string()
                    }
                    GroundCycleContractErrors::ErrAgreementNotSigned(_) => {
                        "agreement is not signed".to_string()
                    }
                    GroundCycleContractErrors::ErrNoApprovedLanding(_) => {
                        "there was no approved landing for these entities".to_string()
                    }
                    GroundCycleContractErrors::ErrRejectApprovedLanding(_) => {
                        "cannot reject approved landing".to_string()
                    }
                    GroundCycleContractErrors::ErrRejectTooEarly(_) => {
                        "reject is too early, wait more time".to_string()
                    }
                    GroundCycleContractErrors::RevertString(e) => e,
                }
                .into());
            }
            Err(e.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use assertables::{assert_in_delta, assert_in_delta_as_result};
    use ethers::{providers::Middleware, signers::Signer, types::U256};
    use log::debug;

    use crate::contracts::{Agreement, AgreementContractEvents, GroundCycleContractEvents};

    use super::*;

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

    #[tokio::test]
    async fn all() {
        env_logger::builder().filter_level(LevelFilter::Trace).is_test(true).init();

        // Client to Ethereum node.
        let provider: Provider<Http> = Provider::<Http>::try_from(RPC_URL).unwrap();

        let agreement_contract_addr: Address = AGREEMENT_CONTRACT_ADDR.parse().unwrap();
        let ground_cycle_contract_addr: Address = GROUND_CYCLE_CONTRACT_ADDR.parse().unwrap();

        let drone_wallet: LocalWallet =
            LocalWallet::from_str(DRONE_PRIVATE_KEY).unwrap().with_chain_id(CHAIN_ID);
        let station_wallet: LocalWallet =
            LocalWallet::from_str(STATION_PRIVATE_KEY).unwrap().with_chain_id(CHAIN_ID);
        let landlord_wallet: LocalWallet =
            LocalWallet::from_str(LANDLORD_PRIVATE_KEY).unwrap().with_chain_id(CHAIN_ID);

        debug!("drone wallet address: {:?}", drone_wallet.address());
        debug!("station wallet address: {:?}", station_wallet.address());
        debug!("landlord wallet address: {:?}", landlord_wallet.address());

        // Amounts of agreements between entities.
        let drone_station_amount: U256 = parse_ether(3).unwrap();
        let station_landlord_amount: U256 = parse_ether(5).unwrap();

        let agreement_contract_balance =
            provider.get_balance(agreement_contract_addr, None).await.unwrap();
        assert!(agreement_contract_balance.is_zero());
        let drone_balance_before =
            provider.get_balance(drone_wallet.address(), None).await.unwrap();
        let station_balance_before =
            provider.get_balance(station_wallet.address(), None).await.unwrap();
        let landlord_balance_before =
            provider.get_balance(landlord_wallet.address(), None).await.unwrap();

        let contracts_client =
            Client::new(provider.clone(), agreement_contract_addr, ground_cycle_contract_addr);

        /*
            Create agreements.
        */

        // Create drone - station agreement.
        debug!("creating drone - station agreement");
        let create_agreement_call = contracts_client
            .agreement(station_wallet.clone())
            .create(drone_wallet.address(), drone_station_amount);
        let pending_tx: PendingTransaction<Http> = create_agreement_call.send().await.unwrap();
        let tx = pending_tx
            .log_msg("waiting for pending tx after creating agreement")
            .interval(Duration::from_millis(250))
            .retries(10)
            .confirmations(1)
            .await
            .unwrap()
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
        let pending_tx: PendingTransaction<Http> = create_agreement_call.send().await.unwrap();
        let tx = pending_tx
            .log_msg("waiting for pending tx after creating agreement")
            .interval(Duration::from_millis(250))
            .retries(10)
            .confirmations(1)
            .await
            .unwrap()
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
            .await
            .unwrap()
            .await
            .unwrap();

        debug!("signing station - landlord agreement");
        contracts_client
            .agreement(landlord_wallet.clone())
            .sign(station_wallet.address(), station_landlord_amount)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        /*
            Get agreements.
        */

        // Get drone - station agreement.
        let drone_station_agreement: Agreement = contracts_client
            .agreement(station_wallet.clone())
            .get(station_wallet.address(), drone_wallet.address())
            .call()
            .await
            .unwrap();
        debug!("agreement between drone and station: {:?}", drone_station_agreement);
        assert_eq!(AgreementStatus::Signed, drone_station_agreement.status.into());
        assert_eq!(drone_station_amount, drone_station_agreement.amount);

        // Get station - landlord agreement.
        let station_landlord_agreement: Agreement = contracts_client
            .agreement(station_wallet.clone())
            .get(station_wallet.address(), landlord_wallet.address())
            .call()
            .await
            .unwrap();
        debug!("agreement between station and landlord: {:?}", station_landlord_agreement);
        assert_eq!(AgreementStatus::Signed, station_landlord_agreement.status.into());
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
        check_contract_res(call_res).unwrap().await.unwrap();
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
        check_contract_res(call_res).unwrap().await.unwrap();
        debug!("station landed successfully");

        /*
           Takeoff.
        */

        debug!("starting takeoff");
        contracts_client
            .ground_cycle(station_wallet.clone())
            .takeoff()
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
        debug!("takeoff was successful");

        /*
            Check balances.
        */

        let agreement_contract_balance =
            provider.get_balance(agreement_contract_addr, None).await.unwrap();
        let drone_balance_after = provider.get_balance(drone_wallet.address(), None).await.unwrap();
        let station_balance_after =
            provider.get_balance(station_wallet.address(), None).await.unwrap();
        let landlord_balance_after =
            provider.get_balance(landlord_wallet.address(), None).await.unwrap();

        debug!("{:?}", format_ether(drone_balance_after));
        debug!("{:?}", format_ether(station_balance_after));
        debug!("{:?}", format_ether(landlord_balance_after));

        assert!(agreement_contract_balance.is_zero());
        assert_in_delta!(
            drone_balance_after,
            drone_balance_before - drone_station_amount,
            parse_ether(0.1).unwrap()
        );
        assert_in_delta!(
            station_balance_after,
            station_balance_before + drone_station_amount - station_landlord_amount,
            parse_ether(0.1).unwrap()
        );
        assert_in_delta!(
            landlord_balance_after,
            landlord_balance_before + station_landlord_amount,
            parse_ether(0.1).unwrap()
        );

        /*
            Check events.
        */

        let agreement_contract_events = contracts_client
            .agreement(station_wallet.clone())
            .events()
            .from_block(0)
            .query()
            .await
            .unwrap();
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
            .await
            .unwrap();
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
                GroundCycleContractEvents::RejectFilter(_) => (),
            }
        }
    }
}
