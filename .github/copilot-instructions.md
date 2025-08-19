# SSH Log CLI

SSH Log CLI is a Rust command-line tool that decrypts and decodes session replay files captured by Cloudflare's Audit SSH proxy. It can decrypt session files and replay PTY sessions or extract non-PTY session data.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites and Installation
- Install Rust from https://doc.rust-lang.org/cargo/getting-started/installation.html
- Ensure you have `scriptreplay` command available (part of util-linux package)
- Rust 1.89.0 or newer recommended

### Build Process
- `cargo build` -- builds debug version. Takes ~45 seconds on first build, ~1 second for incremental builds. NEVER CANCEL. Set timeout to 60+ minutes.
- `cargo build --release` -- builds optimized release version. Takes ~40 seconds. NEVER CANCEL. Set timeout to 60+ minutes.
- `cargo build --no-default-features` -- builds without default features. Takes ~1 second for incremental builds.

### Testing
- `cargo test` -- runs all tests. Takes ~6 seconds. NEVER CANCEL. Set timeout to 30+ minutes.
- `cargo test --no-default-features` -- runs tests without default features. Takes <1 second.

### Code Quality
- `cargo fmt` -- formats code according to Rust standards. Run before committing.
- `cargo fmt --all -- --check` -- checks if code is formatted properly. Must pass for CI.

### Build the Application
1. `cargo build --release`
2. Find the compiled binary at `target/release/ssh-log-cli`

## Validation

### Manual Testing Scenarios
- Always test key generation: `./target/release/ssh-log-cli generate-key-pair -o test_key`
- Verify help system works: `./target/release/ssh-log-cli --help` and `./target/release/ssh-log-cli decrypt --help`
- Check generated keys exist and have proper base64 content
- ALWAYS run through complete CLI workflow after making changes
- Run `cargo fmt` and verify no formatting changes needed before committing

### Expected CI Workflow
- Always run `cargo fmt --all -- --check` or the CI (.github/workflows/lint.yaml) will fail
- All builds must complete successfully on Linux, macOS, and Windows
- Tests run on stable Rust across all platforms

## Common Tasks

### Basic Usage Examples
```bash
# Generate HPKE key pair
./target/release/ssh-log-cli generate-key-pair -o private_key

# Decrypt and save to ZIP file
./target/release/ssh-log-cli decrypt -i encrypted_file -k private_key -o output.zip

# Decrypt and replay PTY session (Linux/macOS only)
./target/release/ssh-log-cli decrypt -i encrypted_file -k private_key --replay
```

### Repository Structure
```
├── src/
│   ├── main.rs          # CLI entry point and command handling
│   ├── data.rs          # Data packet processing and decoding
│   ├── hpke.rs          # HPKE encryption/decryption context
│   ├── metadata.rs      # Session metadata parsing
│   ├── pty.rs           # PTY session replay generation
│   └── zip.rs           # ZIP file operations
├── .github/workflows/   # CI/CD pipelines
├── Cargo.toml          # Rust project configuration
└── README.md           # User documentation
```

### Understanding Output Files

#### PTY Sessions
When decrypting PTY sessions, the output ZIP contains:
- `term_data.txt` -- Terminal data for replay
- `term_times.txt` -- Timing information for scriptreplay

To replay: `scriptreplay --timing term_times.txt term_data.txt`

#### Non-PTY Sessions
When decrypting non-PTY sessions, the output ZIP contains:
- `data_from_client.txt` -- Upstream traffic data
- `data_from_server.txt` -- Downstream traffic data

### Key Project Components
- **main.rs**: CLI command parsing using `clap`, handles both `generate-key-pair` and `decrypt` commands
- **data.rs**: Defines `DataDecoder` for processing encrypted data packets with timing information
- **metadata.rs**: Handles session metadata including PTY information, exit data, and encryption keys
- **pty.rs**: Generates scriptreplay-compatible files for terminal session replay
- **hpke.rs**: HPKE (Hybrid Public Key Encryption) implementation for decryption

### Timing Expectations
- **NEVER CANCEL**: Build times can be long, especially on first build
- Debug build: ~45 seconds first time, ~1 second incremental
- Release build: ~40 seconds
- Tests: ~6 seconds
- Formatting check: instantaneous
- Key generation: <1 second
- CI builds: 5-10 minutes across all platforms

### Common Issues and Solutions
- If build fails with missing `generate_replay` function, ensure `src/pty.rs` has proper implementation
- If formatting check fails, run `cargo fmt` to fix automatically
- For CI failures, check that all workflow commands pass locally first
- PTY replay only works on Linux/macOS systems with `scriptreplay` available

### Development Workflow
1. Make code changes
2. Run `cargo build` to verify compilation
3. Run `cargo test` to verify tests pass
4. Run `cargo fmt` to format code
5. Test CLI functionality manually with key generation and help commands
6. Commit changes after all validations pass