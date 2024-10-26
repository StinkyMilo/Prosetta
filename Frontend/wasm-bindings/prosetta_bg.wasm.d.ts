/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function get_heap_size(): number;
export function __wbg_parserrunner_free(a: number, b: number): void;
export function __wbg_parserrunnerdata_free(a: number, b: number): void;
export function parserrunner_new(): number;
export function parserrunner_run_to_completion(a: number, b: number, c: number): number;
export function parserrunnerdata_get_javascript(a: number, b: number): void;
export function parserrunnerdata_get_html(a: number, b: number): void;
export function parserrunnerdata_get_highlights(a: number, b: number): void;
export function __wbg_highlight_free(a: number, b: number): void;
export function __wbg_get_highlight_line(a: number): number;
export function __wbg_set_highlight_line(a: number, b: number): void;
export function __wbg_get_highlight_index(a: number): number;
export function __wbg_set_highlight_index(a: number, b: number): void;
export function __wbg_get_highlight_length(a: number): number;
export function __wbg_set_highlight_length(a: number, b: number): void;
export function __wbg_get_highlight_color(a: number, b: number): void;
export function __wbg_set_highlight_color(a: number, b: number, c: number): void;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number, d: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number, c: number): void;
