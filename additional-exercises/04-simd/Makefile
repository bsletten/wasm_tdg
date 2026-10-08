WASM = target/wasm32-unknown-unknown/release/simd.wasm

# simd128 is not on by default for wasm32-unknown-unknown.
$(WASM): src/lib.rs Cargo.toml
	RUSTFLAGS="-C target-feature=+simd128" \
	    cargo build --release --target wasm32-unknown-unknown

.PHONY: run disasm clean
run: $(WASM)
	node run.mjs

# Proof that real v128 instructions came out the other end.
disasm: $(WASM)
	wasm-tools print $(WASM) | grep -oE 'f32x4\.[a-z_]+|v128\.[a-z]+' | sort | uniq -c

clean:
	cargo clean
