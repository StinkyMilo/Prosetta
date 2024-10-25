import { EventEmitter } from "events"
import * as marked from 'marked';
import * as lsProtocol from "vscode-languageserver-protocol"

class LspWwConnection extends EventEmitter {
  isConnected = false
  isInitialized = false
  documentVersion = 0

  constructor(options) {
    super()
    this.documentInfo = options
  }

  rpc(method, params) {
    return {
      jsonrpc: "2.0",
      id: 1,
      method: method,
      params: params
    };
  }

  awaitWorkerMessage() {
    return new Promise((resolve) => {
      this.worker.onmessage = (event) => {
        resolve(event.data);
      };
    });
  }

  /**
   * Initialize a connection over a web socket that speaks the LSP protocol
   */
  connect(worker) {
    this.worker = worker
    this.isConnected = true;

    //     rpc.listen({
    //       webSocket: this.worker,
    //       logger: new ConsoleLogger(),
    //       onConnection: connection => {
    //         connection.listen()
    //         this.isConnected = true

    //         this.connection = connection
    //         this.sendInitialize()

    //         this.connection.onNotification(
    //           "textDocument/publishDiagnostics",
    //           params => {
    //             this.emit("diagnostic", params)
    //           }
    //         )

    //         this.connection.onNotification("window/showMessage", params => {
    //           this.emit("logging", params)
    //         })

    //         this.connection.onRequest("client/registerCapability", params => {
    //           params.registrations.forEach(capabilityRegistration => {
    //             this.serverCapabilities = registerServerCapability(
    //               this.serverCapabilities,
    //               capabilityRegistration
    //             )
    //           })

    //           this.emit("logging", params)
    //         })

    //         this.connection.onRequest("client/unregisterCapability", params => {
    //           params.unregisterations.forEach(capabilityUnregistration => {
    //             this.serverCapabilities = unregisterServerCapability(
    //               this.serverCapabilities,
    //               capabilityUnregistration
    //             )
    //           })

    //           this.emit("logging", params)
    //         })

    //         this.connection.onRequest("window/showMessageRequest", params => {
    //           this.emit("logging", params)
    //         })

    //         this.connection.onError(e => {
    //           this.emit("error", e)
    //         })

    //         this.connection.onClose(() => {
    //           this.isConnected = false
    //         })
    //       }
    //     })

    return this
  }

  close() {
    if (this.connection) {
      this.connection.dispose()
    }
    this.worker.close()
  }

  getDocumentUri() {
    return this.documentInfo.documentUri
  }

  async sendInitialize() {
    if (!this.isConnected) {
      return
    }

    const message = {
      capabilities: {
        textDocument: {
          hover: {
            dynamicRegistration: true,
            contentFormat: ["plaintext", "markdown"]
          },

          synchronization: {
            dynamicRegistration: true,
            willSave: false,
            didSave: false,
            willSaveWaitUntil: false
          },

          completion: {
            dynamicRegistration: true,
            completionItem: {
              snippetSupport: false,
              commitCharactersSupport: true,
              documentationFormat: ["plaintext", "markdown"],
              deprecatedSupport: false,
              preselectSupport: false
            },
            contextSupport: false
          },

          signatureHelp: {
            dynamicRegistration: true,
            signatureInformation: {
              documentationFormat: ["plaintext", "markdown"]
            }
          },

          declaration: {
            dynamicRegistration: true,
            linkSupport: true
          },

          definition: {
            dynamicRegistration: true,
            linkSupport: true
          },

          typeDefinition: {
            dynamicRegistration: true,
            linkSupport: true
          },

          implementation: {
            dynamicRegistration: true,
            linkSupport: true
          }
        },

        workspace: {
          didChangeConfiguration: {
            dynamicRegistration: true
          }
        }
      },
      initializationOptions: null,
      processId: null,
      rootUri: this.documentInfo.rootUri,
      workspaceFolders: null
    }

    this.worker.postMessage(this.rpc("initialize", message));
    const msg = await this.awaitWorkerMessage();
    this.isInitialized = true;
    this.serverCapabilities = msg.capabilities;
    const textDocumentMessage = {
      textDocument: {
        uri: this.documentInfo.documentUri,
        languageId: this.documentInfo.languageId,
        text: this.documentInfo.documentText(),
        version: this.documentVersion
      }
    };
    this.worker.postMessage(this.rpc("textDocument/didOpen", textDocumentMessage));
    this.sendChange();
  }

  sendChange() {
    if (!this.isConnected) {
      return
    }
    const textDocumentChange = {
      textDocument: {
        uri: this.documentInfo.documentUri,
        version: this.documentVersion
      },
      contentChanges: [
        {
          text: this.documentInfo.documentText()
        }
      ]
    }
    this.worker.postMessage(this.rpc("textDocument/didChange", textDocumentChange))
    this.documentVersion++;
  }

  async getHoverTooltip(location) {
    if (!this.isInitialized) {
      return
    }
    this.worker.postMessage(this.rpc("textDocument/hover", {
      textDocument: {
        uri: this.documentInfo.documentUri
      },

      position: {
        line: location.line,
        character: location.ch
      }
    }))
    const msg = await this.awaitWorkerMessage();
    msg.result.contents.type = "object";
    msg.result.contents.value = marked.parse(msg.result.contents.value);
    this.emit("hover", msg.result);
  }

  getCompletion(location, token, triggerCharacter, triggerKind) {
    if (!this.isConnected) {
      return
    }
    if (
      !(this.serverCapabilities && this.serverCapabilities.completionProvider)
    ) {
      return
    }

    this.connection
      .sendRequest("textDocument/completion", {
        textDocument: {
          uri: this.documentInfo.documentUri
        },

        position: {
          line: location.line,
          character: location.ch
        },

        context: {
          triggerKind: triggerKind || lsProtocol.CompletionTriggerKind.Invoked,
          triggerCharacter
        }
      })
      .then(params => {
        if (!params) {
          this.emit("completion", params)
          return
        }
        this.emit("completion", "items" in params ? params.items : params)
      })
  }

  getDetailedCompletion(completionItem) {
    if (!this.isConnected) {
      return
    }
    this.connection
      .sendRequest("completionItem/resolve", completionItem)
      .then(result => {
        this.emit("completionResolved", result)
      })
  }

  getSignatureHelp(location) {
    if (!this.isConnected) {
      return
    }
    if (
      !(
        this.serverCapabilities && this.serverCapabilities.signatureHelpProvider
      )
    ) {
      return
    }

    const code = this.documentInfo.documentText()
    const lines = code.split("\n")
    const typedCharacter = lines[location.line][location.ch]

    if (
      this.serverCapabilities.signatureHelpProvider &&
      !this.serverCapabilities.signatureHelpProvider.triggerCharacters.indexOf(
        typedCharacter
      )
    ) {
      // Not a signature character
      return
    }

    this.connection
      .sendRequest("textDocument/signatureHelp", {
        textDocument: {
          uri: this.documentInfo.documentUri
        },

        position: {
          line: location.line,
          character: location.ch
        }
      })
      .then(params => {
        this.emit("signature", params)
      })
  }

  /**
   * Request the locations of all matching document symbols
   */
  getDocumentHighlights(location) {
    if (!this.isConnected) {
      return
    }
    if (
      !(
        this.serverCapabilities &&
        this.serverCapabilities.documentHighlightProvider
      )
    ) {
      return
    }

    this.connection
      .sendRequest("textDocument/documentHighlight", {
        textDocument: {
          uri: this.documentInfo.documentUri
        },

        position: {
          line: location.line,
          character: location.ch
        }
      })
      .then(params => {
        this.emit("highlight", params)
      })
  }

  /**
   * Request a link to the definition of the current symbol. The results will not be displayed
   * unless they are within the same file URI
   */
  getDefinition(location) {
    if (!this.isConnected || !this.isDefinitionSupported()) {
      return
    }
    this.connection
      .sendRequest("textDocument/definition", {
        textDocument: {
          uri: this.documentInfo.documentUri
        },

        position: {
          line: location.line,
          character: location.ch
        }
      })
      .then(result => {
        this.emit("goTo", result)
      })
  }

  /**
   * Request a link to the type definition of the current symbol. The results will not be displayed
   * unless they are within the same file URI
   */
  getTypeDefinition(location) {
    if (!this.isConnected || !this.isTypeDefinitionSupported()) {
      return
    }

    this.connection
      .sendRequest("textDocument/typeDefinition", {
        textDocument: {
          uri: this.documentInfo.documentUri
        },

        position: {
          line: location.line,
          character: location.ch
        }
      })
      .then(result => {
        this.emit("goTo", result)
      })
  }

  /**
   * Request a link to the implementation of the current symbol. The results will not be displayed
   * unless they are within the same file URI
   */
  getImplementation(location) {
    if (!this.isConnected || !this.isImplementationSupported()) {
      return
    }

    this.connection
      .sendRequest("textDocument/implementation", {
        textDocument: {
          uri: this.documentInfo.documentUri
        },

        position: {
          line: location.line,
          character: location.ch
        }
      })
      .then(result => {
        this.emit("goTo", result)
      })
  }

  /**
   * Request a link to all references to the current symbol. The results will not be displayed
   * unless they are within the same file URI
   */
  getReferences(location) {
    if (!this.isConnected || !this.isReferencesSupported()) {
      return
    }

    this.connection
      .sendRequest("textDocument/references", {
        textDocument: {
          uri: this.documentInfo.documentUri
        },

        position: {
          line: location.line,
          character: location.ch
        }
      })
      .then(result => {
        this.emit("goTo", result)
      })
  }

  /**
   * The characters that trigger completion automatically.
   */
  getLanguageCompletionCharacters() {
    if (!this.isConnected) {
      return
    }
    if (
      !(
        this.serverCapabilities &&
        this.serverCapabilities.completionProvider &&
        this.serverCapabilities.completionProvider.triggerCharacters
      )
    ) {
      return []
    }
    return this.serverCapabilities.completionProvider.triggerCharacters
  }

  /**
   * The characters that trigger signature help automatically.
   */
  getLanguageSignatureCharacters() {
    if (!this.isConnected) {
      return
    }
    if (
      !(
        this.serverCapabilities &&
        this.serverCapabilities.signatureHelpProvider &&
        this.serverCapabilities.signatureHelpProvider.triggerCharacters
      )
    ) {
      return []
    }
    return this.serverCapabilities.signatureHelpProvider.triggerCharacters
  }

  /**
   * Does the server support go to definition?
   */
  isDefinitionSupported() {
    return !!(
      this.serverCapabilities && this.serverCapabilities.definitionProvider
    )
  }

  /**
   * Does the server support go to type definition?
   */
  isTypeDefinitionSupported() {
    return !!(
      this.serverCapabilities && this.serverCapabilities.typeDefinitionProvider
    )
  }

  /**
   * Does the server support go to implementation?
   */
  isImplementationSupported() {
    return !!(
      this.serverCapabilities && this.serverCapabilities.implementationProvider
    )
  }

  /**
   * Does the server support find all references?
   */
  isReferencesSupported() {
    return !!(
      this.serverCapabilities && this.serverCapabilities.referencesProvider
    )
  }
}

const ServerCapabilitiesProviders = {
  "textDocument/hover": "hoverProvider",
  "textDocument/completion": "completionProvider",
  "textDocument/signatureHelp": "signatureHelpProvider",
  "textDocument/definition": "definitionProvider",
  "textDocument/typeDefinition": "typeDefinitionProvider",
  "textDocument/implementation": "implementationProvider",
  "textDocument/references": "referencesProvider",
  "textDocument/documentHighlight": "documentHighlightProvider",
  "textDocument/documentSymbol": "documentSymbolProvider",
  "textDocument/workspaceSymbol": "workspaceSymbolProvider",
  "textDocument/codeAction": "codeActionProvider",
  "textDocument/codeLens": "codeLensProvider",
  "textDocument/documentFormatting": "documentFormattingProvider",
  "textDocument/documentRangeFormatting": "documentRangeFormattingProvider",
  "textDocument/documentOnTypeFormatting": "documentOnTypeFormattingProvider",
  "textDocument/rename": "renameProvider",
  "textDocument/documentLink": "documentLinkProvider",
  "textDocument/color": "colorProvider",
  "textDocument/foldingRange": "foldingRangeProvider",
  "textDocument/declaration": "declarationProvider",
  "textDocument/executeCommand": "executeCommandProvider"
}

function registerServerCapability(serverCapabilities, registration) {
  const serverCapabilitiesCopy = JSON.parse(JSON.stringify(serverCapabilities))
  const { method, registerOptions } = registration
  const providerName = ServerCapabilitiesProviders[method]

  if (providerName) {
    if (!registerOptions) {
      serverCapabilitiesCopy[providerName] = true
    } else {
      serverCapabilitiesCopy[providerName] = Object.assign(
        {},
        JSON.parse(JSON.stringify(registerOptions))
      )
    }
  } else {
    throw new Error("Could not register server capability.")
  }

  return serverCapabilitiesCopy
}

function unregisterServerCapability(serverCapabilities, unregistration) {
  const serverCapabilitiesCopy = JSON.parse(JSON.stringify(serverCapabilities))
  const { method } = unregistration
  const providerName = ServerCapabilitiesProviders[method]

  delete serverCapabilitiesCopy[providerName]

  return serverCapabilitiesCopy
}

export default LspWwConnection
