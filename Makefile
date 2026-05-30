.PHONY: build install test test-worker deploy cli worker help

# Default target
help:
	@echo "Last.fm Proxy Worker - Available targets:"
	@echo "  make build      - Build both worker and CLI"
	@echo "  make worker     - Build only the worker"
	@echo "  make cli        - Build only the CLI"
	@echo "  make install    - Install the CLI tool locally"
	@echo "  make test       - Run unit tests"
	@echo "  make test-worker - Smoke-test a running worker (WORKER_URL, default localhost:8787)"
	@echo "  make deploy     - Deploy worker to Cloudflare"
	@echo "  make dev        - Run worker in development mode"
	@echo "  make logs       - Tail worker logs"
	@echo "  make clean      - Clean build artifacts"

# Build everything
build: worker cli

# Build worker
worker:
	@echo "Building Cloudflare Worker..."
	@cargo build --lib --release

# Build CLI
cli:
	@echo "Building CLI tool..."
	@cargo build --bin lastfm-cli --release

# Install CLI locally
install: cli
	@echo "Installing lastfm-cli..."
	@cargo install --path . --bin lastfm-cli
	@echo "CLI installed! Run 'lastfm-cli --help' to get started"

# Run tests
test:
	@echo "Running unit tests..."
	@cargo test

# Smoke-test a running worker (set WORKER_URL, defaults to http://localhost:8787)
test-worker:
	@./scripts/test_worker.sh

# Deploy to Cloudflare
deploy: worker
	@echo "Deploying to Cloudflare Workers..."
	@wrangler deploy

# Development mode
dev:
	@echo "Starting development server..."
	@wrangler dev

# View logs
logs:
	@echo "Tailing worker logs..."
	@wrangler tail

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf build/
	@rm -rf dist/

# Quick commands for CLI
cli-artist:
	@cargo run --bin lastfm-cli -- artist info "$(ARTIST)"

cli-track:
	@cargo run --bin lastfm-cli -- track search "$(TRACK)"

cli-interactive:
	@cargo run --bin lastfm-cli -- interactive