# 03 : Composing Components

Chapter 12 discusses the module linking proposal, and `ch12/hello-modlink`
demonstrates its core-module form: two `.wat` files, one importing a function
and a memory from the other, wired together by a `wasmtime::Linker` in Rust
host code.

This is where that idea ended up. Two components, a WIT interface between
them, and a link step that needs no host code at all.

`calculator/wit/world.wit` is the entire contract:

```wit
interface ages {
    how-old: func(year-now: s32, year-born: s32) -> s32;
}
```

* `calculator` **exports** `ages` and imports nothing.
* `app` **imports** `ages` and exports `wasi:cli/run`.

Neither knows the other exists. On its own, `app` is unrunnable:

```
> make unsatisfied
Error: component imports instance `wasmtdg:calculator/ages@0.1.0`,
       but a matching implementation was not found in the linker
```

`wac plug` satisfies that import with the other component:

```
> make run
born in 2000, in 2026 you are 26
born in 1980, in 2026 you are 46
born in 2030? not yet!
```

`composed.wasm` is not committed -- `make` rebuilds it from the two component
crates, so it cannot go stale.

`make wit` on the result shows `wasmtdg:calculator/ages` is gone from the
import list — it was satisfied internally. The output is one self-contained
component.

**The exercise worth doing:** reimplement `calculator` in another language
that targets components (TinyGo, C with `wit-bindgen`, `componentize-py`) and
compose it with the *same* unmodified `app.wasm`. Nothing in `app` has to
change, because the contract is the WIT file, not an ABI.
