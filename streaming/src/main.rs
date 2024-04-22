use std::process::ExitCode;
use std::str::from_utf8;
use std::str::FromStr;
use std::sync::Arc;

use base64::Engine;
use clap::Parser;
use contracts_rs::DataProvingContract;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::{middleware::SignerMiddleware, signers::Signer, types::Address};
use sha2::{Digest, Sha256};

/// Command line utility to interact with NEXA streaming.
#[derive(Parser)]
#[clap(name = "streaming")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set input for ffmpeg (ex: -i /rec/drone.flv).
    #[arg(short, long)]
    #[arg(default_value = "rec/drone.flv")]
    input: String,
    /// Set output path where you want to save image.
    #[arg(short, long)]
    #[arg(default_value = "last.jpg")]
    output: String,
    /// Set smart contract address.
    #[arg(short, long)]
    #[arg(default_value = "0xc01117e3cc1e59DCB924F54177745Acc56592572")]
    smart_contract_address: String,
    /// Set private key to interact with smart contract.
    private_key: String,
}

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.args(format!("-sseof -3 -i {} -update 1 -q:v 1 {}", cli.input, cli.output).split(' '));
    let output = cmd.output().unwrap();
    if !output.status.success() {
        eprintln!("Exit status is not success: {}", output.status);
        eprintln!("There are logs from output:");
        eprintln!("{:?}", from_utf8(&output.stdout));
        return Ok(ExitCode::FAILURE);
    }

    let buf = std::fs::read(cli.output)?;
    let mut hasher = Sha256::new();
    hasher.update(buf);
    let hash = hasher.finalize();

    let mut encoded = String::new();
    base64::prelude::BASE64_STANDARD.encode_string(hash, &mut encoded);
    eprintln!("Encoded image hash to base64: {:?}", encoded);

    let wallet = LocalWallet::from_str(&cli.private_key)?.with_chain_id(4202u64);
    let provider: Provider<Http> = Provider::<Http>::try_from("https://rpc.sepolia-api.lisk.com")?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    let smart_contract_address = Address::from_str(&cli.smart_contract_address)?;
    let smart_contract = DataProvingContract::new(smart_contract_address, client);
    smart_contract.save(encoded).send().await?;

    Ok(ExitCode::SUCCESS)
}
