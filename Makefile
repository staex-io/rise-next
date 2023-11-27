build:
	cd contracts && forge build

test: lint
	cd contracts && forge test --gas-report --summary --detailed -vv

lint:
	cd contracts && forge fmt
