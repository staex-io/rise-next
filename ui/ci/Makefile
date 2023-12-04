IMAGE := ghcr.io/staex-mcc/rise-ui:latest

rise-ui:
	docker build --tag $(IMAGE) --file Dockerfile ..
	docker push $(IMAGE)
