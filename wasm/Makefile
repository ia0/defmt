TARGET = wasm32-unknown-unknown
MODULE = target/$(TARGET)/release/wasm.wasm
TOOLCHAIN = nightly-2023-03-28-x86_64-unknown-linux-gnu

.PHONY: run
run:
	cargo +$(TOOLCHAIN) build --release --target=$(TARGET)
	cargo +$(TOOLCHAIN) build --release --manifest-path=../print/Cargo.toml
	wasm-interp $(MODULE) --dummy-import-func --run-all-exports \
	  | sed -n 's/^.*env\.print(i32:\(.*\)) =>/\1/p' \
	  | while read x; do printf '%02x' $$x; done | xxd -p -r \
	  | ../target/release/defmt-print -e $(MODULE)
