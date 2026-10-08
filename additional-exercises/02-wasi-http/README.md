# 02 : wasi:http — a Server With No Server In It

Chapter 8 put WebAssembly on the server by embedding it in Node.js or Deno:
the host was a JavaScript runtime and the module was a guest inside it. This
inverts that. The component here *is* the application, and the HTTP server is
the host's problem.

There is no `main`, no socket, no listener and no HTTP library compiled into
the guest. The component exports `wasi:http/incoming-handler` and the host
calls it. That is why the same artifact runs under `wasmtime serve` and on
edge platforms that speak the same world.

```
> make serve
Serving HTTP on http://0.0.0.0:8080/
```

Then:

```
> curl http://localhost:8080/
WebAssembly: The Definitive Guide -- wasi:http example
...
> curl http://localhost:8080/hello
Hello, world!
> curl "http://localhost:8080/how-old?now=2026&born=2000"
You are 26
```

`how_old` has followed us since Chapter 2, where it was two `local.get`s and
an `i32.sub`. It is the same function; only the way the outside world reaches
it has changed.

`make wit` shows the contract. Note the `-S cli` flag in the `serve` target:
`wasmtime serve` provides the `wasi:http/proxy` world by default, and this
component's Rust standard library also pulls in `wasi:cli` imports, so they
have to be enabled explicitly.
