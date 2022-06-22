/* tslint:disable */
/* eslint-disable */
/**
* @param {string} cf_var
* @param {string} cf_breakpoints
* @returns {string}
*/
export function add_init_css(cf_var: string, cf_breakpoints: string): string;
/**
* @param {string} html
* @param {string} cf_breakpoints
* @returns {string}
*/
export function atomic_css_compile_from_html(html: string, cf_breakpoints: string): string;
/**
* @param {string} classes
* @param {string} cf_breakpoints
* @returns {string}
*/
export function atomic_css_compile(classes: string, cf_breakpoints: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly add_init_css: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly atomic_css_compile_from_html: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly atomic_css_compile: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
