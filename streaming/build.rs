use ethers::contract::Abigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Abigen::new("DataProvingContract", "./assets/DataProvingContract.json")?
        .generate()?
        .write_to_file("src/data_proving_contract.rs")?;
    Ok(())
}
