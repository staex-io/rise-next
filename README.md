# RISE-next

## Usage

Setup .env file in project root folder to make Makefile target easier to user.


For local development:

```shell
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
RPC_URL=http://127.0.0.1:8545
```

For Sepolia development:

```shell
PRIVATE_KEY=0xdc42ff0eb5193790f229744ccca9ba33ebc136085d2b19fd38300bcb2e96a7f2
RPC_URL=https://ethereum-sepolia.publicnode.com
```

After that execute commands in order:

```shell
make anvil
make deploy landing_wait_time=300
cd agent && make run
```

## Generate binding for Rust

```shell
make build
```

Sometime it is better to delete `contracts/out` and `agent/src/contracts` to remove old files to generate only actual contracts.
It happens when you rename some contract or you stopped to use some smart contract.

## Create new wallets

```shell
cast wallet new
```

### Send some tokens to created wallet

```shell
source .env
cast send --value 0.1ether --private-key ${PRIVATE_KEY} %address% --rpc-url ${RPC_URL}
cast balance %address% --rpc-url ${RPC_URL}
cast to-unit 19903268211300622 ether
```

## Faucet

As now we use Sepolia testnet you can use this faucet for test tokens: https://sepolia-faucet.pk910.de/#/mine/

## Dev setup

`.vscode/settings.json`:

```json
{
  "solidity.packageDefaultDependenciesContractsDirectory": "contracts/src",
  "solidity.packageDefaultDependenciesDirectory": "contracts/lib",
  "solidity.formatter": "forge"
}
```

// todo: update scheme plantuml
// todo: update scheme plantuml
// todo: update scheme plantuml
// todo: update scheme plantuml
// todo: update scheme plantuml
// todo: update scheme plantuml

// todo: write tests for solidity rejection
