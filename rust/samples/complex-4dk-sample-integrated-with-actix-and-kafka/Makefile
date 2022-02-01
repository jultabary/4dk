.ONESHELL:
# @see http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.DEFAULT_GOAL := help
SHELL := /bin/bash
DIR:=$(strip $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST)))))

export DATABASE_URL := postgres://user:password@localhost:5432/rust-sample
export KAFKA_URL := localhost:9092
export KAFKA_GROUP_ID := my_group_id

.PHONY: help
help: ## provides cli help for this makefile (default) 📖
	@grep -E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: test-unit
test-unit: ## run unit tests✓
	cargo test -- --skip integration_test

.PHONY: test-integration
test-integration: ## run integration tests✓
	make start-infra
	sleep 1
	make init-db
	cargo test -- '_integration_test'
	make stop-infra

.PHONY: test
test: ## run all tests✓
	make start-infra
	sleep 1
	make init-db
	cargo test
	make stop-infra

.PHONY: build
build: ## build app ✓
	cargo build

.PHONY: start-dev
start-dev: ## start for dev
	cargo run

.PHONY: release
release: ## release app ✓
	cargo build --release

.PHONY: start-release
start-release: ## start app for prod ✓
	$(DIR)/target/release/complex_4dk_sample

.PHONY: clean
clean: ## clean compiled code ✓
	cargo clean

.PHONY: start-infra
start-infra: ## start the docker database
	docker compose up -d

.PHONY: stop-infra
stop-infra: ## stop the docker database + kafka broker
	docker compose down

.PHONY: access-db
access-db: ## access to database shell
	docker exec -it rust-postgres /bin/bash

.PHONY: install-diesel-cli
install-diesel-cli: ## will install diesel-cli
	cargo install diesel_cli --no-default-features --features postgres

.PHONY: setup-db
setup-db: ## setup database with diesel
	diesel setup

.PHONY: init-db
init-db: ## init database with diesel
	diesel migration run
