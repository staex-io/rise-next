#!make
include .env

build:
	cd contracts && forge build
	cd contracts && forge bind --overwrite --skip-build --single-file --module -b ../agent/src/contracts
	{ echo '#![allow(warnings)]'; cat agent/src/contracts/mod.rs; } > agent/src/contracts/mod.rs_
	rm -rf agent/src/contracts/mod.rs
	mv agent/src/contracts/mod.rs_ agent/src/contracts/mod.rs
	cd contracts && cp out/Agreement.sol/AgreementContract.json ../ui/src/assets/AgreementContract.json

test: lint
	cd contracts && forge test --gas-report --summary --detailed -vv

coverage: test
	cd contracts && forge coverage --report summary

lint:
	cd contracts && forge fmt

anvil:
	anvil --block-time 1

deploy:
	cd contracts && PRIVATE_KEY=${PRIVATE_KEY} LANDING_WAIT_TIME=$(LANDING_WAIT_TIME) IS_TESTING=true \
		forge script script/GroundCycle.s.sol:GroundCycleScript \
		--fork-url ${RPC_URL} --broadcast -vvvv
