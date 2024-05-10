fmt:
	@# We need nightly version to ignore contracts generated code.
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

build:
	cargo build
	
	# Remove clippy warnings from did smart contract generated source code.
	{ echo '#![allow(warnings)]'; cat src/did_contract.rs; } > src/did_contract.rs_
	rm -rf src/did_contract.rs
	mv src/did_contract.rs_ src/did_contract.rs
	
	# Remove clippy warnings from agreement smart contract generated source code.
	{ echo '#![allow(warnings)]'; cat src/agreement_contract.rs; } > src/agreement_contract.rs_
	rm -rf src/agreement_contract.rs
	mv src/agreement_contract.rs_ src/agreement_contract.rs
	
	# Remove clippy warnings from ground cycle smart contract generated source code.
	{ echo '#![allow(warnings)]'; cat src/ground_cycle_contract.rs; } > src/ground_cycle_contract.rs_
	rm -rf src/ground_cycle_contract.rs
	mv src/ground_cycle_contract.rs_ src/ground_cycle_contract.rs
	
	# Remove clippy warnings from ground cycle no crypto smart contract generated source code.
	{ echo '#![allow(warnings)]'; cat src/ground_cycle_no_crypto_contract.rs; } > src/ground_cycle_no_crypto_contract.rs_
	rm -rf src/ground_cycle_no_crypto_contract.rs
	mv src/ground_cycle_no_crypto_contract.rs_ src/ground_cycle_no_crypto_contract.rs

run:
	cargo run

test:
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)

sql_format:
	docker run --rm \
		-v "${PWD}/migrations:/migrations" \
		-v "${PWD}/scripts/sql_format.sh:/sql_format.sh" \
		--entrypoint="" \
		node:20-alpine ./sql_format.sh
