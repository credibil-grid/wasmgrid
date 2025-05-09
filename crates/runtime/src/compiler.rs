//! # Compiler

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use wasmtime::component::Component;
use wasmtime::{Config, Engine};

/// Compile `wasm32-wasip2` component.
///
/// For example, to compile the `http` component, run:
///
/// ```
/// cargo build --package http@0.1.0 --target wasm32-wasip2 --release
/// ```
///
/// # Errors
///
/// Returns an error if the WASM component cannot be loaded from the specified
/// path, cannot be compiled, or cannot be serialized to the specified output
/// directory.
pub fn compile(wasm: &PathBuf, output: Option<PathBuf>) -> Result<()> {
    let Some(file_name) = wasm.file_name() else {
        return Err(anyhow!("invalid file name"));
    };

    // compile component
    let serialized = serialize(wasm)?;

    // define output file
    let mut out_path = output.unwrap_or_else(|| PathBuf::from("."));
    if out_path.is_dir() {
        let file_name = file_name.to_string_lossy().to_string();
        let file_name = file_name.replace(".wasm", ".bin");
        out_path.push(file_name);
    }

    // create output directory if it doesn't exist
    if let Some(dir) = out_path.parent()
        && !fs::exists(dir)?
    {
        fs::create_dir_all(dir)?;
    }

    let mut file = File::create(out_path)?;
    file.write_all(&serialized)?;

    Ok(())
}

/// Compile and serialize a wasm component.
/// 
/// # Errors
/// 
/// Returns an error if the WASM component cannot be loaded from the specified
/// path, cannot be compiled, or cannot be serialized.
pub fn serialize(wasm: &PathBuf) -> Result<Vec<u8>> {
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, wasm)?;
    component.serialize()
}
