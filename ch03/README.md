## Chapter 3 : WebAssembly Modules

This chapter focuses on understanding the WebAssembly Module structure.

In the previous chapter, I had you build the `.wasm` file from the
`.wat` file, but in this one, I will just provide it in the repo.

You can use the [WebAssembly Code
Explorer](https://wasdk.github.io/wasmcodeexplorer/) to check out the
module structure online.

With the [Wabt](https://github.com/WebAssembly/wabt) tools, you can
interrogate the structure directly:

```
> wasm-objdump -x hello.wasm

hello.wasm:	file format wasm 0x1

Section Details:

Type[1]:
 - type[0] (i32, i32) -> i32
Function[1]:
 - func[0] sig=0 <how_old>
Export[1]:
 - func[0] <how_old> -> "how_old"
Code[1]:
 - func[0] size=7 <how_old>
```

Or with debug support:

```
> wat2wasm hello.wat -o hellodebug.wasm --debug-names
> wasm-objdump -x hellodebug.wasm

hellodebug.wasm:	file format wasm 0x1

Section Details:

Type[1]:
 - type[0] (i32, i32) -> i32
Function[1]:
 - func[0] sig=0 <how_old>
Export[1]:
 - func[0] <how_old> -> "how_old"
Code[1]:
 - func[0] size=7 <how_old>
Custom:
 - name: "name"
 - func[0] <how_old>
 - func[0] local[0] <year_now>
 - func[0] local[1] <year_born>
```

To load the example in the browser, start up a web server as
previously. Keep in mind that you will need to look in the developer
web console for the printed output of `imports.html`.

```
> python -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::1 - - [07/Dec/2021 08:39:53] "GET /imports2.html HTTP/1.1" 200 -
::1 - - [07/Dec/2021 08:39:54] "GET /hellolog.wasm HTTP/1.1" 304 -
```

The file `imports2.html` does some more with the example.

Don't forget that the preferred way to instantiate WebAssembly modules
is highlighted in `preferred.html`.