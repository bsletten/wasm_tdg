// Headless check for the browser page.
//
// The failure this guards against is nasty: Emscripten's generated threads.js
// is a classic script full of top-level `var` declarations. If the page
// declares a global `const`/`let` with any of the same names, threads.js dies
// with "Identifier 'x' has already been declared" *before executing a single
// line* -- so the page looks blank and the only clue is a console SyntaxError.
//
// This evaluates the page's inline script and threads.js in one shared V8
// global, exactly as a browser would, with just enough DOM to get going.
import { readFileSync } from "node:fs";
import vm from "node:vm";
import assert from "node:assert";

// Once threads.js starts running it will try to fetch threads.wasm through our
// stub and reject. That is expected and not what this test is about.
process.on("unhandledRejection", () => {});
process.on("uncaughtException", () => {});

const html = readFileSync(new URL("./index.html", import.meta.url), "utf8");
const gen = readFileSync(new URL("./threads.js", import.meta.url), "utf8");

// The page's inline classic scripts, in order (skip <script src=...>).
const inline = [...html.matchAll(/<script(?![^>]*\bsrc=)[^>]*>([\s\S]*?)<\/script>/g)].map(
  (m) => m[1],
);
assert.ok(inline.length > 0, "no inline script found in index.html");

const el = () => ({
  textContent: "",
  innerHTML: "",
  set src(v) { this._src = v; },
  get src() { return this._src; },
  appendChild() {},
});

const workers = [];
const ctx = {
  console,
  crossOriginIsolated: true,
  SharedArrayBuffer,
  WebAssembly,
  Atomics,
  TextDecoder,
  performance,
  setTimeout,
  URL,
  document: {
    getElementById: el,
    createElement: el,
    body: { appendChild() {} },
    // A browser sets document.currentScript while a classic script executes,
    // including a dynamically inserted one. Emscripten reads it to find the
    // URL it must hand to `new Worker(...)` for each pthread.
    currentScript: { src: "http://localhost/threads.js" },
  },
  addEventListener() {},
  fetch: () => Promise.reject(new Error("stub")),
  XMLHttpRequest: class {},
  Worker: class {
    constructor(url, opts) { workers.push({ url, opts }); }
    postMessage() {}
    terminate() {}
  },
};
ctx.window = ctx;
ctx.globalThis = ctx;
vm.createContext(ctx);

// 1. The page's own script must evaluate.
for (const [i, src] of inline.entries()) {
  try {
    vm.runInContext(src, ctx, { filename: `index.html#inline${i}` });
  } catch (e) {
    console.error(`FAIL: page inline script ${i} threw ${e.constructor.name}: ${e.message}`);
    process.exit(1);
  }
}

// 2. threads.js must evaluate in that same global without a declaration clash.
try {
  vm.runInContext(gen, ctx, { filename: "threads.js" });
} catch (e) {
  if (e instanceof SyntaxError) {
    console.error(`FAIL: threads.js could not evaluate -- ${e.message}`);
    console.error("      The page is declaring a global const/let that collides");
    console.error("      with one of Emscripten's top-level vars.");
    process.exit(1);
  }
  // Anything else means it started running and hit our stubs, which is fine.
  console.log(`threads.js began executing (stopped at a stub: ${e.message})`);
}

// 3. It must have tried to spawn workers from a real URL, not `undefined`.
assert.ok(workers.length > 0, "threads.js never allocated a pthread worker");
for (const w of workers) {
  assert.ok(
    typeof w.url === "string" && w.url.length > 0 && w.url !== "undefined",
    `worker created with a bad script URL: ${w.url}`,
  );
  assert.strictEqual(w.opts?.name, "em-pthread", "worker is not an em-pthread worker");
}

console.log("ok  page + threads.js share a global cleanly");
console.log(`ok  ${workers.length} pthread workers allocated from ${workers[0].url}`);
console.log("ok  no global const/let in the page collides with Emscripten's vars");
process.exit(0);
