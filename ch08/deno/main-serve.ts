// Deno has a built-in HTTP server (`Deno.serve`), so unlike earlier versions of
// this example there is nothing to import from the standard library.
const wasmCode = await Deno.readFile("./a.out.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const add = wasmInstance.exports.add as CallableFunction;

Deno.serve({ hostname: "0.0.0.0", port: 9000 }, () => {
  return new Response("2 + 3 = " + add(2, 3) + "\n");
});
