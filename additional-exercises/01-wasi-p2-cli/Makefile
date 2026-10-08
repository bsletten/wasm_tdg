WASM = target/wasm32-wasip2/release/wasi-p2-cli.wasm

$(WASM): src/main.rs Cargo.toml
	cargo build --release --target wasm32-wasip2

.PHONY: run run-granted wit clean
run: $(WASM)
	wasmtime run $(WASM)

run-granted: $(WASM)
	wasmtime run --env WASM_TDG=hello --dir . $(WASM) one two

wit: $(WASM)
	wasm-tools component wit $(WASM)

clean:
	cargo clean
