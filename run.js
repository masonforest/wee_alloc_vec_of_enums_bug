const fs = require("fs");

async function run() {
  file = fs.readFileSync("target/wasm32-unknown-unknown/debug/wee_alloc_vec_of_enums_bug.wasm");
  var {instance} = await WebAssembly.instantiate(file, {
  env:{
    rust_begin_unwind: () => null,
  }
  });

  writeMemory = (buffer, offset = 0) => {
    var memory = new Uint8Array(instance.exports.memory.buffer, offset, buffer.length);
    buffer.forEach((value, index) => memory[index]= value);
  }

  readMemory= (offset, length) => {
    return new Uint8Array(instance.exports.memory.buffer, offset, length);
  }

  instance.exports.run();
}

run();
