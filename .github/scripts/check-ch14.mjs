// Behavioural check for the ch14 AssemblyScript modules. Run from ch14/.
import fs from "node:fs/promises";
import loader from "@assemblyscript/loader";

const inst = async (p, imports) =>
  (await WebAssembly.instantiate(await fs.readFile(p), imports)).instance;

let failures = 0;
const check = (name, actual, expected) => {
  const ok = String(actual) === String(expected);
  console.log(`${ok ? "ok  " : "FAIL"} ${name}: ${actual}${ok ? "" : ` (expected ${expected})`}`);
  if (!ok) failures++;
};

// hello
check("hello/add(12,30)", (await inst("hello/hello.wasm")).exports.add(12, 30), 42);

// hello-imp: the module imports a JS function under the `hello` namespace
let logged;
const imp = await inst("hello-imp/hello.wasm", { hello: { log: (v) => (logged = v) } });
imp.exports.addAndLog(12, 30);
check("hello-imp log", logged, 42);

// as-mem: round-trip a byte through the exported memory
const mem = await inst("as-mem/mem.wasm");
const u8 = new Uint8Array(mem.exports.memory.buffer);
const loc = mem.exports.whereToStore();
check("as-mem whereToStore()", loc, 100);
u8[loc] = 123;
check("as-mem round-trip", mem.exports.readFromLocation(loc), 123);

// stdlib
const std = await inst("stdlib/stdlib.wasm");
check("stdlib diameter(2)", std.exports.diameter(2.0), 4);
check("stdlib circumference(2)", std.exports.circumference(2.0).toFixed(4), "12.5664");
check("stdlib area(2)", std.exports.area(2.0).toFixed(4), "12.5664");

// loader: strings across the boundary (needs --exportRuntime)
const { exports } = await loader.instantiate(await fs.readFile("loader/loader.wasm"));
const { concat, __newString, __getString } = exports;
check("loader concat", __getString(concat(__newString("Hello, "), __newString("world!"))), "Hello, world!");

process.exit(failures === 0 ? 0 : 1);
