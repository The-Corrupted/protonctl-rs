build:
	cargo build --release
install:
	sudo mv target/release/protonctl-rs /usr/bin/protonctl
clean:
	cargo clean
build_and_install: build install

