import init, { fetch as wasmFetch } from "./worker_pkg/ai_chat_wasm.js";
import wasmModule from "./worker_pkg/ai_chat_wasm_bg.wasm";

export default {
  async fetch(request, env, ctx) {
    // 1. Wasm モジュールを初期化
    await init(wasmModule);

    // 2. Rust 側の fetch 関数を確実に呼び出す
    return await wasmFetch(request, env, ctx);
  },
};
