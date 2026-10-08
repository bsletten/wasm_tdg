# 04 : SIMD

The book lists SIMD among the proposals that were not yet widely supported.
`simd128` is now part of the WebAssembly standard and ships everywhere.

A `v128` holds four `f32` lanes, so a dot product can consume four elements
per instruction. `src/lib.rs` implements the same dot product twice — once
scalar, once with `f32x4` intrinsics — so the results can be checked against
each other and timed.

```
> make disasm
   3 f32x4.add
   4 f32x4.extract_lane
   3 f32x4.mul
   2 v128.const
   6 v128.load

> make run
lanes          : 4096 f32 pairs
dot_scalar     : 2.115388870239258
dot_simd       : 2.1153907775878906
relative drift : 9.017e-7 (reassociation, not a bug)
scalar         : 66.4 ms
simd           : 16.4 ms  (4.06x)
OK
```

Two things worth noticing.

**The results are not bit-identical.** Lane-wise accumulation adds the terms in
a different order, and `f32` addition is not associative. The harness asserts
the relative difference stays within what reassociation explains rather than
checking for equality. This is not a WebAssembly quirk — it is the same thing
that happens with SIMD on native targets — but it is the kind of detail that
bites people porting numeric code.

**`simd128` is opt-in at compile time.** The `Makefile` passes
`RUSTFLAGS="-C target-feature=+simd128"`. Without it you get scalar code and
no error. Use `make disasm` to confirm you actually got vector instructions,
and `ch12/detector` to confirm the browser will accept them.

## In the browser

`index.html` runs the same comparison in your browser, with a
`wasm-feature-detect` check first so it degrades honestly rather than failing
to validate. It is hosted at
<https://bsletten.github.io/wasm_tdg/additional-exercises/04-simd/>, or locally:

```
> make serve      # http://localhost:10002/
```

The module has no imports at all: the two input buffers are `static mut`
arrays, and JavaScript writes into them through the exported `memory`. That is
Chapter 4's technique, unchanged.
