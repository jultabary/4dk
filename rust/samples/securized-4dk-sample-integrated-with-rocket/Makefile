.ONESHELL:
# @see http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.DEFAULT_GOAL := help
SHELL := /bin/bash
DIR:=$(strip $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST)))))

export DATABASE_URL := postgres://user:password@localhost:5432/rust-sample

.PHONY: help
help: ## provides cli help for this makefile (default) 📖
	@grep -E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test: ## run unit tests✓
	cargo test

.PHONY: build
build: ## build app ✓
	cargo build

.PHONY: release
release: ## release app ✓
	cargo build --release

.PHONY: start-release
start-release: ## start app for prod ✓
	$(DIR)/target/release/securized-4dk-sample-integrated-with-rust

.PHONY: clean
clean: ## clean compiled code ✓
	cargo clean

.PHONY: start-db
start-db: ## start the docker database
	docker run -d \
		--name rust-postgres \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_USER=user \
		-e POSTGRES_DB=rust-sample \
		-p 5432:5432 \
		-v $(DIR)/resources/database:/docker-entrypoint-initdb.d \
		postgres:14.1

.PHONY: stop-db
stop-db: ## stop the docker database and remove it
	docker stop rust-postgres
	docker rm rust-postgres

.PHONY: access-db
access-db: ## access to database shell
	docker exec -it rust-postgres /bin/bash

.PHONY: install-diesel-cli
install-diesel-cli: ## will install diesel-cli
	cargo install diesel_cli --no-default-features --features postgres

.PHONY: init-db
init-db: ## init database with diesel
	diesel migration run

.PHONY: setup-db
setup-db: ## init database with diesel
	diesel setup