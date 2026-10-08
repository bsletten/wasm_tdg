# Additional Exercises : WebAssembly Today

The chapter directories in this repository stay faithful to the book. This
directory does not: it is a set of new, self-contained examples showing where
WebAssembly actually got to, built and run against current toolchains.

The book closed several chapters with a version of "this is a proposal, and
support is thin." Most of those proposals landed. The examples here are
organised roughly in the order the ideas build on each other.

| | Example | What it shows | Was it in the book? |
|---|---|---|---|
| 01 | [`01-wasi-p2-cli`](01-wasi-p2-cli) | WASI Preview 2 as a **component** with typed, versioned interfaces | Ch. 11 covered Preview 1 core modules |
| 02 | [`02-wasi-http`](02-wasi-http) | `wasi:http/proxy` — a server with no server in it | No |
| 03 | [`03-component-composition`](03-component-composition) | Linking two components through a WIT interface with `wac` | No |
| 04 | [`04-simd`](04-simd) | Fixed-width SIMD (`v128`), with a measured speedup | Mentioned as a future proposal |
| 05 | [`05-threads`](05-threads) | Real pthreads over `SharedArrayBuffer` | Mentioned as a future proposal |
| 06 | [`06-wasi-p3-async`](06-wasi-p3-async) | WASI Preview 3: `async` in the component model itself | No — did not exist |

Every directory has a `Makefile`. `make run` builds and runs (`make serve` for
02 and 05).

Two of these run in a browser and are hosted on the
[live example site](https://bsletten.github.io/wasm_tdg/):
[SIMD](https://bsletten.github.io/wasm_tdg/additional-exercises/04-simd/) and
[threads](https://bsletten.github.io/wasm_tdg/additional-exercises/05-threads/).
The rest are WASI components or a server, so they only make sense from a
terminal.

## The one big idea: core modules became components

If you read only one thing here, make it 01 and 03.

Chapters 2 through 7 of the book are about *core modules*: a flat list of
imports and exports over `i32`, `i64`, `f32` and `f64`, with strings and
structs hand-marshalled through linear memory. Chapter 4 spends real effort
getting a Japanese string out of memory by hand, and that work was not
incidental — it was the only option.

The **component model** is the answer to that. A component declares its
interface in WIT, with strings, lists, records, variants, options and results
as first-class types, and the toolchain generates the marshalling. Chapter 12's
discussion of module linking is the ancestor of this.

The payoff is visible in 03: `calculator` exports `how-old` and
`app` imports it, neither knows anything about the other, and `wac plug`
fuses them into one artifact. Nothing about that composition is
Rust-specific — a component in any language implementing the same WIT would
drop straight in.

## Toolchain

| Tool | Install | Used by |
|---|---|---|
| Rust + `wasm32-wasip2` | `rustup target add wasm32-wasip2` | 01, 02, 03, 06 |
| `wasm32-unknown-unknown` | `rustup target add wasm32-unknown-unknown` | 04 |
| `wasmtime` | <https://wasmtime.dev> | 01, 02, 03, 06 |
| `wasm-tools` | `cargo install wasm-tools` | inspection everywhere |
| `wac` | `cargo install wac-cli` | 03 |
| Emscripten | <https://emscripten.org> | 05 |
| Node.js | | 04, 05 |

There is a `rust-toolchain.toml` here pinning these projects to stable Rust.
That is deliberate: the `wasm32-wasip2` standard library bakes in specific
WASI interface versions (`wasi:io@0.2.12` and friends), and if your toolchain's
`std` disagrees with the `wasip2`/`wstd` crates, the link fails with

```
rust-lld: error: import module mismatch for symbol: [resource-drop]error
  >>> defined as wasi:io/error@0.2.12 in ...
  >>> defined as wasi:io/error@0.2.4 in ...libstd...
```

which is not an obvious error message for "your Rust is too old."

## Verified with

Everything here was built and run on macOS (Apple silicon) with:

```
rustc 1.99.0          wasmtime 48.0.2       wasm-tools 1.261.0
wac-cli 0.12.0        emcc 6.0.9            node v26.8.2
```

## A worked example in the wild

For these ideas assembled into something real, see the **ikigai resolution
kernel** demo linked from the [root README](../README.md#seeing-it-in-the-wild--the-ikigai-resolution-kernel):
<https://ikigai-rs.github.io/ikigai-web-demo/>. It is a resource-oriented
computing kernel running entirely in the browser, and it shows the lazy
module loading discussed above in production form -- its XSLT and JSON-LD
engines are fetched and instantiated only on first use.

## Further exercises

Things worth building that are not here yet, in rough order of how much they
would add:

* **`wasi:keyvalue` / `wasi:config`.** `wasmtime` already has experimental
  support (`-S keyvalue`, `-S config`). The interesting part is that a
  component written against them runs unchanged against an in-memory store
  locally and a real one in production.
* **A component in a second language.** Take the `calculator` component from
  03 and reimplement it in Go (TinyGo), C (wit-bindgen) or Python
  (componentize-py), then compose it with the *same* unmodified `app`. This
  is the single most convincing demonstration of what the component model is
  for.
* **The GC proposal.** Languages with managed runtimes (Java via TeaVM, Kotlin,
  Dart, OCaml) can now use the host's garbage collector instead of shipping
  their own in linear memory. Chapter 18's "Java in the browser" would look
  very different today.
* **JS string builtins.** Removes the UTF-8/UTF-16 copying at the JS boundary
  that Chapter 4 demonstrates by hand.
* **JSPI (JavaScript Promise Integration).** Lets synchronous WebAssembly code
  call into `async` JavaScript without the Asyncify rewriting Emscripten used
  to need. A good companion to Chapter 6's `hello-delay.html`.
* **Memory64.** A one-line change to the `.wat` in Chapter 4 (`i64` indices),
  and a good way to see how little of the model actually depends on 32-bit
  addresses.
* **Relaxed SIMD.** Trades bit-exact reproducibility for speed on operations
  like fused multiply-add. A natural follow-on to 04.
* **`wasi:http` outgoing requests.** 02 only handles inbound traffic; the same
  world lets a component *make* requests, which is how most edge functions
  earn their keep.
