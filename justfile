# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

default:
	@just build

lint:
	cargo clippy --all-targets --all-features -- -W clippy::pedantic

upgrade:
	cargo upgrade --workspace

test:
	cargo test

build:
	cargo build --release
	strip target/release/rmenu
	strip target/release/rmenu_history

install: test build
	cp target/release/rmenu /opt/homebrew/bin
	cp target/release/rmenu_history /opt/homebrew/bin
	cp scripts/rmenu_launch /opt/homebrew/bin

uninstall:
	rm /opt/homebrew/bin/rmenu
	rm /opt/homebrew/bin/rmenu_history
	rm /opt/homebrew/bin/rmenu_launch
