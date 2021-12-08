import { serve } from "https://deno.land/std@0.93.0/http/server.ts";

const wasmCode = await Deno.readFile("./a.out.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const add = wasmInstance.exports.add as CallableFunction

const server = serve({ hostname: "0.0.0.0", port: 9000 });
console.log(`HTTP webserver running.  Access it at:  http://localhost:9000/`);

for await (const request of server) {
  let bodyContent = "2 + 3 = " + add(2,3);
  request.respond({ status: 200, body: bodyContent });
}


