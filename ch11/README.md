## Chapter 11 : WebAssembly System Interface (WASI)

WASI-based runtimes provide capabilities you wish to give to a bit of executable code.

[wasmtime](https://github.com/bytecodealliance/wasmtime) and
[wasmer](https://wasmer.io) are both WASI-enabled, and are the two
runtimes used throughout this chapter. The book also used
[wasm3](https://github.com/wasm3/wasm3), which is no longer maintained.

To run the `.wat` file, you should be able to just do:

```
> wasmtime hello.wat
hello world
> wasmer hello.wat
hello world
```

To build the standalone C file, the book used the
[wasienv](https://github.com/wasienv/wasienv) toolchain (`wasicc`), which
is no longer maintained. The [WASI SDK](https://github.com/WebAssembly/wasi-sdk)
is its replacement: download a release, point `WASI_SDK_PATH` at it, and
its bundled `clang` is preconfigured with the right target and sysroot.

```
> $WASI_SDK_PATH/bin/clang hello.c -o hello.wasm
> wasmtime hello.wasm
Hello, World!
> wasmer run hello.wasm
Hello, World!
```

If you have the WASI sysroot but are using your own `clang`, you need to
name the target and the sysroot yourself:

```
> clang --target=wasm32-wasip1 --sysroot=$WASI_SYSROOT hello.c -o hello.wasm
```

To run the `hello-world` project, the following should work:

As a standalone Rust application:

```
> cargo build --release
> cargo run --release
```

As a WASI application. Note that Rust's `wasm32-wasi` target was renamed
to `wasm32-wasip1` (for WASI Preview 1) and the old name no longer exists,
so install the target first with `rustup target add wasm32-wasip1`:

```
> cargo build --target wasm32-wasip1 --release
> wasmtime target/wasm32-wasip1/release/hello-world.wasm
Hello, world!
> wasmer run target/wasm32-wasip1/release/hello-world.wasm
Hello, world!
```

To run the `hello-fs` project, the following should work:

As a standalone Rust application:

```
> cargo build --release
> cargo run --release
Read in from file: Hello, world!
```

As a WASI application:

```
> cargo build --target wasm32-wasip1 --release
> wasmtime target/wasm32-wasip1/release/hello-fs.wasm
<FAILS>
> wasmtime --dir=. target/wasm32-wasip1/release/hello-fs.wasm
Read in from file: Hello, world!
> wasmer run target/wasm32-wasip1/release/hello-fs.wasm
<FAILS>
> wasmer run --volume .:/ target/wasm32-wasip1/release/hello-fs.wasm
Read in from file: Hello, world!
```

`wasmer`'s `--dir` flag shown in the book is deprecated and now fails
outright (`Could not check specified current directory at '/home'`).
`--volume HOST:GUEST` replaces it. `wasmtime` still accepts `--dir`.