use agent_stream_kit::{ASKit, AgentFlow};
use tauri::{path::BaseDirectory, AppHandle, Manager, State};
use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_askit::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let askit = app_handle.state::<ASKit>().clone();
                askit_std_agents::register_agents(&askit);
                askit_llm_agents::register_agents(&askit);

                load_settings(&app_handle, &askit).unwrap();

                let flow_path = app_handle
                    .path()
                    .resolve(
                        "resources/flows/chat_simple-ollama.json",
                        BaseDirectory::Resource,
                    )
                    .unwrap();

                let json = std::fs::read_to_string(flow_path).unwrap();
                let flow = AgentFlow::from_json(&json).unwrap();
                askit.add_agent_flow(&flow).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_settings_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_settings(app: &AppHandle, askit: &ASKit) -> Result<(), String> {
    let settings = app.store("settings.json").map_err(|e| e.to_string())?;

    let Some(settings) = settings.get("settings") else {
        return Ok(());
    };

    // OpenAI API Key
    if let Some(openai_key) = settings
        .get("apiKeys")
        .and_then(|v| v.get("openai").cloned())
        .map(|v| v.as_str().unwrap_or_default().to_string())
    {
        if let Some(mut config) = askit.get_global_config("openai_chat") {
            config.set("openai_api_key".to_string(), openai_key.into());
            askit.set_global_config("openai_chat".to_string(), config);
        }
    }

    // Sakura AI Engine API Key
    if let Some(sakura_key) = settings
        .get("apiKeys")
        .and_then(|v| v.get("sakura").cloned())
        .map(|v| v.as_str().unwrap_or_default().to_string())
    {
        if let Some(mut config) = askit.get_global_config("sakura_ai_chat") {
            config.set("sakura_ai_api_key".to_string(), sakura_key.into());
            askit.set_global_config("sakura_ai_chat".to_string(), config);
        }
    }

    // Ollama URL
    if let Some(ollama_url) = settings
        .get("apiKeys")
        .and_then(|v| v.get("ollamaUrl").cloned())
        .map(|v| v.as_str().unwrap_or_default().to_string())
    {
        if let Some(mut config) = askit.get_global_config("ollama_completion") {
            config.set("ollama_url".to_string(), ollama_url.into());
            askit.set_global_config("ollama_completion".to_string(), config);
        }
    }

    Ok(())
}

#[tauri::command]
fn load_settings_cmd(app: AppHandle, askit: State<ASKit>) -> Result<(), String> {
    load_settings(&app, &askit)
}
