WASM = target/wasm32-wasip2/release/wasi_http.wasm

$(WASM): src/lib.rs Cargo.toml
	cargo build --release --target wasm32-wasip2

.PHONY: serve wit clean
serve: $(WASM)
	wasmtime serve -S cli $(WASM)

wit: $(WASM)
	wasm-tools component wit $(WASM)

clean:
	cargo clean
