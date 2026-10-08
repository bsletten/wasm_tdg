//! Fixed-width SIMD (the `simd128` proposal).
//!
//! When the book was written, SIMD was still a proposal behind a flag. It is
//! now part of the WebAssembly standard and shipping everywhere. A single
//! `v128` value holds four `f32` lanes, so a dot product can chew through
//! four elements per instruction.
//!
//! Both functions below compute the same thing, so JavaScript can check the
//! results against each other -- and time them.

#![cfg(target_arch = "wasm32")]

use core::arch::wasm32::*;

/// Scratch buffers the JavaScript side fills in. Keeping them static avoids
/// needing an allocator, so this module has no imports whatsoever.
const MAX: usize = 4096;
static mut A: [f32; MAX] = [0.0; MAX];
static mut B: [f32; MAX] = [0.0; MAX];

#[no_mangle]
pub extern "C" fn buffer_a() -> *mut f32 {
    (&raw mut A).cast()
}

#[no_mangle]
pub extern "C" fn buffer_b() -> *mut f32 {
    (&raw mut B).cast()
}

#[no_mangle]
pub extern "C" fn capacity() -> usize {
    MAX
}

/// Rust 1.89 onwards rejects taking a reference directly to a `static mut`,
/// so go through raw pointers.
unsafe fn slices(len: usize) -> (&'static [f32], &'static [f32]) {
    (
        core::slice::from_raw_parts((&raw const A).cast::<f32>(), len),
        core::slice::from_raw_parts((&raw const B).cast::<f32>(), len),
    )
}

/// One lane at a time.
#[no_mangle]
pub extern "C" fn dot_scalar(len: usize) -> f32 {
    let (a, b) = unsafe { slices(len) };

    let mut sum = 0.0f32;
    for i in 0..len {
        sum += a[i] * b[i];
    }
    sum
}

/// Four lanes at a time with `f32x4`.
#[no_mangle]
pub extern "C" fn dot_simd(len: usize) -> f32 {
    let (a, b) = unsafe { slices(len) };

    let mut acc = f32x4_splat(0.0);
    let chunks = len / 4;

    for i in 0..chunks {
        let va = unsafe { v128_load(a.as_ptr().add(i * 4).cast()) };
        let vb = unsafe { v128_load(b.as_ptr().add(i * 4).cast()) };
        // acc += va * vb, four products at once.
        acc = f32x4_add(acc, f32x4_mul(va, vb));
    }

    // Horizontal add of the four lanes.
    let mut sum = f32x4_extract_lane::<0>(acc)
        + f32x4_extract_lane::<1>(acc)
        + f32x4_extract_lane::<2>(acc)
        + f32x4_extract_lane::<3>(acc);

    // Whatever did not fit in a group of four.
    for i in (chunks * 4)..len {
        sum += a[i] * b[i];
    }
    sum
}
