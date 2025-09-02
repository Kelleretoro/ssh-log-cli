# SSH Log CLI

SSH Log CLI is a Rust command-line tool that decrypts and decodes session replay files captured by Cloudflare's Audit SSH proxy. The tool provides functionality to generate HPKE key pairs and decrypt encrypted SSH session logs.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

- Bootstrap, build, and test the repository:
  - Ensure Rust is installed: `rustc --version` (should show Rust 1.89.0 or later)
  - `cargo check` -- fast syntax check, completes in ~1 second
  - `cargo build` -- debug build takes ~27 seconds. NEVER CANCEL. Set timeout to 60+ minutes.
  - `cargo build --release` -- release build takes ~45 seconds. NEVER CANCEL. Set timeout to 60+ minutes.
  - `cargo test` -- runs tests in ~7 seconds. NEVER CANCEL. Set timeout to 30+ minutes.
  - `cargo fmt --check` -- code formatting check, completes in <1 second
  - `cargo clippy` -- linting check, completes in ~3 seconds

## Core Functionality

- Generate HPKE key pairs:
  - `cargo run -- generate-key-pair -o <PRIVATE_KEY_FILE_NAME>`
  - Creates two files: private key and `<PRIVATE_KEY_FILE_NAME>.pub` public key
  - Both keys are base64 encoded, 44 characters each

- Decrypt SSH session logs:
  - `cargo run -- decrypt -i <INPUT_FILE> -k <PRIVATE_KEY_FILE> [-o <OUTPUT_ZIP>]`
  - `cargo run -- decrypt -i <INPUT_FILE> -k <PRIVATE_KEY_FILE> --replay` (Linux/macOS only)
  - Output defaults to `<input_file>-decrypted.zip` if not specified
  - Replay mode requires `scriptreplay` command to be available in PATH

## Validation

- ALWAYS run through at least one complete end-to-end scenario after making changes:
  - Generate a key pair: `cargo run -- generate-key-pair -o /tmp/test_key`
  - Verify both `/tmp/test_key` and `/tmp/test_key.pub` files are created
  - Test help commands: `cargo run -- --help` and `cargo run -- decrypt --help`
- ALWAYS run `cargo fmt --check` and `cargo clippy` before committing changes or the CI (.github/workflows/lint.yaml) will fail
- Build artifacts are located in `target/` directory and should not be committed

## Repository Structure

```
ssh-log-cli/
├── .github/
│   └── workflows/          # CI/CD pipelines
│       ├── lint.yaml       # Code formatting and linting
│       ├── tests.yml       # Cross-platform testing
│       ├── release.yml     # Binary releases and attestations
│       └── semgrep.yml     # Security scanning
├── src/
│   ├── main.rs            # CLI entry point and command handling
│   ├── data.rs            # Data packet decoding and file generation
│   ├── hpke.rs            # HPKE encryption/decryption context
│   ├── metadata.rs        # Session metadata parsing
│   ├── pty.rs             # PTY session replay generation
│   └── zip.rs             # ZIP file creation utilities
├── Cargo.toml             # Rust project configuration and dependencies
├── Cargo.lock             # Dependency lock file
└── README.md              # Project documentation
```

## Key Dependencies

- `clap` v3.0.0 - Command-line argument parsing with derive features
- `hpke` v0.8.0 - HPKE encryption with serde support
- `chrono` v0.4.19 - Date and time handling
- `serde` and `serde_json` - Serialization and JSON parsing
- `base64` v0.13.0 - Base64 encoding/decoding
- `thiserror` v1.0.20 - Error handling macros
- `zip` v0.6.2 - ZIP file creation and manipulation

## Build Times and Timeouts

**CRITICAL**: All build and test commands can take significant time. NEVER CANCEL these operations:

- `cargo check`: ~1 second (use default timeout)
- `cargo build`: ~27 seconds (set timeout to 60+ minutes)
- `cargo build --release`: ~45 seconds (set timeout to 60+ minutes)  
- `cargo test`: ~7 seconds (set timeout to 30+ minutes)
- `cargo clippy`: ~3 seconds (use default timeout)
- `cargo fmt --check`: <1 second (use default timeout)

## Troubleshooting

- If build fails with "function not found" errors, verify all source files match the working implementation
- The main entry point expects `pty::generate_replay` function to exist in `src/pty.rs`
- SSH session replay functionality requires Linux or macOS (Windows not supported for replay)
- `scriptreplay` command must be available in PATH for replay functionality
- All encrypted input files must be valid Cloudflare SSH audit log format

## Common Commands Reference

### Build and Development
```bash
# Quick syntax check
cargo check

# Build debug version
cargo build

# Build optimized release version  
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

### CLI Usage
```bash
# Generate key pair
./target/release/ssh-log-cli generate-key-pair -o my_key

# Decrypt to ZIP file
./target/release/ssh-log-cli decrypt -i session.log -k my_key -o output.zip

# Decrypt and replay (Linux/macOS)
./target/release/ssh-log-cli decrypt -i session.log -k my_key --replay

# Show help
./target/release/ssh-log-cli --help
./target/release/ssh-log-cli decrypt --help
./target/release/ssh-log-cli generate-key-pair --help
```

## CI/CD Pipeline

The repository uses GitHub Actions with multiple workflows:

- **tests.yml**: Cross-platform testing on Ubuntu, macOS, and Windows
- **lint.yaml**: Code formatting validation with `cargo fmt --check`  
- **release.yml**: Automated binary releases with attestations for tagged versions
- **semgrep.yml**: Security vulnerability scanning

All workflows must pass for successful CI. Pay special attention to formatting requirements enforced by lint.yaml.