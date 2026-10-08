## Chapter 8 : WebAssembly in the Server

This chapter exits the browser for Node.js and Deno. While I didn't
want to create overly complicated examples for the book itself, I will
have additional examples eventually in this repository as well.

The first example is the traditional way to extend Node.js with Add On
code. This is in the directory `node-addon`. If you enter that directory, you
can build the example with (`node-gyp` is no longer installed as a
standalone command, so run it through `npx`):

```
> npx node-gyp configure build
> node test.js
This should be eight: 8
```

In the `node` directory, there is the simple example that adds two
numbers together. If you include the `main()`, you can run it with:

```
> emcc add.c
> node a.out.js
The sum of 2 and 3 is: 5
```

Node.js has experimental support for loading WebAssembly modules as
ES6 modules. If you remove the `main()` from `add.c` and do the
following, you should be able to see this on recent versions of
Node.js:

```
> emcc add.c
> node --experimental-modules --experimental-wasm-modules index.mjs
The sum of 6 and 2 is: 8
```

In the `deno` directory, there are some examples that work with this
exciting new environment.

```
> deno run --allow-read main.ts
2 + 3 =  5
```

or

```
> deno run --allow-read --allow-net main-serve.ts
Listening on http://0.0.0.0:9000/ (http://localhost:9000/)
```

or

```
> deno run --allow-read --allow-write --allow-net --allow-ffi --allow-env db-serve.ts
Listening on http://0.0.0.0:9000/ (http://localhost:9000/)
```

The two server examples have been updated since the book was printed. They
used `serve()` from `deno.land/std`, which has since been removed -- Deno
now has a built-in `Deno.serve`, so there is nothing to import at all. The
SQLite example moved from the unmaintained `deno.land/x/sqlite` to its
successor, `jsr:@db/sqlite`, which needs the extra `--allow-ffi` and
`--allow-env` permissions shown above.