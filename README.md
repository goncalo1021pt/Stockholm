# Stockholm

Educational ransomware simulation for the 42 Cybersecurity Piscine.

Encrypts files in `~/infection` that match WannaCry-targeted extensions using
**AES-256-GCM** (256-bit key, authenticated encryption). Decryption is fully
reversible with the generated key.

> **Warning:** For educational purposes only. Never use this on real systems.

## Requirements

- Docker + Docker Compose

## Build & Run

```bash
# Build the Docker image and compile the binary
make

# Open a shell inside the container
make bash

# Inside the container — run the program
./target/release/stockholm             # encrypt ~/infection
./target/release/stockholm --silent    # encrypt, no output
./target/release/stockholm --reverse <KEY>   # decrypt with key
./target/release/stockholm --help      # show help
./target/release/stockholm --version   # show version
```

## Encryption

- Algorithm: **AES-256-GCM**
- A random 32-byte key is generated on each run and printed to stdout.
- Each file gets its own random 12-byte nonce (prepended to ciphertext).
- Encrypted files are renamed with the `.ft` extension.

## File targets

Only files whose extensions were targeted by the WannaCry ransomware are
affected (`.doc`, `.pdf`, `.jpg`, `.sql`, `.zip`, etc. — 176 extensions total).