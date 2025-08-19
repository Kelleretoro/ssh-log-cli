# SSH Log CLI

SSH Log CLI is a command-line tool written in Rust that decrypts and decodes session replay files captured by Cloudflare's Audit SSH proxy. It generates HPKE key pairs and processes encrypted SSH session logs.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites
- Rust toolchain (1.89.0+ recommended)
- System packages: `util-linux` (for `scriptreplay` command)

### Build and Test Process
- **Bootstrap and build:**
  - `cargo build` -- debug build takes ~1.6 seconds
  - `cargo build --release` -- takes ~42 seconds. NEVER CANCEL. Set timeout to 90+ seconds.
  - `cargo build --no-default-features` -- CI requirement, takes <1 second
- **Run tests:**
  - `cargo test` -- takes ~8 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
  - `cargo test --no-default-features` -- CI requirement, takes <1 second
- **Code quality checks:**
  - `cargo fmt --all -- --check` -- takes <1 second
  - `cargo check` -- fast syntax/type checking, takes ~22 seconds first time
- **Run the application:**
  - Debug: `./target/debug/ssh-log-cli`
  - Release: `./target/release/ssh-log-cli`

### Core Functionality
- **Generate HPKE key pair:**
  - `cargo run -- generate-key-pair -o <private_key_filename>`
  - Creates `<private_key_filename>` and `<private_key_filename>.pub`
- **Decrypt SSH session logs:**
  - `cargo run -- decrypt -i <input_file> -k <private_key> -o <output.zip>`
  - `cargo run -- decrypt -i <input_file> -k <private_key> --replay` (PTY sessions only)

## Validation

### Build Validation
- ALWAYS run `cargo build` first to ensure compilation succeeds
- ALWAYS run `cargo test` to ensure no regressions
- ALWAYS run `cargo fmt --all -- --check` before committing changes
- ALWAYS test both `cargo build --no-default-features` and `cargo test --no-default-features` as required by CI
- Build times are normal: debug ~1.6s, release ~42s, no-default-features <1s

### Functional Validation  
- **Key generation test:** `./target/release/ssh-log-cli generate-key-pair -o /tmp/test_key`
  - Verify both `/tmp/test_key` and `/tmp/test_key.pub` are created
- **Help commands:** `./target/release/ssh-log-cli --help` and `./target/release/ssh-log-cli decrypt --help`
- **Version check:** `./target/release/ssh-log-cli --version`

### Manual Testing Scenarios
- **Complete end-to-end workflow:**
  1. `./target/release/ssh-log-cli generate-key-pair -o /tmp/demo_key`
  2. Verify both `/tmp/demo_key` and `/tmp/demo_key.pub` exist
  3. `./target/release/ssh-log-cli --help` - verify help output
  4. `./target/release/ssh-log-cli decrypt --help` - verify subcommand help
  5. Clean up: `rm -f /tmp/demo_key*`
- **Error handling validation:**
  - `./target/release/ssh-log-cli decrypt -i nonexistent.file -k /tmp/test_key` - should show "Could not open input file"
  - `./target/release/ssh-log-cli generate-key-pair` - should show missing required arguments error
- **Version and help commands:** `./target/release/ssh-log-cli --version` and help commands
- **PTY replay dependency:** Verify `scriptreplay` is available in PATH for PTY replay functionality

## Common Tasks

The following are outputs from frequently run commands. Reference them instead of viewing, searching, or running bash commands to save time.

### Repository Root Structure
```
.
├── .github/
│   └── workflows/          # CI/CD workflows (tests.yml, lint.yaml, release.yml)
├── .gitignore
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Cargo.toml             # Rust project file with dependencies
├── Cargo.lock
├── LICENSE
├── README.md              # User documentation
├── assets/                # Logo and branding assets
├── custom.css             # VS Code customization
├── package.json           # Node.js metadata (unused for build)
└── src/                   # Rust source code
    ├── main.rs            # CLI entry point and command parsing
    ├── data.rs            # Data decoding and processing
    ├── hpke.rs            # HPKE cryptographic operations
    ├── metadata.rs        # SSH session metadata handling
    ├── pty.rs             # PTY session processing and replay
    └── zip.rs             # ZIP file creation utilities
```

### Key Dependencies (from Cargo.toml)
```toml
[dependencies]
clap = {version = "3.0.0", features = ["derive"]}  # CLI parsing
hpke = {version = "0.8.0", features = ["serde_impls"]}  # Encryption
serde = {version = "1.0.130", features = ["derive"]}  # Serialization
zip = "0.6.2"              # ZIP file handling
chrono = "0.4.19"          # Date/time handling
base64 = "0.13.0"          # Base64 encoding/decoding

[dev-dependencies]
assert_cmd = "2.0.2"       # Command testing
assert_fs = "1.0.6"        # Filesystem testing
```

### CLI Commands Output
```bash
$ ./target/release/ssh-log-cli --help
ssh-log-cli 0.1.0

USAGE:
    ssh-log-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decrypt              
    generate-key-pair    
    help                 Print this message or the help of the given subcommand(s)
```

### CI/CD Information
- **Tests:** Run on Linux, macOS, and Windows with stable Rust
- **Linting:** Uses `cargo fmt --all -- --check`
- **Releases:** Creates binaries for x86_64-unknown-linux-musl, x86_64-apple-darwin, x86_64-pc-windows-msvc
- **Security:** Includes Semgrep scanning

## Architecture Notes

### Code Organization
- `main.rs`: CLI argument parsing using Clap, main application logic
- `hpke.rs`: HPKE (Hybrid Public Key Encryption) implementation
- `data.rs`: SSH session data decoding and processing
- `pty.rs`: PTY (pseudo-terminal) session handling and scriptreplay generation
- `metadata.rs`: SSH session metadata structures
- `zip.rs`: ZIP file creation utilities

### Key Functions
- `generate_replay()` in `pty.rs`: Converts encrypted PTY data to scriptreplay format
- `run_decrypt()` in `main.rs`: Main decryption workflow
- `run_generate_key_pair()` in `main.rs`: HPKE key pair generation

### Error Handling
- Custom error types for each module (e.g., `PTYParserError`, `DataError`)
- Uses `thiserror` crate for error derivation
- Proper error propagation with `Result<T, E>` types

## Troubleshooting

### Common Issues
- **Build fails with missing `generate_replay`:** The `pty.rs` file may have placeholder code. Replace with actual implementation.
- **`scriptreplay` not found:** Install `util-linux` package or ensure it's in PATH
- **Long build times:** Release builds take ~42 seconds, this is normal due to cryptographic dependencies

### Build Environment
- Rust 1.89.0+ required
- Works on Linux, macOS, and Windows
- No external C dependencies beyond system libraries
- Uses `musl` target for Linux static builds in CI

### Development Workflow
1. Make changes to source code
2. Run `cargo check` for fast compilation checking
3. Run `cargo build` to check full compilation
4. Run `cargo test` to verify functionality  
5. Run `cargo build --no-default-features` and `cargo test --no-default-features` (CI requirements)
6. Run `cargo fmt --all -- --check` to verify formatting
7. Test CLI functionality manually
8. Commit changes