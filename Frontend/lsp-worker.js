self.onmessage = function(event) {
  const message = event.data;
  console.log("in webworker: ", message);

  switch (message.method) {
    case 'initialize':
      console.log("init ls");
      postMessage({
        jsonrpc: "2.0",
        id: message.id,
        result: {
          capabilities: {
            textDocumentSync: 1,
            completionProvider: {
              resolveProvider: false,
            },
            hoverProvider: true,
          }
        }
      });
      break;

    case 'textDocument/didOpen':
    case 'textDocument/didChange':
      const text = message.params.textDocument.text;
      console.log("Document content: ", text);
      break;

    case 'textDocument/completion':
      const completions = [
        {
          label: 'console.log',
          kind: 1,
          insertText: 'console.log()',
        },
        {
          label: 'function',
          kind: 3,
          insertText: 'function ',
        }
      ];
      postMessage({
        jsonrpc: "2.0",
        id: message.id,
        result: { items: completions }
      });
      break;

    case 'textDocument/hover':
      const hoverResult = {
        contents: {
          kind: 'markdown',
          value: 'This is a simple hover information.',
        },
      };
      console.log("hover!");
      postMessage({
        jsonrpc: "2.0",
        id: message.id,
        result: hoverResult
      });
      break;

    default:
      console.warn("Unknown message", message);
  }
};
