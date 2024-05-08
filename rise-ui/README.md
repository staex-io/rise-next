# UI

## Usage

```shell
make run # to just run
# or
make run_docker # to run in docker container
```

UI should be available at `http://localhost:5173`. 

## .env

Everything except agreement contract address and RPC URL can be empty.

### Local

```shell
VITE_DID_CONTRACT_ADDRESS=0x5FbDB2315678afecb367f032d93F642f64180aa3
VITE_AGREEMENT_CONTRACT_ADDRESS=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
VITE_RPC_URL=http://localhost:8545/
```

### Sepolia Testnet

```shell
VITE_DID_CONTRACT_ADDRESS=0x17536460b997842f8396409514986905eF63b58E
VITE_AGREEMENT_CONTRACT_ADDRESS=0x94a71B1940741145454Bb7AA490A66b86369F160
VITE_RPC_URL=https://ethereum-sepolia.publicnode.com
```

### Lisk Sepolia Testnet

```shell
VITE_DID_CONTRACT_ADDRESS=0xb513e687f5d72C25e3B75e2F59eD1De89806CA3C
VITE_AGREEMENT_CONTRACT_ADDRESS=0x5A0De82C2aea42f18F103bd81Fb7189A2adF5e06
VITE_RPC_URL=https://rpc.sepolia-api.lisk.com
```

## Backup

For local development, testing or demo you can upload [backup](../rise-backup.json).
