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
            "modes": { "tP": "off", "t7": "off", "t8": "off", "t9": "off" },
            "tol": 3
        },
        "res": res
    }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compute])
        .run(tauri::generate_context!())
        .expect("error while running DH-VENT");
}
