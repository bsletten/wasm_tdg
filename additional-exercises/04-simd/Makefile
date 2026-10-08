WASM = target/wasm32-unknown-unknown/release/simd.wasm

# simd128 is not on by default for wasm32-unknown-unknown.
$(WASM): src/lib.rs Cargo.toml
	RUSTFLAGS="-C target-feature=+simd128" \
	    cargo build --release --target wasm32-unknown-unknown

.PHONY: run page serve disasm clean
run: $(WASM)
	node run.mjs

# Copy the module next to index.html so the browser demo can fetch it.
simd.wasm: $(WASM)
	cp $(WASM) simd.wasm

page: simd.wasm

serve: simd.wasm
	python3 -m http.server 10002

# Proof that real v128 instructions came out the other end.
disasm: $(WASM)
	wasm-tools print $(WASM) | grep -oE 'f32x4\.[a-z_]+|v128\.[a-z]+' | sort | uniq -c

clean:
	rm -f simd.wasm
	cargo clean
