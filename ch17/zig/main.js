const fs = require('fs');
const source = fs.readFileSync("./howOld.wasm");
const typedArray = new Uint8Array(source);

WebAssembly.instantiate(typedArray).then(result => {
  const howOld = result.instance.exports.howOld;
  let age = howOld(2021, 2000);
  console.log('You are: ' + age);
});
