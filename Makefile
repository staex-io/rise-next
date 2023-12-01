#!make
include .env

build:
	cd contracts && forge build

test: lint
	cd contracts && forge test --gas-report --summary --detailed -vv

.PHONY: coverage
coverage: test
	cd contracts && forge coverage --report summary

lint:
	cd contracts && forge fmt

anvil:
	anvil --block-time 1

deploy:
	cd contracts && PRIVATE_KEY=${PRIVATE_KEY} \
		forge script script/GroundCycle.s.sol:GroundCycleScript \
		--fork-url ${RPC_URL} --broadcast -vvvv
