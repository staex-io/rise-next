fmt:
	@# We need nightly version to ignore contracts generated code.
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --workspace -- -D warnings

run:
	cargo run

test:
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)
