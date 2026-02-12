import { invoke } from '@tauri-apps/api/core';

export async function startCompression(algorithm: string | null, flags: string[], inputPath: string, outputPath: string) {
    console.log('Submitting to Rust:', {
        algorithm,
        flags,
        inputPath,
        outputPath
    });

    return await invoke('start_compression', { 
        algorithm, 
        flags, 
        inputPath, 
        outputPath 
    });
}
