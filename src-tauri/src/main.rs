#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// The only bridge between the web UI and the Rust calculation engine.
#[tauri::command]
fn compute(input: serde_json::Value) -> Result<serde_json::Value, String> {
    let res = engine::compute_value(input)?;
    Ok(serde_json::json!({
        "mode": "free",
        "full": true,
        "teach": false,
        "cfg": {
            "modes": { "tP": "open", "t7": "open", "t8": "open", "t9": "open" },
            "tol": 3
        },
        "res": res
    }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compute])
        .run(tauri::generate_context!())
        .expect("error while running VENT");
}
