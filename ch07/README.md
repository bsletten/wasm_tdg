## Chapter 7 : WebAssembly Tables

The first examples are straightforward C programs and can be built easily:

```
> clang main.c library.c
> ./a.out
Hello, world.
How are you?
```

To build the other examples, the `.wasm` files are provided but you
can use the `wat2wasm` tool as in previous chapters to regenerate if
need be.

To execute these examples, run a web server and then hit
[http://localhost:10000/tables.html](http://localhost:10000/tables.html)
and
[http://localhost:10000/tables2.html](http://localhost:10000/tables2.html).
