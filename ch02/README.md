## Chapter 2 : "Hello, World!" (Sort of)

Because of the limitations of the WebAssembly platform, this chapter
focuses on the `.wat` text format as a starting point. We will build
to higher level examples in subsequent chapters.

You can paste the `hello.wat` file using [wat2wasm
demo](https://webassembly.github.io/wabt/demo/wat2wasm/index.html) or
by installing the Wabt tools and executing the following:

```
> wat2wasm hello.wat
> ls -alF
total 16
drwxr-xr-x  4 brian  staff  128 Dec  7 07:59 ./
drwxr-xr-x  6 brian  staff  192 Dec  7 07:53 ../
-rw-r--r--  1 brian  staff   45 Dec  7 07:59 hello.wasm
-rw-r--r--  1 brian  staff  200 Dec  7 07:59 hello.wat
```

This `.wasm` file can be executed in a repl in a WebAssembly runtime
such as [wasm3](https://github.com/wasm3/wasm3) as follows:

```
> wasm3 --repl hello.wasm
wasm3> how_old 2021 2000
Result: 21
wasm3> how_old 2021 1980
Result: 41
wasm3> CTRL-D
>
```

With the `utils.js`, the [Bootstrap]() `.css` file and `index.html` in
the directory, you can start up an HTTP server in the directory and
serve up the content to a browser using `emrun` or `Python 3`:

```
> emrun --port 10000 .
```

`emrun` is part of the
[Emscripten](https://emscripten.org/docs/getting_started/downloads.html)
SDK. By default `emrun` will open a browser for you.

For Python 3, execute the following code and then point your browser
to [http://localhost:10000](http://localhost:10000):

```
> python -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::1 - - [07/Dec/2021 08:15:43] "GET / HTTP/1.1" 304 -
::1 - - [07/Dec/2021 08:15:43] "GET /utils.js HTTP/1.1" 304 -
::1 - - [07/Dec/2021 08:15:43] "GET /bootstrap.min.css HTTP/1.1" 304 -
::1 - - [07/Dec/2021 08:15:43] "GET /hello.wasm HTTP/1.1" 304 -
```