use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use worker::{web_sys, worker_sys};

#[wasm_bindgen(js_name = fetch)]
pub async fn fetch(
    js_req: web_sys::Request,
    js_env: JsValue,
    js_ctx: JsValue,
) -> Result<web_sys::Response, JsValue> {
    // 1. 各生データを worker クレートが扱いやすい型に変換
    let req = worker::Request::from(js_req);
    let env = worker::Env::from(js_env);
    let sys_ctx: worker_sys::Context = js_ctx.into();
    let ctx = worker::Context::new(sys_ctx);

    // 2. 実際のロジックを実行する関数へ渡す（またはここに直接処理を書く）
    match main_logic(req, env, ctx).await {
        Ok(res) => Ok(web_sys::Response::from(res)),
        Err(e) => Err(JsValue::from_str(&format!("Worker Error: {:?}", e))),
    }
}

// 実際の処理を担当する非同期関数（例）
async fn main_logic(
    req: worker::Request,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<worker::Response> {
    // ここにAIチャットのWasmロジックや、Leptosなどのルーティング、
    // あるいはOpenAI/Cloudflare AI等のエンドポイント呼び出しを記述します。

    // 単純な疎通確認用のレスポンス例:
    worker::Response::ok(&format!("Hello from Rust Wasm! Path: {}", req.path()))
}
