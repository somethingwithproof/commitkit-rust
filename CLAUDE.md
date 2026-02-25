# CLAUDE.md

Interactive CLI for creating Conventional Commits with validation and templates.

## Stack
- Rust (edition 2021, MSRV 1.85)
- dialoguer for interactive prompts
- clap for CLI parsing

## Build & Test
```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Usage
```bash
commitkit                    # Interactive mode
commitkit --dry-run          # Preview without committing
commitkit --emoji            # Add emojis to commits
commitkit --install-hooks    # Set up git hooks
```

## Config
- File: `.commitkit.toml` (current dir or home dir)
- Customizable prefixes, scopes, subject length
