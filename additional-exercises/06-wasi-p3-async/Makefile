WASM = target/wasm32-wasip2/release/wasi_p3_async.wasm

# There is no wasm32-wasip3 Rust target yet; the wasip3 crate supplies the
# @0.3.0 interfaces on top of the wasip2 target.
$(WASM): src/lib.rs Cargo.toml
	cargo build --release --target wasm32-wasip2

.PHONY: run wit clean
run: $(WASM)
	wasmtime run -W component-model-async $(WASM)

wit: $(WASM)
	wasm-tools component wit $(WASM)

clean:
	cargo clean
