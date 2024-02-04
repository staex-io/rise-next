use std::{
    fmt::Debug,
    str::{from_utf8, FromStr},
    sync::Arc,
    time::{Duration, SystemTime},
};

use clap::{Parser, Subcommand};
use contracts::Agreement;
use ethers::{
    contract::{ContractError, EthLogDecode, Event},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    signers::Signer,
    types::{Address, Filter, H256},
    utils::{format_ether, keccak256, parse_ether},
};
use log::{debug, error, info, warn, LevelFilter};
use serde::Deserialize;
use tokio::time::{self, sleep};

use crate::contracts::{
    AgreementContract, AgreementContractErrors, GroundCycleContract, GroundCycleContractErrors,
    GroundCycleContractEvents,
};

mod contracts;

// We use this step when iterating over blocks
// to get smart contract events from these blocks.
const BLOCKS_STEP: u64 = 10;

type Error = Box<dyn std::error::Error>;

/// Command line utility to interact with RISE smart contracts.
#[derive(Parser)]
#[clap(name = "agent")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// RPC url to Ethereum node.
    #[arg(short, long)]
    rpc_url: Option<String>,
    /// Ethereum node chain id.
    #[arg(short, long)]
    chain_id: Option<u64>,
    /// Agreement contract address.
    #[arg(short, long)]
    agreement_contract_addr: Option<String>,
    /// Ground cycle contract address.
    #[arg(short, long)]
    ground_cycle_contract_addr: Option<String>,
    /// Landing wait time. How much time command should wait until
    /// landing will be approved to avoid landing rejection.
    /// Set it using seconds (ex: "300" as 5m).
    #[arg(short, long)]
    #[arg(default_value = "300")]
    landing_wait_time: u64,
    /// Choose env with predefined config values.
    /// Possible values: custom, local, sepolia.
    #[arg(short, long)]
    #[arg(default_value = "local")]
    env: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create agreement between station and entity.
    CreateAgreement {
        /// Station private key.
        station_private_key: String,
        /// Drone or landlord address.
        entity_address: String,
        /// Amount in a form like "0.1" in ether.
        amount: String,
    },
    /// Sign agreement between entity and station.
    SignAgreement {
        /// Drone or landlord private key.
        entity_private_key: String,
        /// Station address.
        station_address: String,
        /// Amount in a form like "0.1" in ether.
        amount: String,
    },
    /// Process landing by drone.
    LandingByDrone {
        /// Drone private key.
        drone_private_key: String,
        /// Station address.
        station_address: String,
    },
    /// Process landing by station.
    LandingByStation {
        /// Station private key.
        station_private_key: String,
        /// Drone address.
        drone_address: String,
        /// Landlord address.
        landlord_address: String,
    },
    /// Process Takeoff by station.
    Takeoff {
        /// Station private key.
        station_private_key: String,
    },
    /// Show events from smart contracts.
    Events {
        /// Choose block to start query from.
        #[arg(short, long)]
        #[arg(default_value = "4807184")]
        from_block: u64,
    },
}

#[derive(Default)]
struct Config {
    rpc_url: String,
    chain_id: u64,
    agreement_contract_addr: String,
    ground_cycle_contract_addr: String,
}

impl Config {
    fn new(
        env: String,
        rpc_url: Option<String>,
        chain_id: Option<u64>,
        agreement_contract_addr: Option<String>,
        ground_cycle_contract_addr: Option<String>,
    ) -> Self {
        let mut cfg = match env.as_str() {
            "custom" => Self::default(),
            "local" => Self {
                rpc_url: "http://127.0.0.1:8545".to_string(),
                chain_id: 31337,
                agreement_contract_addr: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
                ground_cycle_contract_addr: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
                    .to_string(),
            },
            "sepolia" => Self {
                rpc_url: "https://ethereum-sepolia.publicnode.com".to_string(),
                chain_id: 11155111,
                agreement_contract_addr: "0x61d6C8D1a59d2e191b5204EaA9C736017B963e95".to_string(),
                ground_cycle_contract_addr: "0x9D78aBf1Da69F46227E136Be06d2F1b4a0aaEc52"
                    .to_string(),
            },
            _ => unimplemented!(),
        };
        if let Some(rpc_url) = rpc_url {
            cfg.rpc_url = rpc_url;
        }
        if let Some(chain_id) = chain_id {
            cfg.chain_id = chain_id;
        }
        if let Some(agreement_contract_addr) = agreement_contract_addr {
            cfg.agreement_contract_addr = agreement_contract_addr
        }
        if let Some(ground_cycle_contract_addr) = ground_cycle_contract_addr {
            cfg.ground_cycle_contract_addr = ground_cycle_contract_addr
        }
        cfg
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::builder()
        .filter_level(LevelFilter::Off)
        .filter_module("agent", LevelFilter::Debug)
        .init();
    let cli = Cli::parse();
    let cfg = Config::new(
        cli.env,
        cli.rpc_url,
        cli.chain_id,
        cli.agreement_contract_addr,
        cli.ground_cycle_contract_addr,
    );
    let app = App::new(cfg, cli.landing_wait_time)?;
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
        Commands::Events { from_block } => app.events(from_block).await?,
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
    fn new(cfg: Config, landing_wait_time: u64) -> Result<Self, Error> {
        let provider: Provider<Http> = Provider::<Http>::try_from(cfg.rpc_url)?;
        let agreement_contract_addr: Address = cfg.agreement_contract_addr.parse()?;
        let ground_cycle_contract_addr: Address = cfg.ground_cycle_contract_addr.parse()?;
        let contracts_client =
            Client::new(provider.clone(), agreement_contract_addr, ground_cycle_contract_addr);
        Ok(Self {
            provider,
            chain_id: cfg.chain_id,
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
        let create_call =
            self.contracts_client.agreement_signer(wallet).create(entity_address, amount);
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
        let sign_call =
            self.contracts_client.agreement_signer(wallet).sign(station_address, amount);
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
        let agreement = check_contract_res(
            self.contracts_client.agreement().get(station_address, wallet.address()).call().await,
        )?;
        let block_number = self.provider.get_block_number().await?.as_u64();
        info!(
            "starting landing by drone from block {} with agreement amount {}",
            block_number,
            format_ether(agreement.amount)
        );
        let landing_call = self
            .contracts_client
            .ground_cycle_signer(wallet)
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
        let agreement: Agreement = check_contract_res(
            self.contracts_client.agreement().get(wallet.address(), landlord_address).call().await,
        )?;
        let block_number = self.provider.get_block_number().await?.as_u64();
        info!("starting landing by station");
        let landing_call = self
            .contracts_client
            .ground_cycle_signer(wallet.clone())
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
        self.contracts_client.ground_cycle_signer(wallet).takeoff().send().await?.await?;
        Ok(())
    }

    async fn events(&self, from_block: u64) -> Result<(), Error> {
        let to_block = self.provider.get_block_number().await?.as_u64();

        info!("getting events from agreement smart contract without generated client");
        let agreement_created_hash = H256::from(keccak256("Created(address,address)".as_bytes()));
        let agreement_signed_hash = H256::from(keccak256("Signed(address,address)".as_bytes()));
        let filter = Filter::new().address(self.contracts_client.agreement_addr).from_block(0);
        let logs = self.provider.get_logs(&filter).await?;
        for log in logs {
            let topic0 = log.topics[0];
            let token0 = Address::from(log.topics[1]);
            let token1 = Address::from(log.topics[2]);
            if topic0 == agreement_created_hash {
                info!("agreement was created between {:?} and {:?}", token0, token1);
            } else if topic0 == agreement_signed_hash {
                info!("agreement was signed between {:?} and {:?}", token0, token1);
            } else {
                warn!("unknown topic0 for agreement smart contract log");
            }
        }

        info!("agreement smart contract events");
        Self::read_events(self.contracts_client.agreement().events(), from_block, to_block).await?;

        info!("ground cycle smart contract events");
        Self::read_events(self.contracts_client.ground_cycle().events(), from_block, to_block)
            .await?;

        Ok(())
    }

    async fn wait_for_reject(
        &self,
        private_key: String,
        station_address: Address,
        start_block: u64,
    ) -> Result<(), Error> {
        let started = SystemTime::now();
        let wallet = LocalWallet::from_str(&private_key)?.with_chain_id(self.chain_id);
        loop {
            // Get latest block in a chain.
            let stop_block = self.provider.get_block_number().await?.as_u64();
            let mut from_block = start_block;
            loop {
                let to_block = from_block + BLOCKS_STEP;
                let events = self
                    .contracts_client
                    .ground_cycle()
                    .events()
                    .from_block(from_block)
                    .to_block(to_block)
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
                if to_block > stop_block {
                    break;
                }
                from_block = to_block + 1;
            }
            if started.elapsed().unwrap() > Duration::from_secs(self.landing_wait_time) {
                break;
            }
            info!("waiting for landing approval");
            time::sleep(Duration::from_secs(1)).await;
        }
        warn!("no approved landing for {}s, starting for landing reject", self.landing_wait_time);
        let reject_call = self.contracts_client.ground_cycle_signer(wallet).reject(station_address);
        let call_res = reject_call.send().await;
        check_contract_res(call_res)?.await?;
        warn!("successfully rejected landing");
        Ok(())
    }

    async fn read_events<D: EthLogDecode + Debug>(
        mut client: Event<Arc<Provider<Http>>, Provider<Http>, D>,
        start_block: u64,
        stop_block: u64,
    ) -> Result<(), Error> {
        let mut from_block = start_block;
        loop {
            let to_block = from_block + BLOCKS_STEP;
            client = client.from_block(from_block).to_block(to_block);
            let events = client.query().await?;
            for event in events {
                info!("{:?}", event);
            }
            if to_block > stop_block {
                break;
            }
            from_block = to_block + 1;
        }
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

    fn agreement(&self) -> AgreementContract<Provider<Http>> {
        AgreementContract::new(self.agreement_addr, Arc::new(self.provider.clone()))
    }

    fn agreement_signer(
        &self,
        wallet: LocalWallet,
    ) -> AgreementContract<SignerMiddleware<Provider<Http>, LocalWallet>> {
        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), wallet));
        AgreementContract::new(self.agreement_addr, client)
    }

    fn ground_cycle(&self) -> GroundCycleContract<Provider<Http>> {
        GroundCycleContract::new(self.ground_cycle_addr, Arc::new(self.provider.clone()))
    }

    fn ground_cycle_signer(
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

fn check_contract_res<T, P: Middleware>(res: Result<T, ContractError<P>>) -> Result<T, Error> {
    match res {
        Ok(res) => Ok(res),
        Err(e) => {
            if !e.is_revert() {
                return Err(e.to_string().into());
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
                    GroundCycleContractErrors::ErrTakeoffRequired(_) => {
                        "it is required to takeoff first before doing landing".to_string()
                    }
                    GroundCycleContractErrors::RevertString(e) => e,
                }
                .into());
            }
            Err(e.to_string().into())
        }
    }
}

#[derive(Deserialize)]
struct QrCodeOutput {
    address: String,
}

async fn scan_address() -> Result<String, Error> {
    let mut cmd = ffmpeg_read_camera_cmd();
    let decoder = bardecoder::default_decoder();
    loop {
        let output = cmd.output()?;
        if !output.status.success() {
            error!("failed to scan camera output: {}", from_utf8(&output.stderr)?);
            sleep(Duration::from_secs(1)).await;
            continue;
        }
        let img = image::load_from_memory(&output.stdout)?;
        let results = decoder.decode(&img);
        if results.len() != 1 {
            debug!("no available qr code; continue scanning camera output");
            sleep(Duration::from_millis(250)).await;
            continue;
        }
        if let Ok(result) = &results[0] {
            match serde_json::from_str::<QrCodeOutput>(result) {
                Ok(data) => return Ok(data.address),
                Err(_) => {
                    warn!("failed to decode json response from qr code output");
                    sleep(Duration::from_millis(250)).await;
                    continue;
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn ffmpeg_read_camera_cmd() -> std::process::Command {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.args(
        "-f v4l2 -i /dev/video0 -vframes 1 -f image2pipe -c:v mjpeg -"
            .split(' ')
            .collect::<Vec<&str>>(),
    );
    cmd
}

#[cfg(target_os = "macos")]
fn ffmpeg_read_camera_cmd() -> std::process::Command {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.args(vec![
        "-f",
        "avfoundation",
        "-pixel_format",
        "yuyv422",
        "-probesize",
        "16M",
        "-r",
        "30",
        "-i",
        "0:none",
        "-update",
        "1",
        "-vframes",
        "1",
        "-f",
        "apng",
        "pipe:",
    ]);
    cmd
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use assertables::{assert_in_delta, assert_in_delta_as_result};
    use ethers::{
        providers::{Middleware, PendingTransaction},
        signers::Signer,
        types::U256,
    };
    use log::debug;

    use crate::contracts::{Agreement, AgreementContractEvents, GroundCycleContractEvents};

    use super::*;

    // Default Anvil local RPC address.
    const RPC_URL: &str = "http://127.0.0.1:8545";
    // Default Anvil local chain id.
    const CHAIN_ID: u64 = 31337;

    // Default contracts addresses after deploy to local Anvil node.
    const AGREEMENT_CONTRACT_ADDR: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    const GROUND_CYCLE_CONTRACT_ADDR: &str = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";

    // Default private keys from Anvil node.
    const DRONE_PRIVATE_KEY: &str =
        "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";
    const STATION_PRIVATE_KEY: &str =
        "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97";
    const LANDLORD_PRIVATE_KEY: &str =
        "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

    #[tokio::test]
    async fn all() {
        eprintln!(); // insert first \n before testing logs
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
            .agreement_signer(station_wallet.clone())
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
            .agreement_signer(station_wallet.clone())
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
            .agreement_signer(drone_wallet.clone())
            .sign(station_wallet.address(), drone_station_amount)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        debug!("signing station - landlord agreement");
        contracts_client
            .agreement_signer(landlord_wallet.clone())
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
            .agreement()
            .get(station_wallet.address(), drone_wallet.address())
            .call()
            .await
            .unwrap();
        debug!("agreement between drone and station: {:?}", drone_station_agreement);
        assert_eq!(AgreementStatus::Signed, drone_station_agreement.status.into());
        assert_eq!(drone_station_amount, drone_station_agreement.amount);

        // Get station - landlord agreement.
        let station_landlord_agreement: Agreement = contracts_client
            .agreement()
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
            .ground_cycle_signer(drone_wallet.clone())
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
            .ground_cycle_signer(station_wallet.clone())
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
            .ground_cycle_signer(station_wallet.clone())
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

        let agreement_contract_events =
            contracts_client.agreement().events().from_block(0).query().await.unwrap();
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

        let ground_cycle_contract_events =
            contracts_client.ground_cycle().events().from_block(0).query().await.unwrap();
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
