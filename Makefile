build:
	cd contracts && forge build

test: lint
	cd contracts && \
		forge test --gas-report --summary --detailed -vv --fail-fast

.PHONY: coverage
coverage: test
	cd contracts && forge coverage --report summary

lint:
	cd contracts && forge fmt
