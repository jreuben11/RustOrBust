async function init() {
    // instantiate memory buffer to pass to WASM
    const jsmemory = new WebAssembly.Memory({initial: 1}) 

    // object to import into WASM
    const importObject = {
        console: {
          log: () => {
            console.log("Just logging something!");
          },
          error: () => {
            console.log("I am just error");
          }
        },
        js: {
            mem: jsmemory
        },
      }

    

    // instantiate WASM from byte array buffer:
    // const byteArray = new Int8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x73, 0x75, 0x6d, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b, 0x00, 0x18, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x73, 0x75, 0x6d, 0x02, 0x09, 0x01, 0x00, 0x02, 0x00, 0x01, 0x61, 0x01, 0x01, 0x62]);
    // const wasm = await WebAssembly.instantiate(byteArray.buffer);

    // fetch WASM file into array buffer and instantiant WebAssembly object:
    const response = await fetch("sum.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, importObject);

    // call WASM function
    const sumFunction = wasm.instance.exports.sum;
    // debugger;
    const result = sumFunction(100, 300);
    console.log(result);

    // access memory buffer:
    const wasmMemory = wasm.instance.exports.mem;
    // const uint8Array = new Uint8Array(wasmMemory.buffer, 0, 2);
    const uint8Array = new Uint8Array(jsmemory.buffer, 0, 2);
    const hiText = new TextDecoder().decode(uint8Array);
    console.log(hiText);

}
init();