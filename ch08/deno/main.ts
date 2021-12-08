const wasmCode = await Deno.readFile("./a.out.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const add = wasmInstance.exports.add as CallableFunction

console.log("2 + 3 =  " + add(2,3));
