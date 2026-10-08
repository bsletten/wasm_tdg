APP  = app/target/wasm32-wasip2/release/app.wasm
CALC = calculator/target/wasm32-wasip2/release/calculator.wasm

composed.wasm: $(APP) $(CALC)
	wac plug $(APP) --plug $(CALC) -o composed.wasm

$(APP): app/src/main.rs app/wit/world.wit app/Cargo.toml
	cargo build --release --target wasm32-wasip2 --manifest-path app/Cargo.toml

$(CALC): calculator/src/lib.rs calculator/wit/world.wit calculator/Cargo.toml
	cargo build --release --target wasm32-wasip2 --manifest-path calculator/Cargo.toml

.PHONY: run unsatisfied wit clean
run: composed.wasm
	wasmtime run composed.wasm

# Demonstrates that the app alone is not runnable.
unsatisfied: $(APP)
	-wasmtime run $(APP)

wit: composed.wasm
	wasm-tools component wit composed.wasm

clean:
	rm -f composed.wasm
	cargo clean --manifest-path app/Cargo.toml
	cargo clean --manifest-path calculator/Cargo.toml
