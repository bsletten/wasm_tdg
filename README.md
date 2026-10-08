# WebAssembly : The Definitive Guide

Welcome to the code repository for this book on WebAssembly.

This includes code samples from the book as well as additional projects and
examples of features that were not widely supported at the time of writing.

> **Live examples: <https://bsletten.github.io/wasm_tdg/>** &mdash; the
> browser-runnable examples from the book, compiled from this repository by
> [CI](.github/workflows/pages.yml) and served from GitHub Pages. Twenty-seven
> pages across chapters 2, 3, 4, 5, 6, 7, 10, 12, 13 and 14, plus the SIMD and
> threads exercises. Many log to the JavaScript console rather than the page,
> as they do in the book; the index says which.

> **Status, October 2026.** The book was written in 2021, and a lot of the
> surrounding toolchain moved underneath it. Everything here has been brought
> back to a state where it builds and runs against current tools, and there is
> now [CI](.github/workflows/build.yml) so it stays that way. The chapter
> directories remain faithful to the book; where a command or an API had to
> change, it is called out in that chapter's `README.md` and summarised below.
>
> The [`additional-exercises`](additional-exercises) directory is new. It picks
> up the threads the book left dangling — threads, SIMD, the component model,
> WASI Preview 2 and 3 — with examples built against today's toolchain.

## Layout

| Directory | Chapter |
|---|---|
| [`ch01`](ch01) | Introduction |
| [`ch02`](ch02) | "Hello, World!" (Sort of) |
| [`ch03`](ch03) | WebAssembly Modules |
| [`ch04`](ch04) | WebAssembly Memory |
| [`ch05`](ch05) | Using C/C++ and WebAssembly |
| [`ch06`](ch06) | Legacy Code in the Browser |
| [`ch07`](ch07) | WebAssembly Tables |
| [`ch08`](ch08) | WebAssembly in the Server |
| [`ch10`](ch10) | Rust and WebAssembly |
| [`ch11`](ch11) | WebAssembly System Interface (WASI) |
| [`ch12`](ch12) | Extending the WebAssembly Platform |
| [`ch13`](ch13) | WebAssembly and .NET |
| [`ch14`](ch14) | Using AssemblyScript and WebAssembly |
| [`ch17`](ch17) | WebAssembly and Other Languages |
| [`additional-exercises`](additional-exercises) | **New** — where WebAssembly is now |

Most browser examples just need a static server in the relevant directory:

```
> python3 -m http.server 10000
```

The exception is [`additional-exercises/05-threads`](additional-exercises/05-threads),
which needs the cross-origin isolation headers that `SharedArrayBuffer`
requires; it ships its own `server.py`.

## What changed since the book was printed

### Things that no longer built at all

* **`ch12/hello-modlink`** — `wasmtime` 0.28 cannot be compiled by a modern
  Rust toolchain; the build died inside an old transitive dependency whose
  version range had no fix. All four `ch12` Rust examples are now on
  `wasmtime` 44. See [`ch12/README.md`](ch12/README.md) for the API changes.
* **`ch10/geo-example`** — `wasm-bindgen` releases before 0.2.88 are rejected
  outright by current Rust. The pinned versions became compatible ranges.
* **`ch06/bitmap`** — Emscripten removed `EXTRA_EXPORTED_RUNTIME_METHODS`.
  Worse, `COMPILER` was defined as `-em++`, and `make` reads a leading `-` on
  a recipe line as "ignore errors", so the broken build reported success.
* **`ch05/helloworld`** — the link flag was `-error-limit=0` with one hyphen,
  which `lld` parses as `-e rror-limit=0`, i.e. an entry symbol named
  `rror-limit=0`.
* **`ch17/zig`** — `zig build-lib -dynamic` no longer works for a freestanding
  module, and `std.fs.wasi.PreopenList` was deleted from Zig's standard
  library. `preopens.zig` was rewritten against the raw WASI calls.
* **`ch13`** — both projects targeted `net5.0`, EOL since May 2022. Now
  `net10.0`, with `wasmtime` 48 and Blazor 10.
* **`ch08/node/a.out.js`** — the committed Emscripten artifact called `fetch()`
  on a filesystem path, which Node ≥ 18 now has, and rejects. Regenerated.
* **`ch12/hello-wasi/hello.wat`** — the last file still using `get_local`.

### Things that "worked" but silently did the wrong thing

* **`ch14`** — `asc file.ts -b file.wasm` exits 0, prints the text format to
  standard output, and writes no `.wasm` at all. The flag is `-o` now. There is
  a `package.json` in `ch14` so you no longer have to install `asc` globally.
* **`ch14/loader`** — the example was unfinished and could never have run
  (inline code inside a `<script src=...>`, a `modules.exports` typo, a missing
  `await`, and a bare module specifier a browser cannot resolve). Rewritten.
* **`ch04`, `ch06`** — eight pages referenced a `bootstrap.min.css` that was
  not in those directories. The 404 is visible in the output printed in
  `ch04/README.md`. Added.
* **`ch06/bitmap/index.html`** — clicking the button aborted with ``'FS' was
  not exported``: Emscripten stopped exporting the in-memory filesystem by
  default, and the bare global `FS` is now `Module.FS`. Separately, the page's
  bottom-up row flip was off by one and silently dropped a row of the image.
  See [`ch06/README.md`](ch06/README.md).

### Commands that changed

| Chapter | Was | Now |
|---|---|---|
| ch02, ch10 | `wasm3 ...` | `wasmtime --invoke ...` (wasm3 is unmaintained) |
| ch05 | `clang --target=wasm32` | same, but needs `wasm-ld` — `brew install lld` |
| ch08 | `node-gyp configure build` | `npx node-gyp configure build` |
| ch08 | `deno.land/std` `serve()` | built-in `Deno.serve`; SQLite via `jsr:@db/sqlite` |
| ch11 | `--target wasm32-wasi` | `--target wasm32-wasip1` |
| ch11 | `wasmer --dir=.` | `wasmer run --volume .:/` |
| ch11 | `wasicc` (wasienv) | the [WASI SDK](https://github.com/WebAssembly/wasi-sdk) |
| ch14 | `asc x.ts -b x.wasm` | `asc x.ts -o x.wasm` |
| ch17 | `zig build-lib -dynamic` | `zig build-exe -fno-entry -rdynamic` |
| all | `python -m http.server` | `python3 -m http.server` |

### Additions

* `ch14/hello-imp` — a worked example from the text of Chapter 14 that was
  missing from this repository.
* `ch12/detector` — the book's eleven feature probes became twenty-three, and
  the page now renders a table rather than only logging to the console.
  `memory64`, commented out in the printed example, works.
* A `.gitignore`, a `LICENSE`, [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md),
  and CI. Build output that had been committed (`ch13/**/obj`, `*.o`,
  a `.DS_Store`) is no longer tracked.

## The live example site

<https://bsletten.github.io/wasm_tdg/>

[`.github/workflows/pages.yml`](.github/workflows/pages.yml) builds the
browser-runnable examples from source on every push to `main` and publishes
them. Nothing is hand-copied: the `.wasm` each page loads is the output of
`wat2wasm`, `clang`, `emcc`, `rustc`, `asc` or `dotnet publish` running in CI,
and the workflow fails if any link on the index does not resolve.

The landing page is [`site/index.html`](site/index.html).

What is *not* hosted, because it cannot run in a browser: the native C and Rust
builds (ch01, ch05), the Node/Deno/N-API examples (ch08), the WASI
command-line modules (ch11, ch17), the native programs that *embed* wasmtime
(ch12's four Rust examples, ch13's `wasmtime-dotnet`), and the Preview 2/3
components in `additional-exercises` 01, 02, 03 and 06. All of those are still
built and run by [`build.yml`](.github/workflows/build.yml) on every push --
they just produce terminal output instead of a page.

Two notes on the hosted set:

* **Threads** need cross-origin isolation, and GitHub Pages cannot set
  response headers. The page vendors
  [`coi-serviceworker`](https://github.com/gzuidhof/coi-serviceworker) (MIT),
  which re-serves the page with `Cross-Origin-Opener-Policy` and
  `Cross-Origin-Embedder-Policy` and reloads once. Served locally with its own
  `server.py`, the real headers are present and the worker is a no-op.
* **Blazor** is published assuming it is served from a domain root, so the
  workflow rewrites its `<base href>` for the subpath and writes a
  `.nojekyll` file -- without which Jekyll would strip the `_framework`
  directory the .NET runtime loads from.

## Seeing it in the wild : the ikigai resolution kernel

**Live demo: <https://ikigai-rs.github.io/ikigai-web-demo/>** &middot;
source: <https://github.com/ikigai-rs/ikigai-web-demo>

Most of this repository is deliberately small: one idea per example. If you
want to see the pieces assembled into something real, this demo runs a whole
*resource-oriented computing* kernel in the browser with no server, no `fetch`
and no JavaScript framework. It is a good capstone for the book, because almost
every chapter shows up in it somewhere.

**The page is a resource.** `index.html` is a near-empty shell whose only job
is about five lines of glue calling `compose('urn:data:page')`. Everything you
see is produced by the kernel resolving that URN client-side. Resource shapes
contain transclusion markers — `$a{<iri>}`, optionally with arguments like
`$a{urn:iki:fn:toUpper?in="resource-oriented computing"}` — and `compose()`
expands them recursively, since a composed shape may itself contain more
markers. The recursion happens entirely inside WebAssembly.

**The kernel is the Rust crates, compiled to wasm32.** `ikigai-core`
(resolution), `ikigai-vocab`, `ikigai-engine` (the CLI) and `ikigai-shacl` are
ordinary published crates; the browser build is the same code with a different
target. The build is exactly the three steps Chapter 10 describes, no framework
in between:

```
> cargo build --release --target wasm32-unknown-unknown
> wasm-bindgen --target web --out-dir dist
> # serve dist/ over HTTP
```

Note that the project pins `wasm-bindgen` with `=0.2.108` in `Cargo.toml` and
requires the matching `wasm-bindgen-cli`. That exact-version pin is the same
hazard `ch10/geo-example` ran into from the other direction — the crate and the
CLI have to agree, and a stale pin eventually stops building at all.

**The XSLT engine is a lazy module, and that is the interesting part.** XSLT
and JSON-LD are not in the initial download. They are fetched and instantiated
on first use — around 1.9 MB over the wire, once — so a page that never
transforms XML never pays for an XSLT engine. The SHACL engine behaves the same
way, loading on the first validation. This is the practical shape of what
Chapter 12 discusses as module linking: rather than one monolithic `.wasm`, the
kernel resolves a capability to an implementation at the moment it is needed,
and caches it. The browser-side cache is content-addressed, so repeated
resolution of the same resource does no work.

A nice demonstration of why that matters: `urn:shacl:validate` has *two*
implementations. By default it resolves to the Rust `ikigai-shacl` crate
running in WebAssembly; add `?shacl=js` to the URL and it resolves to the
JavaScript `shacl-engine` instead. Both report the same verdicts. The consumer
does not change — only what the URN resolves to.

**Capabilities are enforced, not advised.** Chapter 11's point about WASI is
that a module gets only what the host grants it. The demo makes that visible
outside WASI: `cap read-only` narrows a session, after which writes are refused
with ``capability does not grant `write` `` while reads still resolve, and the
file module jails paths against traversal.

**`net.html` moves the kernel to the other end of a wire.** It requests
`urn:data:page` from a *remote* kernel over HTTP/3 QUIC via WebTransport,
streaming the `ikigai-wire` protocol — whose codec also runs in WebAssembly,
exposed as `encodeComposeCall()` / `decodeReply()`. Watching the server report
`computed` and then `cached` across successive commands is the clearest picture
of resolution-with-caching you will get. The same kernel, the same protocol,
either side of the network.

## Toolchain versions used

These examples were last verified on macOS (Apple silicon) with:

```
clang (Apple) 21.0.0     emcc 6.0.9           wabt 1.0.39
rustc 1.92 / 1.99        wasmtime 48.0.2      wasmer 7.4.1
node v26.8.2             deno 2.9.6           dotnet 10.0.401
zig 0.17.0               asc 0.28.20          wasm-tools 1.261.0
```

Nothing is pinned to those exact versions except where a `Cargo.lock`,
`package.json` or `.csproj` says so.

## Errata

Code corrections that differ from the printed book are noted in each chapter's
`README.md`. The two most likely to trip a reader following along in the text:

* Chapter 6 prints `-s EXTRA_EXPORTED_RUNTIME_METHODS="['callMain']"` in
  several listings. Use `EXPORTED_RUNTIME_METHODS`.
* Chapter 4's listing of `memory2.wat` and Chapters 12–13's `.wat` listings use
  `get_local`/`set_local`. Those became `local.get`/`local.set`, and `anyfunc`
  became `funcref` in the text format — though `"anyfunc"` is still correct in
  the JavaScript `WebAssembly.Table` constructor, which is why `ch07/tables2.html`
  still says it.
