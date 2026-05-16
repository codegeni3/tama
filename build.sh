#!/bin/bash
set -e

echo "Building Tama compiler..."
cargo build --release

echo "Compiling Python smart contract..."
./target/release/tama build examples/counter.py

echo "To build the compiled contract to WASM, run:"
echo "  cd soroban_contract && stellar contract build"
