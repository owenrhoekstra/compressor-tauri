use std::process::Command;
use std::path::{Path, PathBuf};

/// Creates an intermediate tar archive of the input.
/// Prefixes the filename with a '.' to make it invisible in Finder (macOS).
pub fn pack(input_path: &str) -> Result<String, String> {
    let input = Path::new(input_path);
    if !input.exists() {
        return Err(format!("Input path does not exist: {}", input_path));
    }

    let parent_dir = input.parent().ok_or("Could not determine parent directory")?;
    let file_name = input.file_name().ok_or("Could not determine file name")?.to_string_lossy();
    
    // Prefix with . to hide it on macOS/Linux
    // Using .tmp_ as a prefix to clearly identify it as a temporary file
    let tar_name = format!(".tmp_{}.tar", file_name);
    
    let mut tar_path = PathBuf::from(parent_dir);
    tar_path.push(tar_name);
    let tar_path_str = tar_path.to_string_lossy().to_string();

    println!("Creating intermediate tar: {}", tar_path_str);

    // tar -cf <output> -C <parent> <filename>
    let status = Command::new("tar")
        .arg("-cf")
        .arg(&tar_path_str)
        .arg("-C")
        .arg(parent_dir)
        .arg(file_name.as_ref())
        .status()
        .map_err(|e| format!("Failed to run tar command: {}", e))?;

    if !status.success() {
        return Err(format!("tar command failed with status: {:?}", status.code()));
    }

    Ok(tar_path_str)
}
