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

## Data Flow and Processing

### Encryption/Decryption Flow
1. **Key Generation**: `hpke.rs` creates P-256 ECDH + AES-GCM key pairs
2. **Metadata Parsing**: `metadata.rs` extracts session info from encrypted header  
3. **Data Decryption**: `hpke.rs` decrypts session data using HPKE context
4. **Data Processing**: `data.rs` parses decrypted packets into client/server streams
5. **Output Generation**: Either ZIP archive or PTY replay format

### File Format Details
- **Input**: Cloudflare SSH audit log (encrypted binary format)
- **Metadata**: Base64-encoded JSON header with session details
- **Keys**: 44-character base64 strings (P-256 private/public keys)
- **Output ZIP**: Contains `data_from_client.txt`, `data_from_server.txt` for non-PTY
- **PTY Output**: Contains `term_data.txt`, `term_times.txt` for scriptreplay

### PTY vs Non-PTY Sessions
- **PTY Sessions**: Interactive terminal sessions with allocated pseudo-terminal
  - Generate timing data for replay with `scriptreplay`
  - Include terminal dimensions, modes, and escape sequences
- **Non-PTY Sessions**: Non-interactive sessions (file transfers, command execution)
  - Generate separate client/server data streams
  - No timing information or terminal control sequences

## Validation

### Basic Validation (Required After Any Changes)
- ALWAYS run through at least one complete end-to-end scenario after making changes:
  - Generate a key pair: `cargo run -- generate-key-pair -o /tmp/test_key`
  - Verify both `/tmp/test_key` and `/tmp/test_key.pub` files are created
  - Test help commands: `cargo run -- --help` and `cargo run -- decrypt --help`
- ALWAYS run `cargo fmt --check` and `cargo clippy` before committing changes or the CI (.github/workflows/lint.yaml) will fail
- Build artifacts are located in `target/` directory and should not be committed

### Comprehensive Testing Checklist
```bash
# 1. Code quality checks
cargo fmt --check          # Format validation
cargo clippy               # Lint validation  
cargo check                # Syntax validation

# 2. Build verification
cargo build                # Debug build (~27s)
cargo build --release      # Release build (~45s)

# 3. Functionality testing
cargo run -- --help                        # Main help
cargo run -- generate-key-pair --help      # Subcommand help
cargo run -- decrypt --help                # Subcommand help
cargo run -- generate-key-pair -o test_key # Key generation
ls -la test_key test_key.pub               # Verify key files
wc -c test_key test_key.pub                # Verify 44 chars each
rm test_key test_key.pub                   # Cleanup

# 4. Advanced testing (with real data)
# cargo run -- decrypt -i <real_session.log> -k test_key -o output.zip
# unzip -l output.zip  # Verify ZIP contents
```

### Performance Expectations
- `cargo check`: ~1 second (incremental), ~48 seconds (fresh)
- `cargo build`: ~27 seconds (debug), ~45 seconds (release)
- `cargo test`: ~7 seconds (no actual tests currently)
- `cargo clippy`: ~3 seconds
- Key generation: Near-instant
- File operations: Dependent on input file size

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

## Key Modules and Responsibilities

### `src/main.rs`
- CLI entry point using `clap` with derive macros
- Command routing between `decrypt` and `generate-key-pair` subcommands
- File I/O coordination and temporary directory management
- ZIP file creation orchestration

### `src/hpke.rs`
- HPKE (Hybrid Public Key Encryption) key pair generation and management
- Encryption context creation and key derivation
- Base64 encoding/decoding of keys (always 44 characters)
- Key format validation

### `src/data.rs`
- Data packet decoding from encrypted session streams
- Binary data parsing and transformation
- Session data extraction and formatting
- File generation for client/server data streams

### `src/metadata.rs`
- Session metadata parsing from encrypted headers
- JSON deserialization of session information
- PTY metadata handling (terminal settings, dimensions)
- Exit status and timestamp processing

### `src/pty.rs`
- PTY (Pseudo-Terminal) session replay generation
- Terminal data formatting for `scriptreplay` compatibility
- Timing data calculation and output
- Session header and footer generation

### `src/zip.rs`
- ZIP archive creation utilities
- File compression and directory traversal
- Archive organization and metadata preservation

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

## Tool Calling and Efficiency

**CRITICAL**: For maximum efficiency, whenever you need to perform multiple independent operations, ALWAYS invoke all relevant tools simultaneously rather than sequentially. Especially when:

- Exploring repository structure, reading files, viewing directories
- Validating changes across multiple files
- Running multiple build/test commands that don't depend on each other
- Checking file contents and directory listings

Examples:
```bash
# ❌ Sequential - inefficient
cargo check
cargo clippy
cargo fmt --check

# ✅ Parallel - efficient when using multiple tool calls
# Call cargo check, cargo clippy, and cargo fmt --check simultaneously
```

## Code Style and Preferences

- **Minimal Changes**: Make absolutely minimal modifications - change as few lines as possible to achieve the goal
- **Preserve Existing Patterns**: Follow existing code patterns and conventions in the repository
- **Error Handling**: Use `thiserror` for custom errors and `Result<T, E>` for error propagation
- **Logging**: Avoid adding print statements; use proper error messages in Result types
- **Memory Safety**: Prefer owned types over references when crossing API boundaries
- **Performance**: Use `base64` crate for encoding/decoding, avoid string concatenation in loops

## Testing Strategy

- **No Unit Tests Currently**: The repository has `cargo test` set up but no actual tests yet
- **Manual Testing Required**: Always test CLI functionality manually after code changes
- **End-to-End Validation**: Run complete workflows (key generation → encryption → decryption)
- **Cross-Platform Considerations**: Remember that replay functionality only works on Linux/macOS
- **Error Cases**: Test error handling with invalid inputs, missing files, wrong formats

## Common Issues and Solutions

### Build Issues
- **Missing Dependencies**: Run `cargo check` first to identify missing dependencies
- **Clippy Warnings**: Address clippy warnings before committing - CI will fail otherwise
- **Format Issues**: Always run `cargo fmt --check` - CI enforces strict formatting

### Runtime Issues
- **File Path Issues**: Use absolute paths when working with temporary files
- **Platform Differences**: Replay mode requires `scriptreplay` (not available on Windows)
- **Key Format**: Keys must be exactly 44 base64-encoded characters
- **ZIP Output**: Output files must have `.zip` extension for decrypt operations

### Development Workflow
- **Incremental Testing**: Test changes frequently with `cargo check` → `cargo build` → manual testing
- **CI Validation**: All four workflows (tests.yml, lint.yaml, release.yml, semgrep.yml) must pass
- **Binary Location**: Compiled binaries are in `target/debug/` or `target/release/`

## Contributing Guidelines

### Making Changes
1. **Start Small**: Make minimal, focused changes that solve specific problems
2. **Test Early and Often**: Run validation checklist after each significant change
3. **Follow Patterns**: Maintain consistency with existing code style and error handling
4. **Document Changes**: Update README.md if CLI interface or functionality changes

### Pull Request Requirements
- All CI workflows must pass (formatting, linting, cross-platform builds, security scan)
- Manual testing validation using the comprehensive checklist above
- No new clippy warnings (existing warnings are acceptable for now)
- Proper error handling using `Result<T, E>` and `thiserror` patterns

### Release Process
- Releases are automated via GitHub Actions when tags are created
- Binary artifacts are built for Linux, macOS, and Windows
- Release binaries include SLSA attestations for supply chain security
- Version bumping should follow semantic versioning (major.minor.patch)

This tool is designed to be a secure, efficient, and reliable solution for processing Cloudflare SSH audit logs. Always prioritize security, performance, and maintainability when making changes.