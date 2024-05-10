fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

build:
	cargo build
	{ echo '#![allow(warnings)]'; cat src/data_proving_contract.rs; } > src/data_proving_contract.rs_
	rm -rf src/data_proving_contract.rs
	mv src/data_proving_contract.rs_ src/data_proving_contract.rs

build_rtmp:
	docker build -t nginx-rtmp .

run_rtmp: build_rtmp
	docker run -d \
		-p 1935:1935 \
		-p 8090:8090 \
		-v "${PWD}/rec:/tmp/rec" \
		-v "${PWD}/static:/tmp/static" \
		--name nginx-rtmp \
		nginx-rtmp

start_streaming:
	./stream.sh location=$(location)
