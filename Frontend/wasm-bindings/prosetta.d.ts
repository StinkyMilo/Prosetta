/* tslint:disable */
/* eslint-disable */
export function get_heap_size(): number;
export enum Import {
  List = 0,
  Func = 1,
  Graph = 2,
  Frame = 3,
  Trig = 4,
  Rand = 5,
  Stamp = 6,
  Not = 7,
}
export class Highlight {
  private constructor();
  free(): void;
  line: number;
  index: number;
  length: number;
  color: string[];
}
export class ParserRunner {
  free(): void;
  constructor();
  run_to_completion(source: string): ParserRunnerData;
}
export class ParserRunnerData {
  private constructor();
  free(): void;
  get_javascript(): string;
  get_html(): string;
  get_highlights(): Highlight[];
  get_imports(): any[];
  get_triggers(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_highlight_free: (a: number, b: number) => void;
  readonly __wbg_get_highlight_line: (a: number) => number;
  readonly __wbg_set_highlight_line: (a: number, b: number) => void;
  readonly __wbg_get_highlight_index: (a: number) => number;
  readonly __wbg_set_highlight_index: (a: number, b: number) => void;
  readonly __wbg_get_highlight_length: (a: number) => number;
  readonly __wbg_set_highlight_length: (a: number, b: number) => void;
  readonly __wbg_get_highlight_color: (a: number, b: number) => void;
  readonly __wbg_set_highlight_color: (a: number, b: number, c: number) => void;
  readonly get_heap_size: () => number;
  readonly __wbg_parserrunner_free: (a: number, b: number) => void;
  readonly __wbg_parserrunnerdata_free: (a: number, b: number) => void;
  readonly parserrunner_new: () => number;
  readonly parserrunner_run_to_completion: (a: number, b: number, c: number) => number;
  readonly parserrunnerdata_get_javascript: (a: number, b: number) => void;
  readonly parserrunnerdata_get_html: (a: number, b: number) => void;
  readonly parserrunnerdata_get_highlights: (a: number, b: number) => void;
  readonly parserrunnerdata_get_imports: (a: number, b: number) => void;
  readonly parserrunnerdata_get_triggers: (a: number, b: number) => void;
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
