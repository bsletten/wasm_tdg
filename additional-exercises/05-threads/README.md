# 05 : Threads

Threads were the headline "not yet widely supported" feature when the book was
written — the root README of this repository promises to come back to them.
They now work in every major browser.

The mechanism: linear memory becomes a `SharedArrayBuffer`, pthreads become
Web Workers, and the `i32.atomic.*` / `memory.atomic.*` instructions provide
the synchronisation primitives. `threads.c` is ordinary C using `pthread.h`.

```
> make run
Linear memory is a SharedArrayBuffer, so 4 threads can read it directly.

parallel:
  thread 0 summed [0, 1048576) -> 14.4402
  thread 1 summed [1048576, 2097152) -> 0.6931
  thread 2 summed [2097152, 3145728) -> 0.4055
  thread 3 summed [3145728, 4194304) -> 0.2877

serial   sum = 15.8264537564  (6.6 ms)
parallel sum = 15.8264537564  (2.9 ms)
speedup      = 2.30x
```

```
> make serve      # then open http://localhost:10001/
```

## The part that trips everyone up

`SharedArrayBuffer` is only available to pages that are **cross-origin
isolated**, which requires two response headers:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

A plain `python3 -m http.server` does not set them, and you get
`SharedArrayBuffer is not defined` with no hint as to why. That is what
`server.py` here is for, and why `index.html` checks
`window.crossOriginIsolated` and tells you the truth before trying anything.

This is a restriction that post-dates the book: the headers were mandated in
response to Spectre, which is also why `SharedArrayBuffer` was briefly
disabled entirely. Any chapter-style example that wants threads needs this
server, not the one used elsewhere in the repository.

### ...and on GitHub Pages

A static host cannot set headers at all, which would normally make this
example impossible to publish. The page therefore vendors
[`coi-serviceworker`](https://github.com/gzuidhof/coi-serviceworker) (MIT): a
service worker that re-serves the page with both headers and reloads once, so
the second load is cross-origin isolated. That is why the hosted copy at
<https://bsletten.github.io/wasm_tdg/additional-exercises/05-threads/>
flickers on first visit. Served by `server.py` the real headers are already
there and the worker does nothing.

To confirm the atomics really are in the binary:

```
> wasm-tools print threads.wasm | grep -oE 'memory.atomic.[a-z0-9]+' | sort -u
memory.atomic.notify
memory.atomic.wait32
```
