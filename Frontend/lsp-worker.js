let text = []
self.onmessage = function(event) {
  const message = event.data;

  switch (message.method) {
    case 'initialize':
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
      text = message.params.textDocument.text.split("\n");
      break;

    case 'textDocument/didChange':
      let changes = message.params.contentChanges;
      if (changes[0].range === undefined) {
        text = changes[0].text.split("\n");
      } else {
        for (let change in changes) {
          text.splice(change.range.start, change.range.end - change.range.start, ...change.text.split("\n"));
        }
      }
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
          value: `# ${getWordAtPosition(message.params.position)}\nThis is a simple hover information.\n\nthird line`,
        },
      };
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

function getWordAtPosition(position) {
  let line = text[position.line];
  let character = position.character;
  const words = line.split(/\s+/);
  for (const word of words) {
    const start = line.indexOf(word);
    const end = start + word.length;
    if (character >= start && character <= end) {
      return word;
    }
  }
  return '';
}
