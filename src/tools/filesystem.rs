use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a file or directory in a directory listing
#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: Option<u64>,
}

/// Read the contents of a file
pub async fn read_file(path: &str) -> Result<String> {
    // 1. Validate file path
    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow!("File does not exist: {}", path.display()));
    }
    if !path.is_file() {
        return Err(anyhow!("Path is not a file: {}", path.display()));
    }

    // 2. Read file content
    // In a real implementation, we would request user permission here
    let content = fs::read_to_string(path)?;

    // 3. Return content
    Ok(content)
}

/// List the contents of a directory
pub async fn list_directory(path: &str) -> Result<Vec<FileEntry>> {
    // 1. Validate directory path
    let path = Path::new(path);
    if !path.exists() {
        return Err(anyhow!("Directory does not exist: {}", path.display()));
    }
    if !path.is_dir() {
        return Err(anyhow!("Path is not a directory: {}", path.display()));
    }

    // 2. In a real implementation, we would request user permission here

    // 3. Read directory contents
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        
        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
            is_dir: metadata.is_dir(),
            size: if metadata.is_file() { Some(metadata.len()) } else { None },
        });
    }

    // 4. Return the entries
    Ok(entries)
}