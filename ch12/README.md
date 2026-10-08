## Chapter 12 : Extending the WebAssembly Platform

Being able to detect which features a browser supports is important as
the proposals are adopted incrementally.

The `detector` directory features the
[wasm-feature-detect](https://github.com/GoogleChromeLabs/wasm-feature-detect)
library to advertise which features a browser supports. Simply launch
a web server and check out the JavaScript console to see the results --
the page now also renders them in a table.

This example has grown since the book was printed. When the chapter was
written the library could probe eleven proposals; it now covers
twenty-three, and the detector lists all of them. Things that were
speculative then -- threads, garbage collection, Memory64, JSPI, relaxed
SIMD, tail calls -- are detectable (and in many browsers, supported)
today. The `memory64` entry that was commented out in the printed example
is simply part of the list now.

The `hello-wasi`, `hello-mvr`, `hello-extref` and `hello-modlink`
directories feature native Rust applications that are dynamically
loading WebAssembly modules. Therefore they can be built and run as a
regular Rust applications.

```
> cargo build --release
> cargo run --release
You are 21
```

These four examples were written against `wasmtime` 0.28 and have been
migrated to a current release (44.x). `wasmtime` 0.28 no longer builds at
all on modern Rust toolchains -- `hello-modlink` in particular failed deep
inside an old transitive dependency. The API changes that mattered:

* `get_typed_func::<Params, Results, _>` lost its third type parameter and
  is now `get_typed_func::<Params, Results>`.
* `ExternRef::new(value)` became `ExternRef::new(&mut store, value)` and
  returns a `Rooted<ExternRef>`; reading it back is `eref.data(&store)?`.
  Table values are built with `Ref::Extern(...)` rather than `.into()`.
* `Config::wasm_reference_types(true)` is gone -- reference types are
  always enabled.
* `wasmtime_wasi::sync::WasiCtxBuilder` and
  `wasmtime_wasi::add_to_linker` moved to `wasmtime_wasi::WasiCtxBuilder`
  and `wasmtime_wasi::p1::add_to_linker_sync`, and the context is built
  with `.build_p1()`. `inherit_args()` no longer returns a `Result`.

The four are also grouped into a Cargo workspace (`ch12/Cargo.toml`) so they
share one `ch12/target/` and one `Cargo.lock`. Each embeds wasmtime, so built
separately they produced about 3.2 GB of near-identical output and four cold
compiles; together it is one. `cargo run --release` from inside any example
directory works exactly as described above -- it just reuses what the others
already built. Each example still pins its own dependency versions rather than
inheriting them, so any one directory can be copied out and built on its own.

See the `additional-exercises` directory for examples built on the newer
WASI Preview 2 / Preview 3 component model rather than the Preview 1
core-module interface used here.
