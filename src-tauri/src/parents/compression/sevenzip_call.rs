use tauri::AppHandle;
use std::path::{Path, PathBuf};
use crate::parents::utils;

pub fn run(_app: AppHandle, flags: Vec<String>, input_path: String, output_path: String) -> Result<(), String> {
    let input = Path::new(&input_path);
    let file_name_raw = input.file_name().ok_or("Invalid input")?.to_string_lossy().to_string();

    let mut base_name = file_name_raw.clone();
    if let Some(stripped) = base_name.strip_prefix(".tmp_") {
        base_name = stripped.to_string();
    }

    let timestamp = utils::get_timestamp();
    let output_file_name = format!("{}_{}.7z", timestamp, base_name);
    
    let mut final_output_path = PathBuf::from(&output_path);
    final_output_path.push(output_file_name);
    let output_path_str = final_output_path.to_string_lossy().to_string();

    println!("Executing 7zip with flags: {:?}, input: {}, output: {}", flags, input_path, output_path_str);
    Ok(())
}
