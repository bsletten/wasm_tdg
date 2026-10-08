## Chapter 17 : WebAssembly and Other Languages

Many of the languages discussed in this chapter have browser-based
playgrounds which make it easy to experiment with their connection to
WebAssembly. I will add more examples to this directory over time, but
for now the Zig directory has a couple of examples that are easy to run.

The `howOld.zig` file can be built and tested with Node.js using the following:

```
> zig build-exe howOld.zig -target wasm32-freestanding -fno-entry -rdynamic -OReleaseSmall
> node main.js
You are: 21
```

The book used `zig build-lib ... -dynamic` here. That no longer works:
current Zig versions fail the link with `relocation
R_WASM_MEMORY_ADDR_SLEB cannot be used against symbol ...; recompile with
-fPIC`, because `-dynamic` now means a genuine shared library. For a
freestanding module that just exports some functions, `build-exe` with
`-fno-entry` (no `_start`) and `-rdynamic` (export the public symbols) is
the modern equivalent. `-OReleaseSmall` is optional but takes the result
from roughly 650 KB down to under 100 bytes.

The WASI-based example in `preopens.zig` can be built and executed
from a WASI environment as follows:

```
> zig build-exe preopens.zig -target wasm32-wasi -OReleaseSmall
> wasmtime preopens.wasm
<Nothing Happens>
> wasmtime --dir=. preopens.wasm
0: Preopen{ .fd = 3, .type = DIR, .name = '.' }
> wasmer run --volume .:/ preopens.wasm
0: Preopen{ .fd = 3, .type = DIR, .name = '/' }
1: Preopen{ .fd = 4, .type = DIR, .name = '.' }
2: Preopen{ .fd = 5, .type = DIR, .name = '/' }
```

The point of the example is unchanged -- a WASI module sees only the
directories the host chose to hand it, which is why nothing happens until
you pass `--dir`/`--volume` -- but the source had to be rewritten. Zig's
standard library used to provide a `std.fs.wasi.PreopenList` helper, and
that type has been removed, so the example now walks the preopened file
descriptors directly with `fd_prestat_get`/`fd_prestat_dir_name` from
`std.os.wasi`. (Zig's `for` loop also changed: capturing an index is now
`for (slice, 0..) |item, i|`.) `wasmer`'s `--dir` flag is deprecated and
replaced by `--volume HOST:GUEST`.

One thing to know if you script this: `std.debug.print` writes to **stderr**,
not stdout, so `wasmtime --dir=. preopens.wasm | grep Preopen` finds nothing.
Redirect with `2>&1` first.
