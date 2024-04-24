
ci: fmt clippy

clippy:
	cargo clippy --all-targets --all-features

fmt:
	cargo fmt -- --check

run:
	cargo run --release
