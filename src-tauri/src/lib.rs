mod parents;

use parents::dispatch::dispatcher_cleaner;

#[tauri::command]
fn start_compression(app: tauri::AppHandle, algorithm: Option<String>, flags: Vec<String>, input_path: String, output_path: String) -> Result<(), String> {
    dispatcher_cleaner::dispatch(app, algorithm, flags, input_path, output_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_compression])
    .setup(|app| {
      app.handle().plugin(tauri_plugin_dialog::init())?;
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

