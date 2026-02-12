use crate::parents::compression;
use tauri::{AppHandle, Emitter};
use std::path::Path;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct CompressionEvent {
    pub algorithm: String,
    pub event_type: String, // "progress", "status", "error"
    pub message: String,
    pub percentage: Option<f32>,
}

pub fn dispatch(app: AppHandle, algorithm: Option<String>, flags: Vec<String>, input_path: String, output_path: String) -> Result<(), String> {
    let algo = algorithm.unwrap_or_else(|| "zstd".to_string()).to_lowercase();
    
    let result = (|| -> Result<(), String> {
        // 1. Basic Validation
        let input = Path::new(&input_path);
        if !input.exists() {
            return Err(format!("Input file does not exist: {}", input_path));
        }

        let output = Path::new(&output_path);
        if !output.is_dir() {
            return Err(format!("Output folder does not exist or is not a directory: {}", output_path));
        }

        // 2. Emit Start Event
        let _ = app.emit("compression-event", CompressionEvent {
            algorithm: algo.clone(),
            event_type: "status".to_string(),
            message: format!("Starting {} compression...", algo),
            percentage: Some(0.0),
        });

        // 3. Create intermediate tar if it's a directory, then dispatch
        let is_dir = input.is_dir();
        let mut actual_input_path = input_path.clone();
        let mut is_temporary_tar = false;

        if is_dir {
            let _ = app.emit("compression-event", CompressionEvent {
                algorithm: algo.clone(),
                event_type: "status".to_string(),
                message: "Input is a directory. Creating intermediate tar...".to_string(),
                percentage: None,
            });
            actual_input_path = compression::tar_call::pack(&input_path)?;
            is_temporary_tar = true;
        } else {
            let _ = app.emit("compression-event", CompressionEvent {
                algorithm: algo.clone(),
                event_type: "status".to_string(),
                message: "Input is a file. Skipping tar...".to_string(),
                percentage: None,
            });
        }

        let dispatch_result = match algo.as_str() {
            "zstd" => compression::zstd_call::run(app.clone(), flags, actual_input_path.clone(), output_path.clone()),
            "xz" => compression::xz_call::run(app.clone(), flags, actual_input_path.clone(), output_path.clone()),
            "7z" | "7zip" | "sevenzip" => compression::sevenzip_call::run(app.clone(), flags, actual_input_path.clone(), output_path.clone()),
            "zpaq" => compression::zpaq_call::run(app.clone(), flags, actual_input_path.clone(), output_path.clone()),
            "paq8px" => compression::paq8px_call::run(app.clone(), flags, actual_input_path.clone(), output_path.clone()),
            _ => Err(format!("Unknown algorithm '{}'", algo)),
        };

        // attempt cleanup of intermediate tar if one was created
        if is_temporary_tar {
            if let Err(e) = std::fs::remove_file(&actual_input_path) {
                eprintln!("Warning: failed to remove intermediate tar {}: {}", actual_input_path, e);
            }
        }

        dispatch_result
    })();

    // 4. Emit Final Event
    match &result {
        Ok(_) => {
            println!("Emitting SUCCESS event for {}", algo);
            let _ = app.emit("compression-event", CompressionEvent {
                algorithm: algo,
                event_type: "success".to_string(),
                message: "Compression finished successfully".to_string(),
                percentage: Some(100.0),
            });
        }
        Err(e) => {
            let _ = app.emit("compression-event", CompressionEvent {
                algorithm: algo,
                event_type: "error".to_string(),
                message: e.clone(),
                percentage: None,
            });
        }
    }

    result
}
