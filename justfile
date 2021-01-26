# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

default:
	@just build

lint:
	cargo clippy --all-targets --all-features -- -W clippy:pedantic

build:
	cargo build --release
	strip target/release/rmenu
	strip target/release/history
