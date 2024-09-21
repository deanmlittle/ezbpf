import * as wasm from "./ezbpf_wasm_bg.wasm";
import { __wbg_set_wasm } from "./ezbpf_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./ezbpf_wasm_bg.js";
