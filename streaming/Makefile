build_rtmp:
	docker build -t nginx-rtmp .

run_rtmp:
	docker run -d \
		-p 1935:1935 \
		-v "${PWD}/rec:/tmp/rec" \
		--name nginx-rtmp \
		nginx-rtmp

start_streaming:
	./stream.sh location=$(location)

run_cutter:
	cargo run
