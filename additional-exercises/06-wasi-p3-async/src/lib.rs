//! WASI Preview 3: async in the component model.
//!
//! Preview 2 (see `01-wasi-p2-cli`) gave us typed, versioned interfaces, but
//! I/O was still fundamentally blocking: you got a `pollable` and waited on
//! it. Preview 3 makes `async` part of the component model itself. Look at
//! the signature below -- `async fn run()`. That `async` is not a Rust
//! convention being compiled away; it is in the WIT contract, and the host
//! and guest cooperate on it through the component model's native streams
//! and futures.
//!
//! The practical consequence is that a component can have concurrency
//! without threads, without a reactor compiled into the guest, and
//! interoperably across languages.
//!
//! Preview 3 is genuinely new, and the toolchain shows it: there is no
//! `wasm32-wasip3` Rust target yet, so this builds for `wasm32-wasip2` while
//! the `wasip3` crate's bindings supply the `@0.3.0` interfaces. Confirm with
//! `wasm-tools component wit` -- the exported world is `wasi:cli/run@0.3.0`
//! and the function signature really is `run: async func() -> result`.
//!
//! Recent `wasmtime` releases enable the async ABI by default; older ones
//! need `wasmtime run -W component-model-async`.

wasip3::cli::command::export!(Example);

struct Example;

impl wasip3::exports::cli::run::Guest for Example {
    async fn run() -> Result<(), ()> {
        // A component-model *stream*, not a Rust channel: `rx` is handed to
        // the host, which reads from it while we are still writing to `tx`.
        let (mut tx, rx) = wasip3::wit_stream::new();

        futures::join!(
            // The host drains the stream to stdout...
            async {
                wasip3::cli::stdout::write_via_stream(rx).await.unwrap();
            },
            // ...concurrently with us filling it.
            async {
                for line in [
                    "WASI Preview 3: async is part of the component model.\n",
                    "\n",
                    "These writes are concurrent with the host draining them;\n",
                    "there is no thread and no poll loop in this guest.\n",
                ] {
                    let remaining = tx.write_all(line.as_bytes().to_vec()).await;
                    assert!(remaining.is_empty());
                }
                drop(tx);
            }
        );

        Ok(())
    }
}
