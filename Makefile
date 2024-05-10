#!make
include .env

build:
	cd contracts && forge build --use 0.8.22
	cd contracts/out && cp DID.sol/DIDContract.json ../../rise-ui/src/assets/DIDContract.json && \
		cp Agreement.sol/AgreementContract.json ../../rise-ui/src/assets/AgreementContract.json && \
		cp DataProving.sol/DataProvingContract.json ../../rise-ui/src/assets/DataProvingContract.json
	cd contracts/out && cp DID.sol/DIDContract.json ../../agent/assets/DIDContract.json && \
		cp Agreement.sol/AgreementContract.json ../../agent/assets/AgreementContract.json && \
		cp GroundCycle.sol/GroundCycleContract.json ../../agent/assets/GroundCycleContract.json && \
		cp GroundCycleNoCrypto.sol/GroundCycleNoCryptoContract.json ../../agent/assets/GroundCycleNoCryptoContract.json && \
		cp DataProving.sol/DataProvingContract.json ../../streaming/assets/DataProvingContract.json

test: lint
	cd contracts && forge test --use 0.8.22 --gas-report --summary --detailed -vv

coverage: test
	cd contracts && forge coverage --use 0.8.22 --report debug

lint:
	cd contracts && forge fmt

anvil:
	anvil --block-time 1

.PHONY: deploy
deploy:
	cd contracts && PRIVATE_KEY=${PRIVATE_KEY} LANDING_WAIT_TIME=$(LANDING_WAIT_TIME) IS_TESTING=true \
		forge script --use 0.8.22 script/Rise.s.sol:RiseScript \
		--fork-url ${RPC_URL} --broadcast -vvvv

deploy_data_proving:
	cd contracts && PRIVATE_KEY=${PRIVATE_KEY} \
		forge script --use 0.8.22 script/DataProving.s.sol:DataProvingScript \
		--fork-url ${RPC_URL} --broadcast -vvvv

build_agent:
	docker buildx build \
		--platform linux/amd64 \
		-t ghcr.io/staex-io/rise-next/agent:$(shell date +%d%m%Y%H%M) \
		-f deploy/Dockerfile \
		--push \
		.
