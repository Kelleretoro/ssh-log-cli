# SSH Log CLI
SSH Log CLI is a Rust command-line tool that decrypts and decodes session replay files captured by Cloudflare's Audit SSH proxy. It provides functionality to generate HPKE key pairs and decrypt/replay SSH session capture files.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively
- Bootstrap and build the repository:
  - Ensure Rust is installed (edition 2021 compatible, tested with rustc 1.89.0)
  - `cargo build` -- takes ~32 seconds for debug build. NEVER CANCEL. Set timeout to 90+ seconds.
  - `cargo build --release` -- takes ~39 seconds for optimized build. NEVER CANCEL. Set timeout to 90+ seconds.
- Run tests:
  - `cargo test` -- takes ~4 seconds, currently no tests defined but validates compilation. NEVER CANCEL. Set timeout to 60+ seconds.
- Check code formatting and style:
  - `cargo fmt --all -- --check` -- validates code formatting
  - `cargo fmt --all` -- applies automatic formatting
- Run the application:
  - Debug version: `./target/debug/ssh-log-cli`
  - Release version: `./target/release/ssh-log-cli`
  - ALWAYS build first before running

## Validation Scenarios
- Always manually validate any new code by testing both CLI subcommands after making changes.
- ALWAYS run through at least one complete end-to-end scenario after making changes:
  1. Generate a key pair: `./target/release/ssh-log-cli generate-key-pair -o test_key`
  2. Verify key files are created: `ls test_key test_key.pub`
  3. Test help commands: `./target/release/ssh-log-cli --help`
  4. Clean up test files: `rm test_key test_key.pub`
- Always run `cargo fmt --all` before you are done or the CI (.github/workflows/lint.yaml) will fail.
- The project builds and runs successfully but has no test suite - focus validation on manual CLI testing.

## Common Tasks
The following are outputs from frequently run commands. Reference them instead of viewing, searching, or running bash commands to save time.

### Repo root structure
```
/home/runner/work/ssh-log-cli/ssh-log-cli/
├── .github/
│   └── workflows/           # CI/CD workflows (tests.yml, lint.yaml, etc.)
├── .gitignore              # Excludes target/, *.txt, *.zip, test files
├── CHANGELOG.md            # Project changelog
├── CONTRIBUTING.md         # Contribution guidelines
├── Cargo.toml             # Rust project configuration
├── Cargo.lock            # Dependency lock file
├── LICENSE               # BSD-3-Clause license
├── README.md            # Main project documentation
├── package.json         # Metadata only (not functional Node.js)
├── src/                # Rust source code
│   ├── main.rs        # CLI entry point with clap argument parsing
│   ├── data.rs        # Data packet decoding and processing
│   ├── hpke.rs        # HPKE encryption/decryption implementation
│   ├── metadata.rs    # Session metadata handling
│   ├── pty.rs         # PTY session replay generation
│   └── zip.rs         # ZIP file creation utilities
└── target/            # Build output directory (gitignored)
```

### CLI Commands Available
```bash
# Generate HPKE key pair
./target/release/ssh-log-cli generate-key-pair -o <PRIVATE_KEY_NAME>
# Creates <PRIVATE_KEY_NAME> and <PRIVATE_KEY_NAME>.pub

# Decrypt and extract session data
./target/release/ssh-log-cli decrypt -i <INPUT_FILE> -k <PRIVATE_KEY> [-o <OUTPUT_ZIP>]

# Decrypt and replay PTY session (Linux/macOS only)
./target/release/ssh-log-cli decrypt -i <INPUT_FILE> -k <PRIVATE_KEY> --replay
```

### Key Dependencies
From Cargo.toml:
- **clap 3.0.0** - Command line argument parsing with derive features
- **hpke 0.8.0** - Hybrid Public Key Encryption with serde support
- **base64 0.13.0** - Base64 encoding/decoding
- **serde 1.0.130** - Serialization framework with derive features
- **zip 0.6.2** - ZIP file creation and manipulation
- **tempfile 3.3.0** - Temporary file and directory management
- **walkdir 2.3.2** - Recursive directory traversal

### Build Configuration
- **Rust Edition:** 2021
- **Release Profile:** LTO enabled for size optimization
- **Dev Dependencies:** assert_cmd, assert_fs for integration testing framework (currently unused)

### CI/CD Workflows
Located in `.github/workflows/`:
- **tests.yml** - Runs on Linux, macOS, Windows with stable Rust
- **lint.yaml** - Validates code formatting with rustfmt
- **release.yml** - Handles release automation
- **semgrep.yml** - Security scanning

### Common File Operations
The application expects:
- **Input files:** Base64 encoded encrypted session data
- **Private key files:** Base64 encoded HPKE private keys (44 bytes encoded)
- **Output files:** ZIP archives containing session data
- **PTY output:** `term_data.txt` and `term_times.txt` for scriptreplay
- **Non-PTY output:** `data_from_client.txt` and `data_from_server.txt`

### External Dependencies
- **scriptreplay** - Required for PTY session replay functionality (available at `/usr/bin/scriptreplay`)
- **Rust toolchain** - Required for building (tested with rustc 1.89.0)

### Error Patterns to Watch For
- Build failures due to missing `generate_replay` function in pty.rs (now fixed)
- Formatting violations caught by `cargo fmt --all -- --check`
- Missing .zip extension on output files (validation enforced)
- Conflicts between --replay and --output-file-name flags (mutually exclusive)

### Development Tips
- The project has duplicate source files in `src/src/` that appear to be templates - use files directly in `src/`
- Always test both key generation and decryption help after changes
- The `by kellerEToro/` directory contains branding/metadata files not core to functionality
- Use `target/release/ssh-log-cli` for performance testing, `target/debug/ssh-log-cli` for debugging