// Node harness: fills the module's two static buffers, then checks that the
// scalar and SIMD dot products agree, and times both.
import { readFile } from "node:fs/promises";

const bytes = await readFile(
  new URL("./target/wasm32-unknown-unknown/release/simd.wasm", import.meta.url),
);
const { instance } = await WebAssembly.instantiate(bytes, {});
const ex = instance.exports;

const N = ex.capacity();
const mem = new Float32Array(ex.memory.buffer);
const aOff = ex.buffer_a() / 4;
const bOff = ex.buffer_b() / 4;

for (let i = 0; i < N; i++) {
  mem[aOff + i] = Math.fround(Math.sin(i) * 2);
  mem[bOff + i] = Math.fround(Math.cos(i) * 3);
}

const scalar = ex.dot_scalar(N);
const simd = ex.dot_simd(N);

console.log(`lanes          : ${N} f32 pairs`);
console.log(`dot_scalar     : ${scalar}`);
console.log(`dot_simd       : ${simd}`);
// Lane-wise accumulation sums in a different order, so expect tiny f32 drift.
const drift = Math.abs(scalar - simd) / Math.max(1, Math.abs(scalar));
console.log(`relative drift : ${drift.toExponential(3)} (reassociation, not a bug)`);

const bench = (fn, reps = 20000) => {
  const t0 = performance.now();
  for (let i = 0; i < reps; i++) fn(N);
  return performance.now() - t0;
};
bench(ex.dot_scalar, 2000); // warm up
bench(ex.dot_simd, 2000);
const ts = bench(ex.dot_scalar);
const tv = bench(ex.dot_simd);
console.log(`scalar         : ${ts.toFixed(1)} ms`);
console.log(`simd           : ${tv.toFixed(1)} ms  (${(ts / tv).toFixed(2)}x)`);

if (drift > 1e-4) {
  console.error("FAIL: results disagree by more than f32 reassociation explains");
  process.exit(1);
}
console.log("OK");
