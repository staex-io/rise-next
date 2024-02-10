fmt:
	@# We need nightly version to ignore contracts generated code.
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --workspace --all-targets --all-features -- -D warnings

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
