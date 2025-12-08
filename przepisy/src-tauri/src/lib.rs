
#[path = "../../src/models.rs"]
pub mod models;

pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![commands::fetch_recipes_backend, commands::fetch_recipes_backend_by_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}