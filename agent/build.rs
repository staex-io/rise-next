use ethers::contract::Abigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Abigen::new("DIDContract", "./assets/DIDContract.json")?
        .generate()?
        .write_to_file("src/did_contract.rs")?;
    Abigen::new("AgreementContract", "./assets/AgreementContract.json")?
        .generate()?
        .write_to_file("src/agreement_contract.rs")?;
    Abigen::new("GroundCycleContract", "./assets/GroundCycleContract.json")?
        .generate()?
        .write_to_file("src/ground_cycle_contract.rs")?;
    Abigen::new("GroundCycleNoCryptoContract", "./assets/GroundCycleNoCryptoContract.json")?
        .generate()?
        .write_to_file("src/ground_cycle_no_crypto_contract.rs")?;
    Ok(())
}
