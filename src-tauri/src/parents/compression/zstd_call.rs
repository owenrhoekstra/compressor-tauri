use std::io::Read;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};

use tauri::{AppHandle, Emitter};
use crate::parents::dispatch::dispatcher_cleaner::CompressionEvent;
use crate::parents::utils;

pub fn run(
    app: AppHandle,
    flags: Vec<String>,
    input_path: String,
    output_path: String,
) -> Result<(), String> {
    println!("ENTERED ZSTD RUN");
    println!("Input received: {}", input_path);
    println!("Output path received: {}", output_path);
    // Validate input path
    let input = Path::new(&input_path);
    if !input.exists() {
        return Err(format!("Input file does not exist: {}", input_path));
    }

    // Validate output folder
    let output_dir = Path::new(&output_path);
    if !output_dir.exists() {
        return Err(format!("Output folder does not exist: {}", output_path));
    }

    // Generate output filename (sanitize if input is an intermediate tar)
    let file_name_raw = input
        .file_name()
        .ok_or("Invalid input file name")?
        .to_string_lossy()
        .to_string();

    let mut base_name = file_name_raw.clone();
    if let Some(stripped) = base_name.strip_prefix(".tmp_") {
        base_name = stripped.to_string();
    }

    let timestamp = utils::get_timestamp();
    let output_file_name = format!("{}_{}.zst", timestamp, base_name);

    let mut final_output_path = PathBuf::from(&output_path);
    final_output_path.push(output_file_name);
    let output_path_str = final_output_path.to_string_lossy().to_string();

    println!(
        "Running: zstd --progress -f {:?} {} -o {}",
        flags, input_path, output_path_str
    );

    let mut cmd = Command::new("zstd");

    cmd.arg("--progress")
        .arg("-f")
        .args(&flags)
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path_str)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            "zstd command not found. Please install zstd and ensure it is in your PATH.".to_string()
        } else {
            format!("Failed to start zstd: {}", e)
        }
    })?;

    let mut stderr = child
        .stderr
        .take()
        .ok_or("Failed to capture stderr")?;

    let mut stdout = child
        .stdout
        .take()
        .ok_or("Failed to capture stdout")?;

    let mut buffer = [0u8; 1024];
    let mut current_line = Vec::new();
    let mut full_stderr = String::new();

    loop {
        match stderr.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                for &b in &buffer[..n] {
                    if b == b'\n' || b == b'\r' {
                        emit_progress(&app, &mut current_line, &mut full_stderr);
                    } else {
                        current_line.push(b);
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(format!("Error reading stderr: {}", e)),
        }
    }

    // Handle any trailing data
    if !current_line.is_empty() {
        emit_progress(&app, &mut current_line, &mut full_stderr);
    }

    // Drain stdout to avoid potential deadlock
    let mut stdout_buffer = String::new();
    let _ = stdout.read_to_string(&mut stdout_buffer);

    let status = child.wait().map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!(
            "zstd exited with status {:?}\n\nstderr:\n{}",
            status.code(),
            full_stderr
        ));
    }

    Ok(())
}

fn emit_progress(app: &AppHandle, current_line: &mut Vec<u8>, full_stderr: &mut String) {
    if current_line.is_empty() {
        return;
    }

    let line = String::from_utf8_lossy(current_line).trim().to_string();
    if !line.is_empty() {
        full_stderr.push_str(&line);
        full_stderr.push('\n');

        let percentage = parse_percentage(&line);
        println!("ZSTD Progress: {} | Parsed %: {:?}", line, percentage);

        let _ = app.emit("compression-event", CompressionEvent {
            algorithm: "zstd".to_string(),
            event_type: "progress".to_string(),
            message: line,
            percentage,
        });
    }
    current_line.clear();
}

fn parse_percentage(line: &str) -> Option<f32> {
    if let Some(pos) = line.find('%') {
        let mut start = pos;
        let bytes = line.as_bytes();
        while start > 0 {
            let c = bytes[start - 1];
            if c.is_ascii_digit() || c == b'.' {
                start -= 1;
            } else {
                break;
            }
        }
        if start < pos {
            return line[start..pos].parse::<f32>().ok();
        }
    }
    None
}