.ONESHELL:
# @see http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.DEFAULT_GOAL := help
SHELL := /bin/bash

.PHONY: help
help: ## provides cli help for this makefile (default) 📖
	@grep -E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: start
start: ## start app ✓
	export DATABASE_URL=postgres://user:password@localhost:5432/diesel_demo
	docker run -d \
		--name rust-postgres \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_USER=user \
		-e POSTGRES_DB=rust-sample \
		-v ./resources/database:/docker-entrypoint-initdb.d \
		postgres:14.1
	cargo run

.PHONY: build
build: ## build app ✓
	cargo build

.PHONY: clean
clean: ## clean compiled code ✓
	cargo clean
