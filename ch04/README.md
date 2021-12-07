## Chapter 4 : WebAssembly Memory

In this chapter we begin to focus on using WebAssembly Memory instances.

The code examples are expressed in `.html` files that you can serve up
as previously with Python or some other web server:

```
> python -m http.server 10000
Serving HTTP on :: port 10000 (http://[::]:10000/) ...
::1 - - [07/Dec/2021 08:56:41] "GET /index.html HTTP/1.1" 200 -
::1 - - [07/Dec/2021 08:56:41] code 404, message File not found
::1 - - [07/Dec/2021 08:56:41] "GET /bootstrap.min.css HTTP/1.1" 404 -
::1 - - [07/Dec/2021 08:56:41] "GET /utils.js HTTP/1.1" 200 -
::1 - - [07/Dec/2021 08:56:41] "GET /memory.wasm HTTP/1.1" 200 -
::1 - - [07/Dec/2021 08:57:41] "GET /memory.html HTTP/1.1" 200 -
```
