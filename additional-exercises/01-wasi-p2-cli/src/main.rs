//! A WASI Preview 2 "command" component.
//!
//! Chapter 11 built Preview 1 *core modules*: a single `.wasm` file that
//! imported individual functions like `wasi_snapshot_preview1::fd_write`.
//! Preview 2 replaces that flat list of imports with the component model:
//! the binary is a *component* that imports typed, versioned interfaces such
//! as `wasi:cli/environment` and `wasi:filesystem/types`.
//!
//! The striking part is how little the Rust source has to change. The
//! standard library knows how to target WASI P2, so this is ordinary Rust;
//! what changes is the compilation target and the shape of the artifact.

use std::env;
use std::time::SystemTime;

fn main() {
    println!("--- wasi:cli/environment ---");

    // Arguments arrive through wasi:cli/environment, not argv.
    let args: Vec<String> = env::args().collect();
    println!("args ({}): {:?}", args.len(), args);

    // Environment variables are only the ones the host chose to pass.
    match env::var("WASM_TDG") {
        Ok(v) => println!("WASM_TDG = {v}"),
        Err(_) => println!("WASM_TDG is not set (try --env WASM_TDG=hello)"),
    }

    println!("\n--- wasi:clocks/wall-clock ---");
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => println!("seconds since the epoch: {}", d.as_secs()),
        Err(e) => println!("clock error: {e}"),
    }

    println!("\n--- wasi:filesystem/types ---");
    // Still capability-based: without a --dir there is nothing to read.
    match std::fs::read_dir(".") {
        Ok(entries) => {
            let mut names: Vec<String> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect();
            names.sort();
            println!("entries in '.': {names:?}");
        }
        Err(e) => println!("cannot read '.': {e} (pass --dir . to grant access)"),
    }

    println!("\n--- wasi:cli/exit ---");
    println!("returning 0");
}
