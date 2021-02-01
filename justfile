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

build:
	cargo build --release
	strip target/release/rmenu
	strip target/release/rmenu_history

install: build
	cp target/release/rmenu /usr/local/bin/
	cp target/release/rmenu_history /usr/local/bin/
	cp scripts/rmenu_launch /usr/local/bin/

uninstall:
	rm /usr/local/bin/rmenu
	rm /usr/local/bin/rmenu_history
	rm /usr/local/bin/rmenu_launch
