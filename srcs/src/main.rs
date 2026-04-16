use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Stockholm - file encryption tool")]
struct Args {
    #[arg(short, long, help = "Decrypt files using the provided key")]
    reverse: Option<String>,

    #[arg(short, long, help = "Suppress all output")]
    silent: bool,
}

fn main() {
    let args = Args::parse();

    if args.silent {
        // suppress output later
    }

    match args.reverse {
        Some(key) => println!("Decrypting with key: {}", key),
        None => println!("Encrypting..."),
    }
}