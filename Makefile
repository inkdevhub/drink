.PHONY: run build lint clean help

run: ## Run the project
	cargo run --release

build: ## Build the project
	cargo build --release

lint: ## Run the linter
	cargo fmt
	cargo clippy --release -- -D warnings

clean: ## Clean all the workspace build files
	cargo clean

help: ## Displays this help
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[1;36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[1;36m%-25s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
