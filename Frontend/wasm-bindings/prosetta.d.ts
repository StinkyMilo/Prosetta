/* tslint:disable */
/* eslint-disable */
/**
*/
export class Highlight {
  free(): void;
}
/**
*/
export class ParserRunner {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} source
* @returns {ParserRunnerData}
*/
  run_to_completion(source: string): ParserRunnerData;
}
/**
*/
export class ParserRunnerData {
  free(): void;
/**
* @returns {string}
*/
  get_javascript(): string;
/**
* @returns {string}
*/
  get_html(): string;
/**
* @returns {(Highlight)[]}
*/
  get_lines(): (Highlight)[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_highlight_free: (a: number, b: number) => void;
  readonly __wbg_parserrunner_free: (a: number, b: number) => void;
  readonly __wbg_parserrunnerdata_free: (a: number, b: number) => void;
  readonly parserrunner_new: () => number;
  readonly parserrunner_run_to_completion: (a: number, b: number, c: number) => number;
  readonly parserrunnerdata_get_javascript: (a: number, b: number) => void;
  readonly parserrunnerdata_get_html: (a: number, b: number) => void;
  readonly parserrunnerdata_get_lines: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
