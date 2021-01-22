.PHONY: release

release:
	cargo build --release
	strip target/release/rmenu
	strip target/release/history
