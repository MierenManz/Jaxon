const mod = await WebAssembly.instantiate(Deno.readFileSync("./target/wasm32-unknown-unknown/release/jerry.wasm"));
const len = mod.instance.exports.len();
const ptr = mod.instance.exports.create();

const mem = new Uint8Array(mod.instance.exports.memory.buffer);
const slice = mem.slice(ptr, ptr + len);
console.log(slice);