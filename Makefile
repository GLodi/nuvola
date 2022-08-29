.PHONY: setup
setup:
	cargo upgrade --workspace --pinned --to-lockfile
	cargo update

.PHONY: test
test:
	cargo test --workspace

.PHONY: check
check:
	cargo check --workspace
	cargo fmt --all -- --check
	cargo clippy -- -W warnings

.PHONY: clean
clean:
	cargo clean

.PHONY: serve
serve:
	cargo run -- start --log trace --user root --pass root memory

.PHONY: quick
quick:
	cargo build

.PHONY: build
build:
	cargo build --release

.PHONY: client
client:
	cargo run --bin client

.PHONY: server
server:
	cargo run --bin server
