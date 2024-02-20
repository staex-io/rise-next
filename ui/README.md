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
VITE_DID_CONTRACT_ADDRESS=0x3bA4B1e2a1c775267e7b6288A8D66c411A56C8c3
VITE_AGREEMENT_CONTRACT_ADDRESS=0x6beFEd6d4D0e4a9198266EAdf295F5C1eD78C3c7
VITE_RPC_URL=https://rpc.sepolia-api.lisk.com
```

## Backup

For local development, testing or demo you can upload [backup](../rise-backup.json).

## Vue 3 + Vite

This template should help get you started developing with Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).
