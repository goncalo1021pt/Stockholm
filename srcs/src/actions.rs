use std::io;
use std::path::Path;

use crate::crypto;
use crate::files;

/// Encrypt all eligible files in `dir`.
/// Returns the generated hex key (print and save it!).
/// Prints each filename unless `silent` is true.
pub fn run_encrypt(dir: &Path, silent: bool) -> io::Result<String> {
    let key = crypto::generate_key();
    let targets = files::collect_encryptable(dir)?;

    if targets.is_empty() && !silent {
        println!("No eligible files found in {}", dir.display());
        return Ok(key);
    }

    for path in &targets {
        match files::encrypt_file(path, &key) {
            Ok(new_path) => {
                if !silent {
                    println!("Encrypted: {} -> {}", path.display(), new_path.display());
                }
            }
            Err(e) => {
                // Never crash — report the error and continue
                eprintln!("Error encrypting {}: {}", path.display(), e);
            }
        }
    }

    Ok(key)
}

/// Decrypt all `.ft` files in `dir` using the provided key.
/// Prints each filename unless `silent` is true.
pub fn run_decrypt(dir: &Path, key: &str, silent: bool) -> io::Result<()> {
    // Basic key length guard (must be 64 hex chars = 32 bytes)
    if key.len() < 32 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Key is too short ({} chars). Expected at least 64 hex characters.",
                key.len()
            ),
        ));
    }

    let targets = files::collect_decryptable(dir)?;

    if targets.is_empty() && !silent {
        println!("No .ft files found in {}", dir.display());
        return Ok(());
    }

    for path in &targets {
        match files::decrypt_file(path, key) {
            Ok(original) => {
                if !silent {
                    println!("Decrypted: {} -> {}", path.display(), original.display());
                }
            }
            Err(e) => {
                eprintln!("Error decrypting {}: {}", path.display(), e);
            }
        }
    }

    Ok(())
}