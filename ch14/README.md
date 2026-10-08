## Chapter 14 : Using AssemblyScript and WebAssembly

The `hello`, `hello-imp`, `as-mem`, `stdlib` and `loader` directories have
simple examples that allow you to compile AssemblyScript to WebAssembly and
then access it via JavaScript as we have been doing.

The compiler's `-b` flag (for "binary") was removed; the output file is now
named with `-o`/`--outFile`. This is worth knowing because `asc` does not
fail when handed the old flag -- it silently prints the text format to
standard output and writes no `.wasm` file at all.

There is now a `package.json` here so you do not have to install `asc`
globally or remember the flags:

```
> npm install
> npm run build          # builds all five examples
> npm run build:hello    # ...or just one
```

which is equivalent to running the compiler directly:

```
> npx asc hello/hello.ts -o hello/hello.wasm
> python3 -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET / HTTP/1.1" 200 -
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET /bootstrap.min.css HTTP/1.1" 200 -
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET /utils.js HTTP/1.1" 200 -
::ffff:127.0.0.1 - - [08/Dec/2021 16:33:01] "GET /hello.wasm HTTP/1.1" 200 -
```

Then point a browser at the example you want, e.g.
[http://localhost:10000/hello/](http://localhost:10000/hello/). Most of
these print to the JavaScript console.

A few notes on the individual examples:

* `hello-imp` shows an AssemblyScript module *importing* a JavaScript
  function. It is discussed in the chapter but was missing from this
  repository.
* `loader` uses the [AssemblyScript
  loader](https://www.assemblyscript.org/loader.html) to pass strings
  across the boundary. It must be compiled with `--exportRuntime` so that
  the loader can allocate strings in the module's memory; the `npm run
  build:loader` script does this. The chapter walks through the upstream
  `assemblyscript/examples/loader` project; the version here is a small
  self-contained equivalent.
