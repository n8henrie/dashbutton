target/release/dashbutton: src/main.rs Cargo.toml
	cargo build --release

rootless: target/release/dashbutton
	@command -v setcap >/dev/null || { echo "ERROR: Must be on Linux with libcap installed to use without root permissions"; exit 1; }
	sudo setcap cap_net_raw,cap_net_admin=eip $<

.PHONY: rootless
