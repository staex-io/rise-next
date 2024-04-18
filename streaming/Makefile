build_rtmp:
	docker build -t nginx-rtmp .

run_rtmp:
	docker run -d \
		-p 1935:1935 \
		-v "${PWD}/rec:/tmp/rec" \
		-v "${PWD}/hls:/tmp/hls" \
		--name nginx-rtmp \
		nginx-rtmp

start_streaming:
	./stream.sh location=$(location)
