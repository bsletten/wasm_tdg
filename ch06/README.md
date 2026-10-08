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
> python3 -m http.server 10000
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
  -s EXPORTED_RUNTIME_METHODS="['callMain']"
> python3 -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::1 - - [07/Dec/2021 10:34:41] "GET / HTTP/1.1" 200 -
```

Note the flag name. The book (and the original `Makefile` here) used
`EXTRA_EXPORTED_RUNTIME_METHODS`, which Emscripten has since removed:

```
em++: error: invalid command line setting `-sEXTRA_EXPORTED_RUNTIME_METHODS=['callMain']`:
      No longer supported, use EXPORTED_RUNTIME_METHODS
```

The `Makefile` has been updated to use `EXPORTED_RUNTIME_METHODS`. Two
things made this failure easy to miss: the recipe's `$(COMPILER)` variable
was defined as `-em++`, and `make` treats a leading `-` on a recipe line
as "ignore any error", so a failing build still reported success. That
stray hyphen is gone too.

Two further changes were needed before `index.html` would actually work:

* Emscripten no longer exports the in-memory filesystem by default, so
  `EXPORTED_RUNTIME_METHODS` is now `"['callMain','FS']"`. Without it,
  clicking *Press Me* aborts with ``'FS' was not exported. add it to
  EXPORTED_RUNTIME_METHODS``.
* There is also no longer a bare global `FS`; it hangs off `Module`. The page
  calls `Module.FS.readFile(...)`.

And one genuine bug in the page's own code, unrelated to toolchain drift:
`convertToImageData` computed its destination index as
`(x + width * (height - y)) * 4`. BMP rows run bottom-up so the flip is
right, but at `y == 0` that lands one full row past the end of the
`ImageData` buffer. `Uint8ClampedArray` drops out-of-range writes silently, so
the symptom was a one-row shift rather than an error -- exactly 1,200 of
960,000 pixels never got written for the 1200x800 test image. It is
`height - 1 - y` now.
