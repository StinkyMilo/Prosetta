import { Registration, ServerCapabilities, Unregistration } from 'vscode-languageserver-protocol';
declare function registerServerCapability(serverCapabilities: ServerCapabilities, registration: Registration): ServerCapabilities;
declare function unregisterServerCapability(serverCapabilities: ServerCapabilities, unregistration: Unregistration): ServerCapabilities;
export { registerServerCapability, unregisterServerCapability, };
