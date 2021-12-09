## Chapter 12 : WebAssembly System Interface (WASI)

Being able to detect which features a browser supports is important as
the proposals are adopted incrementally.

The `detector` directory features the
[wasm-feature-detect](https://github.com/GoogleChromeLabs/wasm-feature-detect)
library to advertise which features a browser supports. Simply launch
a web server and check out the JavaScript console to see the results.

The `hello-wasi`, `hello-mvr`, `hello-extref` and `hello-modlink`
directories feature native Rust applications that are dynamically
loading WebAssembly modules. Therefore they can be built and run as a
regular Rust applications.

```
> cargo build --release
> cargo run --release
You are 21
```

As additional proposals are more widely supported, we will add more
examples on how to use them.
