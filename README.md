## Tama

Tama is a Python-to-Soroban compiler, it compiles native Python smart contracts directly to highly-optimized standard Soroban output.

### Features

- **Native Python Syntax**: Write smart contracts using standard Python language constructs.
- **Strict Sandbox Validation**: Built-in AST validator ensures your code is safe and deterministic.
- **Dynamic Project Generation**: Automatically generates a complete Cargo project matching your contract name.
- **Full IDE Support**: Includes the `tama` mock package for full type hinting and autocomplete.

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
   This dynamically creates a standard Rust Soroban SDK project in `counter_contract/`.

2. Build the contract to WASM:
   ```bash
   cd counter_contract
   stellar contract build
   ```
   This compiles the project to `target/wasm32v1-none/release/counter.wasm`.

### Autocomplete Setup

To enable IDE autocompletion for smart contracts, install the helper SDK locally in your Python environment:

```bash
cd python-sdk
pip install -e .
```
