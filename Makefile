.PHONY: release strip all

release:
	cargo build --release

strip: release
	strip target/release/rmenu
	strip target/release/sort_hist
	strip target/release/update_hist

all: strip
