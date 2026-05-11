use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let file_path = &args[2];

    if command != "build" && command != "compile" {
        eprintln!("Error: Unknown command '{}'. Only 'build' is supported.", command);
        print_usage();
        process::exit(1);
    }

    let path = Path::new(file_path);
    if !path.exists() {
        eprintln!("Error: File '{}' not found.", file_path);
        process::exit(1);
    }

    let extension = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if extension != "py" {
        eprintln!("Error: Unsupported file extension '{}'. Only .py is supported.", extension);
        process::exit(1);
    }

    let source = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Failed to read file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    println!("Compiling Python contract: {}...", file_path);

    match tama::compile(&source) {
        Ok(rust_code) => {
            let project_dir = "soroban_contract";
            if let Err(e) = fs::create_dir_all(format!("{}/src", project_dir)) {
                eprintln!("Error: Failed to create project directories: {}", e);
                process::exit(1);
            }

            // Write Cargo.toml for the contract
            let cargo_toml = r#"
[package]
name = "soroban-contract"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "20.5.0"

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
"#;
            if let Err(e) = fs::write(format!("{}/Cargo.toml", project_dir), cargo_toml) {
                eprintln!("Error: Failed to write Cargo.toml: {}", e);
                process::exit(1);
            }

            // Write src/lib.rs
            if let Err(e) = fs::write(format!("{}/src/lib.rs", project_dir), &rust_code) {
                eprintln!("Error: Failed to write src/lib.rs: {}", e);
                process::exit(1);
            }

            println!("\nCompile Complete!");
            println!("Soroban Rust project created in soroban_contract/");
            println!("To build WASM: cd soroban_contract && stellar contract build");
            println!("To deploy contract: stellar contract deploy --wasm soroban_contract/target/wasm32v1-none/release/soroban_contract.wasm --source <account> --network testnet");
        }
        Err(err) => {
            eprintln!("\nCompiler Errors:\n{}", err);
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Tama - Python-to-Soroban Smart Contract Compiler");
    println!("Usage:");
    println!("  tama build <file>    Compile Python (.py) to a Soroban Rust project");
}
