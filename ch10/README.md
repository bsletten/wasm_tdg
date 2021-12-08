## Chapter 10 : Rust and WebAssembly

Rust is an elegant and popular modern systems engineering programming
language that is escaping its past. It is finding wider adoption in
many scenarios because it produces safe and fast code. Because of its
LLVM-basis, it is a natural fit for WebAssembly.

The basic Rust code can be compiled and executed using the `rustc` compiler:

```
> rustc helloworld.rs
> ./helloworld
Hello, World!
```

With the WebAssembly backend added, you can compile regular Rust to
WebAssembly with optimizations and then load it into a runtime like
Wasm3:

```
> rustc -A dead_code --target wasm32-unknown-unknown -C opt-level=3 --crate-type=cdylib add.rs -o add.wasm
> wasm3 --func add add.wasm 2 3
Result: 5
```

Running Rust-sourced WebAssembly in the browser isn't really any
different than other kinds. If you start up a web server and point a
browser to
[https://localhost:10000/index.html](https://localhost:10000/index.html)
you should see the result.

The `geo-example` project has been modified slightly from the
published version because of an API change. From within that directory
with `wasm-bindgen` and
[wasm-pack](https://rustwasm.github.io/wasm-pack/) installed you
should be able to run and see the result in the browser:

```
> wasm-pack build --target web
.
.
.
> python -m http.server 10000
```