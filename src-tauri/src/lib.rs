#[tauri::command]
fn start_compression(algorithm: Option<String>, flags: Vec<String>, input_path: String, output_path: String) {
    println!("Rust received: algorithm={:?}, flags={:?}, input_path={}, output_path={}", algorithm, flags, input_path, output_path);
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
