/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function add_init_css(): string;
/**
* @param {string} html
* @returns {string}
*/
export function atomic_css_compile_from_html(html: string): string;
/**
* @param {string} classes
* @returns {string}
*/
export function atomic_css_compile(classes: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly add_init_css: (a: number) => void;
  readonly atomic_css_compile_from_html: (a: number, b: number, c: number) => void;
  readonly atomic_css_compile: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
