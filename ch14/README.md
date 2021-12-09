## Chapter 14 : Using AssemblyScript and WebAssembly

The `hello`, `as-mem`, `stdlib` and `loader` directories have simple
examples that allow you to compile AssemblyScript to WebAssembly and
then access it via JavaScript as we have been doing.

```
> asc hello.ts -b hello.wasm
> python -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET / HTTP/1.1" 200 -
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET /bootstrap.min.css HTTP/1.1" 200 -
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET /utils.js HTTP/1.1" 200 -
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET /hello.wasm HTTP/1.1" 200 -
```
