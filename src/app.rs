use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />


                <HydrationScripts options=options />


                <title>"Full-Stack Rust AI Chat"</title>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub is_user: bool,
    pub text: String,
}

#[server(endpoint = "/api")]
pub async fn fetch_ai_response(user_input: String) -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;

    let reply = format!(
        "「{}」と言いましたね.私はWasmとRustで動くAIアシスタントです。",
        user_input
    );
    Ok(reply)
}

#[component]
pub fn App() -> impl IntoView {
    let (messages, set_messages) = create_signal(vec![Message {
        is_user: false,
        text: "Hello! WasmBot.".to_string(),
    }]);

    let (input_text, set_input_text) = create_signal(String::new());

    // 送信処理
    let send_message = create_action(move |input: &String| {
        let input = input.clone();
        async move { fetch_ai_response(input).await }
    });

    create_effect(move |_| {
        if let Some(Ok(ai_reply)) = send_message.value().get() {
            set_messages.update(|msgs| {
                msgs.push(Message {
                    is_user: false,
                    text: ai_reply,
                });
            });
        }
    });

    view! {
        <div class="flex flex-col h-screen bg-slate-900 text-slate-100 font-sans">

            <header class="p-4 bg-slate-800 border-b border-slate-700 shadow-md">
                <h1 class="text-xl font-bold tracking-wide text-cyan-400">"Full-Stack Rust AI Chat"</h1>
            </header>

            <div class="flex-1 overflow-y-auto p-4 space-y-4 max-w-3xl w-full mx-auto">
                {move || messages.get().into_iter().map(|msg| {
                    let bubble_class = if msg.is_user {
                        "bg-cyan-600 text-white ml-auto rounded-br-none"
                    } else {
                        "bg-slate-800 text-slate-100 mr-auto rounded-bl-none"
                    };
                    view! {
                        <div class=format!("max-w-[70%] p-3 rounded-2xl shadow-sm leading-relaxed whitespace-pre-wrap {}", bubble_class)>
                            {msg.text}
                        </div>
                    }
                }).collect::<Vec<_>>()}


                {move || send_message.pending().get().then(|| view! {
                    <div class="bg-slate-800 text-slate-400 mr-auto max-w-[70%] p-3 rounded-2xl rounded-bl-none shadow-sm animate-pulse">
                        "AIが思考中..."
                    </div>
                })}
            </div>


            <div class="p-4 bg-slate-800 border-t border-slate-700">
                <form class="max-w-3xl w-full mx-auto flex gap-2" on:submit=move |ev| {
                    ev.prevent_default();
                    let text = input_text.get();
                    if text.is_empty() { return; }

                    set_messages.update(|msgs| {
                        msgs.push(Message { is_user: true, text: text.clone() });
                    });

                    send_message.dispatch(text);
                    set_input_text.set(String::new());
                }>
                    <input
                        type="text"
                        placeholder="メッセージを入力..."
                        class="flex-1 p-3 rounded-xl bg-slate-700 border border-slate-600 focus:outline-none focus:border-cyan-500 text-slate-100 transition-colors"
                        prop:value=input_text
                        on:input=move |ev| set_input_text.set(event_target_value(&ev))
                        disabled=move || send_message.pending().get()
                    />
                    <button
                        type="submit"
                        class="px-6 py-3 bg-cyan-600 hover:bg-cyan-500 disabled:bg-slate-600 text-white font-medium rounded-xl shadow-md transition-colors"
                        disabled=move || send_message.pending().get()
                    >
                        "送信"
                    </button>
                </form>
            </div>
        </div>
    }
}
