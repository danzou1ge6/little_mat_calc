/* tslint:disable */
/* eslint-disable */
/**
*/
export function intp_init(): void;
/**
* @returns {string}
*/
export function standby_prompt(): string;
/**
* @returns {string}
*/
export function startup_text(): string;
/**
* @param {string} src
* @returns {EvalResult}
*/
export function intp_eval(src: string): EvalResult;
/**
*/
export class EvalResult {
  free(): void;
/**
* @returns {string}
*/
  output(): string;
/**
* @returns {string}
*/
  prompt(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly intp_init: () => void;
  readonly standby_prompt: (a: number) => void;
  readonly startup_text: (a: number) => void;
  readonly __wbg_evalresult_free: (a: number) => void;
  readonly evalresult_output: (a: number, b: number) => void;
  readonly evalresult_prompt: (a: number, b: number) => void;
  readonly intp_eval: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
