.PHONY: release

release:
	cargo build --release
	strip target/release/rmenu
	strip target/release/sort_hist
	strip target/release/update_hist

