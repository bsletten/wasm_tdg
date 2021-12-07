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

And obviously, you can serve up the `.html` files with Python:

```
> python -m http.server 10000
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
the console output to see the results.
