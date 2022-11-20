.ONESHELL:
# @see http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.DEFAULT_GOAL := help
SHELL := /bin/bash
DIR:=$(strip $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST)))))

DOCKER := podman
export POSTGRES_USER := user
export POSTGRES_PASSWORD := password
export POSTGRES_DB := rust-sample

.PHONY: help
help: ## provides cli help for this makefile (default) 📖
	@grep -E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test: ## run unit tests✓
	export DATABASE_URL=postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:5432/$(POSTGRES_DB)
	cargo test

.PHONY: build
build: ## build app ✓
	cargo build

.PHONY: release
release: ## release app ✓
	cargo build --release

.PHONY: start-release
start-release: ## start app for prod ✓
	export DATABASE_URL=postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:5432/$(POSTGRES_DB)
	$(DIR)/target/release/todo

.PHONY: clean
clean: ## clean compiled code ✓
	cargo clean

.PHONY: start-db
start-db: ## start the docker database
	$(DOCKER) run -d \
		--name rust-postgres \
		-e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
		-e POSTGRES_USER=$(POSTGRES_USER) \
		-e POSTGRES_DB=$(POSTGRES_DB) \
		-p 5432:5432 \
		postgres:14.1


.PHONY: init-db
init-db: ## init the docker database
	$(DOCKER) exec -it rust-postgres psql -U $(POSTGRES_USER) $(POSTGRES_DB) -c "CREATE TABLE task(id varchar, title varchar, description varchar, datetime timestamptz);"

.PHONY: stop-db
stop-db: ## stop the docker database and remove it
	$(DOCKER) stop rust-postgres
	$(DOCKER) rm rust-postgres

.PHONY: access-db
access-db: ## access to database shell
	$(DOCKER) exec -it rust-postgres psql -U $(POSTGRES_USER) $(POSTGRES_DB)
