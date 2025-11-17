.PHONY: build test fmt lint clean install all

all: build

build:
	cargo build --release

test:
	cargo test
	tabula-test

fmt:
	cargo fmt
	tabula fmt --write

lint:
	cargo clippy
	tabula-lint

clean:
	cargo clean
	rm -rf target/

install:
	cargo install --path compiler
	cargo install --path lsp
	cargo install --path tabpm
	cargo install --path repl
	cargo install --path test-framework
	cargo install --path debugger
	cargo install --path linter
	cargo install --path profiler
	cargo install --path docgen

# Development tools
repl:
	cargo run --bin tabula-repl

lsp:
	cargo run --bin tabula-lsp

# Examples
examples:
	@for file in examples/*.tab; do \
		echo "Running $$file"; \
		cargo run --bin tabula -- run -i $$file || true; \
	done

# Documentation
docs:
	tabula-doc --format html --output docs/html
	tabula-doc --format markdown --output docs/markdown

# Benchmark
bench:
	cargo bench

