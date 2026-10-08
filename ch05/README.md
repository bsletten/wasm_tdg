## Chapter 5 : Using C/C++ and WebAssembly

In this chapter we start diving into more detail about using C and C++
source languages compiled down to WebAssembly.

You can build the first example with `clang`:

```
> clang howold.c
> ./a.out
You are 21!
```

You can build the standalone function to WebAssembly using the following:

```
> clang --target=wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all howold2.c -o howold.wasm
```

**You need a `wasm-ld` on your `PATH` for this to work.** `clang` itself
knows how to *compile* for `wasm32`, but linking needs LLVM's WebAssembly
linker, and that does not ship with Apple's command line tools (you get
`clang: error: unable to execute command: posix_spawn failed`). On macOS:

```
> brew install lld
> export PATH="$(brew --prefix lld)/bin:$PATH"
```

Alternatively, the [WASI SDK](https://github.com/WebAssembly/wasi-sdk) and
the Emscripten SDK both bundle a matching `clang` and `wasm-ld`.

And obviously, you can serve up the `.html` files with Python:

```
> python3 -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::1 - - [07/Dec/2021 09:22:39] "GET /howold.html HTTP/1.1" 200 -
::1 - - [07/Dec/2021 09:22:39] "GET /bootstrap.min.css HTTP/1.1" 200 -
::1 - - [07/Dec/2021 09:22:39] "GET /utils.js HTTP/1.1" 200 -
```

You can compile multiple files as follows:

```> clang simplemain.c simple.c -o simplemain
> ./simplemain
The array sum is: 45
```

Keep in mind that some of the simplex.c examples are broken as
detailed in the text are not good examples of code.

The example in the `helloworld` directory is based upon work by
[Petter Strandmark](https://github.com/PetterS/clang-wasm) but I have
modified it to use a simpler algorithm with merge sort. In that
directory you can do the following:

```
> make
> python server.py
serving at port 4242
127.0.0.1 - - [07/Dec/2021 09:45:43] "GET / HTTP/1.1" 200 -
127.0.0.1 - - [07/Dec/2021 09:45:43] "GET /library.wasm HTTP/1.1" 200 -
```

Note his Python script is using port 4242. You will have to look at
the console output to see the results. The script exists mainly to serve
`.wasm` with the correct `application/wasm` MIME type; modern Python
knows that type already, so a plain `python3 -m http.server` works too.
