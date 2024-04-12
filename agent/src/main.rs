use std::{
    fmt::Debug,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use clap::{Parser, Subcommand};
use client::Client;
use contracts_rs::{
    Agreement, AgreementContractErrors, GroundCycleContractErrors, GroundCycleContractEvents,
};
use ethers::{
    contract::{ContractError, EthLogDecode, Event},
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    signers::Signer,
    types::{Address, Filter, H256},
    utils::{format_ether, keccak256, parse_ether},
};
use log::{debug, error, info, warn, LevelFilter};
use serde::Deserialize;
use tokio::time::{self, sleep};
// use image::{GenericImageView,DynamicImage};

mod client;
mod indexer;

// We use this step when iterating over blocks
// to get smart contract events from these blocks.
pub(crate) const BLOCK_STEP: u64 = 1;

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
    /// DID smart contract address.
    #[arg(short, long)]
    did_contract_addr: Option<String>,
    /// Agreement smart contract address.
    #[arg(short, long)]
    agreement_contract_addr: Option<String>,
    /// Ground cycle smart contract address.
    #[arg(short, long)]
    ground_cycle_contract_addr: Option<String>,
    /// Landing wait time. How much time command should wait until
    /// landing will be approved to avoid landing rejection.
    /// Set it using seconds (ex: "300" as 5m).
    #[arg(short, long)]
    #[arg(default_value = "300")]
    landing_wait_time: u64,
    /// Specify video device index.
    /// It works only on Linux.
    #[arg(long)]
    #[arg(default_value = "0")]
    device_index: Option<u8>,
    /// Choose env with predefined config values.
    /// Possible values: local, sepolia, lisk-sepolia.
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
        /// Station address. Can be empty.
        /// If empty agent tries to scan station address from camera.
        station_address: Option<String>,
    },
    /// Process landing by station.
    LandingByStation {
        /// Station private key.
        station_private_key: String,
        /// Landlord address.
        landlord_address: String,
        /// Drone address. Can be empty.
        /// If empty agent tries to scan drone address from camera.
        drone_address: Option<String>,
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
    /// Run indexer.
    Indexer {
        /// Database data source name.
        /// Use it for database connection.
        #[arg(default_value = "sqlite:rise-next.sqlite")]
        dsn: String,
        /// HTTP API host.
        #[arg(default_value = "127.0.0.1")]
        host: String,
        /// HTTP API port number.
        #[arg(default_value = "4698")]
        port: u16,
        /// From which block indexer should start scanning.
        #[arg(default_value = "5306804")]
        from_block: u64,
    },
}

#[derive(Default)]
struct Config {
    rpc_url: String,
    chain_id: u64,
    did_contract_addr: String,
    agreement_contract_addr: String,
    ground_cycle_contract_addr: String,
    ground_cycle_no_crypto_contract_addr: String,
}

impl Config {
    fn new(
        env: String,
        rpc_url: Option<String>,
        chain_id: Option<u64>,
        did_contract_addr: Option<String>,
        agreement_contract_addr: Option<String>,
        ground_cycle_contract_addr: Option<String>,
    ) -> Self {
        let mut cfg = match env.as_str() {
            "custom" => Self::default(),
            "local" => Self {
                rpc_url: "http://127.0.0.1:8545".to_string(),
                chain_id: 31337,
                did_contract_addr: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
                agreement_contract_addr: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512".to_string(),
                ground_cycle_contract_addr: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
                    .to_string(),
                ground_cycle_no_crypto_contract_addr: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
                    .to_string(), // todo: set it
            },
            "sepolia" => Self {
                rpc_url: "https://ethereum-sepolia.publicnode.com".to_string(),
                chain_id: 11155111,
                did_contract_addr: "0x17536460b997842f8396409514986905eF63b58E".to_string(),
                agreement_contract_addr: "0x94a71B1940741145454Bb7AA490A66b86369F160".to_string(),
                ground_cycle_contract_addr: "0x60197B0C29EE4F80ad3B5e88A86EC235aF05d0CA"
                    .to_string(),
                ground_cycle_no_crypto_contract_addr: "0x60197B0C29EE4F80ad3B5e88A86EC235aF05d0CA"
                    .to_string(), // todo: set it
            },
            "lisk-sepolia" => Self {
                rpc_url: "https://rpc.sepolia-api.lisk.com".to_string(),
                chain_id: 4202,
                did_contract_addr: "0x3bA4B1e2a1c775267e7b6288A8D66c411A56C8c3".to_string(),
                agreement_contract_addr: "0x6beFEd6d4D0e4a9198266EAdf295F5C1eD78C3c7".to_string(),
                ground_cycle_contract_addr: "0x677418C0141780DEbaAac07A508700410CCeBd9F"
                    .to_string(),
                ground_cycle_no_crypto_contract_addr: "0x677418C0141780DEbaAac07A508700410CCeBd9F"
                    .to_string(), // todo: set it
            },
            _ => unimplemented!(),
        };
        if let Some(rpc_url) = rpc_url {
            cfg.rpc_url = rpc_url;
        }
        if let Some(chain_id) = chain_id {
            cfg.chain_id = chain_id;
        }
        if let Some(did_contract_addr) = did_contract_addr {
            cfg.did_contract_addr = did_contract_addr
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
        .filter_module("agent", LevelFilter::Trace)
        .init();
    let cli = Cli::parse();
    let cfg = Config::new(
        cli.env,
        cli.rpc_url,
        cli.chain_id,
        cli.did_contract_addr,
        cli.agreement_contract_addr,
        cli.ground_cycle_contract_addr,
    );
    #[cfg(target_os = "linux")]
    let app = App::new(&cfg, cli.landing_wait_time, cli.device_index)?;
    #[cfg(target_os = "macos")]
    let app = App::new(&cfg, cli.landing_wait_time)?;
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
        Commands::Indexer {
            dsn,
            host,
            port,
            from_block,
        } => {
            indexer::run(cfg, dsn, host, port, from_block).await?;
            tokio::signal::ctrl_c().await?;
        }
    }
    Ok(())
}

struct App {
    provider: Provider<Http>,
    chain_id: u64,
    contracts_client: Client<Provider<Http>>,
    landing_wait_time: u64,
    #[cfg(target_os = "linux")]
    device_index: u8,
}

impl App {
    #[cfg(target_os = "linux")]
    fn new(cfg: &Config, landing_wait_time: u64, device_index: Option<u8>) -> Result<Self, Error> {
        let provider: Provider<Http> = Provider::<Http>::try_from(cfg.rpc_url.clone())?;
        let agreement_contract_addr: Address = cfg.agreement_contract_addr.parse()?;
        let ground_cycle_contract_addr: Address = cfg.ground_cycle_contract_addr.parse()?;
        let contracts_client =
            Client::new(provider.clone(), agreement_contract_addr, ground_cycle_contract_addr);
        Ok(Self {
            provider,
            chain_id: cfg.chain_id,
            contracts_client,
            landing_wait_time,
            device_index: device_index.unwrap_or_default(),
        })
    }

    #[cfg(target_os = "macos")]
    fn new(cfg: &Config, landing_wait_time: u64) -> Result<Self, Error> {
        let provider: Provider<Http> = Provider::<Http>::try_from(cfg.rpc_url.clone())?;
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
        station_address: Option<String>,
    ) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(&drone_private_key)?.with_chain_id(self.chain_id);
        if let Some(station_address) = station_address {
            // If station address is not None we need to execute landing method and exit.
            let station_address: Address = station_address.parse()?;
            self.landing_by_drone_(wallet, station_address).await
        } else {
            // If station address is None we are starting infinity node with camera support.
            #[cfg(target_os = "linux")]
            let station_address = scan_address(self.device_index).await?;
            #[cfg(target_os = "macos")]
            let station_address = scan_address().await?;
            self.landing_by_drone_(wallet, station_address).await
        }
    }

    async fn landing_by_drone_(
        &self,
        wallet: LocalWallet,
        station_address: Address,
    ) -> Result<(), Error> {
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
            .ground_cycle_signer(wallet.clone())
            .landing_by_drone(station_address)
            .value(agreement.amount);
        let call_res = landing_call.send().await;
        check_contract_res(call_res)?.await?;
        info!("drone landed successfully, waiting for confirmation by station");
        self.wait_for_reject(wallet, station_address, block_number).await
    }

    async fn landing_by_station(
        &self,
        station_private_key: String,
        drone_address: Option<String>,
        landlord_address: String,
    ) -> Result<(), Error> {
        let wallet = LocalWallet::from_str(&station_private_key)?.with_chain_id(self.chain_id);
        let landlord_address: Address = landlord_address.parse()?;
        let agreement: Agreement = check_contract_res(
            self.contracts_client.agreement().get(wallet.address(), landlord_address).call().await,
        )?;
        if let Some(drone_address) = drone_address {
            // If drone address is not None we need to execute landing method and exit.
            let drone_address: Address = drone_address.parse()?;
            info!("execute landing by station and exit branch");
            self.landing_by_station_(wallet, agreement, drone_address, landlord_address).await
        } else {
            // If drone address is None we are starting infinity node with camera support.
            #[cfg(target_os = "linux")]
            let drone_address = scan_address(self.device_index).await?;
            #[cfg(target_os = "macos")]
            let drone_address = scan_address().await?;
            self.landing_by_station_(wallet, agreement, drone_address, landlord_address).await
        }
    }

    async fn landing_by_station_(
        &self,
        wallet: LocalWallet,
        agreement: Agreement,
        drone_address: Address,
        landlord_address: Address,
    ) -> Result<(), Error> {
        info!("starting landing by station");
        let block_number = self.provider.get_block_number().await?.as_u64();
        let landing_call = self
            .contracts_client
            .ground_cycle_signer(wallet.clone())
            .landing_by_station(drone_address, landlord_address)
            .value(agreement.amount);
        let call_res = landing_call.send().await;
        check_contract_res(call_res)?.await?;
        info!("station landed successfully, waiting for confirmation by drone");
        self.wait_for_reject(wallet.clone(), wallet.address(), block_number).await
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
        wallet: LocalWallet,
        station_address: Address,
        start_block: u64,
    ) -> Result<(), Error> {
        let started = SystemTime::now();
        loop {
            // Get latest block in a chain.
            let stop_block = self.provider.get_block_number().await?.as_u64();
            let mut from_block = start_block;
            loop {
                let to_block = from_block + BLOCK_STEP;
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
            let to_block = from_block + BLOCK_STEP;
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
                    GroundCycleContractErrors::ErrHandshake(_) => {
                        "failed to pass handshake".to_string()
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

#[cfg(target_os = "linux")]
async fn scan_address(device_index: u8) -> Result<Address, Error> {
    let cmd = ffmpeg_read_camera_cmd(device_index);
    scan_address_(cmd).await
}

#[cfg(target_os = "macos")]
async fn scan_address() -> Result<Address, Error> {
    let cmd = ffmpeg_read_camera_cmd();
    scan_address_(cmd).await
}

async fn scan_address_(mut cmd: std::process::Command) -> Result<Address, Error> {
    let decoder = bardecoder::default_decoder();
    loop {
        let output = cmd.output()?;
        if !output.status.success() {
            error!("failed to scan camera output");
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
                Ok(data) => {
                    if let Ok(address) = data.address.parse() {
                        return Ok(address);
                    } else {
                        error!("address from qr code is invalid")
                    }
                }
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
fn ffmpeg_read_camera_cmd(device_index: u8) -> std::process::Command {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.args(
        format!("-f v4l2 -i /dev/video{} -vframes 1 -f image2pipe -c:v mjpeg -", device_index)
            .split(' ')
            .collect::<Vec<&str>>(),
    );
    cmd
}

#[cfg(target_os = "macos")]
fn ffmpeg_read_camera_cmd() -> std::process::Command {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.args(
        "-f avfoundation -pixel_format yuyv422 -probesize 16M -r 30 -i 0:none -update 1 -vframes 1 -f apng pipe:"
        .split(' ')
        .collect::<Vec<&str>>(),
    );
    cmd
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use assertables::{assert_in_delta, assert_in_delta_as_result};
    use contracts_rs::{Agreement, AgreementContractEvents, GroundCycleContractEvents};
    use ethers::{
        providers::{Middleware, PendingTransaction},
        signers::Signer,
        types::U256,
    };
    use log::debug;

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

        let contracts_client: Client<Provider<Http>> =
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
