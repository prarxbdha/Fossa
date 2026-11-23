mod commands;
mod state;

use state::SharedState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SharedState::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_url,
            commands::set_current_url,
            commands::fetch_document,
            commands::render_document,
            commands::read_fixture
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
