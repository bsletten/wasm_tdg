## Chapter 6 : Legacy Code in the Browser

In this chapter we start examing running existing code in WebAssembly
without much modification.

"Hello, World!" is finally pretty easy to run after all if you use the
[Emscripten](https://emscripten.org) tools:

```
> emcc hello.c -o hello.js
> node hello.js
Hello, World!
```

Using the generated scaffolding in the browser also works quite easily
even though it looks very different from what we have been playing
with so far. Run a web server and then point your browser to
(http://localhost:10000/hello.html)[http://localhost:10000/hello.html].

```
> python -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::1 - - [07/Dec/2021 10:34:41] "GET / HTTP/1.1" 200 -
```

The
[http://localhost:10000/hello-delay.html](http://localhost:10000/hello.html)
version does not call the `main()` method right away but instead does
so when you press the `Press Me` button.

The main example is a port of some code from Arash Partow. His site
has several examples of clean C++ code, but I chose to use his
[Windows Bitmap Generation
code](http://www.partow.net/programming/bitmap/index.html).

This example is contained in the `bitmap` directory. If you have the
[Emscripten](https://emscripten.org) tools installed, you should just
be able to do the following:

```
> make
em++ -ansi -pedantic-errors -Wall -Wall -Werror -Wextra -o bitmap_test.js bitmap_test.cpp
  -L/usr/lib -lstdc++ -lm -s FORCE_FILESYSTEM=1 -s ALLOW_MEMORY_GROWTH=1  -s INVOKE_RUN=0
  -s EXTRA_EXPORTED_RUNTIME_METHODS="['callMain']"
> python -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::1 - - [07/Dec/2021 10:34:41] "GET / HTTP/1.1" 200 -
```
