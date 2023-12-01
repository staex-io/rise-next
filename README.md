# RISE-next

## Usage

Setup .env file in project root.

```
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
RPC_URL=http://127.0.0.1:8545
```

```shell
make anvil
make deploy
cd agent && cargo run
```

## Dev Setup

`.vscode/settings.json`:

```
{
  "solidity.packageDefaultDependenciesContractsDirectory": "contracts/src",
  "solidity.packageDefaultDependenciesDirectory": "contracts/lib",
  "solidity.formatter": "forge"
}
```
