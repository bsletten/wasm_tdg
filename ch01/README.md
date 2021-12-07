## Chapter 1 : Introduction

There are not many code samples in this chapter, but we do have the
recurring "Hello, World!" example in `hello.c`. We can compile this to
`asm.js` using the `emcc` command line tool.

If you have followed the instructions for installing this tool, you
should be able to execute the following:

```
> emcc hello.c
> ls -laF
total 272
drwxr-xr-x  5 brian  staff     160 Dec  7 07:47 ./
drwxr-xr-x  5 brian  staff     160 Dec  7 07:42 ../
-rw-r--r--  1 brian  staff  121686 Dec  7 07:47 a.out.js
-rwxr-xr-x  1 brian  staff   11711 Dec  7 07:47 a.out.wasm*
-rw-r--r--  1 brian  staff      75 Dec  7 07:42 hello.c
> node a.out.js
Hello, world!
```

Obviously it is a standard C program so you can build it with `clang`
or `gcc` as well and it should work as a standalone application.

```
> clang hello.c
> ls -alF
total 120
drwxr-xr-x  5 brian  staff    160 Dec  7 07:55 ./
drwxr-xr-x  6 brian  staff    192 Dec  7 07:53 ../
-rw-r--r--  1 brian  staff    673 Dec  7 07:51 README.md
-rwxr-xr-x  1 brian  staff  49424 Dec  7 07:55 a.out*
-rw-r--r--  1 brian  staff     75 Dec  7 07:42 hello.c
> ./a.out
Hello, world!
```
