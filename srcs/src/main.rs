mod actions;
mod crypto;
mod extensions;
mod files;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "stockholm",
    version = "1.0.0",
    about = "Stockholm - educational ransomware simulation (AES-256-GCM)",
    long_about = "Stockholm encrypts files in ~/infection that match WannaCry-targeted \
                  extensions using AES-256-GCM. It can also reverse the encryption \
                  given the original key.\n\n\
                  WARNING: This tool is for educational purposes only."
)]
struct Args {
    /// Decrypt files using the provided key (hex string)
    #[arg(short, long, value_name = "KEY")]
    reverse: Option<String>,

    /// Suppress all output
    #[arg(short, long)]
    silent: bool,
}

fn main() {
    let args = Args::parse();

    let dir = match files::infection_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: could not locate ~/infection directory: {}", e);
            std::process::exit(1);
        }
    };

    match args.reverse {
        Some(key) => {
            if let Err(e) = actions::run_decrypt(&dir, &key, args.silent) {
                eprintln!("Decryption error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            match actions::run_encrypt(&dir, args.silent) {
                Ok(key) => {
                    // Always print the key regardless of --silent so it is never lost
                    println!("\nEncryption complete.");
                    println!("Your key (save this — you will need it to decrypt):");
                    println!("{}", key);
                }
                Err(e) => {
                    eprintln!("Encryption error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}