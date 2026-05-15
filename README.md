## Tama

Tama is a Python-to-Soroban compiler, it compiles native Python smart contracts directly to highly-optimized standard Soroban output.

### Prerequisites

- Rust (latest stable)
- Stellar CLI (optional, for building and deploying WASM)

### Building the Compiler

To compile the `tama` CLI binary:

```bash
cargo build --release
```

This creates the compiler binary at `./target/release/tama`.

### Usage

1. Compile your Python contract:
   ```bash
   ./target/release/tama build examples/counter.py
   ```
   This creates a standard Rust Soroban SDK project in `soroban_contract/`.

2. Build the contract to WASM:
   ```bash
   cd soroban_contract
   stellar contract build
   ```
   This compiles the project to `target/wasm32v1-none/release/soroban_contract.wasm`.

### Autocomplete Setup

To enable IDE autocompletion for smart contracts, install the helper SDK locally in your Python environment:

```bash
cd python-sdk
pip install -e .
```
