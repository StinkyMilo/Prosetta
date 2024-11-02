var has_initialized = false;
var preinit_queue = [];
var parser;

onmessage = async function(e) {
  let command = e.data.command;
  let data = e.data.data;
  if (!has_initialized && command != "initialize") {
    preinit_queue.push(e);
    return;
  }
  switch (command) {
    case "initialize":
      const wasm = await import('./wasm-bindings/prosetta.js');
      let wasm_instance = await wasm.default();
      // setInterval(() =>
      //   console.log("wasm is using",
      //     wasm_instance.memory.buffer.byteLength, "total and ", wasm.get_heap_size(), " on heap"), 1000);
      parser = new wasm.ParserRunner();
      has_initialized = true;
      while (preinit_queue.length > 0) {
        await this.onmessage(preinit_queue.pop());
      }
      break;
    case "changed":
      let parsedData = parser.run_to_completion(data);
      msg_main("parsed", { js: parsedData.get_javascript(), hl: convert_highlights(parsedData.get_highlights()), imports: parsedData.get_imports() });
      break;
  }
}

function msg_main(command, data) {
  postMessage({ command: command, data: data });
}

function convert_highlights(highlights) {
  let js_hl = [];
  for (let hl of highlights) {
    js_hl.push({ line: hl.line, index: hl.index, length: hl.length, color: hl.color });
  }
  return js_hl;
}
