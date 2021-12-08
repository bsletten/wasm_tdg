## Chapter 11 : WebAssembly System Interface (WASI)

WASI-based runtimes provide capabilities you wish to give to a bit of executable code.

[wasm3](https://github.com/wasm3/wasm3),
[wasmtime](https://github.com/bytecodealliance/wasmtime), and
[wasmer](https://wasmer.io) are all WASI-enabled to one extent or the other.

To run the `.wat` file, you should be able to just do:

```
> wasmtime hello.wat
hello world
> wasmer hello.wat
hello world
```

To build the standalone C file with the
[wasienv](https://github.com/wasienv/wasienv) toolchain installed is
straightforward:

```
> wasicc -o hello hello.c
> ./hello
Hello, World!
> wasmtime hello.wasm                                                                    
Hello, World!
> wasmer hello.wasm                                                                      
Hello, World!
> wasm3 hello.wasm                                                                       
Hello, World!
```

To run the `hello-world` project, the following should work:

As a standalone Rust application:

```
> cargo build --release
> cargo run --release
```

As a WASI application:

```
> cargo build --target wasm32-wasi --release
> wasm3 target/wasm32-wasi/release/hello-world.wasm
Hello, world!
> wasmtime target/wasm32-wasi/release/hello-world.wasm
Hello, world!
> wasmer target/wasm32-wasi/release/hello-world.wasm                            
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
> cargo build --target wasm32-wasi --release
> wasmtime target/wasm32-wasi/release/hello-fs.wasm
<FAILS>
> wasmtime --dir=. target/wasm32-wasi/release/hello-fs.wasm
Read in from file: Hello, world!
> wasmer target/wasm32-wasi/release/hello-fs.wasm
<FAILS>
> wasmer --dir=. target/wasm32-wasi/release/hello-fs.wasm
Read in from file: Hello, world!
```