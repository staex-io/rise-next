use std::{str::FromStr, sync::Arc, time::Duration};

use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, PendingTransaction, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
    utils,
};
use log::{debug, LevelFilter};

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

abigen!(
    AgreementContract,
    "../contracts/out/Agreement.sol/Contract.json"
);
abigen!(
    GroundCycleContract,
    "../contracts/out/GroundCycle.sol/Contract.json"
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();

    let provider: Provider<Http> = Provider::<Http>::try_from(RPC_URL)?;

    let agreement_contract_addr: Address = AGREEMENT_CONTRACT_ADDR.parse()?;
    let ground_cycle_contract_addr: Address = AGREEMENT_CONTRACT_ADDR.parse()?;

    let drone_wallet: LocalWallet =
        LocalWallet::from_str(DRONE_PRIVATE_KEY)?.with_chain_id(CHAIN_ID);
    let station_wallet: LocalWallet =
        LocalWallet::from_str(STATION_PRIVATE_KEY)?.with_chain_id(CHAIN_ID);
    let landlord_wallet: LocalWallet =
        LocalWallet::from_str(LANDLORD_PRIVATE_KEY)?.with_chain_id(CHAIN_ID);

    debug!("drone wallet address: {:?}", drone_wallet.address());
    debug!("station wallet address: {:?}", station_wallet.address());
    debug!("landlord wallet address: {:?}", landlord_wallet.address());

    // todo: get drone, station, landlord balances
    let drone_station_amount: U256 = U256::from(utils::parse_ether(3)?);
    let station_landlord_amount: U256 = U256::from(utils::parse_ether(5)?);

    let create_agreement_method = agreement_contract_client(
        provider.clone(),
        agreement_contract_addr,
        station_wallet.clone(),
    )
    .create(drone_wallet.address(), drone_station_amount);
    let pending_tx: PendingTransaction<Http> = create_agreement_method.send().await?;
    let tx = pending_tx
        .log_msg("waiting for pending tx after creating agreement")
        .interval(Duration::from_millis(250))
        .retries(10)
        .await?
        .unwrap();
    debug!("tx index: {:?}", tx.transaction_index);
    debug!("tx hash: {:?}", tx.transaction_hash);
    debug!("tx block number: {:?}", tx.block_number);
    debug!("tx block hash: {:?}", tx.block_hash);
    debug!("tx status: {:?}", tx.status);
    debug!("tx gas used: {:?}", tx.gas_used);

    let get_agreement_method =
        agreement_contract_client(provider, agreement_contract_addr, station_wallet.clone())
            .get(station_wallet.address(), drone_wallet.address());
    let drone_station_agreement: Agreement = get_agreement_method.call().await?;
    debug!(
        "agreement between drone and station: {:?}",
        drone_station_agreement
    );
    assert_eq!(1, drone_station_agreement.status);
    assert_eq!(drone_station_amount, drone_station_agreement.amount);

    Ok(())
}

fn agreement_contract_client(
    provider: Provider<Http>,
    address: Address,
    wallet: LocalWallet,
) -> AgreementContract<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    AgreementContract::new(address, client)
}

fn ground_cycle_contract_client(
    provider: Provider<Http>,
    address: Address,
    wallet: LocalWallet,
) -> GroundCycleContract<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    GroundCycleContract::new(address, client)
}
