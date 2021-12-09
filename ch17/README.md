## Chapter 17 : WebAssembly and Other Languages

Many of the languages discussed in this chapter have browser-based
playgrounds which make it easy to experiment with their connection to
WebAssembly. I will add more examples to this directory over time, but
for now the Zig directory has a couple of examples that are easy to run.

The `howOld.zig` file can be built and tested with Node.js using the following:

```
> zig build-lib howOld.zig -target wasm32-freestanding -dynamic
> node main.js
You are: 21
```

The WASI-based example in `preopens.zig` can be built and executed
from a WASI environment as follows:

```
> zig build-exe preopens.zig -target wasm32-wasi
> wasmtime preopens.wasm
<Nothing Happens>
> wasmtime --dir=. preopens.wasm
0: Preopen{ .fd = 3, .type = PreopenType{ .Dir = '@"."' } }
> wasmer --dir=. preopens.wasm
0: Preopen{ .fd = 3, .type = PreopenType{ .Dir = '@"/\x00"' } }
1: Preopen{ .fd = 4, .type = PreopenType{ .Dir = '@".\x00"' } }
```
