# 06 : WASI Preview 3 — async in the Component Model

Preview 2 (exercise 01) gave typed, versioned interfaces, but I/O was still
fundamentally blocking: you obtained a `pollable` and waited on it. Preview 3
puts `async` into the component model itself.

```wit
interface run {
  run: async func() -> result;
}
```

That `async` is not a Rust convention being compiled away. It is in the WIT
contract, and host and guest cooperate on it using the component model's
native streams and futures. Confirm it on the built artifact:

```
> make wit
...
  interface run {
    run: async func() -> result;
  }
```

The guest in `src/lib.rs` writes into a component-model *stream* while the
host concurrently drains it to stdout — with no thread, no reactor and no
poll loop compiled into the guest:

```
> make run
WASI Preview 3: async is part of the component model.

These writes are concurrent with the host draining them;
there is no thread and no poll loop in this guest.
```

## Caveat: this is the bleeding edge

Preview 3 is new enough that the toolchain shows the seams.

* **There is no `wasm32-wasip3` Rust target.** This builds for
  `wasm32-wasip2`, and the [`wasip3`](https://crates.io/crates/wasip3) crate
  supplies the `@0.3.0` interface bindings. The resulting component genuinely
  imports and exports `@0.3.0` interfaces — check with `make wit` — but the
  build command still says `wasip2`.
* **Runtime support is recent.** `wasmtime` 48 runs this as-is; earlier
  releases need `wasmtime run -W component-model-async`. The `Makefile`
  passes the flag either way, since it is harmless when the feature is
  already on.

Compare with `ch12/`, which was written when reference types and multi-value
returns were the frontier. The frontier moved.
