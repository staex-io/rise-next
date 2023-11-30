use std::{str::FromStr, sync::Arc};

use ethers::{
    contract::{abigen, ContractError},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Selector, U256},
};
use log::LevelFilter;

const RPC_URL: &str = "http://127.0.0.1:8545";

const AGREEMENT_CONTRACT_ADDR: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
// const GROUND_CYCLE_CONTRACT_ADDR: &str = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";

const DRONE_PRIVATE_KEY: &str =
    "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";
const STATION_PRIVATE_KEY: &str =
    "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97";
// const LANDLORD_PRIVATE_KEY: &str =
//     "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

abigen!(
    AgreementContract,
    "../contracts/out/Agreement.sol/Contract.json"
);
// abigen!(
//     GroundCycleContract,
//     "../contracts/out/GroundCycle.sol/Contract.json"
// );

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();

    let agreement_contract_addr: Address = AGREEMENT_CONTRACT_ADDR.parse()?;
    // let ground_cycle_contract_addr: Address = AGREEMENT_CONTRACT_ADDR.parse()?;
    // let ground_cycle_contract = AgreementContract::new(ground_cycle_contract_addr, client.clone());

    let drone_wallet: LocalWallet = LocalWallet::from_str(DRONE_PRIVATE_KEY)?;
    let station_wallet: LocalWallet = LocalWallet::from_str(STATION_PRIVATE_KEY)?;

    eprintln!("{:?}", drone_wallet.address());
    eprintln!("{:?}", station_wallet.address());

    // let landlord_wallet: LocalWallet = LocalWallet::from_str(DRONE_PRIVATE_KEY)?;

    let provider = Provider::<Http>::try_from(RPC_URL)?;

    let station_client = Arc::new(SignerMiddleware::new(
        provider.clone(),
        station_wallet.clone(),
    ));
    let agreement_contract = AgreementContract::new(agreement_contract_addr, station_client);
    let res = agreement_contract
        .create(drone_wallet.address(), U256::from(100u64))
        .call()
        .await?;
    eprintln!("{:?}", res);

    // let drone_client = Arc::new(SignerMiddleware::new(provider, drone_wallet.clone()));
    // let agreement_contract = AgreementContract::new(agreement_contract_addr, drone_client);
    let res = agreement_contract
        .get(station_wallet.address(), drone_wallet.address())
        .call()
        .await?;
    eprintln!("{:?}", res);

    // let drone_client = Arc::new(SignerMiddleware::new(
    //     provider.clone(),
    //     drone_wallet.clone(),
    // ));
    // let agreement_contract = AgreementContract::new(agreement_contract_addr, drone_client);
    // let res = agreement_contract
    //     .sign(station_wallet.address(), U256::from(100u64))
    //     .call()
    //     .await;
    // match res {
    //     Ok(_) => (),
    //     Err(e) => {
    //         eprintln!("{:?}", e.decode_revert::<ErrInvalidAmount>());
    //         eprintln!("{:?}", e.decode_revert::<ErrNoAgreement>());
    //         eprintln!("{:?}", e.decode_revert::<ErrAlreadySigned>());
    //         eprintln!("{:?}", e.decode_revert::<String>());
    //         // match e.decode_revert {
    //         //     _ => (),
    //         // }

    //         // Selector::from(ErrNoAgreement);
    //         match e {
    //             ContractError::Revert(revert) => {
    //                 // eprintln!("{:?}", revert.0.eq(ErrNoAgreement));
    //                 eprintln!("{:?}", revert);
    //             }
    //             _ => (),
    //         }
    //     }
    // }

    // let drone_client = Arc::new(SignerMiddleware::new(provider, drone_wallet.clone()));
    // let agreement_contract = AgreementContract::new(agreement_contract_addr, drone_client);
    // let res = agreement_contract
    //     .get(station_wallet.address(), drone_wallet.address())
    //     .call()
    //     .await?;
    // eprintln!("{:?}", res);

    // cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "create(address,uint256)" 0x14dc79964da2c08b23698b3d3cc7ca32193d9955 100 "()" --rpc-url ${RPC_URL}
    //
    // cast call --private-key 0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97 0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9 --rpc-url ${RPC_URL} "create(address,uint256)" 0x14dc79964da2c08b23698b3d3cc7ca32193d9955 100
    // cast call --private-key 0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97 0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9 --rpc-url ${RPC_URL} "get(address,address)(Agreement)" 0x23618e81e3f5cdf7f54c3d65f7fbc0abf5b21e8f 0x14dc79964da2c08b23698b3d3cc7ca32193d9955

    Ok(())
}
