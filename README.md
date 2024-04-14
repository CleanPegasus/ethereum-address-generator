# Ethereum Address Generator

This is a vanity Ethereum address generator. It generates Ethereum addresses until it finds one that starts with a specified string and ends with a specified string.

## Requirements

- Rust programming language
- Cargo package manager

## Dependencies

This project uses the following Rust crates:

- `rand`
- `secp256k1`
- `sha3`
- `structopt`

## Usage

1. Clone the repository:

```bash
git clone https://github.com/CleanPegasus/ethereum-address-generator.git
cd ethereum-address-generator
```
2. Build the project:
  
```bash
cargo build --release
```

3. Run the program
```bash
cargo run -- -s START_STRING -n NUM_THREADS -e END_STRING
```
Replace START_STRING with the string you want the Ethereum address to start with, NUM_THREADS with the number of threads you want to use for generation, and END_STRING with the string you want the Ethereum address to end with.

For example, to generate an Ethereum address that starts with '0000' using 8 threads and ends with '69420', you would run:
```bash
cargo run -- -s 0000 -n 8 -e 69420
```