install_path := "/opt/homebrew/bin"
build_path := "./target/release"
script_path := "./scripts"
binaries := `cat Cargo.toml | grep "members" | sed 's/.*\[\(.*\)\]/\1/' | tr -d ",\""`

default:
	@just build

lint:
	cargo clippy --all-targets --all-features -- -W clippy::pedantic

upgrade:
	cargo update
	cargo upgrade

test:
	cargo test

build:
	cargo build --release

install: test build
	for binary in {{binaries}}; do cp {{build_path}}/$binary {{install_path}}; done
	for script in `ls {{script_path}}`; do cp {{script_path}}/$script {{install_path}}; done

uninstall:
	for binary in {{binaries}}; do rm {{install_path}}/$binary; done
	for script in `ls {{script_path}}`; do rm {{install_path}}/$script; done
