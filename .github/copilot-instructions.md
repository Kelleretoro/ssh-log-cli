# SSH Log CLI

SSH Log CLI is a Rust-based command-line tool that decrypts and decodes session replay files captured by Cloudflare's Audit SSH proxy. It supports generating HPKE key pairs, decrypting SSH session captures, and replaying PTY sessions.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites and Setup
- Install Rust via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 
- Ensure you have Rust 1.89.0 or later: `rustc --version`
- On Linux systems, ensure `scriptreplay` is available: `which scriptreplay` (usually pre-installed on Ubuntu/Debian)

### Building the Project
- **NEVER CANCEL** any build commands. Builds can take significant time.
- Development build: `cargo build` -- takes 50-60 seconds for first build, 1-2 seconds for incremental builds. NEVER CANCEL. Set timeout to 5+ minutes.
- Release build: `cargo build --release` -- takes 40-45 seconds. NEVER CANCEL. Set timeout to 5+ minutes.
- Build without default features: `cargo build --no-default-features` -- takes 1-2 seconds for incremental builds.

### Testing the Project
- Run tests: `cargo test` -- takes 5-10 seconds. Currently has 0 unit tests but tests compile successfully.
- Test without default features: `cargo test --no-default-features` -- takes 1-2 seconds.
- Integration tests work via assert_cmd/assert_fs for CLI validation.

### Code Quality and Linting
- Check formatting: `cargo fmt --all -- --check` -- should pass with no output.
- Format code: `cargo fmt --all`
- Always run formatting checks before committing or CI will fail.

### Running the Application
- Build first with: `cargo build`
- Generate key pair: `./target/debug/ssh-log-cli generate-key-pair -o <PRIVATE_KEY_FILE_NAME>`
- Decrypt session file: `./target/debug/ssh-log-cli decrypt -i <INPUT_FILE> -k <PRIVATE_KEY_FILE> [-o <OUTPUT_ZIP>]`
- Replay PTY session (Linux/macOS only): `./target/debug/ssh-log-cli decrypt -i <INPUT_FILE> -k <PRIVATE_KEY_FILE> --replay`

## Validation

### Always Manually Validate Changes
- **CRITICAL**: After making changes, always build and test the full workflow:
  1. `cargo build` to ensure compilation succeeds
  2. `cargo test` to verify tests pass  
  3. `cargo fmt --all -- --check` to verify formatting
  4. Test key generation: `./target/debug/ssh-log-cli generate-key-pair -o /tmp/test_key`
  5. Verify key files created: `ls -la /tmp/test_key*` (should show private key and .pub file)
  6. Test decrypt command error handling: `./target/debug/ssh-log-cli decrypt -i /tmp/nonexistent -k /tmp/test_key` (should show appropriate error)
  7. Clean up: `rm /tmp/test_key*`

### Validation Scenarios for Testing Changes
- **Key Generation Workflow**: Generate a key pair and verify both private and public key files are created with correct permissions and base64 content.
- **CLI Help System**: Run `./target/debug/ssh-log-cli --help`, `./target/debug/ssh-log-cli generate-key-pair --help`, and `./target/debug/ssh-log-cli decrypt --help` to ensure help text is correct.
- **Error Handling**: Test with invalid inputs to ensure proper error messages are displayed.
- **Cross-platform Compatibility**: Changes should work on Linux, macOS, and Windows as verified by GitHub Actions.

### Continuous Integration Requirements
- All builds must pass on Linux (ubuntu-latest), macOS (macos-latest), and Windows (windows-latest).
- Formatting must pass: `cargo fmt --all -- --check`
- Both `cargo build` and `cargo build --no-default-features` must succeed.
- Both `cargo test` and `cargo test --no-default-features` must pass.

## Build Times and Timeout Requirements

### Critical Timing Information
- **NEVER CANCEL build operations** - they may appear to hang but are processing dependencies.
- First `cargo build`: 50-60 seconds. Set timeout to 300+ seconds (5+ minutes).
- `cargo build --release`: 40-45 seconds. Set timeout to 300+ seconds (5+ minutes).  
- `cargo test`: 5-10 seconds including dependency compilation.
- Incremental builds: 1-2 seconds after first build.
- Dependency downloads on first build: Additional 10-20 seconds depending on network.

### GitHub Actions Timing
- Test workflow runs on 3 platforms simultaneously.
- Each platform build takes 1-3 minutes including Rust installation.
- Release builds include cross-compilation and artifact generation.

## Common Tasks

### Repository Structure
```
.
├── .github/
│   └── workflows/          # CI/CD workflows
├── src/                    # Source code
│   ├── main.rs            # CLI entry point and argument parsing
│   ├── hpke.rs            # HPKE encryption/decryption logic
│   ├── data.rs            # Data packet handling
│   ├── metadata.rs        # Session metadata parsing
│   ├── pty.rs             # PTY session replay functionality
│   └── zip.rs             # ZIP file creation utilities
├── Cargo.toml             # Rust project configuration
├── README.md              # Project documentation
└── LICENSE                # BSD 3-Clause license
```

### Key Project Files
- **Cargo.toml**: Contains dependencies and build configuration. Uses Rust 2021 edition.
- **src/main.rs**: Main CLI interface using clap derive macros. Contains decrypt and generate-key-pair subcommands.
- **src/hpke.rs**: Core HPKE (Hybrid Public Key Encryption) implementation for session decryption.
- **src/pty.rs**: PTY session replay functionality - contains generate_replay function.

### Dependencies Overview
- **clap 3.0**: Command-line argument parsing with derive macros
- **hpke 0.8**: HPKE encryption library with serde support
- **base64 0.13**: Base64 encoding/decoding for keys
- **zip 0.6**: ZIP file creation for output archives
- **tempfile 3.3**: Temporary file management
- **assert_cmd/assert_fs**: Integration testing for CLI applications

### Output File Formats
- **PTY Sessions**: Creates `term_data.txt` and `term_times.txt` for use with `scriptreplay`
- **Non-PTY Sessions**: Creates `data_from_client.txt` and `data_from_server.txt`
- **Default output**: `<input_filename>-decrypted.zip` if no output specified

### Error Handling
- Missing or invalid private keys return "Could not read metadata"
- Invalid input files return appropriate error messages
- CLI validates file extensions (output must be .zip for decrypt command)
- Proper error propagation using Result<(), String> pattern

### Platform Considerations
- **Linux/macOS**: Full functionality including PTY replay with scriptreplay
- **Windows**: Core functionality works, PTY replay may have limitations
- **Cross-compilation**: Supported via GitHub Actions for release builds
- **Dependencies**: All major dependencies are cross-platform compatible