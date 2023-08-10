rust-version:
	@echo "Rust command line tools version:"
	rustc --version
	cargo --version
	rustfmt --version
	rustup --version
	clippy-driver --version
format:
	@echo "Formatting code..."
	cargo fmt
lint:
	@echo "Linting code..."
	cargo clippy
test:
	@echo "Testing code..."
	cargo test
run:
	@echo "Running code..."
	cargo run
release:
	@echo "Building release..."
	cargo build --release
all:format lint test run 