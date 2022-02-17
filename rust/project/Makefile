.ONESHELL:
SHELL := /bin/bash
# @see http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.DEFAULT_GOAL := help
DIR:=$(strip $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST)))))
CORE_RUST_DIR:=$(DIR)/core-rust
SECURITY_RUST_DIR := $(DIR)/security-rust
MACRO_RUST_DIR := $(DIR)/fourdk-macro
MACRO_TEST_RUST_DIR := $(DIR)/fourdk-macro-test

.PHONY: help
help: ## provides cli help for this makefile (default) 📖
	@grep -E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test: ## run unit tests✓
	cd $(CORE_RUST_DIR)
	cargo test
	cd -
	cd $(SECURITY_RUST_DIR)
	cargo test	
	cd -
	cd $(MACRO_TEST_RUST_DIR)
	cargo test

.PHONY: release
release: ## release app ✓
	cd $(MACRO_RUST_DIR)
	cargo build --release
	cd -
	cd $(CORE_RUST_DIR)
	cargo build --release
	cd -
	cd $(SECURITY_RUST_DIR)
	cargo build --release

.PHONY: package-and-publish-macro
package-and-publish-macro: ## package and publish macro crate ✓
	cd $(MACRO_RUST_DIR)
	cargo test
	cargo build --release
	cargo package
	cargo publish	

.PHONY: package-and-publish-core
package-and-publish-core: ## package and publish core crate ✓
	cd $(CORE_RUST_DIR)
	cargo test
	cargo build --release
	cargo package
	cargo publish

.PHONY: package-and-publish-security
package-and-publish-security: ## package and publish security crate ✓
	cd $(SECURITY_RUST_DIR)
	cargo test
	cargo build --release
	cargo package
	cargo publish
