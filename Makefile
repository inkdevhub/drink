.PHONY: run build lint test_examples clean help

EXAMPLES = ./examples
EXAMPLES_PATHS := $(shell find $(EXAMPLES) -mindepth 1 -maxdepth 2 -name Cargo.toml -exec dirname "{}" \;)
EXAMPLES_TARGET = ./examples/target

run: ## Run the project
	cargo run --release

build: ## Build the project
	cargo build --release -j 12

lint: ## Run the linter
	cargo +nightly fmt
	cargo clippy --release -- -D warnings

test_examples: ## Run tests for the examples
	@mkdir -p $(EXAMPLES_TARGET)
	@for dir in $(EXAMPLES_PATHS); do \
		echo "Processing $$dir"; \
		cargo test --quiet --manifest-path $$dir/Cargo.toml --release --target-dir $(EXAMPLES_TARGET) || exit 1; \
	done

clean: ## Clean all the workspace build files
	cargo clean

help: ## Displays this help
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[1;36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[1;36m%-25s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
