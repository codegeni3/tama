# Tama SDK

This is the type autocomplete and static analysis package for Tama Python smart contracts

Since your Python smart contracts are compiled natively into Soroban-compatible Rust WebAssembly by the Tama CLI, this package is not run at contract runtime. It is only installed in your local Python environment to give your IDE full type definitions and autocomplete support

## Installation

To install this package locally for autocomplete:

```bash
cd python-sdk
pip install -e .
```

## Usage

In your smart contract (`contract.py`):

```python
from tama import storage

def increment():
    x = storage.get("counter")
    storage.set("counter", x + 1)
```

Once installed, your editor will automatically recognize the `tama` module, provide auto-completion for storage methods and display hovering documentation
