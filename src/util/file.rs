use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::error::Result; 


pub fn read_source_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let path_ref = path.as_ref();
    let mut file = File::open(path_ref)
        .map_err(|e| format!("Failed to open source file '{}': {}", path_ref.display(), e))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read source file '{}': {}", path_ref.display(), e))?;
    
    Ok(contents)
}

pub fn read_binary_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path_ref = path.as_ref();
    let mut file = File::open(path_ref)
        .map_err(|e| format!("Failed to open binary file '{}': {}", path_ref.display(), e))?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read binary file '{}': {}", path_ref.display(), e))?;
    
    Ok(buffer)
}

pub fn write_binary_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<()> {
    let path_ref = path.as_ref();
    let mut file = File::create(path_ref)
        .map_err(|e| format!("Failed to create binary file '{}': {}", path_ref.display(), e))?;
    
    file.write_all(data)
        .map_err(|e| format!("Failed to write data to binary file '{}': {}", path_ref.display(), e))?;
    
    Ok(())
}

pub fn write_text_file<P: AsRef<Path>>(path: P, text: &str) -> Result<()> {
    let path_ref = path.as_ref();
    let mut file = File::create(path_ref)
        .map_err(|e| format!("Failed to create text file '{}': {}", path_ref.display(), e))?;
    
    file.write_all(text.as_bytes())
        .map_err(|e| format!("Failed to write text to file '{}': {}", path_ref.display(), e))?;
    
    Ok(())
}