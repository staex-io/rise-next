lint:
	cargo clippy --tests --workspace -- -D warnings

run:
	cargo run

test:
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)
